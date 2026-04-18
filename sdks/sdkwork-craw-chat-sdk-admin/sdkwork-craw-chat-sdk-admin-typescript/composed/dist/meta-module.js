export class CrawChatAdminMetaModule {
    context;
    constructor(context) {
        this.context = context;
    }
    health() {
        return this.context.backendClient.meta.getHealthz();
    }
}
