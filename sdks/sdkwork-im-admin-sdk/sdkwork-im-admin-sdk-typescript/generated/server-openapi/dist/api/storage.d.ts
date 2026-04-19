import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';
export declare class StorageApi {
    private client;
    constructor(client: HttpClient);
    listStorageProviders(): Promise<LooseJsonValue>;
    getGlobalStorageConfig(): Promise<LooseJsonValue>;
    saveGlobalStorageConfig(body: LooseJsonObject): Promise<LooseJsonValue>;
    getTenantStorageConfig(tenantId: string | number): Promise<LooseJsonValue>;
    saveTenantStorageConfig(tenantId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    deleteTenantStorageConfig(tenantId: string | number): Promise<LooseJsonValue>;
    getTenantEffectiveStorageConfig(tenantId: string | number): Promise<LooseJsonValue>;
    validateGlobalStorageConfig(body: LooseJsonObject): Promise<LooseJsonValue>;
    validateTenantStorageConfig(tenantId: string | number, body: LooseJsonObject): Promise<LooseJsonValue>;
    listStorageAuditTrail(): Promise<LooseJsonValue>;
}
export declare function createStorageApi(client: HttpClient): StorageApi;
//# sourceMappingURL=storage.d.ts.map