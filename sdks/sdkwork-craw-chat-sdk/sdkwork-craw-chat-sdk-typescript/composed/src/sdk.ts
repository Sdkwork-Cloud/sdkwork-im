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
import type {
  CrawChatBackendClientLike,
  CrawChatClientCreateOptions,
  CrawChatClientOptions,
} from './types.js';

export class CrawChatClient {
  private readonly context: CrawChatSdkContext;

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

  constructor(options: CrawChatClientOptions) {
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

  static async create(
    options: CrawChatClientCreateOptions,
  ): Promise<CrawChatClient> {
    return new CrawChatClient({
      backendClient: await resolveBackendClient(options),
    });
  }

  setAuthToken(token: string): this {
    this.context.setAuthToken(token);
    return this;
  }
}

export async function createCrawChatClient(
  options: CrawChatClientCreateOptions,
): Promise<CrawChatClient> {
  return CrawChatClient.create(options);
}
