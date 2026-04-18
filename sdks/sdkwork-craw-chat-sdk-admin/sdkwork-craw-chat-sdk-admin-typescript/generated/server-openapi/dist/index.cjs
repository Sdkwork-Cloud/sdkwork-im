'use strict';

var sdkCommon = require('@sdkwork/sdk-common');

class HttpClient extends sdkCommon.BaseHttpClient {
    constructor(config) {
        super(config);
    }
    getInternalAuthConfig() {
        const self = this;
        self.authConfig = self.authConfig || {};
        return self.authConfig;
    }
    getInternalHeaders() {
        const self = this;
        self.config = self.config || {};
        self.config.headers = self.config.headers || {};
        return self.config.headers;
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
    setApiKey(apiKey) {
        const authConfig = this.getInternalAuthConfig();
        const headers = this.getInternalHeaders();
        authConfig.apiKey = apiKey;
        authConfig.tokenManager?.clearTokens?.();
        if (HttpClient.API_KEY_HEADER === 'Authorization' && HttpClient.API_KEY_USE_BEARER) {
            authConfig.authMode = 'apikey';
            delete headers['Access-Token'];
            return;
        }
        authConfig.authMode = 'dual-token';
        headers[HttpClient.API_KEY_HEADER] = HttpClient.API_KEY_USE_BEARER
            ? `Bearer ${apiKey}`
            : apiKey;
        if (HttpClient.API_KEY_HEADER.toLowerCase() !== 'authorization') {
            delete headers['Authorization'];
        }
        if (HttpClient.API_KEY_HEADER.toLowerCase() !== 'access-token') {
            delete headers['Access-Token'];
        }
    }
    setAuthToken(token) {
        const headers = this.getInternalHeaders();
        if (HttpClient.API_KEY_HEADER.toLowerCase() !== 'authorization') {
            delete headers[HttpClient.API_KEY_HEADER];
        }
        super.setAuthToken(token);
    }
    setAccessToken(token) {
        const headers = this.getInternalHeaders();
        if (HttpClient.API_KEY_HEADER.toLowerCase() !== 'access-token') {
            delete headers[HttpClient.API_KEY_HEADER];
        }
        super.setAccessToken(token);
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
        return sdkCommon.withRetry(() => execute.call(this, {
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
HttpClient.API_KEY_HEADER = 'Authorization';
HttpClient.API_KEY_USE_BEARER = true;
function createHttpClient(config) {
    return new HttpClient(config);
}

const BACKEND_API_PREFIX = '';
function backendApiPath(path) {
    if (!path) {
        return BACKEND_API_PREFIX;
    }
    if (/^https?:\/\//i.test(path)) {
        return path;
    }
    const normalizedPrefixRaw = ('').trim();
    const normalizedPrefix = normalizedPrefixRaw
        ? `/${normalizedPrefixRaw.replace(/^\/+|\/+$/g, '')}`
        : '';
    const normalizedPath = path.startsWith('/') ? path : `/${path}`;
    if (!normalizedPrefix || normalizedPrefix === '/') {
        return normalizedPath;
    }
    if (normalizedPath === normalizedPrefix || normalizedPath.startsWith(`${normalizedPrefix}/`)) {
        return normalizedPath;
    }
    return `${normalizedPrefix}${normalizedPath}`;
}

class ClusterApi {
    constructor(client) {
        this.client = client;
    }
    /** Post nodes {node_id} activate */
    async postApiV1ControlNodesIdActivate(nodeId) {
        return this.client.post(backendApiPath(`/api/v1/control/nodes/${nodeId}/activate`));
    }
    /** Post nodes {node_id} drain */
    async postApiV1ControlNodesIdDrain(nodeId) {
        return this.client.post(backendApiPath(`/api/v1/control/nodes/${nodeId}/drain`));
    }
    /** Post nodes {node_id} routes migrate */
    async postApiV1ControlNodesIdRoutesMigrate(nodeId) {
        return this.client.post(backendApiPath(`/api/v1/control/nodes/${nodeId}/routes/migrate`));
    }
}
function createClusterApi(client) {
    return new ClusterApi(client);
}

class ProtocolApi {
    constructor(client) {
        this.client = client;
    }
    /** Get protocol governance snapshot */
    async getApiV1ControlProtocolGovernance() {
        return this.client.get(backendApiPath(`/api/v1/control/protocol-governance`));
    }
    /** Get protocol registry snapshot */
    async getApiV1ControlProtocolRegistry() {
        return this.client.get(backendApiPath(`/api/v1/control/protocol-registry`));
    }
}
function createProtocolApi(client) {
    return new ProtocolApi(client);
}

class ProvidersApi {
    constructor(client) {
        this.client = client;
    }
    /** Get provider-bindings */
    async getApiV1ControlProviderBindings() {
        return this.client.get(backendApiPath(`/api/v1/control/provider-bindings`));
    }
    /** Post provider-bindings */
    async postApiV1ControlProviderBindings() {
        return this.client.post(backendApiPath(`/api/v1/control/provider-bindings`));
    }
    /** Get provider-policies */
    async getApiV1ControlProviderPolicies() {
        return this.client.get(backendApiPath(`/api/v1/control/provider-policies`));
    }
    /** Get provider-policies diff */
    async getApiV1ControlProviderPoliciesDiff() {
        return this.client.get(backendApiPath(`/api/v1/control/provider-policies/diff`));
    }
    /** Post provider-policies preview */
    async postApiV1ControlProviderPoliciesPreview() {
        return this.client.post(backendApiPath(`/api/v1/control/provider-policies/preview`));
    }
    /** Post provider-policies rollback */
    async postApiV1ControlProviderPoliciesRollback() {
        return this.client.post(backendApiPath(`/api/v1/control/provider-policies/rollback`));
    }
    /** Get provider registry snapshot */
    async getApiV1ControlProviderRegistry() {
        return this.client.get(backendApiPath(`/api/v1/control/provider-registry`));
    }
}
function createProvidersApi(client) {
    return new ProvidersApi(client);
}

class SocialApi {
    constructor(client) {
        this.client = client;
    }
    /** Post social direct-chats bindings */
    async postApiV1ControlSocialDirectChatsBindings() {
        return this.client.post(backendApiPath(`/api/v1/control/social/direct-chats/bindings`));
    }
    /** Get social direct-chats {direct_chat_id} */
    async getApiV1ControlSocialDirectChatsId(directChatId) {
        return this.client.get(backendApiPath(`/api/v1/control/social/direct-chats/${directChatId}`));
    }
    /** Post social external-connections */
    async postApiV1ControlSocialExternalConnections() {
        return this.client.post(backendApiPath(`/api/v1/control/social/external-connections`));
    }
    /** Get social external-connections {connection_id} */
    async getApiV1ControlSocialExternalConnectionsId(connectionId) {
        return this.client.get(backendApiPath(`/api/v1/control/social/external-connections/${connectionId}`));
    }
    /** Post social external-member-links */
    async postApiV1ControlSocialExternalMemberLinks() {
        return this.client.post(backendApiPath(`/api/v1/control/social/external-member-links`));
    }
    /** Get social external-member-links {link_id} */
    async getApiV1ControlSocialExternalMemberLinksId(linkId) {
        return this.client.get(backendApiPath(`/api/v1/control/social/external-member-links/${linkId}`));
    }
    /** Post social friend-requests */
    async postApiV1ControlSocialFriendRequests() {
        return this.client.post(backendApiPath(`/api/v1/control/social/friend-requests`));
    }
    /** Get social friend-requests {request_id} */
    async getApiV1ControlSocialFriendRequestsId(requestId) {
        return this.client.get(backendApiPath(`/api/v1/control/social/friend-requests/${requestId}`));
    }
    /** Post social friendships */
    async postApiV1ControlSocialFriendships() {
        return this.client.post(backendApiPath(`/api/v1/control/social/friendships`));
    }
    /** Get social friendships {friendship_id} */
    async getApiV1ControlSocialFriendshipsId(friendshipId) {
        return this.client.get(backendApiPath(`/api/v1/control/social/friendships/${friendshipId}`));
    }
    /** Post social runtime claim-pending-shared-channel-sync-targeted */
    async postApiV1ControlSocialRuntimeClaimPendingSharedChannelSyncTargeted() {
        return this.client.post(backendApiPath(`/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted`));
    }
    /** Get social runtime dead-letter-shared-channel-sync */
    async getApiV1ControlSocialRuntimeDeadLetterSharedChannelSync() {
        return this.client.get(backendApiPath(`/api/v1/control/social/runtime/dead-letter-shared-channel-sync`));
    }
    /** Get social runtime delivered-shared-channel-sync */
    async getApiV1ControlSocialRuntimeDeliveredSharedChannelSync() {
        return this.client.get(backendApiPath(`/api/v1/control/social/runtime/delivered-shared-channel-sync`));
    }
    /** Get social runtime delivery-state-shared-channel-sync */
    async getApiV1ControlSocialRuntimeDeliveryStateSharedChannelSync() {
        return this.client.get(backendApiPath(`/api/v1/control/social/runtime/delivery-state-shared-channel-sync`));
    }
    /** Get social runtime pending-shared-channel-sync */
    async getApiV1ControlSocialRuntimePendingSharedChannelSync() {
        return this.client.get(backendApiPath(`/api/v1/control/social/runtime/pending-shared-channel-sync`));
    }
    /** Post social runtime reclaim-stale-pending-shared-channel-sync */
    async postApiV1ControlSocialRuntimeReclaimStalePendingSharedChannelSync() {
        return this.client.post(backendApiPath(`/api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync`));
    }
    /** Post social runtime release-pending-shared-channel-sync-targeted */
    async postApiV1ControlSocialRuntimeReleasePendingSharedChannelSyncTargeted() {
        return this.client.post(backendApiPath(`/api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted`));
    }
    /** Post social runtime repair-derived-snapshot */
    async postApiV1ControlSocialRuntimeRepairDerivedSnapshot() {
        return this.client.post(backendApiPath(`/api/v1/control/social/runtime/repair-derived-snapshot`));
    }
    /** Post social runtime repair-shared-channel-sync */
    async postApiV1ControlSocialRuntimeRepairSharedChannelSync() {
        return this.client.post(backendApiPath(`/api/v1/control/social/runtime/repair-shared-channel-sync`));
    }
    /** Post social runtime republish-pending-shared-channel-sync-targeted */
    async postApiV1ControlSocialRuntimeRepublishPendingSharedChannelSyncTargeted() {
        return this.client.post(backendApiPath(`/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted`));
    }
    /** Post social runtime requeue-dead-letter-shared-channel-sync */
    async postApiV1ControlSocialRuntimeRequeueDeadLetterSharedChannelSync() {
        return this.client.post(backendApiPath(`/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync`));
    }
    /** Post social runtime requeue-dead-letter-shared-channel-sync-targeted */
    async postApiV1ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargeted() {
        return this.client.post(backendApiPath(`/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted`));
    }
    /** Post social runtime takeover-pending-shared-channel-sync-targeted */
    async postApiV1ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargeted() {
        return this.client.post(backendApiPath(`/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted`));
    }
    /** Post social shared-channel-policies */
    async postApiV1ControlSocialSharedChannelPolicies() {
        return this.client.post(backendApiPath(`/api/v1/control/social/shared-channel-policies`));
    }
    /** Get social shared-channel-policies {policy_id} */
    async getApiV1ControlSocialSharedChannelPoliciesPolicyId(policyId) {
        return this.client.get(backendApiPath(`/api/v1/control/social/shared-channel-policies/${policyId}`));
    }
    /** Post social user-blocks */
    async postApiV1ControlSocialUserBlocks() {
        return this.client.post(backendApiPath(`/api/v1/control/social/user-blocks`));
    }
    /** Get social user-blocks {block_id} */
    async getApiV1ControlSocialUserBlocksId(blockId) {
        return this.client.get(backendApiPath(`/api/v1/control/social/user-blocks/${blockId}`));
    }
}
function createSocialApi(client) {
    return new SocialApi(client);
}

class SystemApi {
    constructor(client) {
        this.client = client;
    }
    /** Check control plane health */
    async getHealthz() {
        return this.client.get(backendApiPath(`/healthz`));
    }
}
function createSystemApi(client) {
    return new SystemApi(client);
}

class SdkworkBackendClient {
    constructor(config) {
        this.httpClient = createHttpClient(config);
        this.cluster = createClusterApi(this.httpClient);
        this.protocol = createProtocolApi(this.httpClient);
        this.providers = createProvidersApi(this.httpClient);
        this.social = createSocialApi(this.httpClient);
        this.system = createSystemApi(this.httpClient);
    }
    setApiKey(apiKey) {
        this.httpClient.setApiKey(apiKey);
        return this;
    }
    setAuthToken(token) {
        this.httpClient.setAuthToken(token);
        return this;
    }
    setAccessToken(token) {
        this.httpClient.setAccessToken(token);
        return this;
    }
    setTokenManager(manager) {
        this.httpClient.setTokenManager(manager);
        return this;
    }
    get http() {
        return this.httpClient;
    }
}
function createClient(config) {
    return new SdkworkBackendClient(config);
}

class BaseApi {
    constructor(http, basePath) {
        this.http = http;
        this.basePath = basePath;
    }
    async get(path, params, headers) {
        return this.http.get(`${this.basePath}${path}`, params, headers);
    }
    async post(path, body, params, headers, contentType) {
        return this.http.post(`${this.basePath}${path}`, body, params, headers, contentType);
    }
    async put(path, body, params, headers, contentType) {
        return this.http.put(`${this.basePath}${path}`, body, params, headers, contentType);
    }
    async delete(path, params, headers) {
        return this.http.delete(`${this.basePath}${path}`, params, headers);
    }
    async patch(path, body, params, headers, contentType) {
        return this.http.patch(`${this.basePath}${path}`, body, params, headers, contentType);
    }
}

Object.defineProperty(exports, "DEFAULT_TIMEOUT", {
    enumerable: true,
    get: function () { return sdkCommon.DEFAULT_TIMEOUT; }
});
Object.defineProperty(exports, "DefaultAuthTokenManager", {
    enumerable: true,
    get: function () { return sdkCommon.DefaultAuthTokenManager; }
});
Object.defineProperty(exports, "SUCCESS_CODES", {
    enumerable: true,
    get: function () { return sdkCommon.SUCCESS_CODES; }
});
Object.defineProperty(exports, "createTokenManager", {
    enumerable: true,
    get: function () { return sdkCommon.createTokenManager; }
});
exports.BaseApi = BaseApi;
exports.ClusterApi = ClusterApi;
exports.HttpClient = HttpClient;
exports.ProtocolApi = ProtocolApi;
exports.ProvidersApi = ProvidersApi;
exports.SdkworkBackendClient = SdkworkBackendClient;
exports.SocialApi = SocialApi;
exports.SystemApi = SystemApi;
exports.backendApiPath = backendApiPath;
exports.createClient = createClient;
exports.createClusterApi = createClusterApi;
exports.createHttpClient = createHttpClient;
exports.createProtocolApi = createProtocolApi;
exports.createProvidersApi = createProvidersApi;
exports.createSocialApi = createSocialApi;
exports.createSystemApi = createSystemApi;
//# sourceMappingURL=index.cjs.map
