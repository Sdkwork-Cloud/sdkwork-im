# Control Plane API Overview

<p class="api-page-intro">
  The Control Plane API manages runtime governance outside the app request path. It exposes
  protocol registry and governance snapshots, provider registry and policy management, and node
  lifecycle operations for drain, activate, and route migration.
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

- These endpoints are administrative and align with backend or control-plane SDK groups rather than the public app SDK.
- The current admin SDK workspace does not yet include a checked-in admin OpenAPI authority file, so this reference stays aligned directly to `services/control-plane-api/src/lib.rs` and the control-plane tests.
- Read and write permissions are split between `control.read` and `control.write`.

## How To Use This Page

Use the control-plane API docs as the semantic authority first:

1. Read this overview and the linked control-plane operation groups for request, response, and permission behavior.
2. Use [Admin SDK](/sdk/admin-sdk) only to confirm audience boundary, source-of-truth files, and release-state limits.
3. Do not infer stable language-specific admin imports from these HTTP pages, because the repo admin authority contract and validated consumer package manifests are not yet present.

That split is intentional: control-plane route behavior is documented as delivered, while admin
consumer package surfaces are still documented conservatively.

## What To Read Next

- [Admin SDK](/sdk/admin-sdk)
- [SDK Overview](/sdk/index)
- [Authentication and Errors](/api-reference/auth-and-errors)

## Control Plane Domains

<div class="api-link-list">
  <a href="/api-reference/control-plane/protocol"><code>Protocol</code> Health, protocol registry, and governance snapshots</a>
  <a href="/api-reference/control-plane/providers"><code>Providers</code> Registry, effective bindings, policy history, diff, preview, and rollback</a>
  <a href="/api-reference/control-plane/nodes"><code>Nodes</code> Drain, activate, and route migration operations</a>
</div>
