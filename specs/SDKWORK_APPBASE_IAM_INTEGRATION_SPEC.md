# SDKWork Appbase IAM Integration Standard

- Version: 1.0
- Scope: app-side login, registration, sessions, verification codes, OAuth device authorization QR login, appbase IAM UI integration, generated app SDK integration, Sdkwork IM gateway/local runtime parity
- Related: `IAM_SPEC.md`, `SDK_SPEC.md`, `APP_PC_REACT_UI_SPEC.md`, `CONFIG_SPEC.md`, `SECURITY_SPEC.md`, `TEST_SPEC.md`, `DEPLOYMENT_SPEC.md`

This standard defines how applications integrate `sdkwork-appbase` IAM login and registration. It is based on the current Sdkwork IM PC integration and is intended as the reusable reference for other SDKWork apps.

The integration goal is simple: product UI reuses appbase IAM UI, product runtime adapts appbase IAM contracts to the generated app SDK, and all HTTP traffic uses the standard `/app/v3/api` IAM routes through the gateway or configured appbase upstream. Applications must not create parallel login/register forms, raw HTTP auth clients, local SDK forks, or app-specific IAM route names.

## 1. Canonical Architecture

Frontend flow:

```text
React Router / AuthGate
  -> @sdkwork/auth-pc-react SdkworkIamAuthRoutes
  -> product getSdkwork<App>IamRuntime()
  -> product appAuthService
  -> @sdkwork/iam-sdk-adapter createIamAppSdkAdapter()
  -> generated app SDK client
  -> /app/v3/api/auth | /iam | /oauth | /system/iam
```

Backend/gateway flow:

```text
Browser, desktop renderer, or mobile app
  -> /app/v3/api/auth | /iam | /oauth | /system/iam
  -> web gateway route registry
  -> sdkwork-appbase-app-api upstream in split/server mode
  -> product local/private runtime only when embedded mode has no appbase upstream
```

Rules:

- Login, registration, current session, refresh, logout, OAuth, verification codes, password reset, QR login, and current user flows are app-api capabilities under `/app/v3/api`.
- App UI must depend on reusable IAM UI/runtime contracts, not generated SDK constructors.
- Product service code must use the generated app SDK or the approved IAM SDK adapter.
- Backend-api must not expose login/session creation APIs.
- Local/private Rust implementations are parity implementations of appbase IAM, not a divergent product-local auth system.

## 2. Dependency Boundary And Cycle Prevention

The dependency direction is fixed:

```text
product app shell
  -> appbase auth UI package
  -> product IAM runtime factory
  -> product appAuthService
  -> appbase IAM SDK adapter / ports
  -> generated app SDK
```

Rules:

- `@sdkwork/auth-pc-react` and other appbase packages must not import product app packages.
- Product `AuthGate` may import appbase UI and product runtime helpers.
- Product runtime may import appbase auth types, IAM adapters, generated SDK types, and local session helpers.
- Product feature UI must not import generated SDK internals to bypass the runtime/service boundary.
- Generated SDK packages must not import React UI, appbase UI packages, product shell code, or gateway/runtime crates.
- Rust gateway and local runtime crates must not import frontend packages.
- If a reusable appbase package needs product-specific behavior, inject it through runtime interfaces, callbacks, configuration, or appearance tokens. Do not add reverse imports from appbase to the product app.

## 3. Frontend Integration Rules

Every PC React app integrating appbase IAM should provide one auth gate component responsible for route protection and auth route mounting.

Required shape:

```tsx
<SdkworkIamAuthRoutes
  appearance={resolveSdkwork<App>AuthAppearance()}
  basePath="/auth"
  getRuntime={getSdkwork<App>IamRuntime}
  homePath="/"
  locale={resolveAuthLocale()}
  runtimeConfig={resolveSdkwork<App>AuthRuntimeConfig()}
  viewportMode="flow"
/>
```

Rules:

