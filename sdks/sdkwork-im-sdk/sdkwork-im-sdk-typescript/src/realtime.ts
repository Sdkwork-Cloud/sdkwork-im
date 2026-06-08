import type {
  MediaResource,
  MessageBody,
  MessageReplyReference,
  MessageType,
  Sender,
} from '@sdkwork/im-sdk-generated';

export interface ImMessageContext {
  ack(): Promise<void>;
  conversationId?: string;
  eventId?: string;
  eventType?: string;
  messageId?: string;
  payload?: Record<string, unknown>;
  rawEvent?: Record<string, unknown>;
  receivedAt: string;
  sender?: Sender;
  sequence: number;
}

export interface ImDecodedAttachment {
  resource?: MediaResource;
  [key: string]: unknown;
}

export interface ImDecodedMessage {
  attachments: ImDecodedAttachment[];
  body?: MessageBody;
  content?: Record<string, unknown>;
  conversationId?: string;
  deliveryMode?: string;
  messageId?: string;
  messageSeq?: number;
  messageType?: MessageType;
  occurredAt?: string;
  renderHints?: Record<string, unknown>;
  replyTo?: MessageReplyReference;
  sender?: Sender;
  summary?: string;
  text?: string;
  type?: string;
  [key: string]: unknown;
}

export type ImSubscription = () => void;

export interface ImLiveConnectionState {
  status: 'connecting' | 'open' | 'closed' | 'error';
  reason?: string;
}

export interface ImLiveConnection {
  disconnect(code?: number, reason?: string): void;
  lifecycle: {
    onError(handler: (error: unknown) => void): ImSubscription;
    onStateChange(handler: (state: ImLiveConnectionState) => void): ImSubscription;
  };
  messages: {
    onConversation(
      conversationId: string,
      handler: (message: ImDecodedMessage, context: ImMessageContext) => void,
    ): ImSubscription;
  };
}

export type ImWebSocketEventName = 'close' | 'error' | 'message' | 'open';

export interface ImWebSocketLike {
  readyState: number;
  addEventListener(type: ImWebSocketEventName, handler: (event: unknown) => void): void;
  close(code?: number, reason?: string): void;
  send(value: string): void;
}

export interface ImWebSocketFactoryOptions {
  headers: Record<string, string>;
  protocols: string[];
}

export type ImWebSocketFactory = (
  url: string,
  options: ImWebSocketFactoryOptions,
) => ImWebSocketLike;

export interface ImConnectOptions {
  deviceId?: string;
  subscriptions?: {
    conversations?: string[];
  };
}

export interface ImWebSocketCredentialProvider {
  (): string | undefined;
}

export interface ImWebSocketAuthConfig {
  credentialProvider?: ImWebSocketCredentialProvider;
  mode: 'automatic' | 'none';
}

export class ImWebSocketAuthOptions {
  static automatic(options: Omit<ImWebSocketAuthConfig, 'mode'> = {}): ImWebSocketAuthConfig {
    return { ...options, mode: 'automatic' };
  }

  static none(): ImWebSocketAuthConfig {
    return { mode: 'none' };
  }
}

export interface ImCreateLiveConnectionParams {
  accessToken?: string;
  auth?: ImWebSocketAuthConfig;
  authToken?: string;
  headerProvider?: () => Record<string, string>;
  headers?: Record<string, string>;
  options: ImConnectOptions;
  tokenManager?: unknown;
  websocketBaseUrl: string;
  webSocketFactory?: ImWebSocketFactory;
}

interface ListenerBag {
  errors: Set<(error: unknown) => void>;
  messages: Map<string, Set<(message: ImDecodedMessage, context: ImMessageContext) => void>>;
  states: Set<(state: ImLiveConnectionState) => void>;
}

interface ParsedRealtimeMessage {
  context: ImMessageContext;
  message: ImDecodedMessage;
}

interface ResolvedWebSocketCredentials {
  accessToken?: string;
  authToken?: string;
}

