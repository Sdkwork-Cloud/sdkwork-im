import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class UsersApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listOperatorUsers(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/users/operators`));
  }

  async saveOperatorUser(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/users/operators`), body);
  }

  async deleteOperatorUser(userId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/users/operators/${encodeURIComponent(String(userId))}`));
  }

  async updateOperatorUserStatus(userId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/users/operators/${encodeURIComponent(String(userId))}/status`), body);
  }

  async resetOperatorUserPassword(userId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/users/operators/${encodeURIComponent(String(userId))}/password`), body);
  }

  async listPortalUsers(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/users/portal`));
  }

  async savePortalUser(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/users/portal`), body);
  }

  async deletePortalUser(userId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/users/portal/${encodeURIComponent(String(userId))}`));
  }

  async updatePortalUserStatus(userId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/users/portal/${encodeURIComponent(String(userId))}/status`), body);
  }

  async resetPortalUserPassword(userId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/users/portal/${encodeURIComponent(String(userId))}/password`), body);
  }
}

export function createUsersApi(client: HttpClient): UsersApi {
  return new UsersApi(client);
}
