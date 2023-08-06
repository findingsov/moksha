// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.80.1.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';
import 'dart:ffi' as ffi;

class NativePlatform extends FlutterRustBridgeBase<NativeWire> {
  NativePlatform(ffi.DynamicLibrary dylib) : super(NativeWire(dylib));

// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  int api2wire_u64(int raw) {
    return raw;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: finalizer

// Section: api_fill_to_wire
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
// ignore_for_file: type=lint

/// generated by flutter_rust_bridge
class NativeWire implements FlutterRustBridgeWireBase {
  @internal
  late final dartApi = DartApiDl(init_frb_dart_api_dl);

  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  NativeWire(ffi.DynamicLibrary dynamicLibrary) : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  NativeWire.fromLookup(ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName) lookup) : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>('store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr.asFunction<void Function(DartPostCObjectFnType)>();

  Object get_dart_object(
    int ptr,
  ) {
    return _get_dart_object(
      ptr,
    );
  }

  late final _get_dart_objectPtr = _lookup<ffi.NativeFunction<ffi.Handle Function(ffi.UintPtr)>>('get_dart_object');
  late final _get_dart_object = _get_dart_objectPtr.asFunction<Object Function(int)>();

  void drop_dart_object(
    int ptr,
  ) {
    return _drop_dart_object(
      ptr,
    );
  }

  late final _drop_dart_objectPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.UintPtr)>>('drop_dart_object');
  late final _drop_dart_object = _drop_dart_objectPtr.asFunction<void Function(int)>();

  int new_dart_opaque(
    Object handle,
  ) {
    return _new_dart_opaque(
      handle,
    );
  }

  late final _new_dart_opaquePtr = _lookup<ffi.NativeFunction<ffi.UintPtr Function(ffi.Handle)>>('new_dart_opaque');
  late final _new_dart_opaque = _new_dart_opaquePtr.asFunction<int Function(Object)>();

  int init_frb_dart_api_dl(
    ffi.Pointer<ffi.Void> obj,
  ) {
    return _init_frb_dart_api_dl(
      obj,
    );
  }

  late final _init_frb_dart_api_dlPtr =
      _lookup<ffi.NativeFunction<ffi.IntPtr Function(ffi.Pointer<ffi.Void>)>>('init_frb_dart_api_dl');
  late final _init_frb_dart_api_dl = _init_frb_dart_api_dlPtr.asFunction<int Function(ffi.Pointer<ffi.Void>)>();

  void wire_init_cashu(
    int port_,
  ) {
    return _wire_init_cashu(
      port_,
    );
  }

  late final _wire_init_cashuPtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_init_cashu');
  late final _wire_init_cashu = _wire_init_cashuPtr.asFunction<void Function(int)>();

  void wire_get_cashu_balance(
    int port_,
  ) {
    return _wire_get_cashu_balance(
      port_,
    );
  }

  late final _wire_get_cashu_balancePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_get_cashu_balance');
  late final _wire_get_cashu_balance = _wire_get_cashu_balancePtr.asFunction<void Function(int)>();

  void wire_cashu_mint_tokens(
    int port_,
    int amount,
    ffi.Pointer<wire_uint_8_list> hash,
  ) {
    return _wire_cashu_mint_tokens(
      port_,
      amount,
      hash,
    );
  }

