import type { Identifier, JsonObject } from '../types/common.js';
import type { HttpClient } from '../http/client.js';

function encodeIdentifier(id: Identifier): string {
  return encodeURIComponent(String(id));
}

export interface NodesApi {
  activateNode(nodeId: Identifier): Promise<JsonObject>;
  drainNode(nodeId: Identifier): Promise<JsonObject>;
  migrateNodeRoutes(nodeId: Identifier, body: JsonObject): Promise<JsonObject>;
}

export function createNodesApi(httpClient: HttpClient): NodesApi {
  return {
    activateNode(nodeId) {
      return httpClient.post<JsonObject>(
        `/api/v1/control/nodes/${encodeIdentifier(nodeId)}/activate`,
      );
    },
    drainNode(nodeId) {
      return httpClient.post<JsonObject>(
        `/api/v1/control/nodes/${encodeIdentifier(nodeId)}/drain`,
      );
    },
    migrateNodeRoutes(nodeId, body) {
      return httpClient.post<JsonObject>(
        `/api/v1/control/nodes/${encodeIdentifier(nodeId)}/routes/migrate`,
        body,
      );
    },
  };
}
