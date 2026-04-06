# Gateway API 与协议设计

## 1. API 总体原则

- 外部采用 HTTP + WebSocket
- 内部采用 gRPC
- 所有外部请求必须 tenant-aware
- 统一 envelope 与错误码
- 支持离散消息与流式消息

## 2. Gateway 分层

- `Edge Gateway`
- `Session Gateway`
- `App Gateway API`
- `Admin / Ops API`

## 3. 核心外部 API

- 当前代码已落地的最小闭环包括：会话创建与摘要查询、消息发送与时间线查询、媒体上传创建/完成/查询/绑定、流开启/检查点/完成、RTC 创建/邀请/接受/拒绝/结束。

### 3.1 认证与会话

- `POST /api/v1/auth/login`
- `POST /api/v1/auth/refresh`
- `POST /api/v1/sessions/resume`
- `POST /api/v1/devices/register`
- `GET /api/v1/devices/{id}/sync-feed`

### 3.2 会话与成员

- `POST /api/v1/conversations`
- `GET /api/v1/conversations/{id}`
- `GET /api/v1/conversations/{id}/members`
- `POST /api/v1/conversations/{id}/members/add`
- `POST /api/v1/conversations/{id}/members/remove`
- `POST /api/v1/conversations/{id}/members/transfer-owner`
- `POST /api/v1/conversations/{id}/members/change-role`
- `POST /api/v1/conversations/{id}/members/leave`
- `GET /api/v1/conversations/{id}/read-cursor`
- `POST /api/v1/conversations/{id}/read-cursor`
- `GET /api/v1/inbox`
- `POST /api/v1/conversations` 当前只接受以下 `conversationType`：
  - `group`
  - `direct`
- special conversation dedicated create 状态：
  - `agent_dialog` 只能通过 `POST /api/v1/conversations/agent-dialogs` 创建。
  - `agent_handoff / system_channel` 当前仍是保留类型，不能通过 generic create 创建。
- 未注册类型和保留 special type 都必须在 create-time 直接返回 `400 conversation_type_invalid`。
- `POST /api/v1/conversations` 的创建者身份必须来自认证上下文，而不是请求体：
  - `tenantId` 来自认证上下文
  - `creatorId` 来自认证上下文
  - `creatorKind` 来自认证上下文中的 `actor_kind`
- `agent_dialog` 当前阶段已经改为 dedicated create 暴露；generic create 仍然禁止直接创建。
- `agent_handoff / system_channel` 当前阶段仍不允许通过 generic create 暴露；否则会生成没有专用参与者拓扑的残缺会话。
- `POST /api/v1/conversations/{id}/members/transfer-owner` 用于 group owner 向另一个 active member 交接 owner 角色，请求体只接受 `memberId`。
- 当前最小实现中：
  - 仅 `group` 支持 transfer-owner
  - 仅当前 active `owner` 可以发起
  - 目标必须是另一个 active member
  - 交接后旧 owner 降为 `admin`，新 owner 成为唯一 `owner`
- owner transfer 统一写入 `conversation.owner_transferred`，并作为旧 owner 后续 `leave` 的前置治理动作。
- `POST /api/v1/conversations/{id}/members/change-role` 用于 group owner 对另一个 active non-owner member 执行通用角色治理，请求体只接受 `memberId` 与目标 `role`。
- 当前最小实现中：
  - 仅 `group` 支持 `change-role`
  - 仅当前 active `owner` 可以发起
  - 目标必须是当前 active non-owner member
  - 目标 `role` 只允许变更为 `admin / member / guest`
  - 涉及 `owner` 的角色变化必须使用 `transfer-owner`
  - stale `memberId` 会被拒绝，不能命中历史 membership episode
