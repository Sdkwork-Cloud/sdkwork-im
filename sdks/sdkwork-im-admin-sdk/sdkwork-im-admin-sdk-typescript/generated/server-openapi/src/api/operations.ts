import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class OperationsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listRateLimitPolicies(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/gateway/rate-limit-policies`));
  }

  async createRateLimitPolicy(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/gateway/rate-limit-policies`), body);
  }

  async listRateLimitWindows(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/gateway/rate-limit-windows`));
  }

  async listRuntimeStatuses(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/extensions/runtime-statuses`));
  }

  async reloadExtensionRuntimes(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/extensions/runtime-reloads`), body);
  }
}

export function createOperationsApi(client: HttpClient): OperationsApi {
  return new OperationsApi(client);
}
