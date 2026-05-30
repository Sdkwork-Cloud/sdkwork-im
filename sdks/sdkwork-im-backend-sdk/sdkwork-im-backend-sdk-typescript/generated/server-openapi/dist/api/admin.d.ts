import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types';
export declare class AdminUsageSummaryApi {
    private client;
    constructor(client: HttpClient);
    /** getUsageSummary */
    retrieve(): Promise<LooseJsonValue>;
}
export declare class AdminUsageRecordsApi {
    private client;
    constructor(client: HttpClient);
    /** listUsageRecords */
    list(): Promise<LooseJsonValue>;
}
export declare class AdminUsageApi {
    private client;
    readonly records: AdminUsageRecordsApi;
    readonly summary: AdminUsageSummaryApi;
    constructor(client: HttpClient);
}
export declare class AdminStorageValidationTenantsApi {
    private client;
    constructor(client: HttpClient);
    /** validateTenantStorageConfig */
    create(tenantId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminStorageValidationApi {
    private client;
    readonly tenants: AdminStorageValidationTenantsApi;
    constructor(client: HttpClient);
    /** validateGlobalStorageConfig */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminStorageProvidersApi {
    private client;
    constructor(client: HttpClient);
    /** listStorageProviders */
    list(): Promise<LooseJsonValue>;
}
export declare class AdminStorageEffectiveTenantsApi {
    private client;
    constructor(client: HttpClient);
    /** getTenantEffectiveStorageConfig */
    retrieve(tenantId: string | number): Promise<LooseJsonValue>;
}
export declare class AdminStorageEffectiveApi {
    private client;
    readonly tenants: AdminStorageEffectiveTenantsApi;
    constructor(client: HttpClient);
}
export declare class AdminStorageConfigTenantsApi {
    private client;
    constructor(client: HttpClient);
    /** getTenantStorageConfig */
    retrieve(tenantId: string | number): Promise<LooseJsonValue>;
    /** saveTenantStorageConfig */
    create(tenantId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    /** deleteTenantStorageConfig */
    delete(tenantId: string | number): Promise<LooseJsonValue>;
}
export declare class AdminStorageConfigApi {
    private client;
    readonly tenants: AdminStorageConfigTenantsApi;
    constructor(client: HttpClient);
    /** getGlobalStorageConfig */
    retrieve(): Promise<LooseJsonValue>;
    /** saveGlobalStorageConfig */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminStorageAuditApi {
    private client;
    constructor(client: HttpClient);
    /** listStorageAuditTrail */
    list(): Promise<LooseJsonValue>;
}
export declare class AdminStorageApi {
    private client;
    readonly audit: AdminStorageAuditApi;
    readonly config: AdminStorageConfigApi;
    readonly effective: AdminStorageEffectiveApi;
    readonly providers: AdminStorageProvidersApi;
    readonly validation: AdminStorageValidationApi;
    constructor(client: HttpClient);
}
export declare class AdminRoutingSnapshotsApi {
    private client;
    constructor(client: HttpClient);
    /** listCompiledRoutingSnapshots */
    list(): Promise<LooseJsonValue>;
}
export declare class AdminRoutingProfilesApi {
    private client;
    constructor(client: HttpClient);
    /** listRoutingProfiles */
    list(): Promise<LooseJsonValue>;
    /** createRoutingProfile */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminRoutingHealthSnapshotsApi {
    private client;
    constructor(client: HttpClient);
    /** listProviderHealthSnapshots */
    retrieve(): Promise<LooseJsonValue>;
}
export declare class AdminRoutingDecisionLogsApi {
    private client;
    constructor(client: HttpClient);
    /** listRoutingDecisionLogs */
    list(): Promise<LooseJsonValue>;
}
export declare class AdminRoutingApi {
    private client;
    readonly decisionLogs: AdminRoutingDecisionLogsApi;
    readonly healthSnapshots: AdminRoutingHealthSnapshotsApi;
    readonly profiles: AdminRoutingProfilesApi;
    readonly snapshots: AdminRoutingSnapshotsApi;
    constructor(client: HttpClient);
}
export declare class AdminProvidersApi {
    private client;
    constructor(client: HttpClient);
    /** listProviders */
    list(): Promise<LooseJsonValue>;
    /** saveProvider */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
    /** deleteProvider */
    delete(providerId: string | number): Promise<LooseJsonValue>;
}
export declare class AdminModelsProvidersApi {
    private client;
    constructor(client: HttpClient);
    /** deleteModel */
    delete(externalName: string | number, providerId: string | number): Promise<LooseJsonValue>;
}
export declare class AdminModelsApi {
    private client;
    readonly providers: AdminModelsProvidersApi;
    constructor(client: HttpClient);
    /** listModels */
    list(): Promise<LooseJsonValue>;
    /** saveModel */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminModelPricesModelsProvidersApi {
    private client;
    constructor(client: HttpClient);
    /** deleteModelPrice */
    delete(channelId: string | number, modelId: string | number, proxyProviderId: string | number): Promise<LooseJsonValue>;
}
export declare class AdminModelPricesModelsApi {
    private client;
    readonly providers: AdminModelPricesModelsProvidersApi;
    constructor(client: HttpClient);
}
export declare class AdminModelPricesApi {
    private client;
    readonly models: AdminModelPricesModelsApi;
    constructor(client: HttpClient);
    /** listModelPrices */
    list(): Promise<LooseJsonValue>;
    /** saveModelPrice */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminMarketingCampaignsApi {
    private client;
    constructor(client: HttpClient);
    /** listMarketingCampaigns */
    list(): Promise<LooseJsonValue>;
    /** saveMarketingCampaign */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
    /** updateMarketingCampaignStatus */
    status(marketingCampaignId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminMarketingApi {
    private client;
    readonly campaigns: AdminMarketingCampaignsApi;
    constructor(client: HttpClient);
}
export declare class AdminGatewayRateLimitWindowsApi {
    private client;
    constructor(client: HttpClient);
    /** listRateLimitWindows */
    list(): Promise<LooseJsonValue>;
}
export declare class AdminGatewayRateLimitPoliciesApi {
    private client;
    constructor(client: HttpClient);
    /** listRateLimitPolicies */
    list(): Promise<LooseJsonValue>;
    /** createRateLimitPolicy */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminGatewayApi {
    private client;
    readonly rateLimitPolicies: AdminGatewayRateLimitPoliciesApi;
    readonly rateLimitWindows: AdminGatewayRateLimitWindowsApi;
    constructor(client: HttpClient);
}
export declare class AdminExtensionsRuntimeStatusesApi {
    private client;
    constructor(client: HttpClient);
    /** listRuntimeStatuses */
    list(): Promise<LooseJsonValue>;
}
export declare class AdminExtensionsRuntimeReloadsApi {
    private client;
    constructor(client: HttpClient);
    /** reloadExtensionRuntimes */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminExtensionsApi {
    private client;
    readonly runtimeReloads: AdminExtensionsRuntimeReloadsApi;
    readonly runtimeStatuses: AdminExtensionsRuntimeStatusesApi;
    constructor(client: HttpClient);
}
export declare class AdminCredentialsProvidersKeysApi {
    private client;
    constructor(client: HttpClient);
    /** deleteCredential */
    delete(tenantId: string | number, providerId: string | number, keyReference: string | number): Promise<LooseJsonValue>;
}
export declare class AdminCredentialsProvidersApi {
    private client;
    readonly keys: AdminCredentialsProvidersKeysApi;
    constructor(client: HttpClient);
}
export declare class AdminCredentialsApi {
    private client;
    readonly providers: AdminCredentialsProvidersApi;
    constructor(client: HttpClient);
    /** listCredentials */
    list(): Promise<LooseJsonValue>;
    /** saveCredential */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminChannelsApi {
    private client;
    constructor(client: HttpClient);
    /** listChannels */
    list(): Promise<LooseJsonValue>;
    /** saveChannel */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
    /** deleteChannel */
    delete(channelId: string | number): Promise<LooseJsonValue>;
}
export declare class AdminChannelModelsModelsApi {
    private client;
    constructor(client: HttpClient);
    /** deleteChannelModel */
    delete(channelId: string | number, modelId: string | number): Promise<LooseJsonValue>;
}
export declare class AdminChannelModelsApi {
    private client;
    readonly models: AdminChannelModelsModelsApi;
    constructor(client: HttpClient);
    /** listChannelModels */
    list(): Promise<LooseJsonValue>;
    /** saveChannelModel */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminBillingSummaryApi {
    private client;
    constructor(client: HttpClient);
    /** getBillingSummary */
    retrieve(): Promise<LooseJsonValue>;
}
export declare class AdminBillingEventsSummaryApi {
    private client;
    constructor(client: HttpClient);
    /** getBillingEventSummary */
    retrieve(): Promise<LooseJsonValue>;
}
export declare class AdminBillingEventsApi {
    private client;
    readonly summary: AdminBillingEventsSummaryApi;
    constructor(client: HttpClient);
    /** listBillingEvents */
    list(): Promise<LooseJsonValue>;
}
export declare class AdminBillingApi {
    private client;
    readonly events: AdminBillingEventsApi;
    readonly summary: AdminBillingSummaryApi;
    constructor(client: HttpClient);
}
export declare class AdminApiKeysApi {
    private client;
    constructor(client: HttpClient);
    /** listApiKeys */
    list(): Promise<LooseJsonValue>;
    /** createApiKey */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
    /** updateApiKey */
    update(hashedKey: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    /** deleteApiKey */
    delete(hashedKey: string | number): Promise<LooseJsonValue>;
    /** updateApiKeyStatus */
    status(hashedKey: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminApiKeyGroupsApi {
    private client;
    constructor(client: HttpClient);
    /** listApiKeyGroups */
    list(): Promise<LooseJsonValue>;
    /** createApiKeyGroup */
    create(body: LooseJsonObject): Promise<LooseJsonValue>;
    /** updateApiKeyGroup */
    update(groupId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    /** deleteApiKeyGroup */
    delete(groupId: string | number): Promise<LooseJsonValue>;
    /** updateApiKeyGroupStatus */
    status(groupId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare class AdminApi {
    private client;
    readonly apiKeyGroups: AdminApiKeyGroupsApi;
    readonly apiKeys: AdminApiKeysApi;
    readonly billing: AdminBillingApi;
    readonly channelModels: AdminChannelModelsApi;
    readonly channels: AdminChannelsApi;
    readonly credentials: AdminCredentialsApi;
    readonly extensions: AdminExtensionsApi;
    readonly gateway: AdminGatewayApi;
    readonly marketing: AdminMarketingApi;
    readonly modelPrices: AdminModelPricesApi;
    readonly models: AdminModelsApi;
    readonly providers: AdminProvidersApi;
    readonly routing: AdminRoutingApi;
    readonly storage: AdminStorageApi;
    readonly usage: AdminUsageApi;
    constructor(client: HttpClient);
}
export declare function createAdminApi(client: HttpClient): AdminApi;
//# sourceMappingURL=admin.d.ts.map