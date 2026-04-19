import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';
export declare class RoutingApi {
    private client;
    constructor(client: HttpClient);
    listRoutingProfiles(): Promise<LooseJsonValue>;
    createRoutingProfile(body: LooseJsonObject): Promise<LooseJsonValue>;
    listCompiledRoutingSnapshots(): Promise<LooseJsonValue>;
    listRoutingDecisionLogs(): Promise<LooseJsonValue>;
    listProviderHealthSnapshots(): Promise<LooseJsonValue>;
}
export declare function createRoutingApi(client: HttpClient): RoutingApi;
//# sourceMappingURL=routing.d.ts.map