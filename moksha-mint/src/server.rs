use std::collections::HashMap;
use std::str::FromStr;

use crate::error::MokshaMintError;
use axum::extract::{Path, Query, Request, State};
use axum::http::{HeaderName, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::{get_service, post};
use axum::{middleware, Router};
use axum::{routing::get, Json};
use chrono::{Duration, Utc};
use moksha_core::keyset::{generate_hash, Keysets, V1Keyset, V1Keysets};
use moksha_core::proof::Proofs;
use moksha_core::proof::{P2SHScript, Proof};
use tracing_subscriber::EnvFilter;
use utoipa_swagger_ui::SwaggerUi;
use uuid::Uuid;

use crate::mint::Mint;
use crate::model::{GetMintQuery, PostMintQuery};
use moksha_core::blind::BlindedMessage;
use moksha_core::blind::BlindedSignature;
use moksha_core::primitives::{
    Bolt11MeltQuote, Bolt11MintQuote, CheckFeesRequest, CheckFeesResponse, CurrencyUnit,
    KeyResponse, KeysResponse, MintInfoResponse, MintLegacyInfoResponse, Nut10, Nut11, Nut12, Nut4,
    Nut5, Nut6, Nut7, Nut8, Nut9, Nuts, PaymentMethod, PaymentRequest, PostMeltBolt11Request,
    PostMeltBolt11Response, PostMeltQuoteBolt11Request, PostMeltQuoteBolt11Response,
    PostMeltRequest, PostMeltResponse, PostMintBolt11Request, PostMintBolt11Response,
    PostMintQuoteBolt11Request, PostMintQuoteBolt11Response, PostMintRequest, PostMintResponse,
    PostSplitRequest, PostSplitResponse, PostSwapRequest, PostSwapResponse,
};
use secp256k1::PublicKey;

use tower_http::services::ServeDir;

use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{event, info, Level};

use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use utoipa::OpenApi;

pub async fn run_server(mint: Mint) -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    if let Some(ref buildtime) = mint.config.build.build_time {
        info!("build time: {}", buildtime);
    }
    if let Some(ref commithash) = mint.config.build.commit_hash {
        info!("git commit-hash: {}", commithash);
    }
    if let Some(ref serve_wallet_path) = mint.config.server.serve_wallet_path {
        info!("serving wallet from path: {:?}", serve_wallet_path);
    }
    info!("listening on: {}", &mint.config.server.host_port);
    info!("mint-info: {:?}", mint.config.info);
    info!("lightning fee-reserve: {:?}", mint.config.lightning_fee);
    info!("lightning-backend: {}", mint.lightning_type);

    let listener = tokio::net::TcpListener::bind(&mint.config.server.host_port)
        .await
        .unwrap();

    axum::serve(
        listener,
        app(mint)
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_headers(Any)
                    .allow_methods([axum::http::Method::GET, axum::http::Method::POST]),
            )
            .into_make_service(),
    )
    .await?;

    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_keys,
        get_keys_by_id,
        get_keysets,
        post_mint_bolt11,
        post_mint_quote_bolt11,
        get_mint_quote_bolt11,
        post_melt_bolt11,
        post_melt_quote_bolt11,
        get_melt_quote_bolt11,
        post_swap,
        get_info,
        get_health,
    ),
    components(schemas(
        MintInfoResponse,
        Nuts,
        Nut4,
        Nut5,
        Nut6,
        Nut7,
        Nut8,
        Nut9,
        Nut10,
        Nut11,
        Nut12,
        CurrencyUnit,
        PaymentMethod,
        KeysResponse,
        KeyResponse,
        V1Keysets,
        V1Keyset,
        BlindedMessage,
        BlindedSignature,
        Proof,
        Proofs,
        PostMintQuoteBolt11Request,
        PostMintQuoteBolt11Response,
        PostMeltQuoteBolt11Request,
        PostMeltQuoteBolt11Response,
        PostMeltBolt11Request,
        PostMeltBolt11Response,
        PostMintBolt11Request,
        PostMintBolt11Response,
        PostSwapRequest,
        PostSwapResponse,
        P2SHScript
    ))
)]
struct ApiDoc;

