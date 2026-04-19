import 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';

import 'context.dart';

class ControlPlaneProtocolModule {
  final ControlPlaneSdkContext context;

  ControlPlaneProtocolModule(this.context);

  Future<JsonObject> getGovernance() {
    return context.backendClient.protocol.getProtocolGovernance();
  }

  Future<JsonObject> getRegistry() {
    return context.backendClient.protocol.getProtocolRegistry();
  }
}
