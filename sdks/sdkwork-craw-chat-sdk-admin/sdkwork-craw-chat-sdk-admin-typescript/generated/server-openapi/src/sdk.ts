import { createHttpClient, HttpClient } from './http/client.js';
import type { CrawChatAdminBackendConfig } from './types/common.js';

import { createMetaApi, type MetaApi } from './api/meta.js';
import { createProtocolApi, type ProtocolApi } from './api/protocol.js';
import { createProvidersApi, type ProvidersApi } from './api/providers.js';
import { createSocialApi, type SocialApi } from './api/social.js';
import { createSocialRuntimeApi, type SocialRuntimeApi } from './api/social-runtime.js';
import { createNodesApi, type NodesApi } from './api/nodes.js';

export class CrawChatAdminBackendClient {
  private readonly httpClient: HttpClient;

  readonly meta: MetaApi;
  readonly protocol: ProtocolApi;
  readonly providers: ProvidersApi;
  readonly social: SocialApi;
  readonly socialRuntime: SocialRuntimeApi;
  readonly nodes: NodesApi;

  constructor(config: CrawChatAdminBackendConfig) {
    this.httpClient = createHttpClient(config);
    this.meta = createMetaApi(this.httpClient);
    this.protocol = createProtocolApi(this.httpClient);
    this.providers = createProvidersApi(this.httpClient);
    this.social = createSocialApi(this.httpClient);
    this.socialRuntime = createSocialRuntimeApi(this.httpClient);
    this.nodes = createNodesApi(this.httpClient);
  }

  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }
}

export function createClient(config: CrawChatAdminBackendConfig): CrawChatAdminBackendClient {
  return new CrawChatAdminBackendClient(config);
}
