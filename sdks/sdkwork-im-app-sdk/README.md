# SDKWork IM App SDK

`sdkwork-im-app-sdk` is the `/app/v3/api` SDK family for developers building instant messaging
applications on top of Craw Chat.

This workspace is intentionally separate from `sdkwork-im-sdk`:

- `sdkwork-im-sdk` targets the IM open-platform contract exported at `/im/v3/openapi.json`.
- `sdkwork-im-app-sdk` targets the app-development contract exported at `/app/v3/openapi.json`.
- `sdkwork-im-app-sdk` depends on `sdkwork-appbase-app-sdk` for appbase identity, session, IAM,
  verification, and QR auth capability.
- `sdkwork-im-app-sdk` depends on `sdkwork-im-sdk` for standardized IM capability and on
  `sdkwork-rtc-sdk` for provider-standard RTC runtime capability.
- `sdkwork-im-app-sdk` ownsIdentityLifecycle: false. It consumes appbase identity/session context
  through the appbase SDK and must not regenerate appbase-owned app routes.

## SDK Dependency Contract

The app SDK is the application-facing composition point, but its generated transport remains scoped
to `/app/v3/api` only.

- `sdkwork-appbase-app-sdk` remains the owner of appbase `/app/v3/api` identity/session/IAM,
  verification, runtime policy, and QR auth routes.
- `sdkwork-im-sdk` remains the owner of `/im/v3/api` standardized IM routes, realtime adapters, and
  semantic IM modules.
- `sdkwork-rtc-sdk` remains the owner of provider-standard RTC runtime contracts, provider catalogs,
  and runtime bridge semantics.
- `sdkwork-im-app-sdk` owns app-business HTTP routes such as portal access, device twin,
  notifications, automation, provider health, IoT provider APIs, and RTC provider health/callback
  app APIs.
- Generated app transport must not import, vendor, or regenerate `sdkwork-appbase-app-sdk`,
  `sdkwork-im-sdk`, or `sdkwork-rtc-sdk`; consumers compose those SDKs at the app SDK boundary.

Machine-readable contract fields:

| Field | `sdkwork-appbase-app-sdk` | `sdkwork-im-sdk` | `sdkwork-rtc-sdk` |
| --- | --- | --- | --- |
| `sdkDependencies[].workspace` | `sdkwork-appbase-app-sdk` | `sdkwork-im-sdk` | `sdkwork-rtc-sdk` |
| `sdkDependencies[].role` | `appbase-identity-and-session-capability` | `standardized-im-capability` | `provider-standard-rtc-runtime` |
| `sdkDependencies[].required` | `true` | `true` | `true` |
| `sdkDependencies[].dependencyMode` | `consumer-sdk` | `consumer-sdk` | `consumer-sdk` |
| `sdkDependencies[].apiPrefix` | `/app/v3/api` | `/im/v3/api` | `null` |
| `sdkDependencies[].generatedTransportImportPolicy` | `forbidden` | `forbidden` | `forbidden` |

Package-level dependency names:

| Language | Appbase app SDK dependency | IM SDK dependency | RTC SDK dependency |
| --- | --- | --- | --- |
| TypeScript | `@sdkwork/appbase-app-sdk` | `@sdkwork/im-sdk` | `@sdkwork/rtc-sdk` |
| Flutter | `sdkwork_appbase_app_sdk` | `im_sdk` | `rtc_sdk` |
| Rust | `sdkwork-appbase-app-sdk` | `im-sdk` | `rtc_sdk` |
| Java | `com.sdkwork:sdkwork-appbase-app-sdk` | `com.sdkwork:im-sdk` | `com.sdkwork:rtc-sdk` |
| C# | `SDKWork.Appbase.AppSdk` | `Sdkwork.Im.Sdk` | `Sdkwork.Rtc.Sdk` |
| Swift | `sdkwork-appbase-app-sdk` | `ImSdk` | `RtcSdk` |
| Kotlin | `com.sdkwork:sdkwork-appbase-app-sdk` | `com.sdkwork:im-sdk` | `com.sdkwork:rtc-sdk` |
| Go | `github.com/sdkwork/sdkwork-appbase-app-sdk` | `github.com/sdkwork/im-sdk` | `github.com/sdkwork/rtc-sdk` |
| Python | `sdkwork-appbase-app-sdk` | `sdkwork-im-sdk` | `sdkwork-rtc-sdk` |

## Contract Files

- `openapi/craw-chat-app-api.openapi.yaml`
  Authority OpenAPI 3.x contract for `/app/v3/api`.
- `openapi/craw-chat-app-api.sdkgen.yaml`
  Default generator-compatible derived input.
- `openapi/craw-chat-app-api.flutter.sdkgen.yaml`
  Flutter-compatible derived input with primitive component refs expanded.

## Generation

Primary Node entrypoint:

```powershell
node .\bin\generate-sdk.mjs --language typescript --language flutter
```

PowerShell:

```powershell
.\bin\generate-sdk.ps1 -Languages typescript,flutter
```

Bash:

```bash
./bin/generate-sdk.sh --language typescript --language flutter
```

Defaults:

- base URL: `http://127.0.0.1:18090`
- schema URL: `/app/v3/openapi.json`
- API prefix: `/app/v3/api`
- SDK name: `sdkwork-im-app-sdk`
- SDK target/type: `app`
- standard profile: `sdkwork-v3`

Generated output is written under language-specific `sdkwork-im-app-sdk-*` directories. Do not edit
generated output by hand; update the OpenAPI contract or generator inputs and regenerate.

## Flutter Layered Boundary

Flutter keeps both generated and manual-owned layers:

- generated transport package:
  `sdkwork-im-app-sdk-flutter/generated/server-openapi` (`im_app_api_generated`)
- consumer-facing composed package:
  `sdkwork-im-app-sdk-flutter/composed` (`im_app_sdk`)

The composed package re-exports generated transport and provides `ImAppSdkClient` plus semantic
modules (`portal`, `device`, `notification`, `automation`, `provider`, `iot`, `rtc`). Keep HTTP
transport ownership in generated output and place manual ergonomics only in `composed`.

## Verification

```powershell
node .\bin\verify-sdk.mjs
```

The verifier checks the `/app/v3/api` OpenAPI surface, appbase route exclusion, dual-token
`AuthToken` and `AccessToken` security, problem-detail errors, generated language manifests,
TypeScript `SdkworkImAppClient` plus `SdkworkAppClient` compatibility alias surface parity, and
Flutter composed workspace presence/contracts.

## Release Snapshot Boundary

This workspace inherits the current SDK release snapshot from
`artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`.

- `state = generated_pending_publication`
- `generationStatus = generated`
- `releaseStatus = not_published`
- `plannedVersion = null`
- `versionStatus = version_unassigned_pending_freeze`
- `versionDecisionSourcePath = null`
