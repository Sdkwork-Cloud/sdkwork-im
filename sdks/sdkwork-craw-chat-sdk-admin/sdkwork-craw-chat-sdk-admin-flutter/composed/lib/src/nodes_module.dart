import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

import 'context.dart';

class CrawChatAdminNodesModule {
  final CrawChatAdminSdkContext context;

  CrawChatAdminNodesModule(this.context);

  Future<JsonObject> activate(Identifier nodeId) {
    return context.backendClient.nodes.activateNode(nodeId);
  }

  Future<JsonObject> drain(Identifier nodeId) {
    return context.backendClient.nodes.drainNode(nodeId);
  }

  Future<JsonObject> migrateRoutes(Identifier nodeId, JsonObject body) {
    return context.backendClient.nodes.migrateNodeRoutes(nodeId, body);
  }
}
