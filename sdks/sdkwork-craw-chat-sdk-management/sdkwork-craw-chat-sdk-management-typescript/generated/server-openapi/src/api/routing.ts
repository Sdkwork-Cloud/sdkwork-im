import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class RoutingApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listRoutingProfiles(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/routing/profiles`));
  }

  async createRoutingProfile(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/routing/profiles`), body);
  }

  async listCompiledRoutingSnapshots(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/routing/snapshots`));
  }

  async listRoutingDecisionLogs(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/routing/decision-logs`));
  }

  async listProviderHealthSnapshots(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/routing/health-snapshots`));
  }
}

export function createRoutingApi(client: HttpClient): RoutingApi {
  return new RoutingApi(client);
}
