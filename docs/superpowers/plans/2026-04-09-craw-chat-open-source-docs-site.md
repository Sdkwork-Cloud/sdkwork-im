# Craw Chat Open Source Docs Site Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a production-quality VitePress documentation site in `docs/sites` that documents the current `craw-chat` application, all public API surfaces, SDK boundaries, and installation and deployment workflows.

**Architecture:** The site will be a standalone VitePress project with a custom theme, a curated navigation model, and Markdown pages sourced from current Rust routers, runtime scripts, environment templates, and SDK metadata. The content will explicitly separate implemented behavior from placeholder or pending-generation surfaces.

**Tech Stack:** VitePress, Markdown, TypeScript config, custom CSS, npm

---

### Task 1: Scaffold the standalone VitePress site

**Files:**
- Create: `docs/sites/package.json`
- Create: `docs/sites/.vitepress/config.ts`
- Create: `docs/sites/.vitepress/theme/index.ts`
- Create: `docs/sites/.vitepress/theme/custom.css`
- Create: `docs/sites/index.md`

- [ ] **Step 1: Define the site scripts and VitePress dependency**
- [ ] **Step 2: Configure nav, sidebar, search, and edit-link behavior**
- [ ] **Step 3: Add a custom theme entry and site-level visual language**
- [ ] **Step 4: Create a landing page with product positioning and entry links**
- [ ] **Step 5: Verify the project shape is sufficient for `vitepress build`**

### Task 2: Write the core product and architecture docs

**Files:**
- Create: `docs/sites/getting-started/index.md`
- Create: `docs/sites/getting-started/quick-start.md`
- Create: `docs/sites/architecture/overview.md`
- Create: `docs/sites/architecture/runtime-topology.md`
- Create: `docs/sites/architecture/module-map.md`
- Create: `docs/sites/features/index.md`
- Create: `docs/sites/features/capabilities.md`

- [ ] **Step 1: Document prerequisites and first-run path from `bin/*.ps1` and templates**
- [ ] **Step 2: Document the workspace and service topology from `Cargo.toml` and router assembly**
- [ ] **Step 3: Document feature groupings from the route surface and E2E tests**
- [ ] **Step 4: Ensure every page distinguishes current implementation from future scope**

### Task 3: Write the full API and SDK references

**Files:**
- Create: `docs/sites/api-reference/index.md`
- Create: `docs/sites/api-reference/auth-and-errors.md`
- Create: `docs/sites/api-reference/app-api.md`
- Create: `docs/sites/api-reference/platform-api.md`
- Create: `docs/sites/api-reference/iot-api.md`
- Create: `docs/sites/api-reference/control-plane-api.md`
- Create: `docs/sites/sdk/index.md`
- Create: `docs/sites/sdk/app-sdk.md`
- Create: `docs/sites/sdk/admin-sdk.md`
- Create: `docs/sites/sdk/language-support.md`

- [ ] **Step 1: Group endpoints from `local-minimal-node` into coherent reference pages**
- [ ] **Step 2: Document control-plane routes, permissions, and policy behavior**
- [ ] **Step 3: Capture current SDK family boundaries and release-catalog status**
- [ ] **Step 4: Cross-check auth wording against public auth tests**

### Task 4: Write deployment and operations reference and verify the site

**Files:**
- Create: `docs/sites/deployment/index.md`
- Create: `docs/sites/deployment/local-binary.md`
- Create: `docs/sites/deployment/docker.md`
- Create: `docs/sites/deployment/profiles-and-env.md`
- Create: `docs/sites/deployment/runtime-operations.md`
- Create: `docs/sites/reference/cli-and-scripts.md`
- Create: `docs/sites/reference/runtime-directory.md`

- [ ] **Step 1: Document install, init, start, status, stop, restart, and deploy command surfaces**
- [ ] **Step 2: Document profile differences for `local-minimal` and `local-default`**
- [ ] **Step 3: Document runtime-dir inspection, repair, backup, preview, restore, and prune flows**
- [ ] **Step 4: Run `npm install` and `npm run docs:build` inside `docs/sites`**
- [ ] **Step 5: Reconcile any build or content issues before completion**
