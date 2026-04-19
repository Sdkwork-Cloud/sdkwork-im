import 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';

import 'context.dart';
import 'types.dart';

class ControlPlaneProvidersModule {
  final ControlPlaneSdkContext context;

  ControlPlaneProvidersModule(this.context);

  Future<JsonObject> getBindings([ControlPlaneQueryParams? params]) {
    return context.backendClient.providers.getProviderBindings(params);
  }

  Future<JsonObject> upsertBindingPolicy(JsonObject body) {
    return context.backendClient.providers.upsertProviderBindingPolicy(body);
  }

  Future<JsonObject> getPolicyHistory() {
    return context.backendClient.providers.getProviderPolicyHistory();
  }

  Future<JsonObject> getPolicyDiff(ControlPlaneQueryParams params) {
    return context.backendClient.providers.getProviderPolicyDiff(params);
  }

  Future<JsonObject> previewPolicy(JsonObject body) {
    return context.backendClient.providers.previewProviderPolicy(body);
  }

  Future<JsonObject> rollbackPolicy(JsonObject body) {
    return context.backendClient.providers.rollbackProviderPolicy(body);
  }

  Future<JsonObject> getRegistry() {
    return context.backendClient.providers.getProviderRegistry();
  }
}
