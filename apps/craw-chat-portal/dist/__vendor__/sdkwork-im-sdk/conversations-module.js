import { buildTextMessageRequest } from './builders.js';
export class ImConversationsModule {
    context;
    constructor(context) {
        this.context = context;
    }
    create(body) {
        return this.context.transportClient.conversation.createConversation(body);
    }
    createAgentDialog(body) {
        return this.context.transportClient.conversation.createAgentDialog(body);
    }
    createAgentHandoff(body) {
        return this.context.transportClient.conversation.createAgentHandoff(body);
    }
    createSystemChannel(body) {
        return this.context.transportClient.conversation.createSystemChannel(body);
    }
    get(conversationId) {
        return this.context.transportClient.conversation.getConversationSummary(conversationId);
    }
    getAgentHandoffState(conversationId) {
        return this.context.transportClient.conversation.getAgentHandoffState(conversationId);
    }
    acceptAgentHandoff(conversationId) {
        return this.context.transportClient.conversation.acceptAgentHandoff(conversationId);
    }
    resolveAgentHandoff(conversationId) {
        return this.context.transportClient.conversation.resolveAgentHandoff(conversationId);
    }
    closeAgentHandoff(conversationId) {
        return this.context.transportClient.conversation.closeAgentHandoff(conversationId);
    }
    listMembers(conversationId) {
        return this.context.transportClient.conversation.listConversationMembers(conversationId);
    }
    addMember(conversationId, body) {
        return this.context.transportClient.conversation.addConversationMember(conversationId, body);
    }
    removeMember(conversationId, body) {
        return this.context.transportClient.conversation.removeConversationMember(conversationId, body);
    }
    transferOwner(conversationId, body) {
        return this.context.transportClient.conversation.transferConversationOwner(conversationId, body);
    }
    changeMemberRole(conversationId, body) {
        return this.context.transportClient.conversation.changeConversationMemberRole(conversationId, body);
    }
    leave(conversationId) {
        return this.context.transportClient.conversation.leave(conversationId);
    }
    getReadCursor(conversationId) {
        return this.context.transportClient.conversation.getConversationReadCursor(conversationId);
    }
    updateReadCursor(conversationId, body) {
        return this.context.transportClient.conversation.updateConversationReadCursor(conversationId, body);
    }
    listMessages(conversationId) {
        return this.context.transportClient.conversation.listConversationMessages(conversationId);
    }
    postMessage(conversationId, body) {
        return this.context.transportClient.conversation.postConversationMessage(conversationId, body);
    }
    postText(conversationId, text, options = {}) {
        return this.postMessage(conversationId, buildTextMessageRequest(text, options));
    }
    publishSystemMessage(conversationId, body) {
        return this.context.transportClient.conversation.publishSystemChannelMessage(conversationId, body);
    }
    publishSystemText(conversationId, text, options = {}) {
        return this.publishSystemMessage(conversationId, buildTextMessageRequest(text, options));
    }
}
