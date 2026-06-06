# 09AM - 实施计划 - IoT protocol downlink 接入 device.command mainline

## 目标

让 `local-minimal-node` 首次真实消费 `IotProtocolAdapter::encode_downlink()`，并把协议下行归一写入 `device.command` stream mainline。

## 输入

- `docs/prompts/反复执行Step指令.md`
- `docs/step/08-I-IoT-protocol-uplink接入device-telemetry-mainline-2026-04-09.md`
- `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`
- `adapters/iot-mqtt/README.md`
- `services/local-minimal-node/tests/device_stream_e2e_test.rs`

## 范围

本轮包含：

- `POST /app/v3/api/iot/protocol/downlink`
- `IotProtocolAdapter::encode_downlink()`
- `build_default_app_with_iot_protocol_adapter` 的 runtime 主链路消费
- `device.command` stream 写入
- `schemaRef = cc.device.command.v1`
- `streamId = st_device_command_{device_id}`
- `device.command.send` 权限校验
- 测试与文档回写

本轮不包含：

- 真实设备投递确认
- ACK / retry / timeout
- `iot-xiaozhi` 真实协议适配器对接
- 完整下行投递队列

## 执行步骤

1. 先写失败测试，确认当前 `POST /app/v3/api/iot/protocol/downlink` 返回 `404`。
2. 在 `iot.rs` 增加 downlink handler，消费当前注入的 `IotProtocolAdapter`。
3. 在 `access.rs` 增加最小访问控制，确保设备已注册且调用方具备 `device.command.send`。
4. 调用 `encode_downlink()` 后，归一写入 `device.command` stream，并复用现有 realtime stream-frame side effect。
5. 回写 `step / 架构 / review` 与文档测试。

## 验证

- `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- `cargo fmt --all --check`

## 退出标准

- `POST /app/v3/api/iot/protocol/downlink` 返回 `200`
- 响应包含：
  - `frame`
  - `protocolPayload`
- `frame.streamType = device.command`
- 默认 `iot-mqtt` downlink 可编码为协议 payload 并写入统一 `device.command`
- 注入式 adapter 测试证明 route 真实消费 `build_default_app_with_iot_protocol_adapter`
- 文档明确声明：本轮只闭环 `downlink -> device.command`，不是完整设备投递系统
