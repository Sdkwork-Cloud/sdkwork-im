# CrawChatBackendSdk

Generator-owned Swift transport SDK for the Craw Chat app API.

## Installation

Add to `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/sdkwork/craw-chat-backend-sdk-swift", from: "0.1.1")
]
```

## Quick Start

```swift
import CrawChatBackendSdk
import SDKworkCommon

let config = SdkConfig(baseUrl: "http://127.0.0.1:18090")
let client = SdkworkBackendClient(config: config)
client.setApiKey("your-api-key")

// Use the SDK
let result = try await client.auth.me()
print(result)
```

## Authentication Modes (Mutually Exclusive)

Choose exactly one mode for the same client instance.

### Mode A: API Key

```swift
let config = SdkConfig(baseUrl: "http://127.0.0.1:18090")
let client = SdkworkBackendClient(config: config)
client.setApiKey("your-api-key")
// Sends: Authorization: Bearer <apiKey>
```

### Mode B: Dual Token

```swift
let config = SdkConfig(baseUrl: "http://127.0.0.1:18090")
let client = SdkworkBackendClient(config: config)
client.setAuthToken("your-auth-token")
client.setAccessToken("your-access-token")
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```

> Do not call `setApiKey(...)` together with `setAuthToken(...)` + `setAccessToken(...)` on the same client.

## Configuration (Non-Auth)

```swift
let config = SdkConfig(baseUrl: "http://127.0.0.1:18090")
let client = SdkworkBackendClient(config: config)

// Set custom headers
client.setHeader("X-Custom-Header", value: "value")
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

```swift
// Read the current portal session
let result = try await client.auth.me()
print(result)
```

### portal

```swift
// Read the tenant portal home snapshot
let result = try await client.portal.getHome()
print(result)
```

### session

```swift
// Resume the current app session
let body = ResumeSessionRequest(
    deviceId: "1",
    lastSeenSyncSeq: 2
)
let result = try await client.session.resume(body: body)
print(result)
```

### presence

```swift
// Get current presence
let result = try await client.presence.getPresenceMe()
print(result)
```

### realtime

```swift
// Pull realtime events for the current device
let params: [String: Any] = [
    "afterSeq": 1,
    "limit": 2
]
let result = try await client.realtime.listRealtimeEvents(params: params)
print(result)
```

### device

```swift
// Register the current device
let body = RegisterDeviceRequest(deviceId: "1")
let result = try await client.device.register(body: body)
print(result)
```

### inbox

```swift
// Get inbox entries
let result = try await client.inbox.getInbox()
print(result)
```

### conversation

```swift
// Create a conversation
let body = CreateConversationRequest(
    conversationId: "1",
    conversationType: "conversationtype"
)
let result = try await client.conversation.createConversation(body: body)
print(result)
```

### message

```swift
// Recall a posted message
let messageId = "1"
let result = try await client.message.recall(messageId: messageId)
print(result)
```

### media

```swift
// Create a media upload record
let body = CreateUploadRequest(
    mediaAssetId: "1",
    resource: MediaResource()
)
let result = try await client.media.createMediaUpload(body: body)
print(result)
```

### stream

```swift
// Open a stream session
let body = OpenStreamRequest(
    streamId: "1",
    streamType: "streamtype",
    scopeKind: "scopekind",
    scopeId: "1",
    durabilityClass: "transient",
    schemaRef: "schemaref"
)
let result = try await client.stream.open_(body: body)
print(result)
```

### rtc

```swift
// Create an RTC session
let body = CreateRtcSessionRequest(
    rtcSessionId: "1",
    conversationId: "1",
    rtcMode: "rtcmode"
)
let result = try await client.rtc.createRtcSession(body: body)
print(result)
```

## Error Handling

```swift
do {
    try await client.auth.me()
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
