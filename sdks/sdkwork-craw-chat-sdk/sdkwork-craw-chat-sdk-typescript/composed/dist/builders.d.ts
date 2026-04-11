import type { AppendStreamFrameRequest, AppendTextFrameOptions, EditMessageRequest, EditTextMessageOptions, PostJsonRtcSignalOptions, PostMessageRequest, PostRtcSignalRequest, PostTextMessageOptions } from './types.js';
export declare const DEFAULT_TEXT_FRAME_ENCODING = "text/plain; charset=utf-8";
export declare function buildTextMessageRequest(text: string, options?: PostTextMessageOptions): PostMessageRequest;
export declare function buildTextEditRequest(text: string, options?: EditTextMessageOptions): EditMessageRequest;
export declare function buildTextFrameRequest(options: AppendTextFrameOptions): AppendStreamFrameRequest;
export declare function buildJsonRtcSignalRequest(signalType: string, options: PostJsonRtcSignalOptions): PostRtcSignalRequest;
//# sourceMappingURL=builders.d.ts.map