import { CrawChatSdkManagementContext, resolveBackendClient } from './sdk-context.js';
import type {
  CrawChatManagementBackendClientLike,
  CrawChatSdkManagementClientCreateOptions,
  CrawChatSdkManagementClientOptions,
} from './types.js';

export class CrawChatSdkManagementClient {
  private readonly context: CrawChatSdkManagementContext;

  readonly backendClient: CrawChatManagementBackendClientLike;
  readonly auth: CrawChatManagementBackendClientLike['auth'];
  readonly users: CrawChatManagementBackendClientLike['users'];
  readonly marketing: CrawChatManagementBackendClientLike['marketing'];
  readonly tenants: CrawChatManagementBackendClientLike['tenants'];
  readonly access: CrawChatManagementBackendClientLike['access'];
  readonly routing: CrawChatManagementBackendClientLike['routing'];
  readonly catalog: CrawChatManagementBackendClientLike['catalog'];
  readonly usage: CrawChatManagementBackendClientLike['usage'];
  readonly billing: CrawChatManagementBackendClientLike['billing'];
  readonly operations: CrawChatManagementBackendClientLike['operations'];

  constructor(options: CrawChatSdkManagementClientOptions) {
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

  static async create(
    options: CrawChatSdkManagementClientCreateOptions,
  ): Promise<CrawChatSdkManagementClient> {
    return new CrawChatSdkManagementClient({
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

export async function createCrawChatSdkManagementClient(
  options: CrawChatSdkManagementClientCreateOptions,
): Promise<CrawChatSdkManagementClient> {
  return CrawChatSdkManagementClient.create(options);
}
