import type { Identifier, JsonObject } from '../types/common.js';
import type { HttpClient } from '../http/client.js';

function encodeIdentifier(id: Identifier): string {
  return encodeURIComponent(String(id));
}

export interface SocialApi {
  bindDirectChat(body: JsonObject): Promise<JsonObject>;
  getDirectChatSnapshot(directChatId: Identifier): Promise<JsonObject>;
  establishExternalConnection(body: JsonObject): Promise<JsonObject>;
  getExternalConnectionSnapshot(connectionId: Identifier): Promise<JsonObject>;
  bindExternalMemberLink(body: JsonObject): Promise<JsonObject>;
  getExternalMemberLinkSnapshot(linkId: Identifier): Promise<JsonObject>;
  submitFriendRequest(body: JsonObject): Promise<JsonObject>;
  getFriendRequestSnapshot(requestId: Identifier): Promise<JsonObject>;
  activateFriendship(body: JsonObject): Promise<JsonObject>;
  getFriendshipSnapshot(friendshipId: Identifier): Promise<JsonObject>;
  applySharedChannelPolicy(body: JsonObject): Promise<JsonObject>;
  getSharedChannelPolicySnapshot(policyId: Identifier): Promise<JsonObject>;
  blockUser(body: JsonObject): Promise<JsonObject>;
  getUserBlockSnapshot(blockId: Identifier): Promise<JsonObject>;
}

export function createSocialApi(httpClient: HttpClient): SocialApi {
  return {
    bindDirectChat(body) {
      return httpClient.post<JsonObject>('/api/v1/control/social/direct-chats/bindings', body);
    },
    getDirectChatSnapshot(directChatId) {
      return httpClient.get<JsonObject>(
        `/api/v1/control/social/direct-chats/${encodeIdentifier(directChatId)}`,
      );
    },
    establishExternalConnection(body) {
      return httpClient.post<JsonObject>('/api/v1/control/social/external-connections', body);
    },
    getExternalConnectionSnapshot(connectionId) {
      return httpClient.get<JsonObject>(
        `/api/v1/control/social/external-connections/${encodeIdentifier(connectionId)}`,
      );
    },
    bindExternalMemberLink(body) {
      return httpClient.post<JsonObject>('/api/v1/control/social/external-member-links', body);
    },
    getExternalMemberLinkSnapshot(linkId) {
      return httpClient.get<JsonObject>(
        `/api/v1/control/social/external-member-links/${encodeIdentifier(linkId)}`,
      );
    },
    submitFriendRequest(body) {
      return httpClient.post<JsonObject>('/api/v1/control/social/friend-requests', body);
    },
    getFriendRequestSnapshot(requestId) {
      return httpClient.get<JsonObject>(
        `/api/v1/control/social/friend-requests/${encodeIdentifier(requestId)}`,
      );
    },
    activateFriendship(body) {
      return httpClient.post<JsonObject>('/api/v1/control/social/friendships', body);
    },
    getFriendshipSnapshot(friendshipId) {
      return httpClient.get<JsonObject>(
        `/api/v1/control/social/friendships/${encodeIdentifier(friendshipId)}`,
      );
    },
    applySharedChannelPolicy(body) {
      return httpClient.post<JsonObject>('/api/v1/control/social/shared-channel-policies', body);
    },
    getSharedChannelPolicySnapshot(policyId) {
      return httpClient.get<JsonObject>(
        `/api/v1/control/social/shared-channel-policies/${encodeIdentifier(policyId)}`,
      );
    },
    blockUser(body) {
      return httpClient.post<JsonObject>('/api/v1/control/social/user-blocks', body);
    },
    getUserBlockSnapshot(blockId) {
      return httpClient.get<JsonObject>(
        `/api/v1/control/social/user-blocks/${encodeIdentifier(blockId)}`,
      );
    },
  };
}
