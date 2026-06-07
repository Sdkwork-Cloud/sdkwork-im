/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import 'dart:io';
import 'package:hybrid_runtime/hybrid_runtime.dart';

import '../codegen/pack/index.dart' as $p;
import '../codegen/android/index.dart' as $a;
import '../codegen/ios/index.dart' as $i;

export '../codegen/pack/callback.dart' show IWTNStreamEventHandler;
export '../codegen/pack/keytype.dart'
    show WTNSubscribeState, WTNSubscribeStateChangeReason;

/// @nodoc
late dynamic $wtn_instance;

/// @brief 公共流封装类
class WTNStream extends $p.IWTNStream {
  @override
  dynamic $createInstance(List<dynamic> args) {
    print('Do nothing, just return wtn instance.');
    return $wtn_instance;
  }

  WTNStream();

  /// 设置 WTN 公共流事件回调
  Future<int> setWTNStreamEventHandler(
      $p.IWTNStreamEventHandler handler) async {
    $android() {
      return ($instance as $a.IWTNStream).setWTNStreamEventHandler(
        packObject(handler, () => $p.android_IWTNStreamEventHandler()),
      );
    }

    $ios() {
      try {
        ($instance as $i.ByteRTCWTNStream).setWTNStreamDelegate(
          packObject(handler, () => $p.ios_IWTNStreamEventHandler()),
        );
        return Future.value(0);
      } catch (e) {
        return Future.value(-1);
      }
    }

    if (Platform.isAndroid) {
      return $android();
    } else if (Platform.isIOS) {
      return $ios();
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
  }
}
