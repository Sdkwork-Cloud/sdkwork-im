import { ControlPlaneSdkContext, resolveBackendClient } from './sdk-context.js';
import { ControlPlaneMetaModule } from './meta-module.js';
import { ControlPlaneNodesModule } from './nodes-module.js';
import { ControlPlaneProtocolModule } from './protocol-module.js';
import { ControlPlaneProvidersModule } from './providers-module.js';
import { ControlPlaneSocialModule } from './social-module.js';
import { ControlPlaneSocialRuntimeModule } from './social-runtime-module.js';
export class ControlPlaneSdkClient {
    context;
    backendClient;
    meta;
    protocol;
    providers;
    social;
    socialRuntime;
    nodes;
    constructor(options) {
        this.context = new ControlPlaneSdkContext(options.backendClient);
        this.backendClient = options.backendClient;
        this.meta = new ControlPlaneMetaModule(this.context);
        this.protocol = new ControlPlaneProtocolModule(this.context);
        this.providers = new ControlPlaneProvidersModule(this.context);
        this.social = new ControlPlaneSocialModule(this.context);
        this.socialRuntime = new ControlPlaneSocialRuntimeModule(this.context);
        this.nodes = new ControlPlaneNodesModule(this.context);
    }
    static async create(options) {
        return new ControlPlaneSdkClient({
            backendClient: await resolveBackendClient(options),
        });
    }
    setAuthToken(token) {
        this.context.setAuthToken(token);
        return this;
    }
}
export async function createControlPlaneSdkClient(options) {
    return ControlPlaneSdkClient.create(options);
}
