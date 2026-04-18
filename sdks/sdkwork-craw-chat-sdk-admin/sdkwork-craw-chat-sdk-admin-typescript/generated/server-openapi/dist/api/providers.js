export function createProvidersApi(httpClient) {
    return {
        getProviderBindings(params) {
            return httpClient.get('/api/v1/control/provider-bindings', params);
        },
        upsertProviderBindingPolicy(body) {
            return httpClient.post('/api/v1/control/provider-bindings', body);
        },
        getProviderPolicyHistory() {
            return httpClient.get('/api/v1/control/provider-policies');
        },
        getProviderPolicyDiff(params) {
            return httpClient.get('/api/v1/control/provider-policies/diff', params);
        },
        previewProviderPolicy(body) {
            return httpClient.post('/api/v1/control/provider-policies/preview', body);
        },
        rollbackProviderPolicy(body) {
            return httpClient.post('/api/v1/control/provider-policies/rollback', body);
        },
        getProviderRegistry() {
            return httpClient.get('/api/v1/control/provider-registry');
        },
    };
}
