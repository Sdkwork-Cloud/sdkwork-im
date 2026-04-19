import type { ControlPlaneBackendConfig } from './types/common.js';
import { type MetaApi } from './api/meta.js';
import { type ProtocolApi } from './api/protocol.js';
import { type ProvidersApi } from './api/providers.js';
import { type SocialApi } from './api/social.js';
import { type SocialRuntimeApi } from './api/social-runtime.js';
import { type NodesApi } from './api/nodes.js';
export declare class ControlPlaneBackendClient {
    private readonly httpClient;
    readonly meta: MetaApi;
    readonly protocol: ProtocolApi;
    readonly providers: ProvidersApi;
    readonly social: SocialApi;
    readonly socialRuntime: SocialRuntimeApi;
    readonly nodes: NodesApi;
    constructor(config: ControlPlaneBackendConfig);
    setAuthToken(token: string): this;
}
export declare function createClient(config: ControlPlaneBackendConfig): ControlPlaneBackendClient;
//# sourceMappingURL=sdk.d.ts.map