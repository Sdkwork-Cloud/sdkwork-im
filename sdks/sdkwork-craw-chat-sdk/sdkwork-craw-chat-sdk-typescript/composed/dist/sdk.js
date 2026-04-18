import { CrawChatConversationsModule } from './conversations-module.js';
import { CrawChatDevicesModule } from './device-module.js';
import { CrawChatInboxModule } from './inbox-module.js';
import { CrawChatMediaModule } from './media-module.js';
import { CrawChatMessagesModule } from './messages-module.js';
import { CrawChatPresenceModule } from './presence-module.js';
import { CrawChatRealtimeModule } from './realtime-module.js';
import { CrawChatRtcModule } from './rtc-module.js';
import { CrawChatSessionModule } from './session-module.js';
import { CrawChatSdkContext, resolveBackendClient } from './sdk-context.js';
import { CrawChatStreamsModule } from './streams-module.js';
export class CrawChatSdkClient {
    context;
    backendClient;
    session;
    presence;
    realtime;
    devices;
    inbox;
    conversations;
    messages;
    media;
    streams;
    rtc;
    constructor(options) {
        this.context = new CrawChatSdkContext(options.backendClient);
        this.backendClient = options.backendClient;
        this.session = new CrawChatSessionModule(this.context);
        this.presence = new CrawChatPresenceModule(this.context);
        this.realtime = new CrawChatRealtimeModule(this.context);
        this.devices = new CrawChatDevicesModule(this.context);
        this.inbox = new CrawChatInboxModule(this.context);
        this.conversations = new CrawChatConversationsModule(this.context);
        this.messages = new CrawChatMessagesModule(this.context);
        this.media = new CrawChatMediaModule(this.context);
        this.streams = new CrawChatStreamsModule(this.context);
        this.rtc = new CrawChatRtcModule(this.context);
    }
    static async create(options) {
        return new CrawChatSdkClient({
            backendClient: await resolveBackendClient(options),
        });
    }
    setAuthToken(token) {
        this.context.setAuthToken(token);
        return this;
    }
}
export async function createCrawChatSdkClient(options) {
    return CrawChatSdkClient.create(options);
}
