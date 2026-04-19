import type { Identifier, JsonObject } from './types.js';
import type { ControlPlaneSdkContext } from './sdk-context.js';

export class ControlPlaneNodesModule {
  constructor(private readonly context: ControlPlaneSdkContext) {}

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
