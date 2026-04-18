# sdkwork-craw-chat-sdk (Python)

Professional Python SDK for SDKWork API.

## Installation

```bash
pip install sdkwork-craw-chat-backend-sdk
```

## Quick Start

```python
from sdkwork_craw_chat_backend_sdk import SdkworkBackendClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18090",
)

client = SdkworkBackendClient(config)
client.set_api_key("your-api-key")

# Use the SDK
result = client.auth.me()
```

## Authentication Modes (Mutually Exclusive)

Choose exactly one mode for the same client instance.

### Mode A: API Key

```python
config = SdkConfig(base_url="http://127.0.0.1:18090")
client = SdkworkBackendClient(config)
client.set_api_key("your-api-key")
# Sends: Authorization: Bearer <apiKey>
```

### Mode B: Dual Token

```python
config = SdkConfig(base_url="http://127.0.0.1:18090")
client = SdkworkBackendClient(config)
client.set_auth_token("your-auth-token")
client.set_access_token("your-access-token")
# Sends:
# Authorization: Bearer <authToken>
# Access-Token: <accessToken>
```

> Do not call `set_api_key(...)` together with `set_auth_token(...)` + `set_access_token(...)` on the same client.

## Configuration (Non-Auth)

```python
from sdkwork_craw_chat_backend_sdk import SdkworkBackendClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18090",
)

client = SdkworkBackendClient(config)
client.set_header('X-Custom-Header', 'value')
```

## API Modules

- `client.auth` - auth API
- `client.portal` - portal API
- `client.session` - session API
- `client.presence` - presence API
- `client.realtime` - realtime API
- `client.device` - device API
- `client.inbox` - inbox API
- `client.conversation` - conversation API
- `client.message` - message API
- `client.media` - media API
- `client.stream` - stream API
- `client.rtc` - rtc API

## Usage Examples

### auth

```python
# Read the current portal session
result = client.auth.me()
print(result)
```

### portal

```python
# Read the tenant portal home snapshot
result = client.portal.get_home()
print(result)
```

### session

```python
# Resume the current app session
body = {
    'deviceId': 'deviceId',
    'lastSeenSyncSeq': 1,
}
result = client.session.resume(body)
print(result)
```

### presence

```python
# Get current presence
result = client.presence.get_presence_me()
print(result)
```

### realtime

```python
# Pull realtime events for the current device
params = {
    'afterSeq': 1,
    'limit': 2,
}
result = client.realtime.list_realtime_events(params)
print(result)
```

### device

```python
# Register the current device
body = {
    'deviceId': 'deviceId',
}
result = client.device.register(body)
print(result)
```

### inbox

```python
# Get inbox entries
result = client.inbox.get_inbox()
print(result)
```

### conversation

```python
# Create a conversation
body = {
    'conversationId': 'conversationId',
    'conversationType': 'conversationType',
}
result = client.conversation.create_conversation(body)
print(result)
```

### message

```python
# Recall a posted message
message_id = '1'
result = client.message.recall(message_id)
print(result)
```

### media

```python
# Create a media upload record
body = {
    'mediaAssetId': 'mediaAssetId',
    'resource': {
        'id': 1,
        'uuid': 'uuid',
        'url': 'url',
        'bytes': [],
        'localFile': 'localFile',
        'base64': 'base64',
        'type': 'image',
        'mimeType': 'mimeType',
        'size': 1,
        'name': 'name',
        'extension': 'extension',
        'tags': {},
        'metadata': {},
        'prompt': 'prompt',
    },
}
result = client.media.create_media_upload(body)
print(result)
```

### stream

```python
# Open a stream session
body = {
    'streamId': 'streamId',
    'streamType': 'streamType',
    'scopeKind': 'scopeKind',
    'scopeId': 'scopeId',
    'durabilityClass': 'transient',
    'schemaRef': 'schemaRef',
}
result = client.stream.open(body)
print(result)
```

### rtc

```python
# Create an RTC session
body = {
    'rtcSessionId': 'rtcSessionId',
    'conversationId': 'conversationId',
    'rtcMode': 'rtcMode',
}
result = client.rtc.create_rtc_session(body)
print(result)
```

## Error Handling

```python
try:
    client.auth.me()
except Exception as error:
    print(f"Error: {error}")
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

> Set `PYPI_TOKEN` for release (or `TEST_PYPI_TOKEN` for test channel).

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
