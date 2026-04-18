import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';
export declare class OperationsApi {
    private client;
    constructor(client: HttpClient);
    listRateLimitPolicies(): Promise<LooseJsonValue>;
    createRateLimitPolicy(body: LooseJsonObject): Promise<LooseJsonValue>;
    listRateLimitWindows(): Promise<LooseJsonValue>;
    listRuntimeStatuses(): Promise<LooseJsonValue>;
    reloadExtensionRuntimes(body: LooseJsonObject): Promise<LooseJsonValue>;
}
export declare function createOperationsApi(client: HttpClient): OperationsApi;
//# sourceMappingURL=operations.d.ts.map