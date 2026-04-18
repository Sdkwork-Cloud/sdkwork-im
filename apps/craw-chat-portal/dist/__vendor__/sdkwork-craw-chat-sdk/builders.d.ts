import type { AppendStreamFrameRequest, AppendTextFrameOptions, CrawChatClientMessage, CrawChatCreateAgentMessageOptions, CrawChatCreateAgentHandoffMessageOptions, CrawChatCreateAgentStateMessageOptions, CrawChatCreateAiTextMessageOptions, CrawChatCreateAiImageGenerationMessageOptions, CrawChatCreateAiVideoGenerationMessageOptions, CrawChatCreateCardMessageOptions, CrawChatCreateContactMessageOptions, CrawChatCreateCustomMessageOptions, CrawChatCreateDataMessageOptions, CrawChatCreateLinkMessageOptions, CrawChatCreateLocationMessageOptions, CrawChatCreateMediaMessageOptions, CrawChatCreateMessageOptions, CrawChatCreateMusicMessageOptions, CrawChatCreateSignalMessageOptions, CrawChatCreateStickerMessageOptions, CrawChatCreateStreamReferenceMessageOptions, CrawChatCreateToolResultMessageOptions, CrawChatCreateVoiceMessageOptions, CrawChatCreateWorkflowEventMessageOptions, CrawChatMessageChannel, CrawChatMessageKind, EditMessageRequest, EditTextMessageOptions, MediaResourceType, PostJsonRtcSignalOptions, PostMessageRequest, PostRtcSignalRequest, PostTextMessageOptions } from './types.js';
export declare const DEFAULT_MESSAGE_CHANNEL = "conversation";
export declare const DEFAULT_TEXT_FRAME_ENCODING = "text/plain; charset=utf-8";
interface CrawChatInternalCreateRawMessageOptions {
    channel?: CrawChatMessageChannel;
}
interface CrawChatInternalRtcSignalEnvelope<TTransport extends string = string> {
    kind: 'rtc_signal';
    transport: TTransport;
    rtcSessionId: string | number;
    body: PostRtcSignalRequest;
}
export declare function buildTextMessageRequest(text: string, options?: PostTextMessageOptions): PostMessageRequest;
export declare function buildTextEditRequest(text: string, options?: EditTextMessageOptions): EditMessageRequest;
export declare function buildTextFrameRequest(options: AppendTextFrameOptions): AppendStreamFrameRequest;
export declare function buildJsonRtcSignalRequest(signalType: string, options: PostJsonRtcSignalOptions): PostRtcSignalRequest;
export declare function buildClientMessage<TKind extends CrawChatMessageKind>(kind: TKind, conversationId: string | number, body: PostMessageRequest, options?: CrawChatInternalCreateRawMessageOptions): CrawChatClientMessage<TKind>;
export declare function buildTextClientMessage(conversationId: string | number, text: string, options?: CrawChatCreateMessageOptions): CrawChatClientMessage<'text'>;
export declare function buildMediaMessageRequest(mediaType: MediaResourceType, options: CrawChatCreateMediaMessageOptions): PostMessageRequest;
export declare function buildMediaClientMessage<TKind extends 'image' | 'video' | 'audio' | 'file'>(kind: TKind, conversationId: string | number, options: CrawChatCreateMediaMessageOptions): CrawChatClientMessage<TKind>;
export declare function buildDataMessageRequest(options: CrawChatCreateDataMessageOptions): PostMessageRequest;
export declare function buildDataClientMessage(conversationId: string | number, options: CrawChatCreateDataMessageOptions): CrawChatClientMessage<'data'>;
export declare function buildSignalMessageRequest(options: CrawChatCreateSignalMessageOptions): PostMessageRequest;
export declare function buildSignalClientMessage(conversationId: string | number, options: CrawChatCreateSignalMessageOptions): CrawChatClientMessage<'signal'>;
export declare function buildStreamReferenceMessageRequest(options: CrawChatCreateStreamReferenceMessageOptions): PostMessageRequest;
export declare function buildStreamReferenceClientMessage(conversationId: string | number, options: CrawChatCreateStreamReferenceMessageOptions): CrawChatClientMessage<'stream_ref'>;
export declare function buildLocationClientMessage(conversationId: string | number, options: CrawChatCreateLocationMessageOptions): CrawChatClientMessage<'location'>;
export declare function buildLinkClientMessage(conversationId: string | number, options: CrawChatCreateLinkMessageOptions): CrawChatClientMessage<'link'>;
export declare function buildCardClientMessage(conversationId: string | number, options: CrawChatCreateCardMessageOptions): CrawChatClientMessage<'card'>;
export declare function buildMusicClientMessage(conversationId: string | number, options: CrawChatCreateMusicMessageOptions): CrawChatClientMessage<'music'>;
export declare function buildContactClientMessage(conversationId: string | number, options: CrawChatCreateContactMessageOptions): CrawChatClientMessage<'contact'>;
export declare function buildStickerClientMessage(conversationId: string | number, options: CrawChatCreateStickerMessageOptions): CrawChatClientMessage<'sticker'>;
export declare function buildVoiceClientMessage(conversationId: string | number, options: CrawChatCreateVoiceMessageOptions): CrawChatClientMessage<'voice'>;
export declare function buildAgentClientMessage(conversationId: string | number, options: CrawChatCreateAgentMessageOptions): CrawChatClientMessage<'agent'>;
export declare function buildAgentStateClientMessage(conversationId: string | number, options: CrawChatCreateAgentStateMessageOptions): CrawChatClientMessage<'agent_state'>;
export declare function buildAgentHandoffClientMessage(conversationId: string | number, options: CrawChatCreateAgentHandoffMessageOptions): CrawChatClientMessage<'agent_handoff'>;
export declare function buildCustomClientMessage(conversationId: string | number, options: CrawChatCreateCustomMessageOptions): CrawChatClientMessage<'custom'>;
export declare function buildAiTextClientMessage(conversationId: string | number, options: CrawChatCreateAiTextMessageOptions): CrawChatClientMessage<'ai_text'>;
export declare function buildAiImageGenerationClientMessage(conversationId: string | number, options: CrawChatCreateAiImageGenerationMessageOptions): CrawChatClientMessage<'ai_image_generation'>;
export declare function buildToolResultClientMessage(conversationId: string | number, options: CrawChatCreateToolResultMessageOptions): CrawChatClientMessage<'tool_result'>;
export declare function buildWorkflowEventClientMessage(conversationId: string | number, options: CrawChatCreateWorkflowEventMessageOptions): CrawChatClientMessage<'workflow_event'>;
export declare function buildAiVideoGenerationClientMessage(conversationId: string | number, options: CrawChatCreateAiVideoGenerationMessageOptions): CrawChatClientMessage<'ai_video_generation'>;
export declare function buildJsonRtcSignalEnvelope(rtcSessionId: string | number, signalType: string, options: PostJsonRtcSignalOptions): CrawChatInternalRtcSignalEnvelope<'json'>;
export {};
//# sourceMappingURL=builders.d.ts.map