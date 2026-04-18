import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';


export class SystemApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Check control plane health */
  async getHealthz(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/healthz`));
  }
}

export function createSystemApi(client: HttpClient): SystemApi {
  return new SystemApi(client);
}
