# Admin SDK

The admin SDK family covers governance and control-plane consumers. Use it for protocol registry
and policy reads, provider governance, and node lifecycle operations.

## Choose This Family When

- you are reading or applying protocol registry decisions
- you are integrating provider governance or provider-policy workflows
- you are automating node lifecycle operations such as drain, activate, and route migration
- you are building an admin or control-plane consumer rather than an app-runtime client

## Do Not Use The Admin SDK For

- app-runtime chat flows
- conversation, timeline, media, or RTC end-user product surfaces
- message-first send or live receive ergonomics
- portal snapshot or end-user session integration

Use the App SDK instead when your consumer boundary is app-runtime chat flows rather than
governance or control-plane work.

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
- node drain, activate, and route-migration operations

It is not intended for:

- app-facing chat or conversation facades
- `chat-session`, `send-message`, `timeline`, media, or RTC product flows
- replacing the local verification role of `tools/chat-cli`

## What This Site Can Verify Today

Unlike the app SDK family, the admin SDK workspace does not yet contain a checked-in OpenAPI
authority file under `sdks/sdkwork-craw-chat-sdk-admin/`.

The implementation-aligned source of truth is therefore:

- `services/control-plane-api/src/lib.rs`
- control-plane tests such as:
  - `services/control-plane-api/tests/protocol_registry_test.rs`
  - `services/control-plane-api/tests/protocol_governance_test.rs`
  - `services/control-plane-api/tests/provider_registry_test.rs`
  - `services/control-plane-api/tests/drain_routes_test.rs`

The VitePress control-plane reference is written directly against those sources:

- [Control Plane API Overview](/api-reference/control-plane-api)
- [Protocol Governance](/api-reference/control-plane/protocol)
- [Provider Governance](/api-reference/control-plane/providers)
- [Node Operations](/api-reference/control-plane/nodes)

## Why The Admin Docs Stay Conservative

This site intentionally stops short of publishing polished language-specific usage examples for the
admin family because the repo does not yet provide the same validation anchors that the app family
does:

- there is no checked-in admin OpenAPI authority file
- there are no site-verified generated package manifests equivalent to the app family's consumer
  packages
- the release catalog still marks the family as `template_only_pending_generation` and
  `not_published`

That means the audience boundary is real, but the consumer import surface is not yet documented
here as a stable contract.

## How To Use This Page

Use the admin SDK docs in this order:

1. Confirm that your integration is actually a governance or control-plane consumer, not an app-runtime consumer.
2. Read the control-plane HTTP reference for the concrete route semantics and payloads.
3. Use this page only to confirm workspace boundary, source-of-truth files, and release-state limits.
4. Do not promise stable language-specific imports until a checked-in admin authority contract and validated consumer package manifests exist.

## Control Plane Reference Map

Use this page to decide whether you should be in the admin SDK family at all. Once that is clear,
jump to the exact control-plane reference page for the route semantics and payloads:

| Admin concern | Use this docs page for boundary decisions | Exact API reference |
| --- | --- | --- |
| Overall control-plane surface and source-of-truth scope | `Admin SDK` family boundary | [Control Plane API Overview](/api-reference/control-plane-api) |
| Protocol registry and compatibility decisions | `Admin SDK` audience and release-state rules | [Protocol Governance](/api-reference/control-plane/protocol) |
| Provider registry, provider policy, and provider governance | `Admin SDK` audience and release-state rules | [Provider Governance](/api-reference/control-plane/providers) |
| Drain, activation, routing, and node lifecycle work | `Admin SDK` audience and release-state rules | [Node Operations](/api-reference/control-plane/nodes) |
| Permission model for control-plane consumers | `Admin SDK` consumer rules | [Authentication and Errors](/api-reference/auth-and-errors) |

## What To Read Next

- [Control Plane API Overview](/api-reference/control-plane-api)
- [Protocol Governance](/api-reference/control-plane/protocol)
- [Provider Governance](/api-reference/control-plane/providers)
- [Node Operations](/api-reference/control-plane/nodes)

## Consumer Rules

The admin boundary is still meaningful even before package publication:

- governance consumers should treat control-plane snapshots as the source of truth
- clients should not reconstruct protocol compatibility locally when the control plane already
  publishes the decision surface
- admin integrations should not mix app-facing chat features into this SDK family
- permission handling should follow the documented `control.read` and `control.write` model from
  [Authentication and Errors](/api-reference/auth-and-errors)

## Release Reality

| Artifact | Language | Generation state | Release state |
| --- | --- | --- | --- |
| `admin-typescript` | TypeScript | `template_only_pending_generation` | `not_published` |
| `admin-flutter` | Flutter | `template_only_pending_generation` | `not_published` |

All admin artifacts are also still:

- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`

Treat the workspace layout as real, the control-plane implementation as authoritative, and the
release catalog as the publication-state authority.
