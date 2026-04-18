import { CrawChatSdkError } from './errors.js';
import { CrawChatReceiver } from './receiver.js';
import type { CrawChatRealtimeModule } from './realtime-module.js';
import type { CrawChatSdkContext } from './sdk-context.js';
import type {
  CrawChatInternalRealtimeBatch,
  CrawChatInternalReceiverDataEvent,
  CrawChatInternalReceiverEvent,
  CrawChatInternalReceiverMessageEvent,
  CrawChatInternalReceiverRtcSignalEvent,
  CrawChatInternalRealtimeEventsAckedFrame,
  CrawChatInternalRealtimeSubscriptionsSyncedFrame,
  CrawChatInternalRealtimeWebSocketCcpOptions,
  CrawChatInternalRealtimeWebSocketReceiverOptions,
  CrawChatInternalRealtimeWindowFrame,
} from './receiver-internal-types.js';
import type {
  CrawChatRealtimeConnectedFrame,
  CrawChatRealtimeErrorFrame,
  CrawChatRealtimeWebSocketFactoryRequest,
  CrawChatSubscription,
  CrawChatWebSocketCloseEventLike,
  CrawChatWebSocketLike,
  QueryParams,
  RealtimeAckState,
  RealtimeEvent,
  RealtimeEventWindow,
  RealtimeSubscriptionItemInput,
  RealtimeSubscriptionSnapshot,
  SyncRealtimeSubscriptionsRequest,
} from './types.js';

const CCP_WEBSOCKET_SUBPROTOCOL = 'ccp/ws/1';
const DEFAULT_REQUEST_TIMEOUT_MS = 15_000;

type PendingRequest =
  | {
      kind: 'sync';
      resolve: (value: RealtimeSubscriptionSnapshot) => void;
      reject: (reason?: unknown) => void;
      timeout: ReturnType<typeof setTimeout>;
    }
  | {
      kind: 'pull';
      resolve: (value: CrawChatInternalRealtimeWindowFrame) => void;
      reject: (reason?: unknown) => void;
      timeout: ReturnType<typeof setTimeout>;
    }
  | {
      kind: 'ack';
      resolve: (value: RealtimeAckState) => void;
      reject: (reason?: unknown) => void;
      timeout: ReturnType<typeof setTimeout>;
    };

interface CcpProtocolVersion {
  family: string;
  major: number;
  minor: number;
}

type CcpTransportBinding = 'Http1' | 'Ws1' | 'Sse1' | 'Mqtt1';

interface CcpEnvelope {
  protocol: CcpProtocolVersion;
  binding: CcpTransportBinding;
  kind: string;
  schema: string;
  scope?: unknown;
  route?: unknown;
  flags?: string[];
  traceId?: string | null;
  trace_id?: string | null;
  payload: string;
}

interface CcpCapabilitySet {
  items?: string[];
}

type CcpControlFrame =
  | {
      type: 'hello_ack';
      data: {
        protocol: CcpProtocolVersion;
        binding: CcpTransportBinding;
        capabilities?: CcpCapabilitySet;
        accepted: boolean;
      };
    }
  | {
      type: 'auth_ok';
      data: {
        tenant_id: string;
        principal_id: string;
        actor_kind: string;
        device_id?: string | null;
        session_id?: string | null;
      };
    }
  | {
      type: 'session_resumed';
      data: {
        session_id: string;
        resumed: boolean;
      };
    }
  | {
      type: 'error';
      data: {
        code: string;
        message: string;
        retryable?: boolean;
      };
    }
  | {
      type: 'goaway';
      data: {
        code: string;
        message: string;
      };
    }
  | {
      type: 'heartbeat';
      data?: {
        sequence?: number;
      };
    };

