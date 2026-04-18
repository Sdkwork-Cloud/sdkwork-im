import type { CrawChatConnectOptions, CrawChatLiveConnection } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatLiveModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    connect(options?: CrawChatConnectOptions): Promise<CrawChatLiveConnection>;
}
//# sourceMappingURL=live-module.d.ts.map