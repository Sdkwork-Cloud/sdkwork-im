/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import "package:hybrid_runtime/hybrid_runtime.dart";
import './env.dart';

typedef ByteRTCImage = dynamic;

class NativeVariables extends NativeClass {
  static const String _$namespace = r'$var';

  NativeVariables(namespace)
    : super(NativeClassOptions([], className: _$namespace, disableInit: true, bridgeKey: 'com.volcengine.rtc.hybrid_runtime'));

  static Future<ApplicationContext> getApplicationContext() async {
    return await NativeClassUtils.sendVarGet(
      _$namespace,
      'ApplicationContext',
      (String namespace, String instanceId) =>
          ApplicationContext(namespace, instanceId),
      'com.volcengine.rtc.hybrid_runtime',
    );
  }
}