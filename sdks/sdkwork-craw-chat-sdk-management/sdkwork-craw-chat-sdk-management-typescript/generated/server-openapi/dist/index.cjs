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
        const mergedHeaders = { ...(headers ?? {}) };
        if (contentType && contentType.toLowerCase() !== 'multipart/form-data') {
            mergedHeaders['Content-Type'] = contentType;
        }
        return Object.keys(mergedHeaders).length > 0 ? mergedHeaders : undefined;
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
        headers[HttpClient.API_KEY_HEADER] = HttpClient.API_KEY_USE_BEARER ? `Bearer ${apiKey}` : apiKey;
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
            body,
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
    const normalizedPrefix = normalizedPrefixRaw ? `/${normalizedPrefixRaw.replace(/^\/+|\/+$/g, '')}` : '';
    const normalizedPath = path.startsWith('/') ? path : `/${path}`;
    if (!normalizedPrefix || normalizedPrefix === '/') {
        return normalizedPath;
    }
    if (normalizedPath === normalizedPrefix || normalizedPath.startsWith(`${normalizedPrefix}/`)) {
        return normalizedPath;
    }
    return `${normalizedPrefix}${normalizedPath}`;
}

class AuthApi {
    constructor(client) {
        this.client = client;
    }
    async loginAdminUser(body) {
        return this.client.post(backendApiPath(`/api/admin/auth/login`), body);
    }
    async getAdminMe() {
        return this.client.get(backendApiPath(`/api/admin/auth/me`));
    }
}
function createAuthApi(client) {
    return new AuthApi(client);
}

class UsersApi {
    constructor(client) {
        this.client = client;
    }
    async listOperatorUsers() {
        return this.client.get(backendApiPath(`/api/admin/users/operators`));
    }
    async saveOperatorUser(body) {
        return this.client.post(backendApiPath(`/api/admin/users/operators`), body);
    }
    async deleteOperatorUser(userId) {
        return this.client.delete(backendApiPath(`/api/admin/users/operators/${encodeURIComponent(String(userId))}`));
    }
    async updateOperatorUserStatus(userId, body) {
        return this.client.post(backendApiPath(`/api/admin/users/operators/${encodeURIComponent(String(userId))}/status`), body);
    }
    async resetOperatorUserPassword(userId, body) {
        return this.client.post(backendApiPath(`/api/admin/users/operators/${encodeURIComponent(String(userId))}/password`), body);
    }
    async listPortalUsers() {
        return this.client.get(backendApiPath(`/api/admin/users/portal`));
    }
    async savePortalUser(body) {
        return this.client.post(backendApiPath(`/api/admin/users/portal`), body);
    }
    async deletePortalUser(userId) {
        return this.client.delete(backendApiPath(`/api/admin/users/portal/${encodeURIComponent(String(userId))}`));
    }
    async updatePortalUserStatus(userId, body) {
        return this.client.post(backendApiPath(`/api/admin/users/portal/${encodeURIComponent(String(userId))}/status`), body);
    }
    async resetPortalUserPassword(userId, body) {
        return this.client.post(backendApiPath(`/api/admin/users/portal/${encodeURIComponent(String(userId))}/password`), body);
    }
}
function createUsersApi(client) {
    return new UsersApi(client);
}

class MarketingApi {
    constructor(client) {
        this.client = client;
    }
    async listMarketingCampaigns() {
        return this.client.get(backendApiPath(`/api/admin/marketing/campaigns`));
    }
    async saveMarketingCampaign(body) {
        return this.client.post(backendApiPath(`/api/admin/marketing/campaigns`), body);
    }
    async updateMarketingCampaignStatus(marketingCampaignId, body) {
        return this.client.post(backendApiPath(`/api/admin/marketing/campaigns/${encodeURIComponent(String(marketingCampaignId))}/status`), body);
    }
}
function createMarketingApi(client) {
    return new MarketingApi(client);
}

class TenantsApi {
    constructor(client) {
        this.client = client;
    }
    async listTenants() {
        return this.client.get(backendApiPath(`/api/admin/tenants`));
    }
    async saveTenant(body) {
        return this.client.post(backendApiPath(`/api/admin/tenants`), body);
    }
    async deleteTenant(tenantId) {
        return this.client.delete(backendApiPath(`/api/admin/tenants/${encodeURIComponent(String(tenantId))}`));
    }
    async listProjects() {
        return this.client.get(backendApiPath(`/api/admin/projects`));
    }
    async saveProject(body) {
        return this.client.post(backendApiPath(`/api/admin/projects`), body);
    }
    async deleteProject(projectId) {
        return this.client.delete(backendApiPath(`/api/admin/projects/${encodeURIComponent(String(projectId))}`));
    }
}
function createTenantsApi(client) {
    return new TenantsApi(client);
}

