# 113. Local-Minimal Runtime-Dir Stream Typed Diff Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide a domain-aware typed diff summary for `stream-state.json` on top of the generic restore preview diff workflow.

This standard exists because generic object-key diff explains which stream records would change, but it does not explain whether restore would move stream lifecycle state, advance or rewind stream progress, or add, remove, or rewrite persisted frames.

## 2. Scope

This standard applies to:

- `stream-state.json` restore preview actions
- optional typed stream diff summaries layered on top of generic object-key diff
- local text rendering of stream lifecycle and frame change semantics

This standard does not add:

- restore execution changes
- automatic stream repair
- semantic merge of stream state records
- typed summaries for unrelated runtime state files

## 3. Composition Rule

This standard composes with:

- [107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md](./107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md)
- [108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md](./108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md)

The composed explanation order is:

1. file-level restore action
2. generic JSON object key diff
3. stream-specific session and frame semantics

## 4. Eligibility Rule

The typed summary may be emitted only when all of these are true:

- preview action file is `stream-state.json`
- source and target payloads both exist
- source and target payloads differ
- both payloads are parseable as `Map<String, StreamStateRecord>`

If any condition fails, preview must omit the stream typed summary and keep generic preview behavior only.

## 5. Typed Summary Contract

Each eligible preview action may expose an optional `domainSummary` payload containing:

- `summaryKind`
- `addedKeys`
- `removedKeys`
- `streamStateChangedKeys`
- `streamLastFrameAdvancedKeys`
- `streamLastFrameRewoundKeys`
- `streamCheckpointAdvancedKeys`
- `streamCheckpointRewoundKeys`
- `streamResultMessageChangedKeys`
- `addedFrameKeys`
- `removedFrameKeys`
- `modifiedFrameKeys`
- `timestampOnlyChangedKeys`
- `otherModifiedKeys`
- `unchangedKeyCount`
- `unchangedFrameCount`

For this wave, the supported stream summary kind is:

- `stream_state`

## 6. Semantic Rule

Typed stream fields mean:

- `addedKeys`: stream records present in source backup and absent in current runtime state
- `removedKeys`: stream records present in current runtime state and absent in source backup
- `streamStateChangedKeys`: shared records whose session `state` differs
- `streamLastFrameAdvancedKeys`: shared records whose session `lastFrameSeq` is greater in source than target
- `streamLastFrameRewoundKeys`: shared records whose session `lastFrameSeq` is lower in source than target
- `streamCheckpointAdvancedKeys`: shared records whose session `lastCheckpointSeq` moves forward in source relative to target
- `streamCheckpointRewoundKeys`: shared records whose session `lastCheckpointSeq` moves backward in source relative to target
- `streamResultMessageChangedKeys`: shared records whose session `resultMessageId` differs
- `addedFrameKeys`: frame identifiers present in source and absent in target
- `removedFrameKeys`: frame identifiers present in target and absent in source
- `modifiedFrameKeys`: frame identifiers present in both source and target whose persisted frame payload differs
- `timestampOnlyChangedKeys`: shared records whose session and frame content are unchanged but `updated_at` differs
- `otherModifiedKeys`: shared records whose changes fall outside the tracked categories, including unsupported contract drift
- `unchangedKeyCount`: shared record keys whose typed records are fully equal
- `unchangedFrameCount`: shared frame identifiers whose persisted frame payloads are fully equal across compared records

Nested frame identifiers must use this deterministic format:

- `<recordKey>#frame:<frameSeq>`

Example:

- `t_demo:st_chat_delta#frame:2`

The same stream record may appear in multiple semantic categories at once.

For example, a record with an added frame may also appear in `streamLastFrameAdvancedKeys`.

## 7. Read-Only Rule

The stream typed summary is informational only.

It must not:

- mutate runtime stream state
- normalize persisted frame payloads
- change restore preview aggregate status
- suppress generic `changeSummary`
- imply that a restore should or should not be executed

## 8. Formatter Rule

Text rendering must expose an indented `stream-diff` line under the related action.

The line must include:

- added and removed stream record keys
- session state changes
- last-frame advance and rewind keys
- checkpoint advance and rewind keys
- result-message changes
- frame additions, removals, and modifications
- updated-at-only keys
- other-modified keys
- unchanged record count
- unchanged frame count

This line is additive to the generic `json-object-diff` line.

## 9. Determinism Rule

Typed stream summaries must be stable across repeated preview executions against unchanged inputs:

- stream record keys must be ordered deterministically
- frame identifiers must be ordered deterministically
- session comparisons must depend only on parsed payload content
- formatter output must preserve stable ordering

## 10. Verification Standard

Regression coverage must prove:

1. added and removed stream record keys are surfaced
2. session state changes are surfaced
3. last-frame and checkpoint advance/rewind are surfaced separately
4. result-message changes are surfaced
5. frame additions, removals, and modifications are surfaced separately
6. `updated_at`-only record drift is surfaced separately
7. generic preview and full package verification remain green

## 11. Design Consequence

Restore preview for persisted stream state now exposes stream lifecycle and frame replay drift instead of only raw record inequality.

This makes stream recovery intent visible during private deployment incident review while keeping restore execution explicit, read-only preview semantics intact, and component seams unchanged.
