# SDKWork Craw Chat TypeScript SDK

Official package: `@sdkwork/craw-chat-sdk`

This package is the single consumer-facing TypeScript SDK for Craw Chat. It targets browser and Node.js and follows one package rule:

- semantic SDK surface lives at the package root
- generated OpenAPI transport is assembled under `src/generated/**` inside the same package
- application code should start from `CrawChatSdkClient`
- low-level generated access stays available from the same package through `SdkworkBackendClient`
  and `generated`

Authoring boundaries inside this workspace remain explicit:

- `generated/server-openapi`
  Generator-owned transport authoring output before root-package assembly.
- `composed`
  Manual-owned semantic authoring layer used before the single-package root is assembled.
- root package
  The published `@sdkwork/craw-chat-sdk` consumer package that assembles generated transport and
  handwritten runtime modules together.

## Install

```bash
npm install @sdkwork/craw-chat-sdk
```

```bash
pnpm add @sdkwork/craw-chat-sdk
```

## Quick Start

### Browser

```ts
import { CrawChatSdkClient } from '@sdkwork/craw-chat-sdk';

const sdk = new CrawChatSdkClient({
  baseUrl: import.meta.env.VITE_CRAW_CHAT_BASE_URL,
  authToken: window.localStorage.getItem('craw-chat-token') ?? undefined,
});
```

### Node.js

```ts
import { CrawChatSdkClient } from '@sdkwork/craw-chat-sdk';

const sdk = new CrawChatSdkClient({
  baseUrl: process.env.CRAW_CHAT_BASE_URL!,
  authToken: process.env.CRAW_CHAT_TOKEN,
});
```

When HTTP API traffic and realtime WebSocket traffic do not share the same origin:

```ts
const sdk = new CrawChatSdkClient({
  apiBaseUrl: 'https://api.example.com',
  websocketBaseUrl: 'wss://realtime.example.com',
  authToken: process.env.CRAW_CHAT_TOKEN,
});
```

The constructor is synchronous. Network traffic starts only when you call an explicit method such
as `sdk.auth.login()`, `sdk.send()`, `sdk.connect()`, or `sdk.sync.catchUp()`.

## Authentication

Authentication is exposed through `sdk.auth`.

```ts
const login = await sdk.auth.login({
  tenantId: 'tenant-acme',
  login: 'ops_lead',
  password: '***',
  clientKind: 'portal_operator',
});

await sdk.auth.me();

sdk.auth.useToken(login.accessToken);
sdk.auth.clearToken();
```

Behavior:

- `sdk.auth.login()` automatically applies `accessToken` when the backend returns one
- `sdk.auth.useToken(token)` updates the underlying generated backend client
- `sdk.auth.clearToken()` clears the bearer token used by HTTP and realtime helpers

## Portal

Portal snapshot reads are exposed directly on `sdk.portal`.

```ts
const home = await sdk.portal.getHome();
const workspace = await sdk.portal.getWorkspace();
const dashboard = await sdk.portal.getDashboard();

console.log(home.hero?.title, workspace.name, dashboard.hero?.title);
```

Available portal helpers:

- `sdk.portal.getHome()`
- `sdk.portal.getAuth()`
- `sdk.portal.getWorkspace()`
- `sdk.portal.getDashboard()`
- `sdk.portal.getConversations()`
- `sdk.portal.getRealtime()`
- `sdk.portal.getMedia()`
- `sdk.portal.getAutomation()`
- `sdk.portal.getGovernance()`

## Conversations

Use `sdk.conversations` when you want route-aligned access for conversation lifecycle, handoff,
membership, read-state, and direct message posting.

| Task | Method |
| --- | --- |
| Create a conversation | `sdk.conversations.create(...)` |
| Create a one-to-one agent dialog | `sdk.conversations.createAgentDialog(...)` |
| Create a handoff conversation | `sdk.conversations.createAgentHandoff(...)` |
| Create a system channel | `sdk.conversations.createSystemChannel(...)` |
| Read the conversation summary | `sdk.conversations.get(...)` |
| Read handoff state | `sdk.conversations.getAgentHandoffState(...)` |
| Accept / resolve / close handoff | `sdk.conversations.acceptAgentHandoff(...)`, `sdk.conversations.resolveAgentHandoff(...)`, `sdk.conversations.closeAgentHandoff(...)` |
| Manage members | `sdk.conversations.listMembers(...)`, `sdk.conversations.addMember(...)`, `sdk.conversations.removeMember(...)`, `sdk.conversations.transferOwner(...)`, `sdk.conversations.changeMemberRole(...)`, `sdk.conversations.leave(...)` |
| Manage read state | `sdk.conversations.getReadCursor(...)`, `sdk.conversations.updateReadCursor(...)` |
| Read and post route-level messages | `sdk.conversations.listMessages(...)`, `sdk.conversations.postMessage(...)`, `sdk.conversations.postText(...)`, `sdk.conversations.publishSystemMessage(...)`, `sdk.conversations.publishSystemText(...)` |

