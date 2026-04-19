# IM SDK Docs Professionalization Design

## Goal

Upgrade `docs/sites` into a professional open-source documentation portal for IM SDK consumers, with SDK-first onboarding, multi-language integration guidance for TypeScript, Flutter, and Rust, precise ownership boundaries between generated and manual SDK layers, and strong cross-linking between SDK modules and the underlying App API reference.

This design extends the site-level direction already captured in `docs/superpowers/specs/2026-04-09-craw-chat-open-source-docs-site-design.md` and aligns documentation to the currently implemented trilanguage app SDK state in this repository.

## Audience

Primary audience:

- business and product integration developers embedding Craw Chat app capabilities into web, mobile, and backend applications
- maintainers who need to understand where generated SDK output ends and handwritten SDK surface begins

Secondary audience:

- reviewers validating whether a documented SDK capability is actually implemented and verified
- contributors extending the SDK workspace or the documentation portal

The documentation should optimize for the path from "I need to integrate chat" to "I have a correctly initialized client and know which module to call" rather than for repository browsing.

## Canonical Sources

- Documentation site shell: `docs/sites/.vitepress/config.ts`, `docs/sites/index.md`
- Current SDK docs: `docs/sites/sdk/*.md`
- Current API docs: `docs/sites/api-reference/**/*.md`
- App SDK workspace root: `sdks/sdkwork-im-sdk`
- Trilang implementation-aligned worktree: `.worktrees/im-sdk-trilanguage-expansion`
- App SDK authority contract: `sdks/sdkwork-im-sdk/openapi/craw-chat-app.openapi.yaml`
- App SDK derived generator input: `sdks/sdkwork-im-sdk/openapi/craw-chat-app.sdkgen.yaml`
- TypeScript composed SDK: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed/src`
- Flutter composed SDK: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed/lib`
- Rust composed SDK: `sdks/sdkwork-im-sdk/sdkwork-im-sdk-rust/composed/src`
- API routing authority: `services/local-minimal-node/src/node/build.rs`
- Public auth behavior and realtime constraints: app API tests and session gateway implementation under `services/local-minimal-node` and `services/session-gateway`

## Problem Statement

The existing documentation site already provides a reasonably strong API reference, but the SDK section is materially behind the current repository reality.

Current weaknesses:

- the SDK section is too shallow and reads like workspace notes instead of a consumer-facing product manual
- navigation does not reflect the real integration journey for SDK users
- the current language support page still communicates placeholder-era status rather than the current trilanguage delivery model
- API pages mention SDK alignment only lightly and do not guide readers from an API domain to the right SDK module
- documentation does not clearly enough separate generator-owned output from manual-owned SDK surface
- realtime documentation risks confusion between documented WebSocket transport semantics and the actual SDK implementation boundary for this round
- the main workspace and the trilanguage SDK worktree are currently not at the same implementation state, so documentation changes must be authored against the implementation-aligned worktree rather than against stale main-workspace assumptions

## Chosen Approach

Retain the current VitePress site and the existing API reference foundation, but redesign the information architecture so the SDK section becomes task-driven, module-driven, and language-aware.

Recommended approach:

1. Keep the current site shell and API reference foundation.
2. Rework the homepage and top navigation so SDK integration becomes a first-class entrypoint.
3. Expand the SDK section into overview, quick starts, auth/init guidance, module map, module reference pages, ownership rules, and scenario examples.
4. Add explicit SDK mapping sections into API overview pages so readers can move in both directions between API operations and SDK modules.
5. Document only behavior that is implemented and verified in the repository, with precise status labeling where a workspace exists but publication or transport adapters do not.

This keeps the delivery risk low while producing documentation that reads like a mature public SDK portal rather than a generated appendix.

## Repository Alignment Rule

At the time of this design, the current main workspace does not yet mirror the full trilanguage SDK state, while `.worktrees/im-sdk-trilanguage-expansion` does contain the Rust SDK workspace and the latest trilanguage app SDK implementation.

Therefore:

- factual SDK claims for TypeScript, Flutter, and Rust must be derived from the worktree state that actually contains those implementations
- documentation implementation should be authored in the trilanguage SDK worktree so the site stays aligned with real code
- documentation must not be "backported" into a workspace snapshot that would make the published docs describe code that is not present there

## Information Architecture

The documentation should prioritize the integration path below:

