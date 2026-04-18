import { CrawChatSdkAdminContext, resolveBackendClient } from './sdk-context.js';
export class CrawChatSdkAdminClient {
    context;
    backendClient;
    protocol;
    providers;
    cluster;
    social;
    system;
    constructor(options) {
        this.context = new CrawChatSdkAdminContext(options.backendClient);
        this.backendClient = options.backendClient;
        this.protocol = options.backendClient.protocol;
        this.providers = options.backendClient.providers;
        this.cluster = options.backendClient.cluster;
        this.social = options.backendClient.social;
        this.system = options.backendClient.system;
    }
    static async create(options) {
        return new CrawChatSdkAdminClient({
            backendClient: await resolveBackendClient(options),
        });
    }
    setAuthToken(token) {
        this.context.setAuthToken(token);
        return this;
    }
    setAccessToken(token) {
        this.context.setAccessToken(token);
        return this;
    }
    setApiKey(apiKey) {
        this.context.setApiKey(apiKey);
        return this;
    }
}
export async function createCrawChatSdkAdminClient(options) {
    return CrawChatSdkAdminClient.create(options);
}
