# Continuous Optimization: User-Module External Missing-Catalog Unavailable Contract

## Current Step / Wave

- Step: `12`
- Mode: continuous optimization after Step 12 closure

## Why This Round

- The previous loop made `user-module-external` selectable at default runtime bootstrap.
- A remaining real gap stayed behind that success path:
  - missing external catalog path still crashed app assembly
  - unreadable or invalid catalogs already used unavailable semantics
- This left provider config errors inconsistent inside the same provider family.

## Closure Target

1. Add a regression for `CRAW_CHAT_USER_MODULE_PROVIDER=external` without catalog path.
2. Reproduce the startup panic first.
3. Route the missing-path branch into the existing unavailable provider error model.
4. Backwrite review, step, and architecture docs for this loop.

## Actual Delivery

- Added `test_default_app_boots_with_external_user_module_provider_missing_catalog_path_and_returns_provider_unavailable`
- Reproduced the startup panic before patching
- Added `UnavailableUserModuleProvider` for the missing catalog-path branch
- Preserved invalid provider mode fast rejection
- Kept successful `local` and `external with catalog` runtime paths unchanged

## Verification

```powershell
cargo test -p local-minimal-node --offline missing_catalog_path_and_returns_provider_unavailable -- --nocapture
cargo test -p local-minimal-node --offline --test user_module_provider_runtime_selection_test -- --nocapture
cargo test -p local-minimal-node --offline -- --nocapture
cargo fmt --all --check
```

## Next Round

- Prefer one of these two:
  - expose user-module provider health via an operator-facing route or status surface
  - move to the next deeper provider/runtime gap such as object-storage or RTC adapter maturity
