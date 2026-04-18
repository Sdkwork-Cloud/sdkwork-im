import { CrawChatSdkAdminContext, resolveBackendClient } from './sdk-context.js';
import type {
  CrawChatAdminBackendClientLike,
  CrawChatSdkAdminClientCreateOptions,
  CrawChatSdkAdminClientOptions,
} from './types.js';

export class CrawChatSdkAdminClient {
  private readonly context: CrawChatSdkAdminContext;

  readonly backendClient: CrawChatAdminBackendClientLike;
  readonly protocol: CrawChatAdminBackendClientLike['protocol'];
  readonly providers: CrawChatAdminBackendClientLike['providers'];
  readonly cluster: CrawChatAdminBackendClientLike['cluster'];
  readonly social: CrawChatAdminBackendClientLike['social'];
  readonly system: CrawChatAdminBackendClientLike['system'];

  constructor(options: CrawChatSdkAdminClientOptions) {
    this.context = new CrawChatSdkAdminContext(options.backendClient);
    this.backendClient = options.backendClient;
    this.protocol = options.backendClient.protocol;
    this.providers = options.backendClient.providers;
    this.cluster = options.backendClient.cluster;
    this.social = options.backendClient.social;
    this.system = options.backendClient.system;
  }

  static async create(
    options: CrawChatSdkAdminClientCreateOptions,
  ): Promise<CrawChatSdkAdminClient> {
    return new CrawChatSdkAdminClient({
      backendClient: await resolveBackendClient(options),
    });
  }

  setAuthToken(token: string): this {
    this.context.setAuthToken(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.context.setAccessToken(token);
    return this;
  }

  setApiKey(apiKey: string): this {
    this.context.setApiKey(apiKey);
    return this;
  }
}

export async function createCrawChatSdkAdminClient(
  options: CrawChatSdkAdminClientCreateOptions,
): Promise<CrawChatSdkAdminClient> {
  return CrawChatSdkAdminClient.create(options);
}
