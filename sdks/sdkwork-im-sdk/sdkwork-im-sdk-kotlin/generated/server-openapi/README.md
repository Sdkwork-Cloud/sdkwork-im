# sdkwork-im-sdk (Kotlin)

Generator-owned Kotlin transport SDK for sdkwork-im-sdk.

## Installation

Add to your `build.gradle.kts`:

```kotlin
implementation("com.sdkwork:im-sdk-generated:0.1.0")
```

Or with Gradle Groovy:

```groovy
implementation 'com.sdkwork:im-sdk-generated:0.1.0'
```

## Quick Start

```kotlin
import com.sdkwork.im.sdk.generated.SdkworkImClient
import com.sdkwork.im.sdk.generated.*
import com.sdkwork.common.core.SdkConfig
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    val config = SdkConfig(baseUrl = "http://127.0.0.1:18090")
    val client = SdkworkImClient(config)
    // Attach the authenticated SDKWork session tokens
        client.setAuthToken("your-auth-token");
        client.setAccessToken("your-access-token");

    // Use the SDK
    val result = client.presence.meRetrieve()
    println(result)
}
```

## Dual Token Authentication

```
client.setAuthToken("your-auth-token")
client.setAccessToken("your-access-token")
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```


## Configuration (Non-Auth)

```kotlin
val config = SdkConfig(baseUrl = "http://127.0.0.1:18090")
val client = SdkworkImClient(config)
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

```kotlin
// Resume a device runtime session
val body = ResumeDeviceSessionRequest(
    deviceId = "1",
    lastSeenSyncSeq = 2
)
val result = client.device.sessionsResume(body)
println(result)
```

### presence

```kotlin
// Retrieve current principal presence
val result = client.presence.meRetrieve()
println(result)
```

### realtime

```kotlin
// List pending realtime events
val params = linkedMapOf<String, Any>(
    "limit" to 1,
    "cursor" to "cursor"
)
val result = client.realtime.eventsList(params)
println(result)
```

### rtc

```kotlin
// Create an IM-backed RTC session
val body = CreateRtcSessionRequest(
    conversationId = "1",
    mediaKind = "mediakind"
)
val result = client.rtc.sessionsCreate(body)
println(result)
```

### social

```kotlin
// List contact tags
val params = linkedMapOf<String, Any>(
    "limit" to 1,
    "cursor" to "cursor"
)
val result = client.social.contactsTagsList(params)
println(result)
```

### chat

```kotlin
// List IM contacts
val params = linkedMapOf<String, Any>(
    "limit" to 1,
    "cursor" to "cursor"
)
val result = client.chat.contactsList(params)
println(result)
```

### streams

```kotlin
// Open a stream
val body = OpenStreamRequest(
    streamType = "streamtype",
    conversationId = "1"
)
val result = client.streams.create(body)
println(result)
```

## Error Handling

```kotlin
import kotlinx.coroutines.runBlocking

fun main() = runBlocking {
    try {
        val result = client.presence.meRetrieve()
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
