# Local-Minimal Ops Provider Bindings Runtime Visibility Design

## Problem

`local-minimal-node` already promises `providerBindings` inside ops diagnostics, but the field stayed empty because runtime-selected provider bindings were never mirrored into `OpsRuntime`.

## Scope

- In scope:
  - global runtime provider binding visibility for local-minimal
  - RTC binding
  - object-storage binding
  - user-module binding
  - IoT access binding
  - IoT protocol binding
- Out of scope:
  - control-plane policy writes
  - tenant override surfaces for local-minimal
  - standalone new ops APIs

## Design

- Keep `DiagnosticBundle.providerBindings` as the only new consumption point for this loop.
- Assemble one global `ProviderBindingSnapshotView` with:
  - `interfaceVersion=provider-registry/v1`
  - `tenantId=null`
  - precedence:
    - `tenant_override`
    - `deployment_profile`
    - `global_default`
- Source of truth by domain:
  - `rtc`: `RtcRuntime` provider registry
  - `object-storage`: `MediaRuntime` provider registry
  - `user-module`: active provider descriptor over platform-registry baseline
  - `iot-access`: active provider descriptor over platform-registry baseline
  - `iot-protocol`: active provider descriptor over platform-registry baseline

## Selection Rules

- If the active descriptor is the platform default, expose `selectionSource=global_default`.
- If the active descriptor differs from the platform default, expose `selectionSource=deployment_profile`.
- Preserve platform baseline `defaultPluginId` where it exists.

## Risk

- Local-minimal still exposes only a global snapshot, so tenant-scoped binding drift remains a future control-plane concern.
- RTC recording storage and media storage are assumed aligned unless a later loop adds explicit divergence checks.

## Acceptance

- `GET /api/v1/ops/diagnostics` returns one global `providerBindings` snapshot.
- The snapshot exposes the selected plugins for all five provider domains.
- Existing local-minimal tests remain green.
