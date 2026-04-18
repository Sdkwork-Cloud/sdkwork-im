# App SDK

The app SDK family is the consumer-facing SDK surface for the public Craw Chat application runtime.

It is responsible for:

- authentication and session identity
- conversations and outbound messages
- media upload and attachment
- live realtime receive
- durable catch-up and ACK
- RTC lifecycle and signaling

## Choose This Family When

- you are building a browser or Node.js app runtime and need the strongest checked-in semantic SDK
- you are building a Flutter app runtime and want the route-aligned Dart surface above the
  generated package
- you need app-facing auth, portal, message, media, realtime, or RTC workflows
- you want to start from a product-facing SDK instead of raw route-group transport

## Do Not Start Here When

- you only need a generated transport artifact for a non-TypeScript language and do not need a
  checked-in semantic client yet
- you are building governance or control-plane tooling
- you are documenting or automating protocol registry, provider policy, or node lifecycle flows
- you are looking for the admin surface

Use [Admin SDK](/sdk/admin-sdk) for governance or control-plane tooling.

## Fastest Onboarding

For a new integration, read the app SDK pages in this order:

1. [Auth and Client Init](/sdk/auth-and-client-init)
2. one language quick start:
   [TypeScript](/sdk/typescript-quick-start),
   [Flutter](/sdk/flutter-quick-start),
   [Rust](/sdk/rust-quick-start)
3. [Module Map](/sdk/module-map)
4. the capability pages under `/sdk/modules/`
5. the matching route-level API reference pages when you need exact DTOs or statuses

## Workspace Standard

| Layer | Current standard |
| --- | --- |
| SDK workspace root | `sdks/sdkwork-craw-chat-sdk` |
| Live schema export | `/openapi/craw-chat-app.openapi.yaml` |
| Authority snapshot | `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml` |
| Official consumer package | `@sdkwork/craw-chat-sdk` |
| TypeScript consumer package | `@sdkwork/craw-chat-sdk` |
| Flutter consumer package | `craw_chat_sdk` |
| Rust generated crate | `sdkwork-craw-chat-backend-sdk` |
| Primary app-facing client today | `CrawChatSdkClient` in TypeScript |
| Generator-owned authoring source | `generated/server-openapi` |
| Manual-owned semantic boundary | `composed`, except the assembled TypeScript root package that exposes generated transport under `src/generated/**` |

The current implementation priority is TypeScript. Flutter remains in the workspace, but the
next-generation app SDK standard is being landed in TypeScript first.

## Recommended Integration Order

For a new app:

1. construct `new CrawChatSdkClient({...})`
2. authenticate through `sdk.auth`
3. send one text message through `sdk.createTextMessage(...)` and `sdk.send(...)`
4. establish live push with `sdk.connect(...)`
5. add durable replay through `sdk.sync.catchUp(...)`
6. add upload-first media flows
7. add custom, AI, and agent/workflow messages
8. add RTC lifecycle and signaling

That is the recommended product integration path. Do not start from raw route groups unless you
are explicitly building transport tooling.

Use the scenario pages when you want a shorter working example instead of a capability catalog:

- [Session Bootstrap](/sdk/examples/session-bootstrap)
- [Conversation Workflow](/sdk/examples/conversation-workflow)
- [Message and Media](/sdk/examples/message-and-media)
- [Stream and RTC](/sdk/examples/stream-and-rtc)

## TypeScript Consumer Model

The TypeScript package is a single installable SDK that combines:

- semantic app modules at the root package
- generated OpenAPI transport in the same package
- browser and Node.js support
- split `apiBaseUrl` and `websocketBaseUrl`
- payload-first domain receive APIs

```ts
import { CrawChatSdkClient } from '@sdkwork/craw-chat-sdk';

const sdk = new CrawChatSdkClient({
  baseUrl: import.meta.env.VITE_CRAW_CHAT_BASE_URL,
  authToken: window.localStorage.getItem('craw-chat-token') ?? undefined,
});

await sdk.auth.me();
const workspace = await sdk.portal.getWorkspace();
console.log(workspace.name);
```

## App Runtime Domains

