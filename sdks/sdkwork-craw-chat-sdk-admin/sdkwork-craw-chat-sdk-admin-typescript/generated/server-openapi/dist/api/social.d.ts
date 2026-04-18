import type { Identifier, JsonObject } from '../types/common.js';
import type { HttpClient } from '../http/client.js';
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
export declare function createSocialApi(httpClient: HttpClient): SocialApi;
//# sourceMappingURL=social.d.ts.map