# backend_sdk

Generated Flutter transport package for the Craw Chat app API.

## Package Role

This package is the generator-owned transport layer for the checked-in app OpenAPI contract.
Use it when you need direct access to generated HTTP operations and root-exported transport types.

For business-facing chat integrations, prefer the composed Flutter layers under
`sdkwork-craw-chat-sdk-flutter/composed`, where the manual `craw_chat_sdk` package wraps this
transport package with the higher-level chat client surface.

## Installation

Add to `pubspec.yaml`:

```yaml
dependencies:
  backend_sdk: ^0.1.0
```

## Quick Start

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

## Authentication

Craw Chat app routes use bearer authentication only.

```dart
final client = SdkworkBackendClient.withBaseUrl(
  baseUrl: 'http://127.0.0.1:18090',
);

client.setAuthToken('your-bearer-token');
// Sends: Authorization: Bearer <token>
```

## Endpoint Targeting

- In direct local development, point `baseUrl` to the app-facing service origin, typically the
  local `local-minimal-node` HTTP endpoint such as `http://127.0.0.1:18090`.
- In packaged installs, point `baseUrl` to the unified `craw-chat-server` or `web-gateway`
  public origin.
- Keep one deployment model per client configuration. Do not mix direct local service and unified
  gateway assumptions in the same client instance.

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

## Surface Groups

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
- Do not import generated `lib/src/` imports from downstream code.
- Keep business orchestration in the composed Flutter layers under
  `sdkwork-craw-chat-sdk-flutter/composed` instead of re-exporting generated internals.
- The workspace normalization wrapper strips generator-only auth scaffolding and source-tree build
  residue before verification and packaging.

## Regeneration Contract

- Generator-owned files are tracked in `.sdkwork/sdkwork-generator-manifest.json`.
- Each run also writes `.sdkwork/sdkwork-generator-changes.json` so automation can inspect created, updated, deleted, unchanged, scaffolded, and backed-up files plus the classified impact areas, verification plan, and execution decision for the latest generation.
- Apply mode also writes `.sdkwork/sdkwork-generator-report.json` with the full execution report, including `schemaVersion`, `generator`, stable artifact paths, and the execution handoff commands that match CLI `--json` output.
- CLI JSON output also includes an execution handoff with concrete next commands, including reviewed apply commands for dry-run flows.
- Put hand-written wrappers, adapters, and orchestration in `custom/`.
- Files scaffolded under `custom/` are created once and preserved across regenerations.
- If a generated-owned file was modified locally, its previous content is copied to `.sdkwork/manual-backups/` before overwrite or removal.
