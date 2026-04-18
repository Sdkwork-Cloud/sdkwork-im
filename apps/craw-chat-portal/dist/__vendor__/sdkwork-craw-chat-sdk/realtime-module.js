export class CrawChatRealtimeModule {
    context;
    constructor(context) {
        this.context = context;
    }
    replaceSubscriptions(body) {
        return this.context.backendClient.realtime.syncRealtimeSubscriptions(body);
    }
    replaceScopeSubscriptions(scopeType, scopeIds, eventTypes, options = {}) {
        const items = normalizeScopeIds(scopeIds).map((scopeId) => ({
            scopeType,
            scopeId: String(scopeId),
            eventTypes,
        }));
        return this.replaceSubscriptions({
            deviceId: options.deviceId,
            items,
        });
    }
    replaceConversationSubscriptions(conversationIds, eventTypes, options = {}) {
        return this.replaceScopeSubscriptions('conversation', conversationIds, eventTypes, options);
    }
    replaceRtcSubscriptions(rtcSessionIds, eventTypes = ['rtc.signal'], options = {}) {
        return this.replaceScopeSubscriptions('rtc_session', rtcSessionIds, eventTypes, options);
    }
    pullEvents(params) {
        return this.context.backendClient.realtime.listRealtimeEvents(params);
    }
    ackEvents(body) {
        return this.context.backendClient.realtime.ackRealtimeEvents(body);
    }
}
function normalizeScopeIds(value) {
    return Array.isArray(value) ? value : [value];
}
