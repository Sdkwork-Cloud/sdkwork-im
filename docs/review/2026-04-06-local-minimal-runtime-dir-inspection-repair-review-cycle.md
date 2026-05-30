# 2026-04-06 Local-Minimal Runtime-Dir Inspection Repair Review Cycle

## 1. Findings

### 1.1 High: managed `local-minimal` had durable state files but no unified operator inspection surface

- by this point the managed runtime-dir profile already persisted:
  - `commit-journal.json`
  - `realtime-disconnect-fences.json`
  - `realtime-checkpoints.json`
  - `realtime-subscriptions.json`
  - `presence-state.json`
  - `stream-state.json`
  - `rtc-state.json`
  - `notification-tasks.json`
  - `automation-executions.json`
- but there was no single endpoint or script that classified these files as healthy, missing, or corrupt.
- private-deployment operators therefore had to inspect `.runtime/local-minimal/state` manually.

### 1.2 High: `ops-service` exposed node health but not restart-state integrity

- existing ops endpoints covered:
  - service health
  - cluster lifecycle
  - lag
  - diagnostics
- none of them answered the practical operator question:
  - "is my managed runtime-dir state complete and parseable?"
- for commercial private deployment, that leaves a blind spot between "process is up" and "durable state is trustworthy".

### 1.3 Medium: lifecycle scripts stopped at pid/log/health visibility

- `bin/status-local.ps1`
- `bin/status-local.sh`

These scripts surfaced runtime status, but did not provide a supported path to inspect managed runtime-dir persistence state.

## 2. Root Cause

The root cause was a missing operations boundary, not a missing persistence boundary:

1. restart-safe file persistence had been added wave by wave
2. each runtime family defined its own state file path
3. no operator-facing contract unified those files into one inspection model
4. lifecycle scripts and ops APIs therefore stopped at process-level health instead of state-level health

This meant the platform could preserve data, but still leave operators blind to runtime-dir drift, missing files, or malformed JSON.

## 3. Implementation

This review cycle added a read-only runtime-dir inspection layer:

- extended `ops-service` with:
  - `RuntimeDirInspectionView`
  - `RuntimeDirInspectionItem`
  - `GET /backend/v3/api/ops/runtime_dir`
- extended `local-minimal-node` with:
  - managed runtime-dir inspection logic
  - per-file classification:
    - `ok`
    - `missing`
    - `corrupt`
  - recommended action strings:
    - `none`
    - `recreate_on_next_managed_start_or_write`
    - `manual_json_repair_or_restore`
  - ongoing refresh through `refresh_node_operational_view(...)`
- added a local CLI entrypoint:
  - `local-minimal-node inspect-runtime-dir --runtime-dir <path> [--json]`
- added lifecycle scripts:
  - `bin/inspect-runtime-local.ps1`
  - `bin/inspect-runtime-local.sh`
  - `bin/inspect-runtime-local.cmd`
- updated status scripts to point operators at the deeper inspection step

## 4. Inspection Rule

This wave standardizes **inspection and classification**, not automatic business-data repair.

The operator contract is:

1. inspect the managed runtime dir and expected state files
2. classify each file as `ok`, `missing`, or `corrupt`
3. expose file path, parseability, byte size, and parse error when available
4. return aggregate counts and overall status
5. do not silently rewrite files during inspection

This keeps the first repair wave safe for commercial/private deployment:

- inspection is deterministic
- operator tooling is consistent
- no destructive auto-repair is hidden behind a "status" command

## 5. Regression Coverage

- `services/ops-service/tests/http_smoke_test.rs`
  - `test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `services/local-minimal-node/tests/runtime_dir_inspection_test.rs`
  - `test_managed_runtime_dir_inspection_reports_all_expected_files_when_parseable`
  - `test_managed_runtime_dir_inspection_reports_missing_and_corrupt_files_as_degraded`
- `services/local-minimal-node/tests/deployment_profile_test.rs`
  - deployment asset assertions for `inspect-runtime-local.*`

## 6. Verification

Verified in this cycle with fresh command output:

- `cargo test -p ops-service --offline --test http_smoke_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`

Additional broad verification ran after implementation stabilization:

- `cargo fmt --all --check`
- `cargo test -p ops-service --offline`
- `cargo test -p local-minimal-node --offline`

## 7. Standardized Outcome

Managed `local-minimal` private deployment now has a supported runtime-dir inspection surface through both API and local scripts.

Operators can now determine:

- whether the managed profile is runtime-dir-backed or unmanaged
- whether required state files are present
- whether persisted JSON is parseable
- which file needs manual intervention
- which missing file can be recreated by normal managed lifecycle activity

This closes the operations gap between "the node is running" and "the runtime-dir persistence set is actually trustworthy".

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- automatic business-state repair for corrupt files
- semantic validation beyond generic JSON parseability
- cluster-wide reconciliation of state drift across multiple nodes
- durable route ownership failover reconstruction

## 9. Next Wave

The next review wave should target one of these:

1. safe repair workflows for specific runtime families with explicit backup/restore rules
2. stronger semantic validation for managed runtime-dir state content
3. durable multi-node route ownership and failover reconstruction
