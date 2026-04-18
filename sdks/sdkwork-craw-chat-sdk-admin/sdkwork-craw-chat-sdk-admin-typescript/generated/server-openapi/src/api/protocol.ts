import type { JsonObject } from '../types/common.js';
import type { HttpClient } from '../http/client.js';

export interface ProtocolApi {
  getProtocolGovernance(): Promise<JsonObject>;
  getProtocolRegistry(): Promise<JsonObject>;
}

export function createProtocolApi(httpClient: HttpClient): ProtocolApi {
  return {
    getProtocolGovernance() {
      return httpClient.get<JsonObject>('/api/v1/control/protocol-governance');
    },
    getProtocolRegistry() {
      return httpClient.get<JsonObject>('/api/v1/control/protocol-registry');
    },
  };
}
