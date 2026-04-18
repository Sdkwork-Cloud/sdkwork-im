import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

import 'context.dart';

class CrawChatAdminMetaModule {
  final CrawChatAdminSdkContext context;

  CrawChatAdminMetaModule(this.context);

  Future<JsonObject> health() {
    return context.backendClient.meta.getHealthz();
  }
}
