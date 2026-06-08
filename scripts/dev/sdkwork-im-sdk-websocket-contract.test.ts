import assert from 'node:assert/strict';
import {
  ImSdkClient,
  ImWebSocketAuthOptions,
  type ImDecodedMessage,
  type ImMessageContext,
  type ImWebSocketFactoryOptions,
  type ImWebSocketLike,
} from '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index';

type FakeWebSocketEventName = 'close' | 'error' | 'message' | 'open';

class FakeWebSocket implements ImWebSocketLike {
  readonly sent: string[] = [];
  readonly url: string;
  readonly options: ImWebSocketFactoryOptions;
  readyState = 0;

  private readonly listeners = new Map<FakeWebSocketEventName, Set<(event: unknown) => void>>();

  constructor(url: string, options: ImWebSocketFactoryOptions) {
    this.url = url;
    this.options = options;
  }

  addEventListener(type: FakeWebSocketEventName, handler: (event: unknown) => void): void {
    const listeners = this.listeners.get(type) ?? new Set();
    listeners.add(handler);
    this.listeners.set(type, listeners);
  }

  close(code?: number, reason?: string): void {
    this.readyState = 3;
    this.emit('close', { code, reason });
  }

  emit(type: FakeWebSocketEventName, event: unknown): void {
    for (const handler of this.listeners.get(type) ?? []) {
      handler(event);
    }
  }

  open(): void {
    this.readyState = 1;
    this.emit('open', {});
  }

  send(value: string): void {
    this.sent.push(value);
  }
}

class FakeBrowserWebSocket implements ImWebSocketLike {
  static readonly instances: FakeBrowserWebSocket[] = [];

  readonly sent: string[] = [];
  readonly protocols?: string | string[];
  readonly url: string;
  readyState = 0;

  private readonly listeners = new Map<FakeWebSocketEventName, Set<(event: unknown) => void>>();

  constructor(url: string, protocols?: string | string[]) {
    this.url = url;
    this.protocols = protocols;
    FakeBrowserWebSocket.instances.push(this);
  }

  addEventListener(type: FakeWebSocketEventName, handler: (event: unknown) => void): void {
    const listeners = this.listeners.get(type) ?? new Set();
    listeners.add(handler);
    this.listeners.set(type, listeners);
  }

  close(code?: number, reason?: string): void {
    this.readyState = 3;
    this.emit('close', { code, reason });
  }

  emit(type: FakeWebSocketEventName, event: unknown): void {
    for (const handler of this.listeners.get(type) ?? []) {
      handler(event);
    }
  }

  open(): void {
    this.readyState = 1;
    this.emit('open', {});
  }

  send(value: string): void {
    this.sent.push(value);
  }
}

function parseSent(socket: FakeWebSocket, index: number): Record<string, unknown> {
  const raw = socket.sent[index];
  assert.equal(typeof raw, 'string', `expected websocket frame ${index} to be sent`);
  return JSON.parse(raw) as Record<string, unknown>;
}

function parseBrowserSent(socket: FakeBrowserWebSocket, index: number): Record<string, unknown> {
  const raw = socket.sent[index];
  assert.equal(typeof raw, 'string', `expected browser websocket frame ${index} to be sent`);
  return JSON.parse(raw) as Record<string, unknown>;
}

