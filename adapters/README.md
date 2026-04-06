# Adapters

`adapters/` 用于承载所有基础设施后端的可替换实现。

当前阶段约束如下：

- `adapters/local-memory` 是 `local-minimal` profile 的默认实现，用于本地安装、接口联调和最小闭环验证。
- `adapters/journal-redpanda`、`adapters/meta-cockroach`、`adapters/timeline-scylla` 保留为生产默认栈的标准目录。
- 所有适配器必须遵循 `docs/架构/04-技术选型与可插拔策略.md` 中定义的 capability 与 conformance 约束。
- 领域模型与 API 契约不得因后端替换发生变化。

