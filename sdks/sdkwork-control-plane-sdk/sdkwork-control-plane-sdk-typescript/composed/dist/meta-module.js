export class ControlPlaneMetaModule {
    context;
    constructor(context) {
        this.context = context;
    }
    health() {
        return this.context.backendClient.meta.getHealthz();
    }
}
