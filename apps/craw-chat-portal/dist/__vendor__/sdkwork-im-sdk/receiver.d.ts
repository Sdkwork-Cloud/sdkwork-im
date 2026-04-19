import { decodeRealtimeEvent } from './message-codec.js';
import type { ImRealtimeModule } from './realtime-module.js';
import type { ImInternalRealtimeBatch, ImInternalReceiverDataEvent, ImInternalReceiverEvent, ImInternalReceiverMessageEvent, ImInternalReceiverPullAckResult, ImInternalReceiverRtcSignalEvent } from './receiver-internal-types.js';
import type { ImSubscription, QueryParams, RealtimeAckState } from './types.js';
export declare class ImReceiver {
    private readonly realtime;
    private readonly anyHandlers;
    private readonly messageEventHandlers;
    private readonly dataEventHandlers;
    private readonly rtcSignalEventHandlers;
    constructor(realtime: ImRealtimeModule);
    onEvent(handler: (event: ImInternalReceiverEvent) => void): ImSubscription;
    onMessageEvent(handler: (event: ImInternalReceiverMessageEvent) => void): ImSubscription;
    onDataEvent(handler: (event: ImInternalReceiverDataEvent) => void): ImSubscription;
    onRtcSignalEvent(handler: (event: ImInternalReceiverRtcSignalEvent) => void): ImSubscription;
    onScope(scopeType: string, scopeId: string | number, handler: (event: ImInternalReceiverEvent) => void): ImSubscription;
    dispatchRealtimeEvent(event: Parameters<typeof decodeRealtimeEvent>[0]): ImInternalReceiverEvent;
    pull(params?: QueryParams): Promise<ImInternalRealtimeBatch>;
    ack(batchOrSeq: ImInternalRealtimeBatch | number): Promise<RealtimeAckState>;
    pullAndAck(params?: QueryParams): Promise<ImInternalReceiverPullAckResult>;
}
//# sourceMappingURL=receiver.d.ts.map