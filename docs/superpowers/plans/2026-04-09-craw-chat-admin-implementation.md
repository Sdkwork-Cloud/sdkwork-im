# Craw Chat Admin Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `apps/craw-chat-admin` as a standalone IM operator workspace that fully mirrors the `sdkwork-router-admin` architecture standard while replacing router business modules with professional IM admin modules.

**Architecture:** Start by codifying the target contract with failing architecture tests, then copy the reference workspace skeleton, rename the package graph, and recompose the shell, auth, route manifest, and module surfaces around the IM domain. Keep the root app thin and force all business access through `sdkwork-craw-chat-admin-admin-api` and the admin SDK boundary.

**Tech Stack:** React 19, TypeScript, Vite, Tailwind CSS v4, Tauri 2, Node built-in test runner, `@sdkwork/ui-pc-react`, `sdkwork-craw-chat-sdk-admin`

---

## File Structure

### Root Workspace

- Create: `apps/craw-chat-admin/package.json`
- Create: `apps/craw-chat-admin/pnpm-workspace.yaml`
- Create: `apps/craw-chat-admin/turbo.json`
- Create: `apps/craw-chat-admin/tsconfig.json`
- Create: `apps/craw-chat-admin/vite.config.ts`
- Create: `apps/craw-chat-admin/README.md`
- Create: `apps/craw-chat-admin/src/main.tsx`
- Create: `apps/craw-chat-admin/src/App.tsx`
- Create: `apps/craw-chat-admin/src/theme.css`
- Create: `apps/craw-chat-admin/src/vite-env.d.ts`
- Create: `apps/craw-chat-admin/src/types/*.d.ts`

### Foundation Packages

- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-types/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-core/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-shell/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-admin-api/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-auth/**`

### IM Business Packages

- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-overview/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-tenants/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-users/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-conversations/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-messages/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-groups/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-moderation/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-automation/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-announcements/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-realtime/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-system/**`
- Create: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-settings/**`

### Host And Verification

- Create: `apps/craw-chat-admin/src-tauri/**`
- Create: `apps/craw-chat-admin/tests/admin-architecture.test.mjs`
- Create: `apps/craw-chat-admin/tests/admin-product-experience.test.mjs`
- Create: `apps/craw-chat-admin/tests/admin-auth-surface.test.mjs`
- Create: `apps/craw-chat-admin/tests/admin-sdk-boundary.test.mjs`

### Supporting Docs

- Create: `apps/craw-chat-admin/docs/superpowers/specs/2026-04-09-craw-chat-admin-design.md`
- Create: `apps/craw-chat-admin/docs/superpowers/plans/2026-04-09-craw-chat-admin-implementation.md`

### Task 1: Lock The Workspace Contract With Failing Tests

**Files:**
- Create: `apps/craw-chat-admin/tests/admin-architecture.test.mjs`
- Create: `apps/craw-chat-admin/tests/admin-product-experience.test.mjs`
- Create: `apps/craw-chat-admin/tests/admin-auth-surface.test.mjs`
- Create: `apps/craw-chat-admin/tests/admin-sdk-boundary.test.mjs`

- [ ] **Step 1: Write the failing architecture and product-surface tests**

Add tests that assert:

- root workspace files exist
- IM admin foundation packages exist
- IM business packages exist
- route manifest exposes IM modules instead of router modules
- auth surface uses Craw Chat Admin language
- admin SDK boundary forbids raw admin fetch wrappers

- [ ] **Step 2: Run tests to verify they fail**

Run: `node --test apps/craw-chat-admin/tests/*.test.mjs`

Expected: FAIL because the workspace files and packages do not exist yet.

- [ ] **Step 3: Keep the failing tests as the contract**

Do not weaken expectations to match the empty workspace.

- [ ] **Step 4: Commit the test contract**

```bash
git add apps/craw-chat-admin/tests
git commit -m "test: define craw chat admin workspace contract"
```

### Task 2: Materialize The Reference Workspace Skeleton

**Files:**
- Create or modify: `apps/craw-chat-admin/package.json`
- Create or modify: `apps/craw-chat-admin/pnpm-workspace.yaml`
- Create or modify: `apps/craw-chat-admin/turbo.json`
- Create or modify: `apps/craw-chat-admin/tsconfig.json`
- Create or modify: `apps/craw-chat-admin/vite.config.ts`
- Create or modify: `apps/craw-chat-admin/src/**`
- Create or modify: `apps/craw-chat-admin/src-tauri/**`
- Create or modify: `apps/craw-chat-admin/packages/**`

- [ ] **Step 1: Copy the `sdkwork-router-admin` workspace into `apps/craw-chat-admin`**

Preserve directory topology:

- `src/`
- `packages/`
- `tests/`
- `src-tauri/`

- [ ] **Step 2: Rename package names, imports, route constants, and workspace identifiers**

Replace `sdkwork-router-admin` with `sdkwork-craw-chat-admin` across:

- `package.json`
- `tsconfig.json`
- package manifests
- import specifiers
- Tauri metadata
- README and product identifiers

- [ ] **Step 3: Run the architecture tests again**

Run: `node --test apps/craw-chat-admin/tests/admin-architecture.test.mjs`

Expected: still FAIL, but now on missing IM-specific modules and product language rather than missing workspace structure.

- [ ] **Step 4: Commit the skeleton bootstrap**

```bash
git add apps/craw-chat-admin
git commit -m "feat: bootstrap craw chat admin workspace skeleton"
```

### Task 3: Recompose Core, Shell, And Auth Around The IM Domain

**Files:**
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-types/src/index.ts`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-core/src/routes.ts`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-core/src/routePaths.ts`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-core/src/routeManifest.ts`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-core/src/index.tsx`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-shell/src/application/router/AppRoutes.tsx`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-shell/src/components/Sidebar.tsx`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-auth/src/index.tsx`
- Modify: `apps/craw-chat-admin/src/theme.css`

- [ ] **Step 1: Extend the failing tests with IM-specific route and auth expectations**

Assert route keys and labels for:

- overview
- tenants
- users
- conversations
- messages
- groups
- moderation
- automation
- announcements
- realtime
- system
- settings

- [ ] **Step 2: Run the targeted tests to verify red**

Run: `node --test apps/craw-chat-admin/tests/admin-auth-surface.test.mjs apps/craw-chat-admin/tests/admin-product-experience.test.mjs`

Expected: FAIL because router-specific routes and copy are still present.

- [ ] **Step 3: Implement the minimal IM route manifest, shell labels, and auth copy**

Keep:

- thin root app
- lazy shell route loading
- isolated auth routes
- route-manifest-driven navigation

Replace:

- router-specific navigation groups
- router-specific header branding
- login messaging and auth copy

- [ ] **Step 4: Re-run the targeted tests**

Run: `node --test apps/craw-chat-admin/tests/admin-auth-surface.test.mjs apps/craw-chat-admin/tests/admin-product-experience.test.mjs`

Expected: PASS.

- [ ] **Step 5: Commit the shell and auth conversion**

```bash
git add apps/craw-chat-admin
git commit -m "feat: convert admin shell and auth to craw chat domain"
```

### Task 4: Stand Up IM Product Modules With Operator-Grade Surfaces

**Files:**
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-overview/src/index.tsx`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-tenants/src/index.tsx`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-users/src/index.tsx`
- Create or modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-conversations/src/index.tsx`
- Create or modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-messages/src/index.tsx`
- Create or modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-groups/src/index.tsx`
- Create or modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-moderation/src/index.tsx`
- Create or modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-automation/src/index.tsx`
- Create or modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-announcements/src/index.tsx`
- Create or modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-realtime/src/index.tsx`
- Create or modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-system/src/index.tsx`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-settings/src/index.tsx`

- [ ] **Step 1: Strengthen the product-surface tests**

Write assertions that the pages expose operator-grade IM concepts:

- throughput and hotspots on overview
- tenant governance
- user/device posture
- conversation lifecycle
- message audit
- moderation queues
- automation runs
- realtime session posture
- protocol governance and compatibility matrix

- [ ] **Step 2: Run the product-surface tests to verify red**

Run: `node --test apps/craw-chat-admin/tests/admin-product-experience.test.mjs`

Expected: FAIL because the copied router pages still expose router business language.

- [ ] **Step 3: Implement the minimal IM module pages**

Each module should render credible operator surfaces, not placeholder headings only.

- [ ] **Step 4: Re-run the product-surface tests**

Run: `node --test apps/craw-chat-admin/tests/admin-product-experience.test.mjs`

Expected: PASS.

- [ ] **Step 5: Commit the IM module surface pass**

```bash
git add apps/craw-chat-admin
git commit -m "feat: add first-pass im admin product modules"
```

### Task 5: Enforce The Admin SDK Boundary And Verification Flow

**Files:**
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-admin-api/src/index.ts`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-admin-api/src/transport.ts`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-core/src/workbench.tsx`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-core/src/workbenchActions.ts`
- Modify: `apps/craw-chat-admin/packages/sdkwork-craw-chat-admin-core/src/workbenchSnapshot.ts`
- Modify: `apps/craw-chat-admin/tests/admin-sdk-boundary.test.mjs`

- [ ] **Step 1: Write failing SDK boundary assertions**

Assert that:

- business packages do not call raw admin URLs
- `admin-api` is the single control-plane access package inside the workspace
- package source references the IM admin SDK boundary

- [ ] **Step 2: Run the SDK boundary tests to verify red**

Run: `node --test apps/craw-chat-admin/tests/admin-sdk-boundary.test.mjs`

Expected: FAIL until `admin-api` and workbench files stop carrying router-admin references and raw transport assumptions.

- [ ] **Step 3: Implement the minimal admin API boundary**

Keep transport and service seams clean enough that future SDK wiring can slot in without product-package rewrites.

- [ ] **Step 4: Run the complete targeted verification set**

Run: `node --test apps/craw-chat-admin/tests/*.test.mjs`

Expected: PASS.

- [ ] **Step 5: Run typecheck and build**

Run: `pnpm --dir apps/craw-chat-admin typecheck`

Expected: PASS.

Run: `pnpm --dir apps/craw-chat-admin build`

Expected: PASS.

- [ ] **Step 6: Commit the admin API boundary and final verification pass**

```bash
git add apps/craw-chat-admin
git commit -m "feat: finalize craw chat admin workspace"
```
