# Wave D server Windows Service contract

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/windows-service`
- Windows-service documentation state: `template_only_pending_payload`

This directory documents the Windows Service contract for `craw-chat-server`. It is not a second
runtime model. It is a wrapper around the same canonical server payload and startup command.

## Required inputs

- `bin/craw-chat-server.exe`
- `bin/CrawChatServer.exe`
- `deployments/windows-service/CrawChatServer.xml`
- `deployments/templates/server.yaml.example`

## Generated instance-specific outputs

`install-service-server` is expected to render instance-local service files under
`<config-root>/generated/`:

- `generated/CrawChatServer.xml`
- `install-CrawChatServer.ps1`
- `uninstall-CrawChatServer.ps1`

## Wrapper contract

- wrapper host:
  - `bin/CrawChatServer.exe`
- service identity:
  - `CrawChatServer`
- wrapped process:
  - `craw-chat-server --config <config-root>/server.yaml`

## Delivery constraint

- final package forms may be `zip` or `msi`
- package format may vary, but wrapper identity and wrapped process contract may not drift

