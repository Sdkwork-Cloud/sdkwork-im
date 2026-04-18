# TypeScript Quick Start

## Audience

Use this page when you are integrating Craw Chat from a TypeScript or JavaScript application and
want the preferred composed SDK surface.

## Package

- preferred public package: `@sdkwork/craw-chat-sdk`
- generated transport package: `@sdkwork/craw-chat-backend-sdk`
- workspace path: `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript/composed`

## Install

If you are consuming from a local repository checkout before public publication, wire the package as
a local file or workspace dependency:

```json
{
  "dependencies": {
    "@sdkwork/craw-chat-sdk": "file:../../sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript/composed"
  }
}
```

## Create a client

```ts
import { CrawChatClient, type SdkworkBackendConfig } from "@sdkwork/craw-chat-sdk";

const backendConfig: SdkworkBackendConfig = {
  baseUrl: "http://127.0.0.1:18090",
  authToken: process.env.CRAW_CHAT_TOKEN,
};

const client = await CrawChatClient.create({ backendConfig });
```

## First read call

```ts
const inbox = await client.inbox.list();
```

## First write call

```ts
await client.devices.register({
  deviceId: "device-web-01",
});
```

## Common module entrypoints

```ts
await client.session.resume({ deviceId: "device-web-01", lastSeenSyncSeq: 0 });
await client.presence.current();
await client.realtime.pullEvents({ afterSeq: 0, limit: 50 });
await client.conversations.postText("conv-demo-01", "hello from TypeScript");
```

## Builder helpers

The package also exports convenience helpers:

```ts
import {
  buildTextMessageRequest,
  buildTextFrameRequest,
  buildJsonRtcSignalRequest,
} from "@sdkwork/craw-chat-sdk";
```

## Next Steps

- [Auth and Client Init](/sdk/auth-and-client-init)
- [Module Map](/sdk/module-map)
- [Messages Module](/sdk/modules/messages)
- [Message and Media](/sdk/examples/message-and-media)