- Auth routes are mounted under `/auth`.
- Unauthenticated protected routes redirect to `/auth/login?redirect=<encoded_target>`.
- Authenticated users visiting `/auth/*` are redirected to the requested redirect target or app home.
- The app must reuse `SdkworkIamAuthRoutes`; it must not maintain a bespoke login/register form for the same flows.
- UI components must not call `appAuthService.login`, `appAuthService.register`, generated SDK auth methods, raw `fetch`, or `axios` directly.
- Theme, shell controls, desktop title bar behavior, and web/desktop AppHeader behavior belong to product shell code, not reusable appbase IAM business logic.
- Web and desktop host differences must be detected at shell/runtime boundaries. They must not change the IAM API path or SDK contract.

Reference implementation:

- `apps/sdkwork-im-pc/src/AuthGate.tsx`

## 4. Product Runtime Contract

Each application provides a small product-specific IAM runtime layer that adapts the appbase auth runtime contract to the product app SDK wrapper.

Required exported helpers:

```ts
getSdkwork<App>IamRuntime()
resetSdkwork<App>IamRuntime()
resolveSdkwork<App>AuthRuntimeConfig()
resolveSdkwork<App>AuthAppearance()
appAuthService
```

The runtime must expose these appbase service groups:

```text
auth.sessions.create
auth.sessions.current.retrieve
auth.sessions.current.update
auth.sessions.current.delete
auth.sessions.refresh
auth.registrations.create
auth.verificationCodes.create
auth.verificationCodes.verify
auth.passwordResetRequests.create, if enabled
auth.passwordResets.create, if enabled
iam.users.current.retrieve
oauth.authorizationUrls.create
oauth.deviceAuthorizations.create
oauth.deviceAuthorizations.retrieve
oauth.deviceAuthorizations.scans.create
oauth.deviceAuthorizations.passwordCompletions.create
oauth.sessions.create
system.iam.runtime.retrieve, when exposed by the UI
system.iam.verificationPolicy.retrieve
```

Rules:

- Unsupported auth methods must fail explicitly with a clear error. They must not silently fake success.
- Runtime config controls which methods the UI presents, for example password login, email registration, phone registration, QR login, OAuth, recovery, and verification policy.
- Token store operations must persist, read, and clear product session tokens through shared session helpers.
- After login, registration, QR completion, refresh, or logout, product SDK clients that cache auth state must be reset.
- Runtime sessions returned to appbase UI must include `authToken`, `accessToken`, optional `refreshToken`, `sessionId`, `user`, and `context` when available.

Reference implementation:

- `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appAuthRuntime.ts`

## 5. App Auth Service Contract

Each application provides `appAuthService` as the product service facade used by the IAM runtime.

Required service methods:

```ts
login({ username, password, remember? })
register({ username, password, confirmPassword?, email?, phone?, name?, verificationCode? })
logout()
refreshToken(refreshToken?)
sendVerifyCode({ target, verifyType, scene })
verifyCode({ target, verifyType, scene, code })
getCurrentSession()
createOAuthDeviceAuthorization({ purpose? })
retrieveOAuthDeviceAuthorization(deviceAuthorizationId)
createOAuthDeviceAuthorizationScan(deviceAuthorizationId, input?)
createOAuthDeviceAuthorizationPasswordCompletion(deviceAuthorizationId, input)
```

Rules:

- `appAuthService` must call `createIamAppSdkAdapter(getClient())` or an equivalent approved appbase IAM adapter over the generated app SDK.
- `appAuthService` must not call raw HTTP, generic request helpers, local controller endpoints, or manual `Authorization` / `Access-Token` header plumbing.
- DTOs come from the generated app SDK or shared IAM contracts. Product-local DTOs are allowed only as view/service input models, not as replacements for missing SDK response types.
- Login and registration responses must be normalized into the product session shape and persisted immediately.
- Logout must attempt server logout and must always clear local tokens in `finally`.
- Current session restore may return the persisted session if the remote current-session check fails transiently, but new applications should treat this as resilience only. It must not hide missing auth APIs.

