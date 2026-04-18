import { AdminApiError, DEFAULT_TIMEOUT } from '../types/common.js';
function normalizeBaseUrl(baseUrl) {
    return baseUrl.replace(/\/+$/, '');
}
function buildQueryString(params) {
    if (!params) {
        return '';
    }
    const entries = Object.entries(params).filter(([, value]) => value !== undefined);
    if (entries.length === 0) {
        return '';
    }
    return entries
        .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(String(value))}`)
        .join('&');
}
function resolveFetchImplementation(config) {
    if (config.fetch) {
        return config.fetch;
    }
    const candidate = globalThis.fetch;
    if (typeof candidate !== 'function') {
        throw new Error('A fetch implementation is required to use the admin backend SDK.');
    }
    return candidate;
}
async function withTimeout(promise, timeoutMs) {
    if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) {
        return promise;
    }
    return await Promise.race([
        promise,
        new Promise((_resolve, reject) => {
            setTimeout(() => reject(new Error(`Admin backend request timed out after ${timeoutMs}ms.`)), timeoutMs);
        }),
    ]);
}
export class HttpClient {
    config;
    constructor(config) {
        this.config = {
            ...config,
            baseUrl: normalizeBaseUrl(config.baseUrl),
            headers: { ...(config.headers ?? {}) },
            timeout: config.timeout ?? DEFAULT_TIMEOUT,
        };
    }
    setAuthToken(token) {
        this.config.authToken = token;
    }
    buildUrl(path, params) {
        const queryString = buildQueryString(params);
        const normalizedPath = path.startsWith('/') ? path : `/${path}`;
        return `${this.config.baseUrl}${normalizedPath}${queryString ? `?${queryString}` : ''}`;
    }
    buildHeaders(headers) {
        return {
            accept: 'application/json',
            ...(this.config.authToken ? { authorization: `Bearer ${this.config.authToken}` } : {}),
            ...(this.config.headers ?? {}),
            ...(headers ?? {}),
        };
    }
    async parsePayload(response) {
        try {
            return await response.json();
        }
        catch {
            const text = await response.text();
            return text.length > 0 ? { message: text } : {};
        }
    }
    async request(path, options = {}) {
        const fetchImpl = resolveFetchImplementation(this.config);
        const response = await withTimeout(fetchImpl(this.buildUrl(path, options.params), {
            method: options.method ?? 'GET',
            headers: this.buildHeaders(options.body ? { 'content-type': 'application/json', ...(options.headers ?? {}) } : options.headers),
            body: options.body ? JSON.stringify(options.body) : undefined,
        }), this.config.timeout ?? DEFAULT_TIMEOUT);
        const payload = await this.parsePayload(response);
        if (!response.ok) {
            const errorMessage = payload &&
                typeof payload === 'object' &&
                'message' in payload &&
                typeof payload.message === 'string'
                ? payload.message
                : undefined;
            throw new AdminApiError(response.status, payload, errorMessage);
        }
        return payload;
    }
    get(path, params) {
        return this.request(path, { method: 'GET', params });
    }
    post(path, body, params) {
        return this.request(path, { method: 'POST', body, params });
    }
}
export function createHttpClient(config) {
    return new HttpClient(config);
}
