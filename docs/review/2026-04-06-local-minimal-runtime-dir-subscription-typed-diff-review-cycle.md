# 2026-04-06 Local-Minimal Runtime-Dir Subscription Typed Diff Review Cycle

## 1. Findings

### 1.1 High: generic object-key diff still hid nested subscription intent

After Standards 107 through 111, restore preview could explain:

- file-level restore actions
- generic JSON object-key diffs
- disconnect fence typed diffs
- checkpoint typed diffs
- preview-to-restore confirmation fingerprints

But `realtime-subscriptions.json` still stopped at device-record inequality, which meant operators could not tell whether a restore would:

- add or remove a specific scope
- widen or shrink event-type coverage on a shared scope
- only refresh subscription timestamps

### 1.2 High: subscription state needs two semantic levels

`realtime-subscriptions.json` is not just a flat map. It is a device-record map whose values contain nested subscription scopes.

That means operator review needs both:

- record-level additions and removals
- nested scope-level drift inside shared records

### 1.3 Medium: the new summary had to stay additive

The correct next wave was not a new preview payload or a restore behavior change.

The correct seam was an additive typed summary layered onto the existing:

1. action line
2. generic object-key diff
3. file-specific typed diff

## 2. Root Cause

The restore preview stack already had a generic diff foundation, but there was no domain-aware summarizer for `realtime-subscriptions.json`.

Without that summarizer, nested subscription intent remained opaque during restore review.

## 3. Implementation

This review wave adds subscription-specific typed diff support:

- extended `RuntimeDirRestorePreviewDomainSummaryView` with additive subscription fields
- added a `realtime_subscriptions` typed summary path
- compared shared records by nested scope identity and event-type set drift
- surfaced `synced_at`-only record changes separately from scope-level subscription changes
- added `subscription-diff` rendering to the local preview formatter

This wave does not change:

- restore execution logic
- preview fingerprint behavior
- backup catalog behavior
- operator confirmation flow

## 4. Contract Accuracy Note

During TDD, the first subscription regression fixture used snake_case nested item fields.

The real persisted `RealtimeSubscription` contract uses camelCase nested fields, so the fixture was corrected to match actual runtime serialization before final verification.

That correction matters because it ensures the new typed diff is validated against the real on-disk contract rather than an invented test-only shape.

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_restore_preview_test.rs`
  - added and removed device subscription keys
  - added and removed nested scopes
  - event-type expansion and reduction
  - `subscribedAt`-only scope drift
  - `synced_at`-only record drift
  - `subscription-diff` formatter output

## 6. Verification

This wave must be verified with fresh command output:

- `cargo test -p local-minimal-node --offline --test runtime_dir_restore_preview_test -- --nocapture`
- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline`

## 7. Standardized Outcome

`local-minimal` restore preview now explains realtime subscription drift at the level operators actually reason about:

1. which device records change
2. which scopes within those records change
3. whether event delivery coverage widens, narrows, or only refreshes timestamps

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- typed summaries for `stream-state.json`
- typed summaries for `rtc-state.json`
- semantic merge or partial restore of subscription records
- remote orchestration workflows

## 9. Next Wave

The next review wave should target one of these:

1. typed summaries for `stream-state.json`
2. typed summaries for `rtc-state.json`
3. stricter private-deployment restore policy modes requiring preview fingerprint confirmation
