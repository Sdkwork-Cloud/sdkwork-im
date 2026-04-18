# Rust Quick Start

## Audience

Use this page when you are integrating Craw Chat from a Rust backend, service, or systems-oriented
application and want the preferred composed crate.

## Crate

- preferred public crate: `craw-chat-sdk`
- generated transport crate: `sdkwork-craw-chat-backend-sdk`
- workspace path: `sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-rust/composed`

## Add the dependency

If you are consuming from a local checkout before public publication, point Cargo at the composed
crate directly:

```toml
[dependencies]
craw-chat-sdk = { path = "../../sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-rust/composed" }
```

## Create a client

```rust
use craw_chat_sdk::CrawChatClient;

let client = CrawChatClient::new_with_base_url("http://127.0.0.1:18090")?;
client.set_auth_token(token);
```

## First read call

```rust
let inbox = client.inbox().list().await?;
```

## First write call

```rust
use craw_chat_sdk::RegisterDeviceRequest;

client
  .devices()
  .register(RegisterDeviceRequest {
    device_id: Some("device-rust-01".into()),
  })
  .await?;
```

## Common module entrypoints

```rust
use craw_chat_sdk::{PostTextOptions, ResumeSessionRequest};

client
  .session()
  .resume(ResumeSessionRequest {
    device_id: Some("device-rust-01".into()),
    last_seen_sync_seq: Some(0),
  })
  .await?;

client.presence().current().await?;
client
  .conversations()
  .post_text("conv-demo-01", "hello from Rust", PostTextOptions::default())
  .await?;
```

## Builder helpers

The crate re-exports builder helpers for common payload creation:

```rust
use craw_chat_sdk::{
  JsonRtcSignalOptions,
  PostTextOptions,
  TextFrameOptions,
  build_json_rtc_signal,
  build_text_message,
  build_text_stream_frame,
};
```

## Next Steps

- [Auth and Client Init](/sdk/auth-and-client-init)
- [Module Map](/sdk/module-map)
- [Messages Module](/sdk/modules/messages)
- [Stream and RTC](/sdk/examples/stream-and-rtc)
