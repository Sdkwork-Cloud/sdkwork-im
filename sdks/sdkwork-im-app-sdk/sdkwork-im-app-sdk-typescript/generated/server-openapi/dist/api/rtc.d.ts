import type { HttpClient } from '../http/client';
import type { ProviderCallbacksCreateResponse, ProviderHealthRetrieveResponse } from '../types';
export declare class RtcProviderHealthApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve RTC provider health */
    retrieve(): Promise<ProviderHealthRetrieveResponse>;
}
export declare class RtcProviderCallbacksApi {
    private client;
    constructor(client: HttpClient);
    /** Map RTC provider callback */
    create(): Promise<ProviderCallbacksCreateResponse>;
}
export declare class RtcApi {
    private client;
    readonly providerCallbacks: RtcProviderCallbacksApi;
    readonly providerHealth: RtcProviderHealthApi;
    constructor(client: HttpClient);
}
export declare function createRtcApi(client: HttpClient): RtcApi;
//# sourceMappingURL=rtc.d.ts.map