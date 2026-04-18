import type { JsonObject } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';

export class CrawChatAdminMetaModule {
  constructor(private readonly context: CrawChatAdminSdkContext) {}

  health(): Promise<JsonObject> {
    return this.context.backendClient.meta.getHealthz();
  }
}
