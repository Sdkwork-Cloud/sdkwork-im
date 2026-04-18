# Wave D Windows package line

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/packages/windows`
- package-line documentation state: `template_only_pending_payload`
- service manager: `windows-service`

## Delivery forms

- `zip`
- `msi`

## Canonical artifact names

- `craw-chat-server-windows-x86_64.zip`
- `craw-chat-server-<version>-x64.msi`

## Canonical operator entrypoints

- `install-server.ps1`
- `init-config-server.ps1`
- `init-storage-server.ps1`
- `install-service-server.ps1`
- `install-server.cmd`
- `init-config-server.cmd`
- `init-storage-server.cmd`
- `install-service-server.cmd`

## Default install-root mapping

- install root: `%ProgramFiles%\\CrawChat`
- config root: `%CommonApplicationData%\\CrawChat\\default\\config`
- data root: `%CommonApplicationData%\\CrawChat\\default\\data`
- log root: `%CommonApplicationData%\\CrawChat\\default\\logs`
- run root: `%CommonApplicationData%\\CrawChat\\default\\run`

## Related staging contracts

- `artifacts/README.md`
- `artifacts/acceptance-manifest.json`
- `artifacts/layout-tree.txt`
- `../../windows-service/README.md`

## Current interpretation

- the Windows package line is frozen for archive and native-installer packaging
- actual Windows build outputs are still template-only until packaging execution populates the
  staging root
- every Windows package form must preserve the wrapper-required contract for `CrawChatServer.exe`
  and the shared `server.yaml` startup contract

