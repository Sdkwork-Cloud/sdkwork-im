# backend_sdk

Generator-owned Flutter transport SDK for the Craw Chat app API.

## Position In The SDK Family

This package is the low-level generated transport boundary.

For most Flutter app consumers, prefer the official app-facing package:

```yaml
dependencies:
  craw_chat_sdk: ^0.1.0
```

That package exposes:

- `CrawChatClient` as the primary semantic client
- semantic modules and helper builders for app runtime flows
- a root library at `package:craw_chat_sdk/craw_chat_sdk.dart`
- a root library that re-exports `backend_sdk` when you still need generated transport types

Use `backend_sdk` directly only when you explicitly want the standalone generated transport package.

## Installation

Direct transport use:

```yaml
dependencies:
  backend_sdk: ^0.1.0
```

Most app consumers should instead depend on:

```yaml
dependencies:
  craw_chat_sdk: ^0.1.0
```

## Quick Start

Direct transport:

```dart
import 'package:backend_sdk/backend_sdk.dart';

final client = SdkworkBackendClient(
  config: const SdkworkBackendConfig(
    baseUrl: 'http://127.0.0.1:18090',
    authToken: 'your-bearer-token',
  ),
);

final result = await client.inbox.getInbox();
print(result);
```

If you want semantic modules and helper builders from the official consumer package instead:

```dart
import 'package:craw_chat_sdk/craw_chat_sdk.dart';

final sdk = CrawChatClient.create(
  baseUrl: 'http://127.0.0.1:18090',
  authToken: 'your-bearer-token',
);

await sdk.conversations.postText(
  'conversation-1',
  text: 'hello world',
);
```

## Authentication

Craw Chat app routes use bearer authentication only.

```dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18090',
);

client.setAuthToken('your-bearer-token');
// Sends: Authorization: Bearer <token>
```

## Configuration

```dart
final client = SdkworkBackendClient(
  config: const SdkworkBackendConfig(
    baseUrl: 'http://127.0.0.1:18090',
    timeout: 30000,
    headers: <String, String>{
      'X-Custom-Header': 'value',
    },
  ),
);
```

## API Modules

- `client.auth` - portal authentication API
- `client.portal` - tenant portal snapshot API
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

## Publishing

This SDK includes cross-platform publish scripts in `bin/`:

- `bin/publish-core.mjs`
- `bin/publish.sh`
- `bin/publish.ps1`

## License

MIT

## Package Boundary

- Use only the package root entrypoint: `package:backend_sdk/backend_sdk.dart`.
- The package root exports `AuthApi` and `PortalApi` alongside the rest of the generated transport APIs.
- Generated `src/` imports are not part of the supported public API.
- Treat this package as the generator-owned transport boundary, not as the preferred Flutter app-consumer entrypoint.
- Prefer `package:craw_chat_sdk/craw_chat_sdk.dart` when you want the official Flutter SDK surface.
- `craw_chat_sdk` re-exports `backend_sdk`, so most app consumers do not need a second direct dependency on the generated transport package.
- `SdkworkBackendClient` now mounts `client.auth` and `client.portal` from the generated package root.
- Realtime websocket runtime remains outside generated delivery; this generated package is HTTP coordination only today.
- The workspace normalization wrapper strips generator-only auth scaffolding and source-tree build residue before verification and packaging.

## Regeneration Contract

- Generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- Each run also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- Apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across regenerations.
- If a generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
