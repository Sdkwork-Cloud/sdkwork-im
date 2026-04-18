# sdkwork-craw-chat-sdk (Java)

Professional Java SDK for SDKWork API.

## Installation

Add to your `pom.xml`:

```xml
<dependency>
    <groupId>com.sdkwork</groupId>
    <artifactId>craw-chat-backend-sdk</artifactId>
    <version>0.1.1</version>
</dependency>
```

Or with Gradle:

```groovy
implementation 'com.sdkwork:craw-chat-backend-sdk:0.1.1'
```

## Quick Start

```java
import com.sdkwork.craw.chat.backend.SdkworkBackendClient;
import com.sdkwork.common.core.Types;
import com.sdkwork.craw.chat.backend.model.*;

public class Main {
    public static void main(String[] args) throws Exception {
        Types.SdkConfig config = new Types.SdkConfig("http://127.0.0.1:18090");
        SdkworkBackendClient client = new SdkworkBackendClient(config);
        client.setApiKey("your-api-key");

        // Use the SDK
        PortalMeResponse result = client.getAuth().me();
        System.out.println(result);
    }
}
```

## Authentication Modes (Mutually Exclusive)

Choose exactly one mode for the same client instance.

### Mode A: API Key

```java
Types.SdkConfig config = new Types.SdkConfig("http://127.0.0.1:18090");
SdkworkBackendClient client = new SdkworkBackendClient(config);
client.setApiKey("your-api-key");
// Sends: Authorization: Bearer <apiKey>
```

### Mode B: Dual Token

```java
Types.SdkConfig config = new Types.SdkConfig("http://127.0.0.1:18090");
SdkworkBackendClient client = new SdkworkBackendClient(config);
client.setAuthToken("your-auth-token");
client.setAccessToken("your-access-token");
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```

> Do not call `setApiKey(...)` together with `setAuthToken(...)` + `setAccessToken(...)` on the same client.

## Configuration (Non-Auth)

```java
Types.SdkConfig config = new Types.SdkConfig("http://127.0.0.1:18090");
SdkworkBackendClient client = new SdkworkBackendClient(config);

// Set custom headers
client.getHttpClient().setHeader("X-Custom-Header", "value");
```

## API Modules

- `client.getAuth()` - auth API
- `client.getPortal()` - portal API
- `client.getSession()` - session API
- `client.getPresence()` - presence API
- `client.getRealtime()` - realtime API
- `client.getDevice()` - device API
- `client.getInbox()` - inbox API
- `client.getConversation()` - conversation API
- `client.getMessage()` - message API
- `client.getMedia()` - media API
- `client.getStream()` - stream API
- `client.getRtc()` - rtc API

## Usage Examples

### auth

```java
// Read the current portal session
PortalMeResponse result = client.getAuth().me();
System.out.println(result);
```

### portal

```java
// Read the tenant portal home snapshot
Map<String, Object> result = client.getPortal().getHome();
System.out.println(result);
```

### session

```java
// Resume the current app session
ResumeSessionRequest body = new ResumeSessionRequest();
body.setDeviceId("1");
body.setLastSeenSyncSeq(2);
SessionResumeView result = client.getSession().resume(body);
System.out.println(result);
```

### presence

```java
// Get current presence
PresenceSnapshotView result = client.getPresence().getPresenceMe();
System.out.println(result);
```

### realtime

```java
// Pull realtime events for the current device
Map<String, Object> params = new LinkedHashMap<>();
params.put("afterSeq", 1);
params.put("limit", 2);
RealtimeEventWindow result = client.getRealtime().listRealtimeEvents(params);
System.out.println(result);
```

### device

```java
// Register the current device
RegisterDeviceRequest body = new RegisterDeviceRequest();
body.setDeviceId("1");
RegisteredDeviceView result = client.getDevice().register(body);
System.out.println(result);
```

### inbox

```java
// Get inbox entries
InboxResponse result = client.getInbox().getInbox();
System.out.println(result);
```

### conversation

```java
// Create a conversation
CreateConversationRequest body = new CreateConversationRequest();
body.setConversationId("1");
body.setConversationType("conversationtype");
CreateConversationResult result = client.getConversation().createConversation(body);
System.out.println(result);
```

### message

```java
// Recall a posted message
String messageId = "1";
MessageMutationResult result = client.getMessage().recall(messageId);
System.out.println(result);
```

### media

```java
// Create a media upload record
CreateUploadRequest body = new CreateUploadRequest();
body.setMediaAssetId("1");
body.setResource(new MediaResource());
MediaAsset result = client.getMedia().createMediaUpload(body);
System.out.println(result);
```

### stream

```java
// Open a stream session
OpenStreamRequest body = new OpenStreamRequest();
body.setStreamId("1");
body.setStreamType("streamtype");
body.setScopeKind("scopekind");
body.setScopeId("1");
body.setDurabilityClass("transient");
body.setSchemaRef("schemaref");
StreamSession result = client.getStream().open_(body);
System.out.println(result);
```

### rtc

```java
// Create an RTC session
CreateRtcSessionRequest body = new CreateRtcSessionRequest();
body.setRtcSessionId("1");
body.setConversationId("1");
body.setRtcMode("rtcmode");
RtcSession result = client.getRtc().createRtcSession(body);
System.out.println(result);
```

## Error Handling

```java
try {
    PortalMeResponse result = client.getAuth().me();
    System.out.println(result);
} catch (Exception e) {
    System.err.println("Error: " + e.getMessage());
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

> Use Maven `settings.xml` credentials and optional `MAVEN_PUBLISH_PROFILE`.

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
