import { CrawChatAdminSdkContext, resolveBackendClient } from './sdk-context.js';
import { CrawChatAdminMetaModule } from './meta-module.js';
import { CrawChatAdminNodesModule } from './nodes-module.js';
import { CrawChatAdminProtocolModule } from './protocol-module.js';
import { CrawChatAdminProvidersModule } from './providers-module.js';
import { CrawChatAdminSocialModule } from './social-module.js';
import { CrawChatAdminSocialRuntimeModule } from './social-runtime-module.js';
export class CrawChatAdminSdkClient {
    context;
    backendClient;
    meta;
    protocol;
    providers;
    social;
    socialRuntime;
    nodes;
    constructor(options) {
        this.context = new CrawChatAdminSdkContext(options.backendClient);
        this.backendClient = options.backendClient;
        this.meta = new CrawChatAdminMetaModule(this.context);
        this.protocol = new CrawChatAdminProtocolModule(this.context);
        this.providers = new CrawChatAdminProvidersModule(this.context);
        this.social = new CrawChatAdminSocialModule(this.context);
        this.socialRuntime = new CrawChatAdminSocialRuntimeModule(this.context);
        this.nodes = new CrawChatAdminNodesModule(this.context);
    }
    static async create(options) {
        return new CrawChatAdminSdkClient({
            backendClient: await resolveBackendClient(options),
        });
    }
    setAuthToken(token) {
        this.context.setAuthToken(token);
        return this;
    }
}
export async function createCrawChatAdminSdkClient(options) {
    return CrawChatAdminSdkClient.create(options);
}