Required generated/adapter resource calls:

```text
getIam().auth.sessions.create(body)
getIam().auth.sessions.current.retrieve()
getIam().auth.sessions.current.delete()
getIam().auth.sessions.refresh(body)
getIam().auth.registrations.create(body)
getIam().auth.verificationCodes.create(body)
getIam().auth.verificationCodes.verify(body)
getIam().oauth.deviceAuthorizations.create(body)
getIam().oauth.deviceAuthorizations.retrieve(deviceAuthorizationId)
getIam().oauth.deviceAuthorizations.scans.create(deviceAuthorizationId, body)
getIam().oauth.deviceAuthorizations.passwordCompletions.create(deviceAuthorizationId, body)
```

Reference implementation:

- `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appAuthService.ts`

## 6. App SDK Client And Token Rules

Each application exposes a single app SDK client wrapper for app-api calls.

Required wrapper helpers:

```ts
createAppSdkClientConfig(session?)
initAppSdkClient(config?)
getAppSdkClient()
getAppSdkClientWithSession(session?)
resetAppSdkClient()
useAppSdkClient()
resolveAppSdkBaseUrl()
```

Rules:

- Generated app SDK source is the only remote business transport source. Sdkwork IM uses `@sdkwork-internal/im-app-api-generated` for the app-api surface that includes appbase IAM paths.
- Base URL normalization must remove SDK-owned suffixes such as `/app/v3/api` and `/im/v3/api` before constructing the generated SDK client, when the generated client expects an origin/base root.
- Production builds may use same-origin fallback when the web build is served by the unified gateway origin.
- Development may use a documented local gateway fallback, for example `http://127.0.0.1:18079`.
- `authToken` and `accessToken` must be managed through SDK config and token manager support, not service-layer manual headers.
- Product service modules must not set `Authorization` or `Access-Token` directly.
- Tenant, organization, platform, and app context headers are built from verified/persisted IAM session context. They must not be taken from mutable route/query/body state.

Session shape:

```ts
{
  accessToken?: string;
  authToken?: string;
  refreshToken?: string;
  context?: IamAppContext;
  expiresAt?: number;
  sessionId?: string;
  user?: IamUserSummary;
}
```

