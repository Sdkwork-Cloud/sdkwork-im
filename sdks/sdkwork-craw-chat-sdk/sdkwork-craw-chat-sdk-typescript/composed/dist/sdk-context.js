function isRecord(value) {
    return typeof value === 'object' && value !== null;
}
async function dynamicImportModule(moduleName) {
    const dynamicImport = new Function('name', 'return import(name);');
    return dynamicImport(moduleName);
}
export async function createGeneratedBackendClient(backendConfig) {
    const moduleExport = await dynamicImportModule('@sdkwork/craw-chat-backend-sdk');
    const createClient = isRecord(moduleExport) ? moduleExport.createClient : undefined;
    if (typeof createClient !== 'function') {
        throw new Error('Unable to resolve @sdkwork/craw-chat-backend-sdk createClient factory');
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
export class CrawChatSdkContext {
    backendClient;
    constructor(backendClient) {
        this.backendClient = backendClient;
    }
    setAuthToken(token) {
        this.backendClient.setAuthToken?.(token);
    }
}
