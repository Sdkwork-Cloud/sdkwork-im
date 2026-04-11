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
function createHttpClient(config) {
    return new HttpClient(config);
}

const BACKEND_API_PREFIX = '/api/v1';
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

class SessionApi {
    constructor(client) {
        this.client = client;
    }
    /** Resume the current app session */
    async resume(body) {
        return this.client.post(backendApiPath(`/sessions/resume`), body, undefined, undefined, 'application/json');
    }
    /** Disconnect the current app session device route */
    async disconnect(body) {
        return this.client.post(backendApiPath(`/sessions/disconnect`), body, undefined, undefined, 'application/json');
    }
}
function createSessionApi(client) {
    return new SessionApi(client);
}

class PresenceApi {
    constructor(client) {
        this.client = client;
    }
    /** Refresh device presence */
    async heartbeat(body) {
        return this.client.post(backendApiPath(`/presence/heartbeat`), body, undefined, undefined, 'application/json');
    }
    /** Get current presence */
    async getPresenceMe() {
        return this.client.get(backendApiPath(`/presence/me`));
    }
}
function createPresenceApi(client) {
    return new PresenceApi(client);
}

class RealtimeApi {
    constructor(client) {
        this.client = client;
    }
    /** Replace realtime subscriptions for the current device */
    async syncRealtimeSubscriptions(body) {
        return this.client.post(backendApiPath(`/realtime/subscriptions/sync`), body, undefined, undefined, 'application/json');
    }
    /** Pull realtime events for the current device */
    async listRealtimeEvents(params) {
        return this.client.get(backendApiPath(`/realtime/events`), params);
    }
    /** Ack realtime events for the current device */
    async ackRealtimeEvents(body) {
        return this.client.post(backendApiPath(`/realtime/events/ack`), body, undefined, undefined, 'application/json');
    }
}
function createRealtimeApi(client) {
    return new RealtimeApi(client);
}

class DeviceApi {
    constructor(client) {
        this.client = client;
    }
    /** Register the current device */
    async register(body) {
        return this.client.post(backendApiPath(`/devices/register`), body, undefined, undefined, 'application/json');
    }
    /** Get device sync feed entries */
    async getDeviceSyncFeed(deviceId, params) {
        return this.client.get(backendApiPath(`/devices/${deviceId}/sync-feed`), params);
    }
}
function createDeviceApi(client) {
    return new DeviceApi(client);
}

class InboxApi {
    constructor(client) {
        this.client = client;
    }
    /** Get inbox entries */
    async getInbox() {
        return this.client.get(backendApiPath(`/inbox`));
    }
}
function createInboxApi(client) {
    return new InboxApi(client);
}

class ConversationApi {
    constructor(client) {
        this.client = client;
    }
    /** Create a conversation */
    async createConversation(body) {
        return this.client.post(backendApiPath(`/conversations`), body, undefined, undefined, 'application/json');
    }
    /** Create an agent dialog conversation */
    async createAgentDialog(body) {
        return this.client.post(backendApiPath(`/conversations/agent-dialogs`), body, undefined, undefined, 'application/json');
    }
    /** Create an agent handoff conversation */
    async createAgentHandoff(body) {
        return this.client.post(backendApiPath(`/conversations/agent-handoffs`), body, undefined, undefined, 'application/json');
    }
    /** Create a system channel conversation */
    async createSystemChannel(body) {
        return this.client.post(backendApiPath(`/conversations/system-channels`), body, undefined, undefined, 'application/json');
    }
    /** Get projected conversation summary */
    async getConversationSummary(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}`));
    }
    /** Get current agent handoff state */
    async getAgentHandoffState(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}/agent-handoff`));
    }
    /** Accept an agent handoff */
    async acceptAgentHandoff(conversationId) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/agent-handoff/accept`));
    }
    /** Resolve an accepted agent handoff */
    async resolveAgentHandoff(conversationId) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/agent-handoff/resolve`));
    }
    /** Close an agent handoff */
    async closeAgentHandoff(conversationId) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/agent-handoff/close`));
    }
    /** List members in a conversation */
    async listConversationMembers(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}/members`));
    }
    /** Add a member to a conversation */
    async addConversationMember(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/add`), body, undefined, undefined, 'application/json');
    }
    /** Remove a member from a conversation */
    async removeConversationMember(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/remove`), body, undefined, undefined, 'application/json');
    }
    /** Transfer conversation ownership */
    async transferConversationOwner(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/transfer-owner`), body, undefined, undefined, 'application/json');
    }
    /** Change a conversation member role */
    async changeConversationMemberRole(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/change-role`), body, undefined, undefined, 'application/json');
    }
    /** Leave a conversation */
    async leave(conversationId) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/members/leave`));
    }
    /** Get the current member read cursor */
    async getConversationReadCursor(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}/read-cursor`));
    }
    /** Update the current member read cursor */
    async updateConversationReadCursor(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/read-cursor`), body, undefined, undefined, 'application/json');
    }
    /** List projected conversation timeline entries */
    async listConversationMessages(conversationId) {
        return this.client.get(backendApiPath(`/conversations/${conversationId}/messages`));
    }
    /** Post a standard conversation message */
    async postConversationMessage(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/messages`), body, undefined, undefined, 'application/json');
    }
    /** Publish a message into a system channel conversation */
    async publishSystemChannelMessage(conversationId, body) {
        return this.client.post(backendApiPath(`/conversations/${conversationId}/system-channel/publish`), body, undefined, undefined, 'application/json');
    }
}
function createConversationApi(client) {
    return new ConversationApi(client);
}

