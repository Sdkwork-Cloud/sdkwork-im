import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class AccessApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listApiKeys(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/api-keys`));
  }

  async createApiKey(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/api-keys`), body);
  }

  async updateApiKey(hashedKey: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.put<LooseJsonValue>(backendApiPath(`/api/admin/api-keys/${encodeURIComponent(String(hashedKey))}`), body);
  }

  async deleteApiKey(hashedKey: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/api-keys/${encodeURIComponent(String(hashedKey))}`));
  }

  async updateApiKeyStatus(hashedKey: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/api-keys/${encodeURIComponent(String(hashedKey))}/status`), body);
  }

  async listApiKeyGroups(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/api-key-groups`));
  }

  async createApiKeyGroup(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/api-key-groups`), body);
  }

  async updateApiKeyGroup(groupId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.patch<LooseJsonValue>(backendApiPath(`/api/admin/api-key-groups/${encodeURIComponent(String(groupId))}`), body);
  }

  async deleteApiKeyGroup(groupId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/api-key-groups/${encodeURIComponent(String(groupId))}`));
  }

  async updateApiKeyGroupStatus(groupId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/api-key-groups/${encodeURIComponent(String(groupId))}/status`), body);
  }
}

export function createAccessApi(client: HttpClient): AccessApi {
  return new AccessApi(client);
}
