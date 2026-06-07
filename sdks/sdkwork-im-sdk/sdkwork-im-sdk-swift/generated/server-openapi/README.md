# sdkwork-im-sdk (Swift)

Generated SDKWork v3 dual-token transport SDK.

## Installation

Add to `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/sdkwork/ImSdkGenerated", from: "0.1.0")
]
```

## Quick Start

```swift
import ImSDK
import SDKworkCommon

let config = SdkConfig(baseUrl: "http://127.0.0.1:18090")
let client = SdkworkImClient(config: config)
client.setAuthToken("your-auth-token")
client.setAccessToken("your-access-token")

// Use the SDK
let result = try await client.presence.meRetrieve()
print(result)
```

## Authentication

```text
Authorization: Bearer <authToken>
Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```swift
let config = SdkConfig(baseUrl: "http://127.0.0.1:18090")
let client = SdkworkImClient(config: config)

// Set custom headers
client.setHeader("X-Custom-Header", value: "value")
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

```swift
// Resume a device runtime session
let body = ResumeDeviceSessionRequest(
    deviceId: "1",
    lastSeenSyncSeq: 2
)
let result = try await client.device.sessionsResume(body: body)
print(result)
```

### presence

```swift
// Retrieve current principal presence
let result = try await client.presence.meRetrieve()
print(result)
```

### realtime

```swift
// List pending realtime events
let params: [String: Any] = [
    "limit": 1,
    "cursor": "cursor"
]
let result = try await client.realtime.eventsList(params: params)
print(result)
```

### rtc

```swift
// Create an IM-backed RTC session
let body = CreateRtcSessionRequest(
    conversationId: "1",
    mediaKind: "mediakind"
)
let result = try await client.rtc.sessionsCreate(body: body)
print(result)
```

### social

```swift
// List contact tags
let params: [String: Any] = [
    "limit": 1,
    "cursor": "cursor"
]
let result = try await client.social.contactsTagsList(params: params)
print(result)
```

### chat

```swift
// List IM contacts
let params: [String: Any] = [
    "limit": 1,
    "cursor": "cursor"
]
let result = try await client.chat.contactsList(params: params)
print(result)
```

### streams

```swift
// Open a stream
let body = OpenStreamRequest(
    streamType: "streamtype",
    conversationId: "1"
)
let result = try await client.streams.create(body: body)
print(result)
```

## Error Handling

```swift
do {
    try await client.presence.meRetrieve()
} catch {
    print("Error: \(error)")
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

> Set `SWIFT_RELEASE_TAG` (or `SDKWORK_RELEASE_TAG`) for tag-based release.

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
