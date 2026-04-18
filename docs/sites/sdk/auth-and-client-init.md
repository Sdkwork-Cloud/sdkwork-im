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
| TypeScript | `CrawChatClient.create({ backendConfig })` | `client.setAuthToken(token)` |
| Flutter | `CrawChatClient.create(...)` | `client.setAuthToken(token)` |
| Rust | `CrawChatClient::new_with_base_url(...)` | `client.set_auth_token(token)` |

All three languages also expose the generated transport layer, but the preferred integration surface
is the manual-owned `composed` layer.

## Shared Initialization Flow

1. resolve the app base URL
2. create the composed `CrawChatClient`
3. set or inject the bearer token
4. route work through the semantic modules
5. drop to the App API reference only when you need exact payload or operation detail

## TypeScript

```ts
import { CrawChatClient, type SdkworkBackendConfig } from "@sdkwork/craw-chat-sdk";

const backendConfig: SdkworkBackendConfig = {
  baseUrl: "http://127.0.0.1:18090",
  authToken: process.env.CRAW_CHAT_TOKEN,
};

const client = await CrawChatClient.create({ backendConfig });
const session = await client.session.resume({
  deviceId: "device-web-01",
  lastSeenSyncSeq: 0,
});
```

## Flutter

```dart
import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final client = CrawChatClient.create(
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
use craw_chat_sdk::{CrawChatClient, ResumeSessionRequest};

let client = CrawChatClient::new_with_base_url("http://127.0.0.1:18090")?;
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
- `composed` is the manual-owned client surface documented here
- if a generated behavior needs to change, change the contract or generator inputs and regenerate

## Realtime Transport Note

The current SDK round is HTTP-coordination-first:

- session resume, subscription sync, event pull, and event acknowledgement are exposed directly
- WebSocket transport semantics may still be documented for system understanding
- a manual realtime WebSocket adapter is not implied unless it is explicitly documented as
  implemented

## Related Pages

- [TypeScript Quick Start](/sdk/typescript-quick-start)
- [Flutter Quick Start](/sdk/flutter-quick-start)
- [Rust Quick Start](/sdk/rust-quick-start)
- [Module Map](/sdk/module-map)
