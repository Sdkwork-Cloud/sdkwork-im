import { CrawChatConversationsModule } from './conversations-module.js';
import { CrawChatDevicesModule } from './device-module.js';
import { CrawChatInboxModule } from './inbox-module.js';
import { CrawChatMediaModule } from './media-module.js';
import { CrawChatMessagesModule } from './messages-module.js';
import { CrawChatPresenceModule } from './presence-module.js';
import { CrawChatRealtimeModule } from './realtime-module.js';
import { CrawChatRtcModule } from './rtc-module.js';
import { CrawChatSessionModule } from './session-module.js';
import { CrawChatStreamsModule } from './streams-module.js';
import type { CrawChatBackendClientLike, CrawChatSdkClientCreateOptions, CrawChatSdkClientOptions } from './types.js';
export declare class CrawChatSdkClient {
    private readonly context;
    readonly backendClient: CrawChatBackendClientLike;
    readonly session: CrawChatSessionModule;
    readonly presence: CrawChatPresenceModule;
    readonly realtime: CrawChatRealtimeModule;
    readonly devices: CrawChatDevicesModule;
    readonly inbox: CrawChatInboxModule;
    readonly conversations: CrawChatConversationsModule;
    readonly messages: CrawChatMessagesModule;
    readonly media: CrawChatMediaModule;
    readonly streams: CrawChatStreamsModule;
    readonly rtc: CrawChatRtcModule;
    constructor(options: CrawChatSdkClientOptions);
    static create(options: CrawChatSdkClientCreateOptions): Promise<CrawChatSdkClient>;
    setAuthToken(token: string): this;
}
export declare function createCrawChatSdkClient(options: CrawChatSdkClientCreateOptions): Promise<CrawChatSdkClient>;
//# sourceMappingURL=sdk.d.ts.map