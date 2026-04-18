import type { CrawChatManagementBackendClientLike, CrawChatSdkManagementClientCreateOptions, SdkworkBackendConfig } from './types.js';
export declare function createGeneratedBackendClient(backendConfig: SdkworkBackendConfig): Promise<CrawChatManagementBackendClientLike>;
export declare function resolveBackendClient(options: CrawChatSdkManagementClientCreateOptions): Promise<CrawChatManagementBackendClientLike>;
export declare class CrawChatSdkManagementContext {
    readonly backendClient: CrawChatManagementBackendClientLike;
    constructor(backendClient: CrawChatManagementBackendClientLike);
    setAuthToken(token: string): void;
    setAccessToken(token: string): void;
    setApiKey(apiKey: string): void;
}
//# sourceMappingURL=sdk-context.d.ts.map