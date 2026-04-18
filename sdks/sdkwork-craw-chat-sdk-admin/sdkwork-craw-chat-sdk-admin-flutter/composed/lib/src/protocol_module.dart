import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

import 'context.dart';

class CrawChatAdminProtocolModule {
  final CrawChatAdminSdkContext context;

  CrawChatAdminProtocolModule(this.context);

  Future<JsonObject> getGovernance() {
    return context.backendClient.protocol.getProtocolGovernance();
  }

  Future<JsonObject> getRegistry() {
    return context.backendClient.protocol.getProtocolRegistry();
  }
}