- 通用角色治理统一写入 `conversation.member_role_changed`，返回 `previousMember / updatedMember / changedAt`，并同步更新读模型中的成员快照。
- `POST /api/v1/conversations/{id}/members/leave` 用于当前认证主体主动离开当前会话，请求体可为空，不接受 `tenantId`、`principalId`、`memberId`。
- 当前最小实现中：
  - `group` 的 `admin / member / guest` active member 可以 leave
  - `group owner` 必须先完成 owner transfer，之后按非 owner 规则 leave
  - `direct` 与尚未冻结成员生命周期的特殊会话类型统一拒绝 leave
- 主动离开统一写入 `conversation.member_left`，并立即失去 active member 访问权；不得复用 `members/remove` 模拟 self leave。
- `left` 后若由 owner/admin 再次通过 `members/add` 重新加入，服务端必须创建新的 membership episode：
  - 历史 `left / removed` 成员记录保留，不覆盖
  - 新 episode 生成新的 `memberId`
  - 当前 active member 关系总是指向最新 episode

### 3.3 消息

- `POST /api/v1/conversations/{id}/messages`
- `GET /api/v1/conversations/{id}/messages`
- `POST /api/v1/messages/{id}/edit`
- `POST /api/v1/messages/{id}/recall`
- 当前最小实现中，`POST /api/v1/conversations/{id}/messages` 支持 `text` 和 `parts` 混合提交；`parts` 可包含 `text/data/media/signal/stream_ref`。
- 已读推进统一走 `POST /api/v1/conversations/{id}/read-cursor`，避免把 read/ack 状态混入消息发送命令。
- `POST /api/v1/messages/{id}/edit` 与 `POST /api/v1/messages/{id}/recall` 作用于既有消息，服务端必须自行解析消息所属会话，客户端无需重复提交 `conversationId`。
- 编辑和撤回都采用事件追加，不允许原地覆盖 durable message log。

`POST /api/v1/messages/{id}/edit` 请求示例：

```json
{
  "summary": "edited summary",
  "text": "edited body"
}
```

`POST /api/v1/messages/{id}/recall` 请求示例：

```json
{}
```

### 3.4 流

- `POST /api/v1/streams`
- `POST /api/v1/streams/{id}/checkpoint`
- `POST /api/v1/streams/{id}/complete`
- `POST /api/v1/streams/{id}/abort`
- `WebSocket stream.command / stream.event` 用于连续数据帧传输，HTTP 只负责流生命周期命令。

### 3.5 RTC 信令

- `POST /api/v1/rtc/sessions`
- `POST /api/v1/rtc/sessions/{id}/invite`
- `POST /api/v1/rtc/sessions/{id}/accept`
- `POST /api/v1/rtc/sessions/{id}/reject`
- `POST /api/v1/rtc/sessions/{id}/end`
- `POST /api/v1/rtc/sessions/{id}/signals`
- 当前最小实现先通过 `invite/accept/reject/end` 写入信令状态，独立 `signals` 接口保留为后续扩展。
- 当前最小实现中，当 RTC 会话已绑定 `conversationId` 时，`invite/accept/reject/end` 会额外提交一条 `messageType=signal` 的消息，消息体包含 `SignalPart`，并进入时间线与摘要投影。

### 3.6 文件资源

- `POST /api/v1/media/uploads`
- `POST /api/v1/media/uploads/{id}/complete`
- `GET /api/v1/media/{id}`
- `POST /api/v1/media/{id}/attach`
- 当前最小实现已落地 create/complete/get/attach 四个接口，其中 `attach` 会把已就绪媒体资源包装为消息 `MediaPart` 并提交到目标会话。
- 上传完成后服务端会写入 `media.asset.created` 事件，供审计、工作流、投影和私有化替换适配复用。

### 3.7 通知

- `POST /api/v1/notifications/requests`
- `GET /api/v1/notifications`
- `GET /api/v1/notifications/{id}`
- 当前最小实现中，消息提交后会在 `local-minimal-node` 中以 side-effect 方式触发 `notification.requested -> notification.dispatched`。

### 3.8 自动化

