# 95. Local-Minimal Domain Journal Replay Recovery Standard (2026-04-06)

## 1. Goal

The managed `local-minimal` deployment profile must restore conversation-domain continuity across process restart without requiring an external database, message queue, or event bus.

This standard freezes the runtime-dir-backed commit-journal replay contract for private deployment.

## 2. Scope

This standard applies to:

- `local-minimal-node` when launched through managed runtime-dir lifecycle
- startup rebuild of the conversation domain
- startup rebuild of conversation read models used by summary, member, and timeline APIs
- durable local recovery based on the existing `CommitJournal` abstraction

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The durable commit journal path is standardized as:

```text
<runtime-dir>/state/commit-journal.json
```

## 4. Builder Contract

`local-minimal-node` must expose runtime-dir-aware builders:

- `build_default_app_with_runtime_dir(...)`
- `build_public_app_with_runtime_dir(...)`

When a runtime dir is configured, those builders must bind a durable `CommitJournal` implementation at:

- `<runtime-dir>/state/commit-journal.json`

The managed runtime-dir builders must keep the seam pluggable:

- runtime-dir builders use `FileCommitJournal`
- unmanaged/in-process builders may continue to use `MemoryCommitJournal`

No higher-level runtime may assume a specific storage vendor beyond the `CommitJournal` contract.

## 5. Journal Persistence Contract

The local file adapter must provide:

- append-only logical event growth
- replay in recorded append order
- parent directory creation
- temp-file replace semantics during file writes
- JSON serialization of `CommitEnvelope`

The adapter is allowed to rewrite the full JSON payload internally as long as the logical contract remains append-only and replay order remains stable.

## 6. Startup Replay Contract

Managed startup must:

1. load recorded commit envelopes from the durable journal
2. replay them in order into `TimelineProjectionService`
3. replay them in order into `ConversationRuntime`
4. refuse to start silently with empty state if journal load or replay fails

Fail-closed behavior is required. A managed private deployment must not silently discard durable domain history and continue as if the node were fresh.

## 7. Minimum Recovered Domain Surface

Replay must restore enough state for commercial private deployment to continue operating existing conversations after restart.

At minimum, replay must rebuild:

- conversation type
- active member map and member history entries
- read cursors
- message sequence and stored message state
- message index for message mutation APIs
- agent-handoff state
- projection summary view
- projection timeline view
- projection member view

After replay, the following behavior must hold:

- existing conversation summary queries succeed
- existing member list queries succeed
- posting a new message to the existing conversation succeeds
- the message sequence continues from the last committed sequence

## 8. Failure Semantics

Journal corruption, unreadable journal files, or invalid replay order must fail closed.

Examples:

- replaying `message.posted` before `conversation.created`
- replaying member or cursor events for a missing conversation
- invalid JSON payloads in the journal

Managed startup may reject boot by panicking or terminating the process with a clear error. Silent fallback to empty memory is forbidden.

## 9. Explicit Non-Goals

This standard does **not** yet require automatic recovery of:

- live realtime subscription sets outside the separate Standard 96 bootstrap-recovery boundary
- presence runtime state outside the separate Standard 101 persistence boundary
- registered client route runtime caches outside projection rebuild
- stream runtime state outside the separate Standard 97 persistence boundary
- RTC runtime state outside the separate Standard 98 persistence boundary
- media runtime state beyond already committed conversation references
- notification runtime state outside the separate Standard 99 persistence boundary
- automation runtime state outside the separate Standard 100 persistence boundary
- runtime-dir inspection and repair outside the separate Standards 102 and 103 boundaries

Those boundaries require separate recovery standards.

## 10. Verification Standard

Regression coverage must prove:

1. the local file journal persists recorded commit envelopes across reopen
2. `ConversationRuntime` can replay recorded conversation events into a fresh runtime
3. a managed local-minimal rebuild with the same runtime dir restores conversation summary and members
4. posting after rebuild continues the conversation instead of requiring recreation
5. the restored timeline contains both pre-restart and post-restart messages

## 11. Design Consequence

This standard gives private deployment a restart-safe conversation-domain baseline while preserving the component seam needed for future storage replacement.

It also freezes an architectural rule:

- checkpoint durability and domain durability are separate standards
- live subscription bootstrap recovery is a third separate standard
- stream runtime persistence is a fourth separate standard
- RTC runtime persistence is a fifth separate standard
- notification runtime persistence is a sixth separate standard
- automation runtime persistence is a seventh separate standard
- presence runtime persistence is an eighth separate standard
- runtime-dir inspection and repair is a ninth separate standard
- runtime-dir semantic validation is a tenth separate standard
- managed private deployment may compose all nine standards in the same runtime-dir profile
- future storage replacement must happen behind the same journal contract, not by rewriting the conversation/runtime API surface
