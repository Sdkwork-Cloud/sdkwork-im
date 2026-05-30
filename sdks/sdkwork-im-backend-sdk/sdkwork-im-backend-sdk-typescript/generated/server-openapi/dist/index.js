import { BaseHttpClient, withRetry } from '@sdkwork/sdk-common';
export { DEFAULT_TIMEOUT, DefaultAuthTokenManager, SUCCESS_CODES, createTokenManager } from '@sdkwork/sdk-common';

class HttpClient extends BaseHttpClient {
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
        if (normalizedContentType === 'multipart/form-data') {
            return this.encodeMultipartBody(body);
        }
        return body;
    }
    encodeMultipartBody(body) {
        if (body instanceof FormData) {
            return body;
        }
        const formData = new FormData();
        if (body instanceof Map) {
            for (const [key, value] of body.entries()) {
                this.appendMultipartValue(formData, String(key), value);
            }
            return formData;
        }
        if (typeof body === 'object') {
            const record = body;
            for (const [key, value] of Object.entries(record)) {
                if (this.isMultipartMetadataField(key)) {
                    continue;
                }
                this.appendMultipartValue(formData, key, value, this.resolveMultipartFileName(record, key));
            }
            return formData;
        }
        this.appendMultipartValue(formData, 'value', body);
        return formData;
    }
    appendMultipartValue(formData, key, value, fileName) {
        if (value == null) {
            return;
        }
        if (Array.isArray(value)) {
            value.forEach((item) => this.appendMultipartValue(formData, key, item, fileName));
            return;
        }
        if (value instanceof Blob) {
            if (fileName) {
                formData.append(key, value, fileName);
                return;
            }
            formData.append(key, value);
            return;
        }
        if (value instanceof Date) {
            formData.append(key, value.toISOString());
            return;
        }
        if (typeof value === 'object') {
            formData.append(key, JSON.stringify(value));
            return;
        }
        formData.append(key, String(value));
    }
    resolveMultipartFileName(record, key) {
        const fieldSpecificName = record[`${key}FileName`];
        if (typeof fieldSpecificName === 'string' && fieldSpecificName.trim()) {
            return fieldSpecificName.trim();
        }
        const genericName = record.fileName;
        if (key === 'file' && typeof genericName === 'string' && genericName.trim()) {
            return genericName.trim();
        }
        return undefined;
    }
    isMultipartMetadataField(key) {
        return key === 'fileName' || key.endsWith('FileName');
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
    setAccessToken(token) {
        const headers = this.getInternalHeaders();
        headers[HttpClient.ACCESS_TOKEN_HEADER] = token;
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
    applySdkworkAuthHeaders(headers) {
        const authConfig = this.getInternalAuthConfig();
        const tokenManager = authConfig.tokenManager;
        const accessToken = tokenManager?.getAccessToken?.();
        if (!accessToken) {
            return headers;
        }
        return {
            ...(headers ?? {}),
            [HttpClient.ACCESS_TOKEN_HEADER]: accessToken,
        };
    }
    async request(path, options = {}) {
        const execute = this.execute;
        if (typeof execute !== 'function') {
            throw new Error('BaseHttpClient execute method is not available');
        }
        const { body, headers, contentType, method = 'GET', ...rest } = options;
        const requestHeaders = this.applySdkworkAuthHeaders(headers);
        return withRetry(() => execute.call(this, {
            url: path,
            method,
            ...rest,
            body: this.buildRequestBody(body, contentType),
            headers: this.buildRequestHeaders(requestHeaders, body == null ? undefined : contentType),
        }), { maxRetries: 3 });
    }
    async *streamJson(path, options = {}) {
        const stream = BaseHttpClient.prototype.stream;
        if (typeof stream !== 'function') {
            throw new Error('BaseHttpClient stream method is not available');
        }
        const { body, headers, contentType, method = 'GET', ...rest } = options;
        const authHeaders = this.applySdkworkAuthHeaders(headers);
        const requestHeaders = this.buildRequestHeaders({ Accept: 'text/event-stream', ...(authHeaders ?? {}) }, body == null ? undefined : contentType);
        for await (const data of stream.call(this, path, {
            method,
            ...rest,
            body: this.buildRequestBody(body, contentType),
            headers: requestHeaders,
        })) {
            if (data === '[DONE]') {
                return;
            }
            if (typeof data !== 'string' || data.trim().length === 0) {
                continue;
            }
            yield JSON.parse(data);
        }
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
HttpClient.ACCESS_TOKEN_HEADER = 'Access-Token';
function createHttpClient(config) {
    return new HttpClient(config);
}

const BACKEND_API_PREFIX = '/backend/v3/api';
function backendApiPath(path) {
    if (!path) {
        return BACKEND_API_PREFIX;
    }
    if (/^https?:\/\//i.test(path)) {
        return path;
    }
    const normalizedPrefixRaw = (BACKEND_API_PREFIX).trim();
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

class OpsDiagnosticsApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve diagnostics */
    async retrieve() {
        return this.client.get(backendApiPath(`/ops/diagnostics`));
    }
}
class OpsProviderBindingsDriftApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve provider binding drift */
    async list() {
        return this.client.get(backendApiPath(`/ops/provider_bindings/drift`));
    }
}
class OpsProviderBindingsApi {
    constructor(client) {
        this.client = client;
        this.drift = new OpsProviderBindingsDriftApi(client);
    }
    /** List provider bindings */
    async list() {
        return this.client.get(backendApiPath(`/ops/provider_bindings`));
    }
}
class OpsRuntimeDirApi {
    constructor(client) {
        this.client = client;
    }
    /** Inspect runtime directory */
    async retrieve() {
        return this.client.get(backendApiPath(`/ops/runtime_dir`));
    }
}
class OpsCommercialReadinessApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve commercial readiness */
    async retrieve() {
        return this.client.get(backendApiPath(`/ops/commercial_readiness`));
    }
}
class OpsReplayStatusApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve replay status */
    async retrieve() {
        return this.client.get(backendApiPath(`/ops/replay_status`));
    }
}
class OpsLagApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve projection lag */
    async retrieve() {
        return this.client.get(backendApiPath(`/ops/lag`));
    }
}
class OpsClusterApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve cluster state */
    async retrieve() {
        return this.client.get(backendApiPath(`/ops/cluster`));
    }
}
class OpsHealthApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve ops health */
    async retrieve() {
        return this.client.get(backendApiPath(`/ops/health`));
    }
}
class OpsApi {
    constructor(client) {
        this.client = client;
        this.health = new OpsHealthApi(client);
        this.cluster = new OpsClusterApi(client);
        this.lag = new OpsLagApi(client);
        this.replayStatus = new OpsReplayStatusApi(client);
        this.commercialReadiness = new OpsCommercialReadinessApi(client);
        this.runtimeDir = new OpsRuntimeDirApi(client);
        this.providerBindings = new OpsProviderBindingsApi(client);
        this.diagnostics = new OpsDiagnosticsApi(client);
    }
}
function createOpsApi(client) {
    return new OpsApi(client);
}

