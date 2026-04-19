import type { Identifier, JsonObject } from './types.js';
import type { ControlPlaneSdkContext } from './sdk-context.js';
export declare class ControlPlaneNodesModule {
    private readonly context;
    constructor(context: ControlPlaneSdkContext);
    activate(nodeId: Identifier): Promise<JsonObject>;
    drain(nodeId: Identifier): Promise<JsonObject>;
    migrateRoutes(nodeId: Identifier, body: JsonObject): Promise<JsonObject>;
}
//# sourceMappingURL=nodes-module.d.ts.map