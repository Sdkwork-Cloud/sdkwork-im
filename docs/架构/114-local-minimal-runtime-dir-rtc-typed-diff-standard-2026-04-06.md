# 114. Local-Minimal Runtime-Dir RTC Typed Diff Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide a domain-aware typed diff summary for `rtc-state.json` on top of the generic restore preview diff workflow.

This standard exists because generic object-key diff explains which RTC records would change, but it does not explain whether restore would move RTC session lifecycle state, change signaling stream bindings, attach artifact messages, or add, remove, or rewrite persisted RTC signals.

## 2. Scope

This standard applies to:

- `rtc-state.json` restore preview actions
- optional typed RTC diff summaries layered on top of generic object-key diff
- local text rendering of RTC session and signal-log change semantics

This standard does not add:

- restore execution changes
- automatic RTC repair
- semantic merge of RTC state records
- typed summaries for unrelated runtime state files

## 3. Composition Rule

This standard composes with:

- [107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md](./107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md)
- [108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md](./108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md)

The composed explanation order is:

1. file-level restore action
2. generic JSON object key diff
3. RTC-specific session and signal semantics

## 4. Eligibility Rule

The typed summary may be emitted only when all of these are true:

- preview action file is `rtc-state.json`
- source and target payloads both exist
- source and target payloads differ
- both payloads are parseable as `Map<String, RtcStateRecord>`

If any condition fails, preview must omit the RTC typed summary and keep generic preview behavior only.

## 5. Typed Summary Contract

Each eligible preview action may expose an optional `domainSummary` payload containing:

- `summaryKind`
- `addedKeys`
- `removedKeys`
- `rtcStateChangedKeys`
- `rtcSignalingStreamChangedKeys`
- `rtcArtifactMessageChangedKeys`
- `addedSignalKeys`
- `removedSignalKeys`
- `modifiedSignalKeys`
- `timestampOnlyChangedKeys`
- `otherModifiedKeys`
- `unchangedKeyCount`
- `unchangedSignalCount`

For this wave, the supported RTC summary kind is:

- `rtc_state`

## 6. Semantic Rule

Typed RTC fields mean:

- `addedKeys`: RTC records present in source backup and absent in current runtime state
- `removedKeys`: RTC records present in current runtime state and absent in source backup
- `rtcStateChangedKeys`: shared records whose session `state` differs
- `rtcSignalingStreamChangedKeys`: shared records whose session `signalingStreamId` differs
- `rtcArtifactMessageChangedKeys`: shared records whose session `artifactMessageId` differs
- `addedSignalKeys`: persisted signal identifiers present in source and absent in target
- `removedSignalKeys`: persisted signal identifiers present in target and absent in source
- `modifiedSignalKeys`: persisted signal identifiers present in both source and target whose signal payload differs
- `timestampOnlyChangedKeys`: shared records whose session and signal content are unchanged but `updated_at` differs
- `otherModifiedKeys`: shared records whose changes fall outside the tracked categories, including unsupported contract drift
- `unchangedKeyCount`: shared record keys whose typed records are fully equal
- `unchangedSignalCount`: shared persisted signal identifiers whose payloads are fully equal across compared records

RTC signals currently have no stable persisted id, so this wave uses deterministic append-order indexing for signal identifiers:

- `<recordKey>#signal:<index>`

Example:

- `t_demo:rtc_call_a#signal:1`

The same RTC record may appear in multiple semantic categories at once.

For example, an RTC contract drift can also rewrite embedded signal payload fields and therefore surface in `modifiedSignalKeys`.

## 7. Read-Only Rule

The RTC typed summary is informational only.

It must not:

- mutate runtime RTC state
- normalize persisted signal payloads
- change restore preview aggregate status
- suppress generic `changeSummary`
- imply that a restore should or should not be executed

## 8. Formatter Rule

Text rendering must expose an indented `rtc-diff` line under the related action.

The line must include:

- added and removed RTC record keys
- session state changes
- signaling-stream changes
- artifact-message changes
- signal additions, removals, and modifications
- updated-at-only keys
- other-modified keys
- unchanged record count
- unchanged signal count

This line is additive to the generic `json-object-diff` line.

## 9. Determinism Rule

Typed RTC summaries must be stable across repeated preview executions against unchanged inputs:

- RTC record keys must be ordered deterministically
- signal identifiers must be ordered deterministically
- session comparisons must depend only on parsed payload content
- formatter output must preserve stable ordering

## 10. Verification Standard

Regression coverage must prove:

1. added and removed RTC record keys are surfaced
2. session state changes are surfaced
3. signaling-stream changes are surfaced
4. artifact-message changes are surfaced
5. signal additions, removals, and modifications are surfaced separately
6. `updated_at`-only record drift is surfaced separately
7. generic preview and full package verification remain green

## 11. Design Consequence

Restore preview for persisted RTC state now exposes call lifecycle and signaling drift instead of only raw record inequality.

This makes RTC recovery intent visible during private deployment incident review while keeping restore execution explicit, read-only preview semantics intact, and component seams unchanged.
