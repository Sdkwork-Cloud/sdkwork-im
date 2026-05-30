# Continuous Optimization: Local-Minimal Ops Provider Bindings Runtime Visibility

## Context

- Architecture and prior ops docs already froze `providerBindings` and `providerBindingDrift` in the ops diagnostic contract.
- `local-minimal-node` exposed `GET /backend/v3/api/ops/diagnostics`, but runtime provider selection was still invisible there.

## Confirmed Bug

- `GET /backend/v3/api/ops/diagnostics` returned `providerBindings=[]` in the default local-minimal profile.
- Operators could inspect per-domain provider-health routes, but could not see one unified selected-provider snapshot.

## Root Cause

- `OpsRuntime` supports provider binding snapshots, but `local-minimal-node` never populated them.
- `refresh_node_operational_view(...)` refreshed lifecycle, lag, runtime-dir, and projection data only.
- RTC and media runtimes had internal provider registries, but no read accessor for binding visibility.

## Decision

- Keep the existing ops contract.
- Mirror one global provider binding snapshot into `OpsRuntime` during refresh.
- Reuse real RTC and media runtime provider bindings.
- Derive principal-profile, IoT access, and IoT protocol bindings from current selected descriptors over the frozen platform registry baseline.

## Changed Files

- `services/media-service/src/lib.rs`
- `services/rtc-signaling-service/src/lib.rs`
- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/src/node/platform.rs`
- `services/local-minimal-node/src/node/device_registration.rs`
- `services/local-minimal-node/tests/ops_provider_bindings_runtime_test.rs`

## Verification

Red:

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test -- --nocapture
```

- Failed before the patch because `providerBindings.len()` was `0`, expected `1`.

Green:

```powershell
cargo test -p local-minimal-node --offline --test ops_provider_bindings_runtime_test -- --nocapture
cargo fmt --all --check
cargo test -p local-minimal-node --offline -- --nocapture
```

## Remaining Gap

- `local-minimal-node` now exposes a global runtime provider snapshot, but it still does not expose dedicated `/backend/v3/api/ops/provider_bindings` and `/backend/v3/api/ops/provider_bindings/drift` routes like standalone `ops-service`.
- Cross-subsystem binding divergence detection is still implicit; RTC recording storage and media storage are assumed aligned by profile.
