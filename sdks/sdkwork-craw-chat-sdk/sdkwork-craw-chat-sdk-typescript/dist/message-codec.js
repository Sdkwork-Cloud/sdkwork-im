import { extractCustomMessageType, resolveStructuredMessageKind, } from './message-standards.js';
export function decodeMessageBody(body) {
    const parts = getBodyParts(body);
    const attachments = parts
        .filter((part) => part.kind === 'media')
        .map((part) => decodeMediaAttachment(part));
    const dataPart = parts.find((part) => part.kind === 'data');
    if (dataPart) {
        const parsedDataPayload = parsePayload(dataPart.payload, dataPart.encoding);
        const structuredKind = resolveStructuredMessageKind(dataPart.schemaRef);
        switch (structuredKind) {
            case 'location': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('location', body, attachments, {
                    latitude: toRequiredNumber(record.latitude),
                    longitude: toRequiredNumber(record.longitude),
                    name: toOptionalString(record.name),
                    address: toOptionalString(record.address),
                    mapUrl: toOptionalString(record.mapUrl),
                });
            }
            case 'link': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('link', body, attachments, {
                    title: toOptionalString(record.title),
                    url: toRequiredString(record.url),
                    description: toOptionalString(record.description),
                    imageUrl: toOptionalString(record.imageUrl),
                    siteName: toOptionalString(record.siteName),
                });
            }
            case 'card': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('card', body, attachments, {
                    title: toRequiredString(record.title),
                    subtitle: toOptionalString(record.subtitle),
                    imageUrl: toOptionalString(record.imageUrl),
                    actions: toCardActions(record.actions),
                });
            }
            case 'music': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('music', body, attachments, {
                    title: toRequiredString(record.title),
                    artist: toOptionalString(record.artist),
                    album: toOptionalString(record.album),
                    url: toRequiredString(record.url),
                    coverUrl: toOptionalString(record.coverUrl),
                    durationSeconds: toOptionalNumber(record.durationSeconds),
                });
            }
            case 'contact': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('contact', body, attachments, {
                    displayName: toRequiredString(record.displayName),
                    avatarUrl: toOptionalString(record.avatarUrl),
                    description: toOptionalString(record.description),
                    profileUrl: toOptionalString(record.profileUrl),
                    contactId: toOptionalString(record.contactId),
                });
            }
            case 'sticker': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('sticker', body, attachments, {
                    stickerId: toOptionalString(record.stickerId),
                    packId: toOptionalString(record.packId),
                    emoji: toOptionalString(record.emoji),
                });
            }
            case 'voice': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('voice', body, attachments, {
                    durationSeconds: toOptionalNumber(record.durationSeconds),
                    transcription: toOptionalString(record.transcription),
                    waveform: toNumberArray(record.waveform),
                });
            }
            case 'agent': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('agent', body, attachments, {
                    agentId: toRequiredString(record.agentId),
                    agentName: toOptionalString(record.agentName),
                    stage: toOptionalString(record.stage),
                    status: toOptionalString(record.status),
                    capabilities: toStringArray(record.capabilities),
                });
            }
            case 'agent_state': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('agent_state', body, attachments, {
                    agentId: toRequiredString(record.agentId),
                    agentName: toOptionalString(record.agentName),
                    stage: toOptionalString(record.stage),
                    status: toOptionalString(record.status),
                    capabilities: toStringArray(record.capabilities),
                });
            }
            case 'agent_handoff': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('agent_handoff', body, attachments, {
                    handoffId: toOptionalString(record.handoffId),
                    fromAgentId: toRequiredString(record.fromAgentId),
                    fromAgentName: toOptionalString(record.fromAgentName),
                    toAgentId: toRequiredString(record.toAgentId),
                    toAgentName: toOptionalString(record.toAgentName),
                    reason: toOptionalString(record.reason),
                    status: toOptionalString(record.status),
                });
            }
            case 'ai_text': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('ai_text', body, attachments, {
                    prompt: toRequiredString(record.prompt),
                    status: toOptionalString(record.status),
                    model: toOptionalString(record.model),
                    revisedPrompt: toOptionalString(record.revisedPrompt),
                });
            }
            case 'ai_image_generation': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('ai_image_generation', body, attachments, {
                    prompt: toRequiredString(record.prompt),
                    status: toOptionalString(record.status),
                    model: toOptionalString(record.model),
                    revisedPrompt: toOptionalString(record.revisedPrompt),
                });
            }
            case 'ai_video_generation': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('ai_video_generation', body, attachments, {
                    prompt: toRequiredString(record.prompt),
                    status: toOptionalString(record.status),
                    model: toOptionalString(record.model),
                    durationSeconds: toOptionalNumber(record.durationSeconds),
                });
            }
            case 'tool_result': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('tool_result', body, attachments, {
                    toolName: toRequiredString(record.toolName),
                    invocationId: toRequiredString(record.invocationId),
                    status: toOptionalString(record.status),
                    output: record.output,
                    error: record.error,
                });
            }
            case 'workflow_event': {
                const record = toRecord(parsedDataPayload);
                return buildDecodedMessage('workflow_event', body, attachments, {
                    workflowId: toRequiredString(record.workflowId),
                    eventName: toRequiredString(record.eventName),
                    stage: toOptionalString(record.stage),
                    status: toOptionalString(record.status),
                    data: record.data,
                });
            }
            case 'custom': {
                return buildDecodedMessage('custom', body, attachments, {
                    customType: extractCustomMessageType(dataPart.schemaRef),
                    schemaRef: dataPart.schemaRef,
                    data: parsedDataPayload,
                });
            }
            default:
                return buildDecodedMessage('data', body, attachments, {
                    schemaRef: dataPart.schemaRef,
                    encoding: dataPart.encoding,
                    payload: parsedDataPayload,
                    rawPayload: dataPart.payload,
                });
        }
    }
    const signalPart = parts.find((part) => part.kind === 'signal');
    if (signalPart) {
        return buildDecodedMessage('signal', body, attachments, {
            signalType: signalPart.signalType,
            schemaRef: signalPart.schemaRef,
            encoding: signalPart.encoding,
            state: signalPart.state,
            payload: parsePayload(signalPart.payload, signalPart.encoding),
            rawPayload: signalPart.payload,
        });
    }
    const streamReferencePart = parts.find((part) => part.kind === 'stream_ref');
    if (streamReferencePart) {
        return buildDecodedMessage('stream_ref', body, attachments, {
            streamId: streamReferencePart.streamId,
            streamType: streamReferencePart.streamType,
            state: streamReferencePart.state,
        });
    }
    if (attachments.length > 0) {
        const attachmentType = attachments[0]?.type;
        if (attachmentType === 'image' ||
            attachmentType === 'video' ||
            attachmentType === 'audio' ||
            attachmentType === 'file') {
            return buildDecodedMessage(attachmentType, body, attachments, {
                mediaType: attachmentType,
            });
        }
    }
    const text = getBodyText(body);
    if (text) {
        return buildDecodedMessage('text', body, attachments, { text });
    }
    return buildDecodedMessage('custom', body, attachments, {
        parts,
    });
}
export function decodeRtcSignalEvent(signal) {
    return {
        rtcSessionId: signal.rtcSessionId,
        signalType: signal.signalType,
        schemaRef: signal.schemaRef,
        payload: parsePayload(signal.payload, signal.schemaRef),
        conversationId: signal.conversationId,
        rtcMode: signal.rtcMode,
        signalingStreamId: signal.signalingStreamId,
        occurredAt: signal.occurredAt,
        rawSignal: signal,
    };
}
export function decodeStreamFrame(frame) {
    return {
        streamId: frame.streamId,
        streamType: frame.streamType,
        frameType: frame.frameType,
        schemaRef: frame.schemaRef,
        encoding: frame.encoding,
        payload: parsePayload(frame.payload, frame.encoding),
        attributes: frame.attributes ?? {},
        occurredAt: frame.occurredAt,
        rawFrame: frame,
    };
}
export function decodeRealtimeEvent(event) {
    const payload = parsePayload(event.payload, 'application/json');
    const payloadRecord = toRecord(payload);
    if (isDecodableMessagePayload(payloadRecord.body)) {
        return {
            kind: 'message',
            rawEvent: event,
            realtimeSeq: event.realtimeSeq,
            eventType: event.eventType,
            scopeType: event.scopeType,
            scopeId: event.scopeId,
            payload,
            messageId: toOptionalString(payloadRecord.messageId),
            conversationId: toOptionalString(payloadRecord.conversationId) ??
                (event.scopeType === 'conversation' ? event.scopeId : undefined),
            message: decodeMessageBody(payloadRecord.body),
        };
    }
    if (event.eventType === 'rtc.signal' ||
        (typeof payloadRecord.rtcSessionId === 'string' &&
            typeof payloadRecord.signalType === 'string')) {
        return {
            kind: 'rtc_signal',
            rawEvent: event,
            realtimeSeq: event.realtimeSeq,
            eventType: event.eventType,
            scopeType: event.scopeType,
            scopeId: event.scopeId,
            payload,
            signal: {
                rtcSessionId: toOptionalString(payloadRecord.rtcSessionId) ??
                    (event.scopeType === 'rtc_session' ? event.scopeId : ''),
                signalType: toRequiredString(payloadRecord.signalType),
                schemaRef: toOptionalString(payloadRecord.schemaRef),
                payload: parseUnknownPayload(payloadRecord.payload, toOptionalString(payloadRecord.encoding) ??
                    toOptionalString(payloadRecord.schemaRef)),
                conversationId: toOptionalString(payloadRecord.conversationId),
                rtcMode: toOptionalString(payloadRecord.rtcMode),
                signalingStreamId: toOptionalString(payloadRecord.signalingStreamId),
                occurredAt: event.occurredAt,
            },
        };
    }
    if (typeof payloadRecord.schemaRef === 'string' &&
        Object.prototype.hasOwnProperty.call(payloadRecord, 'payload')) {
        return {
            kind: 'data',
            rawEvent: event,
            realtimeSeq: event.realtimeSeq,
            eventType: event.eventType,
            scopeType: event.scopeType,
            scopeId: event.scopeId,
            payload,
            data: {
                schemaRef: toOptionalString(payloadRecord.schemaRef),
                encoding: toOptionalString(payloadRecord.encoding),
                payload: parseUnknownPayload(payloadRecord.payload, toOptionalString(payloadRecord.encoding)),
                rawPayload: typeof payloadRecord.payload === 'string'
                    ? payloadRecord.payload
                    : undefined,
            },
        };
    }
    return {
        kind: 'unknown',
        rawEvent: event,
        realtimeSeq: event.realtimeSeq,
        eventType: event.eventType,
        scopeType: event.scopeType,
        scopeId: event.scopeId,
        payload,
    };
}
function buildDecodedMessage(type, body, attachments, content) {
    return {
        type,
        text: getBodyText(body),
        summary: body.summary,
        renderHints: getBodyRenderHints(body),
        content,
        attachments,
        parts: getBodyParts(body),
        rawBody: body,
    };
}
function decodeMediaAttachment(part) {
    return {
        type: part.resource?.type,
        mediaAssetId: part.mediaAssetId,
        resource: part.resource,
        schemaRef: part.schemaRef,
        encoding: part.encoding,
        payload: parsePayload(part.payload, part.encoding),
    };
}
function getBodyParts(body) {
    return body.parts ?? [];
}
function getBodyText(body) {
    return isPostMessageRequest(body) ? body.text : undefined;
}
function getBodyRenderHints(body) {
    return body.renderHints;
}
function isPostMessageRequest(body) {
    return 'text' in body || 'clientMsgId' in body;
}
function isDecodableMessagePayload(value) {
    return Boolean(value &&
        typeof value === 'object' &&
        ('parts' in value || 'text' in value || 'summary' in value));
}
function parsePayload(value, encoding) {
    if (value == null) {
        return undefined;
    }
    return parseUnknownPayload(value, encoding);
}
function parseUnknownPayload(value, encoding) {
    if (typeof value !== 'string') {
        return value;
    }
    const trimmed = value.trim();
    if (!trimmed) {
        return value;
    }
    if (!shouldAttemptJsonParse(trimmed, encoding)) {
        return value;
    }
    try {
        return JSON.parse(trimmed);
    }
    catch {
        return value;
    }
}
function shouldAttemptJsonParse(value, encoding) {
    if (encoding?.toLowerCase().includes('json')) {
        return true;
    }
    return /^(?:\{|\[|"|true$|false$|null$|-?\d)/.test(value);
}
function toRecord(value) {
    if (value && typeof value === 'object' && !Array.isArray(value)) {
        return value;
    }
    return {};
}
function toRequiredString(value) {
    return typeof value === 'string' ? value : '';
}
function toOptionalString(value) {
    return typeof value === 'string' ? value : undefined;
}
function toRequiredNumber(value) {
    return typeof value === 'number' && Number.isFinite(value) ? value : 0;
}
function toOptionalNumber(value) {
    return typeof value === 'number' && Number.isFinite(value) ? value : undefined;
}
function toStringArray(value) {
    if (!Array.isArray(value)) {
        return undefined;
    }
    return value.filter((entry) => typeof entry === 'string');
}
function toNumberArray(value) {
    if (!Array.isArray(value)) {
        return undefined;
    }
    return value.filter((entry) => typeof entry === 'number' && Number.isFinite(entry));
}
function toCardActions(value) {
    if (!Array.isArray(value)) {
        return [];
    }
    return value.flatMap((entry) => {
        const record = toRecord(entry);
        const label = toOptionalString(record.label);
        if (!label) {
            return [];
        }
        return [
            {
                label,
                url: toOptionalString(record.url),
                action: toOptionalString(record.action),
                payload: record.payload,
            },
        ];
    });
}
