import type { JsonObject } from './types.js';
import type { ControlPlaneSdkContext } from './sdk-context.js';

export class ControlPlaneMetaModule {
  constructor(private readonly context: ControlPlaneSdkContext) {}

  health(): Promise<JsonObject> {
    return this.context.backendClient.meta.getHealthz();
  }
}
