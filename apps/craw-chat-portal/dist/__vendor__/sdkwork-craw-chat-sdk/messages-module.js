import { buildAgentClientMessage, buildAgentHandoffClientMessage, buildAgentStateClientMessage, buildAiImageGenerationClientMessage, buildAiTextClientMessage, buildAiVideoGenerationClientMessage, buildCardClientMessage, buildContactClientMessage, buildCustomClientMessage, buildDataClientMessage, buildLinkClientMessage, buildLocationClientMessage, buildMediaClientMessage, buildMusicClientMessage, buildSignalClientMessage, buildStickerClientMessage, buildStreamReferenceClientMessage, buildTextClientMessage, buildToolResultClientMessage, buildVoiceClientMessage, buildWorkflowEventClientMessage, buildTextEditRequest, } from './builders.js';
import { decodeMessageBody } from './message-codec.js';
import { performPresignedMediaUpload } from './media-upload-runtime.js';
export class CrawChatMessagesModule {
    context;
    constructor(context) {
        this.context = context;
    }
    createText(input) {
        return buildTextClientMessage(input.conversationId, input.text, withoutConversationId(input));
    }
    createImage(input) {
        return buildMediaClientMessage('image', input.conversationId, withoutConversationId(input));
    }
    createVideo(input) {
        return buildMediaClientMessage('video', input.conversationId, withoutConversationId(input));
    }
    createAudio(input) {
        return buildMediaClientMessage('audio', input.conversationId, withoutConversationId(input));
    }
    createFile(input) {
        return buildMediaClientMessage('file', input.conversationId, withoutConversationId(input));
    }
    createData(input) {
        return buildDataClientMessage(input.conversationId, withoutConversationId(input));
    }
    createSignal(input) {
        return buildSignalClientMessage(input.conversationId, withoutConversationId(input));
    }
    createStreamReference(input) {
        return buildStreamReferenceClientMessage(input.conversationId, withoutConversationId(input));
    }
    createLocation(input) {
        return buildLocationClientMessage(input.conversationId, withoutConversationId(input));
    }
    createLink(input) {
        return buildLinkClientMessage(input.conversationId, withoutConversationId(input));
    }
    createCard(input) {
        return buildCardClientMessage(input.conversationId, withoutConversationId(input));
    }
    createMusic(input) {
        return buildMusicClientMessage(input.conversationId, withoutConversationId(input));
    }
    createContact(input) {
        return buildContactClientMessage(input.conversationId, withoutConversationId(input));
    }
    createSticker(input) {
        return buildStickerClientMessage(input.conversationId, withoutConversationId(input));
    }
    createVoice(input) {
        return buildVoiceClientMessage(input.conversationId, withoutConversationId(input));
    }
    createAgent(input) {
        return buildAgentClientMessage(input.conversationId, withoutConversationId(input));
    }
    createAgentState(input) {
        return buildAgentStateClientMessage(input.conversationId, withoutConversationId(input));
    }
    createAgentHandoff(input) {
        return buildAgentHandoffClientMessage(input.conversationId, withoutConversationId(input));
    }
    createCustom(input) {
        return buildCustomClientMessage(input.conversationId, withoutConversationId(input));
    }
    createAiText(input) {
        return buildAiTextClientMessage(input.conversationId, withoutConversationId(input));
    }
    createAiImageGeneration(input) {
        return buildAiImageGenerationClientMessage(input.conversationId, withoutConversationId(input));
    }
    createAiVideoGeneration(input) {
        return buildAiVideoGenerationClientMessage(input.conversationId, withoutConversationId(input));
    }
    createToolResult(input) {
        return buildToolResultClientMessage(input.conversationId, withoutConversationId(input));
    }
    createWorkflowEvent(input) {
        return buildWorkflowEventClientMessage(input.conversationId, withoutConversationId(input));
    }
    decode(body) {
        return decodeMessageBody(body);
    }
    send(message) {
        if (message.target.channel === 'system') {
            return this.context.backendClient.conversation.publishSystemChannelMessage(message.target.conversationId, message.body);
        }
        return this.context.backendClient.conversation.postConversationMessage(message.target.conversationId, message.body);
    }
    async uploadAndSend(options) {
        const uploaded = await performPresignedMediaUpload(this.context, options.upload);
        const message = options.createMessage(uploaded);
        const delivery = await this.send(message);
        return {
            ...uploaded,
            message,
            delivery,
        };
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
function withoutConversationId(input) {
    const { conversationId: _conversationId, ...rest } = input;
    return rest;
}
