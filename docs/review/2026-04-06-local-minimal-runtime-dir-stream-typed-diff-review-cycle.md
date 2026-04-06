# 2026-04-06 Local-Minimal Runtime-Dir Stream Typed Diff Review Cycle

## 1. Findings

### 1.1 High: generic object-key diff still hid stream recovery semantics

After Standards 107 through 112, restore preview could already explain:

- file-level restore actions
- generic JSON object-key diffs
- disconnect fence typed diffs
- checkpoint typed diffs
- subscription typed diffs
- preview-to-restore confirmation fingerprints

But `stream-state.json` still stopped at record inequality, which meant operators could not tell whether a restore would:

- reopen or complete a stream
- advance or rewind `lastFrameSeq`
- advance or rewind `lastCheckpointSeq`
- attach or change `resultMessageId`
- add, remove, or rewrite persisted frames

### 1.2 High: stream state needs both session and frame semantics

`stream-state.json` persists:

- a stream session
- its persisted frames
- a record-level `updated_at`

That means operator review needs two semantic layers inside each shared record:

- session lifecycle and progress drift
- frame-set drift

### 1.3 Medium: category overlap is correct for stream state

During TDD, the first expectation treated categories as mutually exclusive.

That assumption was wrong. A persisted stream can both:

- gain a frame
- and therefore advance `lastFrameSeq`

The final standard keeps categories additive, not exclusive.

## 2. Root Cause

The restore preview stack already had a generic diff foundation, but there was no domain-aware summarizer for `stream-state.json`.

Without that summarizer, stream replay, checkpoint, and frame drift remained opaque during restore review.

## 3. Implementation

This review wave adds stream-specific typed diff support:

- extended `RuntimeDirRestorePreviewDomainSummaryView` with additive stream fields
- added a `stream_state` typed summary path
- compared shared records by session lifecycle semantics and frame-set semantics
- surfaced `updated_at`-only record changes separately from session and frame drift
- added `stream-diff` rendering to the local preview formatter

This wave does not change:

- restore execution logic
- preview fingerprint behavior
- backup catalog behavior
- operator confirmation flow

## 4. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - added and removed stream record keys
  - session state changes
  - last-frame advance and rewind
  - checkpoint advance and rewind
  - result-message changes
  - frame additions, removals, and modifications
  - `updated_at`-only record drift
  - `stream-diff` formatter output

## 5. Verification

This wave must be verified with fresh command output:

- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline`

## 6. Standardized Outcome

`local-minimal` restore preview now explains stream drift at the level operators actually reason about:

1. which stream records change
2. whether stream lifecycle and progress move forward or backward
3. whether persisted frames are added, removed, or rewritten

## 7. Residual Risk

This wave intentionally does **not** yet provide:

- typed summaries for `rtc-state.json`
- typed summaries for `notification-tasks.json`
- semantic merge or partial restore of stream state
- remote orchestration workflows

## 8. Next Wave

The next review wave should target one of these:

1. typed summaries for `rtc-state.json`
2. typed summaries for `notification-tasks.json` or `automation-executions.json`
3. stricter private-deployment restore policy modes requiring preview fingerprint confirmation
