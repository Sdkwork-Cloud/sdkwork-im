import { backendApiPath } from './paths.js';
import type { HttpClient } from '../http/client.js';
import type { QueryParams } from '../types/common.js';
import type { PortalLoginRequest, PortalLoginResponse, PortalMeResponse } from '../types/index.js';


export class AuthApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Sign in to the tenant portal */
  async login(body: PortalLoginRequest): Promise<PortalLoginResponse> {
    return this.client.post<PortalLoginResponse>(backendApiPath(`/auth/login`), body, undefined, undefined, 'application/json');
  }

/** Read the current portal session */
  async me(): Promise<PortalMeResponse> {
    return this.client.get<PortalMeResponse>(backendApiPath(`/auth/me`));
  }
}

export function createAuthApi(client: HttpClient): AuthApi {
  return new AuthApi(client);
}
