import type { InboxResponse } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatInboxModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    list(): Promise<InboxResponse>;
}
//# sourceMappingURL=inbox-module.d.ts.map