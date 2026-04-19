export class ControlPlaneNodesModule {
    context;
    constructor(context) {
        this.context = context;
    }
    activate(nodeId) {
        return this.context.backendClient.nodes.activateNode(nodeId);
    }
    drain(nodeId) {
        return this.context.backendClient.nodes.drainNode(nodeId);
    }
    migrateRoutes(nodeId, body) {
        return this.context.backendClient.nodes.migrateNodeRoutes(nodeId, body);
    }
}
