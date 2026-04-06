# 2026-04-06 Agent Handoff Projection Review Cycle

## 1. Findings

### 1.1 High: `agent_handoff` 生命周期已经进入运行时，但 `projection-service` 仍然完全丢失该状态

- Root cause:
  - `conversation-runtime` 已经落地 `conversation.agent_handoff_status_changed` 事件。
  - `projection-service` 的 `apply(...)` 仍然只处理 `conversation.created / message.* / member.* / read_cursor_updated`，没有消费 handoff 生命周期事件。
  - `ConversationSummaryView` 与 `ConversationInboxEntry` 没有 handoff 专用字段。
- Impact:
  - inbox / summary 读模型看不到 `open / accepted / resolved / closed`。
  - 业务侧只能回退到运行时专用接口，读模型与运行时出现双真相。

### 1.2 High: 会话创建后直到第一条消息出现前，`conversation summary` API 会返回 `404`

- Root cause:
  - 旧实现只在 `message.posted` 时创建 `ConversationSummaryView`。
  - `conversation.created` 只更新了 `conversations` catalog，没有初始化 summary 基线记录。
- Impact:
  - `agent_handoff` 刚创建且尚未发送消息时，`GET /api/v1/conversations/{conversationId}` 无法返回状态。
  - 本轮新增 handoff 生命周期读模型需求无法成立，因为“无消息但有生命周期”的会话最先就被 `404` 截断。

### 1.3 Medium: inbox 排序没有把 handoff 生命周期变化视为会话活动

- Root cause:
  - inbox 的 `lastActivityAt` 只看 `summary.last_message_at` 或 `conversation.created_at`。
  - `accepted / resolved / closed` 这类非消息活动没有纳入排序依据。
- Impact:
  - 交接被接受或关闭后，目标会话不会前移，运营处理视图不稳定。

## 2. Design Decision

本轮冻结以下读模型标准：

- `conversation.created` 必须初始化 summary 基线记录，而不是等第一条消息。
- `ConversationSummaryView` 中与“最后一条消息”绑定的字段允许为空：
  - `lastMessageId`
  - `lastSenderId`
  - `lastSenderKind`
  - `lastSender`
  - `lastMessageAt`
- `agent_handoff` 读模型以嵌套字段形式进入：
  - `conversation summary`
  - `inbox entry`
- `inbox.lastActivityAt` 取以下时间中的最大值：
  - `lastMessageAt`
  - `acceptedAt`
  - `resolvedAt`
  - `closedAt`
  - 若仍为空则回退 `conversation.created_at`

## 3. Implementation Summary

- `crates/im-domain-core/src/conversation.rs`
  - 新增：
    - `ConversationActorView`
    - `ConversationAgentHandoffView`
  - `ConversationInboxEntry` 新增：
    - `agentHandoff`
- `services/projection-service/src/lib.rs`
  - `ConversationSummaryView` 新增：
    - `agentHandoff`
  - `ConversationSummaryView` 中消息尾部字段改为 nullable，以支持“会话已创建但尚无消息”的合法状态。
  - `conversation.created` 现在会创建 summary 基线记录。
  - 新增 `conversation.agent_handoff_status_changed` 投影逻辑。
  - inbox 读取时同步暴露 `agentHandoff`，并使用 handoff 生命周期时间参与 `lastActivityAt` 计算。
- `services/local-minimal-node/tests/access_control_e2e_test.rs`
  - 增加基于真实 HTTP 路由的本地最小节点回归用例，验证创建后立即可读 summary，且 accept 后 inbox / summary 状态同步更新。

## 4. Tests Added

- `services/projection-service/tests/timeline_projection_test.rs`
  - `test_agent_handoff_lifecycle_projects_into_summary_and_inbox_views`
- `services/local-minimal-node/tests/access_control_e2e_test.rs`
  - `test_agent_handoff_summary_and_inbox_projection_in_local_profile`
- `crates/im-domain-core/tests/model_contract_test.rs`
  - 补齐 `ConversationInboxEntry.agentHandoff` 的序列化契约断言

## 5. Verification

- Red phase:
  - `cargo test -p projection-service --offline test_agent_handoff_lifecycle_projects_into_summary_and_inbox_views`
    - failed because summary/inbox 缺少 `agent_handoff` 字段，且 summary 基线不存在
  - `cargo test -p local-minimal-node --offline test_agent_handoff_summary_and_inbox_projection_in_local_profile`
    - failed with `404 != 200` because summary 直到第一条消息前都不存在
- Green phase:
  - `cargo test -p projection-service --offline test_agent_handoff_lifecycle_projects_into_summary_and_inbox_views`
  - `cargo test -p local-minimal-node --offline test_agent_handoff_summary_and_inbox_projection_in_local_profile`

## 6. Remaining Risks

- `projection-service` 目前只把 handoff 状态放进 summary / inbox；尚未投影到独立 admin 视图。
- device sync feed 仍然没有 handoff 生命周期专用同步项，多端如果只消费 sync-feed，暂时还感知不到 accept / resolve / close。
- `agent_dialog` 专用 close/archive 与 `system_channel` scheduled/bulk publish 生命周期仍未冻结。

## 7. Next Wave

1. 为 handoff 生命周期补充 admin-facing read model 或管理查询接口。
2. 评估是否要把 handoff 生命周期变化写入 device sync feed / realtime read model，而不只是在 inbox / summary 可见。
3. 继续冻结 `agent_dialog` 与 `system_channel` 的 post-create 生命周期矩阵。