| Domain | Primary entry |
| --- | --- |
| Auth | `sdk.auth.login`, `sdk.auth.useToken`, `sdk.auth.clearToken`, `sdk.auth.me` |
| Portal snapshots | `sdk.portal.getHome`, `sdk.portal.getAuth`, `sdk.portal.getWorkspace`, `sdk.portal.getDashboard`, `sdk.portal.getConversations`, `sdk.portal.getRealtime`, `sdk.portal.getMedia`, `sdk.portal.getAutomation`, `sdk.portal.getGovernance` |
| Conversations | `sdk.conversations.create`, `createAgentDialog`, `createAgentHandoff`, `createSystemChannel`, `get`, `getAgentHandoffState`, `acceptAgentHandoff`, `resolveAgentHandoff`, `closeAgentHandoff`, `listMembers`, `addMember`, `removeMember`, `transferOwner`, `changeMemberRole`, `leave`, `getReadCursor`, `updateReadCursor`, `listMessages`, `postMessage`, `postText`, `publishSystemMessage`, `publishSystemText` |
| Messages | `sdk.createXxxMessage`, `sdk.send`, `sdk.upload`, `sdk.uploadAndSendMessage`, `sdk.decodeMessage` |
| Media | `sdk.media.createUploadSession`, `sdk.media.createUpload`, `sdk.media.upload`, `sdk.media.uploadAndComplete`, `sdk.upload`, `sdk.media.completeUpload`, `sdk.media.getDownloadUrl`, `sdk.media.get`, `sdk.media.attach`, `sdk.media.attachText` |
| Live realtime | `sdk.connect(...)` and `sdk.live` |
| Durable catch-up | `sdk.sync.catchUp`, `sdk.sync.ack`, `context.ack()` |
| RTC | `sdk.rtc.create`, `sdk.rtc.postJsonSignal`, `sdk.rtc.issueParticipantCredential`, `sdk.rtc.getRecordingArtifact` |
| Generated-only route groups | `sdk.generated.device`, `sdk.generated.session`, `sdk.generated.presence`, `sdk.generated.realtime`, `sdk.generated.stream` |
| Raw generated transport | `sdk.generated`, `sdk.generated.inbox.getInbox()`, `SdkworkBackendClient`, `generated` |

## API Reference Map

Use the semantic SDK for application integration flow. Jump to the route reference below when you
need exact operation contracts or raw payload shapes:

| App domain | SDK surface | Exact API reference |
| --- | --- | --- |
| Auth and portal shell | `sdk.auth`, `sdk.portal` | [Portal and Auth](/api-reference/app/portal-and-auth) |
| Conversation lifecycle | `sdk.conversations.create`, `sdk.conversations.get`, `sdk.conversations.postMessage`, `sdk.conversations.postText` | [Conversations](/api-reference/app/conversations) |
| Membership and read cursors | `sdk.conversations.listMembers`, `sdk.conversations.addMember`, `sdk.conversations.updateReadCursor` | [Membership and Read State](/api-reference/app/membership-and-read-state) |
| Message schemas and semantic send model | `sdk.createTextMessage(...)`, `sdk.send(...)`, `sdk.decodeMessage(...)` | [Messages](/api-reference/app/messages) |
| Upload and attachment lifecycle | `sdk.media.createUploadSession`, `sdk.media.upload`, `sdk.upload`, `sdk.media.completeUpload`, `sdk.media.attachText` | [Media](/api-reference/app/media) |
| Session, presence, and realtime coordination | `sdk.connect(...)`, `sdk.sync.catchUp(...)`, `sdk.sync.ack(...)`, `sdk.generated.session`, `sdk.generated.presence`, `sdk.generated.realtime` | [Session and Realtime](/api-reference/app/session-and-realtime) |
| Device registration and sync feeds | `sdk.generated.device.register(...)`, `sdk.generated.device.getDeviceSyncFeed(...)` | [Device Sync](/api-reference/app/device-sync) |
| RTC lifecycle and signaling-side HTTP calls | `sdk.rtc.create`, `sdk.rtc.postJsonSignal(...)`, `sdk.rtc.issueParticipantCredential(...)`, `sdk.rtc.getRecordingArtifact(...)` | [RTC](/api-reference/app/rtc) |
| Stream ingestion and checkpointing | `sdk.generated.stream.open(...)`, `sdk.generated.stream.appendStreamFrame(...)`, `sdk.generated.stream.checkpoint(...)`, `sdk.generated.stream.complete(...)` | [Streams](/api-reference/app/streams) |

## Realtime Model

The app SDK standard separates two receive paths:

