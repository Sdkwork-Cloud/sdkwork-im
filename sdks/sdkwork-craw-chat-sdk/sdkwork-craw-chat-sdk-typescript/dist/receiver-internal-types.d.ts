import type { CrawChatDecodedDataPayload, CrawChatDecodedMessage, CrawChatDecodedRtcSignal, CrawChatWebSocketFactory, CrawChatWebSocketLike, RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscriptionSnapshot } from './types.js';
export type CrawChatInternalRealtimeWebSocketMode = 'legacy_json' | 'ccp_json';
export interface CrawChatInternalReceiverEventBase<TKind extends string> {
    kind: TKind;
    rawEvent: RealtimeEvent;
    realtimeSeq: number;
    eventType: string;
    scopeType: string;
    scopeId: string;
    payload: unknown;
}
export interface CrawChatInternalReceiverMessageEvent extends CrawChatInternalReceiverEventBase<'message'> {
    messageId?: string;
    conversationId?: string;
    message: CrawChatDecodedMessage;
}
export interface CrawChatInternalReceiverDataEvent extends CrawChatInternalReceiverEventBase<'data'> {
    data: CrawChatDecodedDataPayload;
}
export interface CrawChatInternalReceiverRtcSignalEvent extends CrawChatInternalReceiverEventBase<'rtc_signal'> {
    signal: CrawChatDecodedRtcSignal;
}
export interface CrawChatInternalReceiverUnknownEvent extends CrawChatInternalReceiverEventBase<'unknown'> {
}
export type CrawChatInternalReceiverEvent = CrawChatInternalReceiverMessageEvent | CrawChatInternalReceiverDataEvent | CrawChatInternalReceiverRtcSignalEvent | CrawChatInternalReceiverUnknownEvent;
export interface CrawChatInternalRealtimeBatch {
    items: CrawChatInternalReceiverEvent[];
    highestSeq: number;
    rawWindow: RealtimeEventWindow;
}
export interface CrawChatInternalReceiverPullAckResult {
    batch: CrawChatInternalRealtimeBatch;
    ack?: RealtimeAckState;
}
export interface CrawChatInternalRealtimeWindowFrame {
    type: 'event.window';
    requestId?: string | null;
    reason: string;
    window: RealtimeEventWindow;
    batch: CrawChatInternalRealtimeBatch;
}
export interface CrawChatInternalRealtimeSubscriptionsSyncedFrame {
    type: 'subscriptions.synced';
    requestId?: string | null;
    snapshot: RealtimeSubscriptionSnapshot;
}
export interface CrawChatInternalRealtimeEventsAckedFrame {
    type: 'events.acked';
    requestId?: string | null;
    ack: RealtimeAckState;
}
export interface CrawChatInternalRealtimeWebSocketCcpOptions {
    principalId: string;
    actorKind: string;
    deviceId?: string;
    sessionId?: string;
    capabilities?: string[];
    traceId?: string;
    lastAckedSeq?: number;
}
export interface CrawChatInternalRealtimeWebSocketReceiverOptions {
    url?: string;
    mode?: CrawChatInternalRealtimeWebSocketMode;
    authToken?: string;
    headers?: Record<string, string>;
    protocols?: string[];
    socket?: CrawChatWebSocketLike;
    createSocket?: CrawChatWebSocketFactory;
    ccp?: CrawChatInternalRealtimeWebSocketCcpOptions;
    requestTimeoutMs?: number;
}
//# sourceMappingURL=receiver-internal-types.d.ts.map