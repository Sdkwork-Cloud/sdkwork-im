# sdkwork-im-app-sdk

Generator-owned Kotlin transport SDK for the Craw Chat app-development API.

This package is generated transport. It targets `/app/v3/api` and is not a login,
user, tenant, organization, or account-session SDK. Those identity and token lifecycles are
owned by `sdkwork-appbase`; this SDK only forwards the already validated dual-token context.

## Token Boundary

- `Authorization: Bearer <authToken>` carries the upstream authenticated principal context.
- `Access-Token: <accessToken>` carries the upstream access token context.
- Login, refresh, current-user, tenant, organization, and account-session APIs stay outside this package.

## Regeneration Contract

- Generated files are tracked by the SDK generator under `.sdkwork/`.
- Fix runtime, OpenAPI, or family generator inputs first, then regenerate.
- Hand-written application wrappers must live outside `generated/server-openapi`.
