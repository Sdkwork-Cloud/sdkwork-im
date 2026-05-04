# TypeScript SDK

The official TypeScript consumer package in the IM SDK family is `@sdkwork/im-sdk`.

This package is the primary app-facing SDK for browser and Node.js and follows one package rule:

- one installable package for normal consumers
- one primary client class: `ImSdkClient`
- one generated transport boundary assembled under `src/generated/**`
- one semantic SDK surface at the package root

Use `ImSdkClient` for application code. Route-aligned transport modules such as `sdk.session`,
`sdk.presence`, `sdk.realtime`, `sdk.device`, `sdk.inbox`, and `sdk.stream` are mounted directly on
the same client when you need exact OpenAPI transport control.

## Current Delivery Reality

The TypeScript standard is now intentionally narrow and explicit:

- one public consumer package: `@sdkwork/im-sdk`
- one primary public client: `ImSdkClient`
- one internal generated layer assembled from `generated/server-openapi`
- one generator-owned authoring boundary under `generated/server-openapi`
- no second backend-named public companion package

## Package Contract

| Concern | Value |
| --- | --- |
| Official package | `@sdkwork/im-sdk` |
| Primary client | `ImSdkClient` |
| Runtime targets | Browser and Node.js |
| Route-aligned transport modules | `sdk.session`, `sdk.presence`, `sdk.realtime`, `sdk.device`, `sdk.inbox`, `sdk.stream` |
| Generated source boundary | `src/generated/**` |
| Generator-owned authoring boundary | `generated/server-openapi` |

## Quick Start

### Browser

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: import.meta.env.VITE_CRAW_CHAT_BASE_URL,
  authToken: window.localStorage.getItem('craw-chat-token') ?? undefined,
});
```

### Node.js

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: process.env.CRAW_CHAT_BASE_URL!,
  authToken: process.env.CRAW_CHAT_TOKEN,
});
```

### Split API And WebSocket Origins

If HTTP and realtime upgrades terminate on different origins, keep the config flat:

```ts
const sdk = new ImSdkClient({
  apiBaseUrl: 'https://api.example.com',
  websocketBaseUrl: 'wss://realtime.example.com',
  authToken: window.localStorage.getItem('craw-chat-token') ?? undefined,
});
```

This is the preferred configuration model. You do not need a nested transport-config wrapper for
normal use.

## Authentication

Authentication is exposed through `sdk.auth`.

```ts
const login = await sdk.auth.login({
  tenantId: 'tenant-acme',
  login: 'ops_lead',
  password: '***',
  clientKind: 'portal_operator',
});

console.log(login.accessToken);

sdk.auth.useToken(login.accessToken);
await sdk.auth.me();
sdk.auth.clearToken();
```

Behavior:

- `sdk.auth.login(...)` automatically applies the returned bearer token when `accessToken` is
  present
- `sdk.auth.useToken(...)` updates the underlying generated HTTP client
- `sdk.auth.clearToken()` clears the token used by HTTP and live helpers

## Portal Snapshots

Portal snapshot reads are exposed through `sdk.portal`.

```ts
const home = await sdk.portal.getHome();
const workspace = await sdk.portal.getWorkspace();
const dashboard = await sdk.portal.getDashboard();

console.log(home.hero?.title, workspace.name, dashboard.hero?.title);
```

The TypeScript root client promotes the exact portal route group directly, so these helpers stay
stable and discoverable on the main application client.

| Snapshot | Method |
| --- | --- |
| Public landing | `sdk.portal.getHome()` |
| Sign-in page snapshot | `sdk.portal.getAuth()` |
| Workspace shell | `sdk.portal.getWorkspace()` |
| Dashboard | `sdk.portal.getDashboard()` |
| Conversations portal | `sdk.portal.getConversations()` |
| Realtime portal | `sdk.portal.getRealtime()` |
| Media portal | `sdk.portal.getMedia()` |
| Automation portal | `sdk.portal.getAutomation()` |
| Governance portal | `sdk.portal.getGovernance()` |

## Recommended Integration Order

For a new application, build against the SDK in this order:

