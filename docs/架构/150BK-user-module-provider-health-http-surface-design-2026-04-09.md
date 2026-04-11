# 150BK User-Module Provider Health HTTP Surface Design

## Problem

`user-module` had provider selection and unavailable semantics, but no operator-facing health route. That made it less observable than RTC, media, and IoT.

## Decision

- Add `GET /api/v1/user-module/provider-health`.
- Authenticate it the same way as other provider-health routes.
- Return `ProviderHealthSnapshot` directly.
- Keep HTTP `200`; express availability in `status`.
- Include minimal details:
  - local: `providerKind=local`
  - external healthy: `providerKind=external`, `catalogPath`, `defaultExternalSystem`
  - external unavailable: preserve error/config details and add `providerKind`

## Rationale

- Operators need a direct path to inspect selected provider state and config failures.
- Reusing the existing provider-health contract keeps the surface small and consistent.

## Boundary

- This only adds the local-node user-module provider health route.
- It does not yet add unified selected-provider or provider-registry views to ops/control-plane APIs.
