# sdkwork-craw-chat-sdk-admin Flutter

当前目录保留给管理侧 `Flutter` facade。

## 预期职责

- `control-plane` 读面
- `protocol governance`
- `compatibility matrix`

## 当前约束

- 不承接 app-facing chat facade
- 协议能力决策以 control-plane snapshot 为准
- 当前先冻结路径与职责，不提前宣称生成代码已完成

## 当前发布边界

- 当前 bundle 级发布目录真源：
  - `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
- 当前 catalog 状态：
  - `template_only_pending_generation`
  - `not_published`
- 当前版本占位状态：
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`
- 在真实生成与发布链补齐前，以 bundle catalog 为准，不在本 README 单独发明版本或发布结论
