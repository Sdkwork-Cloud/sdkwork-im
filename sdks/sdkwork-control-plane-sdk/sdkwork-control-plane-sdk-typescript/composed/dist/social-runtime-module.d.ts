import type { JsonObject } from './types.js';
import type { ControlPlaneSdkContext } from './sdk-context.js';
export declare class ControlPlaneSocialRuntimeModule {
    private readonly context;
    constructor(context: ControlPlaneSdkContext);
    claimPendingTargeted(body: JsonObject): Promise<JsonObject>;
    getDeadLetterInventory(): Promise<JsonObject>;
    getDeliveredInventory(): Promise<JsonObject>;
    getDeliveryStateInventory(): Promise<JsonObject>;
    getPendingInventory(): Promise<JsonObject>;
    reclaimStalePending(): Promise<JsonObject>;
    releasePendingTargeted(body: JsonObject): Promise<JsonObject>;
    repairSnapshot(): Promise<JsonObject>;
    repairSharedChannelSync(): Promise<JsonObject>;
    republishPendingTargeted(body: JsonObject): Promise<JsonObject>;
    requeueDeadLetters(): Promise<JsonObject>;
    requeueDeadLettersTargeted(body: JsonObject): Promise<JsonObject>;
    takeoverPendingTargeted(body: JsonObject): Promise<JsonObject>;
}
//# sourceMappingURL=social-runtime-module.d.ts.map