For normal protected app-api operation, both tokens should be present:

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```

Reference implementations:

- `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appSdkClient.ts`
- `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts`

## 7. AppContext Header Rules

The SDK wrapper may attach AppContext headers only from the verified IAM session context.

Standard headers:

```text
X-Sdkwork-App-Id
X-Sdkwork-Tenant-Id
X-Sdkwork-Organization-Id
X-Sdkwork-User-Id
X-Sdkwork-Session-Id
X-Sdkwork-Environment
X-Sdkwork-Deployment-Mode
X-Sdkwork-Auth-Level
X-Sdkwork-Actor-Id
X-Sdkwork-Actor-Kind
X-Sdkwork-Device-Id
X-Sdkwork-Data-Scope
X-Sdkwork-Permission-Scope
X-Sdkwork-Context-Signature, when supported
```

Rules:

- Server-side token/session validation remains authoritative.
- Header context is an SDK/runtime transport optimization and audit hint. It does not replace server verification.
- If request context conflicts with verified token context, server handlers must reject the request unless a documented cross-tenant/platform permission applies.

## 8. Environment Configuration

Applications should support deployment mode and app API base URL configuration through documented environment keys.

Deployment modes:

| Product mode | IAM mode | Meaning |
| --- | --- | --- |
| `desktop-local` | `local` | Local desktop or embedded node runtime, usually no external appbase upstream |
| `server-private` | `private` | Private deployment with gateway and service upstreams |
| `cloud-saas` | `cloud` | SaaS deployment using cloud appbase app-api |

Recommended generic keys:

```text
SDKWORK_<APP>_IAM_DEPLOYMENT_MODE
VITE_SDKWORK_<APP>_IAM_DEPLOYMENT_MODE
VITE_SDKWORK_DEPLOYMENT_MODE
SDKWORK_IAM_MODE
VITE_SDKWORK_IAM_APP_API_BASE_URL
VITE_<APP>_APP_API_BASE_URL
VITE_<APP>_IM_API_BASE_URL
VITE_<APP>_IM_WEBSOCKET_BASE_URL
VITE_SDKWORK_<APP>_AUTH_DEV_DEFAULT_ACCOUNT
VITE_SDKWORK_<APP>_AUTH_DEV_DEFAULT_EMAIL
VITE_SDKWORK_<APP>_AUTH_DEV_DEFAULT_PHONE
VITE_SDKWORK_<APP>_AUTH_DEV_DEFAULT_PASSWORD
VITE_SDKWORK_<APP>_AUTH_DEV_VERIFICATION_CODE
VITE_SDKWORK_<APP>_AUTH_DEV_PREFILL_ENABLED
```

Rules:

- New apps should read generic SDKWork keys first when the shared SDK standard defines them, then app-specific fallback keys where migration requires it.
- Auth runtime development prefill is allowed only for development/test and must be explicitly controlled by environment.
- `authToken` must not be preconfigured in environment. It is runtime session state only.
- App-level `accessToken`, when applicable, may come from app config/env, but a logged-in session access token must override or refresh it according to SDK token manager rules.
- URL validation must reject invalid HTTP/WS base URL values during dev server bootstrap.

Reference implementation:

- `apps/sdkwork-im-pc/scripts/sdkwork-chat-iam-env.mjs`

## 9. Gateway And Backend Routing

The gateway must route appbase IAM app-api paths to `sdkwork-appbase-app-api` in split/server mode, and to the product local/private runtime only when embedded mode has no appbase upstream.

Required gateway route descriptors:

```text
serviceId: sdkwork-appbase-app-api
paths:
  /app/v3/api/auth/{*path}
  /app/v3/api/iam/{*path}
  /app/v3/api/oauth/{*path}
  /app/v3/api/system/iam/{*path}
sdk target:
  sdkwork-im-app-sdk or the app's generated app SDK target
