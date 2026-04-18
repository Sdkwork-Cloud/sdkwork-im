import { decodeRealtimeEvent } from './message-codec.js';
import type { CrawChatRealtimeModule } from './realtime-module.js';
import type { CrawChatInternalRealtimeBatch, CrawChatInternalReceiverDataEvent, CrawChatInternalReceiverEvent, CrawChatInternalReceiverMessageEvent, CrawChatInternalReceiverPullAckResult, CrawChatInternalReceiverRtcSignalEvent } from './receiver-internal-types.js';
import type { CrawChatSubscription, QueryParams, RealtimeAckState } from './types.js';
export declare class CrawChatReceiver {
    private readonly realtime;
    private readonly anyHandlers;
    private readonly messageEventHandlers;
    private readonly dataEventHandlers;
    private readonly rtcSignalEventHandlers;
    constructor(realtime: CrawChatRealtimeModule);
    onEvent(handler: (event: CrawChatInternalReceiverEvent) => void): CrawChatSubscription;
    onMessageEvent(handler: (event: CrawChatInternalReceiverMessageEvent) => void): CrawChatSubscription;
    onDataEvent(handler: (event: CrawChatInternalReceiverDataEvent) => void): CrawChatSubscription;
    onRtcSignalEvent(handler: (event: CrawChatInternalReceiverRtcSignalEvent) => void): CrawChatSubscription;
    onScope(scopeType: string, scopeId: string | number, handler: (event: CrawChatInternalReceiverEvent) => void): CrawChatSubscription;
    dispatchRealtimeEvent(event: Parameters<typeof decodeRealtimeEvent>[0]): CrawChatInternalReceiverEvent;
    pull(params?: QueryParams): Promise<CrawChatInternalRealtimeBatch>;
    ack(batchOrSeq: CrawChatInternalRealtimeBatch | number): Promise<RealtimeAckState>;
    pullAndAck(params?: QueryParams): Promise<CrawChatInternalReceiverPullAckResult>;
}
//# sourceMappingURL=receiver.d.ts.map