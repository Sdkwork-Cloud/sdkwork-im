export class CrawChatPresenceModule {
    context;
    constructor(context) {
        this.context = context;
    }
    heartbeat(body) {
        return this.context.backendClient.presence.heartbeat(body);
    }
    current() {
        return this.context.backendClient.presence.getPresenceMe();
    }
}
