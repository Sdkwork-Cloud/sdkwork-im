function encodeIdentifier(id) {
    return encodeURIComponent(String(id));
}
export function createSocialApi(httpClient) {
    return {
        bindDirectChat(body) {
            return httpClient.post('/api/v1/control/social/direct-chats/bindings', body);
        },
        getDirectChatSnapshot(directChatId) {
            return httpClient.get(`/api/v1/control/social/direct-chats/${encodeIdentifier(directChatId)}`);
        },
        establishExternalConnection(body) {
            return httpClient.post('/api/v1/control/social/external-connections', body);
        },
        getExternalConnectionSnapshot(connectionId) {
            return httpClient.get(`/api/v1/control/social/external-connections/${encodeIdentifier(connectionId)}`);
        },
        bindExternalMemberLink(body) {
            return httpClient.post('/api/v1/control/social/external-member-links', body);
        },
        getExternalMemberLinkSnapshot(linkId) {
            return httpClient.get(`/api/v1/control/social/external-member-links/${encodeIdentifier(linkId)}`);
        },
        submitFriendRequest(body) {
            return httpClient.post('/api/v1/control/social/friend-requests', body);
        },
        getFriendRequestSnapshot(requestId) {
            return httpClient.get(`/api/v1/control/social/friend-requests/${encodeIdentifier(requestId)}`);
        },
        activateFriendship(body) {
            return httpClient.post('/api/v1/control/social/friendships', body);
        },
        getFriendshipSnapshot(friendshipId) {
            return httpClient.get(`/api/v1/control/social/friendships/${encodeIdentifier(friendshipId)}`);
        },
        applySharedChannelPolicy(body) {
            return httpClient.post('/api/v1/control/social/shared-channel-policies', body);
        },
        getSharedChannelPolicySnapshot(policyId) {
            return httpClient.get(`/api/v1/control/social/shared-channel-policies/${encodeIdentifier(policyId)}`);
        },
        blockUser(body) {
            return httpClient.post('/api/v1/control/social/user-blocks', body);
        },
        getUserBlockSnapshot(blockId) {
            return httpClient.get(`/api/v1/control/social/user-blocks/${encodeIdentifier(blockId)}`);
        },
    };
}