- live push through `sdk.connect(...)`
- durable replay through `sdk.sync.catchUp(...)`

This is deliberate.

Live push is for foreground sessions and interactive UI. Durable catch-up is for resume,
reconciliation, worker consumption, and explicit ACK control.

The live runtime is organized into domain streams:

- `live.messages.on(...)`
- `live.messages.onConversation(...)`
- `live.data.on(...)`
- `live.signals.on(...)`
- `live.signals.onRtcSession(...)`
- `live.events.on(...)`
- `live.lifecycle.onStateChange(...)`
- `live.lifecycle.onError(...)`

That model keeps the first callback argument as the final payload object and the second callback
argument as the operational receive context. Each receive context exposes `context.ack()` for
per-event acknowledgement, while `sdk.sync.ack(...)` commits the highest durable replay position
explicitly.

## Message Model

The app SDK is message-first.

Use:

- `sdk.createTextMessage(...)`
- `sdk.createImageMessage(...)`
- `sdk.createLocationMessage(...)`
- `sdk.createCardMessage(...)`
- `sdk.createMusicMessage(...)`
- `sdk.createCustomMessage(...)`
- `sdk.createAiTextMessage(...)`
- `sdk.createAiImageGenerationMessage(...)`
- `sdk.createAiVideoGenerationMessage(...)`
- `sdk.createAgentMessage(...)`
- `sdk.createAgentStateMessage(...)`
- `sdk.createAgentHandoffMessage(...)`
- `sdk.createToolResultMessage(...)`
- `sdk.createWorkflowEventMessage(...)`

Then deliver with:

```ts
await sdk.send(message);
```

For upload-first media flows, prefer `sdk.upload(...)` followed by
`sdk.createImageMessage(...)`/`sdk.send(...)`, or use `sdk.uploadAndSendMessage(...)` when you want
one helper to perform `create upload session -> presigned client upload -> complete -> send`. For
raw message bodies or stored payloads, normalize them through `sdk.decodeMessage(...)` before UI
rendering or routing.

## Conversations And Inbox

Use `sdk.conversations` for route-aligned lifecycle, membership, read-state, and direct posting
flows:

- `sdk.conversations.create(...)`
- `sdk.conversations.createAgentDialog(...)`
- `sdk.conversations.createAgentHandoff(...)`
- `sdk.conversations.createSystemChannel(...)`
- `sdk.conversations.get(...)`
- `sdk.conversations.getAgentHandoffState(...)`
- `sdk.conversations.acceptAgentHandoff(...)`
- `sdk.conversations.resolveAgentHandoff(...)`
- `sdk.conversations.closeAgentHandoff(...)`
- `sdk.conversations.listMembers(...)`
- `sdk.conversations.addMember(...)`
- `sdk.conversations.removeMember(...)`
- `sdk.conversations.transferOwner(...)`
- `sdk.conversations.changeMemberRole(...)`
- `sdk.conversations.leave(...)`
- `sdk.conversations.getReadCursor(...)`
- `sdk.conversations.updateReadCursor(...)`
- `sdk.conversations.listMessages(...)`
- `sdk.conversations.postMessage(...)`
- `sdk.conversations.postText(...)`
- `sdk.conversations.publishSystemMessage(...)`
- `sdk.conversations.publishSystemText(...)`

Use the root `CrawChatSdkClient` message helpers when you want the shortest message-builder path.
Use `sdk.messages` when you want the same behavior through a namespaced module. Use
`sdk.conversations` when you want near-1:1 route mapping with the OpenAPI contract.

Inbox currently stays on the generated transport surface:

```ts
const inbox = await sdk.generated.inbox.getInbox();
console.log(inbox.items.length);
```

## Portal Model

Portal and auth are first-class app-runtime domains in the TypeScript SDK.

- `sdk.auth` owns token acquisition and current-session identity
- `sdk.portal` owns tenant portal snapshots and workspace shell reads
- `sdk.generated.portal` remains available when you need exact generated transport access, but the
  preferred application entrypoint is `sdk.portal`

```ts
const login = await sdk.auth.login({
  tenantId: 'tenant-acme',
  login: 'ops_lead',
  password: '***',
  clientKind: 'portal_operator',
});

const workspace = await sdk.portal.getWorkspace();
const governance = await sdk.portal.getGovernance();

console.log(login.user?.name, workspace.name, governance.hero?.title);
```

