# 150AM - IoT protocol downlink -> device.command mainline 设计

## 1. 目标

把已经存在的 `IotProtocolAdapter::encode_downlink()` 从“仅有 contract”推进到“被 `local-minimal-node` 真实 runtime 消费”，并归一接入 `device.command` stream mainline。

## 2. 设计约束

- 只实现 `downlink` 入口，不实现真实设备投递确认。
- 不扩成完整 IoT 协议网关。
- 不新增脱离主链路的 `encode_downlink` 调试 API。
- 继续复用现有 `streaming_runtime`、`device.command` 与 realtime stream-frame side effect。

## 3. 路由与数据流

本轮冻结 route：

- `POST /app/v3/api/iot/protocol/downlink`

冻结数据流：

1. 解析 auth context。
2. 校验设备已注册且调用方具备 `device.command.send`。
3. 构造 `IotProtocolEncodeRequest`。
4. 调用当前注入的 `IotProtocolAdapter::encode_downlink()`。
5. 打开或复用 `st_device_command_{device_id}`。
6. 追加 `device.command` frame。
7. 发布已有 realtime stream-frame event。
8. 返回 `frame + protocolPayload`。

## 4. 请求与权限冻结

本轮请求体冻结为：

- `deviceId`
- `channel`
- `payloadJson`

本轮权限边界冻结为：

- 设备必须已注册且属于当前 principal
- 调用方必须具备 `device.command.send`

## 5. Stream 语义冻结

本轮冻结统一目标：

- `streamType = device.command`
- `frameType = command`
- `schemaRef = cc.device.command.v1`
- `scopeKind = device`
- `scopeId = deviceId`

命令帧保存统一业务 payload，协议侧编码结果通过 `protocolPayload` 在 downlink 响应中返回，不把 MQTT 私有 broker 投递状态扩散成业务主链状态机。

## 6. 注入边界

`local-minimal-node` 必须消费运行时注入的 `Arc<dyn IotProtocolAdapter>`，因此本轮设计冻结：

- `build_default_app_with_iot_protocol_adapter`
- `build_default_app_with_runtime_dir_and_iot_protocol_adapter`
- `AppState.iot_protocol_adapter`
- route 直接调用 `encode_downlink()`

这样测试可以证明：route 消费的是被注入 provider，而不是隐藏默认值。

## 7. 边界与未完成项

本轮可以声明：

- `IotProtocolAdapter` 已有首条真实 runtime `downlink` mainline consumption
- 默认 `iot-mqtt` downlink 已能进入统一 `device.command`

本轮不能声明：

- 不是设备已真实收到命令
- 不是 ACK / retry / timeout 已完成
- 不是完整投递队列
- 不是 `iot-xiaozhi` runtime adapter 已完成对接

## 8. 测试策略

采用 TDD：

- 先扩展 `iot_protocol_adapter_mainline_test`
- 确认 `POST /app/v3/api/iot/protocol/downlink` 初始返回 `404`
- 再补最小 route / access / stream ingestion
- 绿灯验证：
  - `frame.streamType = device.command`
  - `frame.schemaRef = cc.device.command.v1`
  - `protocolPayload` 存在
  - 注入 adapter 被真实消费
