export class CrawChatInboxModule {
    context;
    constructor(context) {
        this.context = context;
    }
    list() {
        return this.context.backendClient.inbox.getInbox();
    }
}
