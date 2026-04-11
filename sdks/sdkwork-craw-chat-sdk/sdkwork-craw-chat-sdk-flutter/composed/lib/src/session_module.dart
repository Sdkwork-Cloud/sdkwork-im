import 'package:backend_sdk/backend_sdk.dart';

import 'context.dart';

class CrawChatSessionModule {
  final CrawChatSdkContext context;

  CrawChatSessionModule(this.context);

  Future<SessionResumeView?> resume(ResumeSessionRequest body) {
    return context.backendClient.session.resume(body);
  }

  Future<PresenceSnapshotView?> disconnectDevice(PresenceDeviceRequest body) {
    return context.backendClient.session.disconnect(body);
  }
}
