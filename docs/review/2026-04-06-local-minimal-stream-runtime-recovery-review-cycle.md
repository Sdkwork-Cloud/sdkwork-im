# 2026-04-06 Local-Minimal Stream Runtime Recovery Review Cycle

## 1. Findings

### 1.1 High: managed `local-minimal` rebuilds lost stream runtime state even when the same runtime dir was reused

- `ConversationRuntime`, realtime checkpoint truth, disconnect fences, and live subscriptions already had explicit restart recovery boundaries.
- `StreamingRuntime` still rebuilt from empty memory after process restart.
- The operational effect was direct:
  - `GET /im/v3/api/streams/{stream_id}/frames` returned `404`
  - `POST /im/v3/api/streams/{stream_id}/checkpoint` returned `404`
  - `POST /im/v3/api/streams/{stream_id}/complete` returned `404`
  - clients had to reopen the same stream after restart, which breaks continuity for long-lived stream semantics

### 1.2 High: `local-minimal-node` composition had no durable stream runtime seam

- Managed runtime-dir builders already bound durable implementations for:
  - conversation-domain replay
  - realtime checkpoint truth
  - realtime subscription intent
  - disconnect fences
- `streaming_runtime` was still created with `StreamingRuntime::default()`, which remained memory-only.

### 1.3 Medium: stream persistence needed a distinct recovery standard, not an implicit side effect of domain replay

- Stream state is not the same boundary as conversation messages or realtime windows.
- Stream sessions and frames must be recoverable independently so generic data-stream use cases remain vendor-neutral and modular.

## 2. Root Cause

The root cause was the same architectural pattern seen in earlier recovery waves:

1. the platform had no pluggable persistence contract for stream runtime state
2. runtime state lived only in `StreamingRuntime.sessions` and `StreamingRuntime.frames`
3. managed runtime-dir composition never replaced the memory-only implementation
4. rebuild paths therefore lost all stream state even when other runtime boundaries were already durable

So the platform preserved conversation truth around a stream, but not the stream session and frame state itself.

## 3. Implementation

This review cycle completed the missing recovery path:

- added `StreamStateRecord` and `StreamStateStore`
- added adapters:
  - `MemoryStreamStateStore`
  - `FileStreamStateStore`
- extended `StreamingRuntime`
  - added `with_store(...)`
  - restored persisted stream state lazily on access
  - persisted stream state after:
    - `open_stream(...)`
    - `append_frame(...)`
    - `checkpoint_stream(...)`
    - `complete_stream(...)`
    - `abort_stream(...)`
- bound managed `local-minimal` runtime-dir builders to:
  - `<runtime-dir>/state/stream-state.json`
- kept unmanaged/default builders memory-backed

## 4. Regression Coverage

- `services/streaming-service/tests/stream_lifecycle_test.rs`
  - `test_runtime_restores_stream_state_on_rebuild_with_shared_store`
- `adapters/local-disk/src/lib.rs`
  - `test_file_stream_state_store_persists_across_reopen`
- `services/local-minimal-node/tests/stream_runtime_persistence_test.rs`
  - `test_default_local_minimal_profile_restores_stream_runtime_state_after_rebuild`

## 5. Verification

Verified in this cycle with fresh command output:

- `cargo test -p im-adapters-local-disk --offline test_file_stream_state_store_persists_across_reopen -- --nocapture`
- `cargo test -p streaming-service --offline test_runtime_restores_stream_state_on_rebuild_with_shared_store -- --nocapture`
- `cargo test -p local-minimal-node --offline --test stream_runtime_persistence_test -- --nocapture`
- `cargo fmt --all --check`
- `cargo test -p streaming-service --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo test -p local-minimal-node --offline`

## 6. Standardized Outcome

Managed `local-minimal` private deployment now restores stream runtime state across rebuild when the same runtime dir is reused.

The recovered surface covers:

- stream session metadata
- ordered stream frames
- last frame sequence
- checkpoint progression
- post-restart completion / abort continuation

Clients no longer need to reopen an existing stream after restart just to keep operating on it.

## 7. Residual Risk

This wave still leaves several restart boundaries outside the durable profile:

- RTC runtime state
- notification runtime projections
- automation runtime projections
- presence heartbeat truth as a separate non-durable cache/query concern

## 8. Next Wave

The next durability review wave should target:

1. RTC runtime state recovery
2. notification / automation projection durability boundaries
3. runtime-dir inspection and repair tooling for private deployment operations
