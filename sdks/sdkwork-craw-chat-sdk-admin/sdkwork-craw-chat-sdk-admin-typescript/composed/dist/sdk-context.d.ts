import type { CrawChatAdminBackendClientLike, CrawChatSdkAdminClientCreateOptions, SdkworkBackendConfig } from './types.js';
export declare function createGeneratedBackendClient(backendConfig: SdkworkBackendConfig): Promise<CrawChatAdminBackendClientLike>;
export declare function resolveBackendClient(options: CrawChatSdkAdminClientCreateOptions): Promise<CrawChatAdminBackendClientLike>;
export declare class CrawChatSdkAdminContext {
    readonly backendClient: CrawChatAdminBackendClientLike;
    constructor(backendClient: CrawChatAdminBackendClientLike);
    setAuthToken(token: string): void;
    setAccessToken(token: string): void;
    setApiKey(apiKey: string): void;
}
//# sourceMappingURL=sdk-context.d.ts.map