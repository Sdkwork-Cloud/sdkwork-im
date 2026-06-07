# sdkwork-im-sdk (Python)

Generator-owned Python transport SDK for sdkwork-im-sdk.

## Installation

```bash
pip install sdkwork-im-sdk-generated
```

## Quick Start

```python
from sdkwork_im_sdk_generated import SdkworkImClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18090",
)

client = SdkworkImClient(config)
# Attach the authenticated SDKWork session tokens
    client.set_auth_token("your-auth-token")
    client.set_access_token("your-access-token")

# Use the SDK
result = client.presence.me.list()
```

## Dual Token Authentication

```
client.set_auth_token("your-auth-token")
client.set_access_token("your-access-token")
# Sends:
# Authorization: Bearer <authToken>
# Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```python
from sdkwork_im_sdk_generated import SdkworkImClient, SdkConfig

config = SdkConfig(
    base_url="http://127.0.0.1:18090",
)

client = SdkworkImClient(config)
client.set_header('X-Custom-Header', 'value')
```

## API Modules

- `client.device` - device API
- `client.presence` - presence API
- `client.realtime` - realtime API
- `client.rtc` - rtc API
- `client.social` - social API
- `client.chat` - chat API
- `client.streams` - streams API

## Usage Examples

### device

```python
# Resume a device runtime session
body = {
    'deviceId': 'deviceId',
    'lastSeenSyncSeq': 1,
}
result = client.device.sessions.resume(body)
print(result)
```

### presence

```python
# Retrieve current principal presence
result = client.presence.me.list()
print(result)
```

### realtime

```python
# List pending realtime events
params = {
    'limit': 1,
    'cursor': 'cursor',
}
result = client.realtime.events.list(params)
print(result)
```

### rtc

```python
# Create an IM-backed RTC session
body = {
    'conversationId': 'conversationId',
    'mediaKind': 'mediaKind',
}
result = client.rtc.sessions.create(body)
print(result)
```

### social

```python
# List contact tags
params = {
    'limit': 1,
    'cursor': 'cursor',
}
result = client.social.contacts.tags.list(params)
print(result)
```

### chat

```python
# List IM contacts
params = {
    'limit': 1,
    'cursor': 'cursor',
}
result = client.chat.contacts.list(params)
print(result)
```

### streams

```python
# Open a stream
body = {
    'streamType': 'streamType',
    'conversationId': 'conversationId',
}
result = client.streams.create(body)
print(result)
```

## Error Handling

```python
try:
    client.presence.me.list()
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
