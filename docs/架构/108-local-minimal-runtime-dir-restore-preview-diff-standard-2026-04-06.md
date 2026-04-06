# 108. Local-Minimal Runtime-Dir Restore Preview Diff Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide an operator-readable field-level diff summary on top of the existing read-only restore preview workflow.

This standard extends Standard 107 so operators can understand not only that a managed file would be restored, but also which top-level JSON object keys would be added, removed, or modified by that restore.

## 2. Scope

This standard applies to:

- `local-minimal-node` restore preview reporting
- per-file structured change summaries
- local text rendering of restore preview output
- JSON serialization of preview action details

This standard does not add:

- restore execution changes
- runtime-dir mutation
- automatic backup selection
- semantic merge or partial-file restore
- deep nested JSON path diffs

## 3. Composition Rule

This standard layers on top of:

- [107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md](./107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md)

Standard 107 remains the authoritative contract for:

- preview entrypoints
- read-only safety
- aggregate preview status
- per-file `would_restore` / `noop` / `skip` action semantics

This standard only enriches the informational payload attached to those actions.

## 4. Structured Summary Eligibility Rule

Restore preview may emit a structured field-level summary only when all of these are true:

- source file exists in the selected backup snapshot
- target file exists in the current runtime-dir
- source and target payloads differ byte-for-byte
- both payloads are parseable as top-level JSON objects

If any condition fails, preview must keep the existing coarse-grained action classification and omit the structured summary.

## 5. Structured Summary Contract

Each preview action may expose an optional `changeSummary` payload containing:

- `summaryKind`
- `sourceKeyCount`
- `targetKeyCount`
- `addedKeys`
- `removedKeys`
- `modifiedKeys`
- `unchangedKeyCount`

For this wave, the only supported summary kind is:

- `json_object_keys`

Key semantics:

- `addedKeys`: keys present in source backup payload and absent from current target payload
- `removedKeys`: keys present in current target payload and absent from source backup payload
- `modifiedKeys`: keys present in both payloads whose values differ
- `unchangedKeyCount`: keys present in both payloads whose values are equal

## 6. Determinism Rule

Structured summaries must be deterministic:

- keys must be ordered stably
- repeated preview calls against unchanged input must emit the same key ordering
- summary generation must not depend on filesystem enumeration order

Stable ordering is required for incident review, script consumption, and regression testing.

## 7. Safety Rule

This enhancement must remain informational only.

Structured summaries must not:

- rewrite source or target JSON
- normalize persisted runtime payloads
- validate or repair unrelated files
- change overall restore preview statuses
- change actual restore behavior

## 8. Formatter Rule

Text rendering of restore preview output must surface structured summaries in a compact operator-readable form immediately under the related action line.

Formatter output must:

- keep the existing action line intact
- render the structured summary as an additional indented line
- include added, removed, modified, and unchanged counts or keys

This keeps machine-readable JSON and human-readable CLI output aligned.

## 9. Current Deliberate Boundary

This wave intentionally supports only top-level JSON object key summaries.

The following remain coarse-grained:

- array payloads such as commit journals
- scalar JSON payloads
- invalid JSON payloads
- target-missing cases
- source-missing cases

Those cases continue to rely on existing preview actions and details such as `content_differs`, `target_missing`, or `missing_in_source_backup_snapshot`.

## 10. Verification Standard

Regression coverage must prove:

1. a changed JSON object file reports a structured key-level summary with added, removed, and modified keys
2. the formatted preview output surfaces the structured summary
3. sparse snapshot preview behavior remains unchanged
4. invalid backup input still fails without mutation
5. full package verification still passes after the enhancement

## 11. Design Consequence

Restore preview is now split into two operator layers:

1. action classification
2. optional field-level explanation

That improves recovery confidence without coupling preview to restore execution or introducing premature semantic merge behavior.

Future work such as nested path diffs, typed domain-aware summaries, or operator apply confirmations must compose with this standard rather than replacing the read-only preview seam.

Disconnect-fence-specific typed summaries are standardized separately in:

- [109-local-minimal-runtime-dir-disconnect-fence-typed-diff-standard-2026-04-06.md](./109-local-minimal-runtime-dir-disconnect-fence-typed-diff-standard-2026-04-06.md)

Checkpoint-specific typed summaries are standardized separately in:

- [110-local-minimal-runtime-dir-checkpoint-typed-diff-standard-2026-04-06.md](./110-local-minimal-runtime-dir-checkpoint-typed-diff-standard-2026-04-06.md)
