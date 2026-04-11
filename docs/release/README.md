# Release 文档规范

## 1. 目录定位

`docs/release` 用于沉淀版本、changelog、release notes、go/no-go、商业化交付记录。

## 2. 最小文件集

每次有效迭代至少维护：

1. `docs/release/CHANGELOG.md`
2. `docs/release/YYYY-MM-DD-vX.Y.Z-loop-XX.md`

必要时补：

- `docs/release/YYYY-MM-DD-vX.Y.Z-release-notes.md`
- `docs/release/YYYY-MM-DD-vX.Y.Z-go-no-go.md`

## 3. 版本规则

- 商业化正式发布前默认 `0.y.z`
- `minor`：新增能力、完成 step/wave 闭环、外部可感知增强
- `patch`：修复、对齐、文档/测试/脚本补强、非破坏性优化
- `major`：破坏性变更或正式商业版本发布
- 仅当 `S00-S14` 全部闭环且 `release_closure` 达成，才允许 `1.0.0`

## 4. 每条 changelog 必填

- 日期
- 版本
- loop 编号
- 影响 step
- 变更摘要
- 专业影响描述
- 数据模型 / API / 协议 / 行为变化
- 迁移 / 降级 / 回退
- 测试与证据
- 文档回写
- 剩余风险

## 5. 更新规则

只要代码、行为、契约、测试结论、step 闭环状态、架构回写结论发生变化，就必须更新 `CHANGELOG.md` 与本轮 loop 文档。
