# 2026-04-06 Local-Minimal Runtime Checkpoint Persistence Review Cycle

## 1. Findings

### 1.1 High: the restart persistence regression mixed checkpoint durability with conversation-domain recovery

- The new `local-minimal-node` runtime-dir path correctly persisted realtime checkpoint truth to disk, but the regression test still assumed that a process rebuild would also preserve:
  - conversation aggregates
  - membership state
  - live realtime subscriptions
- After rebuild, `POST /api/v1/conversations/{conversation_id}/messages` returned `404`, not because checkpoint recovery failed, but because the new app instance rebuilt `ConversationRuntime` and subscription memory from scratch.
- This made the test fail for the wrong reason and obscured the actual contract boundary of the runtime-dir checkpoint wave.

### 1.2 Medium: a session-gateway unit test still ignored a fallible checkpoint persistence result

- `RealtimeDeliveryRuntime::persist_checkpoint(...)` now returns `Result<_, RealtimeRuntimeError>`.
- One unit test in `services/session-gateway/src/realtime.rs` still called it without consuming the result.
- This did not break behavior, but it kept `cargo test -p session-gateway --offline --no-run` emitting an `unused Result` warning, which is not acceptable for a hardened review cycle.

## 2. Root Cause

The abstraction and persistence implementation were correct, but the regression boundary was not frozen precisely enough:

1. runtime-dir persistence currently covers realtime checkpoint truth only
2. conversation-domain state in `local-minimal-node` is still in-memory unless a later wave introduces its own durable store
3. live subscription state is still rebuilt explicitly by the client or by a future durable recovery wave

In short, the code had the right storage boundary, while the test encoded a larger platform promise than the system currently makes.

## 3. Implementation

This review cycle completed the following:

- corrected `services/local-minimal-node/tests/realtime_checkpoint_persistence_test.rs`
  - rebuild with the same runtime dir
  - verify the restored checkpoint window before any new publish
  - explicitly recreate conversation context after rebuild
  - explicitly resync realtime subscriptions after rebuild
  - then verify the next delivered event continues at `realtimeSeq = 2`
- hardened `services/session-gateway/src/realtime.rs`
  - the checkpoint normalization unit test now consumes the `persist_checkpoint(...)` result explicitly
- normalized workspace formatting with `cargo fmt --all`

## 4. Regression Coverage

- `services/local-minimal-node/tests/realtime_checkpoint_persistence_test.rs`
  - `test_default_local_minimal_profile_persists_realtime_checkpoint_across_rebuild_via_runtime_dir`
- `services/session-gateway/tests/checkpoint_store_error_test.rs`
  - `test_realtime_events_returns_503_when_checkpoint_store_load_fails`
  - `test_realtime_ack_returns_503_when_checkpoint_store_save_fails`
- `services/session-gateway/tests/cluster_routing_test.rs`
  - `test_cluster_bridge_rebind_surfaces_checkpoint_store_failures_as_controlled_errors`
- `services/session-gateway/src/realtime.rs`
  - `test_persist_checkpoint_normalizes_transient_inconsistent_sequence_state`
- `adapters/local-disk/src/lib.rs`
  - `test_file_checkpoint_store_persists_across_reopen`

## 5. Verification

Verified in this cycle with fresh command output:

- `cargo test -p session-gateway --offline --no-run`
- `cargo test -p session-gateway --offline --test checkpoint_store_error_test -- --nocapture`
- `cargo test -p session-gateway --offline --test cluster_routing_test test_cluster_bridge_rebind_surfaces_checkpoint_store_failures_as_controlled_errors -- --nocapture`
- `cargo test -p session-gateway --offline`
- `cargo test -p local-minimal-node --offline --test realtime_checkpoint_persistence_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo fmt --all --check`

## 6. Standardized Outcome

The managed `local-minimal` deployment path now has a verified, restart-safe realtime checkpoint baseline under the runtime dir.

The review also freezes an important recovery boundary:

- checkpoint truth is persisted and restored automatically
- conversation aggregates are not yet restored automatically by this wave
- realtime subscriptions are not yet restored automatically by this wave

That boundary is now explicit instead of being left implicit inside a failing regression.

## 7. Residual Risk

- A cold rebuild of `local-minimal-node` still requires domain context to be re-established before new publishes can flow through the same conversation scope.
- In the current local/private profile, that means one of:
  - recreating or reloading the conversation aggregate from a durable domain store
  - resyncing subscriptions from the client/device control plane
- This is acceptable for the current checkpoint wave, but it is not yet the full commercial cold-restart target for a standalone private deployment profile.

## 8. Next Wave

The next durability wave should make the private deployment profile more self-contained by standardizing one of these two paths:

1. durable conversation/member recovery for `local-minimal-node`
2. durable subscription recovery or explicit resumable subscription bootstrap

Only after that wave can the platform claim broader cold-restart recovery beyond checkpoint truth.

## 9. Status Update

The follow-up domain recovery wave has now been completed in:

- `docs/review/2026-04-06-local-minimal-domain-recovery-review-cycle.md`
- `docs/架构/95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md`

This review document remains the historical record for the checkpoint-only wave. Current managed runtime-dir private deployment composes both standards together.
