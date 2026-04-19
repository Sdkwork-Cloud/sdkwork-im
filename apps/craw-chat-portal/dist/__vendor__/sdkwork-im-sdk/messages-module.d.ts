import type { ImClientMessage, ImCreateAgentInput, ImCreateAgentHandoffInput, ImCreateAgentStateInput, ImCreateAiImageGenerationInput, ImCreateAiTextInput, ImCreateAiVideoGenerationInput, ImCreateCardInput, ImCreateContactInput, ImCreateCustomInput, ImCreateDataInput, ImCreateLinkInput, ImCreateLocationInput, ImCreateMediaInput, ImCreateMusicInput, ImCreateSignalInput, ImCreateStickerInput, ImCreateStreamReferenceInput, ImCreateTextInput, ImCreateToolResultInput, ImCreateVoiceInput, ImCreateWorkflowEventInput, ImDecodableMessageBody, ImDecodedMessage, ImMessageKind, ImUploadAndSendMessageOptions, ImUploadAndSendMessageResult, EditMessageRequest, EditTextMessageOptions, MessageMutationResult, PostMessageResult } from './types.js';
import type { ImSdkContext } from './sdk-context.js';
export declare class ImMessagesModule {
    private readonly context;
    constructor(context: ImSdkContext);
    createText(input: ImCreateTextInput): ImClientMessage<'text'>;
    createImage(input: ImCreateMediaInput): ImClientMessage<'image'>;
    createVideo(input: ImCreateMediaInput): ImClientMessage<'video'>;
    createAudio(input: ImCreateMediaInput): ImClientMessage<'audio'>;
    createFile(input: ImCreateMediaInput): ImClientMessage<'file'>;
    createData(input: ImCreateDataInput): ImClientMessage<'data'>;
    createSignal(input: ImCreateSignalInput): ImClientMessage<'signal'>;
    createStreamReference(input: ImCreateStreamReferenceInput): ImClientMessage<'stream_ref'>;
    createLocation(input: ImCreateLocationInput): ImClientMessage<'location'>;
    createLink(input: ImCreateLinkInput): ImClientMessage<'link'>;
    createCard(input: ImCreateCardInput): ImClientMessage<'card'>;
    createMusic(input: ImCreateMusicInput): ImClientMessage<'music'>;
    createContact(input: ImCreateContactInput): ImClientMessage<'contact'>;
    createSticker(input: ImCreateStickerInput): ImClientMessage<'sticker'>;
    createVoice(input: ImCreateVoiceInput): ImClientMessage<'voice'>;
    createAgent(input: ImCreateAgentInput): ImClientMessage<'agent'>;
    createAgentState(input: ImCreateAgentStateInput): ImClientMessage<'agent_state'>;
    createAgentHandoff(input: ImCreateAgentHandoffInput): ImClientMessage<'agent_handoff'>;
    createCustom(input: ImCreateCustomInput): ImClientMessage<'custom'>;
    createAiText(input: ImCreateAiTextInput): ImClientMessage<'ai_text'>;
    createAiImageGeneration(input: ImCreateAiImageGenerationInput): ImClientMessage<'ai_image_generation'>;
    createAiVideoGeneration(input: ImCreateAiVideoGenerationInput): ImClientMessage<'ai_video_generation'>;
    createToolResult(input: ImCreateToolResultInput): ImClientMessage<'tool_result'>;
    createWorkflowEvent(input: ImCreateWorkflowEventInput): ImClientMessage<'workflow_event'>;
    decode(body: ImDecodableMessageBody): ImDecodedMessage;
    send<TKind extends ImMessageKind>(message: ImClientMessage<TKind>): Promise<PostMessageResult>;
    uploadAndSend<TKind extends ImMessageKind>(options: ImUploadAndSendMessageOptions<TKind>): Promise<ImUploadAndSendMessageResult<TKind>>;
    edit(messageId: string | number, body: EditMessageRequest): Promise<MessageMutationResult>;
    editText(messageId: string | number, text: string, options?: EditTextMessageOptions): Promise<MessageMutationResult>;
    recall(messageId: string | number): Promise<MessageMutationResult>;
}
//# sourceMappingURL=messages-module.d.ts.map