import { CrawChatAuthModule } from './auth-module.js';
import { CrawChatConversationsModule } from './conversations-module.js';
import { CrawChatLiveModule } from './live-module.js';
import { CrawChatMediaModule } from './media-module.js';
import { CrawChatMessagesModule } from './messages-module.js';
import { CrawChatRtcModule } from './rtc-module.js';
import { CrawChatSdkContext, resolveCrawChatClientOptions } from './sdk-context.js';
import { CrawChatSyncModule } from './sync-module.js';
import type { SdkworkBackendClient } from '@sdkwork/craw-chat-backend-sdk';
import type {
  CrawChatClientMessage,
  CrawChatConnectOptions,
  CrawChatCreateAgentHandoffInput,
  CrawChatCreateAgentInput,
  CrawChatCreateAgentStateInput,
  CrawChatCreateAiImageGenerationInput,
  CrawChatCreateAiTextInput,
  CrawChatCreateAiVideoGenerationInput,
  CrawChatCreateCardInput,
  CrawChatCreateContactInput,
  CrawChatCreateCustomInput,
  CrawChatCreateDataInput,
  CrawChatCreateLinkInput,
  CrawChatCreateLocationInput,
  CrawChatCreateMediaInput,
  CrawChatCreateMusicInput,
  CrawChatCreateSignalInput,
  CrawChatCreateStickerInput,
  CrawChatCreateStreamReferenceInput,
  CrawChatCreateTextInput,
  CrawChatCreateToolResultInput,
  CrawChatCreateVoiceInput,
  CrawChatCreateWorkflowEventInput,
  CrawChatDecodedMessage,
  CrawChatDecodableMessageBody,
  CrawChatLiveConnection,
  CrawChatMessageKind,
  CrawChatMediaUploadOptions,
  CrawChatSdkClientOptions,
  CrawChatUploadedMediaAsset,
  CrawChatUploadAndSendMessageOptions,
  CrawChatUploadAndSendMessageResult,
  EditMessageRequest,
  EditTextMessageOptions,
  MessageMutationResult,
  PostMessageResult,
} from './types.js';

export class CrawChatSdkClient {
  private readonly context: CrawChatSdkContext;

  readonly generated: SdkworkBackendClient;
  readonly auth: CrawChatAuthModule;
  readonly portal: SdkworkBackendClient['portal'];
  readonly conversations: CrawChatConversationsModule;
  readonly messages: CrawChatMessagesModule;
  readonly media: CrawChatMediaModule;
  readonly live: CrawChatLiveModule;
  readonly sync: CrawChatSyncModule;
  readonly rtc: CrawChatRtcModule;

  constructor(options: CrawChatSdkClientOptions) {
    const resolved = resolveCrawChatClientOptions(options);
    this.context = new CrawChatSdkContext(
      resolved.backendClient,
      resolved.transport,
      resolved.webSocketFactory,
      resolved.authToken,
    );
    this.generated = resolved.backendClient as SdkworkBackendClient;
    this.auth = new CrawChatAuthModule(this.context);
    this.portal = this.generated.portal;
    this.conversations = new CrawChatConversationsModule(this.context);
    this.messages = new CrawChatMessagesModule(this.context);
    this.media = new CrawChatMediaModule(this.context);
    this.live = new CrawChatLiveModule(this.context);
    this.sync = new CrawChatSyncModule(this.context);
    this.rtc = new CrawChatRtcModule(this.context);
  }

  getApiBaseUrl(): string | undefined {
    return this.context.getApiBaseUrl();
  }

  getWebSocketBaseUrl(): string | undefined {
    return this.context.getWebSocketBaseUrl();
  }

  resolveRealtimeWebSocketUrl(path?: string): string | undefined {
    return this.context.resolveRealtimeWebSocketUrl(path);
  }

  createTextMessage(input: CrawChatCreateTextInput) {
    return this.messages.createText(input);
  }

  createImageMessage(input: CrawChatCreateMediaInput) {
    return this.messages.createImage(input);
  }

