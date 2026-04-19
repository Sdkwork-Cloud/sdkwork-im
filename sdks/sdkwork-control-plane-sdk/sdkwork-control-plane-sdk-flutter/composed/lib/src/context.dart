import 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';

class ControlPlaneSdkContext {
  final ControlPlaneBackendClient backendClient;

  ControlPlaneSdkContext(this.backendClient);

  void setAuthToken(String token) {
    backendClient.setAuthToken(token);
  }
}