export class CrawChatWebSocketReceiver {
  private readonly receiver: CrawChatReceiver;
  private readonly openHandlers = new Set<(event: unknown) => void>();
  private readonly closeHandlers = new Set<
    (event: CrawChatWebSocketCloseEventLike | unknown) => void
  >();
  private readonly socketErrorHandlers = new Set<(event: unknown) => void>();
  private readonly connectedHandlers = new Set<
    (frame: CrawChatRealtimeConnectedFrame) => void
  >();
  private readonly windowHandlers = new Set<
    (frame: CrawChatInternalRealtimeWindowFrame) => void
  >();
  private readonly syncedHandlers = new Set<
    (frame: CrawChatInternalRealtimeSubscriptionsSyncedFrame) => void
  >();
  private readonly ackedHandlers = new Set<
    (frame: CrawChatInternalRealtimeEventsAckedFrame) => void
  >();
  private readonly realtimeErrorHandlers = new Set<
    (frame: CrawChatRealtimeErrorFrame) => void
  >();
  private readonly pendingRequests = new Map<string, PendingRequest>();
  private readonly options: Required<
    Pick<CrawChatInternalRealtimeWebSocketReceiverOptions, 'mode' | 'requestTimeoutMs'>
  >
    & Omit<CrawChatInternalRealtimeWebSocketReceiverOptions, 'mode' | 'requestTimeoutMs'>;

  private socket?: CrawChatWebSocketLike;
  private cleanupSocketListeners: Array<() => void> = [];
  private connectPromise?: Promise<this>;
  private isConnected = false;
  private requestSequence = 0;
  private connectResolve?: (value: this) => void;
  private connectReject?: (reason?: unknown) => void;
  private ccpSessionResumeEnabled = false;

  constructor(
    private readonly realtime: CrawChatRealtimeModule,
    private readonly context: CrawChatSdkContext,
    options: CrawChatInternalRealtimeWebSocketReceiverOptions = {},
  ) {
    this.receiver = new CrawChatReceiver(realtime);
    this.options = {
      ...options,
      mode: options.mode ?? 'legacy_json',
      requestTimeoutMs: options.requestTimeoutMs ?? DEFAULT_REQUEST_TIMEOUT_MS,
    };
  }

  onOpen(handler: (event: unknown) => void): CrawChatSubscription {
    this.openHandlers.add(handler);
    return () => {
      this.openHandlers.delete(handler);
    };
  }

  onClose(
    handler: (event: CrawChatWebSocketCloseEventLike | unknown) => void,
  ): CrawChatSubscription {
    this.closeHandlers.add(handler);
    return () => {
      this.closeHandlers.delete(handler);
    };
  }

  onSocketError(handler: (event: unknown) => void): CrawChatSubscription {
    this.socketErrorHandlers.add(handler);
    return () => {
      this.socketErrorHandlers.delete(handler);
    };
  }

  onConnected(
    handler: (frame: CrawChatRealtimeConnectedFrame) => void,
  ): CrawChatSubscription {
    this.connectedHandlers.add(handler);
    return () => {
      this.connectedHandlers.delete(handler);
    };
  }

  onWindow(
    handler: (frame: CrawChatInternalRealtimeWindowFrame) => void,
  ): CrawChatSubscription {
    this.windowHandlers.add(handler);
    return () => {
      this.windowHandlers.delete(handler);
    };
  }

  onSubscriptionsSynced(
    handler: (frame: CrawChatInternalRealtimeSubscriptionsSyncedFrame) => void,
  ): CrawChatSubscription {
    this.syncedHandlers.add(handler);
    return () => {
      this.syncedHandlers.delete(handler);
    };
  }

  onAcked(
    handler: (frame: CrawChatInternalRealtimeEventsAckedFrame) => void,
  ): CrawChatSubscription {
    this.ackedHandlers.add(handler);
    return () => {
      this.ackedHandlers.delete(handler);
    };
  }

  onRealtimeError(
    handler: (frame: CrawChatRealtimeErrorFrame) => void,
  ): CrawChatSubscription {
    this.realtimeErrorHandlers.add(handler);
    return () => {
      this.realtimeErrorHandlers.delete(handler);
    };
  }

  onEvent(handler: (event: CrawChatInternalReceiverEvent) => void): CrawChatSubscription {
    return this.receiver.onEvent(handler);
  }

