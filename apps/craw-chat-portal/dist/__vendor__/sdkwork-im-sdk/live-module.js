import { ImRealtimeModule } from './realtime-module.js';
import { toReceiveContext } from './receive-context.js';
import { ImWebSocketReceiver } from './websocket-receiver.js';
import { ImSdkError } from './errors.js';
class ImLiveRuntime {
    receiver;
    stateHandlers = new Set();
    errorHandlers = new Set();
    messages;
    data;
    signals;
    events;
    lifecycle;
    state = {
        status: 'connected',
        updatedAt: new Date().toISOString(),
    };
    constructor(receiver) {
        this.receiver = receiver;
        this.receiver.onConnected((frame) => {
            this.updateState({
                status: 'connected',
                connectedFrame: frame,
            });
        });
        this.receiver.onRealtimeError((frame) => {
            const error = new ImSdkError('websocket_protocol_error', frame.message, {
                code: frame.code,
                requestId: frame.requestId ?? undefined,
            });
            this.emitError({
                code: frame.code,
                source: 'realtime',
                error,
                requestId: frame.requestId ?? undefined,
                frame,
            });
            this.updateState({
                status: 'error',
                error,
            });
        });
        this.receiver.onSocketError((event) => {
            const error = event instanceof Error
                ? event
                : new ImSdkError('websocket_transport_error', 'Realtime websocket transport error.');
            this.emitError({
                code: 'socket_error',
                source: 'socket',
                error,
            });
            this.updateState({
                status: 'error',
                error,
            });
        });
        this.receiver.onClose((event) => {
            this.updateState({
                status: 'closed',
                closeEvent: event,
            });
        });
        this.messages = {
            on: (handler) => this.subscribeMessageContexts((context) => {
                handler(context.message, context);
            }),
            onConversation: (conversationId, handler) => this.subscribeConversationMessages(conversationId, (context) => {
                handler(context.message, context);
            }),
        };
        this.data = {
            on: (handler) => this.subscribeDataContexts((context) => {
                handler(context.data, context);
            }),
        };
        this.signals = {
            on: (handler) => this.subscribeSignalContexts((context) => {
                handler(context.signal, context);
            }),
            onRtcSession: (rtcSessionId, handler) => {
                const normalizedRtcSessionId = String(rtcSessionId);
                return this.subscribeSignalContexts((context) => {
                    if (String(context.scopeId) === normalizedRtcSessionId) {
                        handler(context.signal, context);
                    }
                });
            },
        };
        this.events = {
            on: (handler) => this.subscribeRawEvents(handler),
        };
        this.lifecycle = {
            onStateChange: (handler) => this.subscribeStateChanges(handler),
            onError: (handler) => this.subscribeErrors(handler),
            getState: () => this.state,
        };
    }
    subscribeRawEvents(handler) {
        return this.receiver.onEvent((event) => {
            handler(this.toContext(event));
        });
    }
    subscribeMessageContexts(handler) {
        return this.receiver.onMessageEvent((event) => {
            handler(this.toContext(event));
        });
    }
    subscribeConversationMessages(conversationId, handler) {
        const normalizedConversationId = String(conversationId);
        return this.receiver.onMessageEvent((event) => {
            const context = this.toContext(event);
            if (context.kind === 'message'
                && String(context.conversationId ?? context.scopeId) === normalizedConversationId) {
                handler(context);
            }
        });
    }
    subscribeDataContexts(handler) {
        return this.receiver.onDataEvent((event) => {
            const context = this.toContext(event);
            if (context.kind === 'data') {
                handler(context);
            }
        });
    }
    subscribeSignalContexts(handler) {
        return this.receiver.onRtcSignalEvent((event) => {
            const context = this.toContext(event);
            if (context.kind === 'signal') {
                handler(context);
            }
        });
    }
    subscribeStateChanges(handler) {
        this.stateHandlers.add(handler);
        handler(this.state);
        return () => {
            this.stateHandlers.delete(handler);
        };
    }
    subscribeErrors(handler) {
        this.errorHandlers.add(handler);
        return () => {
            this.errorHandlers.delete(handler);
        };
    }
    disconnect(code, reason) {
        this.receiver.close(code, reason);
    }
    toContext(event) {
        return toReceiveContext(event, 'live', () => this.receiver.ackWindow(event.realtimeSeq));
    }
    updateState(partialState) {
        this.state = {
            ...this.state,
            ...partialState,
            updatedAt: new Date().toISOString(),
        };
        for (const handler of this.stateHandlers) {
            handler(this.state);
        }
    }
    emitError(context) {
        for (const handler of this.errorHandlers) {
            handler(context);
        }
    }
}
export class ImLiveModule {
    context;
    constructor(context) {
        this.context = context;
    }
    async connect(options = {}) {
        const receiver = new ImWebSocketReceiver(new ImRealtimeModule(this.context), this.context, {
            deviceId: options.deviceId,
            url: options.url,
            mode: 'legacy_json',
            authToken: this.context.getAuthToken(),
            headers: options.headers,
            protocols: options.protocols,
            socket: options.socket,
            createSocket: options.socket ? undefined : this.context.webSocketFactory,
            requestTimeoutMs: options.requestTimeoutMs,
            webSocketAuth: options.webSocketAuth ?? this.context.webSocketAuth,
        });
        try {
            if (options.deviceId) {
                await this.context.transportClient.session.resume({
                    deviceId: options.deviceId,
                });
            }
            await receiver.connect();
            if (options.subscriptions) {
                await this.context.transportClient.realtime.syncRealtimeSubscriptions({
                    deviceId: options.deviceId,
                    items: toRealtimeSubscriptionItems(options.subscriptions),
                });
            }
            return new ImLiveRuntime(receiver);
        }
        catch (error) {
            receiver.close();
            throw error;
        }
    }
}
function toRealtimeSubscriptionItems(groups) {
    const items = [];
    for (const conversationId of groups.conversations ?? []) {
        items.push({
            scopeType: 'conversation',
            scopeId: String(conversationId),
            eventTypes: ['message.created', 'message.updated', 'message.recalled'],
        });
    }
    for (const rtcSessionId of groups.rtcSessions ?? []) {
        items.push({
            scopeType: 'rtc_session',
            scopeId: String(rtcSessionId),
            eventTypes: ['rtc.signal'],
        });
    }
    for (const item of groups.items ?? []) {
        items.push(item);
    }
    return items;
}
