---
layout: home

hero:
  name: Craw Chat
  text: Implementation-aligned product documentation
  tagline: "Open-source docs aligned to the current Craw Chat repository state: runtime architecture, HTTP APIs, SDK workspaces, and deployment workflows."
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started/index
    - theme: alt
      text: API Reference
      link: /api-reference/index
    - theme: alt
      text: SDK Docs
      link: /sdk/index

features:
  - title: Implementation First
    details: Every page is written against the current repository behavior, not against roadmap assumptions or placeholder directories.
  - title: OpenAPI-style API Docs
    details: HTTP operations are grouped by runtime domain, linked from the sidebar, and documented with request and response schemas plus nested field expansion.
  - title: SDK Boundary Clarity
    details: The docs distinguish release-catalog artifacts, repo package contracts, and consumer package surfaces so repository state is not confused with registry publication.
  - title: Operable Deployment Guides
    details: Local binary, Docker Compose, runtime inspection, repair, backup, preview, and restore flows are documented from the existing scripts and binaries.
  - title: Runtime-centric Architecture
    details: The site explains the Rust workspace, service boundaries, runtime-directory contract, provider model, and control-plane governance through the code that runs today.
  - title: Mature Open-source Structure
    details: The site is organized as a product documentation portal with clear entry points for onboarding, operations, architecture, APIs, SDKs, and reference material.
---

## What Craw Chat Is

Craw Chat is a Rust workspace with two primary runtime surfaces in the current repository state:

- `services/local-minimal-node`, the app-facing node that serves session, conversation, message,
  media, stream, RTC, platform, ops, audit, and provider-health routes.
- `services/control-plane-api`, the separate governance surface for protocol registry, provider
  policy, and node lifecycle operations.

The default runnable profile is `local-minimal`. The `local-default` profile already has script,
config, and Docker entry points, but it still reuses the current `local-minimal` runtime contract
and topology.

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
    <p>The release catalog still shows <code>template_only_pending_generation</code> and <code>not_published</code>. Repo package manifests exist, but that is not publication proof.</p>
  </div>
</div>

## What To Read Next

1. Start with [Getting Started](/getting-started/index) to understand supported runtime modes,
   prerequisites, and auth expectations.
2. Use [Quick Start](/getting-started/quick-start) to initialize config, build the local node, and
   verify health or smoke status.
3. Read [Architecture Overview](/architecture/overview) and
   [Runtime Topology](/architecture/runtime-topology) before changing runtime wiring, providers, or
   deployment assumptions.
4. Use [API Reference](/api-reference/index) for the OpenAPI-style operation catalog with sidebar
   links to every documented endpoint.
5. Read [SDK Overview](/sdk/index), then choose [TypeScript SDK](/sdk/typescript-sdk) or
   [Flutter SDK](/sdk/flutter-sdk) before promising package availability, import paths, or feature
   parity to consumers.
6. Keep [Deployment](/deployment/index) and [Reference](/reference/cli-and-scripts) open when
   running, diagnosing, backing up, or restoring a local environment.

## Choose Your Entry Point

| If you are... | Start here | Why |
| --- | --- | --- |
| Integrating against the public app runtime | [SDK Overview](/sdk/index) | Separates contract authority, repo packages, and release state |
| Building a browser or Node integration today | [TypeScript SDK](/sdk/typescript-sdk) | TypeScript now exposes the widest repo app SDK surface from one package, `@sdkwork/craw-chat-sdk`, including portal auth and portal snapshots |
| Building a Flutter client for app-runtime flows | [Flutter SDK](/sdk/flutter-sdk) | Shows the real exported Dart surface and explicitly calls out the current portal/auth parity gap |
| Working directly with HTTP endpoints | [API Reference](/api-reference/index) | Links every documented operation page from the sidebar |
| Operating or diagnosing a local deployment | [Deployment](/deployment/index) | Covers binary startup, Docker Compose, runtime inspection, backup, repair, preview, and restore |
| Changing runtime or governance behavior | [Architecture Overview](/architecture/overview) | Anchors service boundaries, runtime topology, and source-of-truth files before code changes |

## Current Delivery Surface

| Area | What is currently implemented |
| --- | --- |
| App runtime | Session resume, presence, realtime delivery, device sync, conversations, membership, messages, media, streams, RTC, notifications, automation, audit, ops, and provider health |
| Control plane | Protocol registry, protocol governance, provider registry, effective bindings, provider policy preview and rollback, plus node drain, activate, and route migration |
| Deployment | Local binary lifecycle scripts, Docker Compose bootstrap, runtime inspection, repair, backup listing, archive, preview, and restore |
| SDK workspaces | App SDK workspace with a live-schema-backed OpenAPI authority snapshot, a TypeScript single-package consumer SDK, and Flutter generated-plus-composed layers; admin SDK workspace with fixed audience and language boundaries but no checked-in admin OpenAPI source yet |
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
