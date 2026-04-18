import type { Identifier, JsonObject } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';
export declare class CrawChatAdminSocialModule {
    private readonly context;
    constructor(context: CrawChatAdminSdkContext);
    bindDirectChat(body: JsonObject): Promise<JsonObject>;
    getDirectChat(id: Identifier): Promise<JsonObject>;
    establishExternalConnection(body: JsonObject): Promise<JsonObject>;
    getExternalConnection(id: Identifier): Promise<JsonObject>;
    bindExternalMemberLink(body: JsonObject): Promise<JsonObject>;
    getExternalMemberLink(id: Identifier): Promise<JsonObject>;
    submitFriendRequest(body: JsonObject): Promise<JsonObject>;
    getFriendRequest(id: Identifier): Promise<JsonObject>;
    activateFriendship(body: JsonObject): Promise<JsonObject>;
    getFriendship(id: Identifier): Promise<JsonObject>;
    applySharedChannelPolicy(body: JsonObject): Promise<JsonObject>;
    getSharedChannelPolicy(id: Identifier): Promise<JsonObject>;
    blockUser(body: JsonObject): Promise<JsonObject>;
    getUserBlock(id: Identifier): Promise<JsonObject>;
}
//# sourceMappingURL=social-module.d.ts.map