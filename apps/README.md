# Apps

## Purpose

SDKWork application roots hosted by the Sdkwork IM repository. The current PC browser and desktop
application lives at `apps/sdkwork-im-pc`.

## Owner

SDKWork Chat maintainers.

## Allowed Content

- Independent application roots with `sdkwork.app.config.json`, `AGENTS.md`, `.sdkwork/`, `specs/`,
  packages, scripts, and app-local verification.
- PC React renderer bootstrap, console/admin package families, and desktop host packages under the
  owning application root.

## Forbidden Content

- Repository-root Rust services, crates, or generated SDK output.
- Runtime databases, logs, caches, secrets, or user-private files.
- Cross-application business logic copied from other SDKWork products without an explicit contract.

## Related Specs

- `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- `../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`
- `../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md`
- `../sdkwork-specs/TEST_SPEC.md`

## Verification

Run `pnpm run test:sdkwork-workspace-structure-standard` from the repository root after application
layout changes. Run the owning application root checks from `apps/sdkwork-im-pc` when PC packages,
manifests, or SDK wiring change.
