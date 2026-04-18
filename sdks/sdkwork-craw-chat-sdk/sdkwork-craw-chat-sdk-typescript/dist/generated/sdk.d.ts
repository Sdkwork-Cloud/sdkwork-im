import type { SdkworkBackendConfig } from './types/common.js';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { AuthApi } from './api/auth.js';
import { PortalApi } from './api/portal.js';
import { SessionApi } from './api/session.js';
import { PresenceApi } from './api/presence.js';
import { RealtimeApi } from './api/realtime.js';
import { DeviceApi } from './api/device.js';
import { InboxApi } from './api/inbox.js';
import { ConversationApi } from './api/conversation.js';
import { MessageApi } from './api/message.js';
import { MediaApi } from './api/media.js';
import { StreamApi } from './api/stream.js';
import { RtcApi } from './api/rtc.js';
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
export { SdkworkBackendClient as CrawChatSdkClient };
//# sourceMappingURL=sdk.d.ts.map