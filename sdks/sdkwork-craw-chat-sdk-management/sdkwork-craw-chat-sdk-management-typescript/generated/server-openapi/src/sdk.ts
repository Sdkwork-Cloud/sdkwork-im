import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { AuthApi, createAuthApi } from './api/auth';
import { UsersApi, createUsersApi } from './api/users';
import { MarketingApi, createMarketingApi } from './api/marketing';
import { TenantsApi, createTenantsApi } from './api/tenants';
import { AccessApi, createAccessApi } from './api/access';
import { RoutingApi, createRoutingApi } from './api/routing';
import { CatalogApi, createCatalogApi } from './api/catalog';
import { UsageApi, createUsageApi } from './api/usage';
import { BillingApi, createBillingApi } from './api/billing';
import { OperationsApi, createOperationsApi } from './api/operations';

export class SdkworkBackendClient {
  private httpClient: HttpClient;

  public readonly auth: AuthApi;
  public readonly users: UsersApi;
  public readonly marketing: MarketingApi;
  public readonly tenants: TenantsApi;
  public readonly access: AccessApi;
  public readonly routing: RoutingApi;
  public readonly catalog: CatalogApi;
  public readonly usage: UsageApi;
  public readonly billing: BillingApi;
  public readonly operations: OperationsApi;

  constructor(config: SdkworkBackendConfig) {
    this.httpClient = createHttpClient(config);
    this.auth = createAuthApi(this.httpClient);
    this.users = createUsersApi(this.httpClient);
    this.marketing = createMarketingApi(this.httpClient);
    this.tenants = createTenantsApi(this.httpClient);
    this.access = createAccessApi(this.httpClient);
    this.routing = createRoutingApi(this.httpClient);
    this.catalog = createCatalogApi(this.httpClient);
    this.usage = createUsageApi(this.httpClient);
    this.billing = createBillingApi(this.httpClient);
    this.operations = createOperationsApi(this.httpClient);
  }

  setApiKey(apiKey: string): this {
    this.httpClient.setApiKey(apiKey);
    return this;
  }

  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.httpClient.setAccessToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }

  get http(): HttpClient {
    return this.httpClient;
  }
}

export function createClient(config: SdkworkBackendConfig): SdkworkBackendClient {
  return new SdkworkBackendClient(config);
}

export default SdkworkBackendClient;
