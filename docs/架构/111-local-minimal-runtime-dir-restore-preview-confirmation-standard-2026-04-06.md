# 111. Local-Minimal Runtime-Dir Restore Preview Confirmation Standard (2026-04-06)

## 1. Goal

Managed `local-minimal` private deployment must provide an explicit preview-to-restore confirmation seam so operators can require restore execution to match a previously reviewed restore preview.

This standard closes the remaining operator safety gap after Standards 107 through 110: preview can already explain restore impact in increasing detail, but restore still needs an explicit handoff proving the operator is restoring the state they actually reviewed.

## 2. Scope

This standard applies to:

- restore preview fingerprint generation
- restore-time optional fingerprint confirmation
- local CLI and script entrypoints for preview and restore
- local operator guidance in status scripts

This standard does not add:

- interactive prompts
- mandatory restore approval workflows
- remote orchestration APIs
- restore policy automation

## 3. Composition Rule

This standard composes with:

- [107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md](./107-local-minimal-runtime-dir-restore-preview-standard-2026-04-06.md)
- [108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md](./108-local-minimal-runtime-dir-restore-preview-diff-standard-2026-04-06.md)
- [109-local-minimal-runtime-dir-disconnect-fence-typed-diff-standard-2026-04-06.md](./109-local-minimal-runtime-dir-disconnect-fence-typed-diff-standard-2026-04-06.md)
- [110-local-minimal-runtime-dir-checkpoint-typed-diff-standard-2026-04-06.md](./110-local-minimal-runtime-dir-checkpoint-typed-diff-standard-2026-04-06.md)

The composed operator loop is:

1. inspect current runtime-dir state
2. list candidate backups
3. preview a chosen backup
4. review preview details and capture `previewFingerprint`
5. execute restore with that expected fingerprint when desired

## 4. Preview Fingerprint Rule

Restore preview must emit a deterministic `previewFingerprint` field.

The fingerprint must:

- be derived from preview material excluding the fingerprint field itself
- remain stable across repeated preview calls against unchanged inputs
- change when restore-relevant preview material changes

## 5. Restore Confirmation Rule

Restore must accept an optional expected preview fingerprint parameter.

When the parameter is supplied:

- restore must recompute current preview state
- restore must compare the current preview fingerprint to the expected fingerprint
- restore must fail if they differ

When the parameter is omitted:

- restore may continue to behave exactly as the existing explicit restore seam

This keeps backward compatibility while enabling a safer operator path.

## 6. Zero-Side-Effect Mismatch Rule

If restore fingerprint confirmation fails, restore must fail before:

- creating pre-restore backup directories
- copying any runtime state files
- mutating current runtime state
- writing restore reports for the aborted restore

This rule is the core safety guarantee of the confirmation seam.

## 7. Restore Report Contract

Successful restore reports may expose:

- `confirmedPreviewFingerprint`

When present, it records the preview fingerprint that was matched before restore execution began.

## 8. Local Entry Point Rule

The following local entrypoints must support the confirmation seam:

- `local-minimal-node preview-runtime-restore --backup-dir <path> [--runtime-dir <path>] [--json]`
- `local-minimal-node restore-runtime-dir --backup-dir <path> [--runtime-dir <path>] [--expected-preview-fingerprint <value>] [--json]`
- `bin/restore-runtime-local.ps1`
- `bin/restore-runtime-local.sh`

Status and operator guidance scripts must explain the preferred flow:

- preview
- capture fingerprint
- restore with expected fingerprint

## 9. Determinism Rule

Preview fingerprints must be deterministic and reproducible across repeated local executions against unchanged inputs.

This rule is required so operators can safely pass a reviewed fingerprint from preview to restore.

## 10. Verification Standard

Regression coverage must prove:

1. preview emits a non-empty stable fingerprint
2. restore succeeds when the provided fingerprint matches the current preview
3. restore fails with a controlled error when the fingerprint mismatches
4. mismatch failure causes no pre-restore backup creation and no runtime mutation
5. local restore scripts and status guidance expose the expected fingerprint parameter

## 11. Design Consequence

Restore is still explicit and operator-driven, but it is no longer blind.

The confirmation seam adds a lightweight safety handshake between decision and action without collapsing preview into an interactive workflow or centralized control plane.
