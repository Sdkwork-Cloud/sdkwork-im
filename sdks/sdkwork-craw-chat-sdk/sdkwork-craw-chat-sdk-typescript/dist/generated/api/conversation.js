import { backendApiPath } from './paths.js';
export class ConversationApi {
    client;
    constructor(client) {
        this.client = client;
    }
    /** Create a conversation */
    async createConversation(body) {
        return this.client.post(backendApiPath(`/conversations`), body, undefined, undefined, 'application/json');
    }
    /** Create an agent dialog conversation */
    async createAgentDialog(body) {
        return this.client.post(backendApiPath(`/conversations/agent-dialogs`), body, undefined, undefined, 'application/json');
    }
    /** Create an agent handoff conversation */
    async createAgentHandoff(body) {
        return this.client.post(backendApiPath(`/conversations/agent-handoffs`), body, undefined, undefined, 'application/json');
    }
    /** Create a system channel conversation */
    async createSystemChannel(body) {
        return this.client.post(backendApiPath(`/conversations/system-channels`), body, undefined, undefined, 'application/json');
    }
    /** Get projected conversation summary */
    async getConversationSummary(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}`));
    }
    /** Get current agent handoff state */
    async getAgentHandoffState(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}/agent-handoff`));
    }
    /** Accept an agent handoff */
    async acceptAgentHandoff(conversationId) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/agent-handoff/accept`));
    }
    /** Resolve an accepted agent handoff */
    async resolveAgentHandoff(conversationId) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/agent-handoff/resolve`));
    }
    /** Close an agent handoff */
    async closeAgentHandoff(conversationId) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/agent-handoff/close`));
    }
    /** List members in a conversation */
    async listConversationMembers(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}/members`));
    }
    /** Add a member to a conversation */
    async addConversationMember(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/add`), body, undefined, undefined, 'application/json');
    }
    /** Remove a member from a conversation */
    async removeConversationMember(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/remove`), body, undefined, undefined, 'application/json');
    }
    /** Transfer conversation ownership */
    async transferConversationOwner(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/transfer-owner`), body, undefined, undefined, 'application/json');
    }
    /** Change a conversation member role */
    async changeConversationMemberRole(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/change-role`), body, undefined, undefined, 'application/json');
    }
    /** Leave a conversation */
    async leave(conversationId) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/leave`));
    }
    /** Get the current member read cursor */
    async getConversationReadCursor(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}/read-cursor`));
    }
    /** Update the current member read cursor */
    async updateConversationReadCursor(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/read-cursor`), body, undefined, undefined, 'application/json');
    }
    /** List projected conversation timeline entries */
    async listConversationMessages(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}/messages`));
    }
    /** Post a standard conversation message */
    async postConversationMessage(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/messages`), body, undefined, undefined, 'application/json');
    }
    /** Publish a message into a system channel conversation */
    async publishSystemChannelMessage(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/system-channel/publish`), body, undefined, undefined, 'application/json');
    }
}
export function createConversationApi(client) {
    return new ConversationApi(client);
}
