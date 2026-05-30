# 2026-04-06 Local-Minimal Task Runtime Projection Recovery Review Cycle

## 1. Findings

### 1.1 High: managed `local-minimal` rebuilds lost notification query continuity after restart

- `notification-service` kept task projections only in an in-memory `HashMap<String, NotificationTask>`.
- managed runtime-dir builders already persisted:
  - commit journal
  - realtime checkpoints
  - disconnect fences
  - live subscriptions
  - stream state
  - RTC state
- notification task projections still reset to empty memory after restart.
- the operational effect was immediate:
  - `GET /im/v3/api/notifications` returned an empty list after rebuild
  - `GET /im/v3/api/notifications/{id}` returned `404 notification_not_found`
  - automation side-effect notifications created before restart were no longer queryable

### 1.2 High: managed `local-minimal` rebuilds lost automation execution continuity after restart

- `automation-service` kept execution projections only in an in-memory `HashMap<String, AutomationExecution>`.
- managed runtime-dir builders never replaced the default execution runtime with a file-backed store.
- the operational effect was direct:
  - `GET /im/v3/api/automation/executions/{id}` returned `404 automation_execution_not_found`
  - idempotent retries after restart could not observe the previous execution projection

### 1.3 Medium: projection durability needed its own replaceable seam instead of being hidden inside domain replay

- notification and automation projections are not conversation-domain truth.
- both are operator-visible runtime projections with their own lookup patterns:
  - notification: point lookup plus recipient-scope list
  - automation: point lookup plus request-path idempotency
- commercial private deployment needs these seams to remain replaceable behind contracts rather than implicitly rebuilt from unrelated runtime families.

## 2. Root Cause

The root cause matched the same durability gap pattern as the earlier stream and RTC waves:

1. there were no pluggable projection-store contracts for notification tasks or automation executions
2. both runtimes stored query projections only in process memory
3. managed `local-minimal` runtime-dir builders still instantiated journal-backed runtimes with memory-only projection state
4. restart therefore preserved conversation-domain history but discarded task-runtime query surfaces

So the platform could replay committed domain history around task execution, but it could not restore the task projections that operators and APIs query directly.

## 3. Implementation

This review cycle completed the missing task-runtime recovery path:

- moved shared projection data types into `im-domain-core`
  - `notification::{NotificationStatus, NotificationTask}`
  - `automation::{AutomationExecutionState, AutomationExecution}`
- added pluggable contracts in `im-platform-contracts`
  - `NotificationTaskRecord`
  - `NotificationTaskStore`
  - `AutomationExecutionRecord`
  - `AutomationExecutionStore`
- added adapters:
  - `MemoryNotificationTaskStore`
  - `MemoryAutomationExecutionStore`
  - `FileNotificationTaskStore`
  - `FileAutomationExecutionStore`
- extended `NotificationRuntime`
  - added `with_journal_and_store(...)`
  - restored point lookups lazily by `tenant_id + notification_id`
  - restored recipient lists lazily by `tenant_id + recipient_id`
  - persisted the final dispatched projection after successful request handling
  - surfaced controlled `notification_store_*` errors
- extended `AutomationRuntime`
  - added `with_journal_and_store(...)`
  - restored point lookups lazily by `tenant_id + principal_id + execution_id`
  - restored idempotent request paths before duplicate detection
  - persisted the final succeeded projection after successful request handling
  - surfaced controlled `automation_store_*` errors
- bound managed `local-minimal` runtime-dir builders to:
  - `<runtime-dir>/state/notification-tasks.json`
  - `<runtime-dir>/state/automation-executions.json`
- kept unmanaged/default builders memory-backed

## 4. Regression Coverage

- `services/notification-service/tests/notification_runtime_persistence_test.rs`
  - `test_runtime_restores_notification_projection_on_rebuild_with_shared_store`
- `services/automation-service/tests/automation_runtime_persistence_test.rs`
  - `test_runtime_restores_automation_projection_on_rebuild_with_shared_store`
- `adapters/local-disk/src/lib.rs`
  - `test_file_notification_task_store_persists_across_reopen`
  - `test_file_automation_execution_store_persists_across_reopen`
- `services/local-minimal-node/tests/task_runtime_projection_persistence_test.rs`
  - `test_default_local_minimal_profile_restores_task_runtime_projections_after_rebuild`

## 5. Verification

Verified in this cycle with fresh command output:

- `cargo test -p notification-service --offline --test notification_runtime_persistence_test -- --nocapture`
- `cargo test -p automation-service --offline --test automation_runtime_persistence_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test task_runtime_projection_persistence_test -- --nocapture`

Additional broad verification ran after implementation stabilization:

- `cargo fmt --all --check`
- `cargo test -p notification-service --offline`
- `cargo test -p automation-service --offline`
- `cargo test -p im-adapters-local-disk --offline`
- `cargo test -p local-minimal-node --offline`

## 6. Standardized Outcome

Managed `local-minimal` private deployment now restores task-runtime projections across rebuild when the same runtime dir is reused.

The recovered surface covers:

- notification point lookup continuity
- notification recipient list continuity
- automation execution point lookup continuity
- automation idempotent request continuity
- automation-triggered notification side effects after restart
- runtime-dir file persistence for both task families

Clients and operators no longer lose notification or automation query continuity just because the node restarts.

## 7. Residual Risk

This wave still leaves several private-deployment runtime concerns outside the durable baseline:

- presence heartbeat truth as a separate runtime concern
- operator runtime-dir inspection and repair tooling
- cross-process reconciliation if journal append succeeds but task projection persistence fails mid-request

## 8. Next Wave

The next durability review wave should target:

1. presence heartbeat truth and resume/reconnect reconstruction
2. operator-facing runtime-dir inspection and repair tooling
3. explicit reconciliation tooling for journal-versus-projection drift detection
