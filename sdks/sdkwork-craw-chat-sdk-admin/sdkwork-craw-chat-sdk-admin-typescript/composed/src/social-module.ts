import type { Identifier, JsonObject } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';

export class CrawChatAdminSocialModule {
  constructor(private readonly context: CrawChatAdminSdkContext) {}

  bindDirectChat(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.social.bindDirectChat(body);
  }

  getDirectChat(id: Identifier): Promise<JsonObject> {
    return this.context.backendClient.social.getDirectChatSnapshot(id);
  }

  establishExternalConnection(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.social.establishExternalConnection(body);
  }

  getExternalConnection(id: Identifier): Promise<JsonObject> {
    return this.context.backendClient.social.getExternalConnectionSnapshot(id);
  }

  bindExternalMemberLink(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.social.bindExternalMemberLink(body);
  }

  getExternalMemberLink(id: Identifier): Promise<JsonObject> {
    return this.context.backendClient.social.getExternalMemberLinkSnapshot(id);
  }

  submitFriendRequest(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.social.submitFriendRequest(body);
  }

  getFriendRequest(id: Identifier): Promise<JsonObject> {
    return this.context.backendClient.social.getFriendRequestSnapshot(id);
  }

  activateFriendship(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.social.activateFriendship(body);
  }

  getFriendship(id: Identifier): Promise<JsonObject> {
    return this.context.backendClient.social.getFriendshipSnapshot(id);
  }

  applySharedChannelPolicy(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.social.applySharedChannelPolicy(body);
  }

  getSharedChannelPolicy(id: Identifier): Promise<JsonObject> {
    return this.context.backendClient.social.getSharedChannelPolicySnapshot(id);
  }

  blockUser(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.social.blockUser(body);
  }

  getUserBlock(id: Identifier): Promise<JsonObject> {
    return this.context.backendClient.social.getUserBlockSnapshot(id);
  }
}
