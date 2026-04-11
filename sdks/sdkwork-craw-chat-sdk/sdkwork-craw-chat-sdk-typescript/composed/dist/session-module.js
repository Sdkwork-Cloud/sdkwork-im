export class CrawChatSessionModule {
    context;
    constructor(context) {
        this.context = context;
    }
    resume(body) {
        return this.context.backendClient.session.resume(body);
    }
    disconnectDevice(body) {
        return this.context.backendClient.session.disconnect(body);
    }
}
