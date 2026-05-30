# Local-Minimal Ops Provider Bindings HTTP Surface Implementation Plan

**Goal:** Close the local-minimal operator-surface gap by exposing standalone provider binding routes over the already-populated `OpsRuntime` snapshot.

**Architecture:** Keep one runtime source of truth. `local-minimal-node` only adds HTTP handlers and route registration; snapshot assembly remains in the existing refresh path. This avoids state duplication and keeps standalone ops deployment and embedded local-minimal deployment semantically aligned.

**Tech Stack:** Rust, Axum, `ops-service` contracts, local-minimal integration tests

---

### Task 1: Prove the missing HTTP surface

**Files:**
- Modify: `services/local-minimal-node/tests/ops_provider_bindings_runtime_test.rs`

- [ ] Add a regression test for:
  - `GET /backend/v3/api/ops/provider_bindings`
  - `GET /backend/v3/api/ops/provider_bindings/drift`
- [ ] Run:

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test test_local_minimal_exposes_standalone_ops_provider_binding_routes -- --nocapture
```

- [ ] Confirm red because the route returns `404`.

### Task 2: Expose the routes with the existing runtime view

**Files:**
- Modify: `services/local-minimal-node/src/node.rs`
- Modify: `services/local-minimal-node/src/node/platform.rs`
- Modify: `services/local-minimal-node/src/node/build.rs`

- [ ] Import `ProviderBindingsView` and `ProviderBindingDriftView`.
- [ ] Add thin handlers that:
  - resolve auth
  - enforce `ops.read`
  - call `refresh_node_operational_view(...)`
  - return the corresponding `OpsRuntime` view
- [ ] Register both routes in the local-minimal router.

### Task 3: Verify and document

**Files:**
- Create: `docs/review/continuous-optimization-local-minimal-ops-provider-bindings-http-surface-2026-04-09.md`
- Create: `docs/step/continuous-optimization-local-minimal-ops-provider-bindings-http-surface-2026-04-09.md`
- Create: `docs/架构/150BM-local-minimal-ops-provider-bindings-http-surface-design-2026-04-09.md`
- Modify: `docs/review/README.md`
- Modify: `docs/step/README.md`
- Modify: `docs/架构/README.md`

- [ ] Run:

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test -- --nocapture
cargo fmt --all --check
cargo test -p local-minimal-node --offline -- --nocapture
```

- [ ] Record the red/green evidence and the next remaining gap.
