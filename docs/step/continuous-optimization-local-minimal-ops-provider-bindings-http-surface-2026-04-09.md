# Step: Local-Minimal Ops Provider Bindings HTTP Surface

## Goal

Make `local-minimal-node` expose the same standalone provider binding routes as `ops-service`.

## Red

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test test_local_minimal_exposes_standalone_ops_provider_binding_routes -- --nocapture
```

- `/backend/v3/api/ops/provider_bindings` returned `404`.

## Green

- Added `GET /backend/v3/api/ops/provider_bindings`.
- Added `GET /backend/v3/api/ops/provider_bindings/drift`.
- Reused `OpsRuntime.provider_bindings_view()` and `OpsRuntime.provider_binding_drift_view()`.
- Reused the existing `ops.read` auth gate and `refresh_node_operational_view(...)`.

## Verify

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test -- --nocapture
cargo fmt --all --check
cargo test -p local-minimal-node --offline -- --nocapture
```

## Next

- Emit non-empty `providerBindingDrift.items` when real provider divergence is detected.
- Keep standalone ops routes and `diagnostics` bundle on the same snapshot source.
