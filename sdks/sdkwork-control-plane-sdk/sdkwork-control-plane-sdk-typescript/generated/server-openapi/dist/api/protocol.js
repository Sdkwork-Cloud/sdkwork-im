export function createProtocolApi(httpClient) {
    return {
        getProtocolGovernance() {
            return httpClient.get('/api/v1/control/protocol-governance');
        },
        getProtocolRegistry() {
            return httpClient.get('/api/v1/control/protocol-registry');
        },
    };
}
