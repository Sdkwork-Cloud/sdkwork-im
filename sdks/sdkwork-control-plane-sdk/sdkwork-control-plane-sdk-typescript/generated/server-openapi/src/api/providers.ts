import type { JsonObject, QueryParams } from '../types/common.js';
import type { HttpClient } from '../http/client.js';

export interface ProvidersApi {
  getProviderBindings(params?: QueryParams): Promise<JsonObject>;
  upsertProviderBindingPolicy(body: JsonObject): Promise<JsonObject>;
  getProviderPolicyHistory(): Promise<JsonObject>;
  getProviderPolicyDiff(params: QueryParams): Promise<JsonObject>;
  previewProviderPolicy(body: JsonObject): Promise<JsonObject>;
  rollbackProviderPolicy(body: JsonObject): Promise<JsonObject>;
  getProviderRegistry(): Promise<JsonObject>;
}

export function createProvidersApi(httpClient: HttpClient): ProvidersApi {
  return {
    getProviderBindings(params) {
      return httpClient.get<JsonObject>('/api/v1/control/provider-bindings', params);
    },
    upsertProviderBindingPolicy(body) {
      return httpClient.post<JsonObject>('/api/v1/control/provider-bindings', body);
    },
    getProviderPolicyHistory() {
      return httpClient.get<JsonObject>('/api/v1/control/provider-policies');
    },
    getProviderPolicyDiff(params) {
      return httpClient.get<JsonObject>('/api/v1/control/provider-policies/diff', params);
    },
    previewProviderPolicy(body) {
      return httpClient.post<JsonObject>('/api/v1/control/provider-policies/preview', body);
    },
    rollbackProviderPolicy(body) {
      return httpClient.post<JsonObject>('/api/v1/control/provider-policies/rollback', body);
    },
    getProviderRegistry() {
      return httpClient.get<JsonObject>('/api/v1/control/provider-registry');
    },
  };
}
