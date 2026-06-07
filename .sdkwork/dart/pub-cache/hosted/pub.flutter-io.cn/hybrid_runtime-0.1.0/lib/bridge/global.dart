import 'bridge.dart';
import 'message/index.dart';

const DEFAULT_BRIDGE_KEY = 'com.volcengine.hybrid_runtime';
/// Global Dart Bridge instance

final Map<String, MessageClient> _globalMessageClientMap = <String, MessageClient>{};
/// Global message client instance

/// Get global message client instance
MessageClient getGlobalMessageClient([String? bridgekey]) {
  final bridgeKey = bridgekey ?? DEFAULT_BRIDGE_KEY;
  if (_globalMessageClientMap.containsKey(bridgeKey)) {
    return _globalMessageClientMap[bridgeKey]!;
  } else {
    final bridge = DartBridge.getInstance(bridgeKey);
    _globalMessageClientMap[bridgeKey] = createMessageClient(bridge);
    return _globalMessageClientMap[bridgeKey]!;
  }
}

/// Get global Bridge instance
DartBridge getBridge([String? bridgekey]) {
  final bridgeKey = bridgekey ?? DEFAULT_BRIDGE_KEY;
  return DartBridge.getInstance(bridgeKey);
}
