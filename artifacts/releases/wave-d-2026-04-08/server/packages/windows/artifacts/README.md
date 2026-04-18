# Wave D Windows staging root

- bundle: `wave-d-2026-04-08`
- artifact root: `artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts`
- staging documentation state: `template_only_pending_payload`
- execution state: `template_only_pending_execution`

This staging root is where Windows packaging outputs are expected to land when the release
execution plan is run.

- canonical build command: `cargo build -p web-gateway --release --bin craw-chat-server --offline`

## Expected staged artifacts

- `craw-chat-server-windows-x86_64.zip`
- `craw-chat-server-<version>-x64.msi`
- Windows Service wrapper payload:
  - `CrawChatServer.exe`

## Shared staging contracts

- `acceptance-manifest.json`
- `layout-tree.txt`
- `../../artifact-file-list.txt`
- `../../SHA256SUMS`

## Canonical workflow

- Step 1: build the canonical `craw-chat-server.exe` payload.
- Step 2: assemble Windows package outputs and wrapper payload into this staging root.
- Step 3: refresh `../artifact-file-list.txt`.
- Step 4: refresh `../SHA256SUMS`.
- Step 5: validate `acceptance-manifest.json`.

## Checksum example

- `Get-FileHash -Algorithm SHA256 <artifact> | Format-Table -HideTableHeaders >> ../SHA256SUMS`
