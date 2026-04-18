# Server 版本安装与初始化

`craw-chat-server` 是正式的 server 版本安装入口，对外默认由 `web-gateway` 提供统一端口访问。

- 正式可执行文件名为 `craw-chat-server`
- `start-server` 与托管脚本统一使用 `craw-chat-server --config <config-root>/server.yaml` 启动入口

## 一阶段命令面

- `install-server`
- `init-config-server`
- `init-storage-server`
- `verify-server`
- `plan-release-server`
- `install-service-server`
- `start-server`
- `stop-server`
- `restart-server`
- `status-server`

## 首次安装流程

1. 运行 `install-server`
2. 运行 `init-config-server`
3. 配置 PostgreSQL
4. 运行 `init-storage-server`
5. 运行 `verify-server`
6. 运行 `install-service-server`
7. 运行 `start-server`

补充：
- `start-server` 会优先使用安装目录下的 `bin/craw-chat-server`
- 在源码工作区运行时，脚本会构建 `services/web-gateway` 包并优先使用生成的 `craw-chat-server` 二进制
- `install-service-server` 会在 `<config-root>/generated/` 下生成 `systemd` unit、`launchd` plist，以及 Windows Service wrapper contract 文件，供不同操作系统的托管层继续接管

## 示例

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\install-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\init-config-server.ps1 -InstanceName default
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\init-storage-server.ps1 -InstanceName default -Mode verify-only
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\verify-server.ps1 -InstanceName default
```

```bash
bash bin/install-server.sh --instance default
bash bin/init-config-server.sh --instance default
bash bin/init-storage-server.sh --instance default --mode verify-only
bash bin/verify-server.sh --instance default
```

## Release contract audit

`verify-server` can also audit the machine-readable release bundle contract when operators or release engineers need to confirm that the current payload still lines up with the frozen package matrix, execution graph, provenance manifest, and go / no-go gate.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\verify-server.ps1 `
  -InstanceName default `
  -OutputFormat json `
  -ReleaseGatePath .\artifacts\releases\wave-d-2026-04-08\server\release-gate.json
```

```bash
bash bin/verify-server.sh \
  --instance default \
  --output-format json \
  --release-gate-path artifacts/releases/wave-d-2026-04-08/server/release-gate.json
```

`plan-release-server` is the dry-run companion for the same contract surface. It does not build or package artifacts; it translates the release-gate, package-catalog, release-execution, checksum, and artifact-file-list surfaces into an operator-facing release plan.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\plan-release-server.ps1 `
  -ReleaseGatePath .\artifacts\releases\wave-d-2026-04-08\server\release-gate.json `
  -Platform windows `
  -OutputFormat json
```

```bash
bash bin/plan-release-server.sh \
  --release-gate-path artifacts/releases/wave-d-2026-04-08/server/release-gate.json \
  --platform windows \
  --output-format json
```

- The release-gate manifest is the entrypoint.
- `verify-server` resolves `package-catalog.json`, `release-execution.json`, `release-provenance.json`, `release-checklist.md`, and every per-platform `acceptance-manifest.json` from that gate manifest.
- `verify-server` now performs semantic consistency checks in addition to path checks, so `contractsValid=false` also covers package-ID drift, startup-command drift, service-manager drift, and checklist contract drift.
- `plan-release-server` resolves the same machine-readable entrypoint and emits selected platform plans, staging roots, checksum command examples, and package IDs without mutating the bundle.
- `plan-release-server` now reuses the same release-contract helper as `verify-server` and `status-server`, so the emitted platform plan and `contractsValid` judgment stay synchronized.
- Runtime readiness and release-contract validity are reported together so the operator can tell whether the config is usable and whether the staged bundle is still auditable.
- `status-server` now exposes the same release bundle summary surface when operators want the generated service-contract paths and bundle gate status in one report.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\bin\status-server.ps1 `
  -InstanceName default `
  -OutputFormat json `
  -ReleaseGatePath .\artifacts\releases\wave-d-2026-04-08\server\release-gate.json
```

```bash
bash bin/status-server.sh \
  --instance default \
  --output-format json \
  --release-gate-path artifacts/releases/wave-d-2026-04-08/server/release-gate.json
