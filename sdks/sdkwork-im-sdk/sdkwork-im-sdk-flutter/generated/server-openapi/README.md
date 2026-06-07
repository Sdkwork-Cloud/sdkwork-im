# sdkwork-im-sdk (Flutter)

Generator-owned Flutter transport SDK for sdkwork-im-sdk.

## Installation

Add to `pubspec.yaml`:

```yaml
dependencies:
  im_sdk_generated: ^0.1.0
```

## Quick Start

```dart
import 'package:im_sdk_generated/im_sdk_generated.dart';

final client = SdkworkImClient.withBaseUrl(baseUrl: 'http://127.0.0.1:18090');
// Attach the authenticated SDKWork session tokens
    client.setAuthToken("your-auth-token");
    client.setAccessToken("your-access-token");

// Use the SDK
final result = await client.presence.meRetrieve();
print(result);
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

```dart
final client = SdkworkImClient.withBaseUrl(baseUrl: 'http://127.0.0.1:18090');

// Set custom headers
client.setHeader('X-Custom-Header', 'value');
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
```dart
// Resume a device runtime session
final body = ResumeDeviceSessionRequest(
  deviceId: '1',
  lastSeenSyncSeq: 2,
);
final result = await client.device.sessionsResume(body);
print(result);
```

### presence
```dart
// Retrieve current principal presence
final result = await client.presence.meRetrieve();
print(result);
```

### realtime
```dart
// List pending realtime events
final params = <String, dynamic>{
  'limit': 1,
  'cursor': 'cursor',
};
final result = await client.realtime.eventsList(params);
print(result);
```

### rtc
```dart
// Create an IM-backed RTC session
final body = CreateRtcSessionRequest(
  conversationId: '1',
  mediaKind: 'mediakind',
);
final result = await client.rtc.sessionsCreate(body);
print(result);
```

### social
```dart
// List contact tags
final params = <String, dynamic>{
  'limit': 1,
  'cursor': 'cursor',
};
final result = await client.social.contactsTagsList(params);
print(result);
```

### chat
```dart
// List IM contacts
final params = <String, dynamic>{
  'limit': 1,
  'cursor': 'cursor',
};
final result = await client.chat.contactsList(params);
print(result);
```

### streams
```dart
// Open a stream
final body = OpenStreamRequest(
  streamType: 'streamtype',
  conversationId: '1',
);
final result = await client.streams.create(body);
print(result);
```

## Error Handling

```dart
try {
  final result = await client.presence.meRetrieve();
  print(result);
} catch (e) {
  print('Error: $e');
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

> Ensure `dart pub publish --dry-run` passes before release publish.

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
