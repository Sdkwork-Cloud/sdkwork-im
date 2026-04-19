import type { JsonObject, QueryParams } from './types.js';
import type { ControlPlaneSdkContext } from './sdk-context.js';
export declare class ControlPlaneProvidersModule {
    private readonly context;
    constructor(context: ControlPlaneSdkContext);
    getBindings(params?: QueryParams): Promise<JsonObject>;
    upsertBindingPolicy(body: JsonObject): Promise<JsonObject>;
    getPolicyHistory(): Promise<JsonObject>;
    getPolicyDiff(params: QueryParams): Promise<JsonObject>;
    previewPolicy(body: JsonObject): Promise<JsonObject>;
    rollbackPolicy(body: JsonObject): Promise<JsonObject>;
    getRegistry(): Promise<JsonObject>;
}
//# sourceMappingURL=providers-module.d.ts.map