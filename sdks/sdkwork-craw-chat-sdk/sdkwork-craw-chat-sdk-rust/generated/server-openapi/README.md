# sdkwork-craw-chat-sdk (Rust)

Professional Rust transport SDK for the Craw Chat app API.

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
    client.set_auth_token("your-bearer-token");

    let result = client.inbox().get_inbox().await?;
    println!("{result:?}");
    Ok(())
}
```

## Authentication

Craw Chat app routes use bearer authentication only.

```rust
use sdkwork_craw_chat_backend_sdk::{SdkworkBackendClient, SdkworkConfig};

let client = SdkworkBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18090"))?;
client.set_auth_token("your-bearer-token");
// Sends: Authorization: Bearer <token>
```

## Configuration

```rust
use sdkwork_craw_chat_backend_sdk::{SdkworkBackendClient, SdkworkConfig};

let client = SdkworkBackendClient::new(SdkworkConfig::new("http://127.0.0.1:18090"))?;
client.set_header("X-Custom-Header", "value");
```

## API Modules

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

## Publishing

This SDK includes cross-platform publish scripts in `bin/`:

- `bin/publish-core.mjs`
- `bin/publish.sh`
- `bin/publish.ps1`

## License

MIT

## Package Boundary

- Use only the crate root entrypoint: `sdkwork_craw_chat_backend_sdk`.
- Internal generated module paths are not part of the supported public API.
- The workspace normalization wrapper strips generator-only auth scaffolding before verification and packaging.

## Regeneration Contract

- Generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- Each run also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- Apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across regenerations.
- If a generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
