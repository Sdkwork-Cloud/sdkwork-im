import type { JsonObject } from '../types/common.js';
import type { HttpClient } from '../http/client.js';
export interface SocialRuntimeApi {
    claimPendingSharedChannelSyncTargeted(body: JsonObject): Promise<JsonObject>;
    getDeadLetterSharedChannelSyncInventory(): Promise<JsonObject>;
    getDeliveredSharedChannelSyncInventory(): Promise<JsonObject>;
    getSharedChannelSyncDeliveryStateInventory(): Promise<JsonObject>;
    getPendingSharedChannelSyncInventory(): Promise<JsonObject>;
    reclaimStalePendingSharedChannelSync(): Promise<JsonObject>;
    releasePendingSharedChannelSyncTargeted(body: JsonObject): Promise<JsonObject>;
    repairSocialRuntimeSnapshot(): Promise<JsonObject>;
    repairSharedChannelSync(): Promise<JsonObject>;
    republishPendingSharedChannelSyncTargeted(body: JsonObject): Promise<JsonObject>;
    requeueDeadLetterSharedChannelSync(): Promise<JsonObject>;
    requeueDeadLetterSharedChannelSyncTargeted(body: JsonObject): Promise<JsonObject>;
    takeoverPendingSharedChannelSyncTargeted(body: JsonObject): Promise<JsonObject>;
}
export declare function createSocialRuntimeApi(httpClient: HttpClient): SocialRuntimeApi;
//# sourceMappingURL=social-runtime.d.ts.map