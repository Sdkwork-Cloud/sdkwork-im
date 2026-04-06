# 106. Local-Minimal Runtime-Dir Backup Catalog Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide a supported, read-only backup catalog workflow so operators can list runtime-dir snapshots and preview whether a selected backup is empty, partial, or fully restorable before executing restore.

This standard closes the operator visibility gap discovered after Standard 105: explicit restore existed, but operators still needed lightweight backup quality preview to avoid choosing sparse snapshots blindly.

## 2. Scope

This standard applies to:

- `local-minimal-node` local backup catalog entrypoints
- backup snapshot listing under `<runtime-dir>/backups`
- report metadata preview from existing backup reports
- local lifecycle scripts under `bin/`

This standard does not add:

- backup mutation
- automatic restore selection
- file diff preview
- remote HTTP catalog APIs

## 3. Operator Entry Points

The local/private deployment toolchain must expose:

- `local-minimal-node list-runtime-backups [--runtime-dir <path>] [--json]`
- `bin/list-runtime-backups-local.ps1`
- `bin/list-runtime-backups-local.sh`
- `bin/list-runtime-backups-local.cmd`

These entrypoints are local operator tools, not public application APIs.

## 4. Read-Only Rule

Backup cataloging must remain read-only.

Listing a backup snapshot must never:

- create new directories
- rewrite reports
- copy state files
- select a restore target automatically

This rule keeps cataloging safe to run during diagnosis and incident response.

## 5. Backup Discovery Rule

Cataloging reads candidate snapshots from:

```text
<runtime-dir>/backups/
```

Behavior rules:

- only directories are cataloged as backup snapshots
- a missing `backups/` directory must return an empty catalog, not an error
- entries should be presented newest-first using deterministic name ordering

## 6. Preview Contract

Each backup snapshot preview must include at minimum:

- backup directory name
- full backup directory path
- inferred operation type
- whether `state/` exists
- count of managed files present
- count of managed files absent
- report type when present
- report status when present

The preview is intentionally lightweight. It is not a full diff viewer.

## 7. Snapshot Quality Rule

The catalog must classify snapshot quality from managed file presence:

- `empty_snapshot` when zero managed files are present
- `partial_snapshot` when some but not all managed files are present
- `full_snapshot` when all managed files are present

This rule gives operators a direct signal about restore readiness without reading raw filesystem state manually.

## 8. Report Preview Rule

When present, the catalog must preview metadata from:

- `repair-report.json`
- `restore-report.json`

At minimum, the preview must surface:

- report type
- report `status`

If the report is missing or unreadable, the catalog may omit that metadata but must still list the backup directory.

## 9. Verification Standard

Regression coverage must prove:

1. empty, partial, and full snapshots are all classified correctly
2. report metadata is previewed when report files exist
3. a missing `backups/` directory returns an empty catalog without mutation
4. lifecycle scripts exist across PowerShell, bash, and cmd entrypoints
5. status scripts reference backup listing as part of the supported operator loop

## 10. Composition Rule

This standard composes with:

- [104-local-minimal-runtime-dir-safe-repair-standard-2026-04-06.md](./104-local-minimal-runtime-dir-safe-repair-standard-2026-04-06.md)
- [105-local-minimal-runtime-dir-backup-restore-standard-2026-04-06.md](./105-local-minimal-runtime-dir-backup-restore-standard-2026-04-06.md)

The composed operator loop is now:

1. inspect current runtime-dir state
2. validate structural and semantic health
3. repair the missing-file case when safe
4. list available backups and preview snapshot quality
5. restore an explicitly chosen snapshot when rollback is required

## 11. Design Consequence

Backup listing is now a first-class operator seam instead of an ad hoc filesystem inspection step.

That keeps restore explicit while still giving operators enough signal to avoid choosing a clearly empty or sparse snapshot by accident.

Restore dry-run preview is now standardized separately in:

- [107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md](./107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md)

Future work such as file-level diffs beyond preview summary or remote authenticated catalog APIs must build on this read-only backup catalog contract instead of bypassing it.
