# Runtime Lifecycle Profile Selection Design

## Decision

- `init/install/start/stop/restart` 必须与 runtime ops 共享同一 profile 解析器，而不是各自硬编码 `local-minimal`。

## State Model

- profile set: `local-minimal | local-default`
- config priority:
  - `local-default`: `.runtime/local-default/config/local-default.env` -> `.runtime/local-minimal/config/local-minimal.env`
  - `local-minimal`: `.runtime/local-minimal/config/local-minimal.env`
- runtime-dir fallback: 若 profile config 未显式覆盖，则回退到 `.runtime/local-minimal`

## Contract

- PowerShell / CMD：`-ProfileName <local-minimal|local-default>`
- Bash：`--profile <local-minimal|local-default>`
- `init-config-local.*` 为选定 profile 写入主 config 文件。
- `install/start/stop/restart` 必须按选定 profile 解析 config 与 runtime-dir。
- `restart-local.*` 必须把 profile 继续传给 `stop/start`。

## Boundary

- 当前设计只统一 profile 入口，不声明 `local-default` 已拥有独立 runtime topology。
- 原生 Bash 执行态仍需单独验证。
