export class CrawChatRealtimeModule {
    context;
    constructor(context) {
        this.context = context;
    }
    replaceSubscriptions(body) {
        return this.context.backendClient.realtime.syncRealtimeSubscriptions(body);
    }
    pullEvents(params) {
        return this.context.backendClient.realtime.listRealtimeEvents(params);
    }
    ackEvents(body) {
        return this.context.backendClient.realtime.ackRealtimeEvents(body);
    }
}