class MessageApi {
    constructor(client) {
        this.client = client;
    }
    /** Edit a posted message */
    async edit(messageId, body) {
        return this.client.post(backendApiPath(`/messages/${messageId}/edit`), body, undefined, undefined, 'application/json');
    }
    /** Recall a posted message */
    async recall(messageId) {
        return this.client.post(backendApiPath(`/messages/${messageId}/recall`));
    }
}
function createMessageApi(client) {
    return new MessageApi(client);
}

class MediaApi {
    constructor(client) {
        this.client = client;
    }
    /** Create a media upload record */
    async createMediaUpload(body) {
        return this.client.post(backendApiPath(`/media/uploads`), body, undefined, undefined, 'application/json');
    }
    /** Complete a media upload */
    async completeMediaUpload(mediaAssetId, body) {
        return this.client.post(backendApiPath(`/media/uploads/${mediaAssetId}/complete`), body, undefined, undefined, 'application/json');
    }
    /** Issue a signed media download URL */
    async getMediaDownloadUrl(mediaAssetId, params) {
        return this.client.get(backendApiPath(`/media/${mediaAssetId}/download-url`), params);
    }
    /** Get a media asset by id */
    async getMediaAsset(mediaAssetId) {
        return this.client.get(backendApiPath(`/media/${mediaAssetId}`));
    }
    /** Attach a ready media asset as a conversation message */
    async attachMediaAsset(mediaAssetId, body) {
        return this.client.post(backendApiPath(`/media/${mediaAssetId}/attach`), body, undefined, undefined, 'application/json');
    }
}
function createMediaApi(client) {
    return new MediaApi(client);
}

class StreamApi {
    constructor(client) {
        this.client = client;
    }
    /** Open a stream session */
    async open(body) {
        return this.client.post(backendApiPath(`/streams`), body, undefined, undefined, 'application/json');
    }
    /** List stream frames */
    async listStreamFrames(streamId, params) {
        return this.client.get(backendApiPath(`/streams/${streamId}/frames`), params);
    }
    /** Append a frame to a stream */
    async appendStreamFrame(streamId, body) {
        return this.client.post(backendApiPath(`/streams/${streamId}/frames`), body, undefined, undefined, 'application/json');
    }
    /** Checkpoint a stream session */
    async checkpoint(streamId, body) {
        return this.client.post(backendApiPath(`/streams/${streamId}/checkpoint`), body, undefined, undefined, 'application/json');
    }
    /** Complete a stream session */
    async complete(streamId, body) {
        return this.client.post(backendApiPath(`/streams/${streamId}/complete`), body, undefined, undefined, 'application/json');
    }
    /** Abort a stream session */
    async abort(streamId, body) {
        return this.client.post(backendApiPath(`/streams/${streamId}/abort`), body, undefined, undefined, 'application/json');
    }
}
function createStreamApi(client) {
    return new StreamApi(client);
}

