# Craw Chat SDK Docs Professionalization Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Transform `docs/sites` into a professional SDK-first documentation portal aligned with the implemented TypeScript, Flutter, and Rust Craw Chat app SDKs in the trilanguage SDK worktree.

**Architecture:** Keep the existing VitePress site and API page generator intact, but rebuild the navigation, homepage, and SDK section around integration tasks rather than workspace notes. Author all SDK-facing content from the real worktree manifests, module exports, and composed SDK entrypoints, then add reciprocal SDK mapping blocks in the App API reference so API and SDK documentation stay synchronized.

**Tech Stack:** VitePress 1.6, Markdown, TypeScript theme config, existing `docs:generate` / `docs:build` / `docs:verify` scripts, trilanguage Craw Chat SDK workspace under `.worktrees/craw-chat-sdk-trilanguage-expansion`

---

## Spec Basis

- Approved design spec in the main workspace:
  `D:\javasource\spring-ai-plus\spring-ai-plus-business\apps\craw-chat\docs\superpowers\specs\2026-04-14-craw-chat-sdk-docs-professionalization-design.md`
- Implementation branch and worktree:
  `.worktrees/craw-chat-sdk-trilanguage-expansion`

## File Structure

### Site Shell And Landing Pages

- Modify: `docs/sites/.vitepress/config.ts`
  Reorder top navigation and expand the SDK sidebar into quick starts, module pages, and scenario examples.
- Modify: `docs/sites/index.md`
  Reposition the homepage around SDK integration, implementation alignment, and primary developer entry points.

### Core SDK Overview Pages

- Modify: `docs/sites/sdk/index.md`
  SDK landing page with family matrix, language matrix, delivery-state model, and guided paths.
- Modify: `docs/sites/sdk/app-sdk.md`
  App SDK overview with package identities, audience, auth, ownership boundary, and module summary.
- Modify: `docs/sites/sdk/admin-sdk.md`
  Keep as a secondary control-plane page; make clear it is not the primary app integration path.
- Modify: `docs/sites/sdk/language-support.md`
  Present TypeScript, Flutter, and Rust support based on actual local worktree packages rather than stale placeholder wording.

### SDK Onboarding Pages

- Create: `docs/sites/sdk/typescript-quick-start.md`
- Create: `docs/sites/sdk/flutter-quick-start.md`
- Create: `docs/sites/sdk/rust-quick-start.md`
- Create: `docs/sites/sdk/auth-and-client-init.md`
- Create: `docs/sites/sdk/module-map.md`
- Create: `docs/sites/sdk/generation-and-ownership.md`

These pages explain package installation, client creation, bearer auth, module discovery, and generated/manual ownership rules.

### SDK Module Reference Pages

- Create: `docs/sites/sdk/modules/session-and-presence.md`
- Create: `docs/sites/sdk/modules/realtime.md`
- Create: `docs/sites/sdk/modules/devices-and-inbox.md`
- Create: `docs/sites/sdk/modules/conversations.md`
- Create: `docs/sites/sdk/modules/messages.md`
- Create: `docs/sites/sdk/modules/media.md`
- Create: `docs/sites/sdk/modules/streams.md`
- Create: `docs/sites/sdk/modules/rtc.md`

Each module page should follow one shared pattern: purpose, public entrypoints by language, mapped App API pages, ownership boundary, implementation status, and concise real examples.

### SDK Scenario Pages

- Create: `docs/sites/sdk/examples/session-bootstrap.md`
- Create: `docs/sites/sdk/examples/conversation-workflow.md`
- Create: `docs/sites/sdk/examples/message-and-media.md`
- Create: `docs/sites/sdk/examples/stream-and-rtc.md`

These pages show realistic multi-module integration flows and point back to the module pages for detail.

### API Alignment Pages

- Modify: `docs/sites/api-reference/index.md`
- Modify: `docs/sites/api-reference/app-api.md`
- Modify: `docs/sites/api-reference/auth-and-errors.md`
- Modify: `docs/sites/api-reference/app/session-and-realtime.md`
- Modify: `docs/sites/api-reference/app/device-sync.md`
- Modify: `docs/sites/api-reference/app/conversations.md`
- Modify: `docs/sites/api-reference/app/membership-and-read-state.md`
- Modify: `docs/sites/api-reference/app/messages.md`
- Modify: `docs/sites/api-reference/app/media.md`
- Modify: `docs/sites/api-reference/app/streams.md`
- Modify: `docs/sites/api-reference/app/rtc.md`