1. Construct `ImSdkClient`
2. Authenticate with `sdk.auth`
3. Send a text message with `sdk.createTextMessage(...)` and `sdk.send(...)`
4. Add live push with `sdk.connect(...)`
5. Add durable catch-up with `sdk.sync.catchUp(...)`
6. Add upload-first media flows
7. Add custom, AI, and agent/workflow messages
8. Add RTC lifecycle and signaling

That order keeps the core app lifecycle stable before richer message and transport features are
added.

## API Reference Map

Use the TypeScript SDK as the default application surface. Jump to the route-level reference below
when you need exact OpenAPI operations, request bodies, or transport DTO details:

| App domain | Primary TypeScript SDK surface | Exact API reference |
| --- | --- | --- |
| Auth and current-session identity | `sdk.auth` | [Portal and Auth](/api-reference/app/portal-and-auth) |
| Portal shell and workspace snapshots | `sdk.portal` | [Portal and Auth](/api-reference/app/portal-and-auth) |
| Conversation lifecycle and handoff | `sdk.conversations.create`, `sdk.conversations.createAgentDialog`, `sdk.conversations.createAgentHandoff`, `sdk.conversations.get` | [Conversations](/api-reference/app/conversations) |
| Membership and read cursors | `sdk.conversations.listMembers`, `sdk.conversations.addMember`, `sdk.conversations.updateReadCursor` | [Membership and Read State](/api-reference/app/membership-and-read-state) |
| Message schemas and semantic send ergonomics | `sdk.createTextMessage(...)`, `sdk.send(...)`, `sdk.decodeMessage(...)` | [Messages](/api-reference/app/messages) |
| Upload registration, presigned client upload, completion, and attachment | `sdk.media.createUploadSession(...)`, `sdk.media.upload(...)`, `sdk.upload(...)`, `sdk.media.completeUpload(...)`, `sdk.media.attachText(...)` | [Media](/api-reference/app/media) |
| Session, presence, live subscriptions, and durable replay | `sdk.connect(...)`, `sdk.sync.catchUp(...)`, `sdk.sync.ack(...)`, `sdk.session`, `sdk.presence`, `sdk.realtime` | [Session and Realtime](/api-reference/app/session-and-realtime) |
| Device registration and sync feeds | `sdk.device.register(...)`, `sdk.device.getDeviceSyncFeed(...)` | [Device Sync](/api-reference/app/device-sync) |
| RTC lifecycle and signaling-side HTTP calls | `sdk.rtc.create(...)`, `sdk.rtc.postJsonSignal(...)`, `sdk.rtc.issueParticipantCredential(...)`, `sdk.rtc.getRecordingArtifact(...)` | [RTC](/api-reference/app/rtc) |
| Stream transport and checkpointing | `sdk.stream.open(...)`, `sdk.stream.appendStreamFrame(...)`, `sdk.stream.checkpoint(...)`, `sdk.stream.complete(...)` | [Streams](/api-reference/app/streams) |

## Conversations

`sdk.conversations` is the route-aligned domain for conversation lifecycle, membership changes,
read cursors, direct timeline access, and system-channel publishing. Use it when you want a
near-1:1 mapping with the public OpenAPI routes. Use the root message-first path
`sdk.createXxxMessage(...)` plus `sdk.send(...)` when you want the default builder-first
experience. Use `sdk.messages` when you want the same behavior through a namespaced module surface.

### Conversation Lifecycle And Handoff

| Task | Method |
| --- | --- |
| Create a regular conversation | `sdk.conversations.create(...)` |
| Create a one-to-one agent dialog | `sdk.conversations.createAgentDialog(...)` |
| Create an agent handoff conversation | `sdk.conversations.createAgentHandoff(...)` |
| Create a system channel conversation | `sdk.conversations.createSystemChannel(...)` |
| Read the current summary projection | `sdk.conversations.get(...)` |
| Read handoff state | `sdk.conversations.getAgentHandoffState(...)` |
| Accept a handoff | `sdk.conversations.acceptAgentHandoff(...)` |
| Resolve a handoff | `sdk.conversations.resolveAgentHandoff(...)` |
| Close a handoff | `sdk.conversations.closeAgentHandoff(...)` |

