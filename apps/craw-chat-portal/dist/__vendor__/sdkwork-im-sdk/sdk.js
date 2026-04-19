import { ImAuthModule } from './auth-module.js';
import { ImConversationsModule } from './conversations-module.js';
import { ImLiveModule } from './live-module.js';
import { ImMediaModule } from './media-module.js';
import { ImMessagesModule } from './messages-module.js';
import { ImRtcModule } from './rtc-module.js';
import { ImSdkContext, resolveImClientOptions } from './sdk-context.js';
import { ImSyncModule } from './sync-module.js';
export class ImSdkClient {
    context;
    auth;
    portal;
    session;
    presence;
    realtime;
    device;
    inbox;
    conversations;
    messages;
    media;
    live;
    sync;
    stream;
    rtc;
    constructor(options) {
        const resolved = resolveImClientOptions(options);
        const transportClient = resolved.transportClient;
        this.context = new ImSdkContext(transportClient, resolved.transport, resolved.webSocketFactory, resolved.authToken);
        this.auth = new ImAuthModule(this.context);
        this.portal = transportClient.portal;
        this.session = transportClient.session;
        this.presence = transportClient.presence;
        this.realtime = transportClient.realtime;
        this.device = transportClient.device;
        this.inbox = transportClient.inbox;
        this.conversations = new ImConversationsModule(this.context);
        this.messages = new ImMessagesModule(this.context);
        this.media = new ImMediaModule(this.context);
        this.live = new ImLiveModule(this.context);
        this.sync = new ImSyncModule(this.context);
        this.stream = transportClient.stream;
        this.rtc = new ImRtcModule(this.context);
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
export default ImSdkClient;
