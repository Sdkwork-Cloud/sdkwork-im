import type { HttpClient } from '../http/client';
import type { LooseJsonValue } from '../types/common';
export declare class UsageApi {
    private client;
    constructor(client: HttpClient);
    listUsageRecords(): Promise<LooseJsonValue>;
    getUsageSummary(): Promise<LooseJsonValue>;
}
export declare function createUsageApi(client: HttpClient): UsageApi;
//# sourceMappingURL=usage.d.ts.map