async function main(): Promise<void> {
  const originalWebSocket = Object.getOwnPropertyDescriptor(globalThis, 'WebSocket');
  Object.defineProperty(globalThis, 'WebSocket', {
    configurable: true,
    value: undefined,
  });

  try {
    const sockets: FakeWebSocket[] = [];
    const client = new ImSdkClient({
      accessToken: 'access-token-1',
      authToken: 'auth-token-1',
      headerProvider: () => ({
        'X-Sdkwork-Tenant-Id': 'tenant-1',
        'X-Sdkwork-User-Id': 'user-1',
        'X-Sdkwork-Device-Id': 'device-1',
      }),
      webSocketAuth: ImWebSocketAuthOptions.automatic({
        credentialProvider: () => 'auth-token-1',
      }),
      webSocketFactory: (url, options) => {
        const socket = new FakeWebSocket(url, options);
        sockets.push(socket);
        return socket;
      },
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });

    const connection = await client.connect({
      deviceId: 'device-1',
      subscriptions: {
        conversations: ['conversation-1', 'conversation-2'],
      },
    });

    assert.equal(sockets.length, 1, 'IM SDK must use the injected websocket factory in non-browser runtimes');
    const socket = sockets[0];
    assert.equal(
      socket.url,
      'wss://chat.example.com/sdkwork/chat/im/v3/api/realtime/ws?deviceId=device-1',
      'IM websocket URL must preserve deployment base paths and keep subscriptions out of the handshake query',
    );
    assert.equal(socket.url.includes('conversationId='), false, 'IM websocket subscriptions must be sent as frames');
    assert.deepEqual(
      socket.options.protocols,
      [],
      'legacy JSON realtime mode must not leak auth tokens through websocket subprotocol names',
    );
    assert.deepEqual(socket.options.headers, {
      Authorization: 'Bearer auth-token-1',
      'Access-Token': 'access-token-1',
      'X-Sdkwork-Tenant-Id': 'tenant-1',
      'X-Sdkwork-User-Id': 'user-1',
      'X-Sdkwork-Device-Id': 'device-1',
    });

    const lifecycleStates: string[] = [];
    const received: Array<{ context: ImMessageContext; message: ImDecodedMessage }> = [];
    connection.lifecycle.onStateChange((state) => lifecycleStates.push(state.status));
    connection.messages.onConversation('conversation-1', (message, context) => {
      received.push({ context, message });
    });

    socket.open();

    assert.deepEqual(lifecycleStates, ['open']);
    assert.deepEqual(parseSent(socket, 0), {
      type: 'subscriptions.sync',
      requestId: 'sdkwork-im-subscriptions-sync-1',
      items: [
        {
          scopeType: 'conversation',
          scopeId: 'conversation-1',
          eventTypes: ['message.posted'],
        },
        {
          scopeType: 'conversation',
          scopeId: 'conversation-2',
          eventTypes: ['message.posted'],
        },
      ],
    });

    socket.emit('message', {
      data: JSON.stringify({
        type: 'event.window',
        requestId: null,
        reason: 'push',
        window: {
          deviceId: 'device-1',
          items: [
            {
              eventId: 'event-1',
              eventType: 'message.posted',
              occurredAt: '2026-06-06T10:00:00.000Z',
              payload: JSON.stringify({
                body: {
                  parts: [{ kind: 'text', text: 'hello over websocket' }],
                  renderHints: {
                    sdkworkChatPcType: 'text',
                  },
                  summary: 'hello over websocket',
                },
                conversationId: 'conversation-1',
                deliveryMode: 'discrete',
                messageId: 'message-1',
                messageSeq: 42,
                messageType: 'standard',
                occurredAt: '2026-06-06T10:00:00.000Z',
                sender: {
                  id: 'user-2',
                  kind: 'user',
                  metadata: {},
                },
                summary: 'hello over websocket',
              }),
              realtimeSeq: 7,
              scope: 'conversation',
              scopeId: 'conversation-1',
            },
          ],
          nextAfterSeq: 7,
        },
      }),
    });

    assert.equal(received.length, 1, 'IM SDK must decode realtime event windows into conversation messages');
    assert.equal(received[0].context.eventId, 'event-1');
    assert.equal(received[0].context.conversationId, 'conversation-1');
    assert.equal(received[0].context.sequence, 7);
    assert.equal(received[0].message.messageId, 'message-1');
    assert.equal(received[0].message.summary, 'hello over websocket');
    assert.deepEqual(received[0].message.content, {
      body: {
        parts: [{ kind: 'text', text: 'hello over websocket' }],
        renderHints: {
          sdkworkChatPcType: 'text',
        },
        summary: 'hello over websocket',
      },
      conversationId: 'conversation-1',
      deliveryMode: 'discrete',
      messageId: 'message-1',
      messageSeq: 42,
      messageType: 'standard',
      occurredAt: '2026-06-06T10:00:00.000Z',
      sender: {
        id: 'user-2',
        kind: 'user',
        metadata: {},
      },
      summary: 'hello over websocket',
    });

    await received[0].context.ack();
    assert.deepEqual(parseSent(socket, 1), {
      type: 'events.ack',
      requestId: 'sdkwork-im-events-ack-7',
      ackedSeq: 7,
    });

    connection.disconnect(1000, 'test complete');
    assert.equal(socket.readyState, 3);

    FakeBrowserWebSocket.instances.length = 0;
    Object.defineProperty(globalThis, 'WebSocket', {
      configurable: true,
      value: FakeBrowserWebSocket,
    });

    const browserClient = new ImSdkClient({
      accessToken: 'browser-access-token-1',
      authToken: 'browser-auth-token-1',
      webSocketAuth: ImWebSocketAuthOptions.automatic({
        credentialProvider: () => 'browser-auth-token-1',
      }),
      websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat/',
    });
    const browserConnection = await browserClient.connect({
      deviceId: 'browser-device-1',
      subscriptions: {
        conversations: ['browser-conversation-1'],
      },
    });
    assert.equal(FakeBrowserWebSocket.instances.length, 1, 'browser runtime must use global WebSocket');
    const browserSocket = FakeBrowserWebSocket.instances[0];
    assert.equal(
      browserSocket.url,
      'wss://chat.example.com/sdkwork/chat/im/v3/api/realtime/ws?deviceId=browser-device-1',
      'browser websocket URL must not include subscription identifiers',
    );
    assert.deepEqual(browserSocket.protocols, []);

    const browserStates: string[] = [];
    browserConnection.lifecycle.onStateChange((state) => browserStates.push(state.status));

    browserSocket.open();
    assert.deepEqual(browserStates, [], 'browser websocket must wait for auth.ok before reporting open');
    assert.deepEqual(parseBrowserSent(browserSocket, 0), {
      type: 'auth.init',
      requestId: 'sdkwork-im-auth-init-1',
      authToken: 'browser-auth-token-1',
      accessToken: 'browser-access-token-1',
      deviceId: 'browser-device-1',
    });
    assert.equal(
      browserSocket.sent.some((frame) => frame.includes('subscriptions.sync')),
      false,
      'browser websocket must not send subscriptions before auth.ok',
    );

    browserSocket.emit('message', {
      data: JSON.stringify({
        type: 'auth.ok',
        requestId: 'sdkwork-im-auth-init-1',
        tenantId: 'tenant_real',
        principalId: 'user_real',
        sessionId: 'session_real',
        deviceId: 'browser-device-1',
      }),
    });
    assert.deepEqual(browserStates, ['open']);
    assert.deepEqual(parseBrowserSent(browserSocket, 1), {
      type: 'subscriptions.sync',
      requestId: 'sdkwork-im-subscriptions-sync-1',
      items: [
        {
          scopeType: 'conversation',
          scopeId: 'browser-conversation-1',
          eventTypes: ['message.posted'],
        },
      ],
    });

    browserConnection.disconnect(1000, 'browser test complete');
  } finally {
    if (originalWebSocket) {
      Object.defineProperty(globalThis, 'WebSocket', originalWebSocket);
    } else {
      Reflect.deleteProperty(globalThis, 'WebSocket');
    }
  }

  console.log('sdkwork-im-sdk websocket contract passed');
}

void main();
