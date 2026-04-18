import type { HttpClient } from '../http/client';
import type { LooseJsonValue } from '../types/common';
export declare class BillingApi {
    private client;
    constructor(client: HttpClient);
    listBillingEvents(): Promise<LooseJsonValue>;
    getBillingEventSummary(): Promise<LooseJsonValue>;
    getBillingSummary(): Promise<LooseJsonValue>;
}
export declare function createBillingApi(client: HttpClient): BillingApi;
//# sourceMappingURL=billing.d.ts.map