- `POST /api/v1/automation/executions`
- `GET /api/v1/automation/executions/{id}`
- 当前最小实现中，自动化请求会同步走完 `automation.execution_requested -> automation.execution_completed` 最小链路，并为调用者生成一条 `automation.result` 站内通知。

### 3.9 审计与运维

- `POST /api/v1/audit/records`
- `GET /api/v1/audit/records`
- `GET /api/v1/audit/export`
- `GET /api/v1/ops/health`
- `GET /api/v1/ops/cluster`
- `GET /api/v1/ops/lag`
- `GET /api/v1/ops/diagnostics`
- 当前最小实现中，消息提交、通知请求、自动化执行都会留下审计锚点；运维面暴露单节点 `local-minimal` 的 cluster / lag / diagnostic 视图。

### 3.10 认证上下文约束

- 外部 API 请求体不得显式提交 `tenantId`、`creatorId`、`senderId`、`initiatorId`。
- `tenant`、`actor`、`session` 必须从已校验的 JWT / session / trusted identity context 中提取。
- `deviceId` 优先从 JWT claim 或可信头提取；仅在设备首次注册时允许显式输入，并在进入命令与投影层前绑定到认证上下文。
- gateway 负责把认证上下文转换成内部命令上下文与 envelope 字段。
- 对外 app-facing 入口默认必须要求 `Authorization: Bearer ...`；`trusted identity headers` 仅允许用于内部可信链路、测试装配或显式声明的 internal profile。
- 因此 `tenant_id` 仍然会出现在命令 envelope、事件 envelope、持久化对象与审计日志中，但这些字段属于服务端权威写入字段，不属于客户端可覆盖字段。
- 消息载荷中的发送者统一使用 `sender` 对象，而不是 `senderId` + `senderKind` 平铺字段。

### 3.11 已读游标 API 约定

- `GET /api/v1/conversations/{id}/read-cursor`
- `POST /api/v1/conversations/{id}/read-cursor`
- 租户、操作者、成员身份都必须由认证上下文和服务端成员关系推导，请求体不接受 `tenantId`、`principalId`、`memberId`。
- `POST` 请求体建议：

```json
{
  "readSeq": 12,
  "lastReadMessageId": "msg_c_xxx_12"
}
```

- `GET/POST` 返回体建议：

```json
{
  "tenantId": "t_xxx",
  "conversationId": "c_xxx",
  "memberId": "cm_xxx",
  "principalId": "u_xxx",
  "readSeq": 12,
  "lastReadMessageId": "msg_c_xxx_12",
  "updatedAt": "2026-04-05T10:00:10Z",
  "unreadCount": 3
}
```

- `readSeq` 只能前进不能回退；小于当前游标的重复提交按幂等处理，返回当前高水位。
- 当成员经历 `left -> rejoin` 形成新的 membership episode 时：
  - `GET /read-cursor` 只返回当前 active episode 对应的 `memberId`
  - 新 episode 的初始游标必须从 `readSeq = 0` 开始
  - 历史 episode 的游标仅作为历史数据保留，不能复用到新 episode

### 3.12 Inbox API 约定

- `GET /api/v1/inbox`
- inbox 只返回当前认证主体具备活跃成员关系的会话。
- 返回体建议：

```json
{
  "items": [
    {
      "tenantId": "t_xxx",
      "principalId": "u_xxx",
      "memberId": "cm_xxx",
      "conversationId": "c_xxx",
      "conversationType": "group",
      "messageCount": 12,
      "lastMessageId": "msg_c_xxx_12",
      "lastMessageSeq": 12,
      "lastSenderId": "u_other",
      "lastSenderKind": "user",
      "lastSummary": "hello",
      "unreadCount": 3,
      "lastActivityAt": "2026-04-05T10:00:10Z"
    }
  ]
}
```

- 最小实现阶段 inbox 以查询视图方式构建，不承担精确分页、标签过滤与置顶规则；这些能力后续在不破坏基础结构的前提下追加。

