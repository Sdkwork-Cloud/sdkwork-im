---
layout: home

hero:
  name: Craw Chat
  text: SDK-first integration documentation
  tagline: Implementation-aligned open-source docs for the Craw Chat app runtime, the TypeScript, Flutter, and Rust app SDKs, the App API, and deployment workflows.
  actions:
    - theme: brand
      text: SDK Overview
      link: /sdk/index
    - theme: alt
      text: TypeScript Quick Start
      link: /sdk/typescript-quick-start
    - theme: alt
      text: App API
      link: /api-reference/index

features:
  - title: Trilanguage App SDK
    details: The app-facing SDK family in this worktree exposes composed SDK surfaces for TypeScript, Flutter, and Rust above generated HTTP transport packages.
  - title: App SDK Before Raw HTTP
    details: The docs route integrators to SDK quick starts, client initialization, module pages, and scenario examples before dropping down into operation-level API details.
  - title: Generated vs Composed Clarity
    details: Every SDK page distinguishes generator-owned `generated/server-openapi` output from the manual-owned `composed` layer that consumers should integrate against.
  - title: API and SDK Mapping
    details: App API pages stay operation-focused, while SDK pages stay integration-focused, and both sections cross-link to each other by runtime domain.
  - title: Public Auth Reality
    details: The public integration contract is bearer-token based. Trusted headers remain internal and test-oriented, not the app SDK contract.
  - title: Operable Runtime
    details: Runtime topology, deployment scripts, inspection workflows, backup or restore paths, and environment expectations remain documented from the code that actually runs.
---

## What You Can Integrate Today

The current Craw Chat worktree exposes two distinct developer surfaces:

- the app-facing runtime and App API surface, served by `local-minimal-node`
- the trilanguage app SDK family under `sdks/sdkwork-craw-chat-sdk`

For most integrators, the best reading order is:

1. [SDK Overview](/sdk/index)
2. [Auth and Client Init](/sdk/auth-and-client-init)
3. one language quick start
4. [Module Map](/sdk/module-map)
5. the matching App API domain pages

<div class="landing-grid">
  <div class="fact-card">
    <h3>Preferred App SDK Surface</h3>
    <p>Use the manual-owned <code>composed</code> SDK layer. It exposes <code>CrawChatClient</code> and semantic modules in TypeScript, Flutter, and Rust.</p>
  </div>
  <div class="fact-card">
    <h3>Local App Base URL</h3>
    <p><code>http://127.0.0.1:18090</code> is the default app runtime listener for local binary workflows.</p>
  </div>
  <div class="fact-card">
    <h3>Public Auth Contract</h3>
    <p>SDK consumers authenticate with <code>Authorization: Bearer &lt;token&gt;</code>. Trusted headers are not the public app SDK contract.</p>
  </div>
  <div class="fact-card">
    <h3>Publication Boundary</h3>
    <p>The workspaces exist locally in this repository. Public registry publication is a separate concern and is not implied by local package availability.</p>
  </div>
</div>

## SDK Entry Points

| Language | Preferred package or crate | Quick start | Notes |
| --- | --- | --- | --- |
| TypeScript | `@sdkwork/craw-chat-sdk` | [/sdk/typescript-quick-start](/sdk/typescript-quick-start) | Async `CrawChatClient.create()` layered over the generated backend SDK |
| Flutter | `craw_chat_sdk` | [/sdk/flutter-quick-start](/sdk/flutter-quick-start) | `CrawChatClient.create()` layered over `backend_sdk` |
| Rust | `craw-chat-sdk` | [/sdk/rust-quick-start](/sdk/rust-quick-start) | `CrawChatClient::new_with_base_url()` plus semantic module accessors |

## App Runtime and SDK Surface

| Area | What is currently implemented |
| --- | --- |
| App runtime | Session resume, presence, realtime coordination, device sync, inbox, conversations, membership, messages, media, streams, and RTC |
| App SDK family | TypeScript, Flutter, and Rust workspaces with generated transport layers and manual-owned composed SDK surfaces |
| App API reference | OpenAPI-style domain pages for session and realtime, device sync, conversations, membership, messages, media, streams, and RTC |
| Admin and control plane | Separate audience and workflow from the app SDK. See [/sdk/admin-sdk](/sdk/admin-sdk) and [/api-reference/control-plane-api](/api-reference/control-plane-api). |
| Deployment | Local binary lifecycle scripts, Docker Compose bootstrap, runtime inspection, repair, backup listing, archive, preview, and restore |

::: warning Scope rule
This site documents implemented repository behavior and local workspace surfaces. It does not treat
local packages or crates as automatically published to ecosystem registries, and it does not claim
handwritten WebSocket adapters where the current SDK round documents only the HTTP coordination
surface.
:::

## Recommended Reading

1. Start with [SDK Overview](/sdk/index) if you are integrating application features.
2. Use [Auth and Client Init](/sdk/auth-and-client-init) before writing any client bootstrap code.
3. Pick one language quick start:
   [TypeScript](/sdk/typescript-quick-start),
   [Flutter](/sdk/flutter-quick-start),
   [Rust](/sdk/rust-quick-start).
4. Use [Module Map](/sdk/module-map) to find the right semantic SDK surface for your use case.
5. Drop into [App API Overview](/api-reference/app-api) when you need exact payloads, statuses, or operation-level details.
