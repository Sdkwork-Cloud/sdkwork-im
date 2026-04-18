import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';


export class ProtocolApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Get protocol governance snapshot */
  async getApiV1ControlProtocolGovernance(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/protocol-governance`));
  }

/** Get protocol registry snapshot */
  async getApiV1ControlProtocolRegistry(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/protocol-registry`));
  }
}

export function createProtocolApi(client: HttpClient): ProtocolApi {
  return new ProtocolApi(client);
}
