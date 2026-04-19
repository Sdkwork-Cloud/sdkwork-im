import type { ImAdminBackendClientLike, ImAdminSdkClientCreateOptions, ImAdminBackendConfig } from './types.js';
export declare function createGeneratedBackendClient(backendConfig: ImAdminBackendConfig): Promise<ImAdminBackendClientLike>;
export declare function resolveBackendClient(options: ImAdminSdkClientCreateOptions): Promise<ImAdminBackendClientLike>;
export declare class ImAdminSdkContext {
    readonly backendClient: ImAdminBackendClientLike;
    constructor(backendClient: ImAdminBackendClientLike);
    setAuthToken(token: string): void;
}
//# sourceMappingURL=sdk-context.d.ts.map