```ts
const conversation = await sdk.conversations.create({
  conversationId: 'conversation-order-1',
  conversationType: 'group',
});

await sdk.conversations.createAgentDialog({
  conversationId: 'conversation-agent-1',
  agentId: 'assistant-1',
});

const handoffConversation = await sdk.conversations.createAgentHandoff({
  conversationId: 'conversation-handoff-1',
  targetId: 'billing-specialist',
  targetKind: 'agent',
  handoffSessionId: 'handoff-1',
  handoffReason: 'invoice_exception',
});

await sdk.conversations.createSystemChannel({
  conversationId: 'conversation-system-1',
  subscriberId: 'user-1',
});

const summary = await sdk.conversations.get(conversation.conversationId);
const handoffState = await sdk.conversations.getAgentHandoffState(
  handoffConversation.conversationId,
);

await sdk.conversations.acceptAgentHandoff(handoffConversation.conversationId);
await sdk.conversations.resolveAgentHandoff(handoffConversation.conversationId);
await sdk.conversations.closeAgentHandoff(handoffConversation.conversationId);

console.log(summary.conversationId, handoffState.status);
```

### Membership, Read Cursor, And Direct Posting

| Task | Method |
| --- | --- |
| List visible members | `sdk.conversations.listMembers(...)` |
| Add a member | `sdk.conversations.addMember(...)` |
| Remove a member | `sdk.conversations.removeMember(...)` |
| Transfer ownership | `sdk.conversations.transferOwner(...)` |
| Change a member role | `sdk.conversations.changeMemberRole(...)` |
| Leave the conversation | `sdk.conversations.leave(...)` |
| Read the current principal cursor | `sdk.conversations.getReadCursor(...)` |
| Advance the cursor | `sdk.conversations.updateReadCursor(...)` |
| Read the route-level message timeline | `sdk.conversations.listMessages(...)` |
| Post a raw message body | `sdk.conversations.postMessage(...)` |
| Post plain text quickly | `sdk.conversations.postText(...)` |
| Publish a raw system message | `sdk.conversations.publishSystemMessage(...)` |
| Publish system text quickly | `sdk.conversations.publishSystemText(...)` |

```ts
const conversationId = conversation.conversationId;

await sdk.conversations.addMember(conversationId, {
  principalId: 'user-3',
  principalKind: 'user',
  role: 'member',
});

const members = await sdk.conversations.listMembers(conversationId);

await sdk.conversations.changeMemberRole(conversationId, {
  memberId: members.items[0]?.memberId ?? 'member-1',
  role: 'admin',
});

const cursor = await sdk.conversations.getReadCursor(conversationId);

await sdk.conversations.updateReadCursor(conversationId, {
  readSeq: cursor.readSeq,
});

const timeline = await sdk.conversations.listMessages(conversationId);

await sdk.conversations.postText(conversationId, 'Route-aligned text delivery', {
  summary: 'Route-aligned text delivery',
});

await sdk.conversations.publishSystemText(
  conversationId,
  'System maintenance starts in 5 minutes',
  {
    summary: 'Maintenance window',
  },
);

await sdk.conversations.transferOwner(conversationId, {
  memberId: members.items[0]?.memberId ?? 'member-1',
});

await sdk.conversations.removeMember(conversationId, {
  memberId: 'member-2',
});

await sdk.conversations.leave(conversationId);

console.log(timeline.items.length);
```

### Inbox

Inbox is exposed directly on the root client:

```ts
const inbox = await sdk.inbox.getInbox();
console.log(inbox.items.length);
```

## Messages

The outbound experience is message-first at the client root:

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

### Presigned Upload And Send

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

### Standard Message Families

The semantic layer includes common IM, custom business, and AI-era message families.

