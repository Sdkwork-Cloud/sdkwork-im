import type { AppendStreamFrameRequest, AppendTextFrameOptions, ImClientMessage, ImCreateAgentMessageOptions, ImCreateAgentHandoffMessageOptions, ImCreateAgentStateMessageOptions, ImCreateAiTextMessageOptions, ImCreateAiImageGenerationMessageOptions, ImCreateAiVideoGenerationMessageOptions, ImCreateCardMessageOptions, ImCreateContactMessageOptions, ImCreateCustomMessageOptions, ImCreateDataMessageOptions, ImCreateLinkMessageOptions, ImCreateLocationMessageOptions, ImCreateMediaMessageOptions, ImCreateMessageOptions, ImCreateMusicMessageOptions, ImCreateSignalMessageOptions, ImCreateStickerMessageOptions, ImCreateStreamReferenceMessageOptions, ImCreateToolResultMessageOptions, ImCreateVoiceMessageOptions, ImCreateWorkflowEventMessageOptions, ImMessageChannel, ImMessageKind, EditMessageRequest, EditTextMessageOptions, MediaResourceType, PostJsonRtcSignalOptions, PostMessageRequest, PostRtcSignalRequest, PostTextMessageOptions } from './types.js';
export declare const DEFAULT_MESSAGE_CHANNEL = "conversation";
export declare const DEFAULT_TEXT_FRAME_ENCODING = "text/plain; charset=utf-8";
interface ImInternalCreateRawMessageOptions {
    channel?: ImMessageChannel;
}
interface ImInternalRtcSignalEnvelope<TTransport extends string = string> {
    kind: 'rtc_signal';
    transport: TTransport;
    rtcSessionId: string | number;
    body: PostRtcSignalRequest;
}
export declare function buildTextMessageRequest(text: string, options?: PostTextMessageOptions): PostMessageRequest;
export declare function buildTextEditRequest(text: string, options?: EditTextMessageOptions): EditMessageRequest;
export declare function buildTextFrameRequest(options: AppendTextFrameOptions): AppendStreamFrameRequest;
export declare function buildJsonRtcSignalRequest(signalType: string, options: PostJsonRtcSignalOptions): PostRtcSignalRequest;
export declare function buildClientMessage<TKind extends ImMessageKind>(kind: TKind, conversationId: string | number, body: PostMessageRequest, options?: ImInternalCreateRawMessageOptions): ImClientMessage<TKind>;
export declare function buildTextClientMessage(conversationId: string | number, text: string, options?: ImCreateMessageOptions): ImClientMessage<'text'>;
export declare function buildMediaMessageRequest(mediaType: MediaResourceType, options: ImCreateMediaMessageOptions): PostMessageRequest;
export declare function buildMediaClientMessage<TKind extends 'image' | 'video' | 'audio' | 'file'>(kind: TKind, conversationId: string | number, options: ImCreateMediaMessageOptions): ImClientMessage<TKind>;
export declare function buildDataMessageRequest(options: ImCreateDataMessageOptions): PostMessageRequest;
export declare function buildDataClientMessage(conversationId: string | number, options: ImCreateDataMessageOptions): ImClientMessage<'data'>;
export declare function buildSignalMessageRequest(options: ImCreateSignalMessageOptions): PostMessageRequest;
export declare function buildSignalClientMessage(conversationId: string | number, options: ImCreateSignalMessageOptions): ImClientMessage<'signal'>;
export declare function buildStreamReferenceMessageRequest(options: ImCreateStreamReferenceMessageOptions): PostMessageRequest;
export declare function buildStreamReferenceClientMessage(conversationId: string | number, options: ImCreateStreamReferenceMessageOptions): ImClientMessage<'stream_ref'>;
export declare function buildLocationClientMessage(conversationId: string | number, options: ImCreateLocationMessageOptions): ImClientMessage<'location'>;
export declare function buildLinkClientMessage(conversationId: string | number, options: ImCreateLinkMessageOptions): ImClientMessage<'link'>;
export declare function buildCardClientMessage(conversationId: string | number, options: ImCreateCardMessageOptions): ImClientMessage<'card'>;
export declare function buildMusicClientMessage(conversationId: string | number, options: ImCreateMusicMessageOptions): ImClientMessage<'music'>;
export declare function buildContactClientMessage(conversationId: string | number, options: ImCreateContactMessageOptions): ImClientMessage<'contact'>;
export declare function buildStickerClientMessage(conversationId: string | number, options: ImCreateStickerMessageOptions): ImClientMessage<'sticker'>;
export declare function buildVoiceClientMessage(conversationId: string | number, options: ImCreateVoiceMessageOptions): ImClientMessage<'voice'>;
export declare function buildAgentClientMessage(conversationId: string | number, options: ImCreateAgentMessageOptions): ImClientMessage<'agent'>;
export declare function buildAgentStateClientMessage(conversationId: string | number, options: ImCreateAgentStateMessageOptions): ImClientMessage<'agent_state'>;
export declare function buildAgentHandoffClientMessage(conversationId: string | number, options: ImCreateAgentHandoffMessageOptions): ImClientMessage<'agent_handoff'>;
export declare function buildCustomClientMessage(conversationId: string | number, options: ImCreateCustomMessageOptions): ImClientMessage<'custom'>;
export declare function buildAiTextClientMessage(conversationId: string | number, options: ImCreateAiTextMessageOptions): ImClientMessage<'ai_text'>;
export declare function buildAiImageGenerationClientMessage(conversationId: string | number, options: ImCreateAiImageGenerationMessageOptions): ImClientMessage<'ai_image_generation'>;
export declare function buildToolResultClientMessage(conversationId: string | number, options: ImCreateToolResultMessageOptions): ImClientMessage<'tool_result'>;
export declare function buildWorkflowEventClientMessage(conversationId: string | number, options: ImCreateWorkflowEventMessageOptions): ImClientMessage<'workflow_event'>;
export declare function buildAiVideoGenerationClientMessage(conversationId: string | number, options: ImCreateAiVideoGenerationMessageOptions): ImClientMessage<'ai_video_generation'>;
export declare function buildJsonRtcSignalEnvelope(rtcSessionId: string | number, signalType: string, options: PostJsonRtcSignalOptions): ImInternalRtcSignalEnvelope<'json'>;
export {};
//# sourceMappingURL=builders.d.ts.map