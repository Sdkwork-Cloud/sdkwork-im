# Platform API Overview

<p class="api-page-intro">
  Platform APIs expose business automation, operator tooling, audit, and provider health surfaces
  implemented behind the `local-minimal-node` profile. These endpoints are typically consumed by
  administrative consoles, internal services, and operational tooling.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Notifications</h3>
    <p>Create notification requests and inspect dispatched notification tasks.</p>
    <p><a href="/api-reference/platform/notifications">Open Notification APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Automation</h3>
    <p>Trigger automation executions and inspect execution state.</p>
    <p><a href="/api-reference/platform/automation">Open Automation APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Audit and Ops</h3>
    <p>Read and export audit records, inspect runtime health, lag, cluster topology, and diagnostic bundles.</p>
    <p><a href="/api-reference/platform/ops">Open Operator APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Provider Health</h3>
    <p>Check media, RTC, and user-module provider plugin health from the active node.</p>
    <p><a href="/api-reference/platform/provider-health">Open Provider Health APIs</a></p>
  </div>
</div>

## SDK Alignment

- The local `platform/*` routes documented on this page do not currently have a standalone published SDK family. They are typically consumed through backend tooling, internal services, or direct HTTP integrations.
- Operator and `/api/admin/*` platform surfaces are intended for `sdkwork-craw-chat-sdk-management`; control-plane governance stays in `sdkwork-craw-chat-sdk-admin`; neither belongs in the public `sdkwork-craw-chat-sdk` surface.
- In packaged installs, these routes are still reached through the unified `craw-chat-server` / `web-gateway` public origin even though the implementation remains on the app-node side of the runtime.
- Permission requirements are documented in [Authentication and Errors](/api-reference/auth-and-errors) and repeated on operation pages when they are mandatory.

## Platform API Domains

<div class="api-link-list">
  <a href="/api-reference/platform/notifications"><code>Notifications</code> Request, list, and inspect notification tasks</a>
  <a href="/api-reference/platform/automation"><code>Automation</code> Request and inspect automation executions</a>
  <a href="/api-reference/platform/audit"><code>Audit</code> Record audit anchors, list audit records, and export bundles</a>
  <a href="/api-reference/platform/ops"><code>Ops</code> Cluster health, lag, replay status, runtime directory, provider bindings, and diagnostics</a>
  <a href="/api-reference/platform/provider-health"><code>Health</code> Provider plugin health snapshots for active integrations</a>
</div>
