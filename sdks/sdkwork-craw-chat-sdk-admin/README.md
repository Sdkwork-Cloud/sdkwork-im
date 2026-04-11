# sdkwork-craw-chat-sdk-admin

`sdkwork-craw-chat-sdk-admin` 是管理侧 SDK 容器目录，用于承接 control-plane 与 protocol governance facade。

## 当前消费者范围

- `TypeScript`
  - 目录：[`sdkwork-craw-chat-sdk-admin-typescript`](./sdkwork-craw-chat-sdk-admin-typescript/README.md)
- `Flutter`
  - 目录：[`sdkwork-craw-chat-sdk-admin-flutter`](./sdkwork-craw-chat-sdk-admin-flutter/README.md)

## 当前 facade 边界

管理侧 SDK 未来只承接以下表面：

- `control-plane` 读面
- `protocol governance`
- `compatibility matrix`
- protocol registry / rollout / kill switch 的管理消费面

管理侧 SDK 不承接：

- app-facing conversation facade
- `chat-session` / `send-message` / `timeline`
- 直接替代 `tools/chat-cli` 的现场验证职责

## 与当前治理基线的关系

- 当前权威来源来自 control-plane snapshot 与 protocol governance
- admin facade 只能消费已经被冻结的 registry / compatibility matrix 结果
- 不允许绕开治理快照，在客户端本地拼装协议能力决策
- 当前冻结基线：
  - `appSdkFacade = sdkwork-craw-chat-sdk`
  - `adminSdkFacade = sdkwork-craw-chat-sdk-admin`
  - `protocolRegistryPath = /api/v1/control/protocol-registry`
  - `protocolGovernancePath = /api/v1/control/protocol-governance`
  - `matrixClientTypes = backend / desktop / mobile / web`
  - `policyVersionField = policy_version`
  - `capabilityFlagsField = capability_flags`
  - `historyVisibilityField = history_visibility`
  - `retentionPolicyRefField = retention_policy_ref`
  - `historyVisibilityModes = invited / joined / shared / world_readable`
  - `retentionPolicyScopes = tenant / space / group / channel / thread`

## 当前 close / error registry 治理边界

管理侧 SDK 需要把 close / error registry 作为治理结果的一部分暴露给消费方，而不是留给各端自行猜测：

- `session.disconnect` 是正式恢复事件，当前 consumer 必须收到对应 `goaway` / close 语义，并执行 fresh resume fallback。
- `session.disconnect` 的治理公开面必须明确包含 websocket close code `4001`、close reason `session.disconnect`，以及后续 stale 请求应返回 `reconnect_required`。
- `realtime.overload` 需要区分 pull-only 降级与彻底断链两种恢复路径，不能把所有过载都归为同一类“重连失败”。
- pull-only 降级下，consumer 必须继续使用 `events.pull` / `event.window` 追平窗口，而不是把 live push 暂停误记为数据丢失。
- `goaway` code / message 与 `compatibility matrix`、governance snapshot 一样，属于客户端恢复决策的正式输入。
- 管理侧 facade 需要明确告诉消费方：哪些场景允许 resume fallback，哪些场景应该直接重建连接或等待新的治理结果。

## 当前状态

- 当前目录先冻结 `TypeScript` 与 `Flutter` 管理侧占位路径
- 还没有在本轮提前宣称多语言 admin facade 代码与发布流程已经完成
- 当前 bundle 内的 SDK 发布真源是：
  - `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
  - `generationStatus = template_only_pending_generation`
  - `releaseStatus = not_published`
- 当前版本占位状态已经冻结到 bundle catalog：
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`
- 后续实现必须继续与 `services/control-plane-api/tests/protocol_registry_test.rs`、`services/control-plane-api/tests/protocol_governance_test.rs` 保持一致
