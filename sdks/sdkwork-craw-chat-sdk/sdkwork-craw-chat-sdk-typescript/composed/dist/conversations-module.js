import { buildTextMessageRequest } from './builders.js';
export class CrawChatConversationsModule {
    context;
    constructor(context) {
        this.context = context;
    }
    create(body) {
        return this.context.backendClient.conversation.createConversation(body);
    }
    createAgentDialog(body) {
        return this.context.backendClient.conversation.createAgentDialog(body);
    }
    createAgentHandoff(body) {
        return this.context.backendClient.conversation.createAgentHandoff(body);
    }
    createSystemChannel(body) {
        return this.context.backendClient.conversation.createSystemChannel(body);
    }
    get(conversationId) {
        return this.context.backendClient.conversation.getConversationSummary(conversationId);
    }
    getAgentHandoffState(conversationId) {
        return this.context.backendClient.conversation.getAgentHandoffState(conversationId);
    }
    acceptAgentHandoff(conversationId) {
        return this.context.backendClient.conversation.acceptAgentHandoff(conversationId);
    }
    resolveAgentHandoff(conversationId) {
        return this.context.backendClient.conversation.resolveAgentHandoff(conversationId);
    }
    closeAgentHandoff(conversationId) {
        return this.context.backendClient.conversation.closeAgentHandoff(conversationId);
    }
    listMembers(conversationId) {
        return this.context.backendClient.conversation.listConversationMembers(conversationId);
    }
    addMember(conversationId, body) {
        return this.context.backendClient.conversation.addConversationMember(conversationId, body);
    }
    removeMember(conversationId, body) {
        return this.context.backendClient.conversation.removeConversationMember(conversationId, body);
    }
    transferOwner(conversationId, body) {
        return this.context.backendClient.conversation.transferConversationOwner(conversationId, body);
    }
    changeMemberRole(conversationId, body) {
        return this.context.backendClient.conversation.changeConversationMemberRole(conversationId, body);
    }
    leave(conversationId) {
        return this.context.backendClient.conversation.leave(conversationId);
    }
    getReadCursor(conversationId) {
        return this.context.backendClient.conversation.getConversationReadCursor(conversationId);
    }
    updateReadCursor(conversationId, body) {
        return this.context.backendClient.conversation.updateConversationReadCursor(conversationId, body);
    }
    listMessages(conversationId) {
        return this.context.backendClient.conversation.listConversationMessages(conversationId);
    }
    postMessage(conversationId, body) {
        return this.context.backendClient.conversation.postConversationMessage(conversationId, body);
    }
    postText(conversationId, text, options = {}) {
        return this.postMessage(conversationId, buildTextMessageRequest(text, options));
    }
    publishSystemMessage(conversationId, body) {
        return this.context.backendClient.conversation.publishSystemChannelMessage(conversationId, body);
    }
    publishSystemText(conversationId, text, options = {}) {
        return this.publishSystemMessage(conversationId, buildTextMessageRequest(text, options));
    }
}
