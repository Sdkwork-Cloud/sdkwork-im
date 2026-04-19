import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class UsageApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listUsageRecords(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/usage/records`));
  }

  async getUsageSummary(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/usage/summary`));
  }
}

export function createUsageApi(client: HttpClient): UsageApi {
  return new UsageApi(client);
}
