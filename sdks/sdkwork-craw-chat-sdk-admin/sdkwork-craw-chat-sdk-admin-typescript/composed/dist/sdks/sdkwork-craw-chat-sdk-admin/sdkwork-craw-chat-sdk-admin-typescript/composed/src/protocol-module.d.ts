import type { JsonObject } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';
export declare class CrawChatAdminProtocolModule {
    private readonly context;
    constructor(context: CrawChatAdminSdkContext);
    getGovernance(): Promise<JsonObject>;
    getRegistry(): Promise<JsonObject>;
}
//# sourceMappingURL=protocol-module.d.ts.map