# Sdkwork IM 架构文档

## 当前权威入口（Topology v2）

运行拓扑、环境变量与开发命令以以下文件为准，本目录中的历史设计文档不再作为部署或开发入口：

| 文档 | 用途 |
| --- | --- |
| [docs/topology-greenfield.md](../topology-greenfield.md) | Topology v2 绿场计划与退役清单 |
| [specs/topology.spec.json](../../specs/topology.spec.json) | 机器可读拓扑契约 |
| [configs/topology/*.env](../../configs/topology/) | Profile 环境变量 |
| [docs/sites/architecture/](../sites/architecture/) | 对外文档站点（架构、拓扑、模块） |
| [docs/sites/getting-started/](../sites/getting-started/) | 快速开始与开发命令 |
| [docs/部署/README.md](../部署/README.md) | 部署与验证索引 |

默认开发 profile：`self-hosted.split-services.development` → `pnpm im:dev`

## 本目录定位

`docs/架构/` 为 2026-04 前后架构设计、实施计划与专项标准归档；正文 profile / lifecycle 词汇已迁移到 Topology v2。文件名中保留的历史词（如 `*-local-minimal-*`）仅用于链接定位，不代表当前运行时。

## 专项设计索引（130–150 系列）

长期架构总纲与专项设计见目录内 `130-*` 至 `150-*` 文档；执行层细化见 [docs/step/README.md](../step/README.md)。

## RTC 边界

IM 拥有 `/im/v3/api/calls` 信令；RTC 媒体/provider 标准在同级仓库 `../sdkwork-rtc`，见 [../sdkwork-rtc/docs/rtc-im-boundary.md](../../sdkwork-rtc/docs/rtc-im-boundary.md)（checkout 后可用）。

## 验证

- `pnpm test:topology-baggage` — 活跃路径不得再引用退役 topology
- `pnpm test:runtime-standard` — 运行时与 manifest 契约
- `pnpm test:workflow-commercial-gates` — 商业化治理门禁
