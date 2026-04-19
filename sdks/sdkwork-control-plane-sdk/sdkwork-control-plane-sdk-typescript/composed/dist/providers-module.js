export class ControlPlaneProvidersModule {
    context;
    constructor(context) {
        this.context = context;
    }
    getBindings(params) {
        return this.context.backendClient.providers.getProviderBindings(params);
    }
    upsertBindingPolicy(body) {
        return this.context.backendClient.providers.upsertProviderBindingPolicy(body);
    }
    getPolicyHistory() {
        return this.context.backendClient.providers.getProviderPolicyHistory();
    }
    getPolicyDiff(params) {
        return this.context.backendClient.providers.getProviderPolicyDiff(params);
    }
    previewPolicy(body) {
        return this.context.backendClient.providers.previewProviderPolicy(body);
    }
    rollbackPolicy(body) {
        return this.context.backendClient.providers.rollbackProviderPolicy(body);
    }
    getRegistry() {
        return this.context.backendClient.providers.getProviderRegistry();
    }
}
