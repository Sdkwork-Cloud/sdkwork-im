import type { JsonObject, QueryParams } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';
export declare class CrawChatAdminProvidersModule {
    private readonly context;
    constructor(context: CrawChatAdminSdkContext);
    getBindings(params?: QueryParams): Promise<JsonObject>;
    upsertBindingPolicy(body: JsonObject): Promise<JsonObject>;
    getPolicyHistory(): Promise<JsonObject>;
    getPolicyDiff(params: QueryParams): Promise<JsonObject>;
    previewPolicy(body: JsonObject): Promise<JsonObject>;
    rollbackPolicy(body: JsonObject): Promise<JsonObject>;
    getRegistry(): Promise<JsonObject>;
}
//# sourceMappingURL=providers-module.d.ts.map