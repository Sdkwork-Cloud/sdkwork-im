import { HttpClient, createHttpClient } from './http/client.js';
import { AuthApi, createAuthApi } from './api/auth.js';
import { PortalApi, createPortalApi } from './api/portal.js';
import { SessionApi, createSessionApi } from './api/session.js';
import { PresenceApi, createPresenceApi } from './api/presence.js';
import { RealtimeApi, createRealtimeApi } from './api/realtime.js';
import { DeviceApi, createDeviceApi } from './api/device.js';
import { InboxApi, createInboxApi } from './api/inbox.js';
import { ConversationApi, createConversationApi } from './api/conversation.js';
import { MessageApi, createMessageApi } from './api/message.js';
import { MediaApi, createMediaApi } from './api/media.js';
import { StreamApi, createStreamApi } from './api/stream.js';
import { RtcApi, createRtcApi } from './api/rtc.js';
export class SdkworkBackendClient {
    httpClient;
    auth;
    portal;
    session;
    presence;
    realtime;
    device;
    inbox;
    conversation;
    message;
    media;
    stream;
    rtc;
    constructor(config) {
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
    setAuthToken(token) {
        this.httpClient.setAuthToken(token);
        return this;
    }
    setTokenManager(manager) {
        this.httpClient.setTokenManager(manager);
        return this;
    }
}
export function createClient(config) {
    return new SdkworkBackendClient(config);
}
export default SdkworkBackendClient;
export { SdkworkBackendClient as CrawChatSdkClient };