  createVideoMessage(input: CrawChatCreateMediaInput) {
    return this.messages.createVideo(input);
  }

  createAudioMessage(input: CrawChatCreateMediaInput) {
    return this.messages.createAudio(input);
  }

  createFileMessage(input: CrawChatCreateMediaInput) {
    return this.messages.createFile(input);
  }

  createDataMessage(input: CrawChatCreateDataInput) {
    return this.messages.createData(input);
  }

  createSignalMessage(input: CrawChatCreateSignalInput) {
    return this.messages.createSignal(input);
  }

  createStreamReferenceMessage(input: CrawChatCreateStreamReferenceInput) {
    return this.messages.createStreamReference(input);
  }

  createLocationMessage(input: CrawChatCreateLocationInput) {
    return this.messages.createLocation(input);
  }

  createLinkMessage(input: CrawChatCreateLinkInput) {
    return this.messages.createLink(input);
  }

  createCardMessage(input: CrawChatCreateCardInput) {
    return this.messages.createCard(input);
  }

  createMusicMessage(input: CrawChatCreateMusicInput) {
    return this.messages.createMusic(input);
  }

  createContactMessage(input: CrawChatCreateContactInput) {
    return this.messages.createContact(input);
  }

  createStickerMessage(input: CrawChatCreateStickerInput) {
    return this.messages.createSticker(input);
  }

  createVoiceMessage(input: CrawChatCreateVoiceInput) {
    return this.messages.createVoice(input);
  }

  createAgentMessage(input: CrawChatCreateAgentInput) {
    return this.messages.createAgent(input);
  }

  createAgentStateMessage(input: CrawChatCreateAgentStateInput) {
    return this.messages.createAgentState(input);
  }

  createAgentHandoffMessage(input: CrawChatCreateAgentHandoffInput) {
    return this.messages.createAgentHandoff(input);
  }

  createCustomMessage(input: CrawChatCreateCustomInput) {
    return this.messages.createCustom(input);
  }

  createAiTextMessage(input: CrawChatCreateAiTextInput) {
    return this.messages.createAiText(input);
  }

  createAiImageGenerationMessage(input: CrawChatCreateAiImageGenerationInput) {
    return this.messages.createAiImageGeneration(input);
  }

  createAiVideoGenerationMessage(input: CrawChatCreateAiVideoGenerationInput) {
    return this.messages.createAiVideoGeneration(input);
  }

  createToolResultMessage(input: CrawChatCreateToolResultInput) {
    return this.messages.createToolResult(input);
  }

  createWorkflowEventMessage(input: CrawChatCreateWorkflowEventInput) {
    return this.messages.createWorkflowEvent(input);
  }

  decodeMessage(body: CrawChatDecodableMessageBody): CrawChatDecodedMessage {
    return this.messages.decode(body);
  }

  send<TKind extends CrawChatMessageKind>(
    message: CrawChatClientMessage<TKind>,
  ): Promise<PostMessageResult> {
    return this.messages.send(message);
  }

  upload(options: CrawChatMediaUploadOptions): Promise<CrawChatUploadedMediaAsset> {
    return this.media.upload(options);
  }

  uploadAndSendMessage<TKind extends CrawChatMessageKind>(
    options: CrawChatUploadAndSendMessageOptions<TKind>,
  ): Promise<CrawChatUploadAndSendMessageResult<TKind>> {
    return this.messages.uploadAndSend(options);
  }

  editMessage(
    messageId: string | number,
    body: EditMessageRequest,
  ): Promise<MessageMutationResult> {
    return this.messages.edit(messageId, body);
  }

  editTextMessage(
    messageId: string | number,
    text: string,
    options: EditTextMessageOptions = {},
  ): Promise<MessageMutationResult> {
    return this.messages.editText(messageId, text, options);
  }

  recallMessage(messageId: string | number): Promise<MessageMutationResult> {
    return this.messages.recall(messageId);
  }

  connect(options: CrawChatConnectOptions = {}): Promise<CrawChatLiveConnection> {
    return this.live.connect(options);
  }
}

export default CrawChatSdkClient;
