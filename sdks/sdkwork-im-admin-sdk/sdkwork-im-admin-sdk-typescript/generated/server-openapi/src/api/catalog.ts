import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';

export class CatalogApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }

  async listChannels(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/channels`));
  }

  async saveChannel(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/channels`), body);
  }

  async deleteChannel(channelId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/channels/${encodeURIComponent(String(channelId))}`));
  }

  async listProviders(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/providers`));
  }

  async saveProvider(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/providers`), body);
  }

  async deleteProvider(providerId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/providers/${encodeURIComponent(String(providerId))}`));
  }

  async listCredentials(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/credentials`));
  }

  async saveCredential(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/credentials`), body);
  }

  async deleteCredential(tenantId: string | number, providerId: string | number, keyReference: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/credentials/${encodeURIComponent(String(tenantId))}/providers/${encodeURIComponent(String(providerId))}/keys/${encodeURIComponent(String(keyReference))}`));
  }

  async listModels(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/models`));
  }

  async saveModel(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/models`), body);
  }

  async deleteModel(externalName: string | number, providerId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/models/${encodeURIComponent(String(externalName))}/providers/${encodeURIComponent(String(providerId))}`));
  }

  async listChannelModels(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/channel-models`));
  }

  async saveChannelModel(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/channel-models`), body);
  }

  async deleteChannelModel(channelId: string | number, modelId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/channel-models/${encodeURIComponent(String(channelId))}/models/${encodeURIComponent(String(modelId))}`));
  }

  async listModelPrices(): Promise<LooseJsonValue> {
    return this.client.get<LooseJsonValue>(backendApiPath(`/api/admin/model-prices`));
  }

  async saveModelPrice(body: LooseJsonObject): Promise<LooseJsonValue> {
    return this.client.post<LooseJsonValue>(backendApiPath(`/api/admin/model-prices`), body);
  }

  async deleteModelPrice(channelId: string | number, modelId: string | number, proxyProviderId: string | number): Promise<LooseJsonValue> {
    return this.client.delete<LooseJsonValue>(backendApiPath(`/api/admin/model-prices/${encodeURIComponent(String(channelId))}/models/${encodeURIComponent(String(modelId))}/providers/${encodeURIComponent(String(proxyProviderId))}`));
  }
}

export function createCatalogApi(client: HttpClient): CatalogApi {
  return new CatalogApi(client);
}
