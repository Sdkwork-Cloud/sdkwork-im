# 09BJ User-Module External Missing-Catalog Unavailable Contract Implementation Plan

## Goal

Make `CRAW_CHAT_USER_MODULE_PROVIDER=external` without `CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH` boot with a structured unavailable provider instead of panicking during app assembly.

## Implementation

1. Freeze the missing-catalog branch in `user_module_provider_runtime_selection_test.rs`
2. Keep provider-mode parsing unchanged
3. Convert only the missing catalog-path branch into an unavailable provider wrapper
4. Reuse `ContractError::Unavailable -> 503 provider_unavailable`
5. Backwrite review, step, and architecture indexes

## Non-Goals

- No change to successful external catalog behavior
- No change to invalid provider-mode rejection policy
- No new user-module health HTTP route in this loop
- No broader provider registry redesign
