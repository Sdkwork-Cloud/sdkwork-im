# 100. Local-Minimal Automation Runtime Persistence Standard (2026-04-06)

## 1. Goal

The managed `local-minimal` deployment profile must preserve automation execution projections across process restart without requiring Redis, Kafka, or any external persistence service.

This standard freezes the runtime-dir-backed private-deployment contract for automation execution recovery.

## 2. Scope

This standard applies to:

- `local-minimal-node` managed runtime-dir builders
- `automation-service` runtime projection persistence
- automation execution lookup recovery after rebuild
- automation request idempotency recovery after rebuild
- automation store failure handling

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The durable automation projection path is standardized as:

```text
<runtime-dir>/state/automation-executions.json
```

## 4. Store Contract

The platform must persist automation projections behind a pluggable `AutomationExecutionStore` seam keyed by:

- `tenant_id`
- `principal_id`
- `execution_id`

At minimum, the durable record must preserve:

- the full `AutomationExecution`
- `updated_at`

## 5. Builder Contract

When a runtime dir is configured, managed builders must bind `FileAutomationExecutionStore` at:

```text
<runtime-dir>/state/automation-executions.json
```

Default unmanaged/in-process builders may continue to use a memory-backed store.

## 6. Recovery Rule

`AutomationRuntime` must restore execution projections lazily on access.

Typical sequences:

1. process restarts with the same runtime dir
2. a client calls:
   - `GET /im/v3/api/automation/executions/{id}`
   - `POST /im/v3/api/automation/executions`
3. runtime loads persisted state for `tenant + principal + execution_id`
4. the operation continues against restored in-memory state

Global startup replay is not required for automation execution projections.

## 7. Mutation Contract

The durable automation projection must be updated after successful final execution mutation in:

- `request_execution(...)`

Persistence must record the final completed execution state so restart does not break:

- direct execution lookup
- idempotent execution retries
- downstream notification side effects that reference the execution result

## 8. Failure Semantics

Store failures must never panic the process.

Runtime errors must be surfaced as controlled API errors:

- `automation_store_unavailable -> 503`
- `automation_store_conflict -> 409`
- `automation_store_unsupported -> 501`

The platform must fail closed instead of silently degrading to empty automation state.

## 9. Verification Standard

Regression coverage must prove:

1. the local file adapter persists automation executions across reopen
2. a fresh `AutomationRuntime` with the same store restores direct execution lookup continuity
3. a managed `local-minimal` rebuild with the same runtime dir restores execution query continuity
4. the managed profile writes `automation-executions.json` under the runtime state dir

## 10. Composition Rule

This standard composes with:

- [95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md](./95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md)
- [96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md](./96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md)
- [97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md](./97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md)
- [98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md](./98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md)
- [99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md](./99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md)
- [101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md](./101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md)

The composition outcome is:

- conversation-domain truth survives restart
- realtime live intent survives restart
- stream runtime state survives restart
- RTC runtime state survives restart
- presence inventory and timestamps survive restart
- notification side-effect visibility survives restart
- automation execution query and idempotency surfaces survive restart

## 11. Design Consequence

Automation durability remains a replaceable runtime seam instead of being derived implicitly from unrelated runtime families.

Future storage replacement must remain behind `AutomationExecutionStore`.

Operator inspection and repair workflows are standardized separately by:

- [102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md](./102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md)
- [103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md](./103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md)
