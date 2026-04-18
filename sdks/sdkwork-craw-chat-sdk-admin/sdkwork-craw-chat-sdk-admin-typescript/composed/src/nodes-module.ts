import type { Identifier, JsonObject } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';

export class CrawChatAdminNodesModule {
  constructor(private readonly context: CrawChatAdminSdkContext) {}

  activate(nodeId: Identifier): Promise<JsonObject> {
    return this.context.backendClient.nodes.activateNode(nodeId);
  }

  drain(nodeId: Identifier): Promise<JsonObject> {
    return this.context.backendClient.nodes.drainNode(nodeId);
  }

  migrateRoutes(nodeId: Identifier, body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.nodes.migrateNodeRoutes(nodeId, body);
  }
}
