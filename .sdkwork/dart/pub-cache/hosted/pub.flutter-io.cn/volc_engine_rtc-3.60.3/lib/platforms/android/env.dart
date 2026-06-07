/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import 'package:hybrid_runtime/hybrid_runtime.dart';

class ApplicationContext extends NativeClass {
  ApplicationContext(String namespace, String instanceId)
    : super(
        NativeClassOptions(
          [],
          className: namespace,
          instanceId: instanceId,
          disableInit: true,
          bridgeKey: 'com.volcengine.rtc.hybrid_runtime'
        ),
      );
}