fn app(mint: Mint) -> Router {
    let legacy_routes = Router::new()
        .route("/keys", get(get_legacy_keys))
        .route("/keysets", get(get_legacy_keysets))
        .route("/mint", get(get_legacy_mint).post(post_legacy_mint))
        .route("/checkfees", post(post_legacy_check_fees))
        .route("/melt", post(post_legacy_melt))
        .route("/split", post(post_legacy_split))
        .route("/info", get(get_legacy_info));

    let routes = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/v1/keys", get(get_keys))
        .route("/v1/keys/:id", get(get_keys_by_id))
        .route("/v1/keysets", get(get_keysets))
        .route("/v1/mint/quote/bolt11", post(post_mint_quote_bolt11))
        .route("/v1/mint/quote/bolt11/:quote", get(get_mint_quote_bolt11))
        .route("/v1/mint/bolt11", post(post_mint_bolt11))
        .route("/v1/melt/quote/bolt11", post(post_melt_quote_bolt11))
        .route("/v1/melt/quote/bolt11/:quote", get(get_melt_quote_bolt11))
        .route("/v1/melt/bolt11", post(post_melt_bolt11))
        .route("/v1/swap", post(post_swap))
        .route("/v1/info", get(get_info));

    let general_routes = Router::new().route("/health", get(get_health));

    let server_config = mint.config.server.clone();
    let prefix = server_config.api_prefix.unwrap_or_else(|| "".to_owned());

    let router = Router::new()
        .nest(&prefix, legacy_routes)
        .nest(&prefix, routes)
        .nest("", general_routes)
        .with_state(mint)
        .layer(TraceLayer::new_for_http());

    if let Some(ref serve_wallet_path) = server_config.serve_wallet_path {
        return router.nest_service(
            "/",
            get_service(ServeDir::new(serve_wallet_path))
                .layer(middleware::from_fn(add_response_headers)),
        );
    }
    router
}

/// This function adds response headers that are specific to Flutter web applications.
///
/// It sets the `cross-origin-embedder-policy` header to `require-corp` and the
/// `cross-origin-opener-policy` header to `same-origin`. These headers are necessary
/// for some features of Flutter web applications, such as isolating the application
/// from potential security threats in other browsing contexts.
///
/// # Arguments
///
/// * `req` - The incoming request.
/// * `next` - The next middleware or endpoint in the processing chain.
///
/// # Returns
///
/// This function returns a `Result` with the modified response, or an error if
/// something went wrong while processing the request or response.
async fn add_response_headers(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut res = next.run(req).await;

    res.headers_mut().insert(
        HeaderName::from_static("cross-origin-embedder-policy"),
        HeaderValue::from_static("require-corp"),
    );
    res.headers_mut().insert(
        HeaderName::from_static("cross-origin-opener-policy"),
        HeaderValue::from_static("same-origin"),
    );
    Ok(res)
}

async fn post_legacy_split(
    State(mint): State<Mint>,
    Json(swap_request): Json<PostSplitRequest>,
) -> Result<Json<PostSplitResponse>, MokshaMintError> {
    let response = mint
        .swap(
            &swap_request.proofs,
            &swap_request.outputs,
            &mint.keyset_legacy,
        )
        .await?;

    Ok(Json(PostSplitResponse::with_promises(response)))
}

async fn post_legacy_melt(
    State(mint): State<Mint>,
    Json(melt_request): Json<PostMeltRequest>,
) -> Result<Json<PostMeltResponse>, MokshaMintError> {
    let (paid, preimage, change) = mint
        .melt(
            melt_request.pr,
            0, // FIXME set correct fee reserve for legacy api
            &melt_request.proofs,
            &melt_request.outputs,
            &mint.keyset_legacy,
        )
        .await?;

    Ok(Json(PostMeltResponse {
        paid,
        preimage,
        change,
    }))
}

async fn post_legacy_check_fees(
    State(mint): State<Mint>,
    Json(_check_fees): Json<CheckFeesRequest>,
) -> Result<Json<CheckFeesResponse>, MokshaMintError> {
    let invoice = mint.lightning.decode_invoice(_check_fees.pr).await?;

    Ok(Json(CheckFeesResponse {
        fee: mint.fee_reserve(
            invoice
                .amount_milli_satoshis()
                .ok_or_else(|| crate::error::MokshaMintError::InvalidAmount)?,
        ),
    }))
}

