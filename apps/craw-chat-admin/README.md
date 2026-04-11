# Craw Chat Admin

`sdkwork-craw-chat-admin` is the standalone React and Tauri workspace for the Craw Chat即时通信管理后台.

## Architecture Goals

- use `@sdkwork/ui-pc-react` as the single shared UI foundation
- keep the root app thin and move real composition into workspace packages
- remove legacy local UI layers and avoid compatibility baggage
- fully mirror the proven `sdkwork-router-admin` workspace architecture standard inside the Craw Chat domain
- deliver one coherent operator shell across login, overview, tenancy, identity, moderation, automation, realtime, system governance, and settings
- keep the runtime boundary honest when no compatible `/api/admin/*` backend is configured

## Workspace Layout

```text
apps/craw-chat-admin/
|- src/        # root app bootstrap only
|- packages/   # foundation and business modules
|- tests/      # architecture and product-surface verification
|- src-tauri/  # desktop host and native commands
`- dist/       # production build output
```

## Package Map

### Foundation

- `sdkwork-craw-chat-admin-types`
- `sdkwork-craw-chat-admin-core`
- `sdkwork-craw-chat-admin-shell`
- `sdkwork-craw-chat-admin-admin-api`

### Business

- `sdkwork-craw-chat-admin-auth`
- `sdkwork-craw-chat-admin-overview`
- `sdkwork-craw-chat-admin-users`
- `sdkwork-craw-chat-admin-tenants`
- `sdkwork-craw-chat-admin-moderation`
- `sdkwork-craw-chat-admin-groups`
- `sdkwork-craw-chat-admin-messages`
- `sdkwork-craw-chat-admin-system`
- `sdkwork-craw-chat-admin-settings`
- `sdkwork-craw-chat-admin-conversations`

## UI Standard

- shared styles come from `@sdkwork/ui-pc-react/styles.css`
- shell composition uses `DesktopShellFrame`, `NavigationRail`, `Toolbar`, `SettingsCenter`, and other shared primitives
- local shell CSS is limited to host layout selectors in `packages/sdkwork-craw-chat-admin-shell/src/styles/shell-host.css`
- theme state, locale, sidebar behavior, and workspace continuity are owned by `sdkwork-craw-chat-admin-core`

## Product Surfaces

- `Login`: operator sign-in with an isolated auth shell
- `Overview`: platform posture, queue pressure, SLA watchpoints, and incident summaries
- `Tenants`: tenant, workspace, project, and key-governance workflows
- `Users`: operator accounts, portal identities, and lifecycle control
- `Conversations`: conversation freeze, handoff, archive, and ownership governance
- `Messages`: transcript search, evidence export, and audit review
- `Groups`: group directory, membership posture, and channel governance
- `Moderation`: reports, policy interventions, keyword governance, and escalation visibility
- `Automation`: bot registry, workflow runs, and retry oversight
- `Announcements`: broadcast operations and delivery posture
- `Realtime`: session health, RTC posture, and reconnect watch
- `System`: protocol governance, runtime health, and compatibility posture
- `Settings`: shared operator settings center for appearance, locale, navigation, and workspace continuity

## Shell Model

- `react-router-dom` browser routes mounted under `/admin/`
- authenticated desktop shell with a shared top toolbar, left navigation rail, and right content canvas
- login isolated outside the authenticated shell
- command center, operations pulse, and route context strip persist across modules
- persisted theme mode, accent preset, sidebar width, collapse behavior, and hidden routes
- lazy-loaded page modules behind a shared loading state

## Commands

```bash
pnpm install
pnpm typecheck
pnpm build
pnpm dev
pnpm tauri:dev
pnpm tauri:build
```

## Runtime Contract

The admin frontend contract targets a compatible management backend that serves `/api/admin/*`.

- browser development: set `SDKWORK_ADMIN_PROXY_TARGET=http://host:port` before `pnpm dev`
- desktop runtime: set `SDKWORK_ADMIN_PROXY_TARGET=http://host:port` before `pnpm tauri:dev` or `pnpm tauri:build`
- explicit local demo mode: set `SDKWORK_ADMIN_SANDBOX=1` when you want an in-memory IM admin sandbox instead of a real `/api/admin/*` backend
- compatibility alias: `SDKWORK_ADMIN_BIND` is still accepted for existing operator scripts
- when no compatible backend is configured, both Vite dev mode and the desktop runtime return a structured `503` response for `/api/admin/*` instead of silently proxying to a fake local default
- sandbox mode is opt-in only and does not override a real `SDKWORK_ADMIN_PROXY_TARGET`

## Local Sandbox

The local admin sandbox is intended for product walkthroughs, shell verification, and package smoke validation when no real management backend is available.

- enable with `SDKWORK_ADMIN_SANDBOX=1`
- default credentials: `admin@sdkwork.local` / `ChangeMe123!`
- login, workspace hydration, tenant/project changes, API key issuance, and core operator reads run against an in-memory backend seeded from `dev/admin-sandbox-seed.json`
- sandbox state is ephemeral by design and resets whenever the Vite server or desktop runtime restarts

## Current Backend Reality

Inside the current `craw-chat` workspace, the discovered control-plane service binds `127.0.0.1:18081` and serves `/api/v1/control/*`.

- that service is useful for control-plane governance
- it is not a drop-in replacement for this admin app's `/api/admin/*` contract
- if you want this admin workspace to run against local services, you need a compatible adapter or backend that exposes the expected `/api/admin/*` surface, including login and admin resource routes

## Desktop Host

The desktop app uses the shared `sdkwork-api-product-runtime` instead of a hard-coded local web host.

That keeps the admin desktop app aligned with the `/admin/*` operator shell contract while preserving native IPC commands and embedded asset orchestration.

## Delivery Standard

- architecture, directory layout, and standalone workspace conventions are intentionally aligned with `sdkwork-router-admin`
- product language, modules, and operator workflows are rebuilt for professional即时通信运营与治理场景
- desktop packaging is verified through the local Tauri toolchain and uses embedded admin and portal assets
