import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { PresenceDeviceRequest, PresenceSnapshotView, ResumeSessionRequest, SessionResumeView } from '../types';


export class SessionApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Resume the current app session */
  async resume(body: ResumeSessionRequest): Promise<SessionResumeView> {
    return this.client.post<SessionResumeView>(backendApiPath(`/sessions/resume`), body, undefined, undefined, 'application/json');
  }

/** Disconnect the current app session device route */
  async disconnect(body: PresenceDeviceRequest): Promise<PresenceSnapshotView> {
    return this.client.post<PresenceSnapshotView>(backendApiPath(`/sessions/disconnect`), body, undefined, undefined, 'application/json');
  }
}

export function createSessionApi(client: HttpClient): SessionApi {
  return new SessionApi(client);
}
