# Admin SDK

The admin SDK family is documented here for completeness, but it is not the primary onboarding path
for app-facing chat integrations. If you are building product or user-facing chat features, start
with [App SDK](/sdk/app-sdk) instead.

## Audience

Use the admin family for:

- control-plane reads
- protocol registry and governance workflows
- provider registry and policy governance
- node lifecycle management

Do not use it for:

- end-user session handling
- conversation runtime flows
- inbox, messages, media, streams, or RTC product integration

## Workspace Layout

- root workspace: `sdks/sdkwork-craw-chat-sdk-admin`
- language workspaces:
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript`
  - `sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter`

## Current Source Of Truth

Unlike the app SDK family, the admin SDK workspace does not currently publish a checked-in OpenAPI
authority file under `sdks/sdkwork-craw-chat-sdk-admin/`.

The implementation-aligned truth source remains:

- `services/control-plane-api/src/lib.rs`
- control-plane tests such as:
  - `protocol_registry_test.rs`
  - `protocol_governance_test.rs`
  - `provider_registry_test.rs`
  - `drain_routes_test.rs`

## Consumer Rules

- governance consumers should treat control-plane snapshots as the decision source
- admin integrations should not mix app-facing SDK assumptions into this family
- app API documentation and app SDK documentation remain separate from this control-plane surface

## Publication Boundary

These workspaces exist locally, but this page does not claim public package publication or a frozen
public version line. Keep admin SDK expectations scoped to local workspace and control-plane
documentation reality.

## Read Next

- [Control Plane API Overview](/api-reference/control-plane-api)
- [App SDK](/sdk/app-sdk)
- [Language Support](/sdk/language-support)
