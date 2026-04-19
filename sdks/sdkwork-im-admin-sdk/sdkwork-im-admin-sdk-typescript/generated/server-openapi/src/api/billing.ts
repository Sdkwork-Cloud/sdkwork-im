import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class BillingApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listBillingEvents(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/billing/events`));
  }

  async getBillingEventSummary(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/billing/events/summary`));
  }

  async getBillingSummary(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/billing/summary`));
  }
}

export function createBillingApi(client: HttpClient): BillingApi {
  return new BillingApi(client);
}
