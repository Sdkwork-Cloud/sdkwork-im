# 2026-04-06 Local-Minimal Runtime-Dir Safe Repair Review Cycle

## 1. Findings

### 1.1 High: inspection and semantic validation could detect missing files, but operators still had no safe local repair path

- after Standards 102 and 103, operators could reliably tell whether managed runtime-dir state was:
  - missing
  - corrupt
  - semantically unreplayable
- but the platform still relied on manual file recreation for the simplest recovery case:
  - a managed runtime-dir with missing state files only

### 1.2 High: the safest repair target was already well-defined by the inspection contract

- Standard 102 already classified missing files with:
  - `recommendedAction = recreate_on_next_managed_start_or_write`
- that made `missing` files the correct first repair boundary:
  - the system already knew they were absent
  - the system already knew they were recreatable
  - unlike `corrupt` files, recreating them does not overwrite ambiguous on-disk business data

### 1.3 Medium: operators needed traceable backups even for a "safe" repair

- even when only missing files are recreated, private-deployment tooling still needs:
  - a timestamped backup location
  - a repair report
  - a reproducible operator trail

## 2. Root Cause

The root cause was an incomplete operator lifecycle:

1. runtime-dir state became durable
2. inspection standardized detection
3. semantic validation standardized correctness
4. but there was still no explicit, supported repair command for the low-risk missing-file case

That left a gap between "problem identified" and "safe local remediation executed".

## 3. Implementation

This review cycle adds a separate local repair seam without weakening inspection safety:

- added `repair_runtime_dir(...)` in `local-minimal-node`
- added `RuntimeDirRepairView` and `RuntimeDirRepairActionView`
- added local CLI entrypoint:
  - `local-minimal-node repair-runtime-dir --runtime-dir <path> [--json]`
- added lifecycle wrappers:
  - `bin/repair-runtime-local.ps1`
  - `bin/repair-runtime-local.sh`
  - `bin/repair-runtime-local.cmd`
- updated status scripts to point operators at both:
  - inspection
  - repair

The repair logic is intentionally narrow:

1. inspect current runtime-dir state
2. create a timestamped backup directory under `<runtime-dir>/backups/...`
3. snapshot existing managed state files into that backup
4. recreate only files currently classified as `missing`
5. leave `corrupt` files untouched
6. emit a structured repair report and post-repair inspection result

## 4. Safety Rule

This wave keeps the fail-closed posture:

- `missing` -> may be recreated with typed-empty content
- `corrupt` -> must not be auto-rewritten
- inspection remains read-only
- repair is explicit and local-only

This avoids the dangerous anti-pattern of silently "fixing" ambiguous business data during a health check.

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_repair_test.rs`
  - `test_repair_runtime_dir_recreates_missing_files_with_backup_first_flow`
  - `test_repair_runtime_dir_leaves_corrupt_files_untouched_while_fixing_missing_files`
- `services/local-minimal-node/tests/deployment_profile_test.rs`
  - repair script asset assertions

## 6. Verification

Verified in this cycle with fresh command output:

- `cargo test -p local-minimal-node --offline --test runtime_dir_repair_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`

Broader verification then confirmed the integrated local-minimal profile:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline`
- `powershell -ExecutionPolicy Bypass -File bin\\repair-runtime-local.ps1 -RuntimeDir .runtime\\local-minimal -Json`
- `powershell -ExecutionPolicy Bypass -File bin\\inspect-runtime-local.ps1 -RuntimeDir .runtime\\local-minimal -Json`

## 7. Standardized Outcome

Managed `local-minimal` private deployment now has a full low-risk operator loop for the missing-file case:

1. inspect
2. validate
3. backup
4. recreate missing files
5. re-inspect

That gives operators a supported recovery path for the safest class of runtime-dir failures without weakening protection around corrupt state.

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- automatic repair for `corrupt` files
- guided restore from a chosen backup snapshot
- remote authenticated HTTP repair APIs
- cross-node repair orchestration for clustered private deployment

## 9. Next Wave

The next review wave should target one of these:

1. guided corrupt-file remediation using explicit operator confirmations
2. selective restore from backup snapshots
3. authenticated control-plane repair workflows for private clustered deployments