  late final _wire_cashu_mint_tokensPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint64, ffi.Pointer<wire_uint_8_list>)>>(
          'wire_cashu_mint_tokens');
  late final _wire_cashu_mint_tokens =
      _wire_cashu_mint_tokensPtr.asFunction<void Function(int, int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_get_cashu_mint_payment_request(
    int port_,
    int amount,
  ) {
    return _wire_get_cashu_mint_payment_request(
      port_,
      amount,
    );
  }

  late final _wire_get_cashu_mint_payment_requestPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint64)>>('wire_get_cashu_mint_payment_request');
  late final _wire_get_cashu_mint_payment_request =
      _wire_get_cashu_mint_payment_requestPtr.asFunction<void Function(int, int)>();

  WireSyncReturn wire_decode_invoice(
    ffi.Pointer<wire_uint_8_list> invoice,
  ) {
    return _wire_decode_invoice(
      invoice,
    );
  }

  late final _wire_decode_invoicePtr =
      _lookup<ffi.NativeFunction<WireSyncReturn Function(ffi.Pointer<wire_uint_8_list>)>>('wire_decode_invoice');
  late final _wire_decode_invoice =
      _wire_decode_invoicePtr.asFunction<WireSyncReturn Function(ffi.Pointer<wire_uint_8_list>)>();

  void wire_cashu_pay_invoice(
    int port_,
    ffi.Pointer<wire_uint_8_list> invoice,
  ) {
    return _wire_cashu_pay_invoice(
      port_,
      invoice,
    );
  }

  late final _wire_cashu_pay_invoicePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>(
          'wire_cashu_pay_invoice');
  late final _wire_cashu_pay_invoice =
      _wire_cashu_pay_invoicePtr.asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_join_federation(
    int port_,
    ffi.Pointer<wire_uint_8_list> federation,
  ) {
    return _wire_join_federation(
      port_,
      federation,
    );
  }

  late final _wire_join_federationPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_join_federation');
  late final _wire_join_federation =
      _wire_join_federationPtr.asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_get_fedimint_payment_request(
    int port_,
    int amount,
  ) {
    return _wire_get_fedimint_payment_request(
      port_,
      amount,
    );
  }

  late final _wire_get_fedimint_payment_requestPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint64)>>('wire_get_fedimint_payment_request');
  late final _wire_get_fedimint_payment_request =
      _wire_get_fedimint_payment_requestPtr.asFunction<void Function(int, int)>();

  void wire_fedimint_mint_tokens(
    int port_,
    int amount,
    ffi.Pointer<wire_uint_8_list> operation_id,
  ) {
    return _wire_fedimint_mint_tokens(
      port_,
      amount,
      operation_id,
    );
  }

  late final _wire_fedimint_mint_tokensPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Uint64, ffi.Pointer<wire_uint_8_list>)>>(
          'wire_fedimint_mint_tokens');
  late final _wire_fedimint_mint_tokens =
      _wire_fedimint_mint_tokensPtr.asFunction<void Function(int, int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_get_fedimint_balance(
    int port_,
  ) {
    return _wire_get_fedimint_balance(
      port_,
    );
  }

  late final _wire_get_fedimint_balancePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_get_fedimint_balance');
  late final _wire_get_fedimint_balance = _wire_get_fedimint_balancePtr.asFunction<void Function(int)>();

  void wire_fedimint_pay_invoice(
    int port_,
    ffi.Pointer<wire_uint_8_list> invoice,
  ) {
    return _wire_fedimint_pay_invoice(
      port_,
      invoice,
    );
  }

  late final _wire_fedimint_pay_invoicePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>(
          'wire_fedimint_pay_invoice');
  late final _wire_fedimint_pay_invoice =
      _wire_fedimint_pay_invoicePtr.asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_receive_token(
    int port_,
    ffi.Pointer<wire_uint_8_list> token,
  ) {
    return _wire_receive_token(
      port_,
      token,
    );
  }

  late final _wire_receive_tokenPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_receive_token');
  late final _wire_receive_token =
      _wire_receive_tokenPtr.asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_get_btcprice(
    int port_,
  ) {
    return _wire_get_btcprice(
      port_,
    );
  }

  late final _wire_get_btcpricePtr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>('wire_get_btcprice');
  late final _wire_get_btcprice = _wire_get_btcpricePtr.asFunction<void Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_uint_8_list> Function(ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr.asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturn(
    WireSyncReturn ptr,
  ) {
    return _free_WireSyncReturn(
      ptr,
    );
  }

  late final _free_WireSyncReturnPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturn)>>('free_WireSyncReturn');
  late final _free_WireSyncReturn = _free_WireSyncReturnPtr.asFunction<void Function(WireSyncReturn)>();
}

final class _Dart_Handle extends ffi.Opaque {}

final class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType
    = ffi.Pointer<ffi.NativeFunction<ffi.Bool Function(DartPort port_id, ffi.Pointer<ffi.Void> message)>>;
typedef DartPort = ffi.Int64;
