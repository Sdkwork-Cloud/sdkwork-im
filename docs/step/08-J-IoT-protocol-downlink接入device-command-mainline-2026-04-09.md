# 08-J - IoT protocol downlink 接入 device.command mainline

## 本轮目标

在 `08-I` 已把 `decode_uplink()` 接入 `device.telemetry` 主链路的前提下，补齐对称的下行最小闭环：

- `local-minimal-node`
- `POST /app/v3/api/iot/protocol/downlink`
- `IotProtocolAdapter::encode_downlink()`
- 写入统一 `device.command` stream mainline

本轮只闭环：

- `downlink -> encode_downlink -> device.command`
- 默认 `iot-mqtt` downlink 主链路
- `build_default_app_with_iot_protocol_adapter` 的真实 runtime 消费

本轮不闭环：

- 真实设备投递确认
- ACK / retry / timeout
- `iot-xiaozhi` 真实 runtime adapter
- 完整 IoT 协议网关

## 发现的问题

- `IotProtocolAdapter::encode_downlink()` 仍只停留在 adapter contract，未进入任何真实 runtime 路由。
- `device.command` 已经通过统一 stream 主链路存在，但此前没有协议层 route 把下行命令归一写入该主链路。
- 如果不把 `encode_downlink()` 接入 `device.command`，IoT protocol 仍只有上行主链路，没有形成协议层上下行对称闭环。

## 本轮决策

- 最小 route 固定为：
  - `POST /app/v3/api/iot/protocol/downlink`
- 请求体固定包含：
  - `deviceId`
  - `channel`
  - `payloadJson`
- route 先做权限校验：
  - 设备已注册且已绑定
  - 调用方具备 `device.command.send`
- route 先调用 `encode_downlink()` 生成协议侧 payload，再写入统一 stream。
- 统一目标冻结为：
  - `streamType = device.command`
  - `frameType = command`
  - `schemaRef = cc.device.command.v1`
  - `scopeKind = device`
  - `scopeId = deviceId`
- stream id 冻结为：
  - `st_device_command_{device_id}`
- 响应冻结为：
  - `frame`
  - `protocolPayload`

## 实际落地

- 扩展主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`
- 新增 downlink handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 新增 downlink access guard：
  - `services/local-minimal-node/src/node/access.rs`
- 在 `build.rs` 装配 route：
  - `POST /app/v3/api/iot/protocol/downlink`

## 验证

- 红灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
  - 初始失败点：`POST /app/v3/api/iot/protocol/downlink` 返回 `404`
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 结果

- `IotProtocolAdapter::encode_downlink()` 已首次进入真实 runtime mainline。
- 默认 `iot-mqtt` downlink 已可进入统一 `device.command` stream。
- 注入式 `build_default_app_with_iot_protocol_adapter` 已被测试覆盖，证明 route 真实消费的是注入 adapter。

## 下一轮建议

- 优先考虑从 `device.command` 向真实协议投递 / 消费确认的最小闭环。
- 继续避免把本地闭环误写成“完整设备投递系统已完成”。
