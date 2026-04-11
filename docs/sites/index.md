---
layout: home

hero:
  name: Craw Chat
  text: Implementation-aligned product documentation
  tagline: Open-source documentation for the current Craw Chat repository state, covering runtime architecture, OpenAPI-style HTTP APIs, SDK workspaces, and deployment operations.
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started/index
    - theme: alt
      text: API Reference
      link: /api-reference/index
    - theme: alt
      text: Deployment
      link: /deployment/index

features:
  - title: Implementation First
    details: Every page is written against the current repository behavior, not against roadmap assumptions or placeholder directories.
  - title: OpenAPI-style API Docs
    details: HTTP operations are grouped by runtime domain, linked directly from the sidebar, and documented with request and response schemas plus nested field expansion.
  - title: SDK Boundary Clarity
    details: The docs distinguish between checked-in OpenAPI authority, generated-workspace layout, and actual release status so consumers do not confuse templates with published packages.
  - title: Operable Deployment Guides
    details: Local binary, Docker Compose, runtime inspection, repair, backup, preview, and restore flows are documented from the existing scripts and binaries.
  - title: Runtime-centric Architecture
    details: The site explains the Rust workspace, service boundaries, runtime-directory contract, provider model, and control-plane governance through the code that actually runs.
  - title: Mature Open-source Structure
    details: The site is organized as a product documentation portal with clear entry points for onboarding, operations, architecture, APIs, SDKs, and reference material.
---

## What Craw Chat Is

Craw Chat is a Rust workspace that currently ships two meaningful runtime surfaces:

- `services/local-minimal-node`, the app-facing node that serves session, conversation, message,
  media, stream, RTC, platform, ops, audit, and provider-health routes.
- `services/control-plane-api`, the separate governance surface for protocol registry, provider
  policy, and node lifecycle operations.

The default runnable profile in this repository is `local-minimal`. The `local-default` profile
already has script, config, and Docker entry points, but it still reuses the current
`local-minimal` runtime contract and topology.

<div class="landing-grid">
  <div class="fact-card">
    <h3>Default App Listener</h3>
    <p><code>127.0.0.1:18090</code> for local binary workflows, and <code>0.0.0.0:18090</code> inside Docker Compose.</p>
  </div>
  <div class="fact-card">
    <h3>Control Plane Listener</h3>
    <p><code>127.0.0.1:18081</code> when the standalone <code>control-plane-api</code> binary is run directly.</p>
  </div>
  <div class="fact-card">
    <h3>Public Auth Model</h3>
    <p>Public HTTP surfaces use <code>Authorization: Bearer &lt;token&gt;</code> signed with <code>CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET</code>.</p>
  </div>
  <div class="fact-card">
    <h3>SDK Delivery State</h3>
    <p>The release catalog is still <code>template_only_pending_generation</code> and <code>not_published</code>, even though the SDK workspace structure is already checked in.</p>
  </div>
</div>

## Recommended Reading Path

1. Start with [Getting Started](/getting-started/index) to understand supported runtime modes,
   prerequisites, and auth expectations.
2. Use [Quick Start](/getting-started/quick-start) to initialize config, build the local node, and
   verify health or smoke status.
3. Read [Architecture Overview](/architecture/overview) and
   [Runtime Topology](/architecture/runtime-topology) before changing runtime wiring, providers, or
   deployment assumptions.
4. Use [API Reference](/api-reference/index) for the OpenAPI-style operation catalog with sidebar
   links to every documented endpoint.
5. Read [SDK Overview](/sdk/index) before promising package availability, version numbers, or
   generation status to downstream consumers.
6. Keep [Deployment](/deployment/index) and [Reference](/reference/cli-and-scripts) open when
   running, diagnosing, backing up, or restoring a local environment.

## Current Delivery Surface

| Area | What is currently implemented |
| --- | --- |
| App runtime | Session resume, presence, realtime delivery, device sync, conversations, membership, messages, media, streams, RTC, notifications, automation, audit, ops, and provider health |
| Control plane | Protocol registry, protocol governance, provider registry, effective bindings, provider policy preview and rollback, plus node drain, activate, and route migration |
| Deployment | Local binary lifecycle scripts, Docker Compose bootstrap, runtime inspection, repair, backup listing, archive, preview, and restore |
| SDK workspaces | App SDK workspace with checked-in OpenAPI authority and derived sdkgen input; admin SDK workspace with frozen audience and language boundaries but no checked-in admin OpenAPI source yet |
| Frontend apps | `apps/craw-chat-admin` and `apps/craw-chat-portal` exist as workspace directories but are not documented here as mature product surfaces |

::: warning Scope rule
This documentation intentionally describes only what can be verified from the current repository
state. Placeholder directories, future plans, or unpublished SDK artifacts are explicitly marked as
such rather than documented as delivered features.
:::

<div class="source-note">
  <strong>Implementation sources:</strong>
  App routing is aligned to <code>services/local-minimal-node/src/node/build.rs</code>.
  Control-plane routing is aligned to <code>services/control-plane-api/src/lib.rs</code>.
  Local lifecycle and deployment behavior is aligned to <code>bin/</code>,
  <code>deployments/</code>, and the runtime-management entrypoints in
  <code>services/local-minimal-node/src/main.rs</code>.
</div>
