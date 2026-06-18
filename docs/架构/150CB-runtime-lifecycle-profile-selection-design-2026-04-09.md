# Runtime Lifecycle Profile Selection Design

## Decision

- `init/install/start/stop/restart` 必须与 runtime ops 共享同一 profile 解析器，而不是各自硬编码 `self-hosted.split-services.development`。

## State Model

- profile set: `self-hosted.split-services.development | self-hosted.split-services.development`
- config priority:
  - `self-hosted.split-services.development`: `.runtime/self-hosted.split-services.development/config/self-hosted.split-services.development.env` -> `.runtime/self-hosted.split-services.development/config/self-hosted.split-services.development.env`
  - `self-hosted.split-services.development`: `.runtime/self-hosted.split-services.development/config/self-hosted.split-services.development.env`
- runtime-dir fallback: 若 profile config 未显式覆盖，则回退到 `.runtime/self-hosted.split-services.development`

## Contract

- PowerShell / CMD：`-ProfileName <self-hosted.split-services.development|self-hosted.split-services.development>`
- Bash：`--profile <self-hosted.split-services.development|self-hosted.split-services.development>`
- `init-config-local.*` 为选定 profile 写入主 config 文件。
- `install/start/stop/restart` 必须按选定 profile 解析 config 与 runtime-dir。
- `retired-lifecycle-restart.*` 必须把 profile 继续传给 `stop/start`。

## Boundary

- 当前设计只统一 profile 入口，不声明 `self-hosted.split-services.development` 已拥有独立 runtime topology。
- 原生 Bash 执行态仍需单独验证。