1. What IM SDKs exist
2. Which language package or crate should I start with
3. How do I authenticate and initialize the client
4. Which module solves my use case
5. Which API domain backs that module
6. What is generated versus manual and how do I contribute safely

The top navigation should be reordered to better support that path:

- `Getting Started`
- `SDK`
- `API Reference`
- `Architecture`
- `Deployment`
- `Reference`

`Features` should no longer be a top-level priority item for the main developer journey. Capability storytelling can be absorbed into the homepage and architecture material.

## SDK Section Structure

The SDK section should become a complete subsystem rather than a short appendix.

Required persistent pages:

- `sdk/index.md`
  SDK landing page with consumer positioning, language matrix, module matrix, delivery-state guidance, and entry links.
- `sdk/app-sdk.md`
  App SDK family overview, package identities, supported integration scope, auth model, and release/ownership summary.
- `sdk/language-support.md`
  Language matrix and package naming, focused on actual consumption rather than placeholder release-wave wording.
- `sdk/auth-and-client-init.md`
  Bearer-token model, client initialization flow, environment selection, and shared initialization patterns.
- `sdk/module-map.md`
  One-page map from user tasks to SDK modules and corresponding API domains.
- `sdk/generation-and-ownership.md`
  Precise split between `generated/server-openapi` and manual-owned `composed` surfaces, including regeneration rules.
- `sdk/admin-sdk.md`
  Retained as a secondary page for control-plane audience separation, but not treated as part of the primary app SDK onboarding path.

Required language quick starts:

- `sdk/typescript-quick-start.md`
- `sdk/flutter-quick-start.md`
- `sdk/rust-quick-start.md`

Each quick start should cover:

- package or crate identity
- install or dependency wiring
- client creation
- auth token injection
- one realistic first read call
- one realistic first write call
- where to go next

Required module reference pages:

- `sdk/modules/session-and-presence.md`
- `sdk/modules/realtime.md`
- `sdk/modules/devices-and-inbox.md`
- `sdk/modules/conversations.md`
- `sdk/modules/messages.md`
- `sdk/modules/media.md`
- `sdk/modules/streams.md`
- `sdk/modules/rtc.md`

Each module page should standardize on:

- module purpose
- public entrypoints by language
- primary workflows
- mapped API domains or operations
- implementation status
- generated versus manual ownership notes
- common caveats
- concise, real examples

Required scenario pages:

- `sdk/examples/session-bootstrap.md`
- `sdk/examples/conversation-workflow.md`
- `sdk/examples/message-and-media.md`
- `sdk/examples/stream-and-rtc.md`

These pages should be task-driven and cross-language, showing how multiple modules combine in realistic integration flows.

## API Reference Alignment

The API reference remains important and should not be rewritten wholesale. Instead, it should be upgraded to cooperate with the SDK section.

Required API changes:

- `api-reference/index.md` should explain that the API reference and SDK reference are two views of the same implementation surface.
- `api-reference/app-api.md` should map app domains to SDK modules explicitly.
- `api-reference/auth-and-errors.md` should explain how SDK consumers should think about auth and error handling rather than documenting only raw HTTP expectations.
- app-domain pages should add short "SDK mapping" blocks that point to the relevant SDK module pages and quick starts.

The principle is:

- SDK pages answer "how do I integrate this capability"
- API pages answer "what operations and payloads exist under the hood"

## Delivery-State Rules

Documentation trust depends on explicit delivery-state labeling. Every SDK-facing page should use the same vocabulary:

- `Implemented and verified`
  The SDK surface exists in the current repository and has been validated through verification workflows.
- `Generated and verified`
  The surface exists as generated output and is part of the verified generated layer.
- `Documented contract only`
  The API or transport contract is documented, but no consumer-facing SDK implementation is claimed.
- `Workspace present but not published`
  A package or crate workspace exists locally but publication to ecosystem registries is not being claimed.

Required boundary statements:

- app auth for public SDK consumers is bearer-token only
- trusted internal headers are not the public integration contract
- `generated/server-openapi` is generator-owned and must not be edited in place
- `composed` is manual-owned and is the preferred public SDK surface
- WebSocket realtime transport semantics may be documented, but a handwritten realtime adapter must not be claimed as implemented unless the repository clearly contains it

## Editorial Standards

The rewritten SDK documentation should read like a mature public developer portal.

Required editorial rules:

