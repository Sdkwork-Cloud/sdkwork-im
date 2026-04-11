# Craw Chat Portal Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a professional tenant-facing IM management portal aligned with the `sdkwork-router-portal` architecture while keeping the data layer replaceable for future SDK integration.

**Architecture:** The portal is a self-contained SPA under `apps/craw-chat-portal` with a `packages/` decomposition that mirrors the reference portal. The `portal-api` package provides the only data-access seam in this round; feature packages consume repositories and services from that seam and render console pages through the core shell.

**Tech Stack:** Vanilla ESM, local Vite binary from `docs/sites/node_modules`, Node built-in test runner, CSS custom properties.

---

### Task 1: Lock the application scaffold

**Files:**
- Create: `apps/craw-chat-portal/package.json`
- Create: `apps/craw-chat-portal/index.html`
- Create: `apps/craw-chat-portal/vite.config.js`
- Create: `apps/craw-chat-portal/src/main.js`
- Create: `apps/craw-chat-portal/src/App.js`
- Create: `apps/craw-chat-portal/src/theme.css`
- Test: `apps/craw-chat-portal/tests/portal-architecture.test.mjs`

- [ ] **Step 1: Write the failing scaffold assertions**
- [ ] **Step 2: Run `node --test tests/portal-architecture.test.mjs` and confirm failure**
- [ ] **Step 3: Add the root app scaffold and theme entry**
- [ ] **Step 4: Re-run the architecture test**

### Task 2: Build the reference-aligned core shell

**Files:**
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-core/src/index.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-core/src/application/app/PortalProductApp.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-core/src/application/providers/AppProviders.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-core/src/application/router/routePaths.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-core/src/application/router/routeManifest.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-core/src/application/layouts/MainLayout.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-core/src/application/layouts/PortalSiteLayout.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-core/src/components/*.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-core/src/store/*.js`
- Test: `apps/craw-chat-portal/tests/portal-architecture.test.mjs`

- [ ] **Step 1: Add route-manifest expectations for the protected modules**
- [ ] **Step 2: Run the architecture test and capture the missing exports/files failure**
- [ ] **Step 3: Implement core routing, stores, shell primitives, and layout selection**
- [ ] **Step 4: Re-run the architecture test**

### Task 3: Add the replaceable portal API seam

**Files:**
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-portal-api/src/index.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-portal-api/src/mockData.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-types/src/index.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-commons/src/**/*.js`
- Test: `apps/craw-chat-portal/tests/portal-module-viewmodels.test.mjs`

- [ ] **Step 1: Write failing view-model tests against the expected portal data surface**
- [ ] **Step 2: Run `node --test tests/portal-module-viewmodels.test.mjs` and confirm failure**
- [ ] **Step 3: Implement the mock tenant dataset, auth bootstrap, and shared formatting/render helpers**
- [ ] **Step 4: Re-run the view-model test**

### Task 4: Build tenant IM feature packages

**Files:**
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-home/src/index.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-auth/src/index.js`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-dashboard/src/**/*`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-conversations/src/**/*`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-realtime/src/**/*`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-media/src/**/*`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-automation/src/**/*`
- Create: `apps/craw-chat-portal/packages/craw-chat-portal-governance/src/**/*`
- Test: `apps/craw-chat-portal/tests/portal-module-viewmodels.test.mjs`

- [ ] **Step 1: Keep the feature test red until each package exposes repository, service, and page entrypoints**
- [ ] **Step 2: Implement the six protected modules plus home and login pages**
- [ ] **Step 3: Re-run the feature test**

### Task 5: Verify buildability and finish product polish

**Files:**
- Create: `apps/craw-chat-portal/tests/portal-build-smoke.test.mjs`
- Modify: `apps/craw-chat-portal/src/theme.css`
- Modify: `apps/craw-chat-portal/README.md`

- [ ] **Step 1: Write the failing build smoke test**
- [ ] **Step 2: Run `node --test tests/portal-build-smoke.test.mjs` and confirm failure**
- [ ] **Step 3: Finish responsive polish, commands, and README guidance**
- [ ] **Step 4: Re-run all portal tests**
- [ ] **Step 5: Run `node ../../docs/sites/node_modules/vite/bin/vite.js build --config apps/craw-chat-portal/vite.config.js`**
