import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';
export declare class CatalogApi {
    private client;
    constructor(client: HttpClient);
    listChannels(): Promise<LooseJsonValue>;
    saveChannel(body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteChannel(channelId: string | number): Promise<LooseJsonValue>;
    listProviders(): Promise<LooseJsonValue>;
    saveProvider(body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteProvider(providerId: string | number): Promise<LooseJsonValue>;
    listCredentials(): Promise<LooseJsonValue>;
    saveCredential(body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteCredential(tenantId: string | number, providerId: string | number, keyReference: string | number): Promise<LooseJsonValue>;
    listModels(): Promise<LooseJsonValue>;
    saveModel(body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteModel(externalName: string | number, providerId: string | number): Promise<LooseJsonValue>;
    listChannelModels(): Promise<LooseJsonValue>;
    saveChannelModel(body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteChannelModel(channelId: string | number, modelId: string | number): Promise<LooseJsonValue>;
    listModelPrices(): Promise<LooseJsonValue>;
    saveModelPrice(body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteModelPrice(channelId: string | number, modelId: string | number, proxyProviderId: string | number): Promise<LooseJsonValue>;
}
export declare function createCatalogApi(client: HttpClient): CatalogApi;
//# sourceMappingURL=catalog.d.ts.map