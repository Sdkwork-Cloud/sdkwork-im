import type { HttpClient } from '../http/client';
import type { GovernanceRetrieveResponse } from '../types';
export declare class AutomationGovernanceApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve automation governance */
    retrieve(): Promise<GovernanceRetrieveResponse>;
}
export declare class AutomationApi {
    private client;
    readonly governance: AutomationGovernanceApi;
    constructor(client: HttpClient);
}
export declare function createAutomationApi(client: HttpClient): AutomationApi;
//# sourceMappingURL=automation.d.ts.map