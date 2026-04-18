import type { HttpClient } from '../http/client';
export declare class ProtocolApi {
    private client;
    constructor(client: HttpClient);
    /** Get protocol governance snapshot */
    getApiV1ControlProtocolGovernance(): Promise<void>;
    /** Get protocol registry snapshot */
    getApiV1ControlProtocolRegistry(): Promise<void>;
}
export declare function createProtocolApi(client: HttpClient): ProtocolApi;
//# sourceMappingURL=protocol.d.ts.map