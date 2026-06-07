import 'package:hybrid_runtime/bridge/bridge.dart';
import 'package:hybrid_runtime/bridge/message/interface.dart';

import 'consumer.dart';
import 'proto.dart';

export 'interface.dart';
export 'proto.dart';
export 'consumer.dart';
export 'sender.dart';

/// Creates a new message client instance
/// Sets up the bridge communication and message handling
MessageClient createMessageClient(DartBridge bridge) {
  final proto = MessageProtoImpl();
  final client = MessageClientImpl(proto, bridge);
  bridge.addEventListener((callParams) {
    return client.handleMessage(callParams);
  });
  return client;
}
