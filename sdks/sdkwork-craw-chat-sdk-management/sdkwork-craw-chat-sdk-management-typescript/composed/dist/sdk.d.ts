import type { CrawChatManagementBackendClientLike, CrawChatSdkManagementClientCreateOptions, CrawChatSdkManagementClientOptions } from './types.js';
export declare class CrawChatSdkManagementClient {
    private readonly context;
    readonly backendClient: CrawChatManagementBackendClientLike;
    readonly auth: CrawChatManagementBackendClientLike['auth'];
    readonly users: CrawChatManagementBackendClientLike['users'];
    readonly marketing: CrawChatManagementBackendClientLike['marketing'];
    readonly tenants: CrawChatManagementBackendClientLike['tenants'];
    readonly access: CrawChatManagementBackendClientLike['access'];
    readonly routing: CrawChatManagementBackendClientLike['routing'];
    readonly catalog: CrawChatManagementBackendClientLike['catalog'];
    readonly usage: CrawChatManagementBackendClientLike['usage'];
    readonly billing: CrawChatManagementBackendClientLike['billing'];
    readonly operations: CrawChatManagementBackendClientLike['operations'];
    constructor(options: CrawChatSdkManagementClientOptions);
    static create(options: CrawChatSdkManagementClientCreateOptions): Promise<CrawChatSdkManagementClient>;
    setAuthToken(token: string): this;
    setAccessToken(token: string): this;
    setApiKey(apiKey: string): this;
}
export declare function createCrawChatSdkManagementClient(options: CrawChatSdkManagementClientCreateOptions): Promise<CrawChatSdkManagementClient>;
//# sourceMappingURL=sdk.d.ts.map