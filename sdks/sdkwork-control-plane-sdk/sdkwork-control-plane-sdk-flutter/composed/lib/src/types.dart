import 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';

typedef ControlPlaneQueryParams = QueryParams;

class ControlPlaneSdkClientOptions {
  final ControlPlaneBackendClient backendClient;

  const ControlPlaneSdkClientOptions({
    required this.backendClient,
  });
}
