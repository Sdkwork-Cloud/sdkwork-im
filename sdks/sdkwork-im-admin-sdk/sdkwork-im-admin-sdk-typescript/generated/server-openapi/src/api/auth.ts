import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class AuthApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async loginAdminUser(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/auth/login`), body);
  }

  async getAdminMe(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/auth/me`));
  }
}

export function createAuthApi(client: HttpClient): AuthApi {
  return new AuthApi(client);
}
