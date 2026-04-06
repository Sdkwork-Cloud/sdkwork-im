# 107. Local-Minimal Runtime-Dir Restore Preview Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide a supported, read-only restore preview workflow so operators can dry-run a chosen backup snapshot and inspect the exact restore actions before performing a destructive restore.

This standard closes the operator decision gap remaining after Standards 105 and 106: operators can now discover backups and assess snapshot quality, but they still need a direct preview of what an actual restore would do to the current runtime-dir state.

## 2. Scope

This standard applies to:

- `local-minimal-node` local restore preview entrypoints
- explicit operator-provided backup snapshot selection
- per-file restore action preview
- local lifecycle scripts under `bin/`

This standard does not add:

- restore execution
- filesystem mutation
- approval prompts or interactive confirmations
- remote HTTP preview APIs

## 3. Operator Entry Points

The local/private deployment toolchain must expose:

- `local-minimal-node preview-runtime-restore --backup-dir <path> [--runtime-dir <path>] [--json]`
- `bin/preview-runtime-restore-local.ps1`
- `bin/preview-runtime-restore-local.sh`
- `bin/preview-runtime-restore-local.cmd`

These entrypoints are local operator tools, not public application APIs.

## 4. Read-Only Rule

Restore preview must remain read-only.

Preview must never:

- create backup directories
- copy state files
- rewrite restore reports
- mutate current runtime state

This rule ensures preview is always safe to execute before an incident recovery decision.

## 5. Source Validation Rule

Preview must validate the selected source backup exactly as restore does:

- backup dir must exist
- `<backup-dir>/state/` must exist

If either is missing, preview must fail before touching the target runtime-dir state.

## 6. Preview Action Rule

Preview compares the selected backup snapshot against the current runtime-dir `state/` directory and must classify each managed file into one of these actions:

- `would_restore`
- `noop`
- `skip`

Action semantics:

- `would_restore`: the source file exists and the current target is missing or differs
- `noop`: source and target are byte-identical
- `skip`: the file does not exist in the selected backup snapshot

## 7. Preview Report Contract

The restore preview workflow must emit a structured report containing at minimum:

- overall preview `status`
- `runtimeDir`
- `sourceBackupDir`
- source snapshot quality and file counts
- source report type and source report status when present
- `before` inspection view of the current runtime-dir
- `wouldRestoreFileCount`
- `unchangedFileCount`
- `skippedFileCount`
- per-file preview actions

Recommended preview outcomes include:

- `ready`
- `partial`
- `noop`

## 8. Snapshot Summary Composition Rule

Restore preview must compose with the backup catalog summary from Standard 106.

That means preview should surface:

- source snapshot quality
- source managed file count
- source missing file count
- source report metadata when present

This keeps preview semantics consistent with the catalog workflow instead of inventing a second snapshot vocabulary.

## 9. Verification Standard

Regression coverage must prove:

1. a full snapshot preview reports `ready` and shows both `would_restore` and `noop` actions without mutating runtime state
2. a sparse snapshot preview reports `partial` and shows `skip` actions
3. invalid backup input fails with a controlled error and no mutation
4. lifecycle scripts exist across PowerShell, bash, and cmd entrypoints
5. status scripts reference restore preview as part of the supported operator loop

## 10. Composition Rule

This standard composes with:

- [105-local-minimal-runtime-dir-backup-restore-standard-2026-04-06.md](./105-local-minimal-runtime-dir-backup-restore-standard-2026-04-06.md)
- [106-local-minimal-runtime-dir-backup-catalog-standard-2026-04-06.md](./106-local-minimal-runtime-dir-backup-catalog-standard-2026-04-06.md)

The composed operator loop is now:

1. inspect current runtime-dir state
2. validate structural and semantic health
3. repair the missing-file case when safe
4. list available backups and preview snapshot quality
5. preview a chosen restore against the live runtime-dir
6. execute explicit restore only when the preview looks correct

## 11. Design Consequence

Restore preview is now a first-class operator seam instead of an implicit mental diff between:

- backup catalog output
- current runtime-dir state
- restore behavior

That lowers operator risk while keeping actual restore explicit and separately auditable.

Future work such as field-level diffs, apply confirmations, or remote authenticated preview APIs must build on this read-only preview contract instead of bypassing it.

Field-level diff summaries for eligible JSON object payloads are standardized separately in:

- [108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md](./108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md)

Preview-to-restore confirmation handoff is standardized separately in:

- [111-local-minimal-runtime-dir-restore-preview-confirmation-standard-2026-04-06.md](./111-local-minimal-runtime-dir-restore-preview-confirmation-standard-2026-04-06.md)
