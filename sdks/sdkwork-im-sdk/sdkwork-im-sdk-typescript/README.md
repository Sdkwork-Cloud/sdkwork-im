# @sdkwork/im-sdk

Layered TypeScript SDK for Craw Chat IM runtime APIs.

This package exposes the app-facing `ImSdkClient` facade. Generated HTTP
transport is consumed through `@sdkwork/im-sdk-generated`, whose source remains
under `generated/server-openapi`; realtime websocket behavior and semantic
convenience modules live in `src/`.

## Realtime Websocket

`ImSdkClient.connect()` opens `/im/v3/api/realtime/ws` through the authored
websocket adapter. The adapter preserves deployment base paths, for example
`wss://chat.example.com/sdkwork/chat` becomes
`wss://chat.example.com/sdkwork/chat/im/v3/api/realtime/ws`.

Browser runtimes use `globalThis.WebSocket`. Browser websocket APIs cannot set
custom request headers, so authenticated browser connections send the current
dual-token session in the first `auth.init` frame. The gateway validates that
frame through appbase current-session, projects trusted AppContext headers to
session-gateway, responds with `auth.ok`, and only then does the SDK send
subscription frames.

The handshake URL never carries auth tokens or conversation subscriptions. A
browser connection starts with:

```json
{
  "type": "auth.init",
  "requestId": "sdkwork-im-auth-init-1",
  "authToken": "<auth-token>",
  "accessToken": "<access-token>",
  "deviceId": "<device-id>"
}
```

After `auth.ok`, subscriptions are synchronized with `subscriptions.sync`
frames. `conversationId` values must not be appended to the websocket URL.

Node, Tauri, tests, and other host runtimes can inject a transport:

```ts
import { ImSdkClient, type ImWebSocketFactory } from '@sdkwork/im-sdk';

const webSocketFactory: ImWebSocketFactory = (url, options) => {
  return new NodeWebSocket(url, options.protocols, {
    headers: options.headers,
  });
};

const client = new ImSdkClient({
  authToken,
  accessToken,
  headerProvider: () => ({
    'X-Sdkwork-Tenant-Id': tenantId,
    'X-Sdkwork-User-Id': userId,
    'X-Sdkwork-Device-Id': deviceId,
  }),
  webSocketFactory,
  websocketBaseUrl: 'wss://chat.example.com/sdkwork/chat',
});

const connection = await client.connect({
  connectionTimeoutMs: 15_000,
  deviceId,
  heartbeat: {
    intervalMs: 30_000,
    timeoutMs: 75_000,
  },
  subscriptions: {
    conversations: [conversationId],
  },
});

connection.messages.onConversation(conversationId, async (message, context) => {
  await context.ack();
});

connection.subscriptions.syncConversations([
  conversationId,
  anotherConversationId,
]);
```

The default wire mode is the backend legacy JSON websocket protocol:

- Browser mode sends `auth.init` first and waits for `auth.ok` before reporting the connection open.
- Node/Tauri injected websocket factories may still forward `Authorization`, `Access-Token`, and AppContext headers for trusted host runtimes.
- `subscriptions.sync` is sent as a websocket frame for requested conversation subscriptions. Call `connection.subscriptions.syncConversations(...)` to replace the active conversation subscription snapshot on an existing connection.
- A connection that never reaches the native websocket `open` event emits `websocket_connect_timeout` and queues a close for the first safe moment. Application connection managers should treat this as a reconnectable transport error.
- Heartbeats are enabled by default. The SDK sends `ping` frames and treats any inbound frame as liveness; if the socket stays silent past `timeoutMs`, it emits `websocket_heartbeat_timeout` and closes the connection so the app-level connection manager can reconnect. Pass `heartbeat: false` only for tests or runtimes with their own equivalent liveness checks.
- Server `error` control frames are surfaced through `connection.lifecycle.onError(...)`. Fatal websocket auth, session, token, upstream, and connect errors also move the connection into `error` state and close the socket; non-fatal subscription or business errors leave the socket open.
- `event.window` frames are decoded into `messages.onConversation(...)` callbacks.
- `context.ack()` sends `events.ack` with the received realtime sequence.
- Auth tokens are not sent as websocket URL query parameters or subprotocol names.

Do not hand-edit `generated/server-openapi`. Update the OpenAPI authority or
authored facade source, then run:

```bash
node ../bin/generate-sdk.mjs --language typescript
```
