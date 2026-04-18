# Craw Chat Admin

`sdkwork-craw-chat-admin` is the standalone React and Tauri operator workspace for Craw Chat.

It provides the browser and desktop shell for moderation, tenancy, identity, automation, realtime governance, and platform operations. The app must stay thin at the root and converge on the formal admin SDK boundary wherever the corresponding control-plane contract already exists.

## Architecture Goals

- use `@sdkwork/ui-pc-react` as the shared UI foundation
- keep the root app thin and move real composition into workspace packages
- route formal control-plane HTTP access through `@sdkwork/craw-chat-admin-sdk`
- avoid raw `fetch`, duplicated DTOs, and handwritten route maps inside app packages
- keep browser and desktop runtime behavior aligned
- preserve an explicit sandbox mode when no compatible management backend is available

## Workspace Layout

```text
apps/craw-chat-admin/
  src/          # root bootstrap only
  packages/     # shell, core, and business modules
  tests/        # architecture and product-surface verification
  dev/          # opt-in local admin sandbox
  src-tauri/    # desktop host and native commands
  dist/         # browser production output
```

## Package Map

### Foundation

- `sdkwork-craw-chat-admin-types`
- `sdkwork-craw-chat-admin-core`
- `sdkwork-craw-chat-admin-shell`

### Business

- `sdkwork-craw-chat-admin-auth`
- `sdkwork-craw-chat-admin-overview`
- `sdkwork-craw-chat-admin-tenants`
- `sdkwork-craw-chat-admin-users`
- `sdkwork-craw-chat-admin-conversations`
- `sdkwork-craw-chat-admin-messages`
- `sdkwork-craw-chat-admin-groups`
- `sdkwork-craw-chat-admin-moderation`
- `sdkwork-craw-chat-admin-automation`
- `sdkwork-craw-chat-admin-announcements`
- `sdkwork-craw-chat-admin-realtime`
- `sdkwork-craw-chat-admin-system`
- `sdkwork-craw-chat-admin-settings`

## Formal Admin SDK Boundary

The formal control-plane transport boundary is `@sdkwork/craw-chat-admin-sdk`.

- control-plane URLs, auth handling, and DTO ownership for `/api/v1/control/*` belong to the SDK layers
- generated OpenAPI transport code belongs under `sdks/sdkwork-craw-chat-sdk-admin/*/generated/server-openapi`
- ergonomic business-facing admin client code belongs under `sdks/sdkwork-craw-chat-sdk-admin/*/composed`
- app-local helpers may exist for React state, loaders, or UI wiring, but they must wrap the formal SDK rather than re-implement transport
- the current package now carries both generated `/api/v1/control/*` modules and manual-owned `/api/admin/*` adapter exports, so the admin app no longer needs a separate local admin API workspace package
- browser-only `/api/admin/*` routes are still a separate contract-promotion track until those routes gain the same checked-in OpenAPI authority as the control-plane surface

## UI Standard

- shared styles come from `@sdkwork/ui-pc-react/styles.css`
- shell composition uses shared UI primitives plus app-owned desktop layout modules
- local shell CSS is limited to host layout selectors and app-specific presentation
- theme state, locale, command palette, operations pulse, and route continuity are owned by `sdkwork-craw-chat-admin-core`

## Product Surfaces

- `Login`: operator sign-in and access recovery
- `Overview`: platform posture, queue pressure, incident watch, and handoff summaries
- `Tenants`: tenant, workspace, project, and API key governance
- `Users`: operator accounts, portal identities, device posture, and recovery review
- `Conversations`: handoff, archive, freeze, and ownership governance
- `Messages`: transcript search, evidence export, recall review, and retention guardrails
- `Groups`: directory, membership posture, and channel governance
- `Moderation`: reports, interventions, keyword policy, and escalation visibility
- `Automation`: bot registry, run history, retry oversight, and routing review
- `Announcements`: outbound notice operations and delivery posture
- `Realtime`: session health, RTC posture, reconnect watch, and failover readiness
- `System`: protocol governance, runtime health, and compatibility posture
- `Settings`: shared operator preferences and workspace continuity

## Commands