async fn get_legacy_info(
    State(mint): State<Mint>,
) -> Result<Json<MintLegacyInfoResponse>, MokshaMintError> {
    let mint_info = MintLegacyInfoResponse {
        name: mint.config.info.name,
        pubkey: mint.keyset_legacy.mint_pubkey,
        version: match mint.config.info.version {
            true => Some(mint.config.build.full_version()),
            _ => None,
        },
        description: mint.config.info.description,
        description_long: mint.config.info.description_long,
        contact: mint.config.info.contact,
        nuts: vec![
            "NUT-00".to_string(),
            "NUT-01".to_string(),
            "NUT-02".to_string(),
            "NUT-03".to_string(),
            "NUT-04".to_string(),
            "NUT-05".to_string(),
            "NUT-06".to_string(),
            "NUT-08".to_string(),
            "NUT-09".to_string(),
        ],
        motd: mint.config.info.motd,
        parameter: Default::default(),
    };
    Ok(Json(mint_info))
}

async fn get_legacy_mint(
    State(mint): State<Mint>,
    Query(mint_query): Query<GetMintQuery>,
) -> Result<Json<PaymentRequest>, MokshaMintError> {
    let (pr, hash) = mint
        .create_invoice(generate_hash(), mint_query.amount)
        .await?;
    Ok(Json(PaymentRequest { pr, hash }))
}

async fn post_legacy_mint(
    State(mint): State<Mint>,
    Query(mint_query): Query<PostMintQuery>,
    Json(blinded_messages): Json<PostMintRequest>,
) -> Result<Json<PostMintResponse>, MokshaMintError> {
    event!(
        Level::INFO,
        "post_mint: {mint_query:#?} {blinded_messages:#?}"
    );

    let promises = mint
        .mint_tokens(
            mint_query.hash,
            &blinded_messages.outputs,
            &mint.keyset_legacy,
        )
        .await?;
    Ok(Json(PostMintResponse { promises }))
}

async fn get_legacy_keys(
    State(mint): State<Mint>,
) -> Result<Json<HashMap<u64, PublicKey>>, MokshaMintError> {
    Ok(Json(mint.keyset_legacy.public_keys))
}

async fn get_legacy_keysets(State(mint): State<Mint>) -> Result<Json<Keysets>, MokshaMintError> {
    Ok(Json(Keysets::new(vec![mint.keyset_legacy.keyset_id])))
}

#[utoipa::path(
        get,
        path = "/health",
        responses(
            (status = 200, description = "health check")
        ),
    )]
async fn get_health() -> impl IntoResponse {
    StatusCode::OK
}

// ######################################################################################################

#[utoipa::path(
        post,
        path = "/v1/swap",
        request_body = PostSwapRequest,
        responses(
            (status = 200, description = "post swap", body = [PostSwapResponse])
        ),
    )]
async fn post_swap(
    State(mint): State<Mint>,
    Json(swap_request): Json<PostSwapRequest>,
) -> Result<Json<PostSwapResponse>, MokshaMintError> {
    let response = mint
        .swap(&swap_request.inputs, &swap_request.outputs, &mint.keyset)
        .await?;

    Ok(Json(PostSwapResponse {
        signatures: response,
    }))
}

#[utoipa::path(
        get,
        path = "/v1/keys",
        responses(
            (status = 200, description = "get keys", body = [KeysResponse])
        )
    )]
async fn get_keys(State(mint): State<Mint>) -> Result<Json<KeysResponse>, MokshaMintError> {
    Ok(Json(KeysResponse {
        keysets: vec![KeyResponse {
            id: mint.keyset.keyset_id.clone(),
            unit: CurrencyUnit::Sat,
            keys: mint.keyset.public_keys.clone(),
        }],
    }))
}

#[utoipa::path(
        get,
        path = "/v1/keys/{id}",
        responses(
            (status = 200, description = "get keys by id", body = [KeysResponse])
        ),
        params(
            ("id" = String, Path, description = "keyset id"),
        )
    )]
async fn get_keys_by_id(
    Path(id): Path<String>,
    State(mint): State<Mint>,
) -> Result<Json<KeysResponse>, MokshaMintError> {
    if id != mint.keyset.keyset_id {
        return Err(MokshaMintError::KeysetNotFound(id));
    }

    Ok(Json(KeysResponse {
        keysets: vec![KeyResponse {
            id: mint.keyset.keyset_id.clone(),
            unit: CurrencyUnit::Sat,
            keys: mint.keyset.public_keys.clone(),
        }],
    }))
}

