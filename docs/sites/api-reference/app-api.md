# App API Overview

<p class="api-page-intro">
  The App API is the user-facing HTTP surface implemented by `local-minimal-node`. It combines
  session recovery, device routing, conversation runtime, media transport, streaming, realtime
  delivery, and RTC signaling in a single deployment profile.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Portal and Auth</h3>
    <p>Tenant portal sign-in, current session discovery, public landing snapshots, workspace reads, and authenticated portal module snapshots.</p>
    <p><a href="/api-reference/app/portal-and-auth">Open Portal and Auth APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Transport and Presence</h3>
    <p>Session resume, disconnect, heartbeat, realtime subscriptions, event polling, and WebSocket upgrade.</p>
    <p><a href="/api-reference/app/session-and-realtime">Open Session and Realtime</a></p>
  </div>
  <div class="api-card">
    <h3>Conversation Runtime</h3>
    <p>Inbox, conversation creation, agent dialogs, handoffs, membership management, read cursors, and messages.</p>
    <p><a href="/api-reference/app/conversations">Open Conversation APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Media and Streams</h3>
    <p>Media upload lifecycle, media-to-message attachment, open streams, frame append, checkpoint, complete, and abort.</p>
    <p><a href="/api-reference/app/media">Open Media APIs</a></p>
  </div>
  <div class="api-card">
    <h3>RTC</h3>
    <p>Create, invite, accept, reject, signal, credential issuance, provider callbacks, and recording artifacts.</p>
    <p><a href="/api-reference/app/rtc">Open RTC APIs</a></p>
  </div>
</div>

## SDK Alignment

- These endpoints map to the App SDK documentation in [SDK Overview](/sdk/index) and [App SDK](/sdk/app-sdk).
- Portal auth and portal snapshot helpers are currently exposed most completely in the TypeScript SDK.
- Session resume, realtime subscription sync, poll, and ACK map directly into generated HTTP SDK layers, while the TypeScript package additionally delivers a semantic live runtime through `sdk.connect(...)` above that boundary for `/api/v1/realtime/ws`.
- The repo App SDK authority contract is `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml`, with `craw-chat-app.sdkgen.yaml` as the derived generator input.
- Sidebar grouping follows the same runtime boundaries used in the implementation: portal, session gateway, conversation runtime, media, streaming, and RTC services.

## How To Use This Page

Use the App API pages for route semantics first:

1. Read this section and the linked operation pages for request or response contracts.
2. Switch to [App SDK](/sdk/app-sdk) when you need the official consumer package in each language workspace: `@sdkwork/craw-chat-sdk` in TypeScript and `craw_chat_sdk` in Flutter.
3. Switch to [TypeScript SDK](/sdk/typescript-sdk) when you need checked-in portal auth or portal snapshot helpers today.
4. Switch to [Flutter SDK](/sdk/flutter-sdk) when you need the current Dart export surface and parity caveats.

The API reference remains the authority for HTTP behavior even when the SDK pages describe a
higher-level helper method on top of it.

## What To Read Next

- [SDK Overview](/sdk/index)
- [App SDK](/sdk/app-sdk)
- [TypeScript SDK](/sdk/typescript-sdk)
- [Flutter SDK](/sdk/flutter-sdk)

## App API Domains

<div class="api-link-list">
  <a href="/api-reference/app/portal-and-auth"><code>Portal</code> Tenant sign-in, session discovery, and portal snapshot reads</a>
  <a href="/api-reference/app/session-and-realtime"><code>Session</code> Session resume, presence, realtime subscriptions, and event delivery</a>
  <a href="/api-reference/app/device-sync"><code>Device Sync</code> Device registration and sync-feed projection reads</a>
  <a href="/api-reference/app/conversations"><code>Conversation</code> Conversation creation, system channels, and agent handoff flows</a>
  <a href="/api-reference/app/membership-and-read-state"><code>Membership</code> Member roster operations and read-cursor updates</a>
  <a href="/api-reference/app/messages"><code>Messages</code> Timeline reads, message send, edit, recall, and system-channel publish</a>
  <a href="/api-reference/app/media"><code>Media</code> Upload initiation, completion, media lookup, signed download, and attach</a>
  <a href="/api-reference/app/streams"><code>Streams</code> Stream open, frame append, list, checkpoint, complete, and abort</a>
  <a href="/api-reference/app/rtc"><code>RTC</code> Session lifecycle, signaling, credentials, callbacks, and recording artifacts</a>
</div>
