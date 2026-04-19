import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';
export declare class MarketingApi {
    private client;
    constructor(client: HttpClient);
    listMarketingCampaigns(): Promise<LooseJsonValue>;
    saveMarketingCampaign(body: LooseJsonObject): Promise<LooseJsonValue>;
    updateMarketingCampaignStatus(marketingCampaignId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare function createMarketingApi(client: HttpClient): MarketingApi;
//# sourceMappingURL=marketing.d.ts.map