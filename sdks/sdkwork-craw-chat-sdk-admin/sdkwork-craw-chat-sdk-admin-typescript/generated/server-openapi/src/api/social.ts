import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';


export class SocialApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Post social direct-chats bindings */
  async postApiV1ControlSocialDirectChatsBindings(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/direct-chats/bindings`));
  }

/** Get social direct-chats {direct_chat_id} */
  async getApiV1ControlSocialDirectChatsId(directChatId: string | number): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/direct-chats/${directChatId}`));
  }

/** Post social external-connections */
  async postApiV1ControlSocialExternalConnections(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/external-connections`));
  }

/** Get social external-connections {connection_id} */
  async getApiV1ControlSocialExternalConnectionsId(connectionId: string | number): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/external-connections/${connectionId}`));
  }

/** Post social external-member-links */
  async postApiV1ControlSocialExternalMemberLinks(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/external-member-links`));
  }

/** Get social external-member-links {link_id} */
  async getApiV1ControlSocialExternalMemberLinksId(linkId: string | number): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/external-member-links/${linkId}`));
  }

/** Post social friend-requests */
  async postApiV1ControlSocialFriendRequests(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/friend-requests`));
  }

/** Get social friend-requests {request_id} */
  async getApiV1ControlSocialFriendRequestsId(requestId: string | number): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/friend-requests/${requestId}`));
  }

/** Post social friendships */
  async postApiV1ControlSocialFriendships(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/friendships`));
  }

/** Get social friendships {friendship_id} */
  async getApiV1ControlSocialFriendshipsId(friendshipId: string | number): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/friendships/${friendshipId}`));
  }

/** Post social runtime claim-pending-shared-channel-sync-targeted */
  async postApiV1ControlSocialRuntimeClaimPendingSharedChannelSyncTargeted(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted`));
  }

/** Get social runtime dead-letter-shared-channel-sync */
  async getApiV1ControlSocialRuntimeDeadLetterSharedChannelSync(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/runtime/dead-letter-shared-channel-sync`));
  }

/** Get social runtime delivered-shared-channel-sync */
  async getApiV1ControlSocialRuntimeDeliveredSharedChannelSync(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/runtime/delivered-shared-channel-sync`));
  }

/** Get social runtime delivery-state-shared-channel-sync */
  async getApiV1ControlSocialRuntimeDeliveryStateSharedChannelSync(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/runtime/delivery-state-shared-channel-sync`));
  }

/** Get social runtime pending-shared-channel-sync */
  async getApiV1ControlSocialRuntimePendingSharedChannelSync(): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/runtime/pending-shared-channel-sync`));
  }

/** Post social runtime reclaim-stale-pending-shared-channel-sync */
  async postApiV1ControlSocialRuntimeReclaimStalePendingSharedChannelSync(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync`));
  }

/** Post social runtime release-pending-shared-channel-sync-targeted */
  async postApiV1ControlSocialRuntimeReleasePendingSharedChannelSyncTargeted(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted`));
  }

/** Post social runtime repair-derived-snapshot */
  async postApiV1ControlSocialRuntimeRepairDerivedSnapshot(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/runtime/repair-derived-snapshot`));
  }

/** Post social runtime repair-shared-channel-sync */
  async postApiV1ControlSocialRuntimeRepairSharedChannelSync(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/runtime/repair-shared-channel-sync`));
  }

/** Post social runtime republish-pending-shared-channel-sync-targeted */
  async postApiV1ControlSocialRuntimeRepublishPendingSharedChannelSyncTargeted(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted`));
  }

/** Post social runtime requeue-dead-letter-shared-channel-sync */
  async postApiV1ControlSocialRuntimeRequeueDeadLetterSharedChannelSync(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync`));
  }

/** Post social runtime requeue-dead-letter-shared-channel-sync-targeted */
  async postApiV1ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargeted(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted`));
  }

/** Post social runtime takeover-pending-shared-channel-sync-targeted */
  async postApiV1ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargeted(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted`));
  }

/** Post social shared-channel-policies */
  async postApiV1ControlSocialSharedChannelPolicies(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/shared-channel-policies`));
  }

/** Get social shared-channel-policies {policy_id} */
  async getApiV1ControlSocialSharedChannelPoliciesPolicyId(policyId: string | number): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/shared-channel-policies/${policyId}`));
  }

/** Post social user-blocks */
  async postApiV1ControlSocialUserBlocks(): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/social/user-blocks`));
  }

/** Get social user-blocks {block_id} */
  async getApiV1ControlSocialUserBlocksId(blockId: string | number): Promise<void> {
    return this.client.get<void>(backendApiPath(`/api/v1/control/social/user-blocks/${blockId}`));
  }
}

export function createSocialApi(client: HttpClient): SocialApi {
  return new SocialApi(client);
}
