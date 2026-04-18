import type { CrawChatAdminBackendConfig } from './types/common.js';
import { type MetaApi } from './api/meta.js';
import { type ProtocolApi } from './api/protocol.js';
import { type ProvidersApi } from './api/providers.js';
import { type SocialApi } from './api/social.js';
import { type SocialRuntimeApi } from './api/social-runtime.js';
import { type NodesApi } from './api/nodes.js';
export declare class CrawChatAdminBackendClient {
    private readonly httpClient;
    readonly meta: MetaApi;
    readonly protocol: ProtocolApi;
    readonly providers: ProvidersApi;
    readonly social: SocialApi;
    readonly socialRuntime: SocialRuntimeApi;
    readonly nodes: NodesApi;
    constructor(config: CrawChatAdminBackendConfig);
    setAuthToken(token: string): this;
}
export declare function createClient(config: CrawChatAdminBackendConfig): CrawChatAdminBackendClient;
//# sourceMappingURL=sdk.d.ts.map