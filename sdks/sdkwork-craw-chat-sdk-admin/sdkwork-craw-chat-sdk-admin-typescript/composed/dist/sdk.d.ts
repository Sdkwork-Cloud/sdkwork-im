import type { CrawChatAdminBackendClientLike, CrawChatSdkAdminClientCreateOptions, CrawChatSdkAdminClientOptions } from './types.js';
export declare class CrawChatSdkAdminClient {
    private readonly context;
    readonly backendClient: CrawChatAdminBackendClientLike;
    readonly protocol: CrawChatAdminBackendClientLike['protocol'];
    readonly providers: CrawChatAdminBackendClientLike['providers'];
    readonly cluster: CrawChatAdminBackendClientLike['cluster'];
    readonly social: CrawChatAdminBackendClientLike['social'];
    readonly system: CrawChatAdminBackendClientLike['system'];
    constructor(options: CrawChatSdkAdminClientOptions);
    static create(options: CrawChatSdkAdminClientCreateOptions): Promise<CrawChatSdkAdminClient>;
    setAuthToken(token: string): this;
    setAccessToken(token: string): this;
    setApiKey(apiKey: string): this;
}
export declare function createCrawChatSdkAdminClient(options: CrawChatSdkAdminClientCreateOptions): Promise<CrawChatSdkAdminClient>;
//# sourceMappingURL=sdk.d.ts.map