```bash
pnpm install
pnpm test:storage
pnpm test:admin
pnpm typecheck
pnpm build
pnpm verify
pnpm verify:storage
pnpm dev
pnpm tauri:dev
pnpm tauri:build
```

## Verification

Use the dedicated workspace scripts instead of ad-hoc test runner flags:

- `pnpm test:storage`
  Runs the storage sandbox contract tests and storage draft payload tests.
- `pnpm test:admin`
  Runs shell, product-surface, sandbox, SDK-boundary, and UI-resolution tests.
- `pnpm verify:storage`
  Runs storage-focused verification plus `typecheck` and `build`.
- `pnpm verify`
  Runs the broader admin verification baseline plus `typecheck` and `build`.

The workspace uses package-local wrappers so `pnpm run` and `npm.cmd run` remain stable on Windows even when plain `cmd.exe` does not expose the expected `node` binary on `PATH`.

## Runtime Contract

The browser and desktop admin shell target a compatible management backend that serves `/api/admin/*`.

- browser development: set `SDKWORK_ADMIN_PROXY_TARGET=http://host:port` before `pnpm dev`
- desktop runtime: set `SDKWORK_ADMIN_PROXY_TARGET=http://host:port` before `pnpm tauri:dev` or `pnpm tauri:build`
- set `CRAW_CHAT_PORTAL_API_BASE_URL=http://host:port` when the embedded portal shell or shared desktop runtime should point at a non-default app runtime endpoint
- explicit sandbox mode: set `SDKWORK_ADMIN_SANDBOX=1` when you want an in-memory admin sandbox instead of a real backend
- persistent storage sandbox mode: set `SDKWORK_ADMIN_SANDBOX=1` and `SDKWORK_ADMIN_SANDBOX_STORAGE_FILE=/absolute/path/storage-snapshots.json` when you want storage-management state to survive sandbox restarts
- compatibility alias: `SDKWORK_ADMIN_BIND` is still accepted by existing operator scripts
- when no compatible backend is configured, Vite dev mode and the desktop runtime return a structured `503` response for `/api/admin/*` instead of silently falling back to a fake default target

## Local Sandbox

The local admin sandbox is intended for shell walkthroughs, product verification, and package smoke checks when no management backend is available.

- enable with `SDKWORK_ADMIN_SANDBOX=1`
- default credentials: `admin@sdkwork.local` / `ChangeMe123!`
- login, workspace hydration, tenant changes, API key issuance, and core operator reads run against an in-memory backend seeded from `dev/admin-sandbox-seed.json`
- sandbox state is ephemeral by default and resets whenever the Vite server or desktop runtime restarts
- storage-management state can be persisted independently through `SDKWORK_ADMIN_SANDBOX_STORAGE_FILE`

## Current Backend Reality

Inside the current `craw-chat` workspace, the discovered control-plane service binds `127.0.0.1:18081` and serves `/api/v1/control/*`.

- that runtime is the current authority source for admin SDK OpenAPI 3.x capture
- it is not automatically a drop-in replacement for every browser `/api/admin/*` route expected by the admin shell
- formal SDK generation and documentation must track the real `/api/v1/control/*` surface instead of inventing routes
- if the browser shell still needs a compatibility adapter, that adapter must depend on `@sdkwork/craw-chat-admin-sdk` rather than introducing raw transport code inside app packages

## Storage Contract Reference

The current storage-management route contract is documented in `../../docs/sites/reference/admin-storage-contract.md`.

Use that page when you need the verified route catalog, payload semantics, whole-record tenant override rules, redaction guarantees, or sandbox persistence notes.

## Desktop Host

The desktop app uses the shared `sdkwork-api-product-runtime` instead of a hard-coded local web host.

That keeps the operator shell aligned with the same admin runtime contract while preserving native IPC commands and embedded asset orchestration.

## Delivery Standard

- architecture and workspace layout stay aligned with the broader SDKWork admin workspace conventions
- transport behavior is validated through the formal admin SDK boundary, not by duplicated local HTTP wrappers
- browser, desktop, sandbox, and control-plane integration must remain explicit, reviewable, and testable
