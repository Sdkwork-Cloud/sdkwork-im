import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';


export class ProvidersApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Get provider-bindings */
  async getApiV1ControlProviderBindings(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/provider-bindings`));
  }

/** Post provider-bindings */
  async postApiV1ControlProviderBindings(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/provider-bindings`));
  }

/** Get provider-policies */
  async getApiV1ControlProviderPolicies(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/provider-policies`));
  }

/** Get provider-policies diff */
  async getApiV1ControlProviderPoliciesDiff(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/provider-policies/diff`));
  }

/** Post provider-policies preview */
  async postApiV1ControlProviderPoliciesPreview(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/provider-policies/preview`));
  }

/** Post provider-policies rollback */
  async postApiV1ControlProviderPoliciesRollback(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/provider-policies/rollback`));
  }

/** Get provider registry snapshot */
  async getApiV1ControlProviderRegistry(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/provider-registry`));
  }
}

export function createProvidersApi(client: HttpClient): ProvidersApi {
  return new ProvidersApi(client);
}
