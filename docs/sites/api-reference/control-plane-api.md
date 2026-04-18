# Control Plane API Overview

<p class="api-page-intro">
  The Control Plane API manages runtime governance outside the app node request path. It exposes
  protocol registry and governance snapshots, provider registry and provider policy management,
  social graph and shared-channel runtime control, and node lifecycle operations for drain,
  activate, and route migration.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Protocol Governance</h3>
    <p>Read protocol registry inventory, client compatibility, quota profiles, rollout policies, and kill-switch state.</p>
    <p><a href="/api-reference/control-plane/protocol">Open Protocol Governance APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Provider Governance</h3>
    <p>Inspect provider plugins, read effective bindings, apply binding policies, preview diffs, and roll back versions.</p>
    <p><a href="/api-reference/control-plane/providers">Open Provider Governance APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Social Graph Control</h3>
    <p>Bind direct chats, establish external connections, manage friendships, apply shared-channel policies, and enforce user blocks.</p>
    <p><a href="/api-reference/control-plane/social">Open Social Graph Control APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Social Runtime</h3>
    <p>Inspect pending, delivered, and dead-letter shared-channel sync queues, then repair, reclaim, republish, requeue, or take over targeted work.</p>
    <p><a href="/api-reference/control-plane/social-runtime">Open Social Runtime APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Node Operations</h3>
    <p>Drain nodes, reactivate nodes, and migrate owned realtime routes between nodes.</p>
    <p><a href="/api-reference/control-plane/nodes">Open Node Operation APIs</a></p>
  </div>
</div>

## SDK Alignment

- These endpoints are the checked-in authority for `sdkwork-craw-chat-sdk-admin`.
- Read the SDK guides at [Admin SDK](/sdk/admin-sdk), [Admin TypeScript SDK](/sdk/admin-typescript-sdk), and [Admin Flutter SDK](/sdk/admin-flutter-sdk).
- The live contract source is `services/control-plane-api`, exposed at `/openapi.json` and `/api/v1/control/openapi.json`, then normalized into `sdks/sdkwork-craw-chat-sdk-admin/openapi/admin-control-plane.openapi.yaml`.
- Read and write permissions are split between `control.read` and `control.write`.

## Control Plane Domains

<div class="api-link-list">
  <a href="/api-reference/control-plane/protocol"><code>Protocol</code> Health, protocol registry, and governance snapshots</a>
  <a href="/api-reference/control-plane/providers"><code>Providers</code> Registry, effective bindings, policy history, diff, preview, and rollback</a>
  <a href="/api-reference/control-plane/social"><code>Social</code> Direct-chat, external-collaboration, friendship, shared-channel-policy, and user-block control</a>
  <a href="/api-reference/control-plane/social-runtime"><code>Social Runtime</code> Shared-channel sync queue inventory, repair, reclaim, republish, requeue, and takeover flows</a>
  <a href="/api-reference/control-plane/nodes"><code>Nodes</code> Drain, activate, and route migration operations</a>
</div>
