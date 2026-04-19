'use strict';

var sdkCommon = require('@sdkwork/sdk-common');

function hasOwn(value, key) {
    return Object.prototype.hasOwnProperty.call(value, key);
}
class HttpClient extends sdkCommon.BaseHttpClient {
    constructor(config) {
        super(config);
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
    isResultEnvelope(value) {
        return typeof value === 'object'
            && value !== null
            && (hasOwn(value, 'code') || hasOwn(value, 'data') || hasOwn(value, 'msg') || hasOwn(value, 'message'));
    }
    hasSuccessCode(code) {
        return sdkCommon.SUCCESS_CODES.includes(code) || sdkCommon.SUCCESS_CODES.includes(String(code));
    }
    async handleErrorResponse(response, requestConfig) {
        let payload = null;
        try {
            payload = await response.json();
        }
        catch {
            payload = null;
        }
        let message = `HTTP ${response.status}: ${response.statusText}`;
        if (typeof payload === 'object' && payload !== null) {
            const candidate = payload;
            message = candidate.error?.message?.trim()
                || candidate.msg?.trim()
                || candidate.message?.trim()
                || message;
        }
        const error = sdkCommon.SdkError.fromHttpStatus(response.status, message);
        const applyErrorInterceptors = this.applyErrorInterceptors;
        if (typeof applyErrorInterceptors === 'function') {
            await applyErrorInterceptors.call(this, error, requestConfig);
        }
        throw error;
    }
    async processResponse(response, requestConfig) {
        if (!response.ok) {
            return this.handleErrorResponse(response, requestConfig);
        }
        if (response.status === 204) {
            return undefined;
        }
        const contentType = response.headers.get('content-type') ?? '';
        if (contentType.includes('application/json')) {
            const result = await response.json();
            if (this.isResultEnvelope(result) && this.hasSuccessCode(result.code)) {
                return result.data;
            }
            if (this.isResultEnvelope(result) && hasOwn(result, 'code')) {
                throw sdkCommon.SdkError.fromApiResult(result, response.status);
            }
            return result;
        }
        if (contentType.includes('text/')) {
            return await response.text();
        }
        return await response.json();
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
        this.authConfig = this.authConfig || {};
        this.authConfig.tokenManager = manager;
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

class StorageApi {
    constructor(client) {
        this.client = client;
    }
    async listStorageProviders() {
        return this.client.get(backendApiPath(`/api/admin/storage/providers`));
    }
    async getGlobalStorageConfig() {
        return this.client.get(backendApiPath(`/api/admin/storage/config`));
    }
    async saveGlobalStorageConfig(body) {
        return this.client.post(backendApiPath(`/api/admin/storage/config`), body);
    }
    async getTenantStorageConfig(tenantId) {
        return this.client.get(backendApiPath(`/api/admin/storage/config/tenants/${encodeURIComponent(String(tenantId))}`));
    }
    async saveTenantStorageConfig(tenantId, body) {
        return this.client.post(backendApiPath(`/api/admin/storage/config/tenants/${encodeURIComponent(String(tenantId))}`), body);
    }
    async deleteTenantStorageConfig(tenantId) {
        return this.client.delete(backendApiPath(`/api/admin/storage/config/tenants/${encodeURIComponent(String(tenantId))}`));
    }
    async getTenantEffectiveStorageConfig(tenantId) {
        return this.client.get(backendApiPath(`/api/admin/storage/effective/tenants/${encodeURIComponent(String(tenantId))}`));
    }
    async validateGlobalStorageConfig(body) {
        return this.client.post(backendApiPath(`/api/admin/storage/validate`), body);
    }
    async validateTenantStorageConfig(tenantId, body) {
        return this.client.post(backendApiPath(`/api/admin/storage/validate/tenants/${encodeURIComponent(String(tenantId))}`), body);
    }
    async listStorageAuditTrail() {
        return this.client.get(backendApiPath(`/api/admin/storage/audit`));
    }
}
function createStorageApi(client) {
    return new StorageApi(client);
}

class ImAdminBackendClient {
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
        this.storage = createStorageApi(this.httpClient);
    }
    setAuthToken(token) {
        this.httpClient.setAuthToken(token);
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
function createImAdminBackendClient(config) {
    return new ImAdminBackendClient(config);
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
exports.ImAdminBackendClient = ImAdminBackendClient;
exports.MarketingApi = MarketingApi;
exports.OperationsApi = OperationsApi;
exports.RoutingApi = RoutingApi;
exports.StorageApi = StorageApi;
exports.TenantsApi = TenantsApi;
exports.UsageApi = UsageApi;
exports.UsersApi = UsersApi;
exports.backendApiPath = backendApiPath;
exports.createAccessApi = createAccessApi;
exports.createAuthApi = createAuthApi;
exports.createBillingApi = createBillingApi;
exports.createCatalogApi = createCatalogApi;
exports.createHttpClient = createHttpClient;
exports.createImAdminBackendClient = createImAdminBackendClient;
exports.createMarketingApi = createMarketingApi;
exports.createOperationsApi = createOperationsApi;
exports.createRoutingApi = createRoutingApi;
exports.createStorageApi = createStorageApi;
exports.createTenantsApi = createTenantsApi;
exports.createUsageApi = createUsageApi;
exports.createUsersApi = createUsersApi;
//# sourceMappingURL=index.cjs.map
