import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { PortalSnapshot, PortalWorkspaceView } from '../types';


export class PortalApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Read the tenant portal home snapshot */
  async getHome(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(backendApiPath(`/portal/home`));
  }

/** Read the tenant portal sign-in snapshot */
  async getAuth(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(backendApiPath(`/portal/auth`));
  }

/** Read the current tenant workspace snapshot */
  async getWorkspace(): Promise<PortalWorkspaceView> {
    return this.client.get<PortalWorkspaceView>(backendApiPath(`/portal/workspace`));
  }

/** Read the tenant dashboard snapshot */
  async getDashboard(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(backendApiPath(`/portal/dashboard`));
  }

/** Read the tenant conversations snapshot */
  async getConversations(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(backendApiPath(`/portal/conversations`));
  }

/** Read the tenant realtime snapshot */
  async getRealtime(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(backendApiPath(`/portal/realtime`));
  }

/** Read the tenant media snapshot */
  async getMedia(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(backendApiPath(`/portal/media`));
  }

/** Read the tenant automation snapshot */
  async getAutomation(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(backendApiPath(`/portal/automation`));
  }

/** Read the tenant governance snapshot */
  async getGovernance(): Promise<PortalSnapshot> {
    return this.client.get<PortalSnapshot>(backendApiPath(`/portal/governance`));
  }
}

export function createPortalApi(client: HttpClient): PortalApi {
  return new PortalApi(client);
}