### 3.13 Device Sync Feed API 约定

- `POST /api/v1/devices/register`
- `GET /api/v1/devices/{id}/sync-feed`
- `POST /api/v1/devices/register` 用于把当前认证主体下的设备纳入多端补偿投递集合。请求体可显式携带 `deviceId` 作为首次注册输入；若认证上下文已经携带 `deviceId`，服务端必须校验两者一致。
- `GET /api/v1/devices/{id}/sync-feed` 只允许查询当前认证主体自己的设备补偿流；如果认证上下文自带 `deviceId`，则路径参数必须与其一致。
- 建议查询参数：

```text
GET /api/v1/devices/d_xxx/sync-feed?afterSeq=12
```

- 返回体建议：

```json
{
  "items": [
    {
      "tenantId": "t_xxx",
      "principalId": "u_xxx",
      "deviceId": "d_xxx",
      "syncSeq": 13,
      "originEventId": "evt_msg_c_xxx_13_posted",
      "originEventType": "message.posted",
      "conversationId": "c_xxx",
      "messageId": "msg_c_xxx_13",
      "messageSeq": 13,
      "memberId": null,
      "readSeq": null,
      "lastReadMessageId": null,
      "actorId": "u_other",
      "actorDeviceId": "d_other",
      "summary": "hello",
      "occurredAt": "2026-04-05T10:00:10Z"
    }
  ]
}
```

- 设备补偿流是查询面能力，不替代 WebSocket 实时下行；其职责是为多设备登录、断线恢复、补拉已读推进提供有序补偿。
- 一期标准要求至少投递两类事件：`message.posted`、`conversation.read_cursor_updated`。

### 3.14 Session Resume 与 Presence API 约定

- `POST /api/v1/sessions/resume`
- `POST /api/v1/sessions/disconnect`
- `POST /api/v1/presence/heartbeat`
- `GET /api/v1/presence/me`
- `POST /api/v1/sessions/resume` 只从认证上下文获得 `tenantId`、`actorId`、`sessionId`，不得由业务请求体重复提交这些字段。
- `deviceId` 可来自认证上下文或请求体；如果认证上下文已绑定 `deviceId`，请求体中的 `deviceId` 必须与之完全一致。
- `POST /api/v1/sessions/resume` 的职责是：
  - 确认当前设备已注册
  - 读取该设备当前最新 `syncSeq`
  - 对比客户端上送的 `lastSeenSyncSeq`
  - 返回是否需要补拉 `sync-feed`
  - 刷新当前设备的 presence 快照
- `POST /api/v1/presence/heartbeat` 的职责是刷新当前设备的在线时间与在线状态，不改变 `resumeFromSyncSeq` 语义。
- `POST /api/v1/sessions/disconnect` 的职责是把当前设备 presence 标记为 `offline`，保留最近一次 `lastSyncSeq` 与时间戳用于后续展示。
- `GET /api/v1/presence/me` 返回当前认证主体的设备在线快照，用于多端状态展示、路由优化和恢复前展示。

请求示例：

```http
POST /api/v1/sessions/resume
Content-Type: application/json

{
  "deviceId": "d_xxx",
  "lastSeenSyncSeq": 12
}
```

响应示例：

```json
{
  "tenantId": "t_xxx",
  "actorId": "u_xxx",
  "sessionId": "s_xxx",
  "deviceId": "d_xxx",
  "resumeRequired": true,
  "resumeFromSyncSeq": 13,
  "latestSyncSeq": 19,
  "resumedAt": "2026-04-05T10:00:20Z",
  "presence": {
    "tenantId": "t_xxx",
    "principalId": "u_xxx",
    "currentDeviceId": "d_xxx",
    "devices": [
      {
        "deviceId": "d_xxx",
        "platform": null,
        "sessionId": "s_xxx",
        "status": "online",
        "lastSyncSeq": 19,
        "lastResumeAt": "2026-04-05T10:00:20Z",
        "lastSeenAt": "2026-04-05T10:00:20Z"
      }
    ]
  }
}
```

