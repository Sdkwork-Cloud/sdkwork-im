import type { ImInternalReceiverEvent } from './receiver-internal-types.js';
import type { ImDecodableMessageBody, ImDecodedMessage, ImDecodedRtcSignal, ImDecodedStreamFrame, RealtimeEvent, RtcSignalEvent, StreamFrame } from './types.js';
export declare function decodeMessageBody(body: ImDecodableMessageBody): ImDecodedMessage;
export declare function decodeRtcSignalEvent(signal: RtcSignalEvent): ImDecodedRtcSignal;
export declare function decodeStreamFrame(frame: StreamFrame): ImDecodedStreamFrame;
export declare function decodeRealtimeEvent(event: RealtimeEvent): ImInternalReceiverEvent;
//# sourceMappingURL=message-codec.d.ts.map