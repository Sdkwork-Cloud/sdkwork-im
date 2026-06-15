# Docs

## Purpose

Repository and application documentation, architecture decisions, runbooks, changelogs, design
notes, and developer guides for Craw Chat.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Architecture decisions, runbooks, changelogs, design documents, implementation plans, and
  user/developer guides.
- Documentation fixtures and examples that do not contain secrets.

## Forbidden Content

- Runtime data, generated SDK transport output, logs, caches, credentials, secrets, or private
  customer data.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/DOCUMENTATION_SPEC.md`
- `../sdkwork-specs/ARCHITECTURE_DECISION_SPEC.md`

## Verification

Run documentation-specific checks when available and
`pnpm run test:sdkwork-workspace-structure-standard` after root layout changes.
