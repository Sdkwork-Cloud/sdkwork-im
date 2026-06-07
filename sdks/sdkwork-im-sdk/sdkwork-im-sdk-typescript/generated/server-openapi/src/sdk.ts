import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkImConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { DeviceApi, createDeviceApi } from './api/device';
import { PresenceApi, createPresenceApi } from './api/presence';
import { RealtimeApi, createRealtimeApi } from './api/realtime';
import { RtcApi, createRtcApi } from './api/rtc';
import { SocialApi, createSocialApi } from './api/social';
import { ChatApi, createChatApi } from './api/chat';
import { StreamsApi, createStreamsApi } from './api/streams';

export class SdkworkImClient {
  private httpClient: HttpClient;

  public readonly device: DeviceApi;
  public readonly presence: PresenceApi;
  public readonly realtime: RealtimeApi;
  public readonly rtc: RtcApi;
  public readonly social: SocialApi;
  public readonly chat: ChatApi;
  public readonly streams: StreamsApi;

  constructor(config: SdkworkImConfig) {
    this.httpClient = createHttpClient(config);
    this.device = createDeviceApi(this.httpClient);

    this.presence = createPresenceApi(this.httpClient);

    this.realtime = createRealtimeApi(this.httpClient);

    this.rtc = createRtcApi(this.httpClient);

    this.social = createSocialApi(this.httpClient);

    this.chat = createChatApi(this.httpClient);

    this.streams = createStreamsApi(this.httpClient);
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

export function createClient(config: SdkworkImConfig): SdkworkImClient {
  return new SdkworkImClient(config);
}


export default SdkworkImClient;
