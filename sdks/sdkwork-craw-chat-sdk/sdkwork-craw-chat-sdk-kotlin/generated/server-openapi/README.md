# sdkwork-craw-chat-sdk (Kotlin)

Professional Kotlin SDK for SDKWork API.

## Installation

Add to your `build.gradle.kts`:

```kotlin
implementation("com.sdkwork:craw-chat-backend-sdk:0.1.1")
```

Or with Gradle Groovy:

```groovy
implementation 'com.sdkwork:craw-chat-backend-sdk:0.1.1'
```

## Quick Start

```kotlin
import com.sdkwork.craw.chat.backend.SdkworkBackendClient
import com.sdkwork.craw.chat.backend.*
import com.sdkwork.common.core.SdkConfig
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val config = SdkConfig(baseUrl = "http://127.0.0.1:18090")
    val client = SdkworkBackendClient(config)
    client.setApiKey("your-api-key")

    // Use the SDK
    val result = client.auth.me()
    println(result)
}
```

## Authentication Modes (Mutually Exclusive)

Choose exactly one mode for the same client instance.

### Mode A: API Key

```kotlin
val config = SdkConfig(baseUrl = "http://127.0.0.1:18090")
val client = SdkworkBackendClient(config)
client.setApiKey("your-api-key")
// Sends: Authorization: Bearer <apiKey>
```

### Mode B: Dual Token

```kotlin
val config = SdkConfig(baseUrl = "http://127.0.0.1:18090")
val client = SdkworkBackendClient(config)
client.setAuthToken("your-auth-token")
client.setAccessToken("your-access-token")
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```

> Do not call `setApiKey(...)` together with `setAuthToken(...)` + `setAccessToken(...)` on the same client.

## Configuration (Non-Auth)

```kotlin
val config = SdkConfig(baseUrl = "http://127.0.0.1:18090")
val client = SdkworkBackendClient(config)
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

```kotlin
// Read the current portal session
val result = client.auth.me()
println(result)
```

### portal

```kotlin
// Read the tenant portal home snapshot
val result = client.portal.getHome()
println(result)
```

### session

```kotlin
// Resume the current app session
val body = ResumeSessionRequest(
    deviceId = "1",
    lastSeenSyncSeq = 2
)
val result = client.session.resume(body)
println(result)
```

### presence

```kotlin
// Get current presence
val result = client.presence.getPresenceMe()
println(result)
```

### realtime

```kotlin
// Pull realtime events for the current device
val params = linkedMapOf<String, Any>(
    "afterSeq" to 1,
    "limit" to 2
)
val result = client.realtime.listRealtimeEvents(params)
println(result)
```

### device

```kotlin
// Register the current device
val body = RegisterDeviceRequest(
    deviceId = "1"
)
val result = client.device.register(body)
println(result)
```

### inbox

```kotlin
// Get inbox entries
val result = client.inbox.getInbox()
println(result)
```

### conversation

```kotlin
// Create a conversation
val body = CreateConversationRequest(
    conversationId = "1",
    conversationType = "conversationtype"
)
val result = client.conversation.createConversation(body)
println(result)
```

### message

```kotlin
// Recall a posted message
val messageId = "1"
val result = client.message.recall(messageId)
println(result)
```

### media

```kotlin
// Create a media upload record
val body = CreateUploadRequest(
    mediaAssetId = "1",
    resource = MediaResource()
)
val result = client.media.createMediaUpload(body)
println(result)
```

### stream

```kotlin
// Open a stream session
val body = OpenStreamRequest(
    streamId = "1",
    streamType = "streamtype",
    scopeKind = "scopekind",
    scopeId = "1",
    durabilityClass = "transient",
    schemaRef = "schemaref"
)
val result = client.stream.open_(body)
println(result)
```

### rtc

```kotlin
// Create an RTC session
val body = CreateRtcSessionRequest(
    rtcSessionId = "1",
    conversationId = "1",
    rtcMode = "rtcmode"
)
val result = client.rtc.createRtcSession(body)
println(result)
```

## Error Handling

```kotlin
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    try {
        val result = client.auth.me()
        println(result)
    } catch (e: Exception) {
        println("Error: ${e.message}")
    }
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

> Configure Gradle publishing credentials and optional `GRADLE_PUBLISH_TASK`.

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