These changes add SDK mapping blocks and make auth/error guidance useful for SDK consumers, not just raw HTTP callers.

### Verification Surface

- Use: `docs/sites/package.json`
  Existing verification entrypoints: `npm run docs:build` and `npm run docs:verify`

---

### Task 1: Rebuild the site shell and SDK route skeleton

**Files:**
- Modify: `docs/sites/.vitepress/config.ts`
- Create: `docs/sites/sdk/typescript-quick-start.md`
- Create: `docs/sites/sdk/flutter-quick-start.md`
- Create: `docs/sites/sdk/rust-quick-start.md`
- Create: `docs/sites/sdk/auth-and-client-init.md`
- Create: `docs/sites/sdk/module-map.md`
- Create: `docs/sites/sdk/generation-and-ownership.md`
- Create: `docs/sites/sdk/modules/session-and-presence.md`
- Create: `docs/sites/sdk/modules/realtime.md`
- Create: `docs/sites/sdk/modules/devices-and-inbox.md`
- Create: `docs/sites/sdk/modules/conversations.md`
- Create: `docs/sites/sdk/modules/messages.md`
- Create: `docs/sites/sdk/modules/media.md`
- Create: `docs/sites/sdk/modules/streams.md`
- Create: `docs/sites/sdk/modules/rtc.md`
- Create: `docs/sites/sdk/examples/session-bootstrap.md`
- Create: `docs/sites/sdk/examples/conversation-workflow.md`
- Create: `docs/sites/sdk/examples/message-and-media.md`
- Create: `docs/sites/sdk/examples/stream-and-rtc.md`
- Test: `docs/sites/package.json` via `npm run docs:build`

- [ ] **Step 1: Create the new SDK page skeletons**

Create every new SDK page with stable top-level headings and shared section placeholders so the site structure exists before copywriting begins.

Use a minimal page skeleton like:

```md
# TypeScript Quick Start

## Audience

## Package

## Install

## Create a client

## First request

## Next steps
```

For module pages, use:

```md
# Messages Module

## What this module is for

## Public entrypoints

## API mapping

## Common workflows

## Ownership and status

## Example
```

- [ ] **Step 2: Update the VitePress nav and SDK sidebar**

Change `docs/sites/.vitepress/config.ts` so the top nav order becomes:

```ts
[
  { text: "Getting Started", link: "/getting-started/index" },
  { text: "SDK", link: "/sdk/index" },
  { text: "API Reference", link: "/api-reference/index" },
  { text: "Architecture", link: "/architecture/overview" },
  { text: "Deployment", link: "/deployment/index" },
  { text: "Reference", link: "/reference/cli-and-scripts" },
]
```

Expand the `/sdk/` sidebar to include overview pages, quick starts, module pages, and scenario pages. Keep `Admin SDK` present, but place it below the app-first path rather than near the top.

- [ ] **Step 3: Run the docs build to validate the new route skeleton**

Run:

```bash
npm run docs:build
```

Working directory: `docs/sites`

Expected: PASS. The site should build with the expanded SDK section and without broken config imports.

- [ ] **Step 4: Commit**

```bash
git add docs/sites/.vitepress/config.ts docs/sites/sdk
git commit -m "docs(site): scaffold sdk-first navigation"
```

### Task 2: Rewrite the homepage and core SDK overview pages

**Files:**
- Modify: `docs/sites/index.md`
- Modify: `docs/sites/sdk/index.md`
- Modify: `docs/sites/sdk/app-sdk.md`
- Modify: `docs/sites/sdk/admin-sdk.md`
- Modify: `docs/sites/sdk/language-support.md`
- Test: `docs/sites/package.json` via `npm run docs:build`

- [ ] **Step 1: Rewrite the homepage around developer entry paths**

Replace the current landing copy so the homepage emphasizes:

