# Craw Chat IM App API And SDK Integration Standard

- Version: 1.0
- Scope: Craw Chat PC, `sdkwork-im-app-sdk`, `sdkwork-im-backend-sdk`, `@sdkwork/im-sdk`, appbase IAM integration, local SQLite/PostgreSQL development, and release dependency sourcing
- Related root standards: `../../../specs/API_SPEC.md`, `../../../specs/SDK_SPEC.md`, `../../../specs/IAM_SPEC.md`, `../../../specs/DATABASE_SPEC.md`, `../../../specs/CONFIG_SPEC.md`, `../../../specs/DEPLOYMENT_SPEC.md`, `../../../specs/TEST_SPEC.md`

This local standard narrows the root SDKWork standards for the Craw Chat application. It is authoritative for the Craw Chat app workspace when root examples mention retired generic Spring app/backend SDK packages or authorities.

## 1. API Surface Ownership

| Surface | Prefix | Craw Chat owner | SDK family | Purpose |
| --- | --- | --- | --- | --- |
| IM Open API | `/im/v3/api` | `im-open-api` | `@sdkwork/im-sdk` | Standard open instant-messaging runtime API for conversations, messages, contacts, realtime, streams, and RTC signaling integration. |
| IM App API | `/app/v3/api` | `im-app-api` | `sdkwork-im-app-sdk` | Current Craw Chat app backend API implementation surface for app/client business flows, appbase IAM integration, bootstrap, and product-facing app capabilities. |
| IM Backend API | `/backend/v3/api` | `im-backend-api` | `sdkwork-im-backend-sdk` | Operator, admin, governance, audit, and control-plane capabilities for Craw Chat. |

Rules:

- `im-open-api` is the standard open IM API and must not expose login, register, logout, refresh, password reset, verification-code, or OAuth session creation endpoints.
- `im-app-api` is the current app/backend implementation surface for Craw Chat app clients. It may expose app login and appbase IAM operations through the injected appbase IAM module.
- `im-backend-api` owns backend/admin management. It must not expose user login/session creation endpoints.
- Frontend code must call `im-open-api` through `@sdkwork/im-sdk`, `im-app-api` through `sdkwork-im-app-sdk`, and backend/admin APIs through `sdkwork-im-backend-sdk`.
- Missing backend or IM capabilities must be closed by adding or updating the OpenAPI contract and generator flow. Frontend services must not add raw `/im/v3/*`, `/app/v3/*`, or `/backend/v3/*` HTTP fallbacks.

## 2. Product SDK Ownership

Craw Chat uses product-scoped SDKs, not generic Spring SDK packages.

| Concern | Required package | Required primary client | Compatibility alias |
| --- | --- | --- | --- |
| App API | `@sdkwork-internal/im-app-api-generated` from `sdks/sdkwork-im-app-sdk` | `SdkworkImAppClient` | `SdkworkAppClient` |
| Backend API | `@sdkwork-internal/im-backend-api-generated` from `sdks/sdkwork-im-backend-sdk` | `SdkworkImBackendClient` | `SdkworkBackendClient` |
| IM API | `@sdkwork/im-sdk` from `sdks/sdkwork-im-sdk` | `ImSdkClient` | none |

Rules:

- Craw Chat app code must not import retired generic Spring app/backend SDK packages or authorities.
- Generated client classes must be product-scoped so multiple app SDKs can be installed without class-name collisions.
- Compatibility aliases may exist only inside the generated SDK package for migration. Craw Chat integration code must use the product-scoped client names.
- App SDK and backend SDK generated outputs must remain reproducible from their OpenAPI inputs and generator configuration. Generated files must not be the long-term source of hand-written behavior.

## 3. IAM Login And IM Session Continuity

Craw Chat PC uses appbase IAM UI and runtime, but the concrete client injected into IAM is `SdkworkImAppClient`.

Required flow:

```text
AuthGate
  -> SdkworkIamAuthRoutes
  -> getSdkworkChatIamRuntime()
  -> createAppAuthService(() => getAppSdkClientWithSession(readAppSdkSessionTokens()))
  -> createIamAppSdkAdapter(SdkworkImAppClient)
  -> sdkwork-appbase-app-sdk auth.sessions / registrations / verificationCodes / oauth.deviceAuthorizations
  -> persist authToken + accessToken + refreshToken + context + sessionId + user
  -> reset and recreate @sdkwork/im-sdk with the same token manager and AppContext
```

Rules:

