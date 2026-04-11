# continuous-optimization - IoT protocol downlink device.command mainline - 2026-04-09

## 当前轮次

- step: `08`
- wave: `08-J`
- 目标：让 `IotProtocolAdapter::encode_downlink()` 首次进入真实 runtime mainline

## 为什么做这一轮

- `iot-mqtt` 已经实现 `IotProtocolAdapter`
- `08-I` 已完成 `decode_uplink -> device.telemetry`
- 但 `encode_downlink()` 仍没有任何真实 route 消费

## TDD 记录

- 先扩展：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
- 初始失败原因：
  - `POST /api/v1/iot/protocol/downlink` 返回 `404`
- 绿灯确认：
  - 返回 `200`
  - `frame.streamType = device.command`
  - `frame.schemaRef = cc.device.command.v1`
  - 响应包含 `protocolPayload`
  - 注入式 `build_default_app_with_iot_protocol_adapter` 被真实消费

## 实际改动

- 更新 route handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 更新 access guard：
  - `services/local-minimal-node/src/node/access.rs`
- 更新 route 装配：
  - `services/local-minimal-node/src/node/build.rs`
- 扩展主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 本轮冻结的设计事实

- route：
  - `POST /api/v1/iot/protocol/downlink`
- runtime seam：
  - `IotProtocolAdapter::encode_downlink()`
- 统一目标：
  - `device.command`
- stream id：
  - `st_device_command_{device_id}`
- schema：
  - `cc.device.command.v1`
- 权限：
  - `device.command.send`

## 边界

- 本轮只闭环 `downlink -> device.command`
- 不是设备真实投递确认
- 不是 ACK / retry / timeout
- 不是完整 IoT 协议网关
- 不是 `iot-xiaozhi` runtime 完成交付

## 验证结果

- 已通过：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 下一轮动作

- 优先收敛 `device.command` 与真实协议投递确认之间的最小可验证闭环
