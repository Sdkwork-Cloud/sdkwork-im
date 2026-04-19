import type { ControlPlaneBackendClientLike, ControlPlaneSdkClientCreateOptions, ControlPlaneBackendConfig } from './types.js';
export declare function createGeneratedBackendClient(backendConfig: ControlPlaneBackendConfig): Promise<ControlPlaneBackendClientLike>;
export declare function resolveBackendClient(options: ControlPlaneSdkClientCreateOptions): Promise<ControlPlaneBackendClientLike>;
export declare class ControlPlaneSdkContext {
    readonly backendClient: ControlPlaneBackendClientLike;
    constructor(backendClient: ControlPlaneBackendClientLike);
    setAuthToken(token: string): void;
}
//# sourceMappingURL=sdk-context.d.ts.map