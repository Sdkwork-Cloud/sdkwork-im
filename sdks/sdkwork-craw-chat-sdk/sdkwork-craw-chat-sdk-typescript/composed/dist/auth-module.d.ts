import type { CrawChatAuthLoginRequest, CrawChatAuthLoginResult, CrawChatAuthSession } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatAuthModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    login(body: CrawChatAuthLoginRequest): Promise<CrawChatAuthLoginResult>;
    me(): Promise<CrawChatAuthSession>;
    useToken(token: string): void;
    clearToken(): void;
}
//# sourceMappingURL=auth-module.d.ts.map