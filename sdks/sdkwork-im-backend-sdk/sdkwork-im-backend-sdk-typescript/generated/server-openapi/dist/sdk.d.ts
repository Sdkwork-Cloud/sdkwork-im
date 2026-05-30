import { HttpClient } from './http/client';
import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { OpsApi } from './api/ops';
import { AuditApi } from './api/audit';
import { AutomationApi } from './api/automation';
import { ControlApi } from './api/control';
import { AdminApi } from './api/admin';
export declare class SdkworkBackendClient {
    private httpClient;
    readonly ops: OpsApi;
    readonly audit: AuditApi;
    readonly automation: AutomationApi;
    readonly control: ControlApi;
    readonly admin: AdminApi;
    constructor(config: SdkworkBackendConfig);
    setAuthToken(token: string): this;
    setAccessToken(token: string): this;
    setTokenManager(manager: AuthTokenManager): this;
    get http(): HttpClient;
}
export declare function createClient(config: SdkworkBackendConfig): SdkworkBackendClient;
export default SdkworkBackendClient;
//# sourceMappingURL=sdk.d.ts.map