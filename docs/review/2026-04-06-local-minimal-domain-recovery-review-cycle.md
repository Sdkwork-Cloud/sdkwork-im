# 2026-04-06 Local-Minimal Domain Recovery Review Cycle

## 1. Findings

### 1.1 High: managed `local-minimal` rebuilds lost conversation-domain state even when a runtime dir was present

- The previous runtime-dir hardening wave persisted:
  - realtime disconnect fences
  - realtime checkpoint truth
- But `ConversationRuntime` and `TimelineProjectionService` still rebuilt from empty memory on process restart.
- The effect was operationally severe for private deployment:
  - `GET /im/v3/api/chat/conversations/{conversation_id}` returned `404`
  - membership lookups failed
  - posting after restart failed unless the conversation was recreated externally

### 1.2 Medium: the codebase already had the correct abstraction seam, but it was only memory-backed in `local-minimal-node`

- Conversation writes already flowed through a shared `CommitJournal` abstraction.
- The local-minimal composition still bound that seam to `MemoryCommitJournal`, so the domain event stream disappeared on rebuild.
- This was not a missing architecture concept. It was an unfinished durability composition.

## 2. Root Cause

The root cause was a mismatch between the standardized recovery intent and the actual managed builder wiring:

1. durable runtime-dir deployment already existed
2. conversation writes already emitted ordered commit envelopes
3. startup composition never replaced the in-memory commit journal with a durable adapter
4. startup composition never replayed recorded envelopes back into:
   - `ConversationRuntime`
   - `TimelineProjectionService`

So restart recovery stopped at access-plane truth and never rebuilt the conversation domain.

## 3. Implementation

This review cycle completed the missing recovery path:

- added durable local file journal support through `FileCommitJournal`
  - JSON-backed append-only persistence
  - temp-file replace semantics
  - reopen + replay support
- hardened `ConversationRuntime`
  - added `apply_recovered_envelope(...)`
  - replay rebuilds:
    - conversation type
    - members and active principal map
    - read cursors
    - message index and stored messages
    - handoff state
    - owner/member-role lifecycle state
- rewired managed `local-minimal-node` builders
  - runtime-dir builders now bind:
    - `FileRealtimeDisconnectFenceStore`
    - `FileRealtimeCheckpointStore`
    - `FileCommitJournal`
  - startup replays the durable commit journal into:
    - `TimelineProjectionService`
    - `ConversationRuntime`
- kept unmanaged builders memory-backed
  - no forced persistence for isolated tests or embedded in-process usage

## 4. Regression Coverage

- `services/conversation-runtime/tests/conversation_flow_test.rs`
  - `test_runtime_replays_recorded_conversation_events_after_rebuild`
- `adapters/local-disk/src/lib.rs`
  - `test_file_commit_journal_persists_across_reopen`
- `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`
  - `test_default_local_minimal_profile_rebuild_restores_conversation_domain_state`

## 5. Verification

Verified in this cycle with fresh command output:

- `cargo test -p conversation-runtime --offline test_runtime_replays_recorded_conversation_events_after_rebuild -- --nocapture`
- `cargo test -p im-adapters-local-disk --offline test_file_commit_journal_persists_across_reopen -- --nocapture`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test -- --nocapture`

## 6. Standardized Outcome

Managed `local-minimal` private deployment now restores the minimum viable conversation domain on cold rebuild when the same runtime dir is reused:

- conversation catalog state
- conversation member state
- conversation read cursors
- timeline summary and message list read models
- continued posting into existing conversations after restart

This closes the highest-value server-side recovery gap after the earlier disconnect-fence and checkpoint waves.

## 7. Residual Risk

This wave still does not make the whole platform fully restart-complete. The following remain non-durable or separately standardized:

- live realtime subscription sets
- presence heartbeats and transient device online state
- stream session runtime state
- RTC session runtime state
- media runtime state outside committed conversation references
- notification and automation runtime projections

Restart safety is therefore improved materially, but not yet universal across every subsystem.

## 8. Next Wave

The next durability review wave should target one of these two follow-ups:

1. durable recovery for live subscription/bootstrap state so online delivery can resume with less client-side re-sync
2. durable replay or snapshots for stream/RTC/media/notification runtimes that currently remain memory-first

The key rule stays unchanged: each recovery boundary must be standardized and tested independently instead of being implied by another persistence wave.