- write for external integrators, not internal workspace maintainers
- prefer task-driven page openings over directory listings
- use exact package and crate names from the repository
- use exact module names from the composed SDKs where possible
- do not present unpublished artifacts as generally available releases
- avoid placeholder wording such as "template only pending generation" when the repository already contains real generated and composed SDK surfaces; instead explain the actual local delivery state and publication boundary clearly
- keep code samples grounded in current SDK shape and auth rules
- ensure pages are scannable on desktop and mobile

## Navigation And Sidebar Design

The SDK sidebar should expand from a four-page section into a complete integration map.

Recommended SDK sidebar order:

1. Overview
2. App SDK
3. Language Support
4. TypeScript Quick Start
5. Flutter Quick Start
6. Rust Quick Start
7. Auth and Client Init
8. Module Map
9. Generation and Ownership
10. Session and Presence
11. Realtime
12. Devices and Inbox
13. Conversations
14. Messages
15. Media
16. Streams
17. RTC
18. Session Bootstrap
19. Conversation Workflow
20. Message and Media
21. Stream and RTC

This ordering supports both first-time onboarding and targeted lookup.

## Page Inventory Changes

Pages to rewrite substantially:

- `docs/sites/index.md`
- `docs/sites/.vitepress/config.ts`
- `docs/sites/sdk/index.md`
- `docs/sites/sdk/app-sdk.md`
- `docs/sites/sdk/admin-sdk.md`
- `docs/sites/sdk/language-support.md`
- `docs/sites/api-reference/index.md`
- `docs/sites/api-reference/app-api.md`
- `docs/sites/api-reference/auth-and-errors.md`

Pages to add:

- `docs/sites/sdk/typescript-quick-start.md`
- `docs/sites/sdk/flutter-quick-start.md`
- `docs/sites/sdk/rust-quick-start.md`
- `docs/sites/sdk/auth-and-client-init.md`
- `docs/sites/sdk/module-map.md`
- `docs/sites/sdk/generation-and-ownership.md`
- `docs/sites/sdk/modules/session-and-presence.md`
- `docs/sites/sdk/modules/realtime.md`
- `docs/sites/sdk/modules/devices-and-inbox.md`
- `docs/sites/sdk/modules/conversations.md`
- `docs/sites/sdk/modules/messages.md`
- `docs/sites/sdk/modules/media.md`
- `docs/sites/sdk/modules/streams.md`
- `docs/sites/sdk/modules/rtc.md`
- `docs/sites/sdk/examples/session-bootstrap.md`
- `docs/sites/sdk/examples/conversation-workflow.md`
- `docs/sites/sdk/examples/message-and-media.md`
- `docs/sites/sdk/examples/stream-and-rtc.md`

## Verification Strategy

Documentation completion is not "content written". It requires structural and factual verification.

Required checks:

- confirm page claims against current SDK source trees under `sdks/sdkwork-im-sdk`
- when working in the trilanguage SDK effort, confirm those claims against `.worktrees/im-sdk-trilanguage-expansion/sdks/sdkwork-im-sdk`
- confirm package and crate names against manifests
- confirm module names against composed SDK source files
- confirm API mappings against app routing and current API reference organization
- build the VitePress site successfully from `docs/sites`
- review navigation, sidebar integrity, and internal links after the build

## Risks And Controls

- Risk: documentation outruns implementation and claims SDK capabilities not actually present.
  Control: derive examples, module names, and package identities directly from current source files.

- Risk: publication status is described inaccurately.
  Control: distinguish local workspace availability from public registry publication on every overview page that needs it.

- Risk: realtime documentation overstates transport support.
  Control: explicitly separate HTTP coordination APIs from any claimed SDK WebSocket adapter.

- Risk: the SDK section becomes too large and loses information scent.
  Control: keep overview pages navigational, keep module pages focused, and rely on module map plus scenario pages to route users quickly.

- Risk: API and SDK sections drift apart.
  Control: add reciprocal links and mapping notes in both directions.

## Success Criteria

This documentation round is complete only when all of the following are true:

- the site presents SDK integration as a first-class entrypoint
- TypeScript, Flutter, and Rust all have credible quick-start guidance grounded in real package or crate identities
- every major SDK module has a dedicated reference page with API mappings
- generated versus manual ownership boundaries are explicit and hard to misread
- API overview pages guide readers to the corresponding SDK surfaces
- the site builds successfully and reads like a professional open-source developer portal rather than an internal workspace note set
