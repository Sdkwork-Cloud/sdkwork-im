import type { JsonObject } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';

export class CrawChatAdminProtocolModule {
  constructor(private readonly context: CrawChatAdminSdkContext) {}

  getGovernance(): Promise<JsonObject> {
    return this.context.backendClient.protocol.getProtocolGovernance();
  }

  getRegistry(): Promise<JsonObject> {
    return this.context.backendClient.protocol.getProtocolRegistry();
  }
}
