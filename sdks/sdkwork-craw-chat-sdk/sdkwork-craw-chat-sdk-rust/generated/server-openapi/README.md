# sdkwork-craw-chat-sdk (Rust)

Professional Rust SDK for SDKWork API.

## Installation

```bash
cargo add sdkwork-craw-chat-backend-sdk
```

## Quick Start

```rust
use sdkwork_craw_chat_backend_sdk::{SdkworkBackendClient, SdkworkConfig};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SdkworkBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18090"))?;
    client.set_api_key("your-api-key");

    let result = client.auth().me().await?;
    println!("{result:?}");
    Ok(())
}
```

## Authentication Modes (Mutually Exclusive)

Choose exactly one mode for the same client instance.

### Mode A: API Key

```rust
let client = SdkworkBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18090"))?;
client.set_api_key("your-api-key");
// Sends: Authorization: Bearer <apiKey>
```

### Mode B: Dual Token

```rust
let client = SdkworkBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18090"))?;
client.set_auth_token("your-auth-token");
client.set_access_token("your-access-token");
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```

> Do not call `set_api_key(...)` together with `set_auth_token(...)` + `set_access_token(...)` on the same client.

## Configuration (Non-Auth)

```rust
let client = SdkworkBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18090"))?;
client.set_header("X-Custom-Header", "value");
```

## API Modules

- `client.auth()` - auth API
- `client.portal()` - portal API
- `client.session()` - session API
- `client.presence()` - presence API
- `client.realtime()` - realtime API
- `client.device()` - device API
- `client.inbox()` - inbox API
- `client.conversation()` - conversation API
- `client.message()` - message API
- `client.media()` - media API
- `client.stream()` - stream API
- `client.rtc()` - rtc API

## Usage Examples

### auth

```rust
// Read the current portal session
let result = client.auth().me().await?;
println!("{result:?}");
```

### portal

```rust
// Read the tenant portal home snapshot
let result = client.portal().get_home().await?;
println!("{result:?}");
```

### session

```rust
use sdkwork_craw_chat_backend_sdk::*;
// Resume the current app session
let body = ResumeSessionRequest {
    device_id: Some("1".to_string()),
    last_seen_sync_seq: Some(2_i64),
    ..Default::default()
};
let result = client.session().resume(&body).await?;
println!("{result:?}");
```

### presence

```rust
// Get current presence
let result = client.presence().get_presence_me().await?;
println!("{result:?}");
```

### realtime

```rust
use std::collections::HashMap;
// Pull realtime events for the current device
let mut query = HashMap::new();
query.insert("afterSeq".to_string(), serde_json::json!(1));
query.insert("limit".to_string(), serde_json::json!(2));
let result = client.realtime().list_realtime_events(Some(&query)).await?;
println!("{result:?}");
```

### device

```rust
use sdkwork_craw_chat_backend_sdk::*;
// Register the current device
let body = RegisterDeviceRequest {
    device_id: Some("1".to_string()),
    ..Default::default()
};
let result = client.device().register(&body).await?;
println!("{result:?}");
```

### inbox

```rust
// Get inbox entries
let result = client.inbox().get_inbox().await?;
println!("{result:?}");
```

### conversation

```rust
use sdkwork_craw_chat_backend_sdk::*;
// Create a conversation
let body = CreateConversationRequest {
    conversation_id: "1".to_string(),
    conversation_type: "conversationtype".to_string(),
    ..Default::default()
};
let result = client.conversation().create_conversation(&body).await?;
println!("{result:?}");
```

### message

```rust
// Recall a posted message
let message_id = "1";
let result = client.message().recall(message_id).await?;
println!("{result:?}");
```

### media

```rust
use sdkwork_craw_chat_backend_sdk::*;
// Create a media upload record
let body = CreateUploadRequest {
    media_asset_id: "1".to_string(),
    resource: MediaResource::default(),
    ..Default::default()
};
let result = client.media().create_media_upload(&body).await?;
println!("{result:?}");
```

### stream

```rust
use sdkwork_craw_chat_backend_sdk::*;
// Open a stream session
let body = OpenStreamRequest {
    stream_id: "1".to_string(),
    stream_type: "streamtype".to_string(),
    scope_kind: "scopekind".to_string(),
    scope_id: "1".to_string(),
    durability_class: "transient".to_string(),
    schema_ref: Some("schemaref".to_string()),
    ..Default::default()
};
let result = client.stream().open(&body).await?;
println!("{result:?}");
```

### rtc

```rust
use sdkwork_craw_chat_backend_sdk::*;
// Create an RTC session
let body = CreateRtcSessionRequest {
    rtc_session_id: "1".to_string(),
    conversation_id: Some("1".to_string()),
    rtc_mode: "rtcmode".to_string(),
    ..Default::default()
};
let result = client.rtc().create_rtc_session(&body).await?;
println!("{result:?}");
```

## Error Handling

```rust
use sdkwork_craw_chat_backend_sdk::{SdkworkBackendClient, SdkworkConfig};


let client = SdkworkBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18090"))?;

let outcome: Result<(), _> = async {
    client.auth().me().await?;
    Ok(())
}.await;

match outcome {
    Ok(()) => println!("request completed"),
    Err(error) => eprintln!("request failed: {error}"),
}
```

## Publishing

This SDK includes cross-platform publish scripts in `bin/`:
- `bin/publish-core.mjs`
- `bin/publish.sh`
- `bin/publish.ps1`

### Check

```bash
./bin/publish.sh --action check
```

### Publish

```bash
./bin/publish.sh --action publish --channel release
```

```powershell
.\bin\publish.ps1 --action publish --channel test --dry-run
```

> Set cargo registry credentials before `cargo publish` and use `--dry-run` first.

## License

MIT

## Regeneration Contract

- Generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- Each run also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- Apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across regenerations.
- If a generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
