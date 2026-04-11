# 缓存、流式通信、RTC 与通知设计

## 1. 文档目标

本文档用于统一描述 `craw-chat` 在缓存、流式通信、RTC 信令和通知侧的设计口径，确保：

- 流是真正的一等能力
- 缓存不侵入真相源
- RTC 只做信令
- 通知和自动化不阻塞主提交链路
- AI 流输出与 IoT 设备流能够落在同一套实时模型下

## 2. 缓存设计原则

- 缓存不是业务真相源
- 缓存只做加速、热点承接和恢复提示
- 缓存失效不能破坏提交语义
- 所有缓存都必须 tenant-aware
- 缓存键空间必须与 route、presence、quota、resume 等责任域分离

## 3. 缓存职责边界

### 3.1 可缓存内容

- route snapshot
- session resume hint
- reconnect hint
- device online hint
- presence snapshot
- rate limit counters
- hot conversation summary
- unread hot cache
- device registration snapshot

### 3.2 不可缓存为真相的内容

- 消息历史本体
- 会话顺序号
- durable event 本体
- 角色权限真相
- 媒体资源权威元数据
- stream durable checkpoint 真相

### 3.3 推荐键空间

- `tenant:{tenant}:route:*`
- `tenant:{tenant}:presence:*`
- `tenant:{tenant}:resume:*`
- `tenant:{tenant}:quota:*`
- `tenant:{tenant}:summary:*`

## 4. Resume 与 Presence 设计

### 4.1 Session Resume Hint

推荐缓存键：

- `tenant:{tenant_id}:principal:{principal_id}:device:{device_id}:resume_hint`

推荐值：

- `latest_sync_seq`
- `last_seen_sync_seq`
- `last_resume_at`
- `session_id`
- `resume_token`

### 4.2 Presence Snapshot

推荐缓存键：

- `tenant:{tenant_id}:principal:{principal_id}:presence`

推荐值：

- `current_device_id`
- `devices[]`
- 每设备 `status / session_id / last_sync_seq / last_resume_at / last_seen_at`

### 4.3 恢复原则

- `resume hint` 命中失败时，系统必须回退到权威存储和补偿流，而不能直接判定无需恢复。
- `presence snapshot` 命中失败时，允许回退为“全离线 + 当前会话在线”的保守视图。
- `presence.heartbeat` 可以只刷新缓存和接入层内存态，不要求进入 durable truth。

## 5. 流式通信设计

## 5.1 流的定位

流不是普通消息的补丁，而是与消息并列的一等语义，统一承载：

- LLM token streaming
- Agent tool call 渐进输出
- 长任务进度
- JSON patch
- 实时语音转写
- 设备 telemetry
- command response stream

## 5.2 Stream 生命周期

统一生命周期建议为：

- `stream.open`
- `stream.delta`
- `stream.patch`
- `stream.checkpoint`
- `stream.finalize`
- `stream.abort`

## 5.3 流等级

- `Transient`
- `Durable Session`
- `Event Log`

说明：

- `Transient` 适合弱持久化或实时展示
- `Durable Session` 适合需要恢复和补偿的交互流
- `Event Log` 适合要求完整回放和审计的流

## 5.4 持久化策略

- `Transient` 可不全量 durable
- `Durable Session` 必须落 session、checkpoint 和关键帧
- `Event Log` 需要完整 durable append

## 5.5 Materialization 策略

流结束后可根据策略物化为：

- 普通消息
- 卡片
- 文件
- 任务状态
- 知识片段

## 5.6 自定义流

系统支持通过 `schema_ref + stream_type + policy` 承载用户自定义流，平台只提供统一壳层，不硬编码垂直业务语义。

## 6. RTC 设计

### 6.1 RTC 定位

- RTC 只统一信令层
- 媒体面独立部署
- 信令可绑定会话，但不能污染消息内核的有序主链

### 6.2 RTC 信令类型

- `rtc.offer`
- `rtc.answer`
- `rtc.ice_candidate`
- `rtc.invite`
- `rtc.accept`
- `rtc.reject`
- `rtc.end`
- `rtc.member_state`

### 6.3 Durable 边界

默认 durable：

- `rtc.invite`
- `rtc.accept`
- `rtc.reject`
- `rtc.end`
- artifact / recording / transcript attachment

默认 transient：

- `rtc.ice_candidate`
- `volume level`
- `weak participant state`

### 6.4 与消息体系的关系

当 RTC 会话绑定 `conversationId` 时：

- 关键生命周期可物化为 `SignalPart` 消息
- timeline、summary 和通知视图可以引用 RTC 事件结果
- 媒体传输本身仍独立于 IM 内核

### 6.5 RTC provider 插件边界

- RTC 信令仍由平台统一实现，provider 插件只承接房间、凭证、回调、录制和媒体相关差异。
- 当前冻结的 RTC provider 为 `火山引擎 / 阿里云 / 腾讯云`，全局默认 provider 为 `火山引擎`。
- provider 选择必须来自控制面和 deployment profile，不能在消息或 signaling 路径内联判断厂商。
- 录制、转写、回放等 RTC 产物必须统一回流到 `BlobStore`，不允许长期锁死在 provider 私有文件体系。

## 7. 通知设计

### 7.1 通知源

- 新消息
- @提及
- 会话变更
- Agent 结果
- 自动化结果
- 系统公告
- RTC 呼叫事件
- 设备告警

### 7.2 通知通道

- 站内通知
- WebSocket 实时下发
- Push 请求
- Webhook 回调
- 未来预留短信、邮件和企业连接器

### 7.3 通知事件模型

- `notification.requested`
- `notification.dispatched`
- `notification.failed`
- `notification.acknowledged`

### 7.4 设计原则

- 通知通过 side-effect pipeline 异步触发
- 通知失败不得影响消息提交成功语义
- 大规模广播或推送必须支持租户级限流与降级

## 8. 与 AI / IoT 的关系

### 8.1 AI

- Agent 输出优先走流，再按策略物化成消息或卡片
- tool call 过程可走结构化流
- AI summary 可作为投影或附属消息写回

### 8.2 IoT

- 设备上行 telemetry 直接进入统一流模型
- 设备控制命令可作为消息、信令或 command stream 表达
- 设备回执和异常通知统一走通知与事件链

### 8.3 IoT 协议插件与设备管理

- IoT 接入必须同时具备设备管理与接入体系：`device registry / credential / session / presence / twin / telemetry / command`。
- 当前冻结的协议插件为 `MQTT` 与开源 `小智协议`，两者都必须映射到统一 `device.telemetry`、`device.command` 和 `device twin` 模型。
- 协议差异只能停留在 `IotProtocolAdapter`，不得把 topic、QoS、协议帧细节扩散到消息、流和通知主链。

## 9. 读写分离策略

- 会话命令成功后先提交 event
- 流 durable 部分按 `StreamStore` 规则写入
- 通知和自动化通过 side-effect pipeline 异步触发
- RTC 信令与消息事件可共用 envelope，但消费链独立

## 10. 结论

`craw-chat` 在实时层的核心策略是：缓存只做辅助、流是真正一等能力、RTC 只做信令、RTC/object-storage/IoT 通过插件体系接入、通知永远不阻塞提交主链。这样才能同时兼容 IM、AI、IoT 和后续协作能力的统一演进。