- Default login methods are password-only.
- Verification-code login is disabled by default.
- Registration verification for email and phone is required unless a documented deployment setting changes it.
- QR scan login is enabled through the appbase OAuth device authorization resource: `oauth.deviceAuthorizations.create`, `oauth.deviceAuthorizations.retrieve`, `oauth.deviceAuthorizations.scans.create`, and `oauth.deviceAuthorizations.passwordCompletions.create`.
- The legacy appbase QR auth resource `openPlatform.qrAuth` and `/app/v3/api/open_platform/qr_auth/*` paths are retired and must not be consumed, proxied, regenerated, or documented as current capabilities.
- The current canonical appbase auth package exposes `qrLoginEnabled`; Craw Chat must not pass unsupported runtime-config fields such as `qrLoginType` until the canonical appbase type includes them.
- A successful login must persist `authToken`, `accessToken`, optional `refreshToken`, `context`, `sessionId`, and normalized user data.
- `@sdkwork/im-sdk` construction must receive the same auth token manager, `accessToken`, and platform identity derived from the persisted IAM session. Current `tenantId`, `organizationId`, `userId`, and `sessionId` are request context and must be attached through the dynamic request header provider, not passed as static SDK options.
- After login, chat and RTC code must be able to call IM SDK methods without a second login or manually assembled auth headers.

## 4. Database Sharing

Craw Chat uses the standard SDKWork Chat database policy for appbase IAM and IM data.

Rules:

- Local development defaults to the per-app private SQLite database: `~/.sdkwork/chat/data/chat.sqlite`.
- PostgreSQL is supported by setting canonical `SDKWORK_CHAT_DATABASE_*` variables such as `SDKWORK_CHAT_DATABASE_ENGINE=postgresql`, `SDKWORK_CHAT_DATABASE_HOST`, `SDKWORK_CHAT_DATABASE_NAME`, `SDKWORK_CHAT_DATABASE_USERNAME`, `SDKWORK_CHAT_DATABASE_PASSWORD`, and `SDKWORK_CHAT_DATABASE_SSL_MODE`.
- Legacy `SDKWORK_CLAW_DATABASE_*` variables may be bridged for compatibility, but new configuration and documentation must use the canonical `SDKWORK_CHAT_DATABASE_*` namespace.
- Craw Chat must not create duplicate IAM tables or alternate login schemas when appbase IAM already owns those tables.
- Schema or migration changes require explicit approval before implementation.

## 5. Local Development And Release Dependency Sourcing

Local development and release use different source materialization strategies but both compile from source.

Rules:

- Local `apps/sdkwork-chat-pc/package.json` dependencies for SDKWork packages use relative `link:` specifiers.
- Vite and TypeScript aliases resolve generated IM app/backend SDKs, `@sdkwork/im-sdk`, `@sdkwork/drive-app-sdk`, appbase IAM packages, core PC React, and UI PC React to source entries, not prebuilt `dist`.
- Vite `optimizeDeps.exclude` must include linked SDKWork source packages so live source edits are not hidden by dependency pre-bundling.
- Local PC development exposes one public backend entrypoint, `http://127.0.0.1:18079`, through the unified Craw Chat gateway. Appbase IAM App API defaults route through `sdkwork-api-gateway`; explicit split-deployment overrides may still point to an internal appbase upstream such as `127.0.0.1:18090`, but the renderer must not depend on that port directly.
- The chat-pc `pnpm-workspace.yaml` must not register sibling `sdkwork-appbase`, `sdkwork-core`, or `sdkwork-ui` packages as workspace importers. They remain source-linked dependencies; otherwise pnpm install rewrites sibling `node_modules` and breaks isolated local builds.
- Release builds set `SDKWORK_SHARED_SDK_MODE=git`, run `prepare:shared-sdk`, materialize `sdkwork-im-app-sdk`, `sdkwork-im-backend-sdk`, `sdkwork-im-sdk`, `sdkwork-drive-app-sdk`, `sdkwork-appbase`, `sdkwork-core`, `sdkwork-ui`, `sdkwork-claw-router`, and `sdkwork-birdcoder` from git-backed source checkouts, then build Craw Chat PC from those source links.

## 5.1 Shared Gateway Foundation Composition

Craw Chat product APIs remain Craw Chat owned: IM open API stays under `/im/v3/api`, IM app API stays
under `/app/v3/api`, and IM backend API stays under `/backend/v3/api` with their own SDK families.
Shared foundation APIs are different: appbase, Drive, Notary, RTC provider/runtime bridges, AIoT,
and SDKWork kernel/agent-business runtime dependencies target `sdkwork-api-gateway` as the shared
composition point.

Rules:

- Production and private deployments should use `SDKWORK_CHAT_SERVER_API_BASE_URL` as the common SDK
  root when that root points at `sdkwork-api-gateway` or a reverse proxy backed by it.
- Browser/app SDK bootstrap continues to use `VITE_CRAW_CHAT_APP_API_BASE_URL`,
  `VITE_CRAW_CHAT_IM_API_BASE_URL`, and `VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL`; these are derived
  from the common gateway root by runtime/bootstrap scripts.
- Local PC development starts the sibling `sdkwork-api-gateway` Cargo service. Product-local
  foundation upstream defaults for Appbase, Drive, and Notary point at that gateway root by
  default. Direct dependency-owned service URLs are allowed only through explicit split-deployment
  override env keys such as `CRAW_CHAT_APPBASE_APP_API_UPSTREAM`,
  `CRAW_CHAT_DRIVE_APP_API_UPSTREAM`, and `CRAW_CHAT_NOTARY_APP_API_UPSTREAM`.
