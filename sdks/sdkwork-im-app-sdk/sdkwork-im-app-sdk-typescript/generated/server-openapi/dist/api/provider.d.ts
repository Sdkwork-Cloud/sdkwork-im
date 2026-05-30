import type { HttpClient } from '../http/client';
import type { MediaHealthRetrieveResponse, PrincipalProfileHealthRetrieveResponse } from '../types';
export declare class ProviderPrincipalProfileHealthApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve principal-profile provider health */
    retrieve(): Promise<PrincipalProfileHealthRetrieveResponse>;
}
export declare class ProviderMediaHealthApi {
    private client;
    constructor(client: HttpClient);
    /** Retrieve media provider health */
    retrieve(): Promise<MediaHealthRetrieveResponse>;
}
export declare class ProviderApi {
    private client;
    readonly mediaHealth: ProviderMediaHealthApi;
    readonly principalProfileHealth: ProviderPrincipalProfileHealthApi;
    constructor(client: HttpClient);
}
export declare function createProviderApi(client: HttpClient): ProviderApi;
//# sourceMappingURL=provider.d.ts.map