import { CrawChatAuthModule } from './auth-module.js';
import { CrawChatConversationsModule } from './conversations-module.js';
import { CrawChatLiveModule } from './live-module.js';
import { CrawChatMediaModule } from './media-module.js';
import { CrawChatMessagesModule } from './messages-module.js';
import { CrawChatRtcModule } from './rtc-module.js';
import { CrawChatSdkContext, resolveCrawChatClientOptions } from './sdk-context.js';
import { CrawChatSyncModule } from './sync-module.js';
export class CrawChatSdkClient {
    context;
    generated;
    auth;
    portal;
    conversations;
    messages;
    media;
    live;
    sync;
    rtc;
    constructor(options) {
        const resolved = resolveCrawChatClientOptions(options);
        this.context = new CrawChatSdkContext(resolved.backendClient, resolved.transport, resolved.webSocketFactory, resolved.authToken);
        this.generated = resolved.backendClient;
        this.auth = new CrawChatAuthModule(this.context);
        this.portal = this.generated.portal;
        this.conversations = new CrawChatConversationsModule(this.context);
        this.messages = new CrawChatMessagesModule(this.context);
        this.media = new CrawChatMediaModule(this.context);
        this.live = new CrawChatLiveModule(this.context);
        this.sync = new CrawChatSyncModule(this.context);
        this.rtc = new CrawChatRtcModule(this.context);
    }
    getApiBaseUrl() {
        return this.context.getApiBaseUrl();
    }
    getWebSocketBaseUrl() {
        return this.context.getWebSocketBaseUrl();
    }
    resolveRealtimeWebSocketUrl(path) {
        return this.context.resolveRealtimeWebSocketUrl(path);
    }
    createTextMessage(input) {
        return this.messages.createText(input);
    }
    createImageMessage(input) {
        return this.messages.createImage(input);
    }
    createVideoMessage(input) {
        return this.messages.createVideo(input);
    }
    createAudioMessage(input) {
        return this.messages.createAudio(input);
    }
    createFileMessage(input) {
        return this.messages.createFile(input);
    }
    createDataMessage(input) {
        return this.messages.createData(input);
    }
    createSignalMessage(input) {
        return this.messages.createSignal(input);
    }
    createStreamReferenceMessage(input) {
        return this.messages.createStreamReference(input);
    }
    createLocationMessage(input) {
        return this.messages.createLocation(input);
    }
    createLinkMessage(input) {
        return this.messages.createLink(input);
    }
    createCardMessage(input) {
        return this.messages.createCard(input);
    }
    createMusicMessage(input) {
        return this.messages.createMusic(input);
    }
    createContactMessage(input) {
        return this.messages.createContact(input);
    }
    createStickerMessage(input) {
        return this.messages.createSticker(input);
    }
    createVoiceMessage(input) {
        return this.messages.createVoice(input);
    }
    createAgentMessage(input) {
        return this.messages.createAgent(input);
    }
    createAgentStateMessage(input) {
        return this.messages.createAgentState(input);
    }
    createAgentHandoffMessage(input) {
        return this.messages.createAgentHandoff(input);
    }
    createCustomMessage(input) {
        return this.messages.createCustom(input);
    }
    createAiTextMessage(input) {
        return this.messages.createAiText(input);
    }
    createAiImageGenerationMessage(input) {
        return this.messages.createAiImageGeneration(input);
    }
    createAiVideoGenerationMessage(input) {
        return this.messages.createAiVideoGeneration(input);
    }
    createToolResultMessage(input) {
        return this.messages.createToolResult(input);
    }
    createWorkflowEventMessage(input) {
        return this.messages.createWorkflowEvent(input);
    }
    decodeMessage(body) {
        return this.messages.decode(body);
    }
    send(message) {
        return this.messages.send(message);
    }
    upload(options) {
        return this.media.upload(options);
    }
    uploadAndSendMessage(options) {
        return this.messages.uploadAndSend(options);
    }
    editMessage(messageId, body) {
        return this.messages.edit(messageId, body);
    }
    editTextMessage(messageId, text, options = {}) {
        return this.messages.editText(messageId, text, options);
    }
    recallMessage(messageId) {
        return this.messages.recall(messageId);
    }
    connect(options = {}) {
        return this.live.connect(options);
    }
}
export default CrawChatSdkClient;
