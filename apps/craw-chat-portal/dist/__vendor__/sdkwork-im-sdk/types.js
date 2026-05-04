export const ImWebSocketAuthOptions = {
    automatic(init = {}) {
        return buildImWebSocketAuthOptions('automatic', init);
    },
    headerBearer(init = {}) {
        return buildImWebSocketAuthOptions('headerBearer', init);
    },
    queryBearer(init = {}) {
        return buildImWebSocketAuthOptions('queryBearer', init);
    },
    none(init = {}) {
        return buildImWebSocketAuthOptions('none', init);
    },
};
function buildImWebSocketAuthOptions(mode, init) {
    return {
        mode,
        headerName: init.headerName ?? 'Authorization',
        queryParameterName: init.queryParameterName ?? 'access_token',
        scheme: init.scheme ?? 'Bearer',
        credentialProvider: init.credentialProvider,
    };
}
