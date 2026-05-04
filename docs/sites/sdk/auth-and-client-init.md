# Auth and Client Init

This page explains the shared client bootstrap model across the TypeScript, Flutter, and Rust app
SDKs.

## Public Auth Model

The public app-facing SDK contract is bearer-token based.

- header: `Authorization: Bearer <token>`
- signing secret in local deployments: `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`
- trusted headers are still valid for internal tests and service wiring, but they are not the
  public app SDK contract

If you are documenting or implementing a public consumer path, prefer bearer-token examples only.

## Preferred Client Surface

| Language | Preferred client surface | Auth update method |
| --- | --- | --- |
| TypeScript | `new ImSdkClient({ baseUrl, authToken })` | `sdk.auth.useToken(token)` |
| Flutter | `ImSdkClient.create(...)` | `sdk.auth.useToken(token)` |
| Rust | `ImSdkClient::new_with_base_url(...)` | `client.set_auth_token(token)` |

All three languages also expose the generated transport layer, but the preferred integration surface
is the official app-facing client for each language. TypeScript now ships that public contract from
the root `@sdkwork/im-sdk` package.

## Shared Initialization Flow

1. resolve the app base URL
2. create the preferred app client
3. set or inject the bearer token
4. route work through the semantic modules
5. drop to the App API reference only when you need exact payload or operation detail

## TypeScript

```ts
import { ImSdkClient } from "@sdkwork/im-sdk";

const sdk = new ImSdkClient({
  baseUrl: "http://127.0.0.1:18090",
  authToken: process.env.CRAW_CHAT_TOKEN,
});

const session = await sdk.session.resume({
  deviceId: "device-web-01",
  lastSeenSyncSeq: 0,
});
```

## Flutter

```dart
import 'package:im_sdk/im_sdk.dart';

final client = ImSdkClient.create(
  baseUrl: 'http://127.0.0.1:18090',
  authToken: token,
);

final session = await client.session.resume(
  ResumeSessionRequest(
    deviceId: 'device-mobile-01',
    lastSeenSyncSeq: 0,
  ),
);
```

## Rust

```rust
use im_sdk::{ImSdkClient, ResumeSessionRequest};

let client = ImSdkClient::new_with_base_url("http://127.0.0.1:18090")?;
client.set_auth_token(token);

let session = client
  .session()
  .resume(ResumeSessionRequest {
    device_id: Some("device-rust-01".into()),
    last_seen_sync_seq: Some(0),
  })
  .await?;
```

## Ownership Boundary

- `generated/server-openapi` is generator-owned transport output
- TypeScript assembles that generated boundary into the root `@sdkwork/im-sdk` package under
  `src/generated/**`
- `composed` remains the manual-owned authoring layer or semantic reserve before publication
- if a generated behavior needs to change, change the contract or generator inputs and regenerate

## Realtime Transport Note

The current SDK family splits realtime into generated HTTP coordination and manual live runtimes:

- session resume, subscription sync, catch-up, and event acknowledgement remain exposed directly
- TypeScript and Flutter both ship `sdk.connect(...)` from their manual-owned consumer packages
- TypeScript and Flutter now both keep `ImWebSocketAuthOptions.automatic()` as the standard
  default
- TypeScript automatic auth resolves to query bearer on the default browser `WebSocket` path and
  to header bearer when a custom `webSocketFactory` is present
- Flutter automatic auth resolves to header bearer on native runtimes and query bearer fallback on
  Flutter Web
- TypeScript browser runtimes can still prefer `sdk.connect({ url })` when the gateway issues a
  fully pre-signed realtime URL instead of a credential exchange flow
- Flutter Web should prefer `credentialProvider` with a short-lived realtime ticket when the
  gateway supports browser-safe WebSocket token exchange
- Rust and the transport-standardized languages still remain HTTP-coordination-only today

## Related Pages

- [TypeScript Quick Start](/sdk/typescript-quick-start)
- [Flutter Quick Start](/sdk/flutter-quick-start)
- [Rust Quick Start](/sdk/rust-quick-start)
- [Module Map](/sdk/module-map)
