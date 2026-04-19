import { buildCustomMessageSchema, CRAW_CHAT_JSON_ENCODING, CRAW_CHAT_MESSAGE_SCHEMA_AGENT, CRAW_CHAT_MESSAGE_SCHEMA_AGENT_HANDOFF, CRAW_CHAT_MESSAGE_SCHEMA_AGENT_STATE, CRAW_CHAT_MESSAGE_SCHEMA_AI_TEXT, CRAW_CHAT_MESSAGE_SCHEMA_AI_IMAGE_GENERATION, CRAW_CHAT_MESSAGE_SCHEMA_AI_VIDEO_GENERATION, CRAW_CHAT_MESSAGE_SCHEMA_CARD, CRAW_CHAT_MESSAGE_SCHEMA_CONTACT, CRAW_CHAT_MESSAGE_SCHEMA_LINK, CRAW_CHAT_MESSAGE_SCHEMA_LOCATION, CRAW_CHAT_MESSAGE_SCHEMA_MUSIC, CRAW_CHAT_MESSAGE_SCHEMA_STICKER, CRAW_CHAT_MESSAGE_SCHEMA_TOOL_RESULT, CRAW_CHAT_MESSAGE_SCHEMA_VOICE, CRAW_CHAT_MESSAGE_SCHEMA_WORKFLOW_EVENT, } from './message-standards.js';
import { ImSdkError } from './errors.js';
export const DEFAULT_MESSAGE_CHANNEL = 'conversation';
export const DEFAULT_TEXT_FRAME_ENCODING = 'text/plain; charset=utf-8';
export function buildTextMessageRequest(text, options = {}) {
    return {
        ...options,
        text,
    };
}
export function buildTextEditRequest(text, options = {}) {
    return {
        ...options,
        text,
    };
}
export function buildTextFrameRequest(options) {
    return {
        frameSeq: options.frameSeq,
        frameType: 'text',
        schemaRef: options.schemaRef,
        encoding: options.encoding ?? DEFAULT_TEXT_FRAME_ENCODING,
        payload: options.text,
        attributes: options.attributes,
    };
}
export function buildJsonRtcSignalRequest(signalType, options) {
    return {
        signalType,
        schemaRef: options.schemaRef,
        signalingStreamId: options.signalingStreamId,
        payload: JSON.stringify(options.payload ?? null, null, options.pretty ? 2 : 0),
    };
}
export function buildClientMessage(kind, conversationId, body, options = {}) {
    return {
        kind,
        target: {
            conversationId,
            channel: resolveMessageChannel(options.channel),
        },
        body,
    };
}
export function buildTextClientMessage(conversationId, text, options = {}) {
    const { channel, ...messageOptions } = options;
    return buildClientMessage('text', conversationId, buildTextMessageRequest(text, messageOptions), { channel });
}
export function buildMediaMessageRequest(mediaType, options) {
    const { channel: _channel, text, mediaAssetId, resource, schemaRef, encoding, payload, ...messageOptions } = options;
    return {
        ...messageOptions,
        text,
        parts: [
            buildMediaContentPart(mediaType, {
                mediaAssetId,
                resource,
                schemaRef,
                encoding,
                payload,
            }),
        ],
    };
}
export function buildMediaClientMessage(kind, conversationId, options) {
    return buildClientMessage(kind, conversationId, buildMediaMessageRequest(kind, options), { channel: options.channel });
}
export function buildDataMessageRequest(options) {
    const { channel: _channel, text, schemaRef, encoding, payload, ...messageOptions } = options;
    return {
        ...messageOptions,
        text,
        parts: [
            omitUndefined({
                kind: 'data',
                schemaRef,
                encoding,
                payload,
            }),
        ],
    };
}
export function buildDataClientMessage(conversationId, options) {
    return buildClientMessage('data', conversationId, buildDataMessageRequest(options), { channel: options.channel });
}
export function buildSignalMessageRequest(options) {
    const { channel: _channel, text, signalType, schemaRef, encoding, payload, state, ...messageOptions } = options;
    return {
        ...messageOptions,
        text,
        parts: [
            omitUndefined({
                kind: 'signal',
                signalType,
                schemaRef,
                encoding,
                payload,
                state,
            }),
        ],
    };
}
export function buildSignalClientMessage(conversationId, options) {
    return buildClientMessage('signal', conversationId, buildSignalMessageRequest(options), { channel: options.channel });
}
export function buildStreamReferenceMessageRequest(options) {
    const { channel: _channel, text, streamId, streamType, state, ...messageOptions } = options;
    return {
        ...messageOptions,
        text,
        parts: [
            omitUndefined({
                kind: 'stream_ref',
                streamId: String(streamId),
                streamType,
                state,
            }),
        ],
    };
}
export function buildStreamReferenceClientMessage(conversationId, options) {
    return buildClientMessage('stream_ref', conversationId, buildStreamReferenceMessageRequest(options), { channel: options.channel });
}
export function buildLocationClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('location', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_LOCATION, {
        latitude: options.latitude,
        longitude: options.longitude,
        name: options.name,
        address: options.address,
        mapUrl: options.mapUrl,
    }, options);
}
export function buildLinkClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('link', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_LINK, {
        title: options.title,
        url: options.url,
        description: options.description,
        imageUrl: options.imageUrl,
        siteName: options.siteName,
    }, options);
}
export function buildCardClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('card', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_CARD, {
        title: options.title,
        subtitle: options.subtitle,
        imageUrl: options.imageUrl,
        actions: options.actions ?? [],
    }, options);
}
export function buildMusicClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('music', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_MUSIC, {
        title: options.title,
        artist: options.artist,
        album: options.album,
        url: options.url,
        coverUrl: options.coverUrl,
        durationSeconds: options.durationSeconds,
    }, options);
}
export function buildContactClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('contact', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_CONTACT, {
        displayName: options.displayName,
        avatarUrl: options.avatarUrl,
        description: options.description,
        profileUrl: options.profileUrl,
        contactId: options.contactId,
    }, options);
}
export function buildStickerClientMessage(conversationId, options) {
    return buildStructuredMediaClientMessage('sticker', 'image', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_STICKER, {
        stickerId: options.stickerId,
        packId: options.packId,
        emoji: options.emoji,
    }, options);
}
export function buildVoiceClientMessage(conversationId, options) {
    return buildStructuredMediaClientMessage('voice', 'audio', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_VOICE, {
        durationSeconds: options.durationSeconds,
        transcription: options.transcription,
        waveform: options.waveform,
    }, options);
}
export function buildAgentClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('agent', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_AGENT, {
        agentId: options.agentId,
        agentName: options.agentName,
        stage: options.stage,
        status: options.status,
        capabilities: options.capabilities,
    }, options);
}
export function buildAgentStateClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('agent_state', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_AGENT_STATE, {
        agentId: options.agentId,
        agentName: options.agentName,
        stage: options.stage,
        status: options.status,
        capabilities: options.capabilities,
    }, options);
}
export function buildAgentHandoffClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('agent_handoff', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_AGENT_HANDOFF, {
        handoffId: options.handoffId,
        fromAgentId: options.fromAgentId,
        fromAgentName: options.fromAgentName,
        toAgentId: options.toAgentId,
        toAgentName: options.toAgentName,
        reason: options.reason,
        status: options.status,
    }, options);
}
export function buildCustomClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('custom', conversationId, buildCustomMessageSchema(options.customType), options.data ?? null, options);
}
export function buildAiTextClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('ai_text', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_AI_TEXT, {
        prompt: options.prompt,
        status: options.status,
        model: options.model,
        revisedPrompt: options.revisedPrompt,
    }, options);
}
export function buildAiImageGenerationClientMessage(conversationId, options) {
    return buildStructuredMediaClientMessage('ai_image_generation', 'image', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_AI_IMAGE_GENERATION, {
        prompt: options.prompt,
        status: options.status,
        model: options.model,
        revisedPrompt: options.revisedPrompt,
    }, options);
}
export function buildToolResultClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('tool_result', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_TOOL_RESULT, {
        toolName: options.toolName,
        invocationId: options.invocationId,
        status: options.status,
        output: options.output,
        error: options.error,
    }, options);
}
export function buildWorkflowEventClientMessage(conversationId, options) {
    return buildStructuredDataClientMessage('workflow_event', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_WORKFLOW_EVENT, {
        workflowId: options.workflowId,
        eventName: options.eventName,
        stage: options.stage,
        status: options.status,
        data: options.data,
    }, options);
}
export function buildAiVideoGenerationClientMessage(conversationId, options) {
    return buildStructuredMediaClientMessage('ai_video_generation', 'video', conversationId, CRAW_CHAT_MESSAGE_SCHEMA_AI_VIDEO_GENERATION, {
        prompt: options.prompt,
        status: options.status,
        model: options.model,
        durationSeconds: options.durationSeconds,
    }, options);
}
export function buildJsonRtcSignalEnvelope(rtcSessionId, signalType, options) {
    return {
        kind: 'rtc_signal',
        transport: 'json',
        rtcSessionId,
        body: buildJsonRtcSignalRequest(signalType, options),
    };
}
function buildMediaContentPart(mediaType, options) {
    if (options.mediaAssetId == null && !options.resource) {
        throw new ImSdkError('media_reference_required', `${mediaType} message requires mediaAssetId or resource`, {
            mediaType,
        });
    }
    return omitUndefined({
        kind: 'media',
        schemaRef: options.schemaRef,
        encoding: options.encoding,
        payload: options.payload,
        mediaAssetId: options.mediaAssetId != null ? String(options.mediaAssetId) : undefined,
        resource: options.resource
            ? {
                ...options.resource,
                type: mediaType,
            }
            : {
                type: mediaType,
            },
    });
}
function buildStructuredDataClientMessage(kind, conversationId, schemaRef, payload, options) {
    return buildClientMessage(kind, conversationId, buildStructuredDataMessageRequest(schemaRef, payload, options), { channel: options.channel });
}
function buildStructuredDataMessageRequest(schemaRef, payload, options) {
    const { text } = options;
    return {
        ...pickMessageRequestOptions(options),
        text,
        parts: [buildJsonDataContentPart(schemaRef, payload)],
    };
}
function buildStructuredMediaClientMessage(kind, mediaType, conversationId, schemaRef, payload, options) {
    return buildClientMessage(kind, conversationId, buildStructuredMediaMessageRequest(mediaType, schemaRef, payload, options), { channel: options.channel });
}
function buildStructuredMediaMessageRequest(mediaType, schemaRef, payload, options) {
    const { text, mediaAssetId, resource } = options;
    return {
        ...pickMessageRequestOptions(options),
        text,
        parts: [
            buildJsonDataContentPart(schemaRef, payload),
            buildMediaContentPart(mediaType, {
                mediaAssetId,
                resource,
            }),
        ],
    };
}
function buildJsonDataContentPart(schemaRef, payload) {
    return {
        kind: 'data',
        schemaRef,
        encoding: CRAW_CHAT_JSON_ENCODING,
        payload: JSON.stringify(payload ?? null),
    };
}
function pickMessageRequestOptions(options) {
    return omitUndefined({
        clientMsgId: options.clientMsgId,
        summary: options.summary,
        renderHints: options.renderHints,
    });
}
function resolveMessageChannel(channel) {
    return channel ?? DEFAULT_MESSAGE_CHANNEL;
}
function omitUndefined(value) {
    return Object.fromEntries(Object.entries(value).filter(([, entryValue]) => entryValue !== undefined));
}
