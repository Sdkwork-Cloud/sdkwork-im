export function toReceiveContext(event, source, ack) {
    const base = {
        sequence: event.realtimeSeq,
        source,
        receivedAt: event.rawEvent.occurredAt,
        sender: {
            principalId: event.rawEvent.principalId,
            deviceId: event.rawEvent.deviceId,
        },
        eventType: event.eventType,
        scopeType: event.scopeType,
        scopeId: event.scopeId,
        payload: event.payload,
        rawEvent: event.rawEvent,
        ack,
    };
    switch (event.kind) {
        case 'message':
            return {
                kind: 'message',
                ...base,
                messageId: event.messageId,
                conversationId: event.conversationId,
                message: event.message,
            };
        case 'data':
            return {
                kind: 'data',
                ...base,
                data: event.data,
            };
        case 'rtc_signal':
            return {
                kind: 'signal',
                ...base,
                signal: event.signal,
            };
        default:
            return {
                kind: 'unknown',
                ...base,
            };
    }
}
