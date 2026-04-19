import { ControlPlaneMetaModule } from './meta-module.js';
import { ControlPlaneNodesModule } from './nodes-module.js';
import { ControlPlaneProtocolModule } from './protocol-module.js';
import { ControlPlaneProvidersModule } from './providers-module.js';
import { ControlPlaneSocialModule } from './social-module.js';
import { ControlPlaneSocialRuntimeModule } from './social-runtime-module.js';
import type { ControlPlaneBackendClientLike, ControlPlaneSdkClientCreateOptions, ControlPlaneSdkClientOptions } from './types.js';
export declare class ControlPlaneSdkClient {
    private readonly context;
    readonly backendClient: ControlPlaneBackendClientLike;
    readonly meta: ControlPlaneMetaModule;
    readonly protocol: ControlPlaneProtocolModule;
    readonly providers: ControlPlaneProvidersModule;
    readonly social: ControlPlaneSocialModule;
    readonly socialRuntime: ControlPlaneSocialRuntimeModule;
    readonly nodes: ControlPlaneNodesModule;
    constructor(options: ControlPlaneSdkClientOptions);
    static create(options: ControlPlaneSdkClientCreateOptions): Promise<ControlPlaneSdkClient>;
    setAuthToken(token: string): this;
}
export declare function createControlPlaneSdkClient(options: ControlPlaneSdkClientCreateOptions): Promise<ControlPlaneSdkClient>;
//# sourceMappingURL=sdk.d.ts.map