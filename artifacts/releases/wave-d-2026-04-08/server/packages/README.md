# Wave D server package matrix

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/packages`
- package-matrix documentation state: `template_only_pending_payload`

This directory is the human-readable package matrix for `craw-chat-server`. It maps platform
delivery forms back to the canonical server payload and the shared startup contract.

## Package-matrix contracts

- `../package-catalog.json`
  - package inventory, file-name templates, install roots, and service managers
- `../release-execution.json`
  - per-platform staging and checksum workflow examples
- `../release-gate.json`
  - packaging go / no-go inputs
- `linux/artifacts/acceptance-manifest.json`
- `macos/artifacts/acceptance-manifest.json`
- `windows/artifacts/acceptance-manifest.json`
- `release-checklist.md`
  - human-reviewed sequencing for packaging and verification
- `artifact-file-list.txt`
  - frozen artifact listing contract
- `SHA256SUMS`
  - frozen checksum manifest contract
- `layout-tree.txt`
  - package layout snapshot contract

## Platform package families

- `linux`
  - `tar.gz`
  - `deb`
  - `rpm`
- macOS
  - `tar.gz`
  - `pkg`
- `windows`
  - `zip`
  - `msi`

## Frozen artifact names

- `craw-chat-server-linux-x86_64.tar.gz`
- `craw-chat-server_<version>_amd64.deb`
- `craw-chat-server-<version>-1.x86_64.rpm`
- `craw-chat-server-darwin-universal.tar.gz`
- `craw-chat-server-<version>.pkg`
- `craw-chat-server-windows-x86_64.zip`
- `craw-chat-server-<version>-x64.msi`

## Platform indexes

- `linux/README.md`
- `linux/artifacts/README.md`
- `macos/README.md`
- `macos/artifacts/README.md`
- `windows/README.md`
- `windows/artifacts/README.md`

## Shared invariants

- all package lines map back to the same canonical payload
- all package lines preserve the same `server.yaml` startup contract
- all package lines preserve the same service identity semantics
- Windows package lines additionally preserve the dedicated wrapper contract for
  `CrawChatServer.exe`

## Current interpretation

- the package matrix is frozen for review and automation
- the package contract paths are real and checked in
- the archive and installer artifacts themselves are still template-only staged outputs until
  actual packaging runs populate the staging roots
