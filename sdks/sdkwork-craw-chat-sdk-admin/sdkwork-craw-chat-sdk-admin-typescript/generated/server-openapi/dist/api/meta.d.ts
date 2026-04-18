import type { JsonObject } from '../types/common.js';
import type { HttpClient } from '../http/client.js';
export interface MetaApi {
    getHealthz(): Promise<JsonObject>;
}
export declare function createMetaApi(httpClient: HttpClient): MetaApi;
//# sourceMappingURL=meta.d.ts.map