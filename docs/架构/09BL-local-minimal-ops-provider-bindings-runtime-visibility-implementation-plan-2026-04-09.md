# Local-Minimal Ops Provider Bindings Runtime Visibility Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expose real runtime-selected provider bindings in `local-minimal-node` ops diagnostics.

**Architecture:** Keep the existing ops contract and inject one global provider binding snapshot into `OpsRuntime` during refresh. Reuse real RTC/media runtime bindings and derive the remaining domains from active provider descriptors over the frozen platform registry baseline.

**Tech Stack:** Rust, Axum, `ops-service`, `im-platform-contracts`, `local-minimal-node`

---

### Task 1: Reproduce the Missing Snapshot

**Files:**
- Create: `services/local-minimal-node/tests/ops_provider_bindings_runtime_test.rs`

- [ ] **Step 1: Write the failing test**
- [ ] **Step 2: Run `cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test -- --nocapture`**
- [ ] **Step 3: Confirm failure because `providerBindings` is empty**

### Task 2: Expose Runtime Binding Accessors

**Files:**
- Modify: `services/media-service/src/lib.rs`
- Modify: `services/rtc-signaling-service/src/lib.rs`

- [ ] **Step 1: Add a read-only media binding accessor backed by the existing provider registry**
- [ ] **Step 2: Add a read-only RTC binding accessor backed by the existing provider registry**
- [ ] **Step 3: Keep behavior unchanged for existing health and business routes**

### Task 3: Mirror Local Runtime Bindings into Ops

**Files:**
- Modify: `services/local-minimal-node/src/node.rs`
- Modify: `services/local-minimal-node/src/node/platform.rs`
- Modify: `services/local-minimal-node/src/node/device_registration.rs`

- [ ] **Step 1: Build one global `ProviderBindingSnapshotView` from active runtime selections**
- [ ] **Step 2: Reuse the platform registry baseline for defaults and precedence**
- [ ] **Step 3: Refresh `OpsRuntime` snapshots inside `refresh_node_operational_view(...)`**
- [ ] **Step 4: Re-run the failing test and confirm green**

### Task 4: Close the Loop

**Files:**
- Create: `docs/review/continuous-optimization-local-minimal-ops-provider-bindings-runtime-visibility-2026-04-09.md`
- Create: `docs/step/continuous-optimization-local-minimal-ops-provider-bindings-runtime-visibility-2026-04-09.md`
- Create: `docs/架构/150BL-local-minimal-ops-provider-bindings-runtime-visibility-design-2026-04-09.md`
- Modify: `docs/review/README.md`
- Modify: `docs/step/README.md`
- Modify: `docs/架构/README.md`

- [ ] **Step 1: Run `cargo fmt --all --check`**
- [ ] **Step 2: Run `cargo test -p local-minimal-node --offline -- --nocapture`**
- [ ] **Step 3: Backwrite concise review, step, and architecture docs**
- [ ] **Step 4: Update public indexes**