| Family | Method |
| --- | --- |
| Text | `createTextMessage` |
| Image / video / audio / file | `createImageMessage`, `createVideoMessage`, `createAudioMessage`, `createFileMessage` |
| Location / link / card / music / contact | `createLocationMessage`, `createLinkMessage`, `createCardMessage`, `createMusicMessage`, `createContactMessage` |
| Sticker / voice | `createStickerMessage`, `createVoiceMessage` |
| Custom business payload | `createCustomMessage` |
| Structured data / signal / stream reference | `createDataMessage`, `createSignalMessage`, `createStreamReferenceMessage` |
| AI text / AI image generation / AI video generation | `createAiTextMessage`, `createAiImageGenerationMessage`, `createAiVideoGenerationMessage` |
| Agent / agent state / handoff | `createAgentMessage`, `createAgentStateMessage`, `createAgentHandoffMessage` |
| Tool / workflow | `createToolResultMessage`, `createWorkflowEventMessage` |

Examples:

```ts
const custom = sdk.createCustomMessage({
  conversationId: 'conversation-1',
  customType: 'order.card',
  text: 'Order ready for review',
  data: {
    orderId: 'order-1001',
    amount: 128.5,
  },
});

const aiText = sdk.createAiTextMessage({
  conversationId: 'conversation-1',
  text: 'Assistant answer',
  prompt: 'summarize the last order',
  model: 'gpt-5.4',
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

const workflow = sdk.createWorkflowEventMessage({
  conversationId: 'conversation-1',
  text: 'Workflow advanced to fulfillment',
  workflowId: 'wf-1',
  eventName: 'state.changed',
  stage: 'fulfillment',
  status: 'success',
  data: {
    orderId: 'order-1001',
  },
});
```

### System Channel

Use `channel: 'system'` and send through the same `sdk.send(...)` entrypoint.

```ts
const notice = sdk.createTextMessage({
  conversationId: 'conversation-system-1',
  channel: 'system',
  text: 'Deployment starts in 5 minutes',
  summary: 'Maintenance window',
});

await sdk.send(notice);
```

### Edit, Recall, And Decode

```ts
await sdk.editTextMessage('msg-1', 'Updated content');
await sdk.recallMessage('msg-1');

const decoded = sdk.decodeMessage(aiText.body);
console.log(decoded.type, decoded.summary);
```

## Media

`sdk.media` owns the upload lifecycle and route-aligned attachment helpers. Use it when your
application needs explicit control over upload sessions, presigned client uploads, completion,
signed download URLs, or direct media attachment. Use `sdk.upload(...)` at the client root when
you want the default application upload path.

`sdk.media.createUploadSession(...)` returns a normalized `ImMediaUploadSession`.
`sdk.media.upload(...)`, `sdk.media.uploadAndComplete(...)`, and `sdk.upload(...)` return
`ImUploadedMediaAsset`, which keeps the semantic upload helper stable while still exposing the
generated upload-session data you need for advanced diagnostics.

| Task | Method |
| --- | --- |
| Create a presigned upload session | `sdk.media.createUploadSession(...)` |
| Route-level upload-session alias | `sdk.media.createUpload(...)` |
| Run the standard client upload flow | `sdk.media.upload(...)` or `sdk.upload(...)` |
| Run the namespaced upload-and-complete alias | `sdk.media.uploadAndComplete(...)` |
| Mark an upload complete | `sdk.media.completeUpload(...)` |
| Resolve a signed download URL | `sdk.media.getDownloadUrl(...)` |
| Read the asset metadata | `sdk.media.get(...)` |
| Attach an asset with a raw request body | `sdk.media.attach(...)` |
| Attach an asset with plain text | `sdk.media.attachText(...)` |

```ts
const uploaded = await sdk.media.uploadAndComplete({
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

await sdk.media.attachText(uploaded.mediaAssetId, {
  conversationId: 'conversation-1',
  text: 'Uploaded project brief',
  summary: 'Project brief',
});

const download = await sdk.media.getDownloadUrl(uploaded.mediaAssetId, {
  expiresInSeconds: 600,
});

const asset = await sdk.media.get(uploaded.mediaAssetId);

console.log(uploaded.url, download.downloadUrl, asset.processingState);
```

