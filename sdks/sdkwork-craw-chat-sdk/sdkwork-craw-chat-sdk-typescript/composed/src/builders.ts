import type {
  AppendStreamFrameRequest,
  AppendTextFrameOptions,
  EditMessageRequest,
  EditTextMessageOptions,
  PostJsonRtcSignalOptions,
  PostMessageRequest,
  PostRtcSignalRequest,
  PostTextMessageOptions,
} from './types.js';

export const DEFAULT_TEXT_FRAME_ENCODING = 'text/plain; charset=utf-8';

export function buildTextMessageRequest(
  text: string,
  options: PostTextMessageOptions = {},
): PostMessageRequest {
  return {
    ...options,
    text,
  };
}

export function buildTextEditRequest(
  text: string,
  options: EditTextMessageOptions = {},
): EditMessageRequest {
  return {
    ...options,
    text,
  };
}

export function buildTextFrameRequest(
  options: AppendTextFrameOptions,
): AppendStreamFrameRequest {
  return {
    frameSeq: options.frameSeq,
    frameType: 'text',
    schemaRef: options.schemaRef,
    encoding: options.encoding ?? DEFAULT_TEXT_FRAME_ENCODING,
    payload: options.text,
    attributes: options.attributes,
  };
}

export function buildJsonRtcSignalRequest(
  signalType: string,
  options: PostJsonRtcSignalOptions,
): PostRtcSignalRequest {
  return {
    signalType,
    schemaRef: options.schemaRef,
    signalingStreamId: options.signalingStreamId,
    payload: JSON.stringify(options.payload ?? null, null, options.pretty ? 2 : 0),
  };
}
