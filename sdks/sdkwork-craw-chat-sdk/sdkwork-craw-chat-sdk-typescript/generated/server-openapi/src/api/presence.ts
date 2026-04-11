import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { PresenceDeviceRequest, PresenceSnapshotView } from '../types';


export class PresenceApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Refresh device presence */
  async heartbeat(body: PresenceDeviceRequest): Promise<PresenceSnapshotView> {
    return this.client.post<PresenceSnapshotView>(backendApiPath(`/presence/heartbeat`), body, undefined, undefined, 'application/json');
  }

/** Get current presence */
  async getPresenceMe(): Promise<PresenceSnapshotView> {
    return this.client.get<PresenceSnapshotView>(backendApiPath(`/presence/me`));
  }
}

export function createPresenceApi(client: HttpClient): PresenceApi {
  return new PresenceApi(client);
}
