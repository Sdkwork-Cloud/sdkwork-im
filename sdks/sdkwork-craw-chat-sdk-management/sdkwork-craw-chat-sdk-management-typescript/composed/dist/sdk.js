import { CrawChatSdkManagementContext, resolveBackendClient } from './sdk-context.js';
export class CrawChatSdkManagementClient {
    context;
    backendClient;
    auth;
    users;
    marketing;
    tenants;
    access;
    routing;
    catalog;
    usage;
    billing;
    operations;
    constructor(options) {
        this.context = new CrawChatSdkManagementContext(options.backendClient);
        this.backendClient = options.backendClient;
        this.auth = options.backendClient.auth;
        this.users = options.backendClient.users;
        this.marketing = options.backendClient.marketing;
        this.tenants = options.backendClient.tenants;
        this.access = options.backendClient.access;
        this.routing = options.backendClient.routing;
        this.catalog = options.backendClient.catalog;
        this.usage = options.backendClient.usage;
        this.billing = options.backendClient.billing;
        this.operations = options.backendClient.operations;
    }
    static async create(options) {
        return new CrawChatSdkManagementClient({
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
export async function createCrawChatSdkManagementClient(options) {
    return CrawChatSdkManagementClient.create(options);
}
