# SDK 总览

`sdks/` 用于冻结 `Wave D / Step 12 / CP12-2` 的 SDK 边界，而不是临时堆放空目录。

当前目标只有两个：

- 把 `sdkwork-craw-chat-sdk` 和 `sdkwork-craw-chat-sdk-admin` 的 facade 边界写清楚
- 明确它们与 `compatibility matrix`、`control-plane governance`、`tools/chat-cli` 的关系

## 当前目录

- `sdkwork-craw-chat-sdk`
  - 应用侧 SDK 总入口
  - 覆盖 `TypeScript` 与 `Flutter`
  - 面向 conversation / message / timeline / realtime 等 app-facing surface
- `sdkwork-craw-chat-sdk-admin`
  - 管理侧 SDK 总入口
  - 覆盖 `TypeScript` 与 `Flutter`
  - 面向 control-plane、protocol governance、compatibility matrix 等 admin-facing surface

## 边界原则

- 应用侧 SDK 不暴露 admin control-plane / ops / protocol governance 的写面
- 管理侧 SDK 不冒充聊天产品 SDK，不承接 `chat-session`、`timeline`、`send-message` 这类 app-facing facade
- 两套 SDK 都必须遵守当前 authority model：
  - HTTP authority fields 通过 bearer token
  - realtime authority 通过 `auth_bind`
  - capability 是否启用由 `compatibility matrix` 与 `hello_ack.capabilities` 决定

## 当前可信来源

- CLI 主链路参考：
  - [`docs/部署/CLI聊天验证与兼容矩阵.md`](../docs/部署/CLI聊天验证与兼容矩阵.md)
- registry / compatibility / governance：
  - `crates/craw-chat-ccp-registry/tests/compatibility_matrix_test.rs`
  - `services/control-plane-api/tests/protocol_registry_test.rs`
  - `services/control-plane-api/tests/protocol_governance_test.rs`
- control-plane `sdkCompatibilityBaseline`：
  - `appSdkFacade = sdkwork-craw-chat-sdk`
  - `adminSdkFacade = sdkwork-craw-chat-sdk-admin`
  - `protocolRegistryPath = /api/v1/control/protocol-registry`
  - `protocolGovernancePath = /api/v1/control/protocol-governance`
  - `matrixClientTypes = backend / desktop / mobile / web`
- control-plane `businessPolicyVocabulary`：
  - `policyVersionField = policy_version`
  - `capabilityFlagsField = capability_flags`
  - `historyVisibilityField = history_visibility`
  - `retentionPolicyRefField = retention_policy_ref`
  - `historyVisibilityModes = invited / joined / shared / world_readable`
  - `retentionPolicyScopes = tenant / space / group / channel / thread`

## 当前状态

- `TypeScript` 与 `Flutter` 子目录已经保留为稳定占位路径
- 当前 README 先冻结 facade 边界和消费者范围，不提前宣称代码生成链与发布流程已经完成
- `Wave D` bundle 已新增 machine-readable SDK 目录清单：
  - `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
  - 当前 `state = template_only_pending_generation`
  - 当前 `plannedVersion = null`
  - 当前 `versionStatus = version_unassigned_pending_freeze`
  - 当前 `versionDecisionSourcePath = null`
- 当前 `CP12-3` 已把 `compatibility matrix` 的文档 / 测试 / 控制面映射一起收口：
  - `docs/部署/CLI聊天验证与兼容矩阵.md`
  - `docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md`
  - `services/control-plane-api/tests/protocol_governance_test.rs`
  - `services/control-plane-api/tests/protocol_registry_test.rs`
  - `crates/craw-chat-ccp-registry/tests/compatibility_matrix_test.rs`
- 后续 `CP12-4` 再继续把多终端聊天与流式验证脚本整步收口
