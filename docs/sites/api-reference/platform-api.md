# Platform API Overview

<p class="api-page-intro">
  Platform APIs expose business automation, operator tooling, audit, and provider health surfaces
  implemented behind the `local-minimal-node` profile. These endpoints are typically consumed by
  administrative consoles, internal services, and operator tooling.
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

- These endpoints are typically consumed through administrative or backend SDK layers rather than the public app SDK.
- Permission requirements are documented in [Authentication and Errors](/api-reference/auth-and-errors) and repeated on operation pages when they are mandatory.
- This site does not document a separate published Platform SDK family. Treat these APIs as HTTP-first surfaces unless a repo backend or admin consumer layer is documented elsewhere.

## How To Use This Page

- Start with [Authentication and Errors](/api-reference/auth-and-errors) for shared bearer and permission rules.
- Use the linked operation groups below for exact route semantics.
- Switch to [SDK Overview](/sdk/index) only when you need to understand whether a backend-facing
  package surface is documented as published or only present as repo workspace state.

## What To Read Next

- [Authentication and Errors](/api-reference/auth-and-errors)
- [SDK Overview](/sdk/index)

## Platform API Domains

<div class="api-link-list">
  <a href="/api-reference/platform/notifications"><code>Notifications</code> Request, list, and inspect notification tasks</a>
  <a href="/api-reference/platform/automation"><code>Automation</code> Request and inspect automation executions</a>
  <a href="/api-reference/platform/audit"><code>Audit</code> Record audit anchors, list audit records, and export bundles</a>
  <a href="/api-reference/platform/ops"><code>Ops</code> Cluster health, lag, replay status, runtime directory, provider bindings, and diagnostics</a>
  <a href="/api-reference/platform/provider-health"><code>Health</code> Provider plugin health snapshots for active integrations</a>
</div>
