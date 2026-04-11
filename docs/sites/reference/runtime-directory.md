# Runtime Directory

The runtime directory is a core part of the current `local-minimal-node` contract.

## Default Structure

When local config is initialized, the managed runtime directory contains:

- `config/`
- `logs/`
- `pids/`
- `state/`

The default path is `.runtime/local-minimal` unless a different `CRAW_CHAT_RUNTIME_DIR` is
configured.

## Managed State Files

| File | Purpose |
| --- | --- |
| `commit-journal.json` | Event journal and replay source |
| `realtime-disconnect-fences.json` | Disconnect fence state |
| `realtime-checkpoints.json` | Realtime delivery checkpoints |
| `realtime-subscriptions.json` | Realtime subscription snapshots |
| `presence-state.json` | Presence state |
| `stream-state.json` | Stream session and frame state |
| `rtc-state.json` | RTC session and signal state |
| `notification-tasks.json` | Notification tasks |
| `automation-executions.json` | Automation executions |
| `projection-metadata.json` | Projection metadata |
| `projection-timeline.json` | Projection timeline snapshot |

## Inspection Output

Current runtime inspection aggregates:

- `healthyFileCount`
- `missingFileCount`
- `corruptFileCount`
- per-file status, parseability, parse error, and recommended action

Common recommended actions include:

- `none`
- `recreate_on_next_managed_start_or_write`
- `manual_json_repair_or_restore`

## Backup, Archive, and Restore Contract

### Backup catalog

Backups are tracked under the runtime `backups/` directory and surfaced through a managed catalog.

### Archive

Archive keeps the restore path and writes archive metadata and reports rather than simply deleting
the backup from the active view.

### Restore preview

Preview is read-only. It exists to:

- compare a backup against the target runtime directory
- generate a `previewFingerprint`
- describe restore actions before mutation

## Why `expectedPreviewFingerprint` Matters

`restore-runtime-dir` supports:

```text
--expected-preview-fingerprint <value>
```

This guard protects operators from previewing one snapshot and restoring a different one by mistake.

## Typed Diff Coverage

The current preview implementation includes typed diff summaries for:

- disconnect fences
- stream state
- RTC state
- projection snapshots

That gives operators meaningful domain-level insight instead of a plain file-by-file diff.

## Operational Rule

Use the managed inspect, repair, preview, and restore entrypoints first. Manual edits to
`state/*.json` should be the exception, not the default workflow.
