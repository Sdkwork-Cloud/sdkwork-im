import { createHttpClient, HttpClient } from './http/client.js';
import { createMetaApi } from './api/meta.js';
import { createProtocolApi } from './api/protocol.js';
import { createProvidersApi } from './api/providers.js';
import { createSocialApi } from './api/social.js';
import { createSocialRuntimeApi } from './api/social-runtime.js';
import { createNodesApi } from './api/nodes.js';
export class ControlPlaneBackendClient {
    httpClient;
    meta;
    protocol;
    providers;
    social;
    socialRuntime;
    nodes;
    constructor(config) {
        this.httpClient = createHttpClient(config);
        this.meta = createMetaApi(this.httpClient);
        this.protocol = createProtocolApi(this.httpClient);
        this.providers = createProvidersApi(this.httpClient);
        this.social = createSocialApi(this.httpClient);
        this.socialRuntime = createSocialRuntimeApi(this.httpClient);
        this.nodes = createNodesApi(this.httpClient);
    }
    setAuthToken(token) {
        this.httpClient.setAuthToken(token);
        return this;
    }
}
export function createClient(config) {
    return new ControlPlaneBackendClient(config);
}
