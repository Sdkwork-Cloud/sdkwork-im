import { HttpClient } from './http/client';
import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { ClusterApi } from './api/cluster';
import { ProtocolApi } from './api/protocol';
import { ProvidersApi } from './api/providers';
import { SocialApi } from './api/social';
import { SystemApi } from './api/system';
export declare class SdkworkBackendClient {
    private httpClient;
    readonly cluster: ClusterApi;
    readonly protocol: ProtocolApi;
    readonly providers: ProvidersApi;
    readonly social: SocialApi;
    readonly system: SystemApi;
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