# Continuous Optimization: User-Module External Missing-Catalog Unavailable Contract

## Context

- Current loop: continue provider/runtime alignment after the `user-module-external` default bootstrap fix.
- Surface: default runtime bootstrap for `CRAW_CHAT_USER_MODULE_PROVIDER=external`
- Contract: external provider config errors must be explicitly exposed instead of crashing the whole app before business and ops surfaces can react.

## Confirmed Bug

- `CRAW_CHAT_USER_MODULE_PROVIDER=external` without `CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH` panicked during app assembly.
- The same provider already modeled unreadable or invalid catalogs as `ContractError::Unavailable` and `ProviderHealthSnapshot.status=unavailable`.
- That created a split-brain failure model:
  - missing catalog path -> startup panic
  - unreadable/invalid catalog file -> structured unavailable provider

## Why It Matters

- A leading operator surface should preserve boot and expose explicit unavailability, not turn a single external-directory config miss into a process crash.
- The repo already maps `ContractError::Unavailable -> 503 provider_unavailable`.
- The missing-path branch bypassed that standard and made the runtime less observable than the rest of the provider system.

## Decision

- Keep `CRAW_CHAT_USER_MODULE_PROVIDER=local|external` unchanged.
- Keep invalid provider mode values as fast rejection.
- Only convert the missing external catalog-path branch into an `UnavailableUserModuleProvider`.
- Reuse the existing `provider_unavailable` API path instead of inventing a new error family.

## Changed Files

- `services/local-minimal-node/src/node/user_module.rs`
- `services/local-minimal-node/tests/user_module_provider_runtime_selection_test.rs`
- `docs/review/continuous-optimization-user-module-external-missing-catalog-unavailable-contract-2026-04-09.md`
- `docs/step/continuous-optimization-user-module-external-missing-catalog-unavailable-contract-2026-04-09.md`
- `docs/架构/09BJ-user-module-external-missing-catalog-unavailable-contract-implementation-plan-2026-04-09.md`
- `docs/架构/150BJ-user-module-external-missing-catalog-unavailable-contract-design-2026-04-09.md`

## Verification

Red:

```powershell
cargo test -p local-minimal-node --offline missing_catalog_path_and_returns_provider_unavailable -- --nocapture
```

- Failed before the patch because app assembly panicked at:
  - `CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH is required when CRAW_CHAT_USER_MODULE_PROVIDER=external`

Green:

```powershell
cargo test -p local-minimal-node --offline missing_catalog_path_and_returns_provider_unavailable -- --nocapture
cargo test -p local-minimal-node --offline --test user_module_provider_runtime_selection_test -- --nocapture
cargo test -p local-minimal-node --offline -- --nocapture
cargo fmt --all --check
```

## Remaining Gap

- Missing catalog path now respects unavailable semantics.
- Remaining follow-up items are:
  - decide whether invalid provider mode should stay panic-fast or also become structured bootstrap rejection
  - expose user-module provider health through an operator-facing surface
  - continue deeper provider/runtime work beyond user-module
