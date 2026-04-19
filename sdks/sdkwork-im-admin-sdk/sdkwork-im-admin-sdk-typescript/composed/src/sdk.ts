import { ImAdminSdkContext, resolveBackendClient } from './sdk-context.js';
import type {
  ImAdminBackendClientLike,
  ImAdminSdkClientCreateOptions,
  ImAdminSdkClientOptions,
} from './types.js';

export class ImAdminSdkClient {
  private readonly context: ImAdminSdkContext;

  readonly backendClient: ImAdminBackendClientLike;
  readonly auth: ImAdminBackendClientLike['auth'];
  readonly users: ImAdminBackendClientLike['users'];
  readonly marketing: ImAdminBackendClientLike['marketing'];
  readonly tenants: ImAdminBackendClientLike['tenants'];
  readonly access: ImAdminBackendClientLike['access'];
  readonly routing: ImAdminBackendClientLike['routing'];
  readonly catalog: ImAdminBackendClientLike['catalog'];
  readonly usage: ImAdminBackendClientLike['usage'];
  readonly billing: ImAdminBackendClientLike['billing'];
  readonly operations: ImAdminBackendClientLike['operations'];
  readonly storage: ImAdminBackendClientLike['storage'];

  constructor(options: ImAdminSdkClientOptions) {
    this.context = new ImAdminSdkContext(options.backendClient);
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
    this.storage = options.backendClient.storage;
  }

  static async create(
    options: ImAdminSdkClientCreateOptions,
  ): Promise<ImAdminSdkClient> {
    return new ImAdminSdkClient({
      backendClient: await resolveBackendClient(options),
    });
  }

  setAuthToken(token: string): this {
    this.context.setAuthToken(token);
    return this;
  }
}

export async function createImAdminSdkClient(
  options: ImAdminSdkClientCreateOptions,
): Promise<ImAdminSdkClient> {
  return ImAdminSdkClient.create(options);
}