class RtcApi {
    constructor(client) {
        this.client = client;
    }
    /** Create an RTC session */
    async createRtcSession(body) {
        return this.client.post(backendApiPath(`/rtc/sessions`), body, undefined, undefined, 'application/json');
    }
    /** Invite participants into an RTC session */
    async inviteRtcSession(rtcSessionId, body) {
        return this.client.post(backendApiPath(`/rtc/sessions/${rtcSessionId}/invite`), body, undefined, undefined, 'application/json');
    }
    /** Accept an RTC session */
    async acceptRtcSession(rtcSessionId, body) {
        return this.client.post(backendApiPath(`/rtc/sessions/${rtcSessionId}/accept`), body, undefined, undefined, 'application/json');
    }
    /** Reject an RTC session */
    async rejectRtcSession(rtcSessionId, body) {
        return this.client.post(backendApiPath(`/rtc/sessions/${rtcSessionId}/reject`), body, undefined, undefined, 'application/json');
    }
    /** End an RTC session */
    async endRtcSession(rtcSessionId, body) {
        return this.client.post(backendApiPath(`/rtc/sessions/${rtcSessionId}/end`), body, undefined, undefined, 'application/json');
    }
    /** Post an RTC signaling event */
    async postRtcSignal(rtcSessionId, body) {
        return this.client.post(backendApiPath(`/rtc/sessions/${rtcSessionId}/signals`), body, undefined, undefined, 'application/json');
    }
    /** Issue an RTC participant credential */
    async issueRtcParticipantCredential(rtcSessionId, body) {
        return this.client.post(backendApiPath(`/rtc/sessions/${rtcSessionId}/credentials`), body, undefined, undefined, 'application/json');
    }
    /** Get the RTC recording artifact */
    async getRtcRecordingArtifact(rtcSessionId) {
        return this.client.get(backendApiPath(`/rtc/sessions/${rtcSessionId}/artifacts/recording`));
    }
}
function createRtcApi(client) {
    return new RtcApi(client);
}

class SdkworkBackendClient {
    constructor(config) {
        this.httpClient = createHttpClient(config);
        this.session = createSessionApi(this.httpClient);
        this.presence = createPresenceApi(this.httpClient);
        this.realtime = createRealtimeApi(this.httpClient);
        this.device = createDeviceApi(this.httpClient);
        this.inbox = createInboxApi(this.httpClient);
        this.conversation = createConversationApi(this.httpClient);
        this.message = createMessageApi(this.httpClient);
        this.media = createMediaApi(this.httpClient);
        this.stream = createStreamApi(this.httpClient);
        this.rtc = createRtcApi(this.httpClient);
    }
    setAuthToken(token) {
        this.httpClient.setAuthToken(token);
        return this;
    }
    setTokenManager(manager) {
        this.httpClient.setTokenManager(manager);
        return this;
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
Object.defineProperty(exports, "SUCCESS_CODES", {
    enumerable: true,
    get: function () { return sdkCommon.SUCCESS_CODES; }
});
exports.BaseApi = BaseApi;
exports.ConversationApi = ConversationApi;
exports.DeviceApi = DeviceApi;
exports.InboxApi = InboxApi;
exports.MediaApi = MediaApi;
exports.MessageApi = MessageApi;
exports.PresenceApi = PresenceApi;
exports.RealtimeApi = RealtimeApi;
exports.RtcApi = RtcApi;
exports.SdkworkBackendClient = SdkworkBackendClient;
exports.SessionApi = SessionApi;
exports.StreamApi = StreamApi;
exports.backendApiPath = backendApiPath;
exports.createClient = createClient;
exports.createConversationApi = createConversationApi;
exports.createDeviceApi = createDeviceApi;
exports.createInboxApi = createInboxApi;
exports.createMediaApi = createMediaApi;
exports.createMessageApi = createMessageApi;
exports.createPresenceApi = createPresenceApi;
exports.createRealtimeApi = createRealtimeApi;
exports.createRtcApi = createRtcApi;
exports.createSessionApi = createSessionApi;
exports.createStreamApi = createStreamApi;
//# sourceMappingURL=index.cjs.map
