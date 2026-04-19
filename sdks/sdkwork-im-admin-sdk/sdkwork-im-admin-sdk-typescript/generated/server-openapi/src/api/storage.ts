import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class StorageApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listStorageProviders(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/storage/providers`));
  }

  async getGlobalStorageConfig(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/storage/config`));
  }

  async saveGlobalStorageConfig(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/storage/config`), body);
  }

  async getTenantStorageConfig(tenantId: string | number): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/storage/config/tenants/${encodeURIComponent(String(tenantId))}`));
  }

  async saveTenantStorageConfig(tenantId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/storage/config/tenants/${encodeURIComponent(String(tenantId))}`), body);
  }

  async deleteTenantStorageConfig(tenantId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/storage/config/tenants/${encodeURIComponent(String(tenantId))}`));
  }

  async getTenantEffectiveStorageConfig(tenantId: string | number): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/storage/effective/tenants/${encodeURIComponent(String(tenantId))}`));
  }

  async validateGlobalStorageConfig(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/storage/validate`), body);
  }

  async validateTenantStorageConfig(tenantId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/storage/validate/tenants/${encodeURIComponent(String(tenantId))}`), body);
  }

  async listStorageAuditTrail(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/storage/audit`));
  }
}

export function createStorageApi(client: HttpClient): StorageApi {
  return new StorageApi(client);
}
