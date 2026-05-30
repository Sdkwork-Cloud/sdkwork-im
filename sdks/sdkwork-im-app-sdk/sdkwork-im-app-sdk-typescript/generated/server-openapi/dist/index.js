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

const APP_API_PREFIX = '/app/v3/api';
function appApiPath(path) {
    if (!path) {
        return APP_API_PREFIX;
    }
    if (/^https?:\/\//i.test(path)) {
        return path;
    }
    const normalizedPrefixRaw = (APP_API_PREFIX).trim();
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

class AutomationExecutionsApi {
    constructor(client) {
        this.client = client;
    }
    /** Request an automation execution */
    async create(body) {
        return this.client.post(appApiPath(`/automation/executions`), body, undefined, undefined, 'application/json');
    }
    /** Get an automation execution */
    async retrieve(executionId) {
        return this.client.get(appApiPath(`/automation/executions/${serializePathParameter$2(executionId, { name: 'executionId', style: 'simple', explode: false })}`));
    }
}
class AutomationAgentToolCallsApi {
    constructor(client) {
        this.client = client;
    }
    /** Request an agent tool call */
    async create(body) {
        return this.client.post(appApiPath(`/automation/agent_tool_calls`), body, undefined, undefined, 'application/json');
    }
    /** Complete an agent tool call */
    async complete(executionId, toolCallId, body) {
        return this.client.post(appApiPath(`/automation/executions/${serializePathParameter$2(executionId, { name: 'executionId', style: 'simple', explode: false })}/agent_tool_calls/${serializePathParameter$2(toolCallId, { name: 'toolCallId', style: 'simple', explode: false })}/complete`), body, undefined, undefined, 'application/json');
    }
}
class AutomationAgentResponsesFramesApi {
    constructor(client) {
        this.client = client;
    }
    /** Append a frame to an agent response stream */
    async create(streamId, body) {
        return this.client.post(appApiPath(`/automation/agent_responses/${serializePathParameter$2(streamId, { name: 'streamId', style: 'simple', explode: false })}/frames`), body, undefined, undefined, 'application/json');
    }
}
class AutomationAgentResponsesApi {
    constructor(client) {
        this.client = client;
        this.frames = new AutomationAgentResponsesFramesApi(client);
    }
    /** Start an agent response stream */
    async create(body) {
        return this.client.post(appApiPath(`/automation/agent_responses`), body, undefined, undefined, 'application/json');
    }
    /** Complete an agent response stream */
    async complete(streamId, body) {
        return this.client.post(appApiPath(`/automation/agent_responses/${serializePathParameter$2(streamId, { name: 'streamId', style: 'simple', explode: false })}/complete`), body, undefined, undefined, 'application/json');
    }
}
class AutomationApi {
    constructor(client) {
        this.client = client;
        this.agentResponses = new AutomationAgentResponsesApi(client);
        this.agentToolCalls = new AutomationAgentToolCallsApi(client);
        this.executions = new AutomationExecutionsApi(client);
    }
}
function createAutomationApi(client) {
    return new AutomationApi(client);
}
function serializePathParameter$2(value, spec) {
    if (value === undefined || value === null) {
        return '';
    }
    const style = spec.style || 'simple';
    if (Array.isArray(value)) {
        return serializePathArray$2(spec.name, value, style, spec.explode);
    }
    if (typeof value === 'object') {
        return serializePathObject$2(spec.name, value, style, spec.explode);
    }
    return pathPrefix$2(spec.name, style) + encodePathValue$2(serializePathPrimitive$2(value));
}
function serializePathArray$2(name, values, style, explode) {
    const serialized = values
        .filter((item) => item !== undefined && item !== null)
        .map((item) => encodePathValue$2(serializePathPrimitive$2(item)));
    if (serialized.length === 0) {
        return pathPrefix$2(name, style);
    }
    if (style === 'matrix') {
        return explode
            ? serialized.map((item) => `;${name}=${item}`).join('')
            : `;${name}=${serialized.join(',')}`;
    }
    return pathPrefix$2(name, style) + serialized.join(explode ? '.' : ',');
}
function serializePathObject$2(name, value, style, explode) {
    const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
    if (entries.length === 0) {
        return pathPrefix$2(name, style);
    }
    if (style === 'matrix') {
        return explode
            ? entries.map(([key, entryValue]) => `;${encodePathValue$2(key)}=${encodePathValue$2(serializePathPrimitive$2(entryValue))}`).join('')
            : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue$2(key), encodePathValue$2(serializePathPrimitive$2(entryValue))]).join(',')}`;
    }
    const serialized = explode
        ? entries.map(([key, entryValue]) => `${encodePathValue$2(key)}=${encodePathValue$2(serializePathPrimitive$2(entryValue))}`).join(style === 'label' ? '.' : ',')
        : entries.flatMap(([key, entryValue]) => [encodePathValue$2(key), encodePathValue$2(serializePathPrimitive$2(entryValue))]).join(',');
    return pathPrefix$2(name, style) + serialized;
}
function pathPrefix$2(name, style, _objectValue) {
    if (style === 'label')
        return '.';
    if (style === 'matrix')
        return `;${name}`;
    return '';
}
function encodePathValue$2(value) {
    return encodeURIComponent(value);
}
function serializePathPrimitive$2(value) {
    if (value instanceof Date) {
        return value.toISOString();
    }
    if (typeof value === 'object') {
        return JSON.stringify(value);
    }
    return String(value);
}

class DeviceTwinReportedApi {
    constructor(client) {
        this.client = client;
    }
    /** Update the reported state for a device twin */
    async create(deviceId, body) {
        return this.client.post(appApiPath(`/devices/${serializePathParameter$1(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/twin/reported`), body, undefined, undefined, 'application/json');
    }
}
class DeviceTwinDesiredApi {
    constructor(client) {
        this.client = client;
    }
    /** Update the desired state for a device twin */
    async create(deviceId, body) {
        return this.client.post(appApiPath(`/devices/${serializePathParameter$1(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/twin/desired`), body, undefined, undefined, 'application/json');
    }
}
class DeviceTwinApi {
    constructor(client) {
        this.client = client;
        this.desired = new DeviceTwinDesiredApi(client);
        this.reported = new DeviceTwinReportedApi(client);
    }
    /** Get the device twin */
    async list(deviceId) {
        return this.client.get(appApiPath(`/devices/${serializePathParameter$1(deviceId, { name: 'deviceId', style: 'simple', explode: false })}/twin`));
    }
}
class DeviceApi {
    constructor(client) {
        this.client = client;
        this.twin = new DeviceTwinApi(client);
    }
}
function createDeviceApi(client) {
    return new DeviceApi(client);
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

class NotificationRequestsApi {
    constructor(client) {
        this.client = client;
    }
    /** Request a notification task */
    async create(body) {
        return this.client.post(appApiPath(`/notifications/requests`), body, undefined, undefined, 'application/json');
    }
}
class NotificationApi {
    constructor(client) {
        this.client = client;
        this.requests = new NotificationRequestsApi(client);
    }
    /** List notifications for the current principal */
    async list() {
        return this.client.get(appApiPath(`/notifications`));
    }
    /** Get a notification task */
    async retrieve(notificationId) {
        return this.client.get(appApiPath(`/notifications/${serializePathParameter(notificationId, { name: 'notificationId'})}`));
    }
}
function createNotificationApi(client) {
    return new NotificationApi(client);
}
function serializePathParameter(value, spec) {
    if (value === undefined || value === null) {
        return '';
    }
    if (Array.isArray(value)) {
        return serializePathArray(spec.name, value);
    }
    if (typeof value === 'object') {
        return serializePathObject(spec.name, value);
    }
    return pathPrefix() + encodePathValue(serializePathPrimitive(value));
}
function serializePathArray(name, values, style, explode) {
    const serialized = values
        .filter((item) => item !== undefined && item !== null)
        .map((item) => encodePathValue(serializePathPrimitive(item)));
    if (serialized.length === 0) {
        return pathPrefix();
    }
    return pathPrefix() + serialized.join(',');
}
function serializePathObject(name, value, style, explode) {
    const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
    if (entries.length === 0) {
        return pathPrefix();
    }
    const serialized = entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
    return pathPrefix() + serialized;
}
function pathPrefix(name, style, _objectValue) {
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

class PortalWorkspaceApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the current tenant workspace snapshot */
    async retrieve() {
        return this.client.get(appApiPath(`/portal/workspace`));
    }
}
class PortalRealtimeApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the tenant realtime snapshot */
    async retrieve() {
        return this.client.get(appApiPath(`/portal/realtime`));
    }
}
class PortalMediaApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the tenant media snapshot */
    async retrieve() {
        return this.client.get(appApiPath(`/portal/media`));
    }
}
class PortalHomeApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the tenant portal home snapshot */
    async retrieve() {
        return this.client.get(appApiPath(`/portal/home`));
    }
}
class PortalGovernanceApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the tenant governance snapshot */
    async retrieve() {
        return this.client.get(appApiPath(`/portal/governance`));
    }
}
class PortalDashboardApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the tenant dashboard snapshot */
    async retrieve() {
        return this.client.get(appApiPath(`/portal/dashboard`));
    }
}
class PortalConversationSnapshotApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the tenant conversations snapshot */
    async retrieve() {
        return this.client.get(appApiPath(`/portal/conversations`));
    }
}
class PortalAutomationApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the tenant automation snapshot */
    async retrieve() {
        return this.client.get(appApiPath(`/portal/automation`));
    }
}
class PortalAccessApi {
    constructor(client) {
        this.client = client;
    }
    /** Read the tenant portal sign-in snapshot */
    async retrieve() {
        return this.client.get(appApiPath(`/portal/access`));
    }
}
class PortalApi {
    constructor(client) {
        this.client = client;
        this.access = new PortalAccessApi(client);
        this.automation = new PortalAutomationApi(client);
        this.conversationSnapshot = new PortalConversationSnapshotApi(client);
        this.dashboard = new PortalDashboardApi(client);
        this.governance = new PortalGovernanceApi(client);
        this.home = new PortalHomeApi(client);
        this.media = new PortalMediaApi(client);
        this.realtime = new PortalRealtimeApi(client);
        this.workspace = new PortalWorkspaceApi(client);
    }
}
function createPortalApi(client) {
    return new PortalApi(client);
}

class ProviderPrincipalProfileHealthApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve principal-profile provider health */
    async retrieve() {
        return this.client.get(appApiPath(`/principal/profiles/provider_health`));
    }
}
class ProviderMediaHealthApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve media provider health */
    async retrieve() {
        return this.client.get(appApiPath(`/media/provider_health`));
    }
}
class ProviderApi {
    constructor(client) {
        this.client = client;
        this.mediaHealth = new ProviderMediaHealthApi(client);
        this.principalProfileHealth = new ProviderPrincipalProfileHealthApi(client);
    }
}
function createProviderApi(client) {
    return new ProviderApi(client);
}

class IotProtocolDownlinkApi {
    constructor(client) {
        this.client = client;
    }
    /** Ingest IoT protocol downlink */
    async create() {
        return this.client.post(appApiPath(`/iot/protocol/downlink`));
    }
}
class IotProtocolUplinkApi {
    constructor(client) {
        this.client = client;
    }
    /** Ingest IoT protocol uplink */
    async create() {
        return this.client.post(appApiPath(`/iot/protocol/uplink`));
    }
}
class IotProtocolApi {
    constructor(client) {
        this.client = client;
        this.uplink = new IotProtocolUplinkApi(client);
        this.downlink = new IotProtocolDownlinkApi(client);
    }
}
class IotProtocolProviderHealthApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve IoT protocol provider health */
    async retrieve() {
        return this.client.get(appApiPath(`/iot/protocol/provider_health`));
    }
}
class IotAccessProviderHealthApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve IoT access provider health */
    async retrieve() {
        return this.client.get(appApiPath(`/iot/access/provider_health`));
    }
}
class IotApi {
    constructor(client) {
        this.client = client;
        this.accessProviderHealth = new IotAccessProviderHealthApi(client);
        this.protocolProviderHealth = new IotProtocolProviderHealthApi(client);
        this.protocol = new IotProtocolApi(client);
    }
}
function createIotApi(client) {
    return new IotApi(client);
}

class RtcProviderHealthApi {
    constructor(client) {
        this.client = client;
    }
    /** Retrieve RTC provider health */
    async retrieve() {
        return this.client.get(appApiPath(`/rtc/provider_health`));
    }
}
class RtcProviderCallbacksApi {
    constructor(client) {
        this.client = client;
    }
    /** Map RTC provider callback */
    async create() {
        return this.client.post(appApiPath(`/rtc/provider_callbacks`));
    }
}
class RtcApi {
    constructor(client) {
        this.client = client;
        this.providerCallbacks = new RtcProviderCallbacksApi(client);
        this.providerHealth = new RtcProviderHealthApi(client);
    }
}
function createRtcApi(client) {
    return new RtcApi(client);
}

class SdkworkAppClient {
    constructor(config) {
        this.httpClient = createHttpClient(config);
        this.automation = createAutomationApi(this.httpClient);
        this.device = createDeviceApi(this.httpClient);
        this.notification = createNotificationApi(this.httpClient);
        this.portal = createPortalApi(this.httpClient);
        this.provider = createProviderApi(this.httpClient);
        this.iot = createIotApi(this.httpClient);
        this.rtc = createRtcApi(this.httpClient);
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
    return new SdkworkAppClient(config);
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

export { AutomationApi, BaseApi, DeviceApi, HttpClient, IotApi, NotificationApi, PortalApi, ProviderApi, RtcApi, SdkworkAppClient, appApiPath, createAutomationApi, createClient, createDeviceApi, createHttpClient, createIotApi, createNotificationApi, createPortalApi, createProviderApi, createRtcApi };
