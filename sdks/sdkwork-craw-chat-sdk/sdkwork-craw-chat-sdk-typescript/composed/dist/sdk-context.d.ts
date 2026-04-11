import type { CrawChatBackendClientLike, CrawChatClientCreateOptions, SdkworkBackendConfig } from './types.js';
export declare function createGeneratedBackendClient(backendConfig: SdkworkBackendConfig): Promise<CrawChatBackendClientLike>;
export declare function resolveBackendClient(options: CrawChatClientCreateOptions): Promise<CrawChatBackendClientLike>;
export declare class CrawChatSdkContext {
    readonly backendClient: CrawChatBackendClientLike;
    constructor(backendClient: CrawChatBackendClientLike);
    setAuthToken(token: string): void;
}
//# sourceMappingURL=sdk-context.d.ts.map