class AuditExportApi {
    constructor(client) {
        this.client = client;
    }
    /** Export audit bundle */
    async retrieve() {
        return this.client.get(backendApiPath(`/audit/export`));
    }
}
class AuditRecordsApi {
    constructor(client) {
        this.client = client;
    }
    /** List audit records */
    async list() {
        return this.client.get(backendApiPath(`/audit/records`));
    }
    /** Record audit anchor */
    async create() {
        return this.client.post(backendApiPath(`/audit/records`));
    }
}
class AuditApi {
    constructor(client) {
        this.client = client;
        this.records = new AuditRecordsApi(client);
        this.export = new AuditExportApi(client);
    }
}
function createAuditApi(client) {
    return new AuditApi(client);
}

class AutomationGovernanceApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve automation governance */
    async retrieve() {
        return this.client.get(backendApiPath(`/automation/governance`));
    }
}
class AutomationApi {
    constructor(client) {
        this.client = client;
        this.governance = new AutomationGovernanceApi(client);
    }
}
function createAutomationApi(client) {
    return new AutomationApi(client);
}

class ControlSocialUserBlocksApi {
    constructor(client) {
        this.client = client;
    }
    /** Block a user in the social graph. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/user_blocks`), body, undefined, undefined, 'application/json');
    }
    /** Read a user block snapshot. */
    async retrieve(blockId) {
        return this.client.get(backendApiPath(`/control/social/user_blocks/${serializePathParameter$1(blockId, { name: 'blockId', style: 'simple', explode: false })}`));
    }
}
class ControlSocialSharedChannelPoliciesApi {
    constructor(client) {
        this.client = client;
    }
    /** Apply a shared-channel policy. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/shared_channel_policies`), body, undefined, undefined, 'application/json');
    }
    /** Read a shared-channel policy snapshot. */
    async retrieve(policyId) {
        return this.client.get(backendApiPath(`/control/social/shared_channel_policies/${serializePathParameter$1(policyId, { name: 'policyId', style: 'simple', explode: false })}`));
    }
}
class ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi {
    constructor(client) {
        this.client = client;
    }
    /** Take over selected pending shared-channel sync entries. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/runtime/takeover_pending_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
    }
}
class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi {
    constructor(client) {
        this.client = client;
    }
    /** Requeue selected dead-letter shared-channel sync entries. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
    }
}
class ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi {
    constructor(client) {
        this.client = client;
    }
    /** Requeue all dead-letter shared-channel sync entries. */
    async create() {
        return this.client.post(backendApiPath(`/control/social/runtime/requeue_dead_letter_shared_channel_sync`));
    }
}
class ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi {
    constructor(client) {
        this.client = client;
    }
    /** Republish selected pending shared-channel sync entries. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/runtime/republish_pending_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
    }
}
class ControlSocialRuntimeRepairSharedChannelSyncApi {
    constructor(client) {
        this.client = client;
    }
    /** Repair shared-channel sync backlog state. */
    async create() {
        return this.client.post(backendApiPath(`/control/social/runtime/repair_shared_channel_sync`));
    }
}
class ControlSocialRuntimeRepairDerivedSnapshotApi {
    constructor(client) {
        this.client = client;
    }
    /** Repair the persisted social runtime derived snapshot. */
    async create() {
        return this.client.post(backendApiPath(`/control/social/runtime/repair_derived_snapshot`));
    }
}
class ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi {
    constructor(client) {
        this.client = client;
    }
    /** Release selected pending shared-channel sync entries. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/runtime/release_pending_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
    }
}
class ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi {
    constructor(client) {
        this.client = client;
    }
    /** Reclaim stale shared-channel sync pending ownership. */
    async create() {
        return this.client.post(backendApiPath(`/control/social/runtime/reclaim_stale_pending_shared_channel_sync`));
    }
}
class ControlSocialRuntimePendingSharedChannelSyncApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the pending shared-channel sync queue. */
    async list() {
        return this.client.get(backendApiPath(`/control/social/runtime/pending_shared_channel_sync`));
    }
}
class ControlSocialRuntimeDeliveryStateSharedChannelSyncApi {
    constructor(client) {
        this.client = client;
    }
    /** Read merged shared-channel sync delivery state. */
    async list() {
        return this.client.get(backendApiPath(`/control/social/runtime/delivery_state_shared_channel_sync`));
    }
}
class ControlSocialRuntimeDeliveredSharedChannelSyncApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the delivered shared-channel sync ledger. */
    async list() {
        return this.client.get(backendApiPath(`/control/social/runtime/delivered_shared_channel_sync`));
    }
}
class ControlSocialRuntimeDeadLetterSharedChannelSyncApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the dead-letter shared-channel sync queue. */
    async list() {
        return this.client.get(backendApiPath(`/control/social/runtime/dead_letter_shared_channel_sync`));
    }
}
class ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi {
    constructor(client) {
        this.client = client;
    }
    /** Claim selected pending shared-channel sync entries. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/runtime/claim_pending_shared_channel_sync_targeted`), body, undefined, undefined, 'application/json');
    }
}
class ControlSocialRuntimeApi {
    constructor(client) {
        this.client = client;
        this.claimPendingSharedChannelSyncTargeted = new ControlSocialRuntimeClaimPendingSharedChannelSyncTargetedApi(client);
        this.deadLetterSharedChannelSync = new ControlSocialRuntimeDeadLetterSharedChannelSyncApi(client);
        this.deliveredSharedChannelSync = new ControlSocialRuntimeDeliveredSharedChannelSyncApi(client);
        this.deliveryStateSharedChannelSync = new ControlSocialRuntimeDeliveryStateSharedChannelSyncApi(client);
        this.pendingSharedChannelSync = new ControlSocialRuntimePendingSharedChannelSyncApi(client);
        this.reclaimStalePendingSharedChannelSync = new ControlSocialRuntimeReclaimStalePendingSharedChannelSyncApi(client);
        this.releasePendingSharedChannelSyncTargeted = new ControlSocialRuntimeReleasePendingSharedChannelSyncTargetedApi(client);
        this.repairDerivedSnapshot = new ControlSocialRuntimeRepairDerivedSnapshotApi(client);
        this.repairSharedChannelSync = new ControlSocialRuntimeRepairSharedChannelSyncApi(client);
        this.republishPendingSharedChannelSyncTargeted = new ControlSocialRuntimeRepublishPendingSharedChannelSyncTargetedApi(client);
        this.requeueDeadLetterSharedChannelSync = new ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncApi(client);
        this.requeueDeadLetterSharedChannelSyncTargeted = new ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargetedApi(client);
        this.takeoverPendingSharedChannelSyncTargeted = new ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargetedApi(client);
    }
}
class ControlSocialFriendshipsApi {
    constructor(client) {
        this.client = client;
    }
    /** Activate a friendship event. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/friendships`), body, undefined, undefined, 'application/json');
    }
    /** Read a friendship snapshot. */
    async retrieve(friendshipId) {
        return this.client.get(backendApiPath(`/control/social/friendships/${serializePathParameter$1(friendshipId, { name: 'friendshipId', style: 'simple', explode: false })}`));
    }
    /** Remove a friendship. */
    async remove(friendshipId, body) {
        return this.client.post(backendApiPath(`/control/social/friendships/${serializePathParameter$1(friendshipId, { name: 'friendshipId', style: 'simple', explode: false })}/remove`), body, undefined, undefined, 'application/json');
    }
}
class ControlSocialFriendRequestsApi {
    constructor(client) {
        this.client = client;
    }
    /** Submit a friend request event. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/friend_requests`), body, undefined, undefined, 'application/json');
    }
    /** Read a friend request snapshot. */
    async retrieve(requestId) {
        return this.client.get(backendApiPath(`/control/social/friend_requests/${serializePathParameter$1(requestId, { name: 'requestId', style: 'simple', explode: false })}`));
    }
    /** Accept a friend request. */
    async accept(requestId, body) {
        return this.client.post(backendApiPath(`/control/social/friend_requests/${serializePathParameter$1(requestId, { name: 'requestId', style: 'simple', explode: false })}/accept`), body, undefined, undefined, 'application/json');
    }
    /** Decline a friend request. */
    async decline(requestId, body) {
        return this.client.post(backendApiPath(`/control/social/friend_requests/${serializePathParameter$1(requestId, { name: 'requestId', style: 'simple', explode: false })}/decline`), body, undefined, undefined, 'application/json');
    }
    /** Cancel a friend request. */
    async cancel(requestId, body) {
        return this.client.post(backendApiPath(`/control/social/friend_requests/${serializePathParameter$1(requestId, { name: 'requestId', style: 'simple', explode: false })}/cancel`), body, undefined, undefined, 'application/json');
    }
}
class ControlSocialExternalMemberLinksApi {
    constructor(client) {
        this.client = client;
    }
    /** Bind an external member link. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/external_member_links`), body, undefined, undefined, 'application/json');
    }
    /** Read an external member link snapshot. */
    async retrieve(linkId) {
        return this.client.get(backendApiPath(`/control/social/external_member_links/${serializePathParameter$1(linkId, { name: 'linkId', style: 'simple', explode: false })}`));
    }
}
class ControlSocialExternalConnectionsApi {
    constructor(client) {
        this.client = client;
    }
    /** Establish an external collaboration connection. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/external_connections`), body, undefined, undefined, 'application/json');
    }
    /** Read an external connection snapshot. */
    async retrieve(connectionId) {
        return this.client.get(backendApiPath(`/control/social/external_connections/${serializePathParameter$1(connectionId, { name: 'connectionId', style: 'simple', explode: false })}`));
    }
}
class ControlSocialDirectChatsBindingsApi {
    constructor(client) {
        this.client = client;
    }
    /** Bind a direct chat to a conversation. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/social/direct_chats/bindings`), body, undefined, undefined, 'application/json');
    }
}
class ControlSocialDirectChatsApi {
    constructor(client) {
        this.client = client;
        this.bindings = new ControlSocialDirectChatsBindingsApi(client);
    }
    /** Read a direct chat snapshot. */
    async retrieve(directChatId) {
        return this.client.get(backendApiPath(`/control/social/direct_chats/${serializePathParameter$1(directChatId, { name: 'directChatId', style: 'simple', explode: false })}`));
    }
}
class ControlSocialApi {
    constructor(client) {
        this.client = client;
        this.directChats = new ControlSocialDirectChatsApi(client);
        this.externalConnections = new ControlSocialExternalConnectionsApi(client);
        this.externalMemberLinks = new ControlSocialExternalMemberLinksApi(client);
        this.friendRequests = new ControlSocialFriendRequestsApi(client);
        this.friendships = new ControlSocialFriendshipsApi(client);
        this.runtime = new ControlSocialRuntimeApi(client);
        this.sharedChannelPolicies = new ControlSocialSharedChannelPoliciesApi(client);
        this.userBlocks = new ControlSocialUserBlocksApi(client);
    }
}
class ControlProviderBindingsApi {
    constructor(client) {
        this.client = client;
    }
    /** Read effective provider bindings. */
    async list(params) {
        const query = buildQueryString([
            { name: 'tenantId', value: params?.tenantId, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.get(appendQueryString(backendApiPath(`/control/provider_bindings`), query));
    }
    /** Upsert a provider binding policy. */
    async create(body) {
        return this.client.post(backendApiPath(`/control/provider_bindings`), body, undefined, undefined, 'application/json');
    }
}
class ControlProviderRegistryApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the provider registry snapshot. */
    async retrieve() {
        return this.client.get(backendApiPath(`/control/provider_registry`));
    }
}
class ControlProviderPoliciesDiffApi {
    constructor(client) {
        this.client = client;
    }
    /** Read provider policy diff between two versions. */
    async list(params) {
        const query = buildQueryString([
            { name: 'fromVersion', value: params.fromVersion, style: 'form', explode: true, allowReserved: false },
            { name: 'toVersion', value: params.toVersion, style: 'form', explode: true, allowReserved: false },
        ]);
        return this.client.get(appendQueryString(backendApiPath(`/control/provider_policies/diff`), query));
    }
}
class ControlProviderPoliciesApi {
    constructor(client) {
        this.client = client;
        this.diff = new ControlProviderPoliciesDiffApi(client);
    }
    /** Read provider policy history. */
    async list() {
        return this.client.get(backendApiPath(`/control/provider_policies`));
    }
    /** Preview the effective provider policy result before commit. */
    async preview(body) {
        return this.client.post(backendApiPath(`/control/provider_policies/preview`), body, undefined, undefined, 'application/json');
    }
    /** Rollback provider policy history to a target version. */
    async rollback(body) {
        return this.client.post(backendApiPath(`/control/provider_policies/rollback`), body, undefined, undefined, 'application/json');
    }
}
class ControlProtocolRegistryApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the control-plane protocol registry snapshot. */
    async retrieve() {
        return this.client.get(backendApiPath(`/control/protocol_registry`));
    }
}
class ControlProtocolGovernanceApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the control-plane protocol governance snapshot. */
    async retrieve() {
        return this.client.get(backendApiPath(`/control/protocol_governance`));
    }
}
class ControlNodesRoutesApi {
    constructor(client) {
        this.client = client;
    }
    /** Migrate owned routes from the source node to the target node. */
    async migrate(nodeId, body) {
        return this.client.post(backendApiPath(`/control/nodes/${serializePathParameter$1(nodeId, { name: 'nodeId', style: 'simple', explode: false })}/routes/migrate`), body, undefined, undefined, 'application/json');
    }
}
class ControlNodesApi {
    constructor(client) {
        this.client = client;
        this.routes = new ControlNodesRoutesApi(client);
    }
    /** Activate a realtime node and clear drain state. */
    async activate(nodeId) {
        return this.client.post(backendApiPath(`/control/nodes/${serializePathParameter$1(nodeId, { name: 'nodeId', style: 'simple', explode: false })}/activate`));
    }
    /** Mark a realtime node as draining. */
    async drain(nodeId) {
        return this.client.post(backendApiPath(`/control/nodes/${serializePathParameter$1(nodeId, { name: 'nodeId', style: 'simple', explode: false })}/drain`));
    }
}
class ControlApi {
    constructor(client) {
        this.client = client;
        this.nodes = new ControlNodesApi(client);
        this.protocolGovernance = new ControlProtocolGovernanceApi(client);
        this.protocolRegistry = new ControlProtocolRegistryApi(client);
        this.providerPolicies = new ControlProviderPoliciesApi(client);
        this.providerRegistry = new ControlProviderRegistryApi(client);
        this.providerBindings = new ControlProviderBindingsApi(client);
        this.social = new ControlSocialApi(client);
    }
}
function createControlApi(client) {
    return new ControlApi(client);
}
function appendQueryString(path, rawQueryString) {
    const query = rawQueryString.replace(/^\?+/, '');
    if (!query) {
        return path;
    }
    return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
function serializePathParameter$1(value, spec) {
    if (value === undefined || value === null) {
        return '';
    }
    const style = spec.style || 'simple';
    if (Array.isArray(value)) {
        return serializePathArray$1(spec.name, value, style, spec.explode);
    }
    if (typeof value === 'object') {
        return serializePathObject$1(spec.name, value, style, spec.explode);
    }
    return pathPrefix$1(spec.name, style) + encodePathValue$1(serializePathPrimitive$1(value));
}
function serializePathArray$1(name, values, style, explode) {
    const serialized = values
        .filter((item) => item !== undefined && item !== null)
        .map((item) => encodePathValue$1(serializePathPrimitive$1(item)));
    if (serialized.length === 0) {
        return pathPrefix$1(name, style);
    }
    if (style === 'matrix') {
        return explode
            ? serialized.map((item) => `;${name}=${item}`).join('')
            : `;${name}=${serialized.join(',')}`;
    }
    return pathPrefix$1(name, style) + serialized.join(explode ? '.' : ',');
}
function serializePathObject$1(name, value, style, explode) {
    const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
    if (entries.length === 0) {
        return pathPrefix$1(name, style);
    }
    if (style === 'matrix') {
        return explode
            ? entries.map(([key, entryValue]) => `;${encodePathValue$1(key)}=${encodePathValue$1(serializePathPrimitive$1(entryValue))}`).join('')
            : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue$1(key), encodePathValue$1(serializePathPrimitive$1(entryValue))]).join(',')}`;
    }
    const serialized = explode
        ? entries.map(([key, entryValue]) => `${encodePathValue$1(key)}=${encodePathValue$1(serializePathPrimitive$1(entryValue))}`).join(style === 'label' ? '.' : ',')
        : entries.flatMap(([key, entryValue]) => [encodePathValue$1(key), encodePathValue$1(serializePathPrimitive$1(entryValue))]).join(',');
    return pathPrefix$1(name, style) + serialized;
}
function pathPrefix$1(name, style, _objectValue) {
    if (style === 'label')
        return '.';
    if (style === 'matrix')
        return `;${name}`;
    return '';
}
function encodePathValue$1(value) {
    return encodeURIComponent(value);
}
function serializePathPrimitive$1(value) {
    if (value instanceof Date) {
        return value.toISOString();
    }
    if (typeof value === 'object') {
        return JSON.stringify(value);
    }
    return String(value);
}
function buildQueryString(parameters) {
    const pairs = [];
    for (const parameter of parameters) {
        appendSerializedParameter(pairs, parameter);
    }
    return pairs.join('&');
}
function appendSerializedParameter(pairs, parameter) {
    if (parameter.value === undefined || parameter.value === null) {
        return;
    }
    if (parameter.contentType) {
        pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
        return;
    }
    const style = parameter.style || 'form';
    if (style === 'deepObject') {
        appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
        return;
    }
    if (Array.isArray(parameter.value)) {
        appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
        return;
    }
    if (typeof parameter.value === 'object') {
        appendObjectParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
        return;
    }
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}
function appendArrayParameter(pairs, name, value, style, explode, allowReserved) {
    const values = value
        .filter((item) => item !== undefined && item !== null)
        .map((item) => serializePrimitive(item));
    if (values.length === 0) {
        return;
    }
    if (style === 'form' && explode) {
        for (const item of values) {
            pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
        }
        return;
    }
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}
function appendObjectParameter(pairs, name, value, style, explode, allowReserved) {
    const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
    if (entries.length === 0) {
        return;
    }
    if (style === 'form' && explode) {
        for (const [key, entryValue] of entries) {
            pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
        }
        return;
    }
    const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}
function appendDeepObjectParameter(pairs, name, value, allowReserved) {
    if (!value || typeof value !== 'object' || Array.isArray(value)) {
        pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
        return;
    }
    for (const [key, entryValue] of Object.entries(value)) {
        if (entryValue === undefined || entryValue === null) {
            continue;
        }
        pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
}
function serializePrimitive(value) {
    if (value instanceof Date) {
        return value.toISOString();
    }
    if (typeof value === 'object') {
        return JSON.stringify(value);
    }
    return String(value);
}
function encodeQueryComponent(value) {
    return encodeURIComponent(value);
}
function encodeQueryValue(value, allowReserved) {
    const encoded = encodeURIComponent(value);
    if (!allowReserved) {
        return encoded;
    }
    return encoded.replace(/%3A/gi, ':')
        .replace(/%2F/gi, '/')
        .replace(/%3F/gi, '?')
        .replace(/%23/gi, '#')
        .replace(/%5B/gi, '[')
        .replace(/%5D/gi, ']')
        .replace(/%40/gi, '@')
        .replace(/%21/gi, '!')
        .replace(/%24/gi, '$')
        .replace(/%26/gi, '&')
        .replace(/%27/gi, "'")
        .replace(/%28/gi, '(')
        .replace(/%29/gi, ')')
        .replace(/%2A/gi, '*')
        .replace(/%2B/gi, '+')
        .replace(/%2C/gi, ',')
        .replace(/%3B/gi, ';')
        .replace(/%3D/gi, '=');
}

class AdminUsageSummaryApi {
    constructor(client) {
        this.client = client;
    }
    /** getUsageSummary */
    async retrieve() {
        return this.client.get(backendApiPath(`/admin/usage/summary`));
    }
}
class AdminUsageRecordsApi {
    constructor(client) {
        this.client = client;
    }
    /** listUsageRecords */
    async list() {
        return this.client.get(backendApiPath(`/admin/usage/records`));
    }
}
class AdminUsageApi {
    constructor(client) {
        this.client = client;
        this.records = new AdminUsageRecordsApi(client);
        this.summary = new AdminUsageSummaryApi(client);
    }
}
class AdminStorageValidationTenantsApi {
    constructor(client) {
        this.client = client;
    }
    /** validateTenantStorageConfig */
    async create(tenantId, body) {
        return this.client.post(backendApiPath(`/admin/storage/validate/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
    }
}
class AdminStorageValidationApi {
    constructor(client) {
        this.client = client;
        this.tenants = new AdminStorageValidationTenantsApi(client);
    }
    /** validateGlobalStorageConfig */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/storage/validate`), body, undefined, undefined, 'application/json');
    }
}
class AdminStorageProvidersApi {
    constructor(client) {
        this.client = client;
    }
    /** listStorageProviders */
    async list() {
        return this.client.get(backendApiPath(`/admin/storage/providers`));
    }
}
class AdminStorageEffectiveTenantsApi {
    constructor(client) {
        this.client = client;
    }
    /** getTenantEffectiveStorageConfig */
    async retrieve(tenantId) {
        return this.client.get(backendApiPath(`/admin/storage/effective/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`));
    }
}
class AdminStorageEffectiveApi {
    constructor(client) {
        this.client = client;
        this.tenants = new AdminStorageEffectiveTenantsApi(client);
    }
}
class AdminStorageConfigTenantsApi {
    constructor(client) {
        this.client = client;
    }
    /** getTenantStorageConfig */
    async retrieve(tenantId) {
        return this.client.get(backendApiPath(`/admin/storage/config/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`));
    }
    /** saveTenantStorageConfig */
    async create(tenantId, body) {
        return this.client.post(backendApiPath(`/admin/storage/config/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
    }
    /** deleteTenantStorageConfig */
    async delete(tenantId) {
        return this.client.delete(backendApiPath(`/admin/storage/config/tenants/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}`));
    }
}
class AdminStorageConfigApi {
    constructor(client) {
        this.client = client;
        this.tenants = new AdminStorageConfigTenantsApi(client);
    }
    /** getGlobalStorageConfig */
    async retrieve() {
        return this.client.get(backendApiPath(`/admin/storage/config`));
    }
    /** saveGlobalStorageConfig */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/storage/config`), body, undefined, undefined, 'application/json');
    }
}
class AdminStorageAuditApi {
    constructor(client) {
        this.client = client;
    }
    /** listStorageAuditTrail */
    async list() {
        return this.client.get(backendApiPath(`/admin/storage/audit`));
    }
}
class AdminStorageApi {
    constructor(client) {
        this.client = client;
        this.audit = new AdminStorageAuditApi(client);
        this.config = new AdminStorageConfigApi(client);
        this.effective = new AdminStorageEffectiveApi(client);
        this.providers = new AdminStorageProvidersApi(client);
        this.validation = new AdminStorageValidationApi(client);
    }
}
class AdminRoutingSnapshotsApi {
    constructor(client) {
        this.client = client;
    }
    /** listCompiledRoutingSnapshots */
    async list() {
        return this.client.get(backendApiPath(`/admin/routing/snapshots`));
    }
}
class AdminRoutingProfilesApi {
    constructor(client) {
        this.client = client;
    }
    /** listRoutingProfiles */
    async list() {
        return this.client.get(backendApiPath(`/admin/routing/profiles`));
    }
    /** createRoutingProfile */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/routing/profiles`), body, undefined, undefined, 'application/json');
    }
}
class AdminRoutingHealthSnapshotsApi {
    constructor(client) {
        this.client = client;
    }
    /** listProviderHealthSnapshots */
    async retrieve() {
        return this.client.get(backendApiPath(`/admin/routing/health_snapshots`));
    }
}
class AdminRoutingDecisionLogsApi {
    constructor(client) {
        this.client = client;
    }
    /** listRoutingDecisionLogs */
    async list() {
        return this.client.get(backendApiPath(`/admin/routing/decision_logs`));
    }
}
class AdminRoutingApi {
    constructor(client) {
        this.client = client;
        this.decisionLogs = new AdminRoutingDecisionLogsApi(client);
        this.healthSnapshots = new AdminRoutingHealthSnapshotsApi(client);
        this.profiles = new AdminRoutingProfilesApi(client);
        this.snapshots = new AdminRoutingSnapshotsApi(client);
    }
}
class AdminProvidersApi {
    constructor(client) {
        this.client = client;
    }
    /** listProviders */
    async list() {
        return this.client.get(backendApiPath(`/admin/providers`));
    }
    /** saveProvider */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/providers`), body, undefined, undefined, 'application/json');
    }
    /** deleteProvider */
    async delete(providerId) {
        return this.client.delete(backendApiPath(`/admin/providers/${serializePathParameter(providerId, { name: 'providerId', style: 'simple', explode: false })}`));
    }
}
class AdminModelsProvidersApi {
    constructor(client) {
        this.client = client;
    }
    /** deleteModel */
    async delete(externalName, providerId) {
        return this.client.delete(backendApiPath(`/admin/models/${serializePathParameter(externalName, { name: 'externalName', style: 'simple', explode: false })}/providers/${serializePathParameter(providerId, { name: 'providerId', style: 'simple', explode: false })}`));
    }
}
class AdminModelsApi {
    constructor(client) {
        this.client = client;
        this.providers = new AdminModelsProvidersApi(client);
    }
    /** listModels */
    async list() {
        return this.client.get(backendApiPath(`/admin/models`));
    }
    /** saveModel */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/models`), body, undefined, undefined, 'application/json');
    }
}
class AdminModelPricesModelsProvidersApi {
    constructor(client) {
        this.client = client;
    }
    /** deleteModelPrice */
    async delete(channelId, modelId, proxyProviderId) {
        return this.client.delete(backendApiPath(`/admin/model_prices/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/models/${serializePathParameter(modelId, { name: 'modelId', style: 'simple', explode: false })}/providers/${serializePathParameter(proxyProviderId, { name: 'proxyProviderId', style: 'simple', explode: false })}`));
    }
}
class AdminModelPricesModelsApi {
    constructor(client) {
        this.client = client;
        this.providers = new AdminModelPricesModelsProvidersApi(client);
    }
}
class AdminModelPricesApi {
    constructor(client) {
        this.client = client;
        this.models = new AdminModelPricesModelsApi(client);
    }
    /** listModelPrices */
    async list() {
        return this.client.get(backendApiPath(`/admin/model_prices`));
    }
    /** saveModelPrice */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/model_prices`), body, undefined, undefined, 'application/json');
    }
}
class AdminMarketingCampaignsApi {
    constructor(client) {
        this.client = client;
    }
    /** listMarketingCampaigns */
    async list() {
        return this.client.get(backendApiPath(`/admin/marketing/campaigns`));
    }
    /** saveMarketingCampaign */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/marketing/campaigns`), body, undefined, undefined, 'application/json');
    }
    /** updateMarketingCampaignStatus */
    async status(marketingCampaignId, body) {
        return this.client.post(backendApiPath(`/admin/marketing/campaigns/${serializePathParameter(marketingCampaignId, { name: 'marketingCampaignId', style: 'simple', explode: false })}/status`), body, undefined, undefined, 'application/json');
    }
}
class AdminMarketingApi {
    constructor(client) {
        this.client = client;
        this.campaigns = new AdminMarketingCampaignsApi(client);
    }
}
class AdminGatewayRateLimitWindowsApi {
    constructor(client) {
        this.client = client;
    }
    /** listRateLimitWindows */
    async list() {
        return this.client.get(backendApiPath(`/admin/gateway/rate_limit_windows`));
    }
}
class AdminGatewayRateLimitPoliciesApi {
    constructor(client) {
        this.client = client;
    }
    /** listRateLimitPolicies */
    async list() {
        return this.client.get(backendApiPath(`/admin/gateway/rate_limit_policies`));
    }
    /** createRateLimitPolicy */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/gateway/rate_limit_policies`), body, undefined, undefined, 'application/json');
    }
}
class AdminGatewayApi {
    constructor(client) {
        this.client = client;
        this.rateLimitPolicies = new AdminGatewayRateLimitPoliciesApi(client);
        this.rateLimitWindows = new AdminGatewayRateLimitWindowsApi(client);
    }
}
class AdminExtensionsRuntimeStatusesApi {
    constructor(client) {
        this.client = client;
    }
    /** listRuntimeStatuses */
    async list() {
        return this.client.get(backendApiPath(`/admin/extensions/runtime_statuses`));
    }
}
class AdminExtensionsRuntimeReloadsApi {
    constructor(client) {
        this.client = client;
    }
    /** reloadExtensionRuntimes */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/extensions/runtime_reloads`), body, undefined, undefined, 'application/json');
    }
}
class AdminExtensionsApi {
    constructor(client) {
        this.client = client;
        this.runtimeReloads = new AdminExtensionsRuntimeReloadsApi(client);
        this.runtimeStatuses = new AdminExtensionsRuntimeStatusesApi(client);
    }
}
class AdminCredentialsProvidersKeysApi {
    constructor(client) {
        this.client = client;
    }
    /** deleteCredential */
    async delete(tenantId, providerId, keyReference) {
        return this.client.delete(backendApiPath(`/admin/credentials/${serializePathParameter(tenantId, { name: 'tenantId', style: 'simple', explode: false })}/providers/${serializePathParameter(providerId, { name: 'providerId', style: 'simple', explode: false })}/keys/${serializePathParameter(keyReference, { name: 'keyReference', style: 'simple', explode: false })}`));
    }
}
class AdminCredentialsProvidersApi {
    constructor(client) {
        this.client = client;
        this.keys = new AdminCredentialsProvidersKeysApi(client);
    }
}
class AdminCredentialsApi {
    constructor(client) {
        this.client = client;
        this.providers = new AdminCredentialsProvidersApi(client);
    }
    /** listCredentials */
    async list() {
        return this.client.get(backendApiPath(`/admin/credentials`));
    }
    /** saveCredential */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/credentials`), body, undefined, undefined, 'application/json');
    }
}
class AdminChannelsApi {
    constructor(client) {
        this.client = client;
    }
    /** listChannels */
    async list() {
        return this.client.get(backendApiPath(`/admin/channels`));
    }
    /** saveChannel */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/channels`), body, undefined, undefined, 'application/json');
    }
    /** deleteChannel */
    async delete(channelId) {
        return this.client.delete(backendApiPath(`/admin/channels/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}`));
    }
}
class AdminChannelModelsModelsApi {
    constructor(client) {
        this.client = client;
    }
    /** deleteChannelModel */
    async delete(channelId, modelId) {
        return this.client.delete(backendApiPath(`/admin/channel_models/${serializePathParameter(channelId, { name: 'channelId', style: 'simple', explode: false })}/models/${serializePathParameter(modelId, { name: 'modelId', style: 'simple', explode: false })}`));
    }
}
class AdminChannelModelsApi {
    constructor(client) {
        this.client = client;
        this.models = new AdminChannelModelsModelsApi(client);
    }
    /** listChannelModels */
    async list() {
        return this.client.get(backendApiPath(`/admin/channel_models`));
    }
    /** saveChannelModel */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/channel_models`), body, undefined, undefined, 'application/json');
    }
}
class AdminBillingSummaryApi {
    constructor(client) {
        this.client = client;
    }
    /** getBillingSummary */
    async retrieve() {
        return this.client.get(backendApiPath(`/admin/billing/summary`));
    }
}
class AdminBillingEventsSummaryApi {
    constructor(client) {
        this.client = client;
    }
    /** getBillingEventSummary */
    async retrieve() {
        return this.client.get(backendApiPath(`/admin/billing/events/summary`));
    }
}
class AdminBillingEventsApi {
    constructor(client) {
        this.client = client;
        this.summary = new AdminBillingEventsSummaryApi(client);
    }
    /** listBillingEvents */
    async list() {
        return this.client.get(backendApiPath(`/admin/billing/events`));
    }
}
class AdminBillingApi {
    constructor(client) {
        this.client = client;
        this.events = new AdminBillingEventsApi(client);
        this.summary = new AdminBillingSummaryApi(client);
    }
}
class AdminApiKeysApi {
    constructor(client) {
        this.client = client;
    }
    /** listApiKeys */
    async list() {
        return this.client.get(backendApiPath(`/admin/api_keys`));
    }
    /** createApiKey */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/api_keys`), body, undefined, undefined, 'application/json');
    }
    /** updateApiKey */
    async update(hashedKey, body) {
        return this.client.put(backendApiPath(`/admin/api_keys/${serializePathParameter(hashedKey, { name: 'hashedKey', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
    }
    /** deleteApiKey */
    async delete(hashedKey) {
        return this.client.delete(backendApiPath(`/admin/api_keys/${serializePathParameter(hashedKey, { name: 'hashedKey', style: 'simple', explode: false })}`));
    }
    /** updateApiKeyStatus */
    async status(hashedKey, body) {
        return this.client.post(backendApiPath(`/admin/api_keys/${serializePathParameter(hashedKey, { name: 'hashedKey', style: 'simple', explode: false })}/status`), body, undefined, undefined, 'application/json');
    }
}
class AdminApiKeyGroupsApi {
    constructor(client) {
        this.client = client;
    }
    /** listApiKeyGroups */
    async list() {
        return this.client.get(backendApiPath(`/admin/api_key_groups`));
    }
    /** createApiKeyGroup */
    async create(body) {
        return this.client.post(backendApiPath(`/admin/api_key_groups`), body, undefined, undefined, 'application/json');
    }
    /** updateApiKeyGroup */
    async update(groupId, body) {
        return this.client.patch(backendApiPath(`/admin/api_key_groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
    }
    /** deleteApiKeyGroup */
    async delete(groupId) {
        return this.client.delete(backendApiPath(`/admin/api_key_groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}`));
    }
    /** updateApiKeyGroupStatus */
    async status(groupId, body) {
        return this.client.post(backendApiPath(`/admin/api_key_groups/${serializePathParameter(groupId, { name: 'groupId', style: 'simple', explode: false })}/status`), body, undefined, undefined, 'application/json');
    }
}
class AdminApi {
    constructor(client) {
        this.client = client;
        this.apiKeyGroups = new AdminApiKeyGroupsApi(client);
        this.apiKeys = new AdminApiKeysApi(client);
        this.billing = new AdminBillingApi(client);
        this.channelModels = new AdminChannelModelsApi(client);
        this.channels = new AdminChannelsApi(client);
        this.credentials = new AdminCredentialsApi(client);
        this.extensions = new AdminExtensionsApi(client);
        this.gateway = new AdminGatewayApi(client);
        this.marketing = new AdminMarketingApi(client);
        this.modelPrices = new AdminModelPricesApi(client);
        this.models = new AdminModelsApi(client);
        this.providers = new AdminProvidersApi(client);
        this.routing = new AdminRoutingApi(client);
        this.storage = new AdminStorageApi(client);
        this.usage = new AdminUsageApi(client);
    }
}
function createAdminApi(client) {
    return new AdminApi(client);
}
function serializePathParameter(value, spec) {
    if (value === undefined || value === null) {
        return '';
    }
    const style = spec.style || 'simple';
    if (Array.isArray(value)) {
        return serializePathArray(spec.name, value, style, spec.explode);
    }
    if (typeof value === 'object') {
        return serializePathObject(spec.name, value, style, spec.explode);
    }
    return pathPrefix(spec.name, style) + encodePathValue(serializePathPrimitive(value));
}
function serializePathArray(name, values, style, explode) {
    const serialized = values
        .filter((item) => item !== undefined && item !== null)
        .map((item) => encodePathValue(serializePathPrimitive(item)));
    if (serialized.length === 0) {
        return pathPrefix(name, style);
    }
    if (style === 'matrix') {
        return explode
            ? serialized.map((item) => `;${name}=${item}`).join('')
            : `;${name}=${serialized.join(',')}`;
    }
    return pathPrefix(name, style) + serialized.join(explode ? '.' : ',');
}
function serializePathObject(name, value, style, explode) {
    const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
    if (entries.length === 0) {
        return pathPrefix(name, style);
    }
    if (style === 'matrix') {
        return explode
            ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
            : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
    }
    const serialized = explode
        ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
        : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
    return pathPrefix(name, style) + serialized;
}
function pathPrefix(name, style, _objectValue) {
    if (style === 'label')
        return '.';
    if (style === 'matrix')
        return `;${name}`;
    return '';
}
function encodePathValue(value) {
    return encodeURIComponent(value);
}
function serializePathPrimitive(value) {
    if (value instanceof Date) {
        return value.toISOString();
    }
    if (typeof value === 'object') {
        return JSON.stringify(value);
    }
    return String(value);
}

class SdkworkBackendClient {
    constructor(config) {
        this.httpClient = createHttpClient(config);
        this.ops = createOpsApi(this.httpClient);
        this.audit = createAuditApi(this.httpClient);
        this.automation = createAutomationApi(this.httpClient);
        this.control = createControlApi(this.httpClient);
        this.admin = createAdminApi(this.httpClient);
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
    async request(method, path, body, params, headers, contentType) {
        return this.http.request(`${this.basePath}${path}`, { method: method, body, params, headers, contentType });
    }
}

export { AdminApi, AuditApi, AutomationApi, BaseApi, ControlApi, HttpClient, OpsApi, SdkworkBackendClient, backendApiPath, createAdminApi, createAuditApi, createAutomationApi, createClient, createControlApi, createHttpClient, createOpsApi };