class AccessApi {
    constructor(client) {
        this.client = client;
    }
    async listApiKeys() {
        return this.client.get(backendApiPath(`/api/admin/api-keys`));
    }
    async createApiKey(body) {
        return this.client.post(backendApiPath(`/api/admin/api-keys`), body);
    }
    async updateApiKey(hashedKey, body) {
        return this.client.put(backendApiPath(`/api/admin/api-keys/${encodeURIComponent(String(hashedKey))}`), body);
    }
    async deleteApiKey(hashedKey) {
        return this.client.delete(backendApiPath(`/api/admin/api-keys/${encodeURIComponent(String(hashedKey))}`));
    }
    async updateApiKeyStatus(hashedKey, body) {
        return this.client.post(backendApiPath(`/api/admin/api-keys/${encodeURIComponent(String(hashedKey))}/status`), body);
    }
    async listApiKeyGroups() {
        return this.client.get(backendApiPath(`/api/admin/api-key-groups`));
    }
    async createApiKeyGroup(body) {
        return this.client.post(backendApiPath(`/api/admin/api-key-groups`), body);
    }
    async updateApiKeyGroup(groupId, body) {
        return this.client.patch(backendApiPath(`/api/admin/api-key-groups/${encodeURIComponent(String(groupId))}`), body);
    }
    async deleteApiKeyGroup(groupId) {
        return this.client.delete(backendApiPath(`/api/admin/api-key-groups/${encodeURIComponent(String(groupId))}`));
    }
    async updateApiKeyGroupStatus(groupId, body) {
        return this.client.post(backendApiPath(`/api/admin/api-key-groups/${encodeURIComponent(String(groupId))}/status`), body);
    }
}
function createAccessApi(client) {
    return new AccessApi(client);
}

class RoutingApi {
    constructor(client) {
        this.client = client;
    }
    async listRoutingProfiles() {
        return this.client.get(backendApiPath(`/api/admin/routing/profiles`));
    }
    async createRoutingProfile(body) {
        return this.client.post(backendApiPath(`/api/admin/routing/profiles`), body);
    }
    async listCompiledRoutingSnapshots() {
        return this.client.get(backendApiPath(`/api/admin/routing/snapshots`));
    }
    async listRoutingDecisionLogs() {
        return this.client.get(backendApiPath(`/api/admin/routing/decision-logs`));
    }
    async listProviderHealthSnapshots() {
        return this.client.get(backendApiPath(`/api/admin/routing/health-snapshots`));
    }
}
function createRoutingApi(client) {
    return new RoutingApi(client);
}

class CatalogApi {
    constructor(client) {
        this.client = client;
    }
    async listChannels() {
        return this.client.get(backendApiPath(`/api/admin/channels`));
    }
    async saveChannel(body) {
        return this.client.post(backendApiPath(`/api/admin/channels`), body);
    }
    async deleteChannel(channelId) {
        return this.client.delete(backendApiPath(`/api/admin/channels/${encodeURIComponent(String(channelId))}`));
    }
    async listProviders() {
        return this.client.get(backendApiPath(`/api/admin/providers`));
    }
    async saveProvider(body) {
        return this.client.post(backendApiPath(`/api/admin/providers`), body);
    }
    async deleteProvider(providerId) {
        return this.client.delete(backendApiPath(`/api/admin/providers/${encodeURIComponent(String(providerId))}`));
    }
    async listCredentials() {
        return this.client.get(backendApiPath(`/api/admin/credentials`));
    }
    async saveCredential(body) {
        return this.client.post(backendApiPath(`/api/admin/credentials`), body);
    }
    async deleteCredential(tenantId, providerId, keyReference) {
        return this.client.delete(backendApiPath(`/api/admin/credentials/${encodeURIComponent(String(tenantId))}/providers/${encodeURIComponent(String(providerId))}/keys/${encodeURIComponent(String(keyReference))}`));
    }
    async listModels() {
        return this.client.get(backendApiPath(`/api/admin/models`));
    }
    async saveModel(body) {
        return this.client.post(backendApiPath(`/api/admin/models`), body);
    }
    async deleteModel(externalName, providerId) {
        return this.client.delete(backendApiPath(`/api/admin/models/${encodeURIComponent(String(externalName))}/providers/${encodeURIComponent(String(providerId))}`));
    }
    async listChannelModels() {
        return this.client.get(backendApiPath(`/api/admin/channel-models`));
    }
    async saveChannelModel(body) {
        return this.client.post(backendApiPath(`/api/admin/channel-models`), body);
    }
    async deleteChannelModel(channelId, modelId) {
        return this.client.delete(backendApiPath(`/api/admin/channel-models/${encodeURIComponent(String(channelId))}/models/${encodeURIComponent(String(modelId))}`));
    }
    async listModelPrices() {
        return this.client.get(backendApiPath(`/api/admin/model-prices`));
    }
    async saveModelPrice(body) {
        return this.client.post(backendApiPath(`/api/admin/model-prices`), body);
    }
    async deleteModelPrice(channelId, modelId, proxyProviderId) {
        return this.client.delete(backendApiPath(`/api/admin/model-prices/${encodeURIComponent(String(channelId))}/models/${encodeURIComponent(String(modelId))}/providers/${encodeURIComponent(String(proxyProviderId))}`));
    }
}
function createCatalogApi(client) {
    return new CatalogApi(client);
}