```

## 说明

- 当前落地阶段先冻结 `craw-chat-server` 的 archive/payload/install contract。
- `systemd` 模板已进入仓库，`launchd` 与 Windows Service 将沿同一 contract 补齐。
- 这一阶段不替代 `local-minimal`，而是新增正式 server 安装面。
## Release payload contract

- canonical payload must ship `bin/craw-chat-server` or `bin/craw-chat-server.exe`
- payload must also ship `deployments/templates/server.yaml.example`, `deployments/templates/server.env.example`, and `deployments/templates/postgresql.yaml.example`
- cross-platform service payload must keep `deployments/systemd/craw-chat-server.service`, `deployments/launchd/com.sdkwork.crawchat.server.plist`, and `deployments/windows-service/CrawChatServer.xml`
- Windows package is `wrapper-required`, so `zip` or `msi` artifacts must include `bin/CrawChatServer.exe`
- `install-service-server` renders `generated/CrawChatServer.xml`, `install-CrawChatServer.ps1`, and `uninstall-CrawChatServer.ps1` from that payload contract
- foreground start, systemd, launchd, and Windows Service must all keep the same startup command: `craw-chat-server --config <config-root>/server.yaml`

## Package matrix

- Linux archive and installer forms: `tar.gz`, `deb`, `rpm`
  - canonical initialization entrypoints: `install-server.sh`, `init-config-server.sh`, `init-storage-server.sh`, `install-service-server.sh`
  - default install-root mapping: `/opt/craw-chat`, `/etc/craw-chat/default`, `/var/lib/craw-chat/default`, `/var/log/craw-chat/default`, `/var/run/craw-chat/default`
  - artifact staging and checksum contract: `server/packages/linux/artifacts/README.md`, `sha256sum -b <artifact> >> ../SHA256SUMS`
- macOS archive and installer forms: `tar.gz`, `pkg`
  - canonical initialization entrypoints: `install-server.sh`, `init-config-server.sh`, `init-storage-server.sh`, `install-service-server.sh`
  - current shell-based default install-root mapping: `/opt/craw-chat`, `/etc/craw-chat/default`, `/var/lib/craw-chat/default`, `/var/log/craw-chat/default`, `/var/run/craw-chat/default`
  - artifact staging and checksum contract: `server/packages/macos/artifacts/README.md`, `shasum -a 256 <artifact> >> ../SHA256SUMS`
- Windows archive and installer forms: `zip`, `msi`
  - canonical initialization entrypoints: `install-server.ps1`, `init-config-server.ps1`, `init-storage-server.ps1`, `install-service-server.ps1`
  - command wrapper entrypoints: `install-server.cmd`, `init-config-server.cmd`, `init-storage-server.cmd`, `install-service-server.cmd`
  - default install-root mapping: `%ProgramFiles%\\CrawChat` and `%CommonApplicationData%\\CrawChat\\default\\{config,data,logs,run}`
  - artifact staging and checksum contract: `server/packages/windows/artifacts/README.md`, `Get-FileHash -Algorithm SHA256 <artifact> | Format-Table -HideTableHeaders >> ../SHA256SUMS`
- Package artifact contract files: `server/packages/SHA256SUMS`, `server/packages/artifact-file-list.txt`
- Package release checklist contract: `server/packages/release-checklist.md`
- Package layout tree contract: `server/packages/layout-tree.txt`
- Machine-readable package catalog: `server/package-catalog.json`
  - schema: `artifacts/releases/schemas/server-package-catalog.schema.json`
  - each package entry freezes platform, package type, artifact path, install roots, service manager, and startup command
- Platform acceptance manifests: `server/packages/<platform>/artifacts/acceptance-manifest.json`
  - schema: `artifacts/releases/schemas/server-package-acceptance.schema.json`
  - each manifest freezes per-package acceptance checks, required payload entries, and validation placeholders
- Machine-readable release execution manifest: `server/release-execution.json`
  - schema: `artifacts/releases/schemas/server-release-execution.schema.json`
  - freezes the canonical build command, shared startup command, and per-platform staging execution graph
- Machine-readable release provenance manifest: `server/release-provenance.json`
  - schema: `artifacts/releases/schemas/server-release-provenance.schema.json`
  - freezes which payload source files and release-contract artifacts define the current server release standard
- Machine-readable release gate manifest: `server/release-gate.json`
  - schema: `artifacts/releases/schemas/server-release-gate.schema.json`
  - freezes the go / no-go gate inputs, review-doc links, and per-platform acceptance gate requirements
- All package forms must resolve back to the same canonical payload layout and `server.yaml` startup contract.

## Unified gateway endpoints

- After `start-server`, the unified `web-gateway` port is the operator-facing entry for `/healthz`, `/readyz`, `/openapi.json`, `/openapi/index.json`, and `/docs`.
- The gateway also exposes per-service schema proxies at `/openapi/services/<service-id>.openapi.json`.
- The gateway also exposes per-service rendered docs at `/docs/services/<service-id>`.
- Startup output should list the aggregate schema endpoint, the schema index endpoint, and the service-level schema/docs endpoints for the configured upstream set.
