# Admin SDK

## Workspace Layout

- Root workspace: `sdks/sdkwork-craw-chat-sdk-admin`
- Language workspaces:
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript`
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter`

## Scope

The admin SDK family is intended for:

- control-plane read surfaces
- protocol registry and governance consumption
- provider registry and provider-policy governance
- node lifecycle management

It is not intended for:

- app-facing chat or conversation facades
- `chat-session`, `send-message`, or `timeline` style product flows
- replacing the local verification role of `tools/chat-cli`

## Current Source Of Truth

Unlike the app SDK family, the admin SDK workspace does not currently contain a checked-in OpenAPI
authority file under `sdks/sdkwork-craw-chat-sdk-admin/`.

The implementation-aligned source of truth is therefore:

- `services/control-plane-api/src/lib.rs`
- control-plane tests such as:
  - `services/control-plane-api/tests/protocol_registry_test.rs`
  - `services/control-plane-api/tests/protocol_governance_test.rs`
  - `services/control-plane-api/tests/provider_registry_test.rs`
  - `services/control-plane-api/tests/drain_routes_test.rs`

The VitePress API reference for the control plane is written directly against that source.

## Consumer Rules

The admin SDK boundary is already meaningful even before package publication:

- governance consumers should treat control-plane snapshots as the source of truth
- client behavior should not reconstruct protocol compatibility locally when the control plane
  already publishes the decision surface
- admin integrations should not mix app-facing chat features into this SDK family

## Current Release Status

| Artifact | Language | Generation state | Release state |
| --- | --- | --- | --- |
| `sdkwork-craw-chat-sdk-admin-typescript` | TypeScript | `template_only_pending_generation` | `not_published` |
| `sdkwork-craw-chat-sdk-admin-flutter` | Flutter | `template_only_pending_generation` | `not_published` |

All admin artifacts are also still:

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

That means the audience and workspace boundaries are real, but the release wave is not yet a
published package line.
