import 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';

import 'context.dart';

class ControlPlaneMetaModule {
  final ControlPlaneSdkContext context;

  ControlPlaneMetaModule(this.context);

  Future<JsonObject> health() {
    return context.backendClient.meta.getHealthz();
  }
}
