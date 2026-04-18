import { CrawChatRealtimeModule } from './realtime-module.js';
import { toReceiveContext } from './receive-context.js';
import { CrawChatWebSocketReceiver } from './websocket-receiver.js';
import { CrawChatSdkError } from './errors.js';
import type {
  CrawChatConnectOptions,
  CrawChatDataContext,
  CrawChatLiveErrorContext,
  CrawChatLiveConnection,
  CrawChatLiveDataStream,
  CrawChatLiveEventStream,
  CrawChatLiveLifecycleStream,
  CrawChatLiveMessageStream,
  CrawChatLiveState,
  CrawChatMessageContext,
  CrawChatReceiveContext,
  CrawChatRealtimeSubscriptionGroups,
  CrawChatSignalContext,
  CrawChatLiveSignalStream,
  CrawChatSubscription,
  RealtimeSubscriptionItemInput,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

class CrawChatLiveRuntime implements CrawChatLiveConnection {
  private readonly stateHandlers = new Set<(state: CrawChatLiveState) => void>();
  private readonly errorHandlers = new Set<
    (context: CrawChatLiveErrorContext) => void
  >();
  readonly messages: CrawChatLiveMessageStream;
  readonly data: CrawChatLiveDataStream;
  readonly signals: CrawChatLiveSignalStream;
  readonly events: CrawChatLiveEventStream;
  readonly lifecycle: CrawChatLiveLifecycleStream;
  private state: CrawChatLiveState = {
    status: 'connected',
    updatedAt: new Date().toISOString(),
  };

  constructor(private readonly receiver: CrawChatWebSocketReceiver) {
    this.receiver.onConnected((frame) => {
      this.updateState({
        status: 'connected',
        connectedFrame: frame,
      });
    });

    this.receiver.onRealtimeError((frame) => {
      const error = new CrawChatSdkError('websocket_protocol_error', frame.message, {
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
      const error =
        event instanceof Error
          ? event
          : new CrawChatSdkError(
              'websocket_transport_error',
              'Realtime websocket transport error.',
            );
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
      onConversation: (conversationId, handler) =>
        this.subscribeConversationMessages(conversationId, (context) => {
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

  private subscribeRawEvents(
    handler: (context: CrawChatReceiveContext) => void,
  ): CrawChatSubscription {
    return this.receiver.onEvent((event) => {
      handler(this.toContext(event));
    });
  }

  private subscribeMessageContexts(
    handler: (context: CrawChatMessageContext) => void,
  ): CrawChatSubscription {
    return this.receiver.onMessageEvent((event) => {
      handler(this.toContext(event) as CrawChatMessageContext);
    });
  }

  private subscribeConversationMessages(
    conversationId: string | number,
    handler: (context: CrawChatMessageContext) => void,
  ): CrawChatSubscription {
    const normalizedConversationId = String(conversationId);
    return this.receiver.onMessageEvent((event) => {
      const context = this.toContext(event);
      if (
        context.kind === 'message'
        && String(context.conversationId ?? context.scopeId) === normalizedConversationId
      ) {
        handler(context);
      }
    });
  }

  private subscribeDataContexts(
    handler: (context: CrawChatDataContext) => void,
  ): CrawChatSubscription {
    return this.receiver.onDataEvent((event) => {
      const context = this.toContext(event);
      if (context.kind === 'data') {
        handler(context);
      }
    });
  }

  private subscribeSignalContexts(
    handler: (context: CrawChatSignalContext) => void,
  ): CrawChatSubscription {
    return this.receiver.onRtcSignalEvent((event) => {
      const context = this.toContext(event);
      if (context.kind === 'signal') {
        handler(context);
      }
    });
  }

  private subscribeStateChanges(
    handler: (state: CrawChatLiveState) => void,
  ): CrawChatSubscription {
    this.stateHandlers.add(handler);
    handler(this.state);
    return () => {
      this.stateHandlers.delete(handler);
    };
  }

  private subscribeErrors(
    handler: (context: CrawChatLiveErrorContext) => void,
  ): CrawChatSubscription {
    this.errorHandlers.add(handler);
    return () => {
      this.errorHandlers.delete(handler);
    };
  }

  disconnect(code?: number, reason?: string): void {
    this.receiver.close(code, reason);
  }

  private toContext(event: Parameters<typeof toReceiveContext>[0]): CrawChatReceiveContext {
    return toReceiveContext(
      event,
      'live',
      () => this.receiver.ackWindow(event.realtimeSeq),
    );
  }

  private updateState(
    partialState: Omit<Partial<CrawChatLiveState>, 'updatedAt'> & Pick<CrawChatLiveState, 'status'>,
  ): void {
    this.state = {
      ...this.state,
      ...partialState,
      updatedAt: new Date().toISOString(),
    };

    for (const handler of this.stateHandlers) {
      handler(this.state);
    }
  }

  private emitError(context: CrawChatLiveErrorContext): void {
    for (const handler of this.errorHandlers) {
      handler(context);
    }
  }
}

export class CrawChatLiveModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  async connect(options: CrawChatConnectOptions = {}): Promise<CrawChatLiveConnection> {
    const receiver = new CrawChatWebSocketReceiver(
      new CrawChatRealtimeModule(this.context),
      this.context,
      {
        url: options.url,
        mode: 'legacy_json',
        authToken: this.context.getAuthToken(),
        headers: options.headers,
        protocols: options.protocols,
        socket: options.socket,
        createSocket: options.socket ? undefined : this.context.webSocketFactory,
        requestTimeoutMs: options.requestTimeoutMs,
      },
    );

    try {
      if (options.deviceId) {
        await this.context.backendClient.session.resume({
          deviceId: options.deviceId,
        });
      }

      await receiver.connect();

      if (options.subscriptions) {
        await this.context.backendClient.realtime.syncRealtimeSubscriptions({
          deviceId: options.deviceId,
          items: toRealtimeSubscriptionItems(options.subscriptions),
        });
      }

      return new CrawChatLiveRuntime(receiver);
    } catch (error) {
      receiver.close();
      throw error;
    }
  }
}

function toRealtimeSubscriptionItems(
  groups: CrawChatRealtimeSubscriptionGroups,
): RealtimeSubscriptionItemInput[] {
  const items: RealtimeSubscriptionItemInput[] = [];

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
