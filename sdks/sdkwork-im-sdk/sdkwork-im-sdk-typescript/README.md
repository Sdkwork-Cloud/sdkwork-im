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
custom request headers, so authenticated browser connections rely on the
gateway/session AppContext that served the app.

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
  deviceId,
  subscriptions: {
    conversations: [conversationId],
  },
});

connection.messages.onConversation(conversationId, async (message, context) => {
  await context.ack();
});
```

The default wire mode is the backend legacy JSON websocket protocol:

- `subscriptions.sync` is sent after the socket opens for requested conversation subscriptions.
- `event.window` frames are decoded into `messages.onConversation(...)` callbacks.
- `context.ack()` sends `events.ack` with the received realtime sequence.
- Auth tokens are not sent as websocket subprotocol names.

Do not hand-edit `generated/server-openapi`. Update the OpenAPI authority or
authored facade source, then run:

```bash
node ../bin/generate-sdk.mjs --language typescript
```
