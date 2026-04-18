import { createClient as createGeneratedClient } from './generated/sdk.js';
import { CrawChatSdkError } from './errors.js';
export const DEFAULT_REALTIME_WEBSOCKET_PATH = 'api/v1/realtime/ws';
export function createGeneratedBackendClient(backendConfig) {
    return createGeneratedClient(backendConfig);
}
export function normalizeCrawChatSdkCreateOptions(options) {
    const apiBaseUrl = firstDefinedString(options.apiBaseUrl, options.baseUrl);
    const authToken = firstDefinedString(options.authToken);
    const transport = {
        apiBaseUrl,
        websocketBaseUrl: firstDefinedString(options.websocketBaseUrl),
    };
    if (options.backendClient) {
        return {
            backendClient: options.backendClient,
            transport,
            authToken,
            webSocketFactory: options.webSocketFactory,
        };
    }
    const tokenManager = options.tokenProvider
        ?? options.tokenManager;
    const hasTransportOverrides = apiBaseUrl != null
        || authToken != null
        || tokenManager != null
        || options.timeout != null
        || options.headers != null;
    if (!hasTransportOverrides) {
        return {
            transport,
            authToken,
            webSocketFactory: options.webSocketFactory,
        };
    }
    if (!apiBaseUrl) {
        throw new CrawChatSdkError('api_base_url_required', 'baseUrl or apiBaseUrl is required when creating a generated Craw Chat SDK client');
    }
    return {
        backendConfig: omitUndefined({
            baseUrl: apiBaseUrl,
            authToken,
            tokenManager,
            timeout: options.timeout,
            headers: options.headers,
        }),
        transport,
        authToken,
        webSocketFactory: options.webSocketFactory,
    };
}
export function resolveBackendClient(options) {
    const normalized = normalizeCrawChatSdkCreateOptions(options);
    if (normalized.backendClient) {
        return normalized.backendClient;
    }
    if (normalized.backendConfig) {
        return createGeneratedBackendClient(normalized.backendConfig);
    }
    throw new CrawChatSdkError('backend_client_missing', 'baseUrl or apiBaseUrl is required');
}
export function resolveCrawChatClientOptions(options) {
    const normalized = normalizeCrawChatSdkCreateOptions(options);
    return {
        backendClient: resolveBackendClient(options),
        transport: normalized.transport,
        authToken: normalized.authToken,
        webSocketFactory: normalized.webSocketFactory,
    };
}
export function resolveCrawChatWebSocketBaseUrl(baseUrl) {
    const parsedUrl = new URL(baseUrl);
    if (parsedUrl.protocol === 'https:') {
        parsedUrl.protocol = 'wss:';
    }
    else if (parsedUrl.protocol === 'http:') {
        parsedUrl.protocol = 'ws:';
    }
    return stripTrailingSlash(parsedUrl.toString());
}
export class CrawChatSdkContext {
    backendClient;
    transport;
    webSocketFactory;
    authToken;
    constructor(backendClient, transport = {}, webSocketFactory, initialAuthToken) {
        this.backendClient = backendClient;
        this.transport = transport;
        this.webSocketFactory = webSocketFactory;
        if (initialAuthToken) {
            this.setAuthToken(initialAuthToken);
        }
    }
    setAuthToken(token) {
        this.authToken = token;
        this.backendClient.setAuthToken?.(token);
    }
    clearAuthToken() {
        this.authToken = undefined;
        if (typeof this.backendClient.clearAuthToken === 'function') {
            this.backendClient.clearAuthToken();
            return;
        }
        this.backendClient.setAuthToken?.('');
    }
    getAuthToken() {
        return this.authToken;
    }
    getApiBaseUrl() {
        return this.transport.apiBaseUrl;
    }
    getWebSocketBaseUrl() {
        if (this.transport.websocketBaseUrl) {
            return this.transport.websocketBaseUrl;
        }
        if (this.transport.apiBaseUrl) {
            return resolveCrawChatWebSocketBaseUrl(this.transport.apiBaseUrl);
        }
        return undefined;
    }
    resolveRealtimeWebSocketUrl(path = DEFAULT_REALTIME_WEBSOCKET_PATH) {
        const websocketBaseUrl = this.getWebSocketBaseUrl();
        if (!websocketBaseUrl) {
            return undefined;
        }
        return `${stripTrailingSlash(websocketBaseUrl)}/${stripLeadingSlash(path)}`;
    }
}
function firstDefinedString(...values) {
    for (const value of values) {
        if (typeof value === 'string' && value.trim().length > 0) {
            return value;
        }
    }
    return undefined;
}
function stripTrailingSlash(value) {
    return value.replace(/\/+$/, '');
}
function stripLeadingSlash(value) {
    return value.replace(/^\/+/, '');
}
function omitUndefined(value) {
    return Object.fromEntries(Object.entries(value).filter(([, entryValue]) => entryValue !== undefined));
}
