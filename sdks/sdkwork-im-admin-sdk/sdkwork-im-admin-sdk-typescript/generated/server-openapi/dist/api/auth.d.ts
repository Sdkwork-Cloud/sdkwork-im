import type { HttpClient } from '../http/client';
import type { LooseJsonObject, LooseJsonValue } from '../types/common';
export declare class AuthApi {
    private client;
    constructor(client: HttpClient);
    loginAdminUser(body: LooseJsonObject): Promise<LooseJsonValue>;
    getAdminMe(): Promise<LooseJsonValue>;
}
export declare function createAuthApi(client: HttpClient): AuthApi;
//# sourceMappingURL=auth.d.ts.map