- SDK-first onboarding
- implementation-aligned documentation
- trilanguage app SDK availability in this worktree
- direct links to quick starts, module map, App API overview, and deployment

The page should keep the professional open-source tone but change the action order from general platform framing to practical integration routing.

- [ ] **Step 2: Rewrite `sdk/index.md` and `sdk/app-sdk.md`**

Use real package and crate identities:

```md
- TypeScript composed: `@sdkwork/craw-chat-sdk`
- Flutter composed: `craw_chat_sdk`
- Rust composed: `craw-chat-sdk`
- TypeScript generated: `@sdkwork/craw-chat-backend-sdk`
- Flutter generated: `backend_sdk`
- Rust generated: `sdkwork-craw-chat-backend-sdk`
```

Document the delivery-state vocabulary explicitly:

```md
- Implemented and verified
- Generated and verified
- Documented contract only
- Workspace present but not published
```

- [ ] **Step 3: Rewrite `sdk/language-support.md` and `sdk/admin-sdk.md`**

For language support:

- explain that TypeScript, Flutter, and Rust are all implemented in the current worktree
- separate local workspace availability from ecosystem publication
- link each language to its quick-start page

For `admin-sdk.md`:

- keep the audience separation
- make clear it is a secondary control-plane document
- remove any implication that it is part of the primary app SDK onboarding path

- [ ] **Step 4: Run the docs build**

Run:

```bash
npm run docs:build
```

Working directory: `docs/sites`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add docs/sites/index.md docs/sites/sdk/index.md docs/sites/sdk/app-sdk.md docs/sites/sdk/admin-sdk.md docs/sites/sdk/language-support.md
git commit -m "docs(site): rewrite sdk landing and overview pages"
```

### Task 3: Add auth and language quick starts grounded in the real SDK surfaces

**Files:**
- Modify: `docs/sites/sdk/auth-and-client-init.md`
- Modify: `docs/sites/sdk/typescript-quick-start.md`
- Modify: `docs/sites/sdk/flutter-quick-start.md`
- Modify: `docs/sites/sdk/rust-quick-start.md`
- Test: `docs/sites/package.json` via `npm run docs:build`

- [ ] **Step 1: Write the shared auth and initialization page**

Base `auth-and-client-init.md` on the real public contract:

- bearer-token auth only
- `generated/server-openapi` is generator-owned
- `composed` is the preferred public SDK layer
- WebSocket transport semantics may be documented, but the SDK round is still HTTP-coordination-first

Include a shared initialization table:

```md
| Language | Preferred client | Auth setter |
| --- | --- | --- |
| TypeScript | `new CrawChatClient(...)` | `client.setAuthToken(token)` |
| Flutter | `CrawChatClient.create(...)` | `client.setAuthToken(token)` |
| Rust | `CrawChatClient::new_with_base_url(...)` | `client.set_auth_token(token)` |
```

- [ ] **Step 2: Write the TypeScript quick start**

Use the actual package and entrypoint names:

```ts
import { CrawChatClient } from "@sdkwork/craw-chat-sdk";

const client = new CrawChatClient({
  baseUrl: "http://127.0.0.1:18090",
  authToken: process.env.CRAW_CHAT_TOKEN,
});
```

Then show one realistic first read and one realistic first write rooted in the current modules.

- [ ] **Step 3: Write the Flutter and Rust quick starts**

Flutter should use the real public factory:

```dart
final client = CrawChatClient.create(
  baseUrl: 'http://127.0.0.1:18090',
  authToken: token,
);
```

Rust should use the real composed crate surface:

```rust
use craw_chat_sdk::CrawChatClient;

let client = CrawChatClient::new_with_base_url("http://127.0.0.1:18090")?;
client.set_auth_token(token);
```

In both pages, use a realistic first request and one next-step link list rather than generic marketing copy.

- [ ] **Step 4: Run the docs build**

Run:

```bash
npm run docs:build
```

Working directory: `docs/sites`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add docs/sites/sdk/auth-and-client-init.md docs/sites/sdk/typescript-quick-start.md docs/sites/sdk/flutter-quick-start.md docs/sites/sdk/rust-quick-start.md
git commit -m "docs(sdk): add auth and trilanguage quick starts"
```

