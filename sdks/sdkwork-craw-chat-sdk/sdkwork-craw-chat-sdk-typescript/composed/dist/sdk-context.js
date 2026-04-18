function isRecord(value) {
    return typeof value === 'object' && value !== null;
}
function isModuleNotFoundError(error, moduleName) {
    return (isRecord(error) &&
        error.code === 'ERR_MODULE_NOT_FOUND' &&
        typeof error.message === 'string' &&
        error.message.includes(moduleName));
}
async function dynamicImportModule(moduleName) {
    const dynamicImport = new Function('name', 'return import(name);');
    return dynamicImport(moduleName);
}
async function loadGeneratedBackendModule() {
    try {
        return await dynamicImportModule('@sdkwork/craw-chat-backend-sdk');
    }
    catch (error) {
        if (!isModuleNotFoundError(error, '@sdkwork/craw-chat-backend-sdk')) {
            throw error;
        }
    }
    const workspaceFallbackHref = new URL('../../generated/server-openapi/dist/index.js', import.meta.url).href;
    return dynamicImportModule(workspaceFallbackHref);
}
export async function createGeneratedBackendClient(backendConfig) {
    const moduleExport = await loadGeneratedBackendModule();
    const createClient = isRecord(moduleExport) ? moduleExport.createClient : undefined;
    if (typeof createClient !== 'function') {
        throw new Error('Unable to resolve @sdkwork/craw-chat-backend-sdk createClient factory');
    }
    return createClient(backendConfig);
}
function resolveBackendConfig(options) {
    if (options.baseUrl) {
        return {
            baseUrl: options.baseUrl,
            authToken: options.authToken,
            tokenManager: options.tokenManager,
            timeout: options.timeout,
            headers: options.headers,
        };
    }
    return undefined;
}
export async function resolveBackendClient(options) {
    if (options.backendClient) {
        return options.backendClient;
    }
    const backendConfig = resolveBackendConfig(options);
    if (backendConfig) {
        return createGeneratedBackendClient(backendConfig);
    }
    throw new Error('backendClient or baseUrl is required');
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
