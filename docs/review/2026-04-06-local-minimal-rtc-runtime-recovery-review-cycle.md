# 2026-04-06 Local-Minimal RTC Runtime Recovery Review Cycle

## 1. Findings

### 1.1 High: managed `local-minimal` rebuilds lost RTC session continuity even when the same runtime dir was reused

- `local-minimal-node` already had durable boundaries for:
  - conversation-domain replay
  - realtime checkpoint truth
  - disconnect fences
  - live subscription intent
  - stream runtime state
- RTC state still rebuilt from empty memory after restart.
- The operational effect was direct:
  - `POST /im/v3/api/rtc/sessions/{id}/accept` returned `404 rtc_session_not_found`
  - `POST /im/v3/api/rtc/sessions/{id}/signals` returned `404 rtc_session_not_found`
  - `POST /im/v3/api/rtc/sessions/{id}/end` returned `404 rtc_session_not_found`
  - conversation-bound RTC flows could not continue across restart unless clients recreated the same `rtcSessionId`

### 1.2 High: `RtcRuntime` had no durable seam and `local-minimal-node` always composed it as memory-only

- `RtcRuntime` stored both session truth and signal history only in in-memory `HashMap`s.
- Managed runtime-dir builders never replaced the default runtime with a file-backed implementation.
- The result was a hard restart boundary in the middle of RTC signaling semantics.

### 1.3 Medium: RTC recovery needed its own standard instead of being implicitly coupled to domain replay or stream durability

- RTC session state is not equivalent to:
  - conversation-domain message history
  - realtime subscriptions
  - generic stream session/frame state
- Commercial private deployment needs RTC durability to stay modular and replaceable behind its own contract.

## 2. Root Cause

The root cause matched the pattern seen in earlier recovery waves:

1. the platform had no pluggable persistence contract for RTC runtime state
2. `RtcRuntime.sessions` and `RtcRuntime.signals` were memory-only
3. managed runtime-dir builders still instantiated `RtcRuntime::default()`
4. rebuild paths therefore lost RTC session continuity even when other runtime families already had durable seams

So the platform could preserve conversation truth around an RTC session, but not the RTC runtime state itself.

## 3. Implementation

This review cycle completed the missing RTC recovery path:

- added `RtcStateRecord` and `RtcStateStore`
- added adapters:
  - `MemoryRtcStateStore`
  - `FileRtcStateStore`
- extended `RtcRuntime`
  - added `with_store(...)`
  - restored persisted RTC state lazily on access
  - persisted RTC state after:
    - `create_session(...)`
    - `invite_session(...)`
    - `accept_session(...)`
    - `reject_session(...)`
    - `end_session(...)`
    - `post_signal(...)`
- bound managed `local-minimal` runtime-dir builders to:
  - `<runtime-dir>/state/rtc-state.json`
- kept unmanaged/default builders memory-backed

## 4. Regression Coverage

- `services/rtc-signaling-service/tests/rtc_runtime_persistence_test.rs`
  - `test_runtime_restores_rtc_state_on_rebuild_with_shared_store`
- `adapters/local-disk/src/lib.rs`
  - `test_file_rtc_state_store_persists_across_reopen`
- `services/local-minimal-node/tests/rtc_runtime_persistence_test.rs`
  - `test_default_local_minimal_profile_restores_rtc_runtime_state_after_rebuild`

## 5. Verification

Verified in this cycle with fresh command output:

- `cargo test -p rtc-signaling-service --offline --test rtc_runtime_persistence_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test rtc_runtime_persistence_test -- --nocapture`
- `cargo test -p im-adapters-local-disk --offline test_file_rtc_state_store_persists_across_reopen -- --nocapture`

Additional broad verification ran after the implementation stabilized:

- `cargo fmt --all --check`
- `cargo test -p rtc-signaling-service --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo test -p local-minimal-node --offline`

## 6. Standardized Outcome

Managed `local-minimal` private deployment now restores RTC runtime state across rebuild when the same runtime dir is reused.

The recovered surface covers:

- RTC session metadata
- signaling stream binding
- accept / reject / end state continuity
- ordered in-session signal history
- post-restart custom signal continuation
- conversation-bound RTC side effects after rebuild

Clients no longer need to recreate an existing `rtcSessionId` just to continue a legitimate RTC flow after restart.

## 7. Residual Risk

This wave still leaves several runtime families outside the durable private-deployment baseline:

- notification runtime projections
- automation runtime projections
- presence heartbeat truth as a separate runtime concern
- runtime-dir inspection / repair tooling for operator workflows

## 8. Next Wave

The next durability review wave should target:

1. notification runtime projection durability
2. automation runtime projection durability
3. runtime-dir inspection, repair, and health tooling for private deployment operations
