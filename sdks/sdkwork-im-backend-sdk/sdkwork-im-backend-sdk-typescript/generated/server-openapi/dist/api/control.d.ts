import type { HttpClient } from '../http/client';
import type { AcceptFriendRequestRequest, ActivateFriendshipRequest, ApplySharedChannelPolicyRequest, BindDirectChatRequest, BindExternalMemberLinkRequest, BlockUserRequest, CancelFriendRequestRequest, DeclineFriendRequestRequest, EstablishExternalConnectionRequest, MigrateRoutesRequest, ProtocolGovernanceResponse, ProtocolRegistryResponse, ProviderBindingCommitResponse, ProviderBindingsResponse, ProviderPolicyDiffResponse, ProviderPolicyHistoryResponse, ProviderPolicyRollbackRequest, ProviderRegistrySnapshotResponse, RemoveFriendshipRequest, RouteMigrationResult, RouteNodeLifecycle, SocialDirectChatCommitResponse, SocialDirectChatSnapshotResponse, SocialExternalConnectionCommitResponse, SocialExternalConnectionSnapshotResponse, SocialExternalMemberLinkCommitResponse, SocialExternalMemberLinkSnapshotResponse, SocialFriendRequestCommitResponse, SocialFriendRequestSnapshotResponse, SocialFriendshipCommitResponse, SocialFriendshipSnapshotResponse, SocialRuntimeRepairResponse, SocialSharedChannelPolicyCommitResponse, SocialSharedChannelPolicySnapshotResponse, SocialSharedChannelSyncDeadLetterInventoryResponse, SocialSharedChannelSyncDeadLetterRequeueResponse, SocialSharedChannelSyncDeadLetterTargetedRequeueRequest, SocialSharedChannelSyncDeadLetterTargetedRequeueResponse, SocialSharedChannelSyncDeliveredInventoryResponse, SocialSharedChannelSyncDeliveryStateInventoryResponse, SocialSharedChannelSyncPendingClaimResponse, SocialSharedChannelSyncPendingInventoryResponse, SocialSharedChannelSyncPendingReleaseResponse, SocialSharedChannelSyncPendingStaleReclaimResponse, SocialSharedChannelSyncPendingTakeoverResponse, SocialSharedChannelSyncPendingTargetedClaimRequest, SocialSharedChannelSyncPendingTargetedReleaseRequest, SocialSharedChannelSyncPendingTargetedTakeoverRequest, SocialSharedChannelSyncRepairResponse, SocialSharedChannelSyncTargetedRepublishRequest, SocialSharedChannelSyncTargetedRepublishResponse, SocialUserBlockCommitResponse, SocialUserBlockSnapshotResponse, SubmitFriendRequestRequest, UpsertProviderBindingPolicyRequest } from '../types';
export declare class ControlSocialUserBlocksApi {
    private client;
    constructor(client: HttpClient);
    /** Block a user in the social graph. */
    create(body: BlockUserRequest): Promise<SocialUserBlockCommitResponse>;
    /** Read a user block snapshot. */
    retrieve(blockId: string): Promise<SocialUserBlockSnapshotResponse>;
}
export declare class ControlSocialSharedChannelPoliciesApi {
    private client;
    constructor(client: HttpClient);
    /** Apply a shared-channel policy. */
    create(body: ApplySharedChannelPolicyRequest): Promise<SocialSharedChannelPolicyCommitResponse>;
    /** Read a shared-channel policy snapshot. */
    retrieve(policyId: string): Promise<SocialSharedChannelPolicySnapshotResponse>;
}
export declare class ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi {
    private client;
    constructor(client: HttpClient);
    /** Take over selected pending shared-channel sync entries. */
    create(body: SocialSharedChannelSyncPendingTargetedTakeoverRequest): Promise<SocialSharedChannelSyncPendingTakeoverResponse>;
}
export declare class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi {
    private client;
    constructor(client: HttpClient);
    /** Requeue selected dead-letter shared-channel sync entries. */
    create(body: SocialSharedChannelSyncDeadLetterTargetedRequeueRequest): Promise<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse>;
}
export declare class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi {
    private client;
    constructor(client: HttpClient);
    /** Requeue all dead-letter shared-channel sync entries. */
    create(): Promise<SocialSharedChannelSyncDeadLetterRequeueResponse>;
}
export declare class ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi {
    private client;
    constructor(client: HttpClient);
    /** Republish selected pending shared-channel sync entries. */
    create(body: SocialSharedChannelSyncTargetedRepublishRequest): Promise<SocialSharedChannelSyncTargetedRepublishResponse>;
}
export declare class ControlSocialRuntimeRepairSharedChannelSyncApi {
    private client;
    constructor(client: HttpClient);
    /** Repair shared-channel sync backlog state. */
    create(): Promise<SocialSharedChannelSyncRepairResponse>;
}
export declare class ControlSocialRuntimeRepairDerivedSnapshotApi {
    private client;
    constructor(client: HttpClient);
    /** Repair the persisted social runtime derived snapshot. */
    create(): Promise<SocialRuntimeRepairResponse>;
}
export declare class ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi {
    private client;
    constructor(client: HttpClient);
    /** Release selected pending shared-channel sync entries. */
    create(body: SocialSharedChannelSyncPendingTargetedReleaseRequest): Promise<SocialSharedChannelSyncPendingReleaseResponse>;
}
export declare class ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi {
    private client;
    constructor(client: HttpClient);
    /** Reclaim stale shared-channel sync pending ownership. */
    create(): Promise<SocialSharedChannelSyncPendingStaleReclaimResponse>;
}
export declare class ControlSocialRuntimePendingSharedChannelSyncApi {
    private client;
    constructor(client: HttpClient);
    /** Read the pending shared-channel sync queue. */
    list(): Promise<SocialSharedChannelSyncPendingInventoryResponse>;
}
export declare class ControlSocialRuntimeDeliveryStateSharedChannelSyncApi {
    private client;
    constructor(client: HttpClient);
    /** Read merged shared-channel sync delivery state. */
    list(): Promise<SocialSharedChannelSyncDeliveryStateInventoryResponse>;
}
export declare class ControlSocialRuntimeDeliveredSharedChannelSyncApi {
    private client;
    constructor(client: HttpClient);
    /** Read the delivered shared-channel sync ledger. */
    list(): Promise<SocialSharedChannelSyncDeliveredInventoryResponse>;
}
export declare class ControlSocialRuntimeDeadLetterSharedChannelSyncApi {
    private client;
    constructor(client: HttpClient);
    /** Read the dead-letter shared-channel sync queue. */
    list(): Promise<SocialSharedChannelSyncDeadLetterInventoryResponse>;
}
export declare class ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi {
    private client;
    constructor(client: HttpClient);
    /** Claim selected pending shared-channel sync entries. */
    create(body: SocialSharedChannelSyncPendingTargetedClaimRequest): Promise<SocialSharedChannelSyncPendingClaimResponse>;
}
export declare class ControlSocialRuntimeApi {
    private client;
    readonly claimPendingSharedChannelSyncTargeted: ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi;
    readonly deadLetterSharedChannelSync: ControlSocialRuntimeDeadLetterSharedChannelSyncApi;
    readonly deliveredSharedChannelSync: ControlSocialRuntimeDeliveredSharedChannelSyncApi;
    readonly deliveryStateSharedChannelSync: ControlSocialRuntimeDeliveryStateSharedChannelSyncApi;
    readonly pendingSharedChannelSync: ControlSocialRuntimePendingSharedChannelSyncApi;
    readonly reclaimStalePendingSharedChannelSync: ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi;
    readonly releasePendingSharedChannelSyncTargeted: ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi;
    readonly repairDerivedSnapshot: ControlSocialRuntimeRepairDerivedSnapshotApi;
    readonly repairSharedChannelSync: ControlSocialRuntimeRepairSharedChannelSyncApi;
    readonly republishPendingSharedChannelSyncTargeted: ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi;
    readonly requeueDeadLetterSharedChannelSync: ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi;
    readonly requeueDeadLetterSharedChannelSyncTargeted: ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi;
    readonly takeoverPendingSharedChannelSyncTargeted: ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi;
    constructor(client: HttpClient);
}
export declare class ControlSocialFriendshipsApi {
    private client;
    constructor(client: HttpClient);
    /** Activate a friendship event. */
    create(body: ActivateFriendshipRequest): Promise<SocialFriendshipCommitResponse>;
    /** Read a friendship snapshot. */
    retrieve(friendshipId: string): Promise<SocialFriendshipSnapshotResponse>;
    /** Remove a friendship. */
    remove(friendshipId: string, body: RemoveFriendshipRequest): Promise<SocialFriendshipCommitResponse>;
}
export declare class ControlSocialFriendRequestsApi {
    private client;
    constructor(client: HttpClient);
    /** Submit a friend request event. */
    create(body: SubmitFriendRequestRequest): Promise<SocialFriendRequestCommitResponse>;
    /** Read a friend request snapshot. */
    retrieve(requestId: string): Promise<SocialFriendRequestSnapshotResponse>;
    /** Accept a friend request. */
    accept(requestId: string, body: AcceptFriendRequestRequest): Promise<SocialFriendRequestCommitResponse>;
    /** Decline a friend request. */
    decline(requestId: string, body: DeclineFriendRequestRequest): Promise<SocialFriendRequestCommitResponse>;
    /** Cancel a friend request. */
    cancel(requestId: string, body: CancelFriendRequestRequest): Promise<SocialFriendRequestCommitResponse>;
}
export declare class ControlSocialExternalMemberLinksApi {
    private client;
    constructor(client: HttpClient);
    /** Bind an external member link. */
    create(body: BindExternalMemberLinkRequest): Promise<SocialExternalMemberLinkCommitResponse>;
    /** Read an external member link snapshot. */
    retrieve(linkId: string): Promise<SocialExternalMemberLinkSnapshotResponse>;
}
export declare class ControlSocialExternalConnectionsApi {
    private client;
    constructor(client: HttpClient);
    /** Establish an external collaboration connection. */
    create(body: EstablishExternalConnectionRequest): Promise<SocialExternalConnectionCommitResponse>;
    /** Read an external connection snapshot. */
    retrieve(connectionId: string): Promise<SocialExternalConnectionSnapshotResponse>;
}
export declare class ControlSocialDirectChatsBindingsApi {
    private client;
    constructor(client: HttpClient);
    /** Bind a direct chat to a conversation. */
    create(body: BindDirectChatRequest): Promise<SocialDirectChatCommitResponse>;
}
export declare class ControlSocialDirectChatsApi {
    private client;
    readonly bindings: ControlSocialDirectChatsBindingsApi;
    constructor(client: HttpClient);
    /** Read a direct chat snapshot. */
    retrieve(directChatId: string): Promise<SocialDirectChatSnapshotResponse>;
}
export declare class ControlSocialApi {
    private client;
    readonly directChats: ControlSocialDirectChatsApi;
    readonly externalConnections: ControlSocialExternalConnectionsApi;
    readonly externalMemberLinks: ControlSocialExternalMemberLinksApi;
    readonly friendRequests: ControlSocialFriendRequestsApi;
    readonly friendships: ControlSocialFriendshipsApi;
    readonly runtime: ControlSocialRuntimeApi;
    readonly sharedChannelPolicies: ControlSocialSharedChannelPoliciesApi;
    readonly userBlocks: ControlSocialUserBlocksApi;
    constructor(client: HttpClient);
}
export interface ControlProviderBindingsListParams {
    tenantId?: string;
}
export declare class ControlProviderBindingsApi {
    private client;
    constructor(client: HttpClient);
    /** Read effective provider bindings. */
    list(params?: ControlProviderBindingsListParams): Promise<ProviderBindingsResponse>;
    /** Upsert a provider binding policy. */
    create(body: UpsertProviderBindingPolicyRequest): Promise<ProviderBindingCommitResponse>;
}
export declare class ControlProviderRegistryApi {
    private client;
    constructor(client: HttpClient);
    /** Read the provider registry snapshot. */
    retrieve(): Promise<ProviderRegistrySnapshotResponse>;
}
export interface ControlProviderPoliciesDiffListParams {
    fromVersion: number;
    toVersion: number;
}
export declare class ControlProviderPoliciesDiffApi {
    private client;
    constructor(client: HttpClient);
    /** Read provider policy diff between two versions. */
    list(params: ControlProviderPoliciesDiffListParams): Promise<ProviderPolicyDiffResponse>;
}
export declare class ControlProviderPoliciesApi {
    private client;
    readonly diff: ControlProviderPoliciesDiffApi;
    constructor(client: HttpClient);
    /** Read provider policy history. */
    list(): Promise<ProviderPolicyHistoryResponse>;
    /** Preview the effective provider policy result before commit. */
    preview(body: UpsertProviderBindingPolicyRequest): Promise<ProviderBindingCommitResponse>;
    /** Rollback provider policy history to a target version. */
    rollback(body: ProviderPolicyRollbackRequest): Promise<ProviderBindingCommitResponse>;
}
export declare class ControlProtocolRegistryApi {
    private client;
    constructor(client: HttpClient);
    /** Read the control-plane protocol registry snapshot. */
    retrieve(): Promise<ProtocolRegistryResponse>;
}
export declare class ControlProtocolGovernanceApi {
    private client;
    constructor(client: HttpClient);
    /** Read the control-plane protocol governance snapshot. */
    retrieve(): Promise<ProtocolGovernanceResponse>;
}
export declare class ControlNodesRoutesApi {
    private client;
    constructor(client: HttpClient);
    /** Migrate owned routes from the source node to the target node. */
    migrate(nodeId: string, body: MigrateRoutesRequest): Promise<RouteMigrationResult>;
}
export declare class ControlNodesApi {
    private client;
    readonly routes: ControlNodesRoutesApi;
    constructor(client: HttpClient);
    /** Activate a realtime node and clear drain state. */
    activate(nodeId: string): Promise<RouteNodeLifecycle>;
    /** Mark a realtime node as draining. */
    drain(nodeId: string): Promise<RouteNodeLifecycle>;
}
export declare class ControlApi {
    private client;
    readonly nodes: ControlNodesApi;
    readonly protocolGovernance: ControlProtocolGovernanceApi;
    readonly protocolRegistry: ControlProtocolRegistryApi;
    readonly providerPolicies: ControlProviderPoliciesApi;
    readonly providerRegistry: ControlProviderRegistryApi;
    readonly providerBindings: ControlProviderBindingsApi;
    readonly social: ControlSocialApi;
    constructor(client: HttpClient);
}
export declare function createControlApi(client: HttpClient): ControlApi;
//# sourceMappingURL=control.d.ts.map