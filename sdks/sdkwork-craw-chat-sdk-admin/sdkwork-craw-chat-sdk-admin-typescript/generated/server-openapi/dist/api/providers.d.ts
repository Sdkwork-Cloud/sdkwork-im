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
export declare function createProvidersApi(httpClient: HttpClient): ProvidersApi;
//# sourceMappingURL=providers.d.ts.map