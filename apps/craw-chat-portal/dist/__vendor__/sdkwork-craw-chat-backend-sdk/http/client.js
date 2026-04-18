import { BaseHttpClient, withRetry } from '@sdkwork/sdk-common';
export class HttpClient extends BaseHttpClient {
    constructor(config) {
        super(config);
    }
    getInternalAuthConfig() {
        const self = this;
        self.authConfig = self.authConfig || {};
        return self.authConfig;
    }
    buildRequestHeaders(headers, contentType) {
        const mergedHeaders = {
            ...(headers ?? {}),
        };
        if (contentType && contentType.toLowerCase() !== 'multipart/form-data') {
            mergedHeaders['Content-Type'] = contentType;
        }
        return Object.keys(mergedHeaders).length > 0 ? mergedHeaders : undefined;
    }
    buildRequestBody(body, contentType) {
        if (body == null) {
            return body;
        }
        const normalizedContentType = (contentType ?? '').toLowerCase();
        if (normalizedContentType === 'application/x-www-form-urlencoded') {
            return this.encodeFormBody(body);
        }
        return body;
    }
    encodeFormBody(body) {
        if (body instanceof URLSearchParams) {
            return body.toString();
        }
        if (typeof body === 'string') {
            return body;
        }
        const params = new URLSearchParams();
        if (body instanceof Map) {
            for (const [key, value] of body.entries()) {
                this.appendFormValue(params, String(key), value);
            }
            return params.toString();
        }
        if (typeof body === 'object') {
            for (const [key, value] of Object.entries(body)) {
                this.appendFormValue(params, key, value);
            }
            return params.toString();
        }
        params.append('value', String(body));
        return params.toString();
    }
    appendFormValue(params, key, value) {
        if (value == null) {
            return;
        }
        if (Array.isArray(value)) {
            value.forEach((item) => this.appendFormValue(params, key, item));
            return;
        }
        if (value instanceof Date) {
            params.append(key, value.toISOString());
            return;
        }
        if (typeof value === 'object') {
            params.append(key, JSON.stringify(value));
            return;
        }
        params.append(key, String(value));
    }
    setAuthToken(token) {
        super.setAuthToken(token);
    }
    setTokenManager(manager) {
        const baseProto = Object.getPrototypeOf(HttpClient.prototype);
        if (typeof baseProto.setTokenManager === 'function') {
            baseProto.setTokenManager.call(this, manager);
            return;
        }
        this.getInternalAuthConfig().tokenManager = manager;
    }
    async request(path, options = {}) {
        const execute = this.execute;
        if (typeof execute !== 'function') {
            throw new Error('BaseHttpClient execute method is not available');
        }
        const { body, headers, contentType, method = 'GET', ...rest } = options;
        return withRetry(() => execute.call(this, {
            url: path,
            method,
            ...rest,
            body: this.buildRequestBody(body, contentType),
            headers: this.buildRequestHeaders(headers, body == null ? undefined : contentType),
        }), { maxRetries: 3 });
    }
    async get(path, params, headers) {
        return this.request(path, { method: 'GET', params, headers });
    }
    async post(path, body, params, headers, contentType) {
        return this.request(path, { method: 'POST', body, params, headers, contentType });
    }
    async put(path, body, params, headers, contentType) {
        return this.request(path, { method: 'PUT', body, params, headers, contentType });
    }
    async delete(path, params, headers) {
        return this.request(path, { method: 'DELETE', params, headers });
    }
    async patch(path, body, params, headers, contentType) {
        return this.request(path, { method: 'PATCH', body, params, headers, contentType });
    }
}
export function createHttpClient(config) {
    return new HttpClient(config);
}
