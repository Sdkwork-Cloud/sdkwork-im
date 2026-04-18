import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { ClusterApi, createClusterApi } from './api/cluster';
import { ProtocolApi, createProtocolApi } from './api/protocol';
import { ProvidersApi, createProvidersApi } from './api/providers';
import { SocialApi, createSocialApi } from './api/social';
import { SystemApi, createSystemApi } from './api/system';

export class SdkworkBackendClient {
  private httpClient: HttpClient;

  public readonly cluster: ClusterApi;
  public readonly protocol: ProtocolApi;
  public readonly providers: ProvidersApi;
  public readonly social: SocialApi;
  public readonly system: SystemApi;

  constructor(config: SdkworkBackendConfig) {
    this.httpClient = createHttpClient(config);
    this.cluster = createClusterApi(this.httpClient);

    this.protocol = createProtocolApi(this.httpClient);

    this.providers = createProvidersApi(this.httpClient);

    this.social = createSocialApi(this.httpClient);

    this.system = createSystemApi(this.httpClient);
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
