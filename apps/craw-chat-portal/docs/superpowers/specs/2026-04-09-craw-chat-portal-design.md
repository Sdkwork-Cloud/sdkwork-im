# Craw Chat Portal Design

## Context

`apps/craw-chat-portal` currently exists only as a placeholder. The requested outcome is a tenant-facing IM management portal that aligns structurally with `apps/sdkwork-api-router/apps/sdkwork-router-portal` while fitting the current repository reality:

- no mature frontend workspace exists under `apps/craw-chat-*`
- no generated TypeScript SDK output is present for the app-facing IM SDK
- the portal still needs a clean boundary so future SDK wiring does not require redesigning the app

## Approaches Considered

### 1. Direct React parity with the reference portal

Pros:

- closest surface-level match to `sdkwork-router-portal`
- easiest long-term convergence if the same UI stack is later introduced

Cons:

- the current repository does not ship the needed React toolchain and workspace packages locally
- installing missing packages is not a safe assumption in this environment

### 2. Self-contained SPA with mirrored package boundaries and a replaceable portal API boundary

Pros:

- preserves the architectural shape that matters: `core`, `commons`, `types`, `portal-api`, and feature packages
- does not depend on new package installation
- can ship immediately with a professional console experience
- future SDK integration only replaces repository and service internals

Cons:

- implementation technology differs from the reference app internals
- some reference runtime patterns such as React providers become lightweight equivalents

### 3. VitePress-driven pseudo-console inside `docs/sites`

Pros:

- reuses the only existing frontend dependency area in the repo

Cons:

- wrong product boundary
- weak route isolation
- difficult to evolve into a real portal application

## Decision

Use approach 2.

This round will mirror the reference portal architecture at the application and module boundary level, not at the exact framework level. The core alignment points are:

- root app entry with `src/main.js`, `src/App.js`, `index.html`, `vite.config.js`
- `packages/` decomposition with `core`, `commons`, `types`, `portal-api`, and feature packages
- route manifest and route path registry
- shell store and auth store
- public routes plus protected console routes
- feature packages that expose repository, service, and page entrypoints

## Information Architecture

### Public routes

- `home`: product overview and tenant value narrative
- `login`: demo tenant access surface

### Protected console routes

- `dashboard`: executive summary, queue pressure, SLA posture, next actions
- `conversations`: inbox, handoff flow, message operations, read-state governance
- `realtime`: session resume, presence, subscriptions, device sync, event backlog
- `media`: media lifecycle, stream sessions, RTC posture, provider readiness
- `automation`: workflow executions, notification delivery, operational campaigns
- `governance`: audit trail, provider health, runtime diagnostics, tenant compliance

## Module Boundaries

### `craw-chat-portal-core`

Owns application assembly:

- application bootstrap
- route resolution
- layout selection
- shell rendering
- auth and shell state

### `craw-chat-portal-portal-api`

Owns the replaceable data boundary:

- authentication bootstrap
- workspace snapshot
- feature snapshot readers
- mock data for the current round

No raw `fetch` calls will be baked into feature packages. When generated SDKs are ready, this package is the swap point.

### Feature packages

Each feature package owns:

- repository functions that read from `portal-api`
- service functions that build view models
- page renderers that turn view models into console sections

## UX Direction

The portal should feel like a tenant command center, not a generic dashboard kit:

- steel-and-signal palette with teal, cyan, amber, and neutral graphite
- layered shell with ambient gradients and restrained motion
- dense but readable operational cards
- mobile-safe stacked layout without losing console affordances

## Error Handling

- unauthenticated access redirects to `login`
- unknown routes redirect to `dashboard` or `home` depending on auth state
- stores expose fallback status copy so the shell can render operational guidance instead of blank states

## Testing Strategy

- architecture test for route manifest, package layout, and portal API boundary
- feature view-model test for dashboard, conversations, realtime, media, automation, and governance
- build smoke test through the local Vite binary already available in the repository

## Outcome

At the end of this round, `apps/craw-chat-portal` should be a self-contained, buildable tenant portal with the right long-term architecture and a polished IM management experience, ready for later SDK replacement at the `portal-api` layer.
