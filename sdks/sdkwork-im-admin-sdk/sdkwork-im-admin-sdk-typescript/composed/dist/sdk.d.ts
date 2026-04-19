import type { ImAdminBackendClientLike, ImAdminSdkClientCreateOptions, ImAdminSdkClientOptions } from './types.js';
export declare class ImAdminSdkClient {
    private readonly context;
    readonly backendClient: ImAdminBackendClientLike;
    readonly auth: ImAdminBackendClientLike['auth'];
    readonly users: ImAdminBackendClientLike['users'];
    readonly marketing: ImAdminBackendClientLike['marketing'];
    readonly tenants: ImAdminBackendClientLike['tenants'];
    readonly access: ImAdminBackendClientLike['access'];
    readonly routing: ImAdminBackendClientLike['routing'];
    readonly catalog: ImAdminBackendClientLike['catalog'];
    readonly usage: ImAdminBackendClientLike['usage'];
    readonly billing: ImAdminBackendClientLike['billing'];
    readonly operations: ImAdminBackendClientLike['operations'];
    readonly storage: ImAdminBackendClientLike['storage'];
    constructor(options: ImAdminSdkClientOptions);
    static create(options: ImAdminSdkClientCreateOptions): Promise<ImAdminSdkClient>;
    setAuthToken(token: string): this;
}
export declare function createImAdminSdkClient(options: ImAdminSdkClientCreateOptions): Promise<ImAdminSdkClient>;
//# sourceMappingURL=sdk.d.ts.map