```ts
const conversation = await sdk.conversations.create({
  conversationId: 'conversation-order-1',
  conversationType: 'group',
});

await sdk.conversations.postText(conversation.conversationId, 'hello world', {
  summary: 'Greeting',
});

await sdk.conversations.publishSystemText(
  conversation.conversationId,
  'Maintenance starts in 5 minutes',
  {
    summary: 'Maintenance window',
  },
);

const inbox = await sdk.generated.inbox.getInbox();
console.log(conversation.conversationId, inbox.items.length);
```

## Messages

The outbound model is message-first at the client root:

1. create a message with `sdk.createXxxMessage(...)`
2. send it with `sdk.send(message)`

The same builders remain available on `sdk.messages` when you want a namespaced module surface.
The primary message entrypoints are `sdk.createTextMessage(...)`, `sdk.send(...)`,
`sdk.upload(...)`, `sdk.uploadAndSendMessage(...)`, and `sdk.decodeMessage(...)`.

### Text

```ts
const message = sdk.createTextMessage({
  conversationId: 'conversation-1',
  text: 'hello world',
  summary: 'Greeting',
  renderHints: { tone: 'friendly' },
});

await sdk.send(message);
```

### Media

```ts
const image = sdk.createImageMessage({
  conversationId: 'conversation-1',
  mediaAssetId: 'asset-image-1',
  text: 'Latest storefront concept',
  summary: 'Storefront concept',
});

await sdk.send(image);
```

### Upload And Send

```ts
const uploaded = await sdk.uploadAndSendMessage({
  upload: {
    mediaAssetId: 'asset-image-1',
    bucket: 'tenant-media',
    objectKey: 'conversation-1/storefront.png',
    resource: {
      type: 'image',
      name: 'storefront.png',
      mimeType: 'image/png',
      size: file.size,
    },
    body: file,
  },
  createMessage: (preparedUpload) =>
    sdk.createImageMessage({
      conversationId: 'conversation-1',
      mediaAssetId: preparedUpload.mediaAssetId,
      text: 'Uploaded storefront image',
      summary: 'Storefront image',
    }),
});

console.log(uploaded.mediaAssetId, uploaded.url, uploaded.delivery.messageId);
```

When you only need the presigned upload flow without sending a message immediately, use
`sdk.upload(...)`. It returns a `CrawChatUploadedMediaAsset` with the completed asset metadata,
upload session details, and the resolved CDN or download URL when the backend provides one.

```ts
const uploadedAsset = await sdk.upload({
  mediaAssetId: 'asset-image-2',
  bucket: 'tenant-media',
  objectKey: 'conversation-1/manual-upload.png',
  resource: {
    type: 'image',
    name: 'manual-upload.png',
    mimeType: 'image/png',
    size: file.size,
  },
  body: file,
});

console.log(
  uploadedAsset.mediaAssetId,
  uploadedAsset.asset.status,
  uploadedAsset.session.uploadUrl,
);
```

### Rich Standard Messages

The semantic layer includes common IM and AI-era message families:

- `createLocationMessage`
- `createLinkMessage`
- `createCardMessage`
- `createMusicMessage`
- `createContactMessage`
- `createStickerMessage`
- `createVoiceMessage`
- `createCustomMessage`
- `createAiTextMessage`
- `createAiImageGenerationMessage`
- `createAiVideoGenerationMessage`
- `createAgentMessage`
- `createAgentStateMessage`
- `createAgentHandoffMessage`
- `createToolResultMessage`
- `createWorkflowEventMessage`
- `createDataMessage`
- `createSignalMessage`
- `createStreamReferenceMessage`

Examples:

