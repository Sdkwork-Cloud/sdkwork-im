# @sdkwork/craw-chat-sdk composed authoring layer

This directory is the manual-owned authoring source that assembles into the publishable root
package `@sdkwork/craw-chat-sdk`.

It is not a second consumer package. The consumer contract lives at the root TypeScript package.

## What This Layer Owns

- semantic SDK modules above the generated OpenAPI transport
- `CrawChatSdkClient` orchestration for auth, portal snapshots, messages, live, sync, media, and RTC
- message builders, message decoding, and receive-context normalization
- smoke tests that define the public TypeScript contract before assembly

`generated/server-openapi` remains generator-owned. `composed` remains manual-owned.

## Authoring Rules

- Import generated DTOs only through `src/generated-backend-types.ts`.
- Keep WebSocket lifecycle, live state handling, and receive ergonomics in the manual layer.
- Do not reintroduce the removed TypeScript compatibility surface:
  - `CrawChatClient`
  - `CrawChatSdkClient.create(...)`
  - `createReceiver()`
  - `createWebSocketReceiver()`

## Public Contract Summary

The TypeScript consumer experience that this layer must preserve is:

```ts
import { CrawChatSdkClient } from '@sdkwork/craw-chat-sdk';

const sdk = new CrawChatSdkClient({
  apiBaseUrl: 'https://api.example.com',
  websocketBaseUrl: 'wss://realtime.example.com',
  authToken: '<token>',
});

const workspace = await sdk.portal.getWorkspace();
console.log(workspace.name);

const live = await sdk.connect({
  deviceId: 'web-chrome-01',
  subscriptions: {
    conversations: ['conversation-1'],
  },
});

live.messages.on((message, context) => {
  console.log(message.type, message.summary, context.sequence);
});

live.events.on((context) => {
  console.log(context.kind, context.sequence, context.source);
});

live.lifecycle.onStateChange((state) => {
  console.log(state.status);
});

live.lifecycle.onError((context) => {
  console.log(context.code, context.error);
});

const text = sdk.createTextMessage({
  conversationId: 'conversation-1',
  text: 'hello world',
});

await sdk.send(text);
```

The public shape is intentionally:

- auth and portal promoted to the root client: `sdk.auth` and `sdk.portal`
- message-first on send: `sdk.createXxxMessage(...)` and `sdk.send(...)`
- payload-first on receive: `live.messages.on(...)`, `live.events.on(...)`, `live.lifecycle.onStateChange(...)`, and `live.lifecycle.onError(...)`
- split live and durable replay: `sdk.connect(...)` and `sdk.sync.catchUp(...)`
- flat configuration: `baseUrl`, `apiBaseUrl`, `websocketBaseUrl`, `authToken`

## Browser Realtime Note

The default global `WebSocket` constructor cannot attach `Authorization` headers. That means:

- Node.js and custom runtimes can provide `webSocketFactory` directly
- browser apps need an auth-capable realtime gateway or another runtime-specific upgrade strategy
- docs must not promise bearer-auth realtime over the default browser `WebSocket` path without that extra infrastructure

## Assemble The Root Package

After authoring changes here, rebuild the root package with:

```powershell
node ..\bin\assemble-single-package.mjs
```

## Verification

Local verification used in this workspace:

- `node ../../../../../../sdk/sdkwork-sdk-generator/node_modules/typescript/bin/tsc -p tsconfig.build.json --noEmit`
- `node ../../../../../../sdk/sdkwork-sdk-generator/node_modules/typescript/bin/tsc -p tsconfig.build.json`
- `node ./bin/clean-dist.mjs`
- `node ./test/craw-chat-client.test.mjs`
- `node ../bin/assemble-single-package.mjs`