`GET /api/v1/presence/me` 返回示例：

```json
{
  "tenantId": "t_xxx",
  "principalId": "u_xxx",
  "currentDeviceId": "d_xxx",
  "devices": [
    {
      "deviceId": "d_phone",
      "platform": null,
      "sessionId": "s_phone",
      "status": "online",
      "lastSyncSeq": 19,
      "lastResumeAt": "2026-04-05T10:00:20Z",
      "lastSeenAt": "2026-04-05T10:00:20Z"
    },
    {
      "deviceId": "d_pad",
      "platform": null,
      "sessionId": "s_pad",
      "status": "offline",
      "lastSyncSeq": 12,
      "lastResumeAt": "2026-04-05T09:50:00Z",
      "lastSeenAt": "2026-04-05T09:51:00Z"
    }
  ]
}
```

## 4. WebSocket 协议

### 4.1 连接初始化

客户端连接后发送：

- `auth.bind`
- `session.resume`
- `subscriptions.sync`

说明：

- `session.resume` 是连接建立后第一批上行命令之一，用于告知接入层当前设备是否需要从 `sync-feed` 继续补拉。
- `presence.heartbeat` 可以按固定间隔上送，用于刷新 `lastSeenAt`。
- `session.disconnect` 用于显式断开时把 presence 快照切回 `offline`。
- `presence.event` 协议名预留不变；一期可以只提供 `GET /api/v1/presence/me` 查询快照，不强制要求做广播下行。

### 4.2 下行事件

- `message.event`
- `stream.event`
- `rtc.signal`
- `notification.event`
- `system.event`
- `presence.event`

### 4.3 上行事件

- `message.command`
- `stream.command`
- `rtc.command`
- `ack`
- `ping`

## 5. Command Envelope

```json
{
  "command_id": "cmd_xxx",
  "tenant_id": "t_xxx",
  "command_type": "message.post",
  "target_aggregate_type": "conversation",
  "target_aggregate_id": "c_xxx",
  "idempotency_key": "ik_xxx",
  "issued_at": "2026-04-05T10:00:00Z",
  "payload": {}
}
```

说明：`tenant_id` 由 gateway 根据认证上下文写入，不由客户端在业务请求体中提交。

## 6. Event Envelope

```json
{
  "event_id": "evt_xxx",
  "tenant_id": "t_xxx",
  "event_type": "message.posted",
  "event_version": 1,
  "aggregate_type": "conversation",
  "aggregate_id": "c_xxx",
  "scope_type": "conversation",
  "scope_id": "c_xxx",
  "ordering_seq": 123,
  "occurred_at": "2026-04-05T10:00:01Z",
  "committed_at": "2026-04-05T10:00:01Z",
  "payload": {}
}
```

### 6.1 Message Payload Sender

```json
{
  "sender": {
    "id": "u_xxx",
    "kind": "user",
    "memberId": "cm_xxx",
    "deviceId": "d_xxx",
    "sessionId": "s_xxx",
    "metadata": {}
  }
}
```

## 7. 流式协议

### 7.1 StreamCommand

- `stream.open`
- `stream.data`
- `stream.patch`
- `stream.checkpoint`
- `stream.complete`
- `stream.abort`

### 7.2 StreamFrame

```json
{
  "stream_id": "st_xxx",
  "frame_seq": 10,
  "frame_type": "data",
  "schema_ref": "custom.delta.text.v1",
  "payload": {}
}
```

## 8. 错误码原则

- 参数错误
- 认证失败
- 权限不足
- 租户配额不足
- shard 不持有
- lease 过期
- 幂等冲突
- 事件版本不兼容

## 9. API 演进原则

- 路径稳定
- envelope 稳定
- 类型新增优先于字段破坏式修改
- 所有 schema 显式版本化
