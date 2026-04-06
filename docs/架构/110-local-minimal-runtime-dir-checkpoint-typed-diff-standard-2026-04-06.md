# 110. Local-Minimal Runtime-Dir Checkpoint Typed Diff Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide a domain-aware typed diff summary for `realtime-checkpoints.json` on top of the generic restore preview diff workflow.

This standard exists because generic object-key diff explains which checkpoint records would change, but it does not explain whether checkpoint progress is advancing, rewinding, or only refreshing timestamps.

## 2. Scope

This standard applies to:

- `realtime-checkpoints.json` restore preview actions
- optional typed checkpoint diff summaries layered on top of generic object-key diff
- local text rendering of checkpoint change semantics

This standard does not add:

- restore execution changes
- automatic checkpoint repair
- semantic merge of checkpoint records
- typed summaries for unrelated runtime state files

## 3. Composition Rule

This standard composes with:

- [107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md](./107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md)
- [108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md](./108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md)

The composed explanation order is:

1. file-level restore action
2. generic JSON object key diff
3. checkpoint-specific sequence semantics

## 4. Eligibility Rule

The typed summary may be emitted only when all of these are true:

- preview action file is `realtime-checkpoints.json`
- source and target payloads both exist
- source and target payloads differ
- both payloads are parseable as `Map<String, RealtimeCheckpointRecord>`

If any condition fails, preview must omit the checkpoint typed summary and keep generic preview behavior only.

## 5. Typed Summary Contract

Each eligible preview action may expose an optional `domainSummary` payload containing:

- `summaryKind`
- `addedKeys`
- `removedKeys`
- `latestAdvancedKeys`
- `latestRewoundKeys`
- `ackedAdvancedKeys`
- `ackedRewoundKeys`
- `trimmedAdvancedKeys`
- `trimmedRewoundKeys`
- `timestampOnlyChangedKeys`
- `otherModifiedKeys`
- `unchangedKeyCount`

For this wave, the supported checkpoint summary kind is:

- `realtime_checkpoints`

## 6. Semantic Rule

Typed checkpoint fields mean:

- `addedKeys`: records present in source backup and absent in current runtime state
- `removedKeys`: records present in current runtime state and absent in source backup
- `latestAdvancedKeys`: records whose `latest_realtime_seq` is greater in source than target
- `latestRewoundKeys`: records whose `latest_realtime_seq` is lower in source than target
- `ackedAdvancedKeys`: records whose `acked_through_seq` is greater in source than target
- `ackedRewoundKeys`: records whose `acked_through_seq` is lower in source than target
- `trimmedAdvancedKeys`: records whose `trimmed_through_seq` is greater in source than target
- `trimmedRewoundKeys`: records whose `trimmed_through_seq` is lower in source than target
- `timestampOnlyChangedKeys`: records whose sequence fields are equal but `updated_at` differs
- `otherModifiedKeys`: records that changed outside the typed sequence and timestamp categories
- `unchangedKeyCount`: shared keys whose typed records are equal

The same checkpoint key may appear in multiple directional fields if several sequence counters change simultaneously.

## 7. Read-Only Rule

The checkpoint typed summary is informational only.

It must not:

- mutate runtime checkpoint state
- recalculate or normalize sequence values
- change restore preview aggregate status
- suppress generic `changeSummary`
- imply that a restore should or should not be executed

## 8. Formatter Rule

Text rendering must expose an indented `checkpoint-diff` line under the related action.

The line must include:

- added and removed keys
- latest advance and rewind keys
- acked advance and rewind keys
- trimmed advance and rewind keys
- timestamp-only keys
- other-modified keys
- unchanged count

This line is additive to the generic `json-object-diff` line.

## 9. Determinism Rule

Typed checkpoint summaries must be stable across repeated preview executions against unchanged inputs:

- key ordering must be deterministic
- category membership must depend only on parsed payload content
- formatter output must preserve stable ordering

## 10. Verification Standard

Regression coverage must prove:

1. added and removed checkpoint keys are surfaced
2. latest sequence advance and rewind are surfaced separately
3. acked sequence advance and rewind are surfaced separately
4. trimmed sequence advance and rewind are surfaced separately
5. timestamp-only change is surfaced separately from sequence movement
6. generic preview and full package verification remain green

## 11. Design Consequence

Restore preview for realtime checkpoints now provides typed recovery semantics instead of only raw value inequality.

This makes replay, acknowledgement, and trimming drift visible during incident review while keeping restore execution explicit and separate.
