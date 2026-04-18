# Wave D macOS package line

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/packages/macos`
- package-line documentation state: `template_only_pending_payload`
- service manager: `launchd`

## Delivery forms

- `tar.gz`
- `pkg`

## Canonical artifact names

- `craw-chat-server-darwin-universal.tar.gz`
- `craw-chat-server-<version>.pkg`

## Canonical operator entrypoints

- `install-server.sh`
- `init-config-server.sh`
- `init-storage-server.sh`
- `install-service-server.sh`

## Default install-root mapping

- install root: `/opt/craw-chat`
- config root: `/etc/craw-chat/default`
- data root: `/var/lib/craw-chat/default`
- log root: `/var/log/craw-chat/default`
- run root: `/var/run/craw-chat/default`

## Related staging contracts

- `artifacts/README.md`
- `artifacts/acceptance-manifest.json`
- `artifacts/layout-tree.txt`

## Current interpretation

- the macOS package line is frozen for archive and native-installer packaging
- actual macOS build outputs are still template-only until packaging execution populates the
  staging root
- all macOS package forms must preserve the canonical payload layout and the shared
  `server.yaml` startup contract

