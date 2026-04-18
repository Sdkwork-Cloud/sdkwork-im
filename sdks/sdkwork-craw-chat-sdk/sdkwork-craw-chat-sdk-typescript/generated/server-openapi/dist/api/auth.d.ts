import type { HttpClient } from '../http/client';
import type { PortalLoginRequest, PortalLoginResponse, PortalMeResponse } from '../types';
export declare class AuthApi {
    private client;
    constructor(client: HttpClient);
    /** Sign in to the tenant portal */
    login(body: PortalLoginRequest): Promise<PortalLoginResponse>;
    /** Read the current portal session */
    me(): Promise<PortalMeResponse>;
}
export declare function createAuthApi(client: HttpClient): AuthApi;
//# sourceMappingURL=auth.d.ts.map