import { CrawChatAdminMetaModule } from './meta-module.js';
import { CrawChatAdminNodesModule } from './nodes-module.js';
import { CrawChatAdminProtocolModule } from './protocol-module.js';
import { CrawChatAdminProvidersModule } from './providers-module.js';
import { CrawChatAdminSocialModule } from './social-module.js';
import { CrawChatAdminSocialRuntimeModule } from './social-runtime-module.js';
import type { CrawChatAdminBackendClientLike, CrawChatAdminSdkClientCreateOptions, CrawChatAdminSdkClientOptions } from './types.js';
export declare class CrawChatAdminSdkClient {
    private readonly context;
    readonly backendClient: CrawChatAdminBackendClientLike;
    readonly meta: CrawChatAdminMetaModule;
    readonly protocol: CrawChatAdminProtocolModule;
    readonly providers: CrawChatAdminProvidersModule;
    readonly social: CrawChatAdminSocialModule;
    readonly socialRuntime: CrawChatAdminSocialRuntimeModule;
    readonly nodes: CrawChatAdminNodesModule;
    constructor(options: CrawChatAdminSdkClientOptions);
    static create(options: CrawChatAdminSdkClientCreateOptions): Promise<CrawChatAdminSdkClient>;
    setAuthToken(token: string): this;
}
export declare function createCrawChatAdminSdkClient(options: CrawChatAdminSdkClientCreateOptions): Promise<CrawChatAdminSdkClient>;
//# sourceMappingURL=sdk.d.ts.map