import type { HttpClient } from '../http/client';
export declare class SocialApi {
    private client;
    constructor(client: HttpClient);
    /** Post social direct-chats bindings */
    postApiV1ControlSocialDirectChatsBindings(): Promise<void>;
    /** Get social direct-chats {direct_chat_id} */
    getApiV1ControlSocialDirectChatsId(directChatId: string | number): Promise<void>;
    /** Post social external-connections */
    postApiV1ControlSocialExternalConnections(): Promise<void>;
    /** Get social external-connections {connection_id} */
    getApiV1ControlSocialExternalConnectionsId(connectionId: string | number): Promise<void>;
    /** Post social external-member-links */
    postApiV1ControlSocialExternalMemberLinks(): Promise<void>;
    /** Get social external-member-links {link_id} */
    getApiV1ControlSocialExternalMemberLinksId(linkId: string | number): Promise<void>;
    /** Post social friend-requests */
    postApiV1ControlSocialFriendRequests(): Promise<void>;
    /** Get social friend-requests {request_id} */
    getApiV1ControlSocialFriendRequestsId(requestId: string | number): Promise<void>;
    /** Post social friendships */
    postApiV1ControlSocialFriendships(): Promise<void>;
    /** Get social friendships {friendship_id} */
    getApiV1ControlSocialFriendshipsId(friendshipId: string | number): Promise<void>;
    /** Post social runtime claim-pending-shared-channel-sync-targeted */
    postApiV1ControlSocialRuntimeClaimPendingSharedChannelSyncTargeted(): Promise<void>;
    /** Get social runtime dead-letter-shared-channel-sync */
    getApiV1ControlSocialRuntimeDeadLetterSharedChannelSync(): Promise<void>;
    /** Get social runtime delivered-shared-channel-sync */
    getApiV1ControlSocialRuntimeDeliveredSharedChannelSync(): Promise<void>;
    /** Get social runtime delivery-state-shared-channel-sync */
    getApiV1ControlSocialRuntimeDeliveryStateSharedChannelSync(): Promise<void>;
    /** Get social runtime pending-shared-channel-sync */
    getApiV1ControlSocialRuntimePendingSharedChannelSync(): Promise<void>;
    /** Post social runtime reclaim-stale-pending-shared-channel-sync */
    postApiV1ControlSocialRuntimeReclaimStalePendingSharedChannelSync(): Promise<void>;
    /** Post social runtime release-pending-shared-channel-sync-targeted */
    postApiV1ControlSocialRuntimeReleasePendingSharedChannelSyncTargeted(): Promise<void>;
    /** Post social runtime repair-derived-snapshot */
    postApiV1ControlSocialRuntimeRepairDerivedSnapshot(): Promise<void>;
    /** Post social runtime repair-shared-channel-sync */
    postApiV1ControlSocialRuntimeRepairSharedChannelSync(): Promise<void>;
    /** Post social runtime republish-pending-shared-channel-sync-targeted */
    postApiV1ControlSocialRuntimeRepublishPendingSharedChannelSyncTargeted(): Promise<void>;
    /** Post social runtime requeue-dead-letter-shared-channel-sync */
    postApiV1ControlSocialRuntimeRequeueDeadLetterSharedChannelSync(): Promise<void>;
    /** Post social runtime requeue-dead-letter-shared-channel-sync-targeted */
    postApiV1ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargeted(): Promise<void>;
    /** Post social runtime takeover-pending-shared-channel-sync-targeted */
    postApiV1ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargeted(): Promise<void>;
    /** Post social shared-channel-policies */
    postApiV1ControlSocialSharedChannelPolicies(): Promise<void>;
    /** Get social shared-channel-policies {policy_id} */
    getApiV1ControlSocialSharedChannelPoliciesPolicyId(policyId: string | number): Promise<void>;
    /** Post social user-blocks */
    postApiV1ControlSocialUserBlocks(): Promise<void>;
    /** Get social user-blocks {block_id} */
    getApiV1ControlSocialUserBlocksId(blockId: string | number): Promise<void>;
}
export declare function createSocialApi(client: HttpClient): SocialApi;
//# sourceMappingURL=social.d.ts.map