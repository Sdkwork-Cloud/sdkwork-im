# 105. Local-Minimal Runtime-Dir Backup Restore Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide a supported restore workflow that lets operators explicitly restore managed runtime-dir state from a chosen local backup snapshot.

This standard extends the runtime-dir operator lifecycle from:

1. inspect
2. validate
3. repair missing files

to:

4. restore a selected snapshot when current state must be replaced

## 2. Scope

This standard applies to:

- `local-minimal-node` local restore entrypoints
- explicit operator-provided backup snapshot selection
- restore-before-overwrite safety backup creation
- structured restore reporting
- local lifecycle scripts under `bin/`

This standard does not add:

- public HTTP restore APIs
- automatic backup selection heuristics
- merge/conflict resolution across multiple snapshots
- automatic corrupt-file remediation

## 3. Operator Entry Points

The local/private deployment toolchain must expose:

- `local-minimal-node restore-runtime-dir --backup-dir <path> [--runtime-dir <path>] [--json]`
- `bin/restore-runtime-local.ps1`
- `bin/restore-runtime-local.sh`
- `bin/restore-runtime-local.cmd`

These entrypoints are local operator tools, not public application APIs.

## 4. Explicit Backup Selection Rule

Restore must require an explicit source backup directory:

```text
--backup-dir <path>
```

The system must not silently pick a snapshot on behalf of the operator in this wave.

The rationale is operational safety:

- restore is a destructive overwrite workflow
- operators must control which snapshot becomes authoritative
- implicit snapshot selection would hide recovery intent and complicate auditing

## 5. Source Snapshot Contract

The selected source backup directory must contain:

```text
<backup-dir>/state/
```

Only managed state files under that `state/` directory participate in restore.

If the source backup directory or its `state/` directory is missing, restore must fail before mutating the target runtime directory.

## 6. Restore-Before-Overwrite Rule

Before copying any file from the selected backup snapshot, restore must create a timestamped pre-restore safety backup under:

```text
<runtime-dir>/backups/runtime-dir-restore-<timestamp>/
```

At minimum, that directory must contain:

- `state/` snapshot of existing managed runtime-dir files
- `restore-report.json`

This preserves the immediately previous on-disk state in case the operator needs to roll back the restore action itself.

## 7. File Restore Rule

Restore copies managed files from:

```text
<backup-dir>/state/
```

into:

```text
<runtime-dir>/state/
```

Behavior rules:

- if the managed file exists in the selected backup snapshot, restore must overwrite the target file with that snapshot version
- if the managed file does not exist in the selected backup snapshot, restore must not invent replacement data for that file
- missing source files must be reported as `skipped`

This wave remains file-level and deterministic. It does not perform semantic merges.

## 8. Restore Report Contract

The restore workflow must emit a structured report containing at minimum:

- overall restore `status`
- `runtimeDir`
- `sourceBackupDir`
- `preRestoreBackupDir`
- `restoredFileCount`
- `skippedFileCount`
- `before` inspection view
- `after` inspection view
- per-file restore actions

Recommended restore outcomes include:

- `restored`
- `partial`
- `noop`

## 9. Verification Standard

Regression coverage must prove:

1. restoring a chosen snapshot overwrites current managed state from the selected backup
2. the current state is snapshotted before overwrite into a pre-restore backup directory
3. missing backup inputs fail with a controlled error and no mutation
4. restore lifecycle scripts exist across PowerShell, bash, and cmd entrypoints
5. status scripts reference inspection, repair, and restore as the supported operator loop

## 10. Composition Rule

This standard composes with:

- [102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md](./102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md)
- [103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md](./103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md)
- [104-local-minimal-runtime-dir-safe-repair-standard-2026-04-06.md](./104-local-minimal-runtime-dir-safe-repair-standard-2026-04-06.md)

The composed operator loop is now:

1. inspect current runtime-dir state
2. validate whether state is structurally and semantically usable
3. repair only low-risk missing-file cases when appropriate
4. restore an explicit snapshot when state must be reverted wholesale

## 11. Design Consequence

Restore remains a separate operator seam instead of being hidden inside startup, health checks, or request handling.

That keeps the runtime fail-closed:

- inspection stays read-only
- repair stays narrowly scoped
- restore stays explicit and operator-directed

Future work such as diff preview, guided corrupt-file recovery, or remote authenticated restore orchestration must build on this explicit restore contract instead of bypassing it.

Backup cataloging and readiness preview are now standardized separately in:

- [106-local-minimal-runtime-dir-backup-catalog-standard-2026-04-06.md](./106-local-minimal-runtime-dir-backup-catalog-standard-2026-04-06.md)
- [107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md](./107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md)