#[utoipa::path(
        get,
        path = "/v1/keysets",
        responses(
            (status = 200, description = "get keysets", body = [V1Keysets])
        ),
    )]
async fn get_keysets(State(mint): State<Mint>) -> Result<Json<V1Keysets>, MokshaMintError> {
    Ok(Json(V1Keysets::new(
        mint.keyset.keyset_id,
        CurrencyUnit::Sat,
        true,
    )))
}

#[utoipa::path(
        post,
        path = "/v1/mint/quote/bolt11",
        request_body = PostMintQuoteBolt11Request,
        responses(
            (status = 200, description = "post mint quote", body = [PostMintQuoteBolt11Response])
        ),
    )]
async fn post_mint_quote_bolt11(
    State(mint): State<Mint>,
    Json(request): Json<PostMintQuoteBolt11Request>,
) -> Result<Json<PostMintQuoteBolt11Response>, MokshaMintError> {
    // FIXME check currency unit
    let key = Uuid::new_v4();
    let (pr, _hash) = mint.create_invoice(key.to_string(), request.amount).await?;

    let quote = Bolt11MintQuote {
        quote_id: key,
        payment_request: pr.clone(),
        expiry: quote_expiry(), // FIXME use timestamp type in DB
        paid: false,
    };

    mint.db.add_bolt11_mint_quote(&quote).await?;
    Ok(Json(quote.into()))
}

#[utoipa::path(
        post,
        path = "/v1/mint/bolt11/{quote_id}",
        request_body = PostMintBolt11Request,
        responses(
            (status = 200, description = "post mint quote", body = [PostMintBolt11Response])
        ),
        params(
            ("quote_id" = String, Path, description = "quote id"),
        )
    )]
async fn post_mint_bolt11(
    State(mint): State<Mint>,
    Json(request): Json<PostMintBolt11Request>,
) -> Result<Json<PostMintBolt11Response>, MokshaMintError> {
    let signatures = mint
        .mint_tokens(request.quote.clone(), &request.outputs, &mint.keyset)
        .await?;

    let old_quote = &mint
        .db
        .get_bolt11_mint_quote(&Uuid::from_str(request.quote.as_str())?)
        .await?;

    mint.db
        .update_bolt11_mint_quote(&Bolt11MintQuote {
            paid: true,
            ..old_quote.clone()
        })
        .await?;
    Ok(Json(PostMintBolt11Response { signatures }))
}

#[utoipa::path(
        post,
        path = "/v1/melt/quote/bolt11",
        request_body = PostMeltQuoteBolt11Request,
        responses(
            (status = 200, description = "post mint quote", body = [PostMeltQuoteBolt11Response])
        ),
    )]
async fn post_melt_quote_bolt11(
    State(mint): State<Mint>,
    Json(melt_request): Json<PostMeltQuoteBolt11Request>,
) -> Result<Json<PostMeltQuoteBolt11Response>, MokshaMintError> {
    let invoice = mint
        .lightning
        .decode_invoice(melt_request.request.clone())
        .await?;
    let amount = invoice
        .amount_milli_satoshis()
        .ok_or_else(|| crate::error::MokshaMintError::InvalidAmount)?;
    let fee_reserve = mint.fee_reserve(amount) / 1_000; // FIXME check if this is correct
    info!("fee_reserve: {}", fee_reserve);

    let amount_sat = amount / 1_000;
    let key = Uuid::new_v4();
    let quote = Bolt11MeltQuote {
        quote_id: key,
        amount: amount_sat,
        fee_reserve,
        expiry: quote_expiry(),
        payment_request: melt_request.request.clone(),
        paid: false,
    };
    mint.db.add_bolt11_melt_quote(&quote).await?;

    Ok(Json(quote.try_into().map_err(|_| {
        crate::error::MokshaMintError::InvalidQuote("".to_string())
    })?))
}

fn quote_expiry() -> u64 {
    // FIXME add config option for expiry
    let now = Utc::now() + Duration::minutes(30);
    now.timestamp() as u64
}

#[utoipa::path(
        post,
        path = "/v1/melt/bolt11",
        request_body = PostMeltBolt11Request,
        responses(
            (status = 200, description = "post melt", body = [PostMeltBolt11Response])
        ),
    )]