```ts
const location = sdk.createLocationMessage({
  conversationId: 'conversation-1',
  text: 'Meet me here',
  latitude: 31.2304,
  longitude: 121.4737,
  name: 'The Bund',
  address: 'Shanghai',
});

const aiText = sdk.createAiTextMessage({
  conversationId: 'conversation-1',
  text: 'Assistant answer',
  prompt: 'summarize the last order',
  model: 'gpt-5.4',
  status: 'completed',
});

const aiImage = sdk.createAiImageGenerationMessage({
  conversationId: 'conversation-1',
  text: 'Generated hero illustration',
  prompt: 'cinematic storefront at sunrise',
  model: 'gpt-image-1',
  mediaAssetId: 'asset-generated-hero-1',
  resource: {
    type: 'image',
    name: 'storefront-hero.png',
    mimeType: 'image/png',
    url: 'https://cdn.example.com/generated/storefront-hero.png',
  },
  status: 'completed',
});

const agent = sdk.createAgentMessage({
  conversationId: 'conversation-1',
  text: 'Primary support agent joined',
  agentId: 'assistant-1',
  agentName: 'Assistant',
  stage: 'active',
  status: 'online',
  capabilities: ['summarize', 'route'],
});

const handoff = sdk.createAgentHandoffMessage({
  conversationId: 'conversation-1',
  text: 'Escalating to billing specialist',
  fromAgentId: 'router',
  toAgentId: 'billing-specialist',
  reason: 'invoice_exception',
  status: 'pending',
});

await sdk.send(location);
await sdk.send(aiText);
await sdk.send(aiImage);
await sdk.send(agent);
await sdk.send(handoff);
```

### System Channel

Set `channel: 'system'` to publish into the system channel. `sdk.send(...)` will route to
the correct HTTP endpoint automatically.

```ts
const notice = sdk.createTextMessage({
  conversationId: 'conversation-system-1',
  channel: 'system',
  text: 'Deployment starts in 5 minutes',
  summary: 'Maintenance window',
});

await sdk.send(notice);
```

### Decode

If you already own a raw `MessageBody` or `PostMessageRequest`, decode it through the semantic
message codec:

```ts
const decoded = sdk.decodeMessage(aiText.body);
console.log(decoded.type, decoded.summary);
```

## Media

Use `sdk.media` for explicit upload lifecycle control.

- `sdk.media.createUpload(...)`
- `sdk.media.createUploadSession(...)`
- `sdk.media.upload(...)`
- `sdk.media.completeUpload(...)`
- `sdk.media.uploadAndComplete(...)`
- `sdk.media.getDownloadUrl(...)`
- `sdk.media.get(...)`
- `sdk.media.attach(...)`
- `sdk.media.attachText(...)`

```ts
const prepared = await sdk.media.uploadAndComplete({
  mediaAssetId: 'asset-file-1',
  bucket: 'tenant-media',
  objectKey: 'conversation-1/brief.pdf',
  resource: {
    type: 'file',
    name: 'brief.pdf',
    mimeType: 'application/pdf',
    size: file.size,
  },
  body: file,
});

await sdk.media.attachText(prepared.mediaAssetId, {
  conversationId: 'conversation-1',
  text: 'Uploaded project brief',
  summary: 'Project brief',
});
```

`sdk.upload(...)` is the preferred root helper for standard application uploads. `sdk.media.upload`
and `sdk.media.uploadAndComplete(...)` expose the same presigned S3-style client upload flow from
the namespaced media module. Use `sdk.media.createUploadSession(...)` plus
`sdk.media.completeUpload(...)` only when you need to control the upload transaction manually.

## Realtime

The SDK exposes two receive models on purpose:

- `sdk.connect(...)`
  Live push over WebSocket
- `sdk.sync.catchUp(...)`
  Durable HTTP catch-up and replay window acknowledgement

### Live Push

```ts
const live = await sdk.connect({
  deviceId: 'web-chrome-01',
  subscriptions: {
    conversations: ['conversation-1'],
    rtcSessions: ['rtc-1'],
  },
});

live.messages.on((message, context) => {
  console.log(message.type, message.summary, context.sequence);
  void context.ack();
});

live.messages.onConversation('conversation-1', (message, context) => {
  console.log(context.conversationId, message.type);
});

live.data.on((data, context) => {
  console.log(data.schemaRef, data.payload, context.sequence);
});

live.signals.on((signal, context) => {
  console.log(signal.signalType, signal.payload, context.scopeId);
});

live.signals.onRtcSession('rtc-1', (signal, context) => {
  console.log(signal.signalType, context.scopeId);
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

console.log(live.lifecycle.getState().status);
```

The recommended live callback shape is payload-first by domain stream:

- `message`, `data`, or `signal` as the first callback argument
- `context` as the second callback argument
- `context.sequence`
- `context.receivedAt`
- `context.sender`
- `context.source`
- `context.rawEvent`
- `context.ack()`

The live runtime surface is:

- `live.messages.on(...)` for the primary inbound message stream
- `live.messages.onConversation(...)` for conversation-scoped inbound message handling
- `live.data.on(...)` for non-message structured data delivery
- `live.signals.on(...)` for generic signaling delivery
- `live.signals.onRtcSession(...)` for RTC-session-scoped signaling
- `live.events.on(...)` for the normalized receive context before your app-specific routing
- `live.lifecycle.onStateChange(...)` for `connected`, `error`, and `closed` transitions
- `live.lifecycle.onError(...)` for realtime protocol and socket-level failures
- `live.lifecycle.getState()` for the latest connection snapshot

