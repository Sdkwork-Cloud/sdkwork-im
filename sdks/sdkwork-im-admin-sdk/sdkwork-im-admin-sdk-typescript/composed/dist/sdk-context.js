function isRecord(value) {
    return typeof value === 'object' && value !== null;
}
async function dynamicImportModule(moduleName) {
    const dynamicImport = new Function('name', 'return import(name);');
    return dynamicImport(moduleName);
}
export async function createGeneratedBackendClient(backendConfig) {
    const moduleExport = await dynamicImportModule('@sdkwork/im-admin-backend-sdk');
    const createClient = isRecord(moduleExport) ? moduleExport.createImAdminBackendClient : undefined;
    if (typeof createClient !== 'function') {
        throw new Error('Unable to resolve @sdkwork/im-admin-backend-sdk createImAdminBackendClient factory');
    }
    return createClient(backendConfig);
}
export async function resolveBackendClient(options) {
    if (options.backendClient) {
        return options.backendClient;
    }
    if (options.backendConfig) {
        return createGeneratedBackendClient(options.backendConfig);
    }
    throw new Error('backendClient or backendConfig is required');
}
export class ImAdminSdkContext {
    backendClient;
    constructor(backendClient) {
        this.backendClient = backendClient;
    }
    setAuthToken(token) {
        this.backendClient.setAuthToken?.(token);
    }
}
