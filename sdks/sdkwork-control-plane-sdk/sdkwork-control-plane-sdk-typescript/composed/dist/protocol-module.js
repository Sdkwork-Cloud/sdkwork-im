export class ControlPlaneProtocolModule {
    context;
    constructor(context) {
        this.context = context;
    }
    getGovernance() {
        return this.context.backendClient.protocol.getProtocolGovernance();
    }
    getRegistry() {
        return this.context.backendClient.protocol.getProtocolRegistry();
    }
}
