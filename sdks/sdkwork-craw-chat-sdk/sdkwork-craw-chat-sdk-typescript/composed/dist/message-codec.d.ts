import type { CrawChatInternalReceiverEvent } from './receiver-internal-types.js';
import type { CrawChatDecodableMessageBody, CrawChatDecodedMessage, CrawChatDecodedRtcSignal, CrawChatDecodedStreamFrame, RealtimeEvent, RtcSignalEvent, StreamFrame } from './types.js';
export declare function decodeMessageBody(body: CrawChatDecodableMessageBody): CrawChatDecodedMessage;
export declare function decodeRtcSignalEvent(signal: RtcSignalEvent): CrawChatDecodedRtcSignal;
export declare function decodeStreamFrame(frame: StreamFrame): CrawChatDecodedStreamFrame;
export declare function decodeRealtimeEvent(event: RealtimeEvent): CrawChatInternalReceiverEvent;
//# sourceMappingURL=message-codec.d.ts.map