  onMessageEvent(
    handler: (event: CrawChatInternalReceiverMessageEvent) => void,
  ): CrawChatSubscription {
    return this.receiver.onMessageEvent(handler);
  }

  onDataEvent(
    handler: (event: CrawChatInternalReceiverDataEvent) => void,
  ): CrawChatSubscription {
    return this.receiver.onDataEvent(handler);
  }

  onRtcSignalEvent(
    handler: (event: CrawChatInternalReceiverRtcSignalEvent) => void,
  ): CrawChatSubscription {
    return this.receiver.onRtcSignalEvent(handler);
  }

  onScope(
    scopeType: string,
    scopeId: string | number,
    handler: (event: CrawChatInternalReceiverEvent) => void,
  ): CrawChatSubscription {
    return this.receiver.onScope(scopeType, scopeId, handler);
  }

  async connect(): Promise<this> {
    if (this.isConnected) {
      return this;
    }
    if (this.connectPromise) {
      return this.connectPromise;
    }

    this.connectPromise = new Promise<this>(async (resolve, reject) => {
      this.connectResolve = resolve;
      this.connectReject = reject;

      try {
        this.socket = await this.resolveSocket();
        this.attachSocket(this.socket);

        if (this.socket.readyState === 1) {
          void this.handleSocketOpen({ type: 'open' });
        }
      } catch (error) {
        this.rejectConnect(error);
      }
    });

    return this.connectPromise;
  }

  close(code?: number, reason?: string): void {
    this.socket?.close(code, reason);
    this.cleanupPendingRequests(
      new CrawChatSdkError(
        'websocket_closed',
        'Realtime connection closed before the pending operation completed.',
      ),
    );
    this.resetSocketState();
  }

  async syncSubscriptions(
    body:
      | SyncRealtimeSubscriptionsRequest
      | RealtimeSubscriptionItemInput[],
  ): Promise<RealtimeSubscriptionSnapshot> {
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
    const responsePromise = this.createPendingRequest(
      requestId,
      'sync',
    ) as Promise<RealtimeSubscriptionSnapshot>;

    await this.sendBusinessFrame(payload, 'cmd', 'cc.realtime.subscriptions.sync.v1');
    return responsePromise;
  }

  async pullWindow(
    params: QueryParams = {},
  ): Promise<CrawChatInternalRealtimeWindowFrame> {
    const requestId = this.createRequestId('events_pull');
    const payload = {
      type: 'events.pull',
      requestId,
      afterSeq: params.afterSeq,
      limit: params.limit,
    };
    const responsePromise = this.createPendingRequest(
      requestId,
      'pull',
    ) as Promise<CrawChatInternalRealtimeWindowFrame>;

    await this.sendBusinessFrame(payload, 'cmd', 'cc.realtime.events.pull.v1');
    return responsePromise;
  }

  async ackWindow(
    batchOrSeq: CrawChatInternalRealtimeBatch | number,
  ): Promise<RealtimeAckState> {
    const requestId = this.createRequestId('events_ack');
    const ackedSeq =
      typeof batchOrSeq === 'number' ? batchOrSeq : batchOrSeq.highestSeq;
    const payload = {
      type: 'events.ack',
      requestId,
      ackedSeq,
    };
    const responsePromise = this.createPendingRequest(
      requestId,
      'ack',
    ) as Promise<RealtimeAckState>;

    await this.sendBusinessFrame(payload, 'ack', 'cc.realtime.events.ack.v1');
    return responsePromise;
  }

