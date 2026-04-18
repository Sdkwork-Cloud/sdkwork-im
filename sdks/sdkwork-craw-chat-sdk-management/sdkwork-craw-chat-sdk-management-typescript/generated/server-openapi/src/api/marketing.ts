import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class MarketingApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listMarketingCampaigns(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/marketing/campaigns`));
  }

  async saveMarketingCampaign(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/marketing/campaigns`), body);
  }

  async updateMarketingCampaignStatus(marketingCampaignId: string | number, body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/marketing/campaigns/${encodeURIComponent(String(marketingCampaignId))}/status`), body);
  }
}

export function createMarketingApi(client: HttpClient): MarketingApi {
  return new MarketingApi(client);
}
