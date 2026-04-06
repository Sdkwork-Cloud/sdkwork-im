# local-memory

`local-memory` 是当前 `local-minimal` profile 的默认适配器包。

提供能力：

- `MemoryCommitJournal`
- `MemoryMetadataStore`
- `MemoryTimelineProjectionStore`

适用场景：

- 本地开发
- 最小安装验证
- 单节点联调
- 契约与回归测试

限制：

- 状态仅保存在进程内存中
- 不提供重启恢复
- 不提供多节点一致性
- 不代表生产默认后端实现

