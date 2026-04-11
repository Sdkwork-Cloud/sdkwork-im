import { buildTextEditRequest } from './builders.js';
export class CrawChatMessagesModule {
    context;
    constructor(context) {
        this.context = context;
    }
    edit(messageId, body) {
        return this.context.backendClient.message.edit(messageId, body);
    }
    editText(messageId, text, options = {}) {
        return this.edit(messageId, buildTextEditRequest(text, options));
    }
    recall(messageId) {
        return this.context.backendClient.message.recall(messageId);
    }
}