async fn post_melt_bolt11(
    State(mint): State<Mint>,
    Json(melt_request): Json<PostMeltBolt11Request>,
) -> Result<Json<PostMeltBolt11Response>, MokshaMintError> {
    let quote = mint
        .db
        .get_bolt11_melt_quote(&Uuid::from_str(melt_request.quote.as_str())?)
        .await?;

    println!(
        "post_melt_bolt11 fee_reserve >>>>>>>>>>>>>> : {:#?}",
        &quote
    );

    let (paid, payment_preimage, change) = mint
        .melt(
            quote.payment_request.to_owned(),
            quote.fee_reserve,
            &melt_request.inputs,
            &melt_request.outputs,
            &mint.keyset,
        )
        .await?;
    mint.db
        .update_bolt11_melt_quote(&Bolt11MeltQuote { paid, ..quote })
        .await?;

    Ok(Json(PostMeltBolt11Response {
        paid,
        payment_preimage: Some(payment_preimage),
        change,
    }))
}

#[utoipa::path(
        get,
        path = "/v1/mint/quote/bolt11/{quote_id}",
        responses(
            (status = 200, description = "get mint quote by id", body = [PostMintQuoteBolt11Response])
        ),
        params(
            ("quote_id" = String, Path, description = "quote id"),
        )
    )]
async fn get_mint_quote_bolt11(
    Path(quote_id): Path<String>,
    State(mint): State<Mint>,
) -> Result<Json<PostMintQuoteBolt11Response>, MokshaMintError> {
    info!("get_quote: {}", quote_id);

    let quote = mint
        .db
        .get_bolt11_mint_quote(&Uuid::from_str(quote_id.as_str())?)
        .await?;

    let paid = mint
        .lightning
        .is_invoice_paid(quote.payment_request.clone())
        .await?;

    Ok(Json(Bolt11MintQuote { paid, ..quote }.into()))
}

#[utoipa::path(
        get,
        path = "/v1/melt/quote/bolt11/{quote_id}",
        responses(
            (status = 200, description = "post mint quote", body = [PostMeltQuoteBolt11Response])
        ),
        params(
            ("quote_id" = String, Path, description = "quote id"),
        )
    )]
async fn get_melt_quote_bolt11(
    Path(quote_id): Path<String>,
    State(mint): State<Mint>,
) -> Result<Json<PostMeltQuoteBolt11Response>, MokshaMintError> {
    info!("get_melt_quote: {}", quote_id);
    let quote = mint
        .db
        .get_bolt11_melt_quote(&Uuid::from_str(quote_id.as_str())?)
        .await?;

    // FIXME check for paid?
    Ok(Json(quote.into()))
}

#[utoipa::path(
        get,
        path = "/v1/info",
        responses(
            (status = 200, description = "get mint info", body = [MintInfoResponse])
        )
    )]
