import type { JsonObject } from './types.js';
import type { ControlPlaneSdkContext } from './sdk-context.js';
export declare class ControlPlaneMetaModule {
    private readonly context;
    constructor(context: ControlPlaneSdkContext);
    health(): Promise<JsonObject>;
}
//# sourceMappingURL=meta-module.d.ts.map