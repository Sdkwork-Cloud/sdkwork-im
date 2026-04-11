import type { InboxResponse } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatInboxModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  list(): Promise<InboxResponse> {
    return this.context.backendClient.inbox.getInbox();
  }
}
