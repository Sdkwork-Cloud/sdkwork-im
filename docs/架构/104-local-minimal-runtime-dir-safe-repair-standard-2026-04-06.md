# 104. Local-Minimal Runtime-Dir Safe Repair Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide a supported, backup-first repair workflow for the low-risk case where required runtime-dir files are missing.

This standard freezes the first mutating operator workflow that composes with Standards 102 and 103.

## 2. Scope

This standard applies to:

- `local-minimal-node` local repair entrypoints
- backup-first recreation of missing runtime-dir state files
- repair report generation
- local lifecycle scripts under `bin/`

This standard does not add remote HTTP repair endpoints and does not auto-repair corrupt files.

## 3. Repair Boundary

This wave may repair only files classified by inspection as:

- `missing`

This wave must not automatically rewrite files classified as:

- `corrupt`

The rationale is strict:

- `missing` means there is no ambiguous on-disk business data to preserve in place
- `corrupt` may still contain operator-relevant evidence or partially salvageable state

## 4. Operator Entry Points

The local/private deployment toolchain must expose:

- `local-minimal-node repair-runtime-dir --runtime-dir <path> [--json]`
- `bin/repair-runtime-local.ps1`
- `bin/repair-runtime-local.sh`
- `bin/repair-runtime-local.cmd`

These entrypoints are local-operator tools, not public application APIs.

## 5. Backup Rule

Before any mutation, repair must create a timestamped backup directory under:

```text
<runtime-dir>/backups/runtime-dir-repair-<timestamp>/
```

At minimum, the backup must contain:

- `state/` snapshot of all existing managed runtime-dir files
- `repair-report.json`

The backup must be created even when the actual repair target is only missing-file recreation.

## 6. Missing-File Recreation Rule

Repair must recreate missing managed files with typed-empty content:

- `commit-journal.json` -> `[]`
- all other managed state files -> `{}`

This rule matches the durable type contracts frozen by Standards 95-103.

## 7. Corrupt-File Rule

If inspection reports a file as `corrupt`, repair must:

- leave the file unchanged
- mark the file as skipped in the repair report
- preserve manual operator intervention as the next action

This wave must not silently coerce or overwrite corrupt JSON.

## 8. Repair Report Contract

The repair workflow must emit a structured report containing at minimum:

- overall repair `status`
- `runtimeDir`
- `backupDir`
- `repairedFileCount`
- `skippedFileCount`
- `before` inspection view
- `after` inspection view
- per-file repair actions

Recommended repair outcomes include:

- `repaired`
- `partial`
- `noop`

## 9. Verification Standard

Regression coverage must prove:

1. a fully missing managed runtime-dir can be repaired to an `ok` post-repair inspection
2. the repair workflow writes a backup directory and repair report
3. corrupt files are left untouched while missing files are recreated
4. repair scripts exist across PowerShell, bash, and cmd entrypoints

## 10. Composition Rule

This standard composes with:

- [102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md](./102-local-minimal-runtime-dir-inspection-repair-standard-2026-04-06.md)
- [103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md](./103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md)

The composition outcome is:

1. inspect without mutation
2. validate against real durable semantics
3. perform explicit low-risk local repair where allowed

## 11. Design Consequence

Safe repair remains a separate operator seam instead of being hidden inside inspection, startup, or application request handlers.

Future backup restore, corrupt-file remediation, or remote repair orchestration must build on this explicit repair contract instead of bypassing it.

Backup restore is now standardized separately in:

- [105-local-minimal-runtime-dir-backup-restore-standard-2026-04-06.md](./105-local-minimal-runtime-dir-backup-restore-standard-2026-04-06.md)