## Media Lifecycle

Use `sdk.media` for explicit upload and attachment control:

- `sdk.media.createUploadSession(...)`
- `sdk.media.createUpload(...)`
- `sdk.media.upload(...)`
- `sdk.media.uploadAndComplete(...)`
- `sdk.upload(...)`
- `sdk.media.completeUpload(...)`
- `sdk.media.getDownloadUrl(...)`
- `sdk.media.get(...)`
- `sdk.media.attach(...)`
- `sdk.media.attachText(...)`

```ts
const uploaded = await sdk.upload({
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
```

`sdk.upload(...)` is the preferred root entrypoint for normal app code. `sdk.media.upload(...)` and
`sdk.media.uploadAndComplete(...)` expose the same presigned S3-compatible client upload flow from
the namespaced media module. `sdk.media.createUploadSession(...)` plus
`sdk.media.completeUpload(...)` remains available for advanced manual control.

## Generated-Only Route Groups

Some public app routes are currently exposed directly on the generated transport instead of a
handwritten semantic module:

- `sdk.generated.device.register(...)`
- `sdk.generated.device.getDeviceSyncFeed(...)`
- `sdk.generated.session.resume(...)`
- `sdk.generated.session.disconnect(...)`
- `sdk.generated.presence.heartbeat(...)`
- `sdk.generated.presence.getPresenceMe()`
- `sdk.generated.realtime.syncRealtimeSubscriptions(...)`
- `sdk.generated.realtime.listRealtimeEvents(...)`
- `sdk.generated.realtime.ackRealtimeEvents(...)`
- `sdk.generated.stream.open(...)`
- `sdk.generated.stream.listStreamFrames(...)`
- `sdk.generated.stream.appendStreamFrame(...)`
- `sdk.generated.stream.checkpoint(...)`
- `sdk.generated.stream.complete(...)`
- `sdk.generated.stream.abort(...)`

That boundary is intentional: the generated layer owns exact OpenAPI transport, while the semantic
SDK focuses on chat, realtime receive ergonomics, media, and RTC lifecycle.

## RTC Model

RTC stays on `sdk.rtc`, while inbound signaling remains on the live runtime.

- `sdk.rtc.create(...)` creates the RTC session
- `sdk.rtc.postJsonSignal(...)` sends common JSON signaling payloads
- `sdk.rtc.issueParticipantCredential(...)` issues provider join credentials
- `sdk.rtc.getRecordingArtifact(...)` fetches recording metadata
- `live.signals.onRtcSession(...)` receives inbound signaling events
- `live.lifecycle.onStateChange(...)` tracks connection lifecycle changes

```ts
const session = await sdk.rtc.create({
  rtcSessionId: 'rtc-1',
  conversationId: 'conversation-1',
  rtcMode: 'group_call',
});

await sdk.rtc.postJsonSignal(session.rtcSessionId, 'offer', {
  signalingStreamId: 'rtc-signal-1',
  payload: {
    sdp: 'v=0...',
  },
});

await sdk.rtc.issueParticipantCredential(session.rtcSessionId, {
  participantId: 'user-1',
});

await sdk.rtc.getRecordingArtifact(session.rtcSessionId);

live.signals.onRtcSession(session.rtcSessionId, (signal, context) => {
  console.log(signal.signalType, signal.payload, context.scopeId);
});
```

## Generated Contract Source

The semantic SDK is built on top of the app OpenAPI 3.x contract exported by the running service.

The current source chain is:

1. live service schema export at `/openapi/craw-chat-app.openapi.yaml`
2. checked-in authority snapshot at `openapi/craw-chat-app.openapi.yaml`
3. generated transport under `generated/server-openapi`
4. assembled single-package TypeScript consumer output

That means the generated layer owns only the HTTP contract. The semantic layer owns live receive,
message ergonomics, and runtime orchestration.

## What To Read Next

- [TypeScript SDK](/sdk/typescript-sdk)
- [SDK Overview](/sdk/)
- [Portal and Auth](/api-reference/app/portal-and-auth)
- [Conversations](/api-reference/app/conversations)
- [Messages API](/api-reference/app/messages)
- [Media API](/api-reference/app/media)
- [Session and Realtime API](/api-reference/app/session-and-realtime)
- [RTC API](/api-reference/app/rtc)
- [Streams API](/api-reference/app/streams)
