import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class TenantsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listTenants(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/tenants`));
  }

  async saveTenant(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/tenants`), body);
  }

  async deleteTenant(tenantId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/tenants/${encodeURIComponent(String(tenantId))}`));
  }

  async listProjects(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/projects`));
  }

  async saveProject(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/projects`), body);
  }

  async deleteProject(projectId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/projects/${encodeURIComponent(String(projectId))}`));
  }
}

export function createTenantsApi(client: HttpClient): TenantsApi {
  return new TenantsApi(client);
}
