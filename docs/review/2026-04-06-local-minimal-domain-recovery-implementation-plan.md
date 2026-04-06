# Local-Minimal Domain Recovery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make managed `local-minimal` rebuilds restore conversation-domain state from a durable local journal so private deployment can continue operating after restart without recreating conversations.

**Architecture:** Introduce a file-backed append-only commit journal under the runtime dir, then replay recorded conversation events into `ConversationRuntime` and `TimelineProjectionService` during startup. Keep the recovery seam pluggable: unmanaged builders stay memory-backed, while managed runtime-dir builders opt into the durable journal.

**Tech Stack:** Rust, Axum, serde/serde_json, local file persistence, existing `CommitJournal` abstraction, existing projection/event payload contracts.

---

### Task 1: Freeze Recovery Boundary And Red Test

**Files:**
- Create: `docs/review/2026-04-06-local-minimal-domain-recovery-review-cycle.md`
- Create: `docs/架构/95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md`
- Test: `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`

- [ ] **Step 1: Write the failing regression test**

Cover this flow:
- build app with `build_default_app_with_runtime_dir(...)`
- create conversation
- post first message
- rebuild app with the same runtime dir
- verify `GET /api/v1/conversations/{conversation_id}` succeeds
- verify `GET /api/v1/conversations/{conversation_id}/members` succeeds
- verify `POST /api/v1/conversations/{conversation_id}/messages` succeeds without recreating the conversation
- verify `GET /api/v1/conversations/{conversation_id}/messages` still contains the pre-restart message and the post-restart message

- [ ] **Step 2: Run the new test to verify it fails**

Run: `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test -- --nocapture`
Expected: FAIL with `404`/missing conversation state after rebuild.

### Task 2: Add Durable File Journal Adapter

**Files:**
- Modify: `crates/im-domain-events/src/lib.rs`
- Modify: `adapters/local-disk/src/lib.rs`
- Test: `adapters/local-disk/src/lib.rs`

- [ ] **Step 1: Add serde support for commit envelopes**

Add `Serialize`/`Deserialize` derives required for durable journal persistence.

- [ ] **Step 2: Write the file journal test**

Add a local-disk adapter test proving append + reopen + recorded replay ordering.

- [ ] **Step 3: Run the adapter test to verify it fails**

Run: `cargo test -p im-adapters-local-disk --offline test_file_commit_journal_persists_across_reopen -- --nocapture`
Expected: FAIL because the file journal adapter does not exist yet.

- [ ] **Step 4: Implement `FileCommitJournal`**

Requirements:
- append-only JSON-backed persistence
- startup replay via `recorded()`
- parent directory creation
- temp-file replace semantics

- [ ] **Step 5: Run the adapter test to verify it passes**

Run: `cargo test -p im-adapters-local-disk --offline test_file_commit_journal_persists_across_reopen -- --nocapture`
Expected: PASS.

### Task 3: Add Conversation Runtime Replay

**Files:**
- Modify: `services/conversation-runtime/src/lib.rs`
- Test: `services/conversation-runtime/tests/conversation_flow_test.rs`

- [ ] **Step 1: Write failing replay coverage**

Add a focused unit test that:
- records conversation create/member/message events
- rebuilds a fresh runtime
- replays envelopes
- verifies active member lookup and posting continue from the restored state

- [ ] **Step 2: Run the replay test to verify it fails**

Run: `cargo test -p conversation-runtime --offline replay -- --nocapture`
Expected: FAIL because the runtime cannot rebuild from envelopes yet.

- [ ] **Step 3: Implement replay entrypoints**

Add minimal replay methods that rebuild:
- conversation type
- members and principal membership map
- read cursors
- message index and stored message state
- member epoch / handoff epoch
- handoff state

- [ ] **Step 4: Run the replay test to verify it passes**

Run: `cargo test -p conversation-runtime --offline replay -- --nocapture`
Expected: PASS.

### Task 4: Wire Local-Minimal Startup Recovery

**Files:**
- Modify: `services/local-minimal-node/src/lib.rs`
- Test: `services/local-minimal-node/tests/domain_recovery_persistence_test.rs`

- [ ] **Step 1: Refactor local-minimal journal construction**

Introduce a journal wrapper that can use:
- memory journal for unmanaged/test builders
- file journal for runtime-dir-managed builders

- [ ] **Step 2: Replay durable journal during runtime-dir startup**

During managed builder initialization:
- load recorded envelopes
- replay them into `TimelineProjectionService`
- replay them into `ConversationRuntime`

- [ ] **Step 3: Run the new local-minimal domain recovery test**

Run: `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test -- --nocapture`
Expected: PASS.

### Task 5: Prevent Regressions And Close The Loop

**Files:**
- Modify: `docs/review/2026-04-06-local-minimal-domain-recovery-review-cycle.md`
- Modify: `docs/架构/95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md`
- Modify: `docs/部署/local-minimal-runtime-checkpoint-persistence-and-runtime-dir-standard-2026-04-06.md`

- [ ] **Step 1: Run focused verification**

Run:
- `cargo test -p conversation-runtime --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo test -p local-minimal-node --offline --test domain_recovery_persistence_test -- --nocapture`

Expected: PASS.

- [ ] **Step 2: Run broader verification**

Run:
- `cargo fmt --all --check`
- `cargo test -p session-gateway --offline`
- `cargo test -p local-minimal-node --offline`

Expected: PASS.

- [ ] **Step 3: Record findings and residual risks**

Document:
- what restart recovery now covers
- what still remains non-durable
- next wave recommendation
