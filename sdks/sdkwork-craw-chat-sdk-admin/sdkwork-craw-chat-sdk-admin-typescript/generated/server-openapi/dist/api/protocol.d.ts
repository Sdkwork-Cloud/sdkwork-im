import type { JsonObject } from '../types/common.js';
import type { HttpClient } from '../http/client.js';
export interface ProtocolApi {
    getProtocolGovernance(): Promise<JsonObject>;
    getProtocolRegistry(): Promise<JsonObject>;
}
export declare function createProtocolApi(httpClient: HttpClient): ProtocolApi;
//# sourceMappingURL=protocol.d.ts.map