# Continuous Optimization: Local-Minimal Ops Provider Bindings HTTP Surface

## Context

- `ops-service` already exposed:
  - `GET /api/v1/ops/provider-bindings`
  - `GET /api/v1/ops/provider-bindings/drift`
- `local-minimal-node` had already mirrored provider binding snapshots into `OpsRuntime`.
- The same node still returned `404` for the standalone routes, so its operator surface lagged behind the frozen ops contract.

## Confirmed Bug

- `GET /api/v1/ops/provider-bindings` returned `404` in `local-minimal-node`.
- `GET /api/v1/ops/provider-bindings/drift` was also missing.
- Operators could read the data only through `GET /api/v1/ops/diagnostics`, which broke route-level parity with standalone ops deployment.

## Root Cause

- The runtime state existed.
- `build.rs` never registered the two routes.
- `platform.rs` never exposed thin handlers for `provider_bindings_view()` and `provider_binding_drift_view()`.

## Decision

- Do not invent new storage or reassemble snapshots again.
- Reuse `OpsRuntime` as the single runtime source.
- Keep the same `ops.read` gate and refresh flow as the other ops endpoints.

## Changed Files

- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/src/node/build.rs`
- `services/local-minimal-node/src/node/platform.rs`
- `services/local-minimal-node/tests/ops_provider_bindings_runtime_test.rs`

## Verification

Red:

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test test_local_minimal_exposes_standalone_ops_provider_binding_routes -- --nocapture
```

- Failed before the patch with `404 != 200`.

Green:

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test -- --nocapture
cargo fmt --all --check
cargo test -p local-minimal-node --offline -- --nocapture
```

## Next Gap

- `providerBindingDrift` is still empty in the default local-minimal profile because no cross-subsystem divergence rule is emitted yet.
- A later loop should add explicit drift evidence when media storage, RTC recording storage, or deployment-profile overrides disagree.
