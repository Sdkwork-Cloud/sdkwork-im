# 112. Local-Minimal Runtime-Dir Subscription Typed Diff Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide a domain-aware typed diff summary for `realtime-subscriptions.json` on top of the generic restore preview diff workflow.

This standard exists because generic object-key diff explains which device subscription records would change, but it does not explain whether restore would add or remove nested scopes, widen or narrow event-type coverage, or only refresh synchronization timestamps.

## 2. Scope

This standard applies to:

- `realtime-subscriptions.json` restore preview actions
- optional typed subscription diff summaries layered on top of generic object-key diff
- local text rendering of subscription change semantics

This standard does not add:

- restore execution changes
- automatic subscription repair
- semantic merge of subscription records
- typed summaries for unrelated runtime state files

## 3. Composition Rule

This standard composes with:

- [107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md](./107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md)
- [108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md](./108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md)

The composed explanation order is:

1. file-level restore action
2. generic JSON object key diff
3. subscription-specific nested scope semantics

## 4. Eligibility Rule

The typed summary may be emitted only when all of these are true:

- preview action file is `realtime-subscriptions.json`
- source and target payloads both exist
- source and target payloads differ
- both payloads are parseable as `Map<String, RealtimeSubscriptionRecord>`

If any condition fails, preview must omit the subscription typed summary and keep generic preview behavior only.

## 5. Typed Summary Contract

Each eligible preview action may expose an optional `domainSummary` payload containing:

- `summaryKind`
- `addedKeys`
- `removedKeys`
- `addedScopeKeys`
- `removedScopeKeys`
- `eventTypesAddedScopeKeys`
- `eventTypesRemovedScopeKeys`
- `subscribedAtOnlyChangedScopeKeys`
- `timestampOnlyChangedKeys`
- `otherModifiedKeys`
- `unchangedKeyCount`
- `unchangedScopeCount`

For this wave, the supported subscription summary kind is:

- `realtime_subscriptions`

## 6. Semantic Rule

Typed subscription fields mean:

- `addedKeys`: device subscription records present in source backup and absent in current runtime state
- `removedKeys`: device subscription records present in current runtime state and absent in source backup
- `addedScopeKeys`: nested scopes present in a shared source record and absent in the corresponding target record
- `removedScopeKeys`: nested scopes present in a shared target record and absent in the corresponding source record
- `eventTypesAddedScopeKeys`: shared scopes where source contains event types not present in target
- `eventTypesRemovedScopeKeys`: shared scopes where source omits event types that are present in target
- `subscribedAtOnlyChangedScopeKeys`: shared scopes where event-type coverage is unchanged but `subscribedAt` differs
- `timestampOnlyChangedKeys`: shared device records whose nested subscription items are unchanged but `synced_at` differs
- `otherModifiedKeys`: shared device records whose changes fall outside the tracked categories, including malformed identity drift or unsupported structural differences
- `unchangedKeyCount`: shared device record keys whose typed records are fully equal
- `unchangedScopeCount`: shared nested scopes whose typed subscription items are fully equal across compared records

Nested scope identifiers must use this deterministic format:

- `<recordKey>#<scopeType>:<scopeId>`

Example:

- `t_demo:u_demo:d_pad#conversation:c_demo`

## 7. Read-Only Rule

The subscription typed summary is informational only.

It must not:

- mutate runtime subscription state
- normalize or rewrite event-type ordering
- change restore preview aggregate status
- suppress generic `changeSummary`
- imply that a restore should or should not be executed

## 8. Formatter Rule

Text rendering must expose an indented `subscription-diff` line under the related action.

The line must include:

- added and removed device record keys
- added and removed nested scope keys
- event-type expansion and reduction scope keys
- subscribed-at-only scope keys
- synced-timestamp-only record keys
- other-modified record keys
- unchanged record count
- unchanged scope count

This line is additive to the generic `json-object-diff` line.

## 9. Determinism Rule

Typed subscription summaries must be stable across repeated preview executions against unchanged inputs:

- device keys must be ordered deterministically
- nested scope identifiers must be ordered deterministically
- event-type set comparison must depend only on parsed payload content
- formatter output must preserve stable ordering

## 10. Verification Standard

Regression coverage must prove:

1. added and removed device subscription keys are surfaced
2. added and removed nested scopes are surfaced
3. event-type expansion and reduction are surfaced separately
4. `subscribedAt`-only scope drift is surfaced separately
5. `synced_at`-only record drift is surfaced separately
6. generic preview and full package verification remain green

## 11. Design Consequence

Restore preview for realtime subscriptions now exposes nested delivery intent instead of only raw record inequality.

This makes subscription coverage drift visible during private deployment incident review while keeping restore execution explicit, read-only preview semantics intact, and component seams unchanged.
