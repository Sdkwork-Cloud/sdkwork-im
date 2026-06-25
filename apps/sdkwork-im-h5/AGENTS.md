# SDKWork IM H5 Application

## Entry Point

This is the H5 mobile application root for SDKWork IM. See [../../AGENTS.md](../../AGENTS.md) for repository-level agent instructions.

## SDKWork Specs

- `../../../sdkwork-specs/README.md`
- `../../../sdkwork-specs/SOUL.md`
- `../../../sdkwork-specs/APP_H5_ARCHITECTURE_SPEC.md`
- `../../../sdkwork-specs/APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`
- `../../../sdkwork-specs/APP_SDK_INTEGRATION_SPEC.md`
- `../../../sdkwork-specs/CONFIG_SPEC.md`

## Application Identity

- App ID: `sdkwork-im-h5`
- Runtime family: `h5`
- Framework: `react`
- Surface: `mobile-browser`
- Dev URL: `http://127.0.0.1:3010`

## Build And Verify

```powershell
pnpm install
pnpm run lint
pnpm run build
pnpm run test:sdkwork-im-h5-architecture-standard
```
