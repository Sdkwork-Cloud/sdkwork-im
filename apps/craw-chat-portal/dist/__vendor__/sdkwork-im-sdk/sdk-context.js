import { createTransportClient as createRawTransportClient } from './generated/index.js';
import { ImSdkError } from './errors.js';
export const DEFAULT_REALTIME_WEBSOCKET_PATH = 'api/v1/realtime/ws';
export function createTransportClient(transportConfig) {
    return createRawTransportClient(transportConfig);
}
export function normalizeImSdkCreateOptions(options) {
    const apiBaseUrl = firstDefinedString(options.apiBaseUrl, options.baseUrl);
    const authToken = firstDefinedString(options.authToken);
    const transport = {
        apiBaseUrl,
        websocketBaseUrl: firstDefinedString(options.websocketBaseUrl),
    };
    if (options.transportClient) {
        return {
            transportClient: options.transportClient,
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
        throw new ImSdkError('api_base_url_required', 'baseUrl or apiBaseUrl is required when creating ImSdkClient');
    }
    return {
        transportConfig: omitUndefined({
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
export function resolveTransportClient(options) {
    const normalized = normalizeImSdkCreateOptions(options);
    if (normalized.transportClient) {
        return normalized.transportClient;
    }
    if (normalized.transportConfig) {
        return createTransportClient(normalized.transportConfig);
    }
    throw new ImSdkError('api_base_url_required', 'baseUrl or apiBaseUrl is required');
}
export function resolveImClientOptions(options) {
    const normalized = normalizeImSdkCreateOptions(options);
    return {
        transportClient: resolveTransportClient(options),
        transport: normalized.transport,
        authToken: normalized.authToken,
        webSocketFactory: normalized.webSocketFactory,
    };
}
export function resolveImWebSocketBaseUrl(baseUrl) {
    const parsedUrl = new URL(baseUrl);
    if (parsedUrl.protocol === 'https:') {
        parsedUrl.protocol = 'wss:';
    }
    else if (parsedUrl.protocol === 'http:') {
        parsedUrl.protocol = 'ws:';
    }
    return stripTrailingSlash(parsedUrl.toString());
}
export class ImSdkContext {
    transportClient;
    transport;
    webSocketFactory;
    authToken;
    constructor(transportClient, transport = {}, webSocketFactory, initialAuthToken) {
        this.transportClient = transportClient;
        this.transport = transport;
        this.webSocketFactory = webSocketFactory;
        if (initialAuthToken) {
            this.setAuthToken(initialAuthToken);
        }
    }
    setAuthToken(token) {
        this.authToken = token;
        this.transportClient.setAuthToken?.(token);
    }
    clearAuthToken() {
        this.authToken = undefined;
        if (typeof this.transportClient.clearAuthToken === 'function') {
            this.transportClient.clearAuthToken();
            return;
        }
        this.transportClient.setAuthToken?.('');
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
            return resolveImWebSocketBaseUrl(this.transport.apiBaseUrl);
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
