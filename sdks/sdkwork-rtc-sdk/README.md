# SDKWork RTC SDK

`sdkwork-rtc-sdk` is the RTC provider-standard SDK family for Sdkwork IM. It is
not a route-generated SDK workspace and does not own an OpenAPI route prefix.

The authoritative implementation workspace currently lives at
`../sdkwork-rtc/sdks/sdkwork-rtc-sdk`. This in-repository README is the Sdkwork IM
boundary index used by release, CLI, and SDK documentation contracts.

## Boundary

- Audience: RTC provider runtime integrations.
- SDK role: provider-standard RTC runtime.
- Package model: provider-neutral contracts plus provider adapters.
- Route generation: none; this SDK is not generated from `/im/v3/api`,
  `/app/v3/api`, or `/backend/v3/api`.
- Consumers compose this SDK beside `sdkwork-im-sdk`,
  `sdkwork-im-app-sdk`, and `sdkwork-im-backend-sdk`.

## Release Snapshot Boundary

This workspace inherits the current SDK release snapshot from
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `state = template_only_pending_generation`
- `generationStatus = template_only_pending_generation`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

The release catalog remains the machine-readable source of truth:
`sdk-release-catalog.json`.
