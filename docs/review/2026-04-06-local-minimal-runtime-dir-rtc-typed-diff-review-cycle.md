# 2026-04-06 Local-Minimal Runtime-Dir RTC Typed Diff Review Cycle

## 1. Findings

### 1.1 High: generic object-key diff still hid RTC recovery semantics

After Standards 107 through 113, restore preview could already explain:

- file-level restore actions
- generic JSON object-key diffs
- disconnect fence typed diffs
- checkpoint typed diffs
- subscription typed diffs
- stream typed diffs
- preview-to-restore confirmation fingerprints

But `rtc-state.json` still stopped at record inequality, which meant operators could not tell whether a restore would:

- move a call from `started` to `accepted`, `rejected`, or `ended`
- change `signalingStreamId`
- change `artifactMessageId`
- add, remove, or rewrite persisted RTC signals

### 1.2 High: RTC state needs both session and signal semantics

`rtc-state.json` persists:

- an RTC session
- its persisted signaling events
- a record-level `updated_at`

That means operator review needs two semantic layers inside each shared record:

- session lifecycle and binding drift
- signal-log drift

### 1.3 Medium: category overlap is correct for RTC state

During TDD, the first expectation assumed a contract drift record would stay only in `otherModifiedKeys`.

That assumption was wrong. Because persisted `RtcSignalEvent` also embeds RTC session context such as `rtcMode`, a session-contract change can rewrite nested signal payload objects and therefore legitimately surface in `modifiedSignalKeys`.

The final standard keeps categories additive, not exclusive.

## 2. Root Cause

The restore preview stack already had a generic diff foundation, but there was no domain-aware summarizer for `rtc-state.json`.

Without that summarizer, RTC lifecycle, signaling-stream binding, and signal-log drift remained opaque during restore review.

## 3. Implementation

This review wave adds RTC-specific typed diff support:

- extended `RuntimeDirRestorePreviewDomainSummaryView` with additive RTC fields
- added an `rtc_state` typed summary path
- compared shared records by RTC-session semantics and signal-log semantics
- surfaced `updated_at`-only record changes separately from session and signal drift
- added `rtc-diff` rendering to the local preview formatter

This wave does not change:

- restore execution logic
- preview fingerprint behavior
- backup catalog behavior
- operator confirmation flow

## 4. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - added and removed RTC record keys
  - session state changes
  - signaling-stream changes
  - artifact-message changes
  - signal additions, removals, and modifications
  - `updated_at`-only record drift
  - `rtc-diff` formatter output

## 5. Verification

This wave must be verified with fresh command output:

- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline`

## 6. Standardized Outcome

`local-minimal` restore preview now explains RTC drift at the level operators actually reason about:

1. which RTC records change
2. whether RTC session lifecycle and bindings changed
3. whether persisted signals were added, removed, or rewritten

## 7. Residual Risk

This wave intentionally does **not** yet provide:

- typed summaries for `notification-tasks.json`
- typed summaries for `automation-executions.json`
- semantic merge or partial restore of RTC state
- remote orchestration workflows

## 8. Next Wave

The next review wave should target one of these:

1. typed summaries for `notification-tasks.json`
2. typed summaries for `automation-executions.json`
3. stricter private-deployment restore policy modes requiring preview fingerprint confirmation
