# Craw Chat Portal

`craw-chat-portal` is the tenant-facing IM operations portal for Craw Chat.

Current round goals:

- align the application structure with `sdkwork-router-portal`
- provide a professional tenant console for IM operations
- keep the data-access boundary replaceable so generated SDKs can be wired in later

The implementation in this round uses a self-contained SPA structure because the repository does not yet carry a mature frontend workspace or generated TypeScript SDK output for the tenant portal.

## Product Scope

The current portal ships these tenant workspaces:

- `dashboard`
- `conversations`
- `realtime`
- `media`
- `automation`
- `governance`

The shell now also provides:

- route-aware command deck metadata
- related-route quick jumps per workspace
- workspace pulse metrics in the masthead area
- live shell-posture and theme summaries in the command deck
- command-deck quick reset for customized shell posture
- last console route restore through shell preferences
- operator-selectable console entry strategy
- pinned default console module preference for standard shift entry
- operator-selectable sidebar module visibility

Public entry surfaces:

- `/`
- `/login`

Protected console entry surfaces:

- `/console`
- `/console/dashboard`
- `/console/conversations`
- `/console/realtime`
- `/console/media`
- `/console/automation`
- `/console/governance`

## Architecture

The application mirrors the structural intent of `sdkwork-router-portal` even though this repository does not yet contain the same frontend runtime stack.

### Core packages

- `packages/craw-chat-portal-core`
  Owns bootstrap, routing, shell layout, auth state, and shell state.
- `packages/craw-chat-portal-commons`
  Owns shared rendering primitives, formatting, and theme helpers.
- `packages/craw-chat-portal-types`
  Owns route keys, navigation group labels, and theme options.
- `packages/craw-chat-portal-portal-api`
  Owns the replaceable data boundary.

### Feature packages

- `packages/craw-chat-portal-dashboard`
- `packages/craw-chat-portal-conversations`
- `packages/craw-chat-portal-realtime`
- `packages/craw-chat-portal-media`
- `packages/craw-chat-portal-automation`
- `packages/craw-chat-portal-governance`
- `packages/craw-chat-portal-home`
- `packages/craw-chat-portal-auth`

Each feature package keeps the same internal split:

- `repository`
- `services`
- `page entrypoint`

That alignment now also applies to `home` and `auth`, so public entry surfaces follow the same feature-boundary discipline as console modules.

## Replaceable Data Boundary

The current round uses an HTTP-backed default data source that talks to the local portal/auth backend, while keeping the boundary shaped so generated SDK wiring can replace the transport later without rewriting feature packages.

Relevant files:

- `packages/craw-chat-portal-portal-api/src/index.js`
- `packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js`
- `packages/craw-chat-portal-portal-api/src/runtime/createPortalDataSource.js`
- `packages/craw-chat-portal-portal-api/src/runtime/dataSources/httpPortalDataSource.js`
- `packages/craw-chat-portal-portal-api/src/runtime/dataSources/mockPortalDataSource.js`
- `packages/craw-chat-portal-core/src/application/router/navigation.js`

Feature packages must consume `portal-api`; they should not issue raw HTTP calls directly.
That now includes the public `home` and `auth` surfaces, so tenant entry pages and console modules share the same replaceable control-plane seam.

The runtime data source can now be swapped explicitly for future SDK-backed work:

- `getActivePortalDataSource()`
- `setActivePortalDataSource(overrides)`
- `resetActivePortalDataSource()`

Runtime overrides are validated before activation so the swap payload itself must be a plain object, required control-plane methods cannot be replaced with invalid non-function values, typoed override keys fail fast instead of being accepted silently, and the active runtime data source stays immutable to consumers. The exported `activePortalDataSource` surface also remains enumerable and introspection-safe for future SDK-backed consumers that inspect available seam methods before wiring adapters. Auth bootstrap and sign-in flows now also fail closed on malformed session or workspace payloads instead of persisting or promoting partial auth state, and session-token persistence rejects malformed values before touching browser storage while clearing malformed stored values during reads.

`mockPortalDataSource` remains available for isolated tests and explicit runtime overrides, but it is no longer the default runtime path.

That makes `portal-api` the only integration seam that needs to change once the admin-facing TypeScript SDK is actually generated and materialized.

## Runtime Resilience

The shell now handles control-plane and snapshot instability without dropping into a blank screen:

- bootstrap loading state while workspace session hydration is in flight
- public home/login routes keep their own tenant-facing loading states while public snapshots are in flight
- console recovery state when bootstrap fails, with a real retry path back through auth hydration
- stale persisted session tokens are cleared when bootstrap explicitly reports that no valid tenant session can be restored
- persisted session tokens are also cleared when hydrate resolves a malformed workspace payload, preventing repeated recovery loops on bad control-plane data
- public home/login routes degrade gracefully to anonymous rendering when session bootstrap fails, instead of blocking the tenant entry experience on auth hydration
- module recovery state when a route snapshot fails, with retry that preserves the operator shell
- demo sign-in recovery state when tenant sign-in fails
- public home/login recovery states now keep tenant-facing copy instead of falling back to console-module failure language
- feature service layers reject malformed module snapshots and malformed snapshot items before undefined fields can leak into shell tables or cards
- public entry builders reject malformed home hero/auth control-plane payloads before undefined copy leaks into the tenant-facing pages
- stale async render protection so slower route loads cannot overwrite a newer navigation result
- HTML escaping across tenant-controlled shell labels, table cells, snapshot hero copy, and auth control-plane copy

These guards live mainly in:

- `packages/craw-chat-portal-core/src/application/app/PortalProductApp.js`
- `tests/portal-routing-and-state.test.mjs`
- `tests/portal-api-boundary.test.mjs`
- `tests/portal-html-safety.test.mjs`

## Local Commands

From `apps/craw-chat-portal`:

```powershell
node scripts/build.mjs
node scripts/preview.mjs --root . --port 4176
node scripts/preview.mjs --root dist --port 4176
node --test --experimental-test-isolation=none tests/*.test.mjs
```

## Verification

The current portal is backed by automated checks for:

- architecture alignment and route manifest shape
- feature view-model completeness
- routing restoration and shell persistence
- portal-api boundary governance
- shell command deck metadata, rendering, and posture summaries
- HTML safety for shell, table, and page snapshot rendering
- standalone build output generation

Tests live under `tests/`.