```

Rules:

- `/app/v3/api/auth/sessions` must route to appbase IAM app-api in split/server mode.
- Embedded mode may delegate missing `sdkwork-appbase-app-api` upstreams to the product runtime.
- Product-specific app-api routes, such as portal routes, may be delegated to product runtime by explicit path rules.
- CORS must allow browser and desktop renderer origins and all auth/AppContext headers required by the generated SDK.
- Gateway CORS preflight must be handled locally.
- Backend/admin APIs must stay under `/backend/v3/api` and must not implement app login/register/session creation routes.

Reference implementations:

- `services/sdkwork-im-gateway/src/lib.rs`
- `services/sdkwork-im-gateway/tests/http_proxy_test.rs`
- `crates/sdkwork-im-gateway-config/src/lib.rs`

## 10. Local/Private Runtime Parity

Local/private runtime IAM must expose the same path, method, schema, envelope, error, token, and context semantics as appbase app-api.

Minimum local/private app-api routes:

```text
POST   /app/v3/api/auth/sessions
GET    /app/v3/api/auth/sessions/current
PATCH  /app/v3/api/auth/sessions/current
DELETE /app/v3/api/auth/sessions/current
POST   /app/v3/api/auth/sessions/refresh
POST   /app/v3/api/auth/registrations
POST   /app/v3/api/auth/verification_codes
POST   /app/v3/api/auth/verification_codes/verify
POST   /app/v3/api/auth/password_reset_requests
POST   /app/v3/api/auth/password_resets
GET    /app/v3/api/iam/users/current
GET    /app/v3/api/system/iam/runtime
GET    /app/v3/api/system/iam/verification_policy
POST   /app/v3/api/oauth/authorization_urls
POST   /app/v3/api/oauth/device_authorizations
GET    /app/v3/api/oauth/device_authorizations/{deviceAuthorizationId}
POST   /app/v3/api/oauth/device_authorizations/{deviceAuthorizationId}/scans
POST   /app/v3/api/oauth/device_authorizations/{deviceAuthorizationId}/password_completions
POST   /app/v3/api/oauth/sessions
```

Forbidden legacy paths:

```text
/app/v3/api/auth/login
/app/v3/api/auth/register
/app/v3/api/auth/refresh
/app/v3/api/auth/verify/send
/app/v3/api/auth/verify/check
/app/v3/api/open_platform/qr_auth/sessions
/app/v3/api/open_platform/qr_auth/sessions/{sessionKey}
/app/v3/api/open_platform/qr_auth/sessions/{sessionKey}/scans
/app/v3/api/open_platform/qr_auth/sessions/{sessionKey}/passwords
/auth/login
/auth/register
```

Rules:

- Local/private runtime must be treated as appbase IAM parity, not an application-specific shortcut.
- OpenAPI exports must include the standard appbase IAM paths and must exclude legacy auth paths.
- OperationIds must generate resource-style SDK methods such as `client.auth.sessions.create(body)`.
- Any missing local/private capability must be closed in the runtime contract and OpenAPI export, not hidden by a frontend fallback.

Reference implementations:

- `crates/sdkwork-api-product-runtime/src/local_iam.rs`
- `services/local-minimal-node/tests/openapi_schema_export_test.rs`
- `services/local-minimal-node/tests/openapi_im_v3_contract_test.rs`
- `services/local-minimal-node/tests/chat_runtime_session_namespace_test.rs`

## 11. Appbase Package And Workspace Aliasing

PC React apps must resolve appbase IAM packages to the canonical workspace/source packages.

Required packages for PC React IAM:

```text
@sdkwork/auth-pc-react
@sdkwork/auth-pc-react/auth
@sdkwork/appbase-pc-react
@sdkwork/iam-contracts
@sdkwork/iam-sdk-adapter
@sdkwork/iam-sdk-ports
```

Rules:

- Vite, TypeScript paths, package manager workspace config, and test tooling must resolve to the same appbase package source.
- Do not create product-local copies of appbase IAM UI packages.
- Do not hand-edit generated SDK output to make appbase package imports compile.
- If appbase UI needs a missing capability, extend the appbase contract and adapter, then regenerate or rebuild the approved wrapper.

Reference implementation:

- `apps/sdkwork-im-pc/vite.config.ts`
- `apps/sdkwork-im-pc/scripts/auth-appbase-ui-contract.test.mjs`

## 12. Verification Requirements

Each integrated app must include structural and behavioral verification.

Frontend structural checks:

- `AuthGate` imports `SdkworkIamAuthRoutes` from `@sdkwork/auth-pc-react`.
- `AuthGate` mounts appbase routes with `basePath="/auth"` and `homePath="/"`.
- `AuthGate` does not define a bespoke `<form>` for login/register.
- Product UI does not call `appAuthService.login` or `appAuthService.register` outside the appbase runtime.
- Appbase package aliases resolve to canonical workspace packages.
- Host-specific shell behavior, such as desktop AppHeader visibility, stays outside appbase IAM business logic.

Service/SDK checks:

- `appAuthService` uses `createIamAppSdkAdapter(getClient())`.
- Auth methods use generated/adapter resource calls.
- Service code does not use raw `fetch`, `axios`, generic request helpers, manual `Authorization`, or manual `Access-Token`.
- Session persistence stores `authToken`, `accessToken`, `refreshToken`, `context`, `sessionId`, and `user` when returned.
- SDK clients reset after session mutation.

Gateway/runtime checks:

- `/app/v3/api/auth/sessions` routes to `sdkwork-appbase-app-api` in split/server mode.
- Embedded mode delegates missing appbase upstream IAM requests to product runtime.
- CORS preflight succeeds and allows auth/AppContext headers.
- OpenAPI app schema includes standard IAM and OAuth device authorization paths.
- OpenAPI app schema excludes legacy auth paths.

Reference commands for Sdkwork IM:

```text
node apps/sdkwork-im-pc/scripts/auth-appbase-ui-contract.test.mjs
cargo test -p sdkwork-im-gateway --test http_proxy_test
cargo test -p local-minimal-node --test openapi_schema_export_test
cargo test -p local-minimal-node --test openapi_im_v3_contract_test
cargo test -p local-minimal-node --test chat_runtime_session_namespace_test
```

Run narrower tests first, then broader workspace checks when the blast radius warrants it.

## 13. Current Sdkwork IM Reference Map

| Responsibility | Reference file |
| --- | --- |
| Auth route guard and appbase auth UI mounting | `apps/sdkwork-im-pc/src/AuthGate.tsx` |
| Product IAM runtime, token store, appbase runtime config, appearance | `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appAuthRuntime.ts` |
| Product auth service facade over IAM SDK adapter | `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appAuthService.ts` |
| Generated app SDK client bootstrap and base URL normalization | `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/appSdkClient.ts` |
| Session persistence, token manager, AppContext headers | `apps/sdkwork-im-pc/packages/sdkwork-im-pc-core/src/sdk/session.ts` |
| IAM deployment mode and env bootstrap | `apps/sdkwork-im-pc/scripts/sdkwork-chat-iam-env.mjs` |
| Appbase package aliasing | `apps/sdkwork-im-pc/vite.config.ts` |
| Appbase UI/service structural contract tests | `apps/sdkwork-im-pc/scripts/auth-appbase-ui-contract.test.mjs` |
| Gateway route descriptors, CORS, embedded fallback | `services/sdkwork-im-gateway/src/lib.rs` |
| Gateway routing and local IAM proxy tests | `services/sdkwork-im-gateway/tests/http_proxy_test.rs` |
| Local/private IAM parity router | `crates/sdkwork-api-product-runtime/src/local_iam.rs` |
| OpenAPI path parity tests | `services/local-minimal-node/tests/openapi_schema_export_test.rs` |

## 14. Quick Integration Checklist For New Apps

1. Add appbase IAM UI/runtime packages and configure Vite/TypeScript/workspace aliases.
2. Create the app SDK client wrapper around the generated app SDK.
3. Create session helpers for tokens, user, AppContext, token manager, and SDK reset behavior.
4. Create `appAuthService` over `createIamAppSdkAdapter(getClient())`.
5. Create `getSdkwork<App>IamRuntime()` and map appbase runtime methods to `appAuthService`.
6. Create auth runtime config and appearance helpers.
7. Mount `SdkworkIamAuthRoutes` in the app auth gate under `/auth`.
8. Configure IAM deployment mode and app API base URL env handling.
9. Configure gateway route descriptors for appbase IAM app-api paths.
10. Configure embedded/local runtime parity only when the app supports local/private mode.
11. Add structural tests for UI reuse, SDK adapter usage, package aliasing, and raw HTTP/header bans.
12. Add gateway/OpenAPI tests for standard paths and legacy path exclusion.

## 15. Anti-Patterns

The following are not allowed:

- Product-local login/register forms that duplicate `SdkworkIamAuthRoutes`.
- `fetch`, `axios`, or generic request helper calls for IAM login/register/session APIs.
- Manual `Authorization` or `Access-Token` headers in product service code.
- Product-local generated SDK forks for appbase IAM.
- Hand-edited generated SDK files.
- Legacy route names such as `/auth/login`, `/auth/register`, `/app/v3/api/auth/login`, or `/app/v3/api/auth/register`.
- Backend-api login/session creation endpoints.
- Appbase packages importing product app packages.
- Product feature UI importing generated SDK internals to bypass the runtime/service boundary.
- Mock success branches that hide missing SDK, OpenAPI, or backend runtime capabilities.