interface WebSocketTokenManagerLike {
  getAccessToken?: () => string | undefined;
  getAuthToken?: () => string | undefined;
  getTokens?: () => {
    accessToken?: string;
    authToken?: string;
  } | undefined;
}

type BrowserWebSocketConstructor = new (
  url: string,
  protocols?: string | string[],
) => ImWebSocketLike;

const IM_REALTIME_WEBSOCKET_PATH = '/im/v3/api/realtime/ws';
const SOCKET_OPEN_STATE = 1;
const SOCKET_CLOSING_STATE = 2;
const SOCKET_CLOSED_STATE = 3;

function subscribe<T>(set: Set<T>, value: T): ImSubscription {
  set.add(value);
  return () => {
    set.delete(value);
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function pickStringArray(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value
    .filter((item): item is string => typeof item === 'string')
    .map((item) => item.trim())
    .filter((item) => item.length > 0);
}

function pickNumber(...values: unknown[]): number | undefined {
  for (const value of values) {
    if (typeof value === 'number' && Number.isFinite(value)) {
      return value;
    }
    if (typeof value === 'string' && value.trim().length > 0) {
      const parsed = Number(value);
      if (Number.isFinite(parsed)) {
        return parsed;
      }
    }
  }
  return undefined;
}

function parseRecordPayload(value: unknown): Record<string, unknown> | undefined {
  if (isRecord(value)) {
    return value;
  }
  if (typeof value !== 'string' || value.trim().length === 0) {
    return undefined;
  }
  try {
    const parsed: unknown = JSON.parse(value);
    return isRecord(parsed) ? parsed : undefined;
  } catch {
    return undefined;
  }
}

function normalizeAttachments(value: unknown): ImDecodedAttachment[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value.filter(isRecord) as ImDecodedAttachment[];
}

function normalizeMessage(
  record: Record<string, unknown>,
  contentFallback?: Record<string, unknown>,
): ImDecodedMessage {
  return {
    ...record,
    attachments: normalizeAttachments(record.attachments),
    content: parseRecordPayload(record.content) ?? contentFallback,
    renderHints: parseRecordPayload(record.renderHints),
  } as ImDecodedMessage;
}

function normalizeAuthorizationHeader(token: string): string {
  return /^Bearer\s+/iu.test(token) ? token : `Bearer ${token}`;
}

function addHeader(headers: Record<string, string>, key: string, value: unknown): void {
  if (typeof value !== 'string' || value.trim().length === 0) {
    return;
  }
  headers[key] = value.trim();
}

function buildWebSocketHeaders({
  accessToken,
  authToken,
  headerProvider,
  headers,
}: ResolvedWebSocketCredentials & Pick<ImCreateLiveConnectionParams, 'headerProvider' | 'headers'>): Record<string, string> {
  const resolvedHeaders: Record<string, string> = {};

  if (authToken) {
    resolvedHeaders.Authorization = normalizeAuthorizationHeader(authToken);
  }
  addHeader(resolvedHeaders, 'Access-Token', accessToken);

  for (const [key, value] of Object.entries(headers ?? {})) {
    addHeader(resolvedHeaders, key, value);
  }
  for (const [key, value] of Object.entries(headerProvider?.() ?? {})) {
    addHeader(resolvedHeaders, key, value);
  }

  return resolvedHeaders;
}

function resolveWebSocketCredentials({
  accessToken,
  auth,
  authToken,
  tokenManager,
}: Pick<ImCreateLiveConnectionParams, 'accessToken' | 'auth' | 'authToken' | 'tokenManager'>): ResolvedWebSocketCredentials {
  const manager = isRecord(tokenManager) ? tokenManager as WebSocketTokenManagerLike : undefined;
  const managerTokens = manager?.getTokens?.();
  const websocketCredential = auth?.mode === 'automatic' ? auth.credentialProvider?.() : undefined;
  return {
    accessToken: pickString(manager?.getAccessToken?.(), managerTokens?.accessToken, accessToken),
    authToken: pickString(manager?.getAuthToken?.(), managerTokens?.authToken, authToken, websocketCredential),
  };
}

function appendRealtimeRoutePath(pathname: string): string {
  const basePath = pathname.replace(/\/+$/u, '');
  if (basePath.endsWith(IM_REALTIME_WEBSOCKET_PATH)) {
    return basePath;
  }
  return `${basePath}${IM_REALTIME_WEBSOCKET_PATH}`;
}

function buildWebSocketUrl(websocketBaseUrl: string, options: ImConnectOptions): string {
  const url = new URL(websocketBaseUrl);
  url.pathname = appendRealtimeRoutePath(url.pathname);

  if (options.deviceId) {
    url.searchParams.set('deviceId', options.deviceId);
  }

  return url.toString();
}

function resolveWebSocketFactory(factory?: ImWebSocketFactory): ImWebSocketFactory {
  if (factory) {
    return factory;
  }

  const WebSocketConstructor = globalThis.WebSocket as unknown;
  if (typeof WebSocketConstructor !== 'function') {
    throw new Error('IM websocket transport is unavailable; provide ImSdkClientOptions.webSocketFactory.');
  }

  return (url, options) => new (WebSocketConstructor as BrowserWebSocketConstructor)(
    url,
    options.protocols,
  );
}

function unwrapWirePayload(parsed: Record<string, unknown>): Record<string, unknown> {
  const payload = parseRecordPayload(parsed.payload);
  if (pickString(parsed.schema) && payload && pickString(payload.type)) {
    return payload;
  }
  return parsed;
}

function extractMessageData(event: unknown): string | undefined {
  if (typeof event === 'string') {
    return event;
  }
  if (isRecord(event) && typeof event.data === 'string') {
    return event.data;
  }
  return undefined;
}

function resolveEventScopeType(event: Record<string, unknown>): string | undefined {
  return pickString(event.scopeType, event.scope);
}

function resolveConversationId(
  message: ImDecodedMessage,
  payload: Record<string, unknown> | undefined,
  event: Record<string, unknown>,
): string | undefined {
  const scopeType = resolveEventScopeType(event);
  return pickString(
    message.conversationId,
    payload?.conversationId,
    event.conversationId,
    scopeType === 'conversation' ? event.scopeId : undefined,
  );
}

function createNoopContextAck(): Promise<void> {
  return Promise.resolve();
}

function parseDirectRealtimePayload(frame: Record<string, unknown>): ParsedRealtimeMessage | null {
  const payload = parseRecordPayload(frame.payload);
  const messageRecord = payload ?? frame;
  const message = normalizeMessage(messageRecord, payload);
  const messageSender = isRecord(message.sender) ? message.sender as unknown as Sender : undefined;
  const payloadSender = payload && isRecord(payload.sender) ? payload.sender as unknown as Sender : undefined;
  const conversationId = resolveConversationId(message, payload, frame);
  const messageId = pickString(message.messageId, payload?.messageId, frame.messageId, frame.eventId);
  const sequence = pickNumber(
    frame.realtimeSeq,
    frame.sequence,
    message.messageSeq,
    payload?.messageSeq,
    payload?.sequence,
    frame.messageSeq,
  ) ?? 0;

  if (!conversationId) {
    return null;
  }

  return {
    context: {
      ack: createNoopContextAck,
      conversationId,
      eventId: pickString(frame.eventId),
      eventType: pickString(frame.eventType, frame.type),
      messageId,
      payload,
      rawEvent: frame,
      receivedAt: pickString(frame.receivedAt, frame.occurredAt, message.occurredAt) ?? new Date().toISOString(),
      sender: messageSender ?? payloadSender,
      sequence,
    },
    message,
  };
}

function parseRealtimeEventWindow(frame: Record<string, unknown>): ParsedRealtimeMessage[] {
  const window = isRecord(frame.window) ? frame.window : undefined;
  const items = Array.isArray(window?.items) ? window.items.filter(isRecord) : [];
  const messages: ParsedRealtimeMessage[] = [];

  for (const item of items) {
    const eventType = pickString(item.eventType, item.type);
    const scopeType = resolveEventScopeType(item);
    if (eventType && eventType !== 'message.posted') {
      continue;
    }
    if (scopeType && scopeType !== 'conversation') {
      continue;
    }

    const payload = parseRecordPayload(item.payload);
    const messageRecord = payload ?? item;
    const message = normalizeMessage(messageRecord, payload);
    const occurredAt = pickString(message.occurredAt, payload?.occurredAt, item.occurredAt);
    if (occurredAt) {
      message.occurredAt = occurredAt;
    }

    const messageSender = isRecord(message.sender) ? message.sender as unknown as Sender : undefined;
    const payloadSender = payload && isRecord(payload.sender) ? payload.sender as unknown as Sender : undefined;
    const conversationId = resolveConversationId(message, payload, item);
    if (!conversationId) {
      continue;
    }

    const sequence = pickNumber(
      item.realtimeSeq,
      item.sequence,
      payload?.realtimeSeq,
      payload?.messageSeq,
      item.messageSeq,
      window?.nextAfterSeq,
    ) ?? 0;
    const messageId = pickString(message.messageId, payload?.messageId, item.messageId, item.eventId);

    messages.push({
      context: {
        ack: createNoopContextAck,
        conversationId,
        eventId: pickString(item.eventId),
        eventType: eventType ?? 'message.posted',
        messageId,
        payload,
        rawEvent: item,
        receivedAt: pickString(frame.receivedAt, item.receivedAt, item.occurredAt, occurredAt) ?? new Date().toISOString(),
        sender: messageSender ?? payloadSender,
        sequence,
      },
      message,
    });
  }

  return messages;
}

function parseRealtimePayloads(raw: string): ParsedRealtimeMessage[] {
  try {
    const parsed: unknown = JSON.parse(raw);
    if (!isRecord(parsed)) {
      return [];
    }
    const frame = unwrapWirePayload(parsed);
    if (pickString(frame.type) === 'event.window') {
      return parseRealtimeEventWindow(frame);
    }

    const directMessage = parseDirectRealtimePayload(frame);
    return directMessage ? [directMessage] : [];
  } catch {
    return [];
  }
}

function sendSubscriptionSync(
  socket: ImWebSocketLike,
  conversations: string[],
  requestId: string,
): void {
  if (conversations.length === 0 || socket.readyState !== SOCKET_OPEN_STATE) {
    return;
  }
  socket.send(JSON.stringify({
    type: 'subscriptions.sync',
    requestId,
    items: conversations.map((conversationId) => ({
      scopeType: 'conversation',
      scopeId: conversationId,
      eventTypes: ['message.posted'],
    })),
  }));
}

function sendAuthInit(
  socket: ImWebSocketLike,
  credentials: Required<ResolvedWebSocketCredentials>,
  deviceId: string | undefined,
  requestId: string,
): void {
  if (socket.readyState !== SOCKET_OPEN_STATE) {
    return;
  }
  socket.send(JSON.stringify({
    type: 'auth.init',
    requestId,
    authToken: credentials.authToken,
    accessToken: credentials.accessToken,
    ...(deviceId ? { deviceId } : {}),
  }));
}

function parseRealtimeControlFrame(raw: string): Record<string, unknown> | undefined {
  try {
    const parsed: unknown = JSON.parse(raw);
    return isRecord(parsed) ? unwrapWirePayload(parsed) : undefined;
  } catch {
    return undefined;
  }
}

function isAuthOkFrame(raw: string, requestId: string): boolean {
  const frame = parseRealtimeControlFrame(raw);
  return pickString(frame?.type) === 'auth.ok'
    && (!pickString(frame?.requestId) || pickString(frame?.requestId) === requestId);
}

function readCloseReason(event: unknown): string | undefined {
  return isRecord(event) ? pickString(event.reason) : undefined;
}

export function createImLiveConnection({
  accessToken,
  auth,
  authToken,
  headerProvider,
  headers,
  options,
  tokenManager,
  websocketBaseUrl,
  webSocketFactory,
}: ImCreateLiveConnectionParams): ImLiveConnection {
  const listeners: ListenerBag = {
    errors: new Set(),
    messages: new Map(),
    states: new Set(),
  };
  const subscriptionConversations = pickStringArray(options.subscriptions?.conversations);
  const credentials = resolveWebSocketCredentials({ accessToken, auth, authToken, tokenManager });
  const url = buildWebSocketUrl(websocketBaseUrl, {
    ...options,
    subscriptions: { conversations: subscriptionConversations },
  });
  const usesBrowserWebSocket = !webSocketFactory;
  const socket = resolveWebSocketFactory(webSocketFactory)(url, {
    headers: buildWebSocketHeaders({ ...credentials, headerProvider, headers }),
    protocols: [],
  });
  const authInitRequestId = 'sdkwork-im-auth-init-1';
  const frameAuthRequired = usesBrowserWebSocket && auth?.mode !== 'none';
  const frameAuthCredentials = credentials.accessToken && credentials.authToken
    ? { accessToken: credentials.accessToken, authToken: credentials.authToken }
    : undefined;
  let awaitingAuthOk = frameAuthRequired;
  let subscriptionSyncCounter = 0;

  const emitState = (state: ImLiveConnectionState): void => {
    for (const handler of listeners.states) {
      handler(state);
    }
  };

  const emitOpenAndSyncSubscriptions = (): void => {
    emitState({ status: 'open' });
    subscriptionSyncCounter += 1;
    sendSubscriptionSync(
      socket,
      subscriptionConversations,
      `sdkwork-im-subscriptions-sync-${subscriptionSyncCounter}`,
    );
  };

  socket.addEventListener('open', () => {
    if (frameAuthRequired) {
      if (!frameAuthCredentials) {
        emitState({ status: 'error', reason: 'websocket auth tokens are not ready' });
        socket.close(4401, 'websocket auth tokens are not ready');
        return;
      }
      sendAuthInit(socket, frameAuthCredentials, options.deviceId, authInitRequestId);
      return;
    }

    emitOpenAndSyncSubscriptions();
  });
  socket.addEventListener('close', (event: unknown) => emitState({ status: 'closed', reason: readCloseReason(event) }));
  socket.addEventListener('error', (event: unknown) => {
    emitState({ status: 'error' });
    for (const handler of listeners.errors) {
      handler(event);
    }
  });
  socket.addEventListener('message', (event: unknown) => {
    const raw = extractMessageData(event);
    if (!raw) {
      return;
    }
    if (awaitingAuthOk) {
      if (isAuthOkFrame(raw, authInitRequestId)) {
        awaitingAuthOk = false;
        emitOpenAndSyncSubscriptions();
      }
      return;
    }
    const decodedMessages = parseRealtimePayloads(raw);
    for (const decoded of decodedMessages) {
      if (!decoded.context.conversationId) {
        continue;
      }
      decoded.context.ack = () => {
        if (socket.readyState === SOCKET_OPEN_STATE) {
          const ackedSeq = decoded.context.sequence;
          socket.send(JSON.stringify({
            type: 'events.ack',
            requestId: `sdkwork-im-events-ack-${ackedSeq}`,
            ackedSeq,
          }));
        }
        return Promise.resolve();
      };
      const handlers = listeners.messages.get(decoded.context.conversationId);
      if (!handlers) {
        continue;
      }
      for (const handler of handlers) {
        handler(decoded.message, decoded.context);
      }
    }
  });

  emitState({ status: 'connecting' });

  return {
    disconnect(code = 1000, reason = 'client disconnect') {
      if (socket.readyState === SOCKET_CLOSING_STATE || socket.readyState === SOCKET_CLOSED_STATE) {
        return;
      }
      socket.close(code, reason);
    },
    lifecycle: {
      onError(handler) {
        return subscribe(listeners.errors, handler);
      },
      onStateChange(handler) {
        return subscribe(listeners.states, handler);
      },
    },
    messages: {
      onConversation(conversationId, handler) {
        const handlers = listeners.messages.get(conversationId) ?? new Set();
        listeners.messages.set(conversationId, handlers);
        return subscribe(handlers, handler);
      },
    },
  };
}
