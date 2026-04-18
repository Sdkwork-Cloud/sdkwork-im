import type { CrawChatAdminBackendClientLike, CrawChatAdminSdkClientCreateOptions, CrawChatAdminBackendConfig } from './types.js';
export declare function createGeneratedBackendClient(backendConfig: CrawChatAdminBackendConfig): Promise<CrawChatAdminBackendClientLike>;
export declare function resolveBackendClient(options: CrawChatAdminSdkClientCreateOptions): Promise<CrawChatAdminBackendClientLike>;
export declare class CrawChatAdminSdkContext {
    readonly backendClient: CrawChatAdminBackendClientLike;
    constructor(backendClient: CrawChatAdminBackendClientLike);
    setAuthToken(token: string): void;
}
//# sourceMappingURL=sdk-context.d.ts.map