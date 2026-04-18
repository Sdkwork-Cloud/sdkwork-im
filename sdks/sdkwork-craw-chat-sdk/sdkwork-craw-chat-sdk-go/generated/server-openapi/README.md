# sdkwork-craw-chat-sdk (Go)

Professional Go SDK for SDKWork API.

## Installation

```bash
go get github.com/sdkwork/craw-chat-backend-sdk
```

## Quick Start

```go
package main

import (
    "fmt"
    "github.com/sdkwork/craw-chat-backend-sdk"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"

)

func main() {
    cfg := sdkhttp.NewDefaultConfig("http://127.0.0.1:18090")
    client := github.com/sdkwork/craw-chat-backend-sdk.NewSdkworkBackendClientWithConfig(cfg)
    client.SetApiKey("your-api-key")
    
    // Use the SDK
    result, err := client.Auth.Me()
    if err != nil {
        panic(err)
    }
    fmt.Println(result)
}
```

## Authentication Modes (Mutually Exclusive)

Choose exactly one mode for the same client instance.

### Mode A: API Key

```go
cfg := sdkhttp.NewDefaultConfig("http://127.0.0.1:18090")
client := github.com/sdkwork/craw-chat-backend-sdk.NewSdkworkBackendClientWithConfig(cfg)
client.SetApiKey("your-api-key")
// Sends: Authorization: Bearer <apiKey>
```

### Mode B: Dual Token

```go
cfg := sdkhttp.NewDefaultConfig("http://127.0.0.1:18090")
client := github.com/sdkwork/craw-chat-backend-sdk.NewSdkworkBackendClientWithConfig(cfg)
client.SetAuthToken("your-auth-token")
client.SetAccessToken("your-access-token")
// Sends:
// Authorization: Bearer <authToken>
// Access-Token: <accessToken>
```

> Do not call `SetApiKey(...)` together with `SetAuthToken(...)` + `SetAccessToken(...)` on the same client.

## Configuration (Non-Auth)

```go
cfg := sdkhttp.NewDefaultConfig("http://127.0.0.1:18090")
client := github.com/sdkwork/craw-chat-backend-sdk.NewSdkworkBackendClientWithConfig(cfg)

// Set custom headers
client.SetHeader("X-Custom-Header", "value")
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

```go
// Read the current portal session
result, err := client.Auth.Me()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### portal

```go
// Read the tenant portal home snapshot
result, err := client.Portal.GetHome()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### session

```go
// Resume the current app session
body := sdktypes.ResumeSessionRequest{
    DeviceId: "deviceId",
    LastSeenSyncSeq: 2,
}
result, err := client.Session.Resume(body)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### presence

```go
// Get current presence
result, err := client.Presence.GetPresenceMe()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### realtime

```go
// Pull realtime events for the current device
params := map[string]interface{}{
    "afterSeq": 1,
    "limit": 2,
}
result, err := client.Realtime.ListRealtimeEvents(params)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### device

```go
// Register the current device
body := sdktypes.RegisterDeviceRequest{
    DeviceId: "deviceId",
}
result, err := client.Device.Register(body)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### inbox

```go
// Get inbox entries
result, err := client.Inbox.GetInbox()
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### conversation

```go
// Create a conversation
body := sdktypes.CreateConversationRequest{
    ConversationId: "conversationId",
    ConversationType: "conversationType",
}
result, err := client.Conversation.CreateConversation(body)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### message

```go
// Recall a posted message
messageId := "1"
result, err := client.Message.Recall(messageId)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### media

```go
// Create a media upload record
body := sdktypes.CreateUploadRequest{
    MediaAssetId: "mediaAssetId",
    Resource: sdktypes.MediaResource{
    Id: 1,
    Uuid: "uuid",
    Url: "url",
    Bytes: []int{
    1,
},
    LocalFile: "localFile",
    Base64: "base64",
    Type: "image",
    MimeType: "mimeType",
    Size: 9,
    Name: "name",
    Extension: "extension",
    Tags: sdktypes.StringMap{},
    Metadata: sdktypes.StringMap{},
    Prompt: "prompt",
},
}
result, err := client.Media.CreateMediaUpload(body)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### stream

```go
// Open a stream session
body := sdktypes.OpenStreamRequest{
    StreamId: "streamId",
    StreamType: "streamType",
    ScopeKind: "scopeKind",
    ScopeId: "scopeId",
    DurabilityClass: "transient",
    SchemaRef: "schemaRef",
}
result, err := client.Stream.Open(body)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

### rtc

```go
// Create an RTC session
body := sdktypes.CreateRtcSessionRequest{
    RtcSessionId: "rtcSessionId",
    ConversationId: "conversationId",
    RtcMode: "rtcMode",
}
result, err := client.Rtc.CreateRtcSession(body)
if err != nil {
    panic(err)
}
fmt.Println(result)
```

## Error Handling

```go
_, err := client.Auth.Me()
if err != nil {
    // Handle error
    fmt.Println("Error:", err)
    return
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

> Set `GO_RELEASE_TAG` (or `SDKWORK_RELEASE_TAG`) and push tag if needed.

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
