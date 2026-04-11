# Lifecycle Profile Doc Contract Alignment Design

## Decision

- 顶层 operator 文档必须显式公开 lifecycle 全链的 profile 入口，而不能只公开 runtime ops 或 deploy 侧入口。

## Contract

- README 与快速启动文档都必须展示：
  - `install/init/start/restart/stop` 的 `local-default` 示例
  - `--profile <local-minimal|local-default>` / `-ProfileName <local-minimal|local-default>`
  - `.runtime/local-default/config/local-default.env`
  - `local-default` 当前仍复用 `.runtime/local-minimal` runtime-dir

## Boundary

- 这是文档合同设计，不改变 runtime selection 实现。
- 若未来 `local-default` 拥有独立 topology，继续扩充同一入口，不新增别名文档。
