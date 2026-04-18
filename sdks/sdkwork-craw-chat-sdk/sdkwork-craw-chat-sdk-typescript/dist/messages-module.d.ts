import type { CrawChatClientMessage, CrawChatCreateAgentInput, CrawChatCreateAgentHandoffInput, CrawChatCreateAgentStateInput, CrawChatCreateAiImageGenerationInput, CrawChatCreateAiTextInput, CrawChatCreateAiVideoGenerationInput, CrawChatCreateCardInput, CrawChatCreateContactInput, CrawChatCreateCustomInput, CrawChatCreateDataInput, CrawChatCreateLinkInput, CrawChatCreateLocationInput, CrawChatCreateMediaInput, CrawChatCreateMusicInput, CrawChatCreateSignalInput, CrawChatCreateStickerInput, CrawChatCreateStreamReferenceInput, CrawChatCreateTextInput, CrawChatCreateToolResultInput, CrawChatCreateVoiceInput, CrawChatCreateWorkflowEventInput, CrawChatDecodableMessageBody, CrawChatDecodedMessage, CrawChatMessageKind, CrawChatUploadAndSendMessageOptions, CrawChatUploadAndSendMessageResult, EditMessageRequest, EditTextMessageOptions, MessageMutationResult, PostMessageResult } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatMessagesModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    createText(input: CrawChatCreateTextInput): CrawChatClientMessage<'text'>;
    createImage(input: CrawChatCreateMediaInput): CrawChatClientMessage<'image'>;
    createVideo(input: CrawChatCreateMediaInput): CrawChatClientMessage<'video'>;
    createAudio(input: CrawChatCreateMediaInput): CrawChatClientMessage<'audio'>;
    createFile(input: CrawChatCreateMediaInput): CrawChatClientMessage<'file'>;
    createData(input: CrawChatCreateDataInput): CrawChatClientMessage<'data'>;
    createSignal(input: CrawChatCreateSignalInput): CrawChatClientMessage<'signal'>;
    createStreamReference(input: CrawChatCreateStreamReferenceInput): CrawChatClientMessage<'stream_ref'>;
    createLocation(input: CrawChatCreateLocationInput): CrawChatClientMessage<'location'>;
    createLink(input: CrawChatCreateLinkInput): CrawChatClientMessage<'link'>;
    createCard(input: CrawChatCreateCardInput): CrawChatClientMessage<'card'>;
    createMusic(input: CrawChatCreateMusicInput): CrawChatClientMessage<'music'>;
    createContact(input: CrawChatCreateContactInput): CrawChatClientMessage<'contact'>;
    createSticker(input: CrawChatCreateStickerInput): CrawChatClientMessage<'sticker'>;
    createVoice(input: CrawChatCreateVoiceInput): CrawChatClientMessage<'voice'>;
    createAgent(input: CrawChatCreateAgentInput): CrawChatClientMessage<'agent'>;
    createAgentState(input: CrawChatCreateAgentStateInput): CrawChatClientMessage<'agent_state'>;
    createAgentHandoff(input: CrawChatCreateAgentHandoffInput): CrawChatClientMessage<'agent_handoff'>;
    createCustom(input: CrawChatCreateCustomInput): CrawChatClientMessage<'custom'>;
    createAiText(input: CrawChatCreateAiTextInput): CrawChatClientMessage<'ai_text'>;
    createAiImageGeneration(input: CrawChatCreateAiImageGenerationInput): CrawChatClientMessage<'ai_image_generation'>;
    createAiVideoGeneration(input: CrawChatCreateAiVideoGenerationInput): CrawChatClientMessage<'ai_video_generation'>;
    createToolResult(input: CrawChatCreateToolResultInput): CrawChatClientMessage<'tool_result'>;
    createWorkflowEvent(input: CrawChatCreateWorkflowEventInput): CrawChatClientMessage<'workflow_event'>;
    decode(body: CrawChatDecodableMessageBody): CrawChatDecodedMessage;
    send<TKind extends CrawChatMessageKind>(message: CrawChatClientMessage<TKind>): Promise<PostMessageResult>;
    uploadAndSend<TKind extends CrawChatMessageKind>(options: CrawChatUploadAndSendMessageOptions<TKind>): Promise<CrawChatUploadAndSendMessageResult<TKind>>;
    edit(messageId: string | number, body: EditMessageRequest): Promise<MessageMutationResult>;
    editText(messageId: string | number, text: string, options?: EditTextMessageOptions): Promise<MessageMutationResult>;
    recall(messageId: string | number): Promise<MessageMutationResult>;
}
//# sourceMappingURL=messages-module.d.ts.map