### Task 4: Add the module map and full SDK module reference set

**Files:**
- Modify: `docs/sites/sdk/module-map.md`
- Modify: `docs/sites/sdk/modules/session-and-presence.md`
- Modify: `docs/sites/sdk/modules/realtime.md`
- Modify: `docs/sites/sdk/modules/devices-and-inbox.md`
- Modify: `docs/sites/sdk/modules/conversations.md`
- Modify: `docs/sites/sdk/modules/messages.md`
- Modify: `docs/sites/sdk/modules/media.md`
- Modify: `docs/sites/sdk/modules/streams.md`
- Modify: `docs/sites/sdk/modules/rtc.md`
- Test: `docs/sites/package.json` via `npm run docs:build`

- [ ] **Step 1: Write the module map**

Turn `module-map.md` into the routing table from business task to SDK module and App API page:

```md
| I need to... | SDK module | Primary page | App API domain |
| --- | --- | --- | --- |
| resume a user session | `session` | `/sdk/modules/session-and-presence` | `/api-reference/app/session-and-realtime` |
| pull realtime events | `realtime` | `/sdk/modules/realtime` | `/api-reference/app/session-and-realtime` |
| post a text message | `messages` or `conversations` | `/sdk/modules/messages` | `/api-reference/app/messages` |
```

- [ ] **Step 2: Write the transport, presence, and conversation-adjacent module pages**

Populate:

- `session-and-presence.md`
- `realtime.md`
- `devices-and-inbox.md`
- `conversations.md`

Each page must include:

- the public entrypoints by language
- the mapped API pages
- implementation status
- the exact generated/manual boundary for that capability

- [ ] **Step 3: Write the message, media, stream, and RTC module pages**

Populate:

- `messages.md`
- `media.md`
- `streams.md`
- `rtc.md`

Use real module naming from the composed SDKs and call out the builder surfaces where they exist:

```md
- TypeScript exports `builders`
- Flutter exports `builders.dart`
- Rust re-exports `build_*` helpers from `builders.rs`
```

- [ ] **Step 4: Run the docs build**

Run:

```bash
npm run docs:build
```

Working directory: `docs/sites`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add docs/sites/sdk/module-map.md docs/sites/sdk/modules
git commit -m "docs(sdk): add module map and module reference pages"
```

### Task 5: Add scenario-driven SDK example pages

**Files:**
- Modify: `docs/sites/sdk/examples/session-bootstrap.md`
- Modify: `docs/sites/sdk/examples/conversation-workflow.md`
- Modify: `docs/sites/sdk/examples/message-and-media.md`
- Modify: `docs/sites/sdk/examples/stream-and-rtc.md`
- Test: `docs/sites/package.json` via `npm run docs:build`

- [ ] **Step 1: Write the session bootstrap example**

Show the first-run integration path:

- create client
- inject bearer token
- resume or establish app session
- start the initial presence or realtime coordination flow

Link outward to `auth-and-client-init.md`, `session-and-presence.md`, and `realtime.md`.

- [ ] **Step 2: Write the conversation and message examples**

Populate:

- `conversation-workflow.md`
- `message-and-media.md`

Each page should show a realistic multi-step flow instead of isolated one-line snippets:

```md
1. create or locate a conversation
2. list members or inbox state
3. send a text message
4. attach media or update read cursor
```

- [ ] **Step 3: Write the stream and RTC example**

Populate `stream-and-rtc.md` with:

- open stream
- append frames
- checkpoint or complete
- create RTC session
- post or receive RTC signaling

Be explicit that WebSocket transport is documented separately from the current SDK implementation boundary.

- [ ] **Step 4: Run the docs build**

Run:

```bash
npm run docs:build
```

Working directory: `docs/sites`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add docs/sites/sdk/examples
git commit -m "docs(sdk): add scenario-driven integration examples"
```

### Task 6: Add SDK mapping and consumer guidance to the App API reference

