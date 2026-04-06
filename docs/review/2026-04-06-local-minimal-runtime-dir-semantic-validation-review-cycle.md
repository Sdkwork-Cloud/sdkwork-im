# 2026-04-06 Local-Minimal Runtime-Dir Semantic Validation Review Cycle

## 1. Findings

### 1.1 High: generic JSON parseability misclassified typed-store corruption as healthy

- the first runtime-dir inspection wave only checked whether each state file could be parsed as generic `serde_json::Value`
- that allowed structurally wrong but syntactically valid JSON to be reported as healthy
- example:
  - `realtime-checkpoints.json = []`
  - `presence-state.json = []`
  - `notification-tasks.json = []`
- these files are not valid runtime state because the real durable contract is a typed `BTreeMap<...>` for each store

### 1.2 High: `commit-journal.json` could be syntactically valid but still unreplayable

- the previous inspection wave did not distinguish between:
  - "the file is valid JSON"
  - "the file can actually rebuild runtime state"
- this left a serious private-deployment blind spot:
  - `commit-journal.json` could deserialize into `Vec<CommitEnvelope>`
  - but replay still fail because payloads or event order violate startup invariants
- example:
  - `message.posted` payload is structurally valid
  - but the journal contains no preceding `conversation.created`
  - startup therefore fails closed even though the file is syntactically valid

### 1.3 Medium: runtime-dir inspection tests had a parallel temp-dir collision risk

- the integration tests created temp runtime directories using only a timestamp-derived suffix
- under parallel test execution on Windows this can collide in practice
- the symptom is misleading test failure in otherwise healthy scenarios because two tests accidentally share one runtime-dir

## 2. Root Cause

The root cause was a mismatch between the inspection contract and the real recovery contract:

1. the durable runtime files were persisted behind typed storage abstractions
2. inspection validated them only as generic JSON values
3. `commit-journal.json` startup safety depended on replay semantics, not just syntax
4. tests initially encoded "generic JSON" as the healthy baseline, which no longer matched the stricter operator standard

The platform therefore had an inspection surface, but not yet a trustworthy semantic-validation surface.

## 3. Implementation

This review cycle tightened managed `local-minimal` inspection without changing the endpoint shape:

- added typed validation helpers in `im-adapters-local-disk` for all managed runtime-dir files
- kept `ops-service` response fields stable
- changed `local-minimal-node::inspect_runtime_dir(...)` to validate each file against its actual storage type instead of `serde_json::Value`
- added a journal-specific semantic validation path:
  1. typed-load `commit-journal.json` as `Vec<CommitEnvelope>`
  2. replay envelopes into a fresh `TimelineProjectionService`
  3. replay envelopes into a fresh `ConversationRuntime`
  4. mark the file `corrupt` when replay invariants fail
- stabilized `runtime_dir_inspection_test.rs` with an atomic temp-dir suffix so parallel execution does not contaminate state

## 4. Validation Model

This wave standardizes a stricter interpretation of inspection outcomes:

1. missing required file:
   - `status = missing`
   - `parseable = false`
2. malformed JSON or wrong typed top-level shape:
   - `status = corrupt`
   - `parseable = false`
3. typed parse succeeds but journal replay fails:
   - `status = corrupt`
   - `parseable = true`
4. typed parse and replay both succeed:
   - `status = ok`
   - `parseable = true`

This preserves the Standard 102 response shape while making the result operationally meaningful for restart-backed private deployment.

## 5. Regression Coverage

- `services/local-minimal-node/tests/runtime_dir_inspection_test.rs`
  - `test_managed_runtime_dir_inspection_reports_all_expected_files_when_parseable`
  - `test_managed_runtime_dir_inspection_reports_missing_and_corrupt_files_as_degraded`
  - `test_managed_runtime_dir_inspection_reports_typed_store_shape_violation_as_corrupt`
  - `test_managed_runtime_dir_inspection_reports_journal_replay_violation_as_corrupt`
- `services/ops-service/tests/http_smoke_test.rs`
  - `test_cluster_lag_health_runtime_dir_and_diagnostics_over_http`
- `services/local-minimal-node/tests/deployment_profile_test.rs`
  - deployment asset and script normalization coverage

## 6. Verification

Verified in this cycle with fresh command output:

- `cargo test -p local-minimal-node --offline --test runtime_dir_inspection_test -- --nocapture`
- `cargo test -p ops-service --offline --test http_smoke_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`

Broader verification then confirmed the full affected crates and operator entrypoints:

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p ops-service --offline`
- `cargo test -p local-minimal-node --offline`
- `powershell -ExecutionPolicy Bypass -File bin\\inspect-runtime-local.ps1 -RuntimeDir .runtime\\local-minimal -Json`

## 7. Standardized Outcome

Managed `local-minimal` runtime-dir inspection now answers the operator question more honestly:

- is the file present?
- is it valid for the real durable store type?
- if it is the journal, can it actually rebuild runtime state?

This closes the gap between "JSON exists on disk" and "restart will actually succeed".

## 8. Residual Risk

This wave intentionally does **not** yet provide:

- automatic repair or reseeding for corrupt files
- cross-file referential validation between non-journal runtime stores
- backup/restore orchestration for operator repair workflows
- cluster-wide drift detection across multiple managed nodes

## 9. Next Wave

The next review wave should target one of these:

1. safe repair workflows with backup-first operator tooling
2. cross-file semantic validation beyond journal replay
3. multi-node runtime-dir drift detection and failover reconstruction