async fn get_info(State(mint): State<Mint>) -> Result<Json<MintInfoResponse>, MokshaMintError> {
    // TODO implement From-trait
    let mint_info = MintInfoResponse {
        name: mint.config.info.name,
        pubkey: mint.keyset.mint_pubkey,
        version: match mint.config.info.version {
            true => Some(mint.config.build.full_version()),
            _ => None,
        },
        description: mint.config.info.description,
        description_long: mint.config.info.description_long,
        contact: mint.config.info.contact,
        nuts: Nuts::default(),
        motd: mint.config.info.motd,
    };
    Ok(Json(mint_info))
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::Arc};

    use crate::{config::MintConfig, onchain::MockOnchain, server::app};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use moksha_core::{
        keyset::{Keysets, V1Keysets},
        primitives::{CurrencyUnit, KeysResponse, MintLegacyInfoResponse},
    };
    use secp256k1::PublicKey;
    use tower::ServiceExt;

    use crate::{
        config::MintInfoConfig,
        database::MockDatabase,
        lightning::{LightningType, MockLightning},
        mint::Mint,
    };

    #[tokio::test]
    async fn test_get_keys() -> anyhow::Result<()> {
        let app = app(create_mock_mint(Default::default()));
        let response = app
            .oneshot(Request::builder().uri("/keys").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let keys: HashMap<u64, PublicKey> = serde_json::from_slice(&body)?;
        assert_eq!(64, keys.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_keysets() -> anyhow::Result<()> {
        let app = app(create_mock_mint(Default::default()));
        let response = app
            .oneshot(Request::builder().uri("/keysets").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let keysets = serde_json::from_slice::<Keysets>(&body)?;
        assert_eq!(Keysets::new(vec!["53eJP2+qJyTd".to_string()]), keysets);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_info() -> anyhow::Result<()> {
        let mint_info_settings = MintInfoConfig {
            name: Some("Bob's Cashu mint".to_string()),
            version: true,
            description: Some("A mint for testing".to_string()),
            description_long: Some("A mint for testing long".to_string()),
            ..Default::default()
        };
        let app = app(create_mock_mint(mint_info_settings));
        let response = app
            .oneshot(Request::builder().uri("/info").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let info = serde_json::from_slice::<MintLegacyInfoResponse>(&body)?;
        assert!(!info.parameter.peg_out_only);
        assert_eq!(info.nuts.len(), 9);
        assert_eq!(info.name, Some("Bob's Cashu mint".to_string()));
        assert_eq!(info.description, Some("A mint for testing".to_string()));
        assert_eq!(
            info.description_long,
            Some("A mint for testing long".to_string())
        );
        Ok(())
    }

    fn create_mock_mint(info: MintInfoConfig) -> Mint {
        let db = Arc::new(MockDatabase::new());
        let lightning = Arc::new(MockLightning::new());

        Mint::new(
            "mytestsecret".to_string(),
            "".to_string(),
            lightning,
            LightningType::Lnbits(Default::default()),
            db,
            MintConfig {
                info,
                ..Default::default()
            },
            Arc::new(MockOnchain::default()),
        )
    }

    // ################ v1 api tests #####################

    #[tokio::test]
    async fn test_get_keys_v1() -> anyhow::Result<()> {
        let app = app(create_mock_mint(Default::default()));
        let response = app
            .oneshot(Request::builder().uri("/v1/keys").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let keys: KeysResponse = serde_json::from_slice(&body)?;
        let keysets = keys.keysets;
        assert_eq!(&1, &keysets.len());
        assert_eq!(64, keysets[0].keys.len());
        assert_eq!(16, keysets[0].id.len());
        assert_eq!(CurrencyUnit::Sat, keysets[0].unit);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_keysets_v1() -> anyhow::Result<()> {
        let app = app(create_mock_mint(Default::default()));
        let response = app
            .oneshot(Request::builder().uri("/v1/keysets").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let keysets = serde_json::from_slice::<V1Keysets>(&body)?;
        assert_eq!(1, keysets.keysets.len());
        assert_eq!(16, keysets.keysets[0].id.len());
        Ok(())
    }

    // ### v1 api tests

    #[tokio::test]
    async fn test_get_v1_keys() -> anyhow::Result<()> {
        let app = app(create_mock_mint(Default::default()));
        let response = app
            .oneshot(Request::builder().uri("/v1/keys").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let keys: KeysResponse = serde_json::from_slice(&body)?;
        assert_eq!(1, keys.keysets.len());
        assert_eq!(
            64,
            keys.keysets.get(0).expect("keyset not found").keys.len()
        );
        println!("{:#?}", keys.keysets.get(0).unwrap().id);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_v1_keys_id_invalid() -> anyhow::Result<()> {
        let app = app(create_mock_mint(Default::default()));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/keys/unknownkeyset")
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_v1_keys_id() -> anyhow::Result<()> {
        let app = app(create_mock_mint(Default::default()));
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/keys/00f545318e4fad2b")
                    .body(Body::empty())?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let keys: KeysResponse = serde_json::from_slice(&body)?;
        assert_eq!(1, keys.keysets.len());
        assert_eq!(
            64,
            keys.keysets.get(0).expect("keyset not found").keys.len()
        );
        assert_eq!(
            "00f545318e4fad2b",
            keys.keysets.get(0).expect("keyset not found").id
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_v1_keysets() -> anyhow::Result<()> {
        let app = app(create_mock_mint(Default::default()));
        let response = app
            .oneshot(Request::builder().uri("/v1/keysets").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let keys: V1Keysets = serde_json::from_slice(&body)?;
        assert_eq!(1, keys.keysets.len());
        let keyset = keys.keysets.get(0).expect("keyset not found");
        assert!(keyset.active);
        assert_eq!(CurrencyUnit::Sat, keyset.unit);
        assert_eq!("00f545318e4fad2b", keyset.id);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_health() -> anyhow::Result<()> {
        let app = app(create_mock_mint(Default::default()));
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty())?)
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        Ok(())
    }
}