**Files:**
- Modify: `docs/sites/api-reference/index.md`
- Modify: `docs/sites/api-reference/app-api.md`
- Modify: `docs/sites/api-reference/auth-and-errors.md`
- Modify: `docs/sites/api-reference/app/session-and-realtime.md`
- Modify: `docs/sites/api-reference/app/device-sync.md`
- Modify: `docs/sites/api-reference/app/conversations.md`
- Modify: `docs/sites/api-reference/app/membership-and-read-state.md`
- Modify: `docs/sites/api-reference/app/messages.md`
- Modify: `docs/sites/api-reference/app/media.md`
- Modify: `docs/sites/api-reference/app/streams.md`
- Modify: `docs/sites/api-reference/app/rtc.md`
- Test: `docs/sites/package.json` via `npm run docs:build`

- [ ] **Step 1: Rewrite the API overview pages to cooperate with the SDK section**

Update `api-reference/index.md` and `app-api.md` so they explicitly explain:

- API Reference = operation-level truth
- SDK Reference = integration-level guidance
- readers can move in both directions between those surfaces

Use short mapping blocks, not long duplicated prose.

- [ ] **Step 2: Update `auth-and-errors.md` for SDK consumers**

Add SDK-facing guidance for:

- bearer token setup expectations
- common error envelope interpretation
- when to retry, re-authenticate, or re-resume session
- how transport notes relate to SDK behavior without overstating implementation

- [ ] **Step 3: Add SDK mapping blocks to each App API domain page**

For every App API domain page, add a concise block like:

```md
## SDK Mapping

- SDK module page: [/sdk/modules/messages](/sdk/modules/messages)
- Quick start: [/sdk/typescript-quick-start](/sdk/typescript-quick-start)
- Related example: [/sdk/examples/message-and-media](/sdk/examples/message-and-media)
```

Map each domain page to the correct SDK page:

- session/realtime -> `session-and-presence.md`, `realtime.md`
- device-sync -> `devices-and-inbox.md`
- conversations + membership -> `conversations.md`
- messages -> `messages.md`
- media -> `media.md`
- streams -> `streams.md`
- rtc -> `rtc.md`

- [ ] **Step 4: Run the docs build**

Run:

```bash
npm run docs:build
```

Working directory: `docs/sites`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add docs/sites/api-reference
git commit -m "docs(api): add sdk mappings and consumer guidance"
```

### Task 7: Polish language, link quality, and final documentation verification

**Files:**
- Modify: `docs/sites/index.md`
- Modify: `docs/sites/.vitepress/config.ts`
- Modify: `docs/sites/sdk/index.md`
- Modify: `docs/sites/sdk/app-sdk.md`
- Modify: `docs/sites/sdk/language-support.md`
- Modify: `docs/sites/api-reference/index.md`
- Modify: `docs/sites/api-reference/app-api.md`
- Modify: `docs/sites/api-reference/auth-and-errors.md`
- Modify: any new SDK pages that need copy, cross-link, or status-label cleanup after full review
- Test: `docs/sites/package.json` via `npm run docs:verify`
- Test: `docs/sites/package.json` via `npm run docs:build`

- [ ] **Step 1: Remove stale wording and align delivery-state vocabulary**

Search the rewritten SDK pages for stale placeholder phrasing and replace it with the approved status model.

Run:

```bash
rg "template_only_pending_generation|all four artifacts|generation and publication still pending" docs/sites/index.md docs/sites/sdk docs/sites/api-reference
```

Expected: either no matches or only intentional matches inside the secondary `admin-sdk.md` page where historical release-catalog context is still being documented explicitly.

- [ ] **Step 2: Check internal cross-links and content consistency**

Review the new pages for:

- consistent package and crate naming
- consistent status labels
- correct module-to-API mapping
- correct app-vs-admin audience separation

Use:

```bash
rg "CrawChatClient|@sdkwork/craw-chat-sdk|craw_chat_sdk|craw-chat-sdk|generated/server-openapi|Workspace present but not published" docs/sites/sdk docs/sites/api-reference docs/sites/index.md
```

- [ ] **Step 3: Run documentation verification**

Run:

```bash
npm run docs:verify
npm run docs:build
```

Working directory: `docs/sites`

Expected: PASS for both commands

- [ ] **Step 4: Commit**

```bash
git add docs/sites
git commit -m "docs(site): polish sdk professionalization pass"
```
