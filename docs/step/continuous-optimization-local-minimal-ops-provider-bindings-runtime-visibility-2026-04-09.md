# Step: Local-Minimal Ops Provider Bindings Runtime Visibility

## Goal

Make `local-minimal-node` expose one unified runtime provider binding snapshot through `GET /backend/v3/api/ops/diagnostics`.

## Why Now

- The app already had RTC, media, principal-profile, IoT access, and IoT protocol provider selection.
- Operators still had to inspect multiple routes to infer the selected stack.
- This violated the existing ops contract that already reserved `providerBindings`.

## Red

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test -- --nocapture
```

- `providerBindings` was empty.

## Green

- Added RTC and media runtime binding accessors.
- Added local-minimal runtime snapshot assembly over the frozen provider-registry contract.
- Refreshed `OpsRuntime` with that snapshot during `refresh_node_operational_view(...)`.
- Added HTTP regression coverage for:
  - `provider-registry/v1`
  - global snapshot presence
  - selected plugins:
    - `rtc-volcengine`
    - `object-storage-volcengine`
    - `principal-profile-upstream-context`
    - `iot-access-local`
    - `iot-mqtt`

## Verify

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test -- --nocapture
cargo fmt --all --check
cargo test -p local-minimal-node --offline -- --nocapture
```

## Next

- Decide whether local-minimal should also expose standalone `/backend/v3/api/ops/provider_bindings` and `/backend/v3/api/ops/provider_bindings/drift`.
- Decide whether object-storage selection must be unified from one shared runtime registry instead of inferred from per-service registries.