- `services/web-gateway`, `crates/craw-chat-gateway-config`, and `services/local-minimal-node`
  remain only as migration compatibility and local/private runtime parity layers. They must not be
  treated as the long-term foundation API aggregation authority.
- Direct Rust dependencies on sibling foundation runtime repositories are tracked by Cargo
  workspace metadata. `specs/component.spec.json` records only their migration state and target
  gateway feature family; it must not duplicate those dependencies into a standalone gateway catalog.
- Appbase, Drive, Notary, RTC, Agent/Kernel, and AIoT dependency surfaces are declared in
  `specs/component.spec.json` as shared-gateway targets with current legacy web-gateway
  compatibility. Add new foundation surfaces there only when an existing SDKWork spec or runtime
  contract proves the surface, prefix, SDK family, and migration target.
- No new product server code may directly mount foundation API runtime crates when the required
  surface is already served by `sdkwork-api-gateway`. Embed `sdkwork-api-gateway-runtime` through
  its public router builders or consume the gateway as an external service.

## 6. IM Media Upload And Drive Attribution

Craw Chat owns IM conversation semantics, but SDKWork Drive owns all chat file upload, storage, Drive space, node, upload session, download grant, provider, and media lifecycle behavior.

Standard upload attribution for chat message attachments:

| Field | Value |
| --- | --- |
| Drive SDK package | `@sdkwork/drive-app-sdk` |
| Drive app API prefix | `/app/v3/api/drive` |
| `appId` | `chat` |
| `appResourceType` | `im_conversation` |
| `appResourceId` | backend conversation id |
| `scene` | `im` |
| `source` | `chat_message` |
| file upload profile | `attachment` for generic files; image, voice, and video use the Drive SDK media upload methods and Drive profile defaults |
| IM persisted reference | `driveUri`, `spaceId`, `nodeId`, and Drive-backed `MediaResource` |

Rules:

- Chat images, voice messages, videos, and files must be uploaded through `sdkwork-drive-app-sdk` uploader methods before the IM message is posted.
- UI-local `File`, `Blob`, `blob:` object URLs, `data:` URLs, recorder buffers, screenshot buffers, and picker previews are transient UI state only. They must not be stored in IM message bodies, render hints, `MediaResource.url`, `MediaResource.publicUrl`, or backend persistence.
- IM message content parts for uploaded media must use `kind = media`, a canonical Drive reference `drive://spaces/{spaceId}/nodes/{nodeId}`, and a Drive-backed `MediaResource` with `source = drive`.
- Chat services must not construct fake Drive URIs from conversation ids, local content, file names, timestamps, or browser preview URLs.
- Chat services must not call raw `/app/v3/api/drive/*`, `/drive/uploader/*`, `/upload_sessions/*`, provider presign URLs, S3, OSS, MinIO, or local object-storage endpoints directly.
- Backend IM validation must reject Drive-backed media snapshots that contain local preview delivery URLs, including nested posters, thumbnails, or variants.

## 7. Backend Capability Planning

When the front end needs a capability that is not present in the backend:

1. Classify the capability as `im-open-api`, `im-app-api`, or `im-backend-api`.
2. Define a resource-oriented path under `/im/v3/api`, `/app/v3/api`, or `/backend/v3/api`.
3. Define dotted operation IDs that generate nested SDK methods.
4. Define request/response schemas, pagination, idempotency, errors, auth requirements, and AppContext behavior.
5. Update the OpenAPI source and generator inputs.
6. Regenerate the owning SDK family.
7. Connect frontend services through the generated SDK or approved composed SDK layer.

No temporary frontend mock branch, raw HTTP client, manual `Authorization`, manual `Access-Token`, or local DTO fork is allowed as a substitute for the missing API contract.

Current local-only shell endpoints under `apps/sdkwork-chat-pc/server.ts` and `local-api.ts` are limited to development support:

- `GET /api/config/modules`
- `POST /api/agent/doc`
- `POST /api/agent/icon`

These endpoints are not IM API endpoints and must not move under `/im/v3/api`. When promoted beyond local shell support, they must be classified under `im-app-api`, added to `/app/v3/api` OpenAPI, regenerated through `sdkwork-im-app-sdk`, and consumed through the product app SDK instead of frontend raw `fetch`.

## 8. Verification

Minimum verification for this standard:

- `node apps/sdkwork-chat-pc/scripts/auth-appbase-ui-contract.test.mjs`
- `node scripts/dev/sdkwork-chat-pc-sdk-integration.test.mjs`
- `node scripts/dev/sdkwork-chat-pc-im-api-standard.test.mjs`
- `node scripts/dev/sdkwork-chat-pc-dev-command.test.mjs`
- `node sdks/sdkwork-im-app-sdk/tests/app-sdk-auth-surface-contract.test.mjs`
- `node sdks/test/verify-im-v3-sdk-family-contract.test.mjs`
- `pnpm install --frozen-lockfile` from `apps/sdkwork-chat-pc`
- `pnpm lint` from `apps/sdkwork-chat-pc`
- `pnpm build` from `apps/sdkwork-chat-pc`
