import type { ImDecodedDataPayload, ImDecodedMessage, ImDecodedRtcSignal, ImWebSocketAuthOptions, ImWebSocketFactory, ImWebSocketLike, RealtimeAckState, RealtimeEvent, RealtimeEventWindow, RealtimeSubscriptionSnapshot } from './types.js';
export type ImInternalRealtimeWebSocketMode = 'legacy_json' | 'ccp_json';
export interface ImInternalReceiverEventBase<TKind extends string> {
    kind: TKind;
    rawEvent: RealtimeEvent;
    realtimeSeq: number;
    eventType: string;
    scopeType: string;
    scopeId: string;
    payload: unknown;
}
export interface ImInternalReceiverMessageEvent extends ImInternalReceiverEventBase<'message'> {
    messageId?: string;
    conversationId?: string;
    message: ImDecodedMessage;
}
export interface ImInternalReceiverDataEvent extends ImInternalReceiverEventBase<'data'> {
    data: ImDecodedDataPayload;
}
export interface ImInternalReceiverRtcSignalEvent extends ImInternalReceiverEventBase<'rtc_signal'> {
    signal: ImDecodedRtcSignal;
}
export interface ImInternalReceiverUnknownEvent extends ImInternalReceiverEventBase<'unknown'> {
}
export type ImInternalReceiverEvent = ImInternalReceiverMessageEvent | ImInternalReceiverDataEvent | ImInternalReceiverRtcSignalEvent | ImInternalReceiverUnknownEvent;
export interface ImInternalRealtimeBatch {
    items: ImInternalReceiverEvent[];
    highestSeq: number;
    rawWindow: RealtimeEventWindow;
}
export interface ImInternalReceiverCatchUpAckResult {
    batch: ImInternalRealtimeBatch;
    ack?: RealtimeAckState;
}
export interface ImInternalRealtimeWindowFrame {
    type: 'event.window';
    requestId?: string | null;
    reason: string;
    window: RealtimeEventWindow;
    batch: ImInternalRealtimeBatch;
}
export interface ImInternalRealtimeSubscriptionsSyncedFrame {
    type: 'subscriptions.synced';
    requestId?: string | null;
    snapshot: RealtimeSubscriptionSnapshot;
}
export interface ImInternalRealtimeEventsAckedFrame {
    type: 'events.acked';
    requestId?: string | null;
    ack: RealtimeAckState;
}
export interface ImInternalRealtimeWebSocketCcpOptions {
    principalId: string;
    actorKind: string;
    deviceId?: string;
    sessionId?: string;
    capabilities?: string[];
    traceId?: string;
    lastAckedSeq?: number;
}
export interface ImInternalRealtimeWebSocketReceiverOptions {
    deviceId?: string;
    url?: string;
    mode?: ImInternalRealtimeWebSocketMode;
    authToken?: string;
    webSocketAuth?: ImWebSocketAuthOptions;
    headers?: Record<string, string>;
    protocols?: string[];
    socket?: ImWebSocketLike;
    createSocket?: ImWebSocketFactory;
    ccp?: ImInternalRealtimeWebSocketCcpOptions;
    requestTimeoutMs?: number;
}
//# sourceMappingURL=receiver-internal-types.d.ts.map