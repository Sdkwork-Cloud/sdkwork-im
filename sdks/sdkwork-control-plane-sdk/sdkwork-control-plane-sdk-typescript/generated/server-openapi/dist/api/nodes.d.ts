import type { Identifier, JsonObject } from '../types/common.js';
import type { HttpClient } from '../http/client.js';
export interface NodesApi {
    activateNode(nodeId: Identifier): Promise<JsonObject>;
    drainNode(nodeId: Identifier): Promise<JsonObject>;
    migrateNodeRoutes(nodeId: Identifier, body: JsonObject): Promise<JsonObject>;
}
export declare function createNodesApi(httpClient: HttpClient): NodesApi;
//# sourceMappingURL=nodes.d.ts.map