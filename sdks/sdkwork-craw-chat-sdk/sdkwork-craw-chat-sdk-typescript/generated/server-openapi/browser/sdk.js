import { createHttpClient } from './http/client.js';
import { createAuthApi } from './api/auth.js';
import { createPortalApi } from './api/portal.js';
import { createSessionApi } from './api/session.js';
import { createPresenceApi } from './api/presence.js';
import { createRealtimeApi } from './api/realtime.js';
import { createDeviceApi } from './api/device.js';
import { createInboxApi } from './api/inbox.js';
import { createConversationApi } from './api/conversation.js';
import { createMessageApi } from './api/message.js';
import { createMediaApi } from './api/media.js';
import { createStreamApi } from './api/stream.js';
import { createRtcApi } from './api/rtc.js';
export class SdkworkBackendClient {
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
