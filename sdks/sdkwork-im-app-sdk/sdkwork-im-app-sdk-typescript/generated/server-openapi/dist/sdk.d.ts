import { HttpClient } from './http/client';
import type { SdkworkAppConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { AutomationApi } from './api/automation';
import { DeviceApi } from './api/device';
import { NotificationApi } from './api/notification';
import { PortalApi } from './api/portal';
import { ProviderApi } from './api/provider';
import { IotApi } from './api/iot';
export declare class SdkworkImAppClient {
    private httpClient;
    readonly automation: AutomationApi;
    readonly device: DeviceApi;
    readonly notification: NotificationApi;
    readonly portal: PortalApi;
    readonly provider: ProviderApi;
    readonly iot: IotApi;
    constructor(config: SdkworkAppConfig);
    setAuthToken(token: string): this;
    setAccessToken(token: string): this;
    setTokenManager(manager: AuthTokenManager): this;
    get http(): HttpClient;
}
export declare function createClient(config: SdkworkAppConfig): SdkworkImAppClient;
export { SdkworkImAppClient as SdkworkAppClient };
export default SdkworkImAppClient;
//# sourceMappingURL=sdk.d.ts.map