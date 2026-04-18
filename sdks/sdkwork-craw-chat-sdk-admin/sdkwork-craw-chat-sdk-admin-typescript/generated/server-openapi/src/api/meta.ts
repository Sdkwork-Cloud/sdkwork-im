import type { JsonObject } from '../types/common.js';
import type { HttpClient } from '../http/client.js';

export interface MetaApi {
  getHealthz(): Promise<JsonObject>;
}

export function createMetaApi(httpClient: HttpClient): MetaApi {
  return {
    getHealthz() {
      return httpClient.get<JsonObject>('/healthz');
    },
  };
}
