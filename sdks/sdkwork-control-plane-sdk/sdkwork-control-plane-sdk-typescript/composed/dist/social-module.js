export class ControlPlaneSocialModule {
    context;
    constructor(context) {
        this.context = context;
    }
    bindDirectChat(body) {
        return this.context.backendClient.social.bindDirectChat(body);
    }
    getDirectChat(id) {
        return this.context.backendClient.social.getDirectChatSnapshot(id);
    }
    establishExternalConnection(body) {
        return this.context.backendClient.social.establishExternalConnection(body);
    }
    getExternalConnection(id) {
        return this.context.backendClient.social.getExternalConnectionSnapshot(id);
    }
    bindExternalMemberLink(body) {
        return this.context.backendClient.social.bindExternalMemberLink(body);
    }
    getExternalMemberLink(id) {
        return this.context.backendClient.social.getExternalMemberLinkSnapshot(id);
    }
    submitFriendRequest(body) {
        return this.context.backendClient.social.submitFriendRequest(body);
    }
    getFriendRequest(id) {
        return this.context.backendClient.social.getFriendRequestSnapshot(id);
    }
    activateFriendship(body) {
        return this.context.backendClient.social.activateFriendship(body);
    }
    getFriendship(id) {
        return this.context.backendClient.social.getFriendshipSnapshot(id);
    }
    applySharedChannelPolicy(body) {
        return this.context.backendClient.social.applySharedChannelPolicy(body);
    }
    getSharedChannelPolicy(id) {
        return this.context.backendClient.social.getSharedChannelPolicySnapshot(id);
    }
    blockUser(body) {
        return this.context.backendClient.social.blockUser(body);
    }
    getUserBlock(id) {
        return this.context.backendClient.social.getUserBlockSnapshot(id);
    }
}
