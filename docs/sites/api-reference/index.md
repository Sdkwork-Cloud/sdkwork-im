# API Reference

<p class="api-page-intro">
  This API reference is aligned to the HTTP surface implemented by the current repository state.
  It documents request and response shapes using an OpenAPI-style layout, with expandable nested
  schemas for deeply structured payloads.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>App API</h3>
    <p>Session recovery, realtime delivery, device sync, conversations, membership, messages, media, streams, and RTC.</p>
    <p><a href="/api-reference/app-api">Open App API overview</a></p>
  </div>
  <div class="api-card">
    <h3>Platform API</h3>
    <p>Notifications, automation, audit records, operator diagnostics, and provider health endpoints.</p>
    <p><a href="/api-reference/platform-api">Open Platform API overview</a></p>
  </div>
  <div class="api-card">
    <h3>IoT API</h3>
    <p>IoT protocol ingest and provider health endpoints for access and protocol adapters.</p>
    <p><a href="/api-reference/iot-api">Open IoT API overview</a></p>
  </div>
  <div class="api-card">
    <h3>Control Plane API</h3>
    <p>Protocol governance, provider registry and policy management, plus node drain, activate, and route migration operations.</p>
    <p><a href="/api-reference/control-plane-api">Open Control Plane overview</a></p>
  </div>
</div>

## Standards Used

- Operation pages are grouped by runtime domain and linked individually in the sidebar.
- Request and response payloads are rendered from a shared schema registry to keep documentation aligned with implementation.
- Nested request and response fields are expandable so large payloads remain readable on desktop and mobile.
- Shared auth rules and error envelope semantics are documented in [Authentication and Errors](/api-reference/auth-and-errors).

## Recommended Reading Path

<div class="api-link-list">
  <a href="/api-reference/auth-and-errors"><code>Auth</code> Authentication, trusted headers, and the error envelope</a>
  <a href="/api-reference/app/session-and-realtime"><code>App</code> Session and realtime transport semantics</a>
  <a href="/api-reference/app/conversations"><code>App</code> Conversation creation and handoff flows</a>
  <a href="/api-reference/platform/ops"><code>Platform</code> Operator diagnostics and runtime inspection</a>
  <a href="/api-reference/control-plane/providers"><code>Control Plane</code> Provider registry, binding policies, and preview or rollback flows</a>
</div>
