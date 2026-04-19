import type { JsonObject } from './types.js';
import type { ControlPlaneSdkContext } from './sdk-context.js';
export declare class ControlPlaneProtocolModule {
    private readonly context;
    constructor(context: ControlPlaneSdkContext);
    getGovernance(): Promise<JsonObject>;
    getRegistry(): Promise<JsonObject>;
}
//# sourceMappingURL=protocol-module.d.ts.map