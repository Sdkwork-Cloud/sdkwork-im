# SDKWork IM Backend SDK Component Specs

This directory is the local standards index for `sdkwork-im-backend-sdk`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../../../../../specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `sdkwork-im-backend-sdk` |
| Type | `sdk-family` |
| Root | `craw-chat/sdks/sdkwork-im-backend-sdk` |
| Domain | `communication` |
| Capability | `chat` |
| Languages | `csharp, flutter, go, java, kotlin, python, rust, swift, typescript` |
| Status | `standardizing` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- Generated SDK language outputs are represented at this SDK family root instead of duplicating local specs in generated folders.

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [API_SPEC.md](../../../../../specs/API_SPEC.md) | HTTP/OpenAPI and generated SDK contract rules. |
| [COMPONENT_SPEC.md](../../../../../specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [DOCUMENTATION_SPEC.md](../../../../../specs/DOCUMENTATION_SPEC.md) | Module README, examples, ADR, changelog, and runbook rules. |
| [SDK_SPEC.md](../../../../../specs/SDK_SPEC.md) | SDK generation and SDK integration rules. |
| [TEST_SPEC.md](../../../../../specs/TEST_SPEC.md) | Contract, SDK, and documentation verification rules. |

## SDK Clients

- `SdkworkBackendClient`

## Verification

- `node sdks/sdkwork-im-backend-sdk/bin/verify-sdk.mjs`
