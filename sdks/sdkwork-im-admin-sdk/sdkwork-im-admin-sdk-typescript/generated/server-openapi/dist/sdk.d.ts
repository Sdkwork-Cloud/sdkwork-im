import { HttpClient } from './http/client';
import type { ImAdminBackendConfig } from './types/common';
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
import { StorageApi } from './api/storage';
export declare class ImAdminBackendClient {
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
    readonly storage: StorageApi;
    constructor(config: ImAdminBackendConfig);
    setAuthToken(token: string): this;
    setTokenManager(manager: AuthTokenManager): this;
    get http(): HttpClient;
}
export declare function createImAdminBackendClient(config: ImAdminBackendConfig): ImAdminBackendClient;
export default ImAdminBackendClient;
//# sourceMappingURL=sdk.d.ts.map