import type { FetchLike, Identifier, JsonObject, QueryParams } from './generated-backend-types.js';
export type { ControlPlaneBackendConfig, FetchLike, Identifier, JsonObject, QueryParams, } from './generated-backend-types.js';
export interface ControlPlaneBackendClientLike {
    meta: {
        getHealthz(): Promise<JsonObject>;
    };
    protocol: {
        getProtocolGovernance(): Promise<JsonObject>;
        getProtocolRegistry(): Promise<JsonObject>;
    };
    providers: {
        getProviderBindings(params?: QueryParams): Promise<JsonObject>;
        upsertProviderBindingPolicy(body: JsonObject): Promise<JsonObject>;
        getProviderPolicyHistory(): Promise<JsonObject>;
        getProviderPolicyDiff(params: QueryParams): Promise<JsonObject>;
        previewProviderPolicy(body: JsonObject): Promise<JsonObject>;
        rollbackProviderPolicy(body: JsonObject): Promise<JsonObject>;
        getProviderRegistry(): Promise<JsonObject>;
    };
    social: {
        bindDirectChat(body: JsonObject): Promise<JsonObject>;
        getDirectChatSnapshot(id: Identifier): Promise<JsonObject>;
        establishExternalConnection(body: JsonObject): Promise<JsonObject>;
        getExternalConnectionSnapshot(id: Identifier): Promise<JsonObject>;
        bindExternalMemberLink(body: JsonObject): Promise<JsonObject>;
        getExternalMemberLinkSnapshot(id: Identifier): Promise<JsonObject>;
        submitFriendRequest(body: JsonObject): Promise<JsonObject>;
        getFriendRequestSnapshot(id: Identifier): Promise<JsonObject>;
        activateFriendship(body: JsonObject): Promise<JsonObject>;
        getFriendshipSnapshot(id: Identifier): Promise<JsonObject>;
        applySharedChannelPolicy(body: JsonObject): Promise<JsonObject>;
        getSharedChannelPolicySnapshot(id: Identifier): Promise<JsonObject>;
        blockUser(body: JsonObject): Promise<JsonObject>;
        getUserBlockSnapshot(id: Identifier): Promise<JsonObject>;
    };
    socialRuntime: {
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
    };
    nodes: {
        activateNode(nodeId: Identifier): Promise<JsonObject>;
        drainNode(nodeId: Identifier): Promise<JsonObject>;
        migrateNodeRoutes(nodeId: Identifier, body: JsonObject): Promise<JsonObject>;
    };
    setAuthToken?(token: string): unknown;
}
export interface ControlPlaneSdkClientOptions {
    backendClient: ControlPlaneBackendClientLike;
}
export interface ControlPlaneSdkClientCreateOptions {
    backendClient?: ControlPlaneBackendClientLike;
    baseUrl?: string;
    authToken?: string;
    headers?: Record<string, string>;
    timeout?: number;
    fetch?: FetchLike;
}
//# sourceMappingURL=types.d.ts.map