# Runtime Operations

Runtime directory management is a first-class operational surface in the current repository.
It does not replace the formal packaged `craw-chat-server` contract.

## Shell Wrappers

| Script | Purpose |
| --- | --- |
| `inspect-runtime-local.*` | Inspect managed runtime-directory state files |
| `repair-runtime-local.*` | Repair missing managed state files |
| `list-runtime-backups-local.*` | List backup catalog entries |
| `archive-runtime-backup-local.*` | Archive one backup snapshot while preserving restore metadata |
| `prune-runtime-archives-local.*` | Prune archived backups whose retention has elapsed and are not on legal hold |
| `preview-runtime-restore-local.*` | Preview restore actions without mutating the target runtime directory |
| `restore-runtime-local.*` | Execute restore, optionally guarded by `ExpectedPreviewFingerprint` |

## Underlying Binary Subcommands

The local binary itself exposes matching subcommands:

- `inspect-runtime-dir`
- `repair-runtime-dir`
- `list-runtime-backups`
- `archive-runtime-backup`
- `prune-archived-runtime-backups`
- `preview-runtime-restore`
- `restore-runtime-dir`

These are defined in `services/local-minimal-node/src/main.rs`.

## Managed State Files

The current runtime-management surface tracks these files under `state/`:

- `commit-journal.json`
- `realtime-disconnect-fences.json`
- `realtime-checkpoints.json`
- `realtime-subscriptions.json`
- `presence-state.json`
- `stream-state.json`
- `rtc-state.json`
- `notification-tasks.json`
- `automation-executions.json`
- `projection-metadata.json`
- `projection-timeline.json`

## Inspection Status Semantics

| Status | Meaning |
| --- | --- |
| `ok` | File exists and is parseable |
| `missing` | File is absent |
| `corrupt` | File exists but cannot be parsed or does not satisfy the expected typed shape |
| `degraded` | The runtime view contains one or more missing or corrupt managed files |

Recommended actions in the inspection output include values such as:

- `none`
- `recreate_on_next_managed_start_or_write`
- `manual_json_repair_or_restore`

## Backup Catalog Semantics

| Field | Meaning |
| --- | --- |
| `snapshot_quality` | `full_snapshot`, `partial_snapshot`, or `empty_snapshot` |
| `lifecycle_stage` | `active` or `archived` |
| `report_type` | `restore`, `repair`, or `archive` |
| `report_status` | Status values such as `restored`, `partial`, `repaired`, or `archived` |

## Preview And Restore Workflow

The intended workflow is:

1. list available backups
2. preview the restore against a selected backup
3. capture the `previewFingerprint`
4. execute restore with `-ExpectedPreviewFingerprint`

That guard prevents an operator from previewing one backup and accidentally restoring another.

## Typed Diff Coverage

The current preview flow provides typed diff summaries for at least these managed domains:

- realtime disconnect fences
- stream state
- RTC state
- projection snapshots

That makes restore preview more useful than a raw file-copy diff because it can summarize state
movement such as checkpoint advance, frame rewind, signal changes, or timestamp-only updates.

PostgreSQL is the frozen storage baseline and operators manage a config root in the packaged server
flow. The runtime-directory commands documented on this page remain diagnostic tooling for
development profiles only.

The local runtime restore commands remain diagnostic tooling for development profiles only.

## What To Read Next

- [Runtime Directory](/reference/runtime-directory)
- [Local Binary](/deployment/local-binary)
- [CLI and Scripts](/reference/cli-and-scripts)
