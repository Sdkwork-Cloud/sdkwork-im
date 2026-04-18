import { CrawChatAdminSdkContext, resolveBackendClient } from './sdk-context.js';
import { CrawChatAdminMetaModule } from './meta-module.js';
import { CrawChatAdminNodesModule } from './nodes-module.js';
import { CrawChatAdminProtocolModule } from './protocol-module.js';
import { CrawChatAdminProvidersModule } from './providers-module.js';
import { CrawChatAdminSocialModule } from './social-module.js';
import { CrawChatAdminSocialRuntimeModule } from './social-runtime-module.js';
import type {
  CrawChatAdminBackendClientLike,
  CrawChatAdminSdkClientCreateOptions,
  CrawChatAdminSdkClientOptions,
} from './types.js';

export class CrawChatAdminSdkClient {
  private readonly context: CrawChatAdminSdkContext;

  readonly backendClient: CrawChatAdminBackendClientLike;
  readonly meta: CrawChatAdminMetaModule;
  readonly protocol: CrawChatAdminProtocolModule;
  readonly providers: CrawChatAdminProvidersModule;
  readonly social: CrawChatAdminSocialModule;
  readonly socialRuntime: CrawChatAdminSocialRuntimeModule;
  readonly nodes: CrawChatAdminNodesModule;

  constructor(options: CrawChatAdminSdkClientOptions) {
    this.context = new CrawChatAdminSdkContext(options.backendClient);
    this.backendClient = options.backendClient;
    this.meta = new CrawChatAdminMetaModule(this.context);
    this.protocol = new CrawChatAdminProtocolModule(this.context);
    this.providers = new CrawChatAdminProvidersModule(this.context);
    this.social = new CrawChatAdminSocialModule(this.context);
    this.socialRuntime = new CrawChatAdminSocialRuntimeModule(this.context);
    this.nodes = new CrawChatAdminNodesModule(this.context);
  }

  static async create(
    options: CrawChatAdminSdkClientCreateOptions,
  ): Promise<CrawChatAdminSdkClient> {
    return new CrawChatAdminSdkClient({
      backendClient: await resolveBackendClient(options),
    });
  }

  setAuthToken(token: string): this {
    this.context.setAuthToken(token);
    return this;
  }
}

export async function createCrawChatAdminSdkClient(
  options: CrawChatAdminSdkClientCreateOptions,
): Promise<CrawChatAdminSdkClient> {
  return CrawChatAdminSdkClient.create(options);
}
