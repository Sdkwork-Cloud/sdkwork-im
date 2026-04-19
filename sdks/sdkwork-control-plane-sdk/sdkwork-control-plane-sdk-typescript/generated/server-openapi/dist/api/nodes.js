function encodeIdentifier(id) {
    return encodeURIComponent(String(id));
}
export function createNodesApi(httpClient) {
    return {
        activateNode(nodeId) {
            return httpClient.post(`/api/v1/control/nodes/${encodeIdentifier(nodeId)}/activate`);
        },
        drainNode(nodeId) {
            return httpClient.post(`/api/v1/control/nodes/${encodeIdentifier(nodeId)}/drain`);
        },
        migrateNodeRoutes(nodeId, body) {
            return httpClient.post(`/api/v1/control/nodes/${encodeIdentifier(nodeId)}/routes/migrate`, body);
        },
    };
}
