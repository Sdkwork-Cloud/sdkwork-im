import { HttpClient } from './http/client';
import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { AuthApi } from './api/auth';
import { UsersApi } from './api/users';
import { MarketingApi } from './api/marketing';
import { TenantsApi } from './api/tenants';
import { AccessApi } from './api/access';
import { RoutingApi } from './api/routing';
import { CatalogApi } from './api/catalog';
import { UsageApi } from './api/usage';
import { BillingApi } from './api/billing';
import { OperationsApi } from './api/operations';
export declare class SdkworkBackendClient {
    private httpClient;
    readonly auth: AuthApi;
    readonly users: UsersApi;
    readonly marketing: MarketingApi;
    readonly tenants: TenantsApi;
    readonly access: AccessApi;
    readonly routing: RoutingApi;
    readonly catalog: CatalogApi;
    readonly usage: UsageApi;
    readonly billing: BillingApi;
    readonly operations: OperationsApi;
    constructor(config: SdkworkBackendConfig);
    setApiKey(apiKey: string): this;
    setAuthToken(token: string): this;
    setAccessToken(token: string): this;
    setTokenManager(manager: AuthTokenManager): this;
    get http(): HttpClient;
}
export declare function createClient(config: SdkworkBackendConfig): SdkworkBackendClient;
export default SdkworkBackendClient;
//# sourceMappingURL=sdk.d.ts.map