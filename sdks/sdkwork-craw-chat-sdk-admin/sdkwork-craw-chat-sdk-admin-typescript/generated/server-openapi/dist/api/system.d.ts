import type { HttpClient } from '../http/client';
export declare class SystemApi {
    private client;
    constructor(client: HttpClient);
    /** Check control plane health */
    getHealthz(): Promise<void>;
}
export declare function createSystemApi(client: HttpClient): SystemApi;
//# sourceMappingURL=system.d.ts.map