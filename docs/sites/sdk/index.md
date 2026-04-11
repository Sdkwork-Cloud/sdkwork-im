# SDK Overview

The repository currently defines two SDK families with different consumers, contracts, and release
truth sources:

- `sdkwork-craw-chat-sdk`
  The app-facing SDK workspace.
- `sdkwork-craw-chat-sdk-admin`
  The admin and control-plane SDK workspace.

## Current Delivery Reality

The release catalog at `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json` currently
declares:

- `state = template_only_pending_generation`
- all four artifacts have `generationStatus = template_only_pending_generation`
- all four artifacts have `releaseStatus = not_published`

That means the repository has a real SDK workspace structure, but the current release wave does not
yet represent published packages.

## SDK Family Matrix

| Family | Audience | Languages | Contract source | Current release state |
| --- | --- | --- | --- | --- |
| `sdkwork-craw-chat-sdk` | App and product integrations | TypeScript, Flutter | Checked-in OpenAPI 3.0.3 authority at `sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml` plus the derived `craw-chat-app.sdkgen.yaml` | Workspace present, catalog still `template_only_pending_generation` |
| `sdkwork-craw-chat-sdk-admin` | Admin and control-plane integrations | TypeScript, Flutter | `services/control-plane-api/src/lib.rs` plus control-plane tests; no checked-in admin OpenAPI authority file under the admin SDK workspace yet | Workspace present, catalog still `template_only_pending_generation` |

## API Group To SDK Mapping

| API group | SDK family | Notes |
| --- | --- | --- |
| App API | `sdkwork-craw-chat-sdk` | Public app-facing surface; websocket upgrade is documented but not generated as a manual realtime adapter in this round |
| Platform API | No separate published family | Routes exist and are documented, but not split into a standalone SDK family |
| IoT API | No separate published family | Currently documented as HTTP and provider-integration surfaces |
| Control Plane API | `sdkwork-craw-chat-sdk-admin` | Administrative and governance surface |

## Source-of-truth Rules

- The app SDK workspace has a checked-in OpenAPI authority contract and a derived sdkgen input.
- The admin SDK workspace currently freezes consumer boundary and audience rules, but its source of
  truth still comes directly from the control-plane service and tests.
- Generated output must not be hand-edited in place. Change the authority contract or workspace
  wrapper inputs and regenerate.

## Recommended Reading

- [App SDK](/sdk/app-sdk)
- [Admin SDK](/sdk/admin-sdk)
- [Language Support](/sdk/language-support)
- [App API Overview](/api-reference/app-api)
- [Control Plane API Overview](/api-reference/control-plane-api)
