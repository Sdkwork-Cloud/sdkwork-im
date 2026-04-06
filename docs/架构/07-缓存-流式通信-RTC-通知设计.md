# 缓存、流式通信、RTC 与通知设计

## 1. 缓存设计原则

- 缓存不是业务真相源
- 缓存只做加速与热点缓冲
- 缓存失效不能破坏提交语义
- 缓存必须 tenant-aware

## 2. 缓存用途

### 2.1 可缓存内容

- route snapshot
- session resume hint
- device online hint
- presence snapshot
- rate limit counters
- hot conversation summary
- device sync cursor
- device registration snapshot

### 2.2 不可缓存为真相的内容

- 消息历史
- 会话顺序号
- durable 事件
- 角色权限真相
- 文件资源权威元数据
- 设备补偿流事件本体

### 2.3 Device Sync Feed 与缓存边界

- `device registration snapshot` 只缓存“某主体当前有哪些已知设备”，真相可来自元数据存储或注册事件重放。
- `device sync cursor` 只缓存设备最近消费到的 `sync_seq`，真相仍由补偿流分区内的有序记录决定。
- `session resume hint` 可以缓存最近一次活跃 `device_id + sync_seq`，用于网关快速判定是否需要补拉 `sync-feed`。
- `device online hint` 和 `presence snapshot` 不参与补偿流正确性，只参与路由优化和是否优先走实时下行。

### 2.4 Session Resume Hint 与 Presence Snapshot 标准

- `session resume hint` 推荐键：
  - `tenant:{tenant_id}:principal:{principal_id}:device:{device_id}:resume_hint`
- 推荐值：
  - `latest_sync_seq`
  - `last_seen_sync_seq`
  - `last_resume_at`
  - `session_id`
- `presence snapshot` 推荐键：
  - `tenant:{tenant_id}:principal:{principal_id}:presence`
- 推荐值：
  - `current_device_id`
  - `devices[]`
  - 每设备 `status / session_id / last_sync_seq / last_resume_at / last_seen_at`

缓存使用原则：

- `session resume hint` 命中失败时，接入层必须退化为查询设备补偿流最新水位，不能因为缓存丢失导致恢复判定错误。
- `presence snapshot` 命中失败时，允许退化为“所有已注册设备默认为离线，当前设备按本次 resume 标记为在线”。
- `device online hint` 与 `presence snapshot` 允许短 TTL 和最终一致，不参与消息可靠性承诺。
- `presence.heartbeat` 可以只刷新缓存和接入层内存态，不要求写入 durable event。
- `session.disconnect` 必须优先刷新 `presence snapshot`，即便后续没有额外 durable 流程也不能继续显示为在线。
- `message.edited` 与 `message.recalled` 不要求额外缓存真相；若需要加速 timeline 展示，只能缓存最终投影视图，不能绕开事件重放语义。

## 3. 流式通信设计

## 3.1 流等级

- `Transient`
- `Durable Session`
- `Event Log`

## 3.2 StreamSession

生命周期：

- `created`
- `opened`
- `active`
- `checkpointed`
- `completed`
- `aborted`
- `expired`

## 3.3 持久化策略

- `Transient` 默认不落 durable commit
- `Durable Session` 落关键事件
- `Event Log` 事件全量 durable

## 3.4 用户自定义数据流

平台支持通过 `schema_ref + stream_type + policy` 定义用户自定义数据流：

- 状态同步
- 长任务输出
- 增量渲染
- 通知流水
- 结构化事件流

平台不内置垂直业务语义，但提供统一标准。

## 4. RTC 信令设计

### 4.1 信令类型

- `rtc.offer`
- `rtc.answer`
- `rtc.ice_candidate`
- `rtc.invite`
- `rtc.accept`
- `rtc.reject`
- `rtc.end`
- `rtc.member_state`

### 4.2 持久化边界

默认 durable：

- call started
- invite sent
- accept/reject
- call ended
- artifact attached

当前最小实现已落地：

- `rtc.invite`
- `rtc.accept`
- `rtc.reject`
- `rtc.end`

这些状态变化在 `conversationId` 存在时会映射为消息 `SignalPart`，并通过统一消息链进入 timeline / conversation summary。

默认 transient：

- ice candidate
- volume level
- weak participant state

## 5. 通知设计

### 5.1 通知源

- 新消息
- 提及
- 会话变更
- 自动化结果
- 系统公告
- RTC 呼叫事件

### 5.2 通知通道

- 站内
- WebSocket 实时下发
- Push 请求
- Webhook 回调

### 5.3 通知任务模型

- `notification.requested`
- `notification.dispatched`
- `notification.failed`

### 5.4 当前最小实现

- `local-minimal-node` 在 `message.posted` 提交成功后会触发一条 `inapp` 通知任务
- 通知任务当前走 `requested -> dispatched` 同步最小流水
- 自动化执行完成后会额外生成 `automation.result` 类通知

## 6. 读写分离策略

- 会话命令成功后先提交 event
- 通知通过 side-effect pipeline 异步触发
- RTC 信令事件与消息事件共用 envelope，但走不同消费链
- 当前最小实现中，通知与自动化 side-effect 在 `local-minimal-node` 编排层串联，保持主服务边界清晰，后续可替换为独立异步 worker
