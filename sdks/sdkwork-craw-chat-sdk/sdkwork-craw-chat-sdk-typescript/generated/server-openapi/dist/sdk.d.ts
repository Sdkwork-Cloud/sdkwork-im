import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { AuthApi } from './api/auth';
import { PortalApi } from './api/portal';
import { SessionApi } from './api/session';
import { PresenceApi } from './api/presence';
import { RealtimeApi } from './api/realtime';
import { DeviceApi } from './api/device';
import { InboxApi } from './api/inbox';
import { ConversationApi } from './api/conversation';
import { MessageApi } from './api/message';
import { MediaApi } from './api/media';
import { StreamApi } from './api/stream';
import { RtcApi } from './api/rtc';
export declare class SdkworkBackendClient {
    private readonly httpClient;
    readonly auth: AuthApi;
    readonly portal: PortalApi;
    readonly session: SessionApi;
    readonly presence: PresenceApi;
    readonly realtime: RealtimeApi;
    readonly device: DeviceApi;
    readonly inbox: InboxApi;
    readonly conversation: ConversationApi;
    readonly message: MessageApi;
    readonly media: MediaApi;
    readonly stream: StreamApi;
    readonly rtc: RtcApi;
    constructor(config: SdkworkBackendConfig);
    setAuthToken(token: string): this;
    setTokenManager(manager: AuthTokenManager): this;
}
export declare function createClient(config: SdkworkBackendConfig): SdkworkBackendClient;
export default SdkworkBackendClient;
//# sourceMappingURL=sdk.d.ts.map