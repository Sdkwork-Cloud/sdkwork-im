import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { AuthApi, createAuthApi } from './api/auth';
import { PortalApi, createPortalApi } from './api/portal';
import { SessionApi, createSessionApi } from './api/session';
import { PresenceApi, createPresenceApi } from './api/presence';
import { RealtimeApi, createRealtimeApi } from './api/realtime';
import { DeviceApi, createDeviceApi } from './api/device';
import { InboxApi, createInboxApi } from './api/inbox';
import { ConversationApi, createConversationApi } from './api/conversation';
import { MessageApi, createMessageApi } from './api/message';
import { MediaApi, createMediaApi } from './api/media';
import { StreamApi, createStreamApi } from './api/stream';
import { RtcApi, createRtcApi } from './api/rtc';

export class SdkworkBackendClient {
  private readonly httpClient: HttpClient;

  public readonly auth: AuthApi;
  public readonly portal: PortalApi;
  public readonly session: SessionApi;
  public readonly presence: PresenceApi;
  public readonly realtime: RealtimeApi;
  public readonly device: DeviceApi;
  public readonly inbox: InboxApi;
  public readonly conversation: ConversationApi;
  public readonly message: MessageApi;
  public readonly media: MediaApi;
  public readonly stream: StreamApi;
  public readonly rtc: RtcApi;

  constructor(config: SdkworkBackendConfig) {
    this.httpClient = createHttpClient(config);
    this.auth = createAuthApi(this.httpClient);
    this.portal = createPortalApi(this.httpClient);
    this.session = createSessionApi(this.httpClient);
    this.presence = createPresenceApi(this.httpClient);
    this.realtime = createRealtimeApi(this.httpClient);
    this.device = createDeviceApi(this.httpClient);
    this.inbox = createInboxApi(this.httpClient);
    this.conversation = createConversationApi(this.httpClient);
    this.message = createMessageApi(this.httpClient);
    this.media = createMediaApi(this.httpClient);
    this.stream = createStreamApi(this.httpClient);
    this.rtc = createRtcApi(this.httpClient);
  }

  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }
}

export function createClient(config: SdkworkBackendConfig): SdkworkBackendClient {
  return new SdkworkBackendClient(config);
}

export default SdkworkBackendClient;