`sdk.upload(...)` is the preferred root shortcut for normal application code. Use
`sdk.media.uploadAndComplete(...)` when you want the same presigned upload behavior from the
namespaced media module, and use `sdk.media.createUploadSession(...)` plus
`sdk.media.completeUpload(...)` only when you need to control the presigned transaction manually.

## Realtime Model

The SDK intentionally separates live push from durable catch-up.

| Need | Use |
| --- | --- |
| Live WebSocket push | `sdk.connect(...)` |
| Durable resume / background replay / explicit ACK | `sdk.sync.catchUp(...)` and `sdk.sync.ack(...)` |

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

The recommended receive surface is payload-first by domain stream. Your callback receives the final
semantic object first and the operational receive context second. That keeps rendering and business
logic focused on `message`, `data`, or `signal`, while `context` remains available for sequencing,
sender metadata, raw-event inspection, and `context.ack()`.

Each context gives you:

- the semantic payload: `message`, `data`, or `signal`
- `sequence`
- `receivedAt`
- `sender`
- `source`
- `rawEvent`
- `context.ack()`

Use the live domain streams this way:

- `live.messages.on(...)` for the primary inbound message stream
- `live.messages.onConversation(...)` for conversation-scoped message handling
- `live.data.on(...)` for non-message structured data delivery
- `live.signals.on(...)` for generic signaling delivery
- `live.signals.onRtcSession(...)` for RTC-session-scoped signaling
- `live.events.on(...)` for the normalized receive context before app-specific routing
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

Use `sdk.sync` for resume, replay, reconciliation, worker consumption, or when your application
cannot rely on a continuously open WebSocket connection. `context.ack()` advances acknowledgement
through the current receive context, while `sdk.sync.ack(...)` lets you commit the highest durable
replay position explicitly.

### Browser And Node.js WebSocket Factories

If your realtime gateway needs a custom upgrade strategy, pass `webSocketFactory` at client
construction time.

Important: the default global `WebSocket` constructor cannot attach `Authorization` headers. In
plain browser environments, authenticated realtime should use a browser-safe credential path:

- `ImWebSocketAuthOptions.automatic()` is the standard TypeScript default
- automatic auth resolves to query bearer for the default browser `WebSocket` constructor
- automatic auth resolves to header bearer when a custom `webSocketFactory` is present
- prefer exchanging the primary access token for a short-lived realtime ticket or query credential
- prefer `wss://` plus short-lived credentials over long-lived query bearer tokens
- use `sdk.connect({ url })` when the gateway returns a pre-signed realtime URL

Node.js and custom runtimes can provide header-based upgrades through `webSocketFactory`.

```ts
import { ImSdkClient, ImWebSocketAuthOptions } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: 'https://api.example.com',
  authToken: window.localStorage.getItem('craw-chat-token') ?? undefined,
  webSocketAuth: ImWebSocketAuthOptions.queryBearer({
    queryParameterName: 'rt',
    credentialProvider: async ({ authToken }) =>
      issueRealtimeTicket(authToken),
  }),
});

const live = await sdk.connect({
  subscriptions: {
    conversations: ['conversation-1'],
  },
});
```

```ts
const realtimeTicket = await issueRealtimeTicket();

const live = await sdk.connect({
  url: `wss://realtime.example.com/api/v1/realtime/ws?rt=${encodeURIComponent(realtimeTicket)}`,
  subscriptions: {
    conversations: ['conversation-1'],
  },
});
```

```ts
import WebSocket from 'ws';

const sdk = new ImSdkClient({
  baseUrl: process.env.CRAW_CHAT_BASE_URL!,
  authToken: process.env.CRAW_CHAT_TOKEN,
  webSocketAuth: ImWebSocketAuthOptions.headerBearer(),
  webSocketFactory: ({ url, protocols, headers }) =>
    new WebSocket(url, protocols, { headers }),
});
```

## RTC

RTC lifecycle stays in `sdk.rtc`, while inbound signaling arrives through the live runtime.

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

Handle incoming RTC signaling through `live.signals.onRtcSession(...)`. Use
`sdk.rtc.postJsonSignal(...)` for common JSON signaling, `sdk.rtc.issueParticipantCredential(...)`
for provider join credentials, and `sdk.rtc.getRecordingArtifact(...)` for recording metadata.

## Route-Aligned Transport Modules

Some route groups are intentionally exposed as direct transport modules on `ImSdkClient` because
they already match the public OpenAPI contract cleanly. Use the higher-level semantic modules for
chat, live receive, media, and RTC workflows. Use the root transport modules when you need exact
route-group control for session, presence, realtime coordination, device registration, inbox, or
stream transport.

```ts
import { ImSdkClient } from '@sdkwork/im-sdk';

