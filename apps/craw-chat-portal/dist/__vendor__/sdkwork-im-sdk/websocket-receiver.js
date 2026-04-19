import { ImSdkError } from './errors.js';
import { ImReceiver } from './receiver.js';
const CCP_WEBSOCKET_SUBPROTOCOL = 'ccp/ws/1';
const DEFAULT_REQUEST_TIMEOUT_MS = 15_000;
export class ImWebSocketReceiver {
    realtime;
    context;
    receiver;
    openHandlers = new Set();
    closeHandlers = new Set();
    socketErrorHandlers = new Set();
    connectedHandlers = new Set();
    windowHandlers = new Set();
    syncedHandlers = new Set();
    ackedHandlers = new Set();
    realtimeErrorHandlers = new Set();
    pendingRequests = new Map();
    options;
    socket;
    cleanupSocketListeners = [];
    connectPromise;
    isConnected = false;
    requestSequence = 0;
    connectResolve;
    connectReject;
    ccpSessionResumeEnabled = false;
    constructor(realtime, context, options = {}) {
        this.realtime = realtime;
        this.context = context;
        this.receiver = new ImReceiver(realtime);
        this.options = {
            ...options,
            mode: options.mode ?? 'legacy_json',
            requestTimeoutMs: options.requestTimeoutMs ?? DEFAULT_REQUEST_TIMEOUT_MS,
        };
    }
    onOpen(handler) {
        this.openHandlers.add(handler);
        return () => {
            this.openHandlers.delete(handler);
        };
    }
    onClose(handler) {
        this.closeHandlers.add(handler);
        return () => {
            this.closeHandlers.delete(handler);
        };
    }
    onSocketError(handler) {
        this.socketErrorHandlers.add(handler);
        return () => {
            this.socketErrorHandlers.delete(handler);
        };
    }
    onConnected(handler) {
        this.connectedHandlers.add(handler);
        return () => {
            this.connectedHandlers.delete(handler);
        };
    }
    onWindow(handler) {
        this.windowHandlers.add(handler);
        return () => {
            this.windowHandlers.delete(handler);
        };
    }
    onSubscriptionsSynced(handler) {
        this.syncedHandlers.add(handler);
        return () => {
            this.syncedHandlers.delete(handler);
        };
    }
    onAcked(handler) {
        this.ackedHandlers.add(handler);
        return () => {
            this.ackedHandlers.delete(handler);
        };
    }
    onRealtimeError(handler) {
        this.realtimeErrorHandlers.add(handler);
        return () => {
            this.realtimeErrorHandlers.delete(handler);
        };
    }
    onEvent(handler) {
        return this.receiver.onEvent(handler);
    }
    onMessageEvent(handler) {
        return this.receiver.onMessageEvent(handler);
    }
    onDataEvent(handler) {
        return this.receiver.onDataEvent(handler);
    }
    onRtcSignalEvent(handler) {
        return this.receiver.onRtcSignalEvent(handler);
    }
    onScope(scopeType, scopeId, handler) {
        return this.receiver.onScope(scopeType, scopeId, handler);
    }
    async connect() {
        if (this.isConnected) {
            return this;
        }
        if (this.connectPromise) {
            return this.connectPromise;
        }
        this.connectPromise = new Promise(async (resolve, reject) => {
            this.connectResolve = resolve;
            this.connectReject = reject;
            try {
                this.socket = await this.resolveSocket();
                this.attachSocket(this.socket);
                if (this.socket.readyState === 1) {
                    void this.handleSocketOpen({ type: 'open' });
                }
            }
            catch (error) {
                this.rejectConnect(error);
            }
        });
        return this.connectPromise;
    }
    close(code, reason) {
        this.socket?.close(code, reason);
        this.cleanupPendingRequests(new ImSdkError('websocket_closed', 'Realtime connection closed before the pending operation completed.'));
        this.resetSocketState();
    }
    async syncSubscriptions(body) {
        const requestId = this.createRequestId('subscriptions_sync');
        const items = Array.isArray(body) ? body : body.items;
        const payload = Array.isArray(body)
            ? {
                type: 'subscriptions.sync',
                requestId,
                items,
            }
            : {
                type: 'subscriptions.sync',
                requestId,
                items,
            };
        const responsePromise = this.createPendingRequest(requestId, 'sync');
        await this.sendBusinessFrame(payload, 'cmd', 'cc.realtime.subscriptions.sync.v1');
        return responsePromise;
    }
    async pullWindow(params = {}) {
        const requestId = this.createRequestId('events_pull');
        const payload = {
            type: 'events.pull',
            requestId,
            afterSeq: params.afterSeq,
            limit: params.limit,
        };
        const responsePromise = this.createPendingRequest(requestId, 'pull');
        await this.sendBusinessFrame(payload, 'cmd', 'cc.realtime.events.pull.v1');
        return responsePromise;
    }
    async ackWindow(batchOrSeq) {
        const requestId = this.createRequestId('events_ack');
        const ackedSeq = typeof batchOrSeq === 'number' ? batchOrSeq : batchOrSeq.highestSeq;
        const payload = {
            type: 'events.ack',
            requestId,
            ackedSeq,
        };
        const responsePromise = this.createPendingRequest(requestId, 'ack');
        await this.sendBusinessFrame(payload, 'ack', 'cc.realtime.events.ack.v1');
        return responsePromise;
    }
    async resolveSocket() {
        if (this.options.socket) {
            return this.options.socket;
        }
        const url = this.options.url ?? this.context.resolveRealtimeWebSocketUrl();
        if (!url) {
            throw new ImSdkError('websocket_url_required', 'websocketBaseUrl or connect({ url }) is required to establish realtime connectivity.');
        }
        const protocols = this.options.protocols
            ?? (this.options.mode === 'ccp_json' ? [CCP_WEBSOCKET_SUBPROTOCOL] : []);
        const authToken = normalizeAuthToken(this.options.authToken);
        const headers = {
            ...(this.options.headers ?? {}),
        };
        if (authToken) {
            headers.Authorization = `Bearer ${authToken}`;
        }
        const request = {
            url,
            protocols,
            headers,
            authToken,
        };
        if (this.options.createSocket) {
            return this.options.createSocket(request);
        }
        return createDefaultSocket(request);
    }
    attachSocket(socket) {
        this.cleanupSocketListeners = [
            attachSocketListener(socket, 'open', (event) => {
                void this.handleSocketOpen(event);
            }),
            attachSocketListener(socket, 'message', (event) => {
                void this.handleSocketMessage(event);
            }),
            attachSocketListener(socket, 'close', (event) => {
                this.handleSocketClose(event);
            }),
            attachSocketListener(socket, 'error', (event) => {
                this.handleSocketError(event);
            }),
        ];
    }
    async handleSocketOpen(event) {
        for (const handler of this.openHandlers) {
            handler(event);
        }
        if (this.options.mode !== 'ccp_json') {
            return;
        }
        const ccp = requireCcpOptions(this.options.ccp);
        this.ccpSessionResumeEnabled = false;
        await this.sendCcpControlFrame('cc.control.hello.v1', {
            type: 'hello',
            data: {
                protocol: {
                    family: 'ccp',
                    major: 1,
                    minor: 0,
                },
                binding: 'Ws1',
                capabilities: {
                    items: ccp.capabilities ?? ['payload.json'],
                },
                trace_id: ccp.traceId ?? null,
            },
        });
    }
    async handleSocketMessage(event) {
        try {
            const message = await resolveSocketMessageData(event);
            if (message == null) {
                return;
            }
            const parsed = JSON.parse(message);
            const businessFrame = this.unwrapBusinessFrame(parsed);
            if (!businessFrame) {
                return;
            }
            if (businessFrame.type === 'realtime.connected') {
                const connectedFrame = businessFrame;
                this.isConnected = true;
                for (const handler of this.connectedHandlers) {
                    handler(connectedFrame);
                }
                this.resolveConnect();
                return;
            }
            if (businessFrame.type === 'subscriptions.synced') {
                const frame = businessFrame;
                for (const handler of this.syncedHandlers) {
                    handler(frame);
                }
                this.resolvePendingRequest(frame.requestId ?? undefined, frame.snapshot);
                return;
            }
            if (businessFrame.type === 'events.acked') {
                const frame = businessFrame;
                for (const handler of this.ackedHandlers) {
                    handler(frame);
                }
                this.resolvePendingRequest(frame.requestId ?? undefined, frame.ack);
                return;
            }
            if (businessFrame.type === 'event.window') {
                const window = businessFrame;
                const batch = toReceiverBatch(this.receiver, window.window);
                const frame = {
                    ...window,
                    batch,
                };
                for (const handler of this.windowHandlers) {
                    handler(frame);
                }
                this.resolvePendingRequest(frame.requestId ?? undefined, frame);
                return;
            }
            if (businessFrame.type === 'error') {
                const frame = businessFrame;
                const error = new ImSdkError('websocket_protocol_error', frame.message, {
                    remoteCode: frame.code,
                    requestId: frame.requestId ?? undefined,
                });
                for (const handler of this.realtimeErrorHandlers) {
                    handler(frame);
                }
                if (frame.requestId) {
                    this.rejectPendingRequest(frame.requestId, error);
                }
                else {
                    this.rejectConnect(error);
                }
            }
        }
        catch (error) {
            this.handleSocketError(error);
        }
    }
    handleSocketClose(event) {
        for (const handler of this.closeHandlers) {
            handler(event);
        }
        this.cleanupPendingRequests(new ImSdkError('websocket_closed', 'Realtime websocket closed before the pending operation completed.'));
        if (!this.isConnected) {
            this.rejectConnect(new ImSdkError('websocket_closed', 'Realtime websocket closed before the connected frame arrived.'));
        }
        this.resetSocketState();
    }
    handleSocketError(event) {
        for (const handler of this.socketErrorHandlers) {
            handler(event);
        }
        if (!this.isConnected) {
            this.rejectConnect(event instanceof Error
                ? event
                : new ImSdkError('websocket_transport_error', 'Realtime websocket failed before the connection became ready.'));
        }
    }
    unwrapBusinessFrame(parsed) {
        if (looksLikeCcpEnvelope(parsed)) {
            if (parsed.kind === 'control') {
                const frame = JSON.parse(parsed.payload);
                void this.handleCcpControlFrame(frame);
                return undefined;
            }
            return JSON.parse(parsed.payload);
        }
        return parsed && typeof parsed === 'object'
            ? parsed
            : undefined;
    }
    async handleCcpControlFrame(frame) {
        if (frame.type === 'hello_ack') {
            if (!frame.data.accepted) {
                this.rejectConnect(new ImSdkError('websocket_protocol_error', 'Realtime CCP hello negotiation was rejected by the server.'));
                return;
            }
            this.ccpSessionResumeEnabled = Boolean(frame.data.capabilities?.items?.includes('session.resume'));
            const ccp = requireCcpOptions(this.options.ccp);
            await this.sendCcpControlFrame('cc.control.auth_bind.v1', {
                type: 'auth_bind',
                data: {
                    principal_id: ccp.principalId,
                    actor_kind: ccp.actorKind,
                    device_id: ccp.deviceId ?? null,
                    session_id: ccp.sessionId ?? null,
                },
            });
            return;
        }
        if (frame.type === 'auth_ok') {
            const ccp = requireCcpOptions(this.options.ccp);
            if (this.ccpSessionResumeEnabled && ccp.sessionId) {
                await this.sendCcpControlFrame('cc.control.session_resume.v1', {
                    type: 'session_resume',
                    data: {
                        session_id: ccp.sessionId,
                        last_acked_seq: ccp.lastAckedSeq ?? 0,
                    },
                });
            }
            return;
        }
        if (frame.type === 'session_resumed' || frame.type === 'heartbeat') {
            return;
        }
        if (frame.type === 'goaway' || frame.type === 'error') {
            const error = new ImSdkError('websocket_protocol_error', frame.data.message, {
                code: frame.data.code,
            });
            this.rejectConnect(error);
            this.cleanupPendingRequests(error);
            this.close();
        }
    }
    async sendBusinessFrame(payload, ccpKind, ccpSchema) {
        const socket = this.requireSocket();
        if (this.options.mode === 'ccp_json') {
            await this.sendJson(socket, buildCcpEnvelope(ccpKind, ccpSchema, payload));
            return;
        }
        await this.sendJson(socket, payload);
    }
    async sendCcpControlFrame(schema, payload) {
        await this.sendJson(this.requireSocket(), buildCcpEnvelope('control', schema, payload));
    }
    async sendJson(socket, payload) {
        await Promise.resolve(socket.send(JSON.stringify(payload)));
    }
    requireSocket() {
        if (!this.socket) {
            throw new ImSdkError('websocket_not_connected', 'Realtime connection is not connected. Call connect() first.');
        }
        return this.socket;
    }
    createPendingRequest(requestId, kind) {
        return new Promise((resolve, reject) => {
            const timeout = setTimeout(() => {
                this.pendingRequests.delete(requestId);
                reject(new ImSdkError('websocket_request_timeout', `Realtime websocket request ${requestId} timed out.`, {
                    requestId,
                    kind,
                }));
            }, this.options.requestTimeoutMs);
            this.pendingRequests.set(requestId, {
                kind,
                resolve: resolve,
                reject,
                timeout,
            });
        });
    }
    resolvePendingRequest(requestId, value) {
        if (!requestId) {
            return;
        }
        const pending = this.pendingRequests.get(requestId);
        if (!pending) {
            return;
        }
        clearTimeout(pending.timeout);
        this.pendingRequests.delete(requestId);
        pending.resolve(value);
    }
    rejectPendingRequest(requestId, reason) {
        const pending = this.pendingRequests.get(requestId);
        if (!pending) {
            return;
        }
        clearTimeout(pending.timeout);
        this.pendingRequests.delete(requestId);
        pending.reject(reason);
    }
    cleanupPendingRequests(reason) {
        for (const [requestId, pending] of this.pendingRequests) {
            clearTimeout(pending.timeout);
            pending.reject(reason);
            this.pendingRequests.delete(requestId);
        }
    }
    createRequestId(prefix) {
        this.requestSequence += 1;
        return `${prefix}_${this.requestSequence}`;
    }
    resolveConnect() {
        const resolve = this.connectResolve;
        this.connectResolve = undefined;
        this.connectReject = undefined;
        resolve?.(this);
    }
    rejectConnect(reason) {
        const reject = this.connectReject;
        this.connectResolve = undefined;
        this.connectReject = undefined;
        this.connectPromise = undefined;
        reject?.(reason);
    }
    resetSocketState() {
        for (const cleanup of this.cleanupSocketListeners) {
            cleanup();
        }
        this.cleanupSocketListeners = [];
        this.socket = undefined;
        this.isConnected = false;
        this.connectPromise = undefined;
        this.ccpSessionResumeEnabled = false;
    }
}
function toReceiverBatch(receiver, rawWindow) {
    const items = rawWindow.items.map((item) => receiver.dispatchRealtimeEvent(item));
    const highestSeq = items.reduce((currentMax, item) => Math.max(currentMax, item.realtimeSeq), rawWindow.ackedThroughSeq ?? 0);
    return {
        items,
        highestSeq,
        rawWindow,
    };
}
function normalizeAuthToken(token) {
    if (!token) {
        return undefined;
    }
    return token.startsWith('Bearer ') ? token.slice('Bearer '.length) : token;
}
function createDefaultSocket(request) {
    const WebSocketConstructor = globalThis.WebSocket;
    if (typeof WebSocketConstructor !== 'function') {
        throw new ImSdkError('websocket_factory_required', 'No global WebSocket implementation is available. Provide webSocketFactory to establish realtime connectivity in this runtime.');
    }
    if (Object.keys(request.headers).length > 0) {
        throw new ImSdkError('websocket_factory_required', 'The default WebSocket implementation cannot attach Authorization headers. Provide webSocketFactory for authenticated realtime connections.');
    }
    return new WebSocketConstructor(request.url, request.protocols);
}
function requireCcpOptions(options) {
    if (options) {
        return options;
    }
    throw new ImSdkError('ccp_auth_required', 'CCP websocket mode requires ccp.principalId and ccp.actorKind so auth_bind can be negotiated.');
}
function looksLikeCcpEnvelope(value) {
    if (!value || typeof value !== 'object') {
        return false;
    }
    const candidate = value;
    return (typeof candidate.protocol === 'object'
        && typeof candidate.kind === 'string'
        && typeof candidate.schema === 'string'
        && typeof candidate.payload === 'string');
}
function buildCcpEnvelope(kind, schema, payload) {
    return {
        protocol: {
            family: 'ccp',
            major: 1,
            minor: 0,
        },
        binding: 'Ws1',
        kind,
        schema,
        scope: null,
        route: null,
        flags: [],
        traceId: null,
        payload: JSON.stringify(payload),
    };
}
async function resolveSocketMessageData(event) {
    if (typeof event === 'string') {
        return event;
    }
    if (event instanceof ArrayBuffer) {
        return new TextDecoder().decode(event);
    }
    if (ArrayBuffer.isView(event)) {
        return new TextDecoder().decode(event);
    }
    const data = event?.data;
    if (typeof data === 'string') {
        return data;
    }
    if (data instanceof ArrayBuffer) {
        return new TextDecoder().decode(data);
    }
    if (ArrayBuffer.isView(data)) {
        return new TextDecoder().decode(data);
    }
    if (typeof Blob !== 'undefined' && data instanceof Blob) {
        return data.text();
    }
    if (data && typeof data === 'object') {
        return JSON.stringify(data);
    }
    return undefined;
}
function attachSocketListener(socket, type, listener) {
    if (socket.addEventListener && socket.removeEventListener) {
        socket.addEventListener(type, listener);
        return () => {
            socket.removeEventListener?.(type, listener);
        };
    }
    if (socket.on && socket.off) {
        const wrappedListener = (...args) => {
            if (type === 'message') {
                listener({ data: args[0] });
                return;
            }
            if (type === 'close') {
                listener({
                    code: args[0],
                    reason: typeof args[1] === 'string'
                        ? args[1]
                        : decodeUnknownText(args[1]),
                });
                return;
            }
            listener(args[0]);
        };
        socket.on(type, wrappedListener);
        return () => {
            socket.off?.(type, wrappedListener);
        };
    }
    const propertyName = `on${type}`;
    const previous = socket[propertyName];
    socket[propertyName] = listener;
    return () => {
        socket[propertyName] = previous;
    };
}
function decodeUnknownText(value) {
    if (typeof value === 'string') {
        return value;
    }
    if (value instanceof ArrayBuffer) {
        return new TextDecoder().decode(value);
    }
    if (ArrayBuffer.isView(value)) {
        return new TextDecoder().decode(value);
    }
    return undefined;
}
