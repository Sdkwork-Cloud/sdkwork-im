# sdkwork-craw-chat-sdk (C#)

Professional C# SDK for SDKWork API.

## Installation

```bash
dotnet add package Sdkwork.CrawChat.BackendSdk
```

Or add to your `.csproj`:

```xml
<PackageReference Include="Sdkwork.CrawChat.BackendSdk" Version="0.1.1" />
```

## Quick Start

```csharp
using Sdkwork.CrawChat.BackendSdk.Models;
using Sdkwork.CrawChat.BackendSdk;
using SDKwork.Common.Core;

var config = new SdkConfig("http://127.0.0.1:18090");
var client = new SdkworkBackendClient(config);
client.SetApiKey("your-api-key");

var result = await client.Auth.MeAsync();
Console.WriteLine(result);
```

## Authentication Modes (Mutually Exclusive)

Choose exactly one mode for the same client instance.

### Mode A: API Key

```csharp
var config = new SdkConfig("http://127.0.0.1:18090");
var client = new SdkworkBackendClient(config);
client.SetApiKey("your-api-key");
// Sends: Authorization: Bearer <apiKey>
```

### Mode B: Dual Token

```csharp
var config = new SdkConfig("http://127.0.0.1:18090");
var client = new SdkworkBackendClient(config);
client.SetAuthToken("your-auth-token");
client.SetAccessToken("your-access-token");
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```

> Do not call `SetApiKey(...)` together with `SetAuthToken(...)` + `SetAccessToken(...)` on the same client.

## Configuration (Non-Auth)

```csharp
var config = new SdkConfig("http://127.0.0.1:18090");
var client = new SdkworkBackendClient(config);

// Set custom headers
client.SetHeader("X-Custom-Header", "value");
```

## API Modules

- `client.Auth` - auth API
- `client.Portal` - portal API
- `client.Session` - session API
- `client.Presence` - presence API
- `client.Realtime` - realtime API
- `client.Device` - device API
- `client.Inbox` - inbox API
- `client.Conversation` - conversation API
- `client.Message` - message API
- `client.Media` - media API
- `client.Stream` - stream API
- `client.Rtc` - rtc API

## Usage Examples

### auth

```csharp
// Read the current portal session
var result = await client.Auth.MeAsync();
Console.WriteLine(result);
```

### portal

```csharp
// Read the tenant portal home snapshot
var result = await client.Portal.GetHomeAsync();
Console.WriteLine(result);
```

### session

```csharp
// Resume the current app session
var body = new ResumeSessionRequest
{
    DeviceId = "1",
    LastSeenSyncSeq = 2,
};
var result = await client.Session.ResumeAsync(body);
Console.WriteLine(result);
```

### presence

```csharp
// Get current presence
var result = await client.Presence.GetPresenceMeAsync();
Console.WriteLine(result);
```

### realtime

```csharp
// Pull realtime events for the current device
var query = new Dictionary<string, object>
{
    ["afterSeq"] = 1,
    ["limit"] = 2,
};
var result = await client.Realtime.ListRealtimeEventsAsync(query);
Console.WriteLine(result);
```

### device

```csharp
// Register the current device
var body = new RegisterDeviceRequest
{
    DeviceId = "1",
};
var result = await client.Device.RegisterAsync(body);
Console.WriteLine(result);
```

### inbox

```csharp
// Get inbox entries
var result = await client.Inbox.GetInboxAsync();
Console.WriteLine(result);
```

### conversation

```csharp
// Create a conversation
var body = new CreateConversationRequest
{
    ConversationId = "1",
    ConversationType = "conversationtype",
};
var result = await client.Conversation.CreateConversationAsync(body);
Console.WriteLine(result);
```

### message

```csharp
// Recall a posted message
var messageId = "1";
var result = await client.Message.RecallAsync(messageId);
Console.WriteLine(result);
```

### media

```csharp
// Create a media upload record
var body = new CreateUploadRequest
{
    MediaAssetId = "1",
    Resource = new MediaResource(),
};
var result = await client.Media.CreateMediaUploadAsync(body);
Console.WriteLine(result);
```

### stream

```csharp
// Open a stream session
var body = new OpenStreamRequest
{
    StreamId = "1",
    StreamType = "streamtype",
    ScopeKind = "scopekind",
    ScopeId = "1",
    DurabilityClass = "transient",
    SchemaRef = "schemaref",
};
var result = await client.Stream.OpenAsync(body);
Console.WriteLine(result);
```

### rtc

```csharp
// Create an RTC session
var body = new CreateRtcSessionRequest
{
    RtcSessionId = "1",
    ConversationId = "1",
    RtcMode = "rtcmode",
};
var result = await client.Rtc.CreateRtcSessionAsync(body);
Console.WriteLine(result);
```

## Error Handling

```csharp
try
{
    await client.Auth.MeAsync();
}
catch (HttpRequestException ex)
{
    Console.WriteLine($"Error: {ex.Message}");
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

> Set `NUGET_API_KEY` for release (or `NUGET_TEST_API_KEY` for test channel).

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
