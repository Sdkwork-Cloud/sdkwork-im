# 109. Local-Minimal Runtime-Dir Disconnect Fence Typed Diff Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide a domain-aware typed diff summary for `realtime-disconnect-fences.json` on top of the generic restore preview diff workflow.

This standard exists because generic JSON object key summaries explain which fence keys would change, but they still do not distinguish operationally important changes such as `owner_node_id` migration and `session_id` turnover.

## 2. Scope

This standard applies to:

- `realtime-disconnect-fences.json` restore preview actions
- optional typed diff payloads layered on top of generic preview summaries
- local text rendering of disconnect fence changes

This standard does not add:

- any restore execution changes
- semantic merge
- automatic cleanup of stale fences
- typed summaries for other runtime state files

## 3. Composition Rule

This standard composes with:

- [107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md](./107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md)
- [108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md](./108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md)

The composition order is:

1. restore preview determines whether a file would change
2. generic diff explains top-level key additions, removals, and modifications
3. this standard explains disconnect-fence-specific semantics for modified entries

## 4. Eligibility Rule

The typed summary may be emitted only when all of these are true:

- preview action file is `realtime-disconnect-fences.json`
- source and target payloads both exist
- source and target payloads differ
- both payloads are parseable as `Map<String, RealtimeDisconnectFenceRecord>`

If any condition fails, preview must omit the typed summary and keep generic preview behavior only.

## 5. Typed Summary Contract

Each eligible preview action may expose an optional `domainSummary` payload containing:

- `summaryKind`
- `addedKeys`
- `removedKeys`
- `ownerNodeChangedKeys`
- `sessionChangedKeys`
- `otherModifiedKeys`
- `unchangedKeyCount`

For this wave, the only supported summary kind is:

- `disconnect_fences`

## 6. Semantic Rule

Typed summary fields mean:

- `addedKeys`: fence keys present in source backup and absent in current runtime state
- `removedKeys`: fence keys present in current runtime state and absent in source backup
- `ownerNodeChangedKeys`: shared fence keys whose `owner_node_id` differs
- `sessionChangedKeys`: shared fence keys whose `session_id` differs
- `otherModifiedKeys`: shared fence keys that differ but did not change `owner_node_id` or `session_id`
- `unchangedKeyCount`: shared fence keys that are byte-equivalent as typed records

The same key may appear in both `ownerNodeChangedKeys` and `sessionChangedKeys` if both fields differ.

## 7. Read-Only Rule

The typed summary is informational only.

It must not:

- mutate runtime state
- normalize disconnect fence payloads
- suppress generic key-level diff output
- change `would_restore`, `noop`, or `skip` action semantics
- infer that a restore is safe or unsafe on its own

## 8. Formatter Rule

Text rendering must expose an indented `disconnect-fence-diff` line under the related action.

The formatter must include:

- added keys
- removed keys
- owner-changed keys
- session-changed keys
- other-modified keys
- unchanged count

This line is additive to the existing `json-object-diff` line and must not replace it.

## 9. Determinism Rule

Typed summaries must be stable across repeated preview executions against unchanged inputs:

- key ordering must be deterministic
- summary classification must depend only on parsed payload content
- formatter output must preserve stable ordering

## 10. Verification Standard

Regression coverage must prove:

1. added and removed fence keys are surfaced
2. owner-node migrations are surfaced separately
3. session changes are surfaced separately
4. non-owner and non-session modifications fall into `otherModifiedKeys`
5. generic preview and full package verification remain green

## 11. Design Consequence

Restore preview for disconnect fences now has three explanation layers:

1. file-level action
2. generic object-key diff
3. typed fence semantics

This gives operators more confidence during node failover and recovery workflows without collapsing preview into restore orchestration.
