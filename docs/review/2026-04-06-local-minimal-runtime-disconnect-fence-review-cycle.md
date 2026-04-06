# 2026-04-06 Local-Minimal Runtime Disconnect Fence Review Cycle

## 1. Findings

### 1.1 High: the installable local-minimal profile still lost reconnect-required fences after process rebuild

- The previous wave introduced a pluggable `RealtimeDisconnectFenceStore`, but the default `local-minimal-node` deployment path still built its access-plane bridge with in-memory fence state only.
- That meant the documented restart-safe behavior only held in tests that manually reused a shared memory store.
- The actual managed private-deployment path had no durable fence file under `.runtime/local-minimal`, so a restart could silently drop the explicit reconnect requirement created by `session.disconnect`.

### 1.2 High: the bridge would panic on real store failures

- `RealtimeClusterBridge` exposed a `Result`-based store contract, but fence load/save/clear paths still used `expect(...)`.
- As soon as a real file-backed store was introduced, any I/O or parse failure would crash the process instead of surfacing a controlled service error.
- This was acceptable while only in-memory adapters existed, but it is not acceptable for commercial private deployment.

## 2. Root Cause

The platform had already frozen the abstraction seam, but not the deployable default:

1. the runtime profile did not bind a durable local store
2. the lifecycle scripts did not export a stable runtime-dir contract for state files
3. the bridge still assumed store operations were infallible

## 3. Implementation

This review cycle implemented the following:

- Added `im-adapters-local-disk` with `FileRealtimeDisconnectFenceStore`
  - JSON persistence
  - runtime-dir-backed file path
  - save/load/clear support
- Added explicit runtime-dir builders in `local-minimal-node`
  - `build_default_app_with_runtime_dir(...)`
  - `build_public_app_with_runtime_dir(...)`
- Bound managed local-minimal runtime persistence through:
  - `CRAW_CHAT_RUNTIME_DIR`
  - `.runtime/local-minimal/state/realtime-disconnect-fences.json`
- Hardened `RealtimeClusterBridge`
  - fence save/load/clear now return controlled `RealtimeClusterError`
  - store failures surface as `disconnect_fence_store_unavailable`
  - HTTP/API adapters map that condition to `503 Service Unavailable`
- Updated lifecycle scripts
  - `init-config-local.*` now writes `CRAW_CHAT_RUNTIME_DIR`
  - `start-local.*` now exports `CRAW_CHAT_RUNTIME_DIR`
  - runtime `state/` directory is part of the managed runtime layout

## 4. Regression Coverage

- `adapters/local-disk/src/lib.rs`
  - `test_file_disconnect_fence_store_persists_and_clears_across_reopen`
- `services/session-gateway/src/cluster.rs`
  - `test_disconnect_fence_store_failures_surface_as_controlled_cluster_errors`
- `services/local-minimal-node/tests/disconnect_fence_persistence_test.rs`
  - `test_default_local_minimal_profile_persists_disconnect_fence_across_rebuild_via_runtime_dir`
- `services/local-minimal-node/tests/runtime_config_test.rs`
  - runtime-dir resolution coverage
- `services/local-minimal-node/tests/deployment_profile_test.rs`
  - lifecycle script/runtime-dir contract coverage

## 5. Verification

Verified in this cycle with:

- `cargo test -p im-adapters-local-disk --offline`
- `cargo test -p session-gateway --offline`
- `cargo test -p local-minimal-node --offline`

## 6. Standardized Outcome

The managed local-minimal deployment path is now restart-safe for disconnect fences when launched through the runtime-dir-aware builders or lifecycle scripts.

The bridge no longer treats durable store failures as panics.

## 7. Residual Risk

- `build_default_app()` and `build_public_app()` still fall back to in-memory fence state when `CRAW_CHAT_RUNTIME_DIR` is not configured.
- This is intentional to preserve isolated test semantics and non-managed in-process usage.
- Commercial/private deployment must use the managed scripts or explicitly configure `CRAW_CHAT_RUNTIME_DIR`.

## 8. Next Wave

The next high-value durability wave is to extend the same runtime-dir-backed persistence strategy to realtime checkpoint truth so local/private deployment can preserve more delivery state across process restart, not only reconnect-required fences.