  private async resolveSocket(): Promise<CrawChatWebSocketLike> {
    if (this.options.socket) {
      return this.options.socket;
    }

    const url = this.options.url ?? this.context.resolveRealtimeWebSocketUrl();
    if (!url) {
      throw new CrawChatSdkError(
        'websocket_url_required',
        'websocketBaseUrl or connect({ url }) is required to establish realtime connectivity.',
      );
    }

    const protocols =
      this.options.protocols
      ?? (this.options.mode === 'ccp_json' ? [CCP_WEBSOCKET_SUBPROTOCOL] : []);
    const authToken = normalizeAuthToken(this.options.authToken);
    const headers: Record<string, string> = {
      ...(this.options.headers ?? {}),
    };
    if (authToken) {
      headers.Authorization = `Bearer ${authToken}`;
    }

    const request: CrawChatRealtimeWebSocketFactoryRequest = {
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

  private attachSocket(socket: CrawChatWebSocketLike): void {
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

  private async handleSocketOpen(event: unknown): Promise<void> {
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

  private async handleSocketMessage(event: unknown): Promise<void> {
    try {
      const message = await resolveSocketMessageData(event);
      if (message == null) {
        return;
      }

      const parsed = JSON.parse(message) as unknown;
      const businessFrame = this.unwrapBusinessFrame(parsed);
      if (!businessFrame) {
        return;
      }

      if (businessFrame.type === 'realtime.connected') {
        const connectedFrame = businessFrame as unknown as CrawChatRealtimeConnectedFrame;
        this.isConnected = true;
        for (const handler of this.connectedHandlers) {
          handler(connectedFrame);
        }
        this.resolveConnect();
        return;
      }

      if (businessFrame.type === 'subscriptions.synced') {
        const frame =
          businessFrame as unknown as CrawChatInternalRealtimeSubscriptionsSyncedFrame;
        for (const handler of this.syncedHandlers) {
          handler(frame);
        }
        this.resolvePendingRequest(frame.requestId ?? undefined, frame.snapshot);
        return;
      }

      if (businessFrame.type === 'events.acked') {
        const frame =
          businessFrame as unknown as CrawChatInternalRealtimeEventsAckedFrame;
        for (const handler of this.ackedHandlers) {
          handler(frame);
        }
        this.resolvePendingRequest(frame.requestId ?? undefined, frame.ack);
        return;
      }

      if (businessFrame.type === 'event.window') {
        const window =
          businessFrame as Omit<CrawChatInternalRealtimeWindowFrame, 'batch'>;
        const batch = toReceiverBatch(this.receiver, window.window);
        const frame: CrawChatInternalRealtimeWindowFrame = {
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
        const frame = businessFrame as unknown as CrawChatRealtimeErrorFrame;
        const error = new CrawChatSdkError('websocket_protocol_error', frame.message, {
          remoteCode: frame.code,
          requestId: frame.requestId ?? undefined,
        });
        for (const handler of this.realtimeErrorHandlers) {
          handler(frame);
        }
        if (frame.requestId) {
          this.rejectPendingRequest(frame.requestId, error);
        } else {
          this.rejectConnect(error);
        }
      }
    } catch (error) {
      this.handleSocketError(error);
    }
  }

  private handleSocketClose(event: unknown): void {
    for (const handler of this.closeHandlers) {
      handler(event);
    }

    this.cleanupPendingRequests(
      new CrawChatSdkError(
        'websocket_closed',
        'Realtime websocket closed before the pending operation completed.',
      ),
    );

    if (!this.isConnected) {
      this.rejectConnect(
        new CrawChatSdkError(
          'websocket_closed',
          'Realtime websocket closed before the connected frame arrived.',
        ),
      );
    }

    this.resetSocketState();
  }

  private handleSocketError(event: unknown): void {
    for (const handler of this.socketErrorHandlers) {
      handler(event);
    }

    if (!this.isConnected) {
      this.rejectConnect(
        event instanceof Error
          ? event
          : new CrawChatSdkError(
              'websocket_transport_error',
              'Realtime websocket failed before the connection became ready.',
            ),
      );
    }
  }

  private unwrapBusinessFrame(parsed: unknown): Record<string, unknown> | undefined {
    if (looksLikeCcpEnvelope(parsed)) {
      if (parsed.kind === 'control') {
        const frame = JSON.parse(parsed.payload) as CcpControlFrame;
        void this.handleCcpControlFrame(frame);
        return undefined;
      }

      return JSON.parse(parsed.payload) as Record<string, unknown>;
    }

    return parsed && typeof parsed === 'object'
      ? (parsed as Record<string, unknown>)
      : undefined;
  }

  private async handleCcpControlFrame(frame: CcpControlFrame): Promise<void> {
    if (frame.type === 'hello_ack') {
      if (!frame.data.accepted) {
        this.rejectConnect(
          new CrawChatSdkError(
            'websocket_protocol_error',
            'Realtime CCP hello negotiation was rejected by the server.',
          ),
        );
        return;
      }

      this.ccpSessionResumeEnabled = Boolean(
        frame.data.capabilities?.items?.includes('session.resume'),
      );
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
      const error = new CrawChatSdkError(
        'websocket_protocol_error',
        frame.data.message,
        {
          code: frame.data.code,
        },
      );
      this.rejectConnect(error);
      this.cleanupPendingRequests(error);
      this.close();
    }
  }

  private async sendBusinessFrame(
    payload: Record<string, unknown>,
    ccpKind: string,
    ccpSchema: string,
  ): Promise<void> {
    const socket = this.requireSocket();

    if (this.options.mode === 'ccp_json') {
      await this.sendJson(
        socket,
        buildCcpEnvelope(ccpKind, ccpSchema, payload),
      );
      return;
    }

    await this.sendJson(socket, payload);
  }

  private async sendCcpControlFrame(
    schema: string,
    payload: Record<string, unknown>,
  ): Promise<void> {
    await this.sendJson(
      this.requireSocket(),
      buildCcpEnvelope('control', schema, payload),
    );
  }

  private async sendJson(
    socket: CrawChatWebSocketLike,
    payload: unknown,
  ): Promise<void> {
    await Promise.resolve(socket.send(JSON.stringify(payload)));
  }

  private requireSocket(): CrawChatWebSocketLike {
    if (!this.socket) {
      throw new CrawChatSdkError(
        'websocket_not_connected',
        'Realtime connection is not connected. Call connect() first.',
      );
    }

    return this.socket;
  }

  private createPendingRequest(
    requestId: string,
    kind: PendingRequest['kind'],
  ): Promise<unknown> {
    return new Promise((resolve, reject) => {
      const timeout = setTimeout(() => {
        this.pendingRequests.delete(requestId);
        reject(
          new CrawChatSdkError(
            'websocket_request_timeout',
            `Realtime websocket request ${requestId} timed out.`,
            {
              requestId,
              kind,
            },
          ),
        );
      }, this.options.requestTimeoutMs);

      this.pendingRequests.set(requestId, {
        kind,
        resolve: resolve as PendingRequest['resolve'],
        reject,
        timeout,
      } as PendingRequest);
    });
  }

  private resolvePendingRequest(requestId: string | undefined, value: unknown): void {
    if (!requestId) {
      return;
    }
    const pending = this.pendingRequests.get(requestId);
    if (!pending) {
      return;
    }

    clearTimeout(pending.timeout);
    this.pendingRequests.delete(requestId);
    pending.resolve(value as never);
  }

  private rejectPendingRequest(requestId: string, reason: unknown): void {
    const pending = this.pendingRequests.get(requestId);
    if (!pending) {
      return;
    }

    clearTimeout(pending.timeout);
    this.pendingRequests.delete(requestId);
    pending.reject(reason);
  }

  private cleanupPendingRequests(reason: unknown): void {
    for (const [requestId, pending] of this.pendingRequests) {
      clearTimeout(pending.timeout);
      pending.reject(reason);
      this.pendingRequests.delete(requestId);
    }
  }

  private createRequestId(prefix: string): string {
    this.requestSequence += 1;
    return `${prefix}_${this.requestSequence}`;
  }

  private resolveConnect(): void {
    const resolve = this.connectResolve;
    this.connectResolve = undefined;
    this.connectReject = undefined;
    resolve?.(this);
  }

  private rejectConnect(reason: unknown): void {
    const reject = this.connectReject;
    this.connectResolve = undefined;
    this.connectReject = undefined;
    this.connectPromise = undefined;
    reject?.(reason);
  }

  private resetSocketState(): void {
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

function toReceiverBatch(
  receiver: CrawChatReceiver,
  rawWindow: RealtimeEventWindow,
): CrawChatInternalRealtimeBatch {
  const items = rawWindow.items.map((item: RealtimeEvent) =>
    receiver.dispatchRealtimeEvent(item),
  );
  const highestSeq = items.reduce(
    (currentMax: number, item: CrawChatInternalReceiverEvent) =>
      Math.max(currentMax, item.realtimeSeq),
    rawWindow.ackedThroughSeq ?? 0,
  );

  return {
    items,
    highestSeq,
    rawWindow,
  };
}

function normalizeAuthToken(token: string | undefined): string | undefined {
  if (!token) {
    return undefined;
  }

  return token.startsWith('Bearer ') ? token.slice('Bearer '.length) : token;
}

function createDefaultSocket(
  request: CrawChatRealtimeWebSocketFactoryRequest,
): CrawChatWebSocketLike {
  const WebSocketConstructor = globalThis.WebSocket;
  if (typeof WebSocketConstructor !== 'function') {
    throw new CrawChatSdkError(
      'websocket_factory_required',
      'No global WebSocket implementation is available. Provide webSocketFactory to establish realtime connectivity in this runtime.',
    );
  }

  if (Object.keys(request.headers).length > 0) {
    throw new CrawChatSdkError(
      'websocket_factory_required',
      'The default WebSocket implementation cannot attach Authorization headers. Provide webSocketFactory for authenticated realtime connections.',
    );
  }

  return new WebSocketConstructor(
    request.url,
    request.protocols,
  ) as unknown as CrawChatWebSocketLike;
}

function requireCcpOptions(
  options: CrawChatInternalRealtimeWebSocketCcpOptions | undefined,
): CrawChatInternalRealtimeWebSocketCcpOptions {
  if (options) {
    return options;
  }

  throw new CrawChatSdkError(
    'ccp_auth_required',
    'CCP websocket mode requires ccp.principalId and ccp.actorKind so auth_bind can be negotiated.',
  );
}

function looksLikeCcpEnvelope(value: unknown): value is CcpEnvelope {
  if (!value || typeof value !== 'object') {
    return false;
  }

  const candidate = value as Record<string, unknown>;
  return (
    typeof candidate.protocol === 'object'
    && typeof candidate.kind === 'string'
    && typeof candidate.schema === 'string'
    && typeof candidate.payload === 'string'
  );
}

function buildCcpEnvelope(
  kind: string,
  schema: string,
  payload: Record<string, unknown>,
): CcpEnvelope {
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

async function resolveSocketMessageData(event: unknown): Promise<string | undefined> {
  if (typeof event === 'string') {
    return event;
  }
  if (event instanceof ArrayBuffer) {
    return new TextDecoder().decode(event);
  }
  if (ArrayBuffer.isView(event)) {
    return new TextDecoder().decode(event);
  }
  const data = (event as { data?: unknown })?.data;
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

function attachSocketListener(
  socket: CrawChatWebSocketLike,
  type: 'open' | 'message' | 'close' | 'error',
  listener: (event: unknown) => void,
): () => void {
  if (socket.addEventListener && socket.removeEventListener) {
    socket.addEventListener(type, listener);
    return () => {
      socket.removeEventListener?.(type, listener);
    };
  }

  if (socket.on && socket.off) {
    const wrappedListener = (...args: unknown[]) => {
      if (type === 'message') {
        listener({ data: args[0] });
        return;
      }
      if (type === 'close') {
        listener({
          code: args[0],
          reason:
            typeof args[1] === 'string'
              ? args[1]
              : decodeUnknownText(args[1]),
        });
        return;
      }
      listener(args[0]);
    };
    socket.on(type, wrappedListener as (event: unknown) => void);
    return () => {
      socket.off?.(type, wrappedListener as (event: unknown) => void);
    };
  }

  const propertyName = `on${type}` as 'onopen' | 'onmessage' | 'onclose' | 'onerror';
  const previous = socket[propertyName];
  socket[propertyName] = listener as never;

  return () => {
    socket[propertyName] = previous as never;
  };
}

function decodeUnknownText(value: unknown): string | undefined {
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
