export function createMetaApi(httpClient) {
    return {
        getHealthz() {
            return httpClient.get('/healthz');
        },
    };
}