### Durable Catch-Up

```ts
const batch = await sdk.sync.catchUp({ limit: 50 });

for (const item of batch.items) {
  if (item.kind === 'message') {
    console.log(item.sequence, item.message.type, item.message.summary);
    await item.ack();
  }
}

await sdk.sync.ack(batch);
```

Use `sdk.sync` when your app needs resume, replay, background processing, or explicit ACK control.
`context.ack()` acknowledges through the current receive context, while `sdk.sync.ack(...)` lets you
commit the highest durable replay position explicitly.

### Browser And Node WebSocket Factories

If the realtime gateway requires a custom upgrade strategy, provide `webSocketFactory`.

Important: the default global `WebSocket` constructor cannot attach `Authorization` headers. In
plain browser environments, authenticated realtime usually requires an auth-capable gateway or
another runtime-specific upgrade strategy. Node.js and custom runtimes can provide that behavior
through `webSocketFactory`.

```ts
import WebSocket from 'ws';

const sdk = new CrawChatSdkClient({
  baseUrl: process.env.CRAW_CHAT_BASE_URL!,
  authToken: process.env.CRAW_CHAT_TOKEN,
  webSocketFactory: ({ url, protocols, headers }) =>
    new WebSocket(url, protocols, { headers }),
});
```

## RTC

RTC lifecycle operations are available from `sdk.rtc`.

```ts
const session = await sdk.rtc.create({
  rtcSessionId: 'rtc-1',
  conversationId: 'conversation-1',
  rtcMode: 'group_call',
});

await sdk.rtc.invite(session.rtcSessionId, {
  signalingStreamId: 'rtc-signal-1',
});

await sdk.rtc.postJsonSignal(session.rtcSessionId, 'offer', {
  signalingStreamId: 'rtc-signal-1',
  payload: {
    sdp: 'v=0...',
  },
});

const credential = await sdk.rtc.issueParticipantCredential(session.rtcSessionId, {
  participantId: 'user-1',
});

const recording = await sdk.rtc.getRecordingArtifact(session.rtcSessionId);

live.signals.onRtcSession(session.rtcSessionId, (signal, context) => {
  console.log(signal.signalType, signal.payload, context.scopeId);
});
```

Realtime RTC signals are received through `live.signals.onRtcSession(...)`. Use
`sdk.rtc.postJsonSignal(...)` for common JSON signaling,
`sdk.rtc.issueParticipantCredential(...)` for provider join credentials, and
`sdk.rtc.getRecordingArtifact(...)` for recording metadata. If you need lower-level route-group
control, use the generated transport exports described below.

## Generated Transport Access

The same package also exposes the generated OpenAPI layer.

```ts
import {
  CrawChatSdkClient,
  SdkworkBackendClient,
  createGeneratedBackendClient,
  generated,
} from '@sdkwork/craw-chat-sdk';

const sdk = new CrawChatSdkClient({
  baseUrl: 'http://127.0.0.1:18090',
  authToken: 'token',
});

const rawClient = new SdkworkBackendClient({
  baseUrl: 'http://127.0.0.1:18090',
  authToken: 'token',
});

await rawClient.conversation.listConversationMessages('conversation-1');
console.log(generated);
```

Guidance:

- start from `CrawChatSdkClient` for application code
- use `SdkworkBackendClient` for transport-level tooling or advanced gaps
- use `generated` when you need exact generated DTOs and route-group classes

## Verification

Local workspace checks:

```bash
node ./bin/assemble-single-package.mjs
node ../../../../../sdk/sdkwork-sdk-generator/node_modules/typescript/bin/tsc -p tsconfig.build.json --noEmit
node ../bin/verify-typescript-live-contract.mjs --package root
node ./test/craw-chat-client.test.mjs
node ../bin/verify-typescript-single-package-publishability.mjs
```

From the SDK workspace root, the default cross-language verification entrypoint is `sdk-verify` via
`./bin/verify-sdk.sh` or `.\bin\verify-sdk.ps1`.

Release-oriented verification also checks `npm pack --dry-run`, `verify-typescript-live-contract.mjs`, `verify-typescript-generated-build-determinism.mjs`, runtime root exports, and cleanup of dead auth scaffolding plus stray `src/index.js` and `src/index.d.ts` residue before the TypeScript package is considered publishable.
On Windows hosts, release-oriented verification also includes `verify-typescript-generated-build-concurrency.mjs` and `verify-typescript-workspace-concurrency.mjs` so both stable generated-package builds and full workspace verifications stay race-free under overlapping runs.
