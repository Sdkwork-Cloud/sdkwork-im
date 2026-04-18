# Control Plane API Overview

<p class="api-page-intro">
  The Control Plane API manages runtime governance outside the app node request path. It exposes
  protocol registry and governance snapshots, provider registry and provider policy management, and
  node lifecycle operations for drain, activate, and route migration.
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
    <h3>Node Operations</h3>
    <p>Drain nodes, reactivate nodes, and migrate owned realtime routes between nodes.</p>
    <p><a href="/api-reference/control-plane/nodes">Open Node Operation APIs</a></p>
  </div>
</div>

## SDK Alignment

- These endpoints are administrative and align with the control-plane SDK family `sdkwork-craw-chat-sdk-admin` rather than the public app SDK.
- The checked-in admin SDK authority contract is `sdks/sdkwork-craw-chat-sdk-admin/openapi/craw-chat-control-plane.openapi.json`, with `craw-chat-control-plane.sdkgen.json` as the derived generator input.
- This reference still stays aligned directly to `services/control-plane-api/src/lib.rs` and the control-plane test suite, because runtime implementation remains the source of truth behind the exported authority snapshot.
- Standalone governance development can call `control-plane-api` directly, but packaged installs expose the same governance routes through the unified `craw-chat-server` / `web-gateway` public origin.
- Read and write permissions are split between `control.read` and `control.write`.

## Control Plane Domains

<div class="api-link-list">
  <a href="/api-reference/control-plane/protocol"><code>Protocol</code> Health, protocol registry, and governance snapshots</a>
  <a href="/api-reference/control-plane/providers"><code>Providers</code> Registry, effective bindings, policy history, diff, preview, and rollback</a>
  <a href="/api-reference/control-plane/nodes"><code>Nodes</code> Drain, activate, and route migration operations</a>
</div>
