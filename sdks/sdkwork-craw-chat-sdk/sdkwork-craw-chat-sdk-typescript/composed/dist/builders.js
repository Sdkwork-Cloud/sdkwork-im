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
