# IoT API Overview

<p class="api-page-intro">
  The IoT API covers protocol ingress and provider health inspection for device access and protocol
  adapters. These endpoints are exposed by the same application runtime but are grouped separately
  because they bridge device protocols into stream and message workflows.
</p>

<div class="api-overview-grid">
  <div class="api-card">
    <h3>Protocol Ingress</h3>
    <p>Ingest raw uplink payloads and submit structured downlink payloads that are encoded onto device command streams.</p>
    <p><a href="/api-reference/iot/protocol-and-health">Open IoT Protocol APIs</a></p>
  </div>
  <div class="api-card">
    <h3>Provider Health</h3>
    <p>Inspect health snapshots for the IoT access provider and IoT protocol provider selected by the active node.</p>
    <p><a href="/api-reference/iot/protocol-and-health#get-iot-access-provider-health">Jump to provider health</a></p>
  </div>
</div>

## SDK Alignment

- IoT-facing administrative flows typically sit behind backend or operator SDKs rather than the public app SDK.
- Downlink requests ultimately write frames into the same streaming model used by the App API.
- This site does not document a separate published IoT SDK family. Treat these routes as HTTP-first surfaces unless a repo consumer layer is documented separately.
- The routes documented on this page do not currently have a standalone published SDK family.
- In packaged installs, these IoT routes are still published through the unified `craw-chat-server`
  / `web-gateway` public origin rather than a separate public device-ingress port.
- In packaged installs, these IoT routes are still published through the unified `craw-chat-server` / `web-gateway` public origin rather than a separate public device-ingress port.

## How To Use This Page

- Start with [Authentication and Errors](/api-reference/auth-and-errors) for shared auth and error rules.
- Use the linked IoT operation group for exact ingress, downlink, and provider-health behavior.
- Switch to [SDK Overview](/sdk/index) only when you need to determine whether a repo package also has a documented publication state.

## What To Read Next

- [Authentication and Errors](/api-reference/auth-and-errors)
- [SDK Overview](/sdk/index)

## IoT API Domains

<div class="api-link-list">
  <a href="/api-reference/iot/protocol-and-health"><code>IoT</code> Provider health, uplink ingest, and downlink submission</a>
</div>
