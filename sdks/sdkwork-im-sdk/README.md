# SDKWork IM SDK

`sdkwork-im-sdk` is the Craw Chat standard IM SDK family for `/im/v3/api`.
Runtime schema discovery uses `/im/v3/openapi.json`.

The family owns the open IM runtime surface: device sessions, presence, realtime
subscription bootstrap, social friend/contact flows, conversations, messages,
streams, and IM-backed RTC session state recovery. App-business routes,
backend/operator routes, appbase identity, Drive file lifecycle, and provider RTC
SDK behavior are dependency capabilities and must not be regenerated into this
family.

The TypeScript family is layered:

- `@sdkwork/im-sdk-generated` is the generator-owned HTTP transport package.
- `@sdkwork/im-sdk` is the authored app-facing facade package.
- The websocket adapter is authored under `sdkwork-im-sdk-typescript/src` because
  websocket transport is not generated HTTP output.

## Structure

| Path | Ownership |
| --- | --- |
| `openapi/craw-chat-im.openapi.yaml` | Owner-only authority OpenAPI for `/im/v3/api` |
| `openapi/craw-chat-im.sdkgen.yaml` | Derived HTTP generator input |
| `openapi/craw-chat-im.flutter.sdkgen.yaml` | Flutter-compatible derived generator input |
| `sdkwork-im-sdk-*/generated/server-openapi` | Generator-owned transport SDK output |
| `sdkwork-im-sdk-typescript/src` | Authored TypeScript composed facade and websocket adapter |

## Websocket Contract

`/im/v3/api/realtime/ws` is intentionally excluded from generated HTTP SDK input
and is implemented by the authored TypeScript realtime adapter. The adapter uses
legacy JSON websocket mode by default, sends `subscriptions.sync` on open,
decodes `event.window` frames into conversation message callbacks, and
acknowledges delivery with `events.ack`.

Browser runtimes use `globalThis.WebSocket` and depend on gateway/session
AppContext for websocket authentication. Node, Tauri, tests, and other hosts can
inject `ImSdkClientOptions.webSocketFactory`; the SDK passes resolved
`Authorization`, `Access-Token`, and SDKWork context headers to that factory.
Auth tokens must not be placed in websocket subprotocol names.

## Generation

```bash
node sdks/sdkwork-im-sdk/bin/generate-sdk.mjs
```

Generate a single language during focused development:

```bash
node sdks/sdkwork-im-sdk/bin/generate-sdk.mjs --language typescript
```

Use `--fixed-sdk-version` for release-pinned generation. Do not hand-edit files
under `generated/server-openapi`; update the OpenAPI authority or composed source
and regenerate.

## Verification

```bash
node sdks/sdkwork-im-sdk/bin/verify-sdk.mjs
```

Use `--language typescript` only for narrow TypeScript facade work. Full family
verification covers TypeScript, Flutter, Rust, Java, C#, Swift, Kotlin, Go, and
Python generated outputs.

## Dependency Boundary

`sdkDependencies` is explicit and empty for this SDK family. Consumers compose
other standard SDK families directly:

- `sdkwork-im-app-sdk` depends on `sdkwork-im-sdk` for IM runtime capability.
- `sdkwork-im-app-sdk` depends on `sdkwork-rtc-sdk` for provider RTC runtime.
- Appbase identity/session APIs stay in `sdkwork-appbase-*` SDK families.

Generated transport must not import dependency SDK packages.
