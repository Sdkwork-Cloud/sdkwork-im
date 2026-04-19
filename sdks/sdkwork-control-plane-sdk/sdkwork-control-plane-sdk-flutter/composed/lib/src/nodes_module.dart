import 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';

import 'context.dart';

class ControlPlaneNodesModule {
  final ControlPlaneSdkContext context;

  ControlPlaneNodesModule(this.context);

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
