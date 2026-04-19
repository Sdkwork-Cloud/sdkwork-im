import type { JsonObject, QueryParams } from './types.js';
import type { ControlPlaneSdkContext } from './sdk-context.js';

export class ControlPlaneProvidersModule {
  constructor(private readonly context: ControlPlaneSdkContext) {}

  getBindings(params?: QueryParams): Promise<JsonObject> {
    return this.context.backendClient.providers.getProviderBindings(params);
  }

  upsertBindingPolicy(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.providers.upsertProviderBindingPolicy(body);
  }

  getPolicyHistory(): Promise<JsonObject> {
    return this.context.backendClient.providers.getProviderPolicyHistory();
  }

  getPolicyDiff(params: QueryParams): Promise<JsonObject> {
    return this.context.backendClient.providers.getProviderPolicyDiff(params);
  }

  previewPolicy(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.providers.previewProviderPolicy(body);
  }

  rollbackPolicy(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.providers.rollbackProviderPolicy(body);
  }

  getRegistry(): Promise<JsonObject> {
    return this.context.backendClient.providers.getProviderRegistry();
  }
}
