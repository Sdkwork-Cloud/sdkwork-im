import 'package:backend_sdk/backend_sdk.dart';

import 'context.dart';

class CrawChatPresenceModule {
  final CrawChatSdkContext context;

  CrawChatPresenceModule(this.context);

  Future<PresenceSnapshotView?> heartbeat(PresenceDeviceRequest body) {
    return context.backendClient.presence.heartbeat(body);
  }

  Future<PresenceSnapshotView?> current() {
    return context.backendClient.presence.getPresenceMe();
  }
}
