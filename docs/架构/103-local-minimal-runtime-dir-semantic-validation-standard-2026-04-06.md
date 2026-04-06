# 103. Local-Minimal Runtime-Dir Semantic Validation Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must validate runtime-dir state files against their real durable data contracts, not just against generic JSON syntax.

This standard freezes the semantic-validation layer that sits on top of Standard 102 inspection.

## 2. Scope

This standard applies to:

- `local-minimal-node` runtime-dir inspection logic
- typed validation of all managed runtime-dir state files
- replay-aware validation of `commit-journal.json`
- operator-visible semantics of `parseable`, `status`, and `parseError`

This standard does not add automatic file repair.

## 3. Surface Compatibility Rule

Standard 102 remains the response-shape contract:

- `GET /api/v1/ops/runtime-dir`
- `local-minimal-node inspect-runtime-dir --runtime-dir <path> [--json]`
- the existing `RuntimeDirInspectionView` and `RuntimeDirInspectionItem` fields

This standard tightens validation behavior behind that same surface.

## 4. Required Typed Validation Matrix

Managed inspection must validate each file against its real durable type:

- `commit-journal.json` -> `Vec<CommitEnvelope>`
- `realtime-disconnect-fences.json` -> `BTreeMap<String, RealtimeDisconnectFenceRecord>`
- `realtime-checkpoints.json` -> `BTreeMap<String, RealtimeCheckpointRecord>`
- `realtime-subscriptions.json` -> `BTreeMap<String, RealtimeSubscriptionRecord>`
- `presence-state.json` -> `BTreeMap<String, PresenceStateRecord>`
- `stream-state.json` -> `BTreeMap<String, StreamStateRecord>`
- `rtc-state.json` -> `BTreeMap<String, RtcStateRecord>`
- `notification-tasks.json` -> `BTreeMap<String, NotificationTaskRecord>`
- `automation-executions.json` -> `BTreeMap<String, AutomationExecutionRecord>`

Validation as generic `serde_json::Value` is insufficient and non-compliant with this standard.

## 5. Journal Replay Rule

`commit-journal.json` requires deeper validation than the other files.

After typed load succeeds, inspection must also replay recorded envelopes into a fresh:

- `TimelineProjectionService`
- `ConversationRuntime`

The journal must be reported as `corrupt` when replay fails because of:

- invalid event payload shape
- missing prerequisite conversation state
- impossible replay ordering
- other fail-closed startup invariants

This rule aligns inspection with Standard 95 startup-replay safety.

## 6. Parseability Semantics

The `parseable` field is interpreted as follows:

1. missing file -> `parseable = false`
2. malformed JSON or wrong typed durable shape -> `parseable = false`
3. typed parse succeeds but semantic replay fails -> `parseable = true`
4. typed parse and semantic replay both succeed -> `parseable = true`

This distinction is required so operators can tell the difference between:

- broken file structure
- structurally valid but operationally unreplayable state

## 7. Failure Semantics

Inspection must fail closed:

- do not silently coerce arrays into maps
- do not auto-repair corrupt state during inspection
- do not suppress replay errors for `commit-journal.json`
- do not downgrade semantic corruption to `ok`

The operator-visible outcome must remain deterministic:

- `ok`
- `missing`
- `corrupt`

with the Standard 102 recommended action vocabulary preserved.

## 8. Verification Standard

Regression coverage must prove:

1. valid minimal managed state files report `ok`
2. wrong typed top-level file shape reports `corrupt`
3. invalid journal replay order or missing prerequisites report `corrupt`
4. journal replay corruption keeps `parseable = true`
5. endpoint and script entrypoints keep the Standard 102 contract shape

## 9. Composition Rule

This standard composes with:

- [95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md](./95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md)
- [96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md](./96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md)
- [97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md](./97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md)
- [98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md](./98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md)
- [99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md](./99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md)
- [100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md](./100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md)
- [101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md](./101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md)
- [102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md](./102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md)

The composition outcome is:

- runtime-dir inspection remains stable for operators
- semantic validation reflects the real durable store contracts
- journal inspection and startup replay now agree on what counts as valid state

## 10. Design Consequence

Semantic validation remains a replaceable operator-safety seam instead of being hidden inside one script, one storage backend, or one deployment mode.

Future storage replacement must preserve the same typed and replay-aware validation semantics behind the inspection contract.

Backup-first local repair for missing files is standardized separately by:

- [104-local-minimal-runtime-dir-safe-repair-standard-2026-04-06.md](./104-local-minimal-runtime-dir-safe-repair-standard-2026-04-06.md)
