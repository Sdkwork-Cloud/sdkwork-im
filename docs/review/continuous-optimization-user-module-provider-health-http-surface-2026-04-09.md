# Continuous Optimization: User-Module Provider Health HTTP Surface

## Context

- `user-module` already supported runtime provider selection and unavailable semantics.
- `local-minimal-node` already exposed `provider-health` routes for RTC, media, IoT access, and IoT protocol.
- `user-module` still had no operator-facing health route.

## Confirmed Bug

- `GET /api/v1/user-module/provider-health` returned `404`.
- That hid `user-module-external` unavailable state behind business-route failures instead of giving operators a direct read path.

## Root Cause

- `build.rs` never registered a user-module health route.
- `AppState` had no user-module health accessor.
- `user_module.rs` implemented provider health snapshots but had no HTTP handler.
- Env-driven user-module tests also lacked a serialization guard, so parallel execution could leak env state between tests.

## Decision

- Add `/api/v1/user-module/provider-health`.
- Reuse `ProviderHealthSnapshot` and the same auth model as other provider-health routes.
- Keep HTTP `200`; expose availability through `status=healthy|unavailable`.
- Add minimal `providerKind` details and preserve external error/config details.
- Serialize env-mutating user-module tests with a per-binary async mutex.

## Changed Files

- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/src/node/build.rs`
- `services/local-minimal-node/src/node/user_module.rs`
- `services/local-minimal-node/tests/user_module_provider_http_test.rs`
- `services/local-minimal-node/tests/user_module_provider_runtime_selection_test.rs`

## Verification

Red:

```powershell
cargo test -p local-minimal-node --offline --test user_module_provider_http_test -- --nocapture
```

- Failed before the patch with `404 != 200`.

Green:

```powershell
cargo test -p local-minimal-node --offline --test user_module_provider_http_test -- --nocapture
cargo test -p local-minimal-node --offline --test user_module_provider_runtime_selection_test -- --nocapture
cargo fmt --all --check
cargo test -p local-minimal-node --offline -- --nocapture
```

## Remaining Gap

- Invalid `CRAW_CHAT_USER_MODULE_PROVIDER` still panics fast.
- Unified provider registry / selected-plugin visibility is still not surfaced through a single ops or control-plane view.
