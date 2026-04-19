import type { JsonObject } from './types.js';
import type { ControlPlaneSdkContext } from './sdk-context.js';

export class ControlPlaneProtocolModule {
  constructor(private readonly context: ControlPlaneSdkContext) {}

  getGovernance(): Promise<JsonObject> {
    return this.context.backendClient.protocol.getProtocolGovernance();
  }

  getRegistry(): Promise<JsonObject> {
    return this.context.backendClient.protocol.getProtocolRegistry();
  }
}
