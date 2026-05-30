# 102. Local-Minimal Runtime-Dir Inspection Repair Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must expose a standardized operator-facing inspection surface for runtime-dir persistence health.

This standard freezes the first safe inspection/repair boundary for restart-backed local deployment:

- inspect
- classify
- recommend next action

This wave does **not** freeze automatic mutation or auto-repair behavior.

## 2. Scope

This standard applies to:

- `local-minimal-node` managed runtime-dir builders
- `ops-service` runtime-dir inspection contract
- local lifecycle inspection scripts under `bin/`
- runtime-dir state file visibility for private deployment

This standard does not redefine SaaS production storage choices.

## 3. Required Runtime Layout

Managed local-minimal runtime directories must include:

- `.runtime/local-minimal/config`
- `.runtime/local-minimal/logs`
- `.runtime/local-minimal/pids`
- `.runtime/local-minimal/state`

The managed inspection set is:

```text
<runtime-dir>/state/commit-journal.json
<runtime-dir>/state/realtime-disconnect-fences.json
<runtime-dir>/state/realtime-checkpoints.json
<runtime-dir>/state/realtime-subscriptions.json
<runtime-dir>/state/presence-state.json
<runtime-dir>/state/stream-state.json
<runtime-dir>/state/rtc-state.json
<runtime-dir>/state/notification-tasks.json
<runtime-dir>/state/automation-executions.json
```

## 4. Inspection Contract

The platform must expose a read-only inspection model containing:

- overall `status`
- `runtimeDir`
- `stateDir`
- `healthyFileCount`
- `missingFileCount`
- `corruptFileCount`
- per-file inspection records

Each per-file inspection item must expose at minimum:

- `fileName`
- `path`
- `required`
- `exists`
- `parseable`
- `status`
- `sizeBytes`
- `parseError`
- `recommendedAction`

## 5. Status Model

The overall inspection status must use:

- `unmanaged`
- `ok`
- `degraded`

Per-file status must use:

- `ok`
- `missing`
- `corrupt`

The aggregate rules are:

1. unmanaged profile with no runtime-dir contract -> `unmanaged`
2. managed profile with zero missing and zero corrupt files -> `ok`
3. managed profile with one or more missing or corrupt files -> `degraded`

## 6. Recommended Action Model

This standard freezes the first action vocabulary:

- `none`
- `recreate_on_next_managed_start_or_write`
- `manual_json_repair_or_restore`

Rules:

1. `ok` file -> `none`
2. `missing` file -> `recreate_on_next_managed_start_or_write`
3. `corrupt` file -> `manual_json_repair_or_restore`

The platform must not silently rewrite files during inspection.

## 7. API Contract

Authorized ops clients must be able to query:

```text
GET /backend/v3/api/ops/runtime_dir
```

Authorization remains:

- `ops.read`

The endpoint must return the runtime-dir inspection view defined by this standard.

## 8. Local Operator Entry Points

The managed local/private deployment toolchain must expose:

- `local-minimal-node inspect-runtime-dir --runtime-dir <path> [--json]`
- `bin/inspect-runtime-local.ps1`
- `bin/inspect-runtime-local.sh`
- `bin/inspect-runtime-local.cmd`

`status-local.*` scripts may remain lightweight, but they must point operators at the dedicated runtime-dir inspection scripts.

## 9. Failure Semantics

Inspection failures must never mutate business state.

The platform must:

- fail closed on malformed JSON by reporting `corrupt`
- fail closed on absent required files by reporting `missing`
- keep the inspection surface deterministic even when the node is otherwise healthy

This standard intentionally does not require semantic auto-repair or silent reseeding of corrupt files.

## 10. Verification Standard

Regression coverage must prove:

1. `ops-service` exposes `/backend/v3/api/ops/runtime_dir`
2. unmanaged default ops runtime reports `unmanaged`
3. managed local-minimal runtime-dir inspection reports `ok` when all required files are parseable
4. managed local-minimal runtime-dir inspection reports `degraded` when files are missing or corrupt
5. local lifecycle assets include `inspect-runtime-local.*`
6. status scripts reference the runtime-dir inspection step

## 11. Composition Rule

This standard composes with:

- [95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md](./95-local-minimal-domain-journal-replay-recovery-standard-2026-04-06.md)
- [96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md](./96-local-minimal-live-subscription-bootstrap-recovery-standard-2026-04-06.md)
- [97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md](./97-local-minimal-stream-runtime-persistence-standard-2026-04-06.md)
- [98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md](./98-local-minimal-rtc-runtime-persistence-standard-2026-04-06.md)
- [99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md](./99-local-minimal-notification-runtime-persistence-standard-2026-04-06.md)
- [100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md](./100-local-minimal-automation-runtime-persistence-standard-2026-04-06.md)
- [101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md](./101-local-minimal-presence-runtime-persistence-standard-2026-04-06.md)

The composition outcome is:

- restart-backed runtime state remains durable
- operators can inspect the full managed runtime-dir set coherently
- missing or malformed state files become visible before silent operational drift

## 12. Design Consequence

Runtime-dir inspection remains a replaceable operations seam instead of being hard-coded into one shell script, one storage vendor, or one deployment mode.

Future work may add safe repair workflows, but those workflows must build on the inspection contract defined here instead of bypassing it.

## 13. Evolution Note

This standard freezes the inspection surface and first-wave classification model.

Stronger typed and replay-aware semantic validation is standardized separately by:

- [103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md](./103-local-minimal-runtime-dir-semantic-validation-standard-2026-04-06.md)