class UsageApi {
    constructor(client) {
        this.client = client;
    }
    async listUsageRecords() {
        return this.client.get(backendApiPath(`/api/admin/usage/records`));
    }
    async getUsageSummary() {
        return this.client.get(backendApiPath(`/api/admin/usage/summary`));
    }
}
function createUsageApi(client) {
    return new UsageApi(client);
}

class BillingApi {
    constructor(client) {
        this.client = client;
    }
    async listBillingEvents() {
        return this.client.get(backendApiPath(`/api/admin/billing/events`));
    }
    async getBillingEventSummary() {
        return this.client.get(backendApiPath(`/api/admin/billing/events/summary`));
    }
    async getBillingSummary() {
        return this.client.get(backendApiPath(`/api/admin/billing/summary`));
    }
}
function createBillingApi(client) {
    return new BillingApi(client);
}

class OperationsApi {
    constructor(client) {
        this.client = client;
    }
    async listRateLimitPolicies() {
        return this.client.get(backendApiPath(`/api/admin/gateway/rate-limit-policies`));
    }
    async createRateLimitPolicy(body) {
        return this.client.post(backendApiPath(`/api/admin/gateway/rate-limit-policies`), body);
    }
    async listRateLimitWindows() {
        return this.client.get(backendApiPath(`/api/admin/gateway/rate-limit-windows`));
    }
    async listRuntimeStatuses() {
        return this.client.get(backendApiPath(`/api/admin/extensions/runtime-statuses`));
    }
    async reloadExtensionRuntimes(body) {
        return this.client.post(backendApiPath(`/api/admin/extensions/runtime-reloads`), body);
    }
}
function createOperationsApi(client) {
    return new OperationsApi(client);
}

class SdkworkBackendClient {
    constructor(config) {
        this.httpClient = createHttpClient(config);
        this.auth = createAuthApi(this.httpClient);
        this.users = createUsersApi(this.httpClient);
        this.marketing = createMarketingApi(this.httpClient);
        this.tenants = createTenantsApi(this.httpClient);
        this.access = createAccessApi(this.httpClient);
        this.routing = createRoutingApi(this.httpClient);
        this.catalog = createCatalogApi(this.httpClient);
        this.usage = createUsageApi(this.httpClient);
        this.billing = createBillingApi(this.httpClient);
        this.operations = createOperationsApi(this.httpClient);
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
exports.AccessApi = AccessApi;
exports.AuthApi = AuthApi;
exports.BACKEND_API_PREFIX = BACKEND_API_PREFIX;
exports.BillingApi = BillingApi;
exports.CatalogApi = CatalogApi;
exports.HttpClient = HttpClient;
exports.MarketingApi = MarketingApi;
exports.OperationsApi = OperationsApi;
exports.RoutingApi = RoutingApi;
exports.SdkworkBackendClient = SdkworkBackendClient;
exports.TenantsApi = TenantsApi;
exports.UsageApi = UsageApi;
exports.UsersApi = UsersApi;
exports.backendApiPath = backendApiPath;
exports.createAccessApi = createAccessApi;
exports.createAuthApi = createAuthApi;
exports.createBillingApi = createBillingApi;
exports.createCatalogApi = createCatalogApi;
exports.createClient = createClient;
exports.createHttpClient = createHttpClient;
exports.createMarketingApi = createMarketingApi;
exports.createOperationsApi = createOperationsApi;
exports.createRoutingApi = createRoutingApi;
exports.createTenantsApi = createTenantsApi;
exports.createUsageApi = createUsageApi;
exports.createUsersApi = createUsersApi;
//# sourceMappingURL=index.cjs.map