const sdk = new ImSdkClient({
  baseUrl: 'http://127.0.0.1:18090',
  authToken: 'token',
});

await sdk.session.resume({ deviceId: 'web-chrome-01' });
await sdk.presence.getPresenceMe();
await sdk.realtime.listRealtimeEvents({ limit: 20 });
await sdk.conversations.listMessages('conversation-1');
await sdk.device.register({ deviceId: 'web-chrome-01' });
await sdk.inbox.getInbox();
await sdk.stream.open({
  streamId: 'stream-demo-1',
  streamType: 'custom.delta.text',
  scopeKind: 'conversation',
  scopeId: 'conversation-1',
  durabilityClass: 'durableSession',
  schemaRef: 'custom.delta.text.v1',
});
```

Use the root transport modules when you need exact DTOs or route-group control. Reach for
`sdk.session.resume(...)`, `sdk.presence.getPresenceMe()`, `sdk.realtime.listRealtimeEvents(...)`,
`sdk.device.register(...)`, `sdk.inbox.getInbox()`, and `sdk.stream.open(...)` when the route group
already matches the API cleanly. Use the semantic domains on `ImSdkClient` for normal application
integration.

## Assembly Metadata

The TypeScript workspace publishes its contract into `.sdkwork-assembly.json`.

Use that file when you need the verified package-layer picture instead of guessing from folder
names:

- `generatedAt` tells you whether the assembly content actually changed
- `manifestPath` records the manifest that defined each package layer
- TypeScript records `generated`, `composed`, and `root` package layers together
- the internal generated layer is assembled from `generated/server-openapi` and emitted under `src/generated/**`
- the public consumer layer is the root `@sdkwork/im-sdk` package

## Local Workspace Workflow

When you maintain the checked-in TypeScript workspace locally, use the folders for their exact
roles:

- `generated/server-openapi`
  Generator-owned internal build workspace that materializes the assembled transport layer
- `composed`
  Manual-owned authoring boundary that imports the generated layer through the internal alias
- repository package root
  The assembled single-package consumer output published as `@sdkwork/im-sdk`

If you are debugging package metadata or release layout, start from `.sdkwork-assembly.json` before
editing manifests by hand.

## Verification

Local verification from the TypeScript workspace:

```bash
node ../bin/verify-typescript-workspace.mjs
node ./bin/assemble-single-package.mjs
node ../../../../../sdk/sdkwork-sdk-generator/node_modules/typescript/bin/tsc -p tsconfig.build.json --noEmit
node ./test/craw-chat-client.test.mjs
```

From the repository root, the family-level entrypoint is:

```bash
node ./sdks/sdkwork-im-sdk/bin/verify-sdk.mjs --language typescript
```

## What To Read Next

- Read [App SDK](/sdk/app-sdk) when you want the product-facing family rules above the
  TypeScript-specific details.
- Read [Flutter SDK](/sdk/flutter-sdk) when you need the current secondary consumer baseline and
  its parity gaps.
- Read [Language Support](/sdk/language-support) when you need the full cross-language maturity
  matrix.
- Read [Generator Boundary](/sdk/generator-boundary) when you need the exact split between
  `src/generated/**`, `generated/server-openapi`, and `composed`.
- Read [Portal and Auth](/api-reference/app/portal-and-auth) when you need the underlying HTTP
  contract for auth and portal snapshots.
- Read [Messages](/api-reference/app/messages), [Session and Realtime](/api-reference/app/session-and-realtime),
  and [RTC](/api-reference/app/rtc) when you need the route-level contract behind the semantic
  TypeScript SDK.
