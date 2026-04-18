import type { Identifier, JsonObject } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';
export declare class CrawChatAdminNodesModule {
    private readonly context;
    constructor(context: CrawChatAdminSdkContext);
    activate(nodeId: Identifier): Promise<JsonObject>;
    drain(nodeId: Identifier): Promise<JsonObject>;
    migrateRoutes(nodeId: Identifier, body: JsonObject): Promise<JsonObject>;
}
//# sourceMappingURL=nodes-module.d.ts.map