import { ControlPlaneSdkContext, resolveBackendClient } from './sdk-context.js';
import { ControlPlaneMetaModule } from './meta-module.js';
import { ControlPlaneNodesModule } from './nodes-module.js';
import { ControlPlaneProtocolModule } from './protocol-module.js';
import { ControlPlaneProvidersModule } from './providers-module.js';
import { ControlPlaneSocialModule } from './social-module.js';
import { ControlPlaneSocialRuntimeModule } from './social-runtime-module.js';
import type {
  ControlPlaneBackendClientLike,
  ControlPlaneSdkClientCreateOptions,
  ControlPlaneSdkClientOptions,
} from './types.js';

export class ControlPlaneSdkClient {
  private readonly context: ControlPlaneSdkContext;

  readonly backendClient: ControlPlaneBackendClientLike;
  readonly meta: ControlPlaneMetaModule;
  readonly protocol: ControlPlaneProtocolModule;
  readonly providers: ControlPlaneProvidersModule;
  readonly social: ControlPlaneSocialModule;
  readonly socialRuntime: ControlPlaneSocialRuntimeModule;
  readonly nodes: ControlPlaneNodesModule;

  constructor(options: ControlPlaneSdkClientOptions) {
    this.context = new ControlPlaneSdkContext(options.backendClient);
    this.backendClient = options.backendClient;
    this.meta = new ControlPlaneMetaModule(this.context);
    this.protocol = new ControlPlaneProtocolModule(this.context);
    this.providers = new ControlPlaneProvidersModule(this.context);
    this.social = new ControlPlaneSocialModule(this.context);
    this.socialRuntime = new ControlPlaneSocialRuntimeModule(this.context);
    this.nodes = new ControlPlaneNodesModule(this.context);
  }

  static async create(
    options: ControlPlaneSdkClientCreateOptions,
  ): Promise<ControlPlaneSdkClient> {
    return new ControlPlaneSdkClient({
      backendClient: await resolveBackendClient(options),
    });
  }

  setAuthToken(token: string): this {
    this.context.setAuthToken(token);
    return this;
  }
}

export async function createControlPlaneSdkClient(
  options: ControlPlaneSdkClientCreateOptions,
): Promise<ControlPlaneSdkClient> {
  return ControlPlaneSdkClient.create(options);
}
