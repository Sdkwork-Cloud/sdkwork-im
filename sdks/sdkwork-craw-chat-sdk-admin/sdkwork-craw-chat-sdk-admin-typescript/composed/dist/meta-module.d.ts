import type { JsonObject } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';
export declare class CrawChatAdminMetaModule {
    private readonly context;
    constructor(context: CrawChatAdminSdkContext);
    health(): Promise<JsonObject>;
}
//# sourceMappingURL=meta-module.d.ts.map