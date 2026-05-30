# 09AL - 实施计划 - IoT protocol uplink 接入 device.telemetry mainline

## 目标

让 `local-minimal-node` 首次真实消费 `IotProtocolAdapter::decode_uplink()`，并把协议 uplink 归一写入 `device.telemetry` stream mainline。

## 输入

- `docs/prompts/反复执行Step指令.md`
- `docs/step/08-H-IoT-protocol-provider-health-http-surface-2026-04-09.md`
- `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`
- `services/local-minimal-node/tests/device_stream_e2e_test.rs`
- `services/local-minimal-node/src/node/iot.rs`

## 范围

本轮包含：

- `POST /backend/v3/api/iot/protocol/uplink`
- `IotProtocolAdapter::decode_uplink()`
- `build_default_app_with_iot_protocol_adapter` 的 runtime 主链路消费
- `device.telemetry` stream 写入
- `schemaRef = cc.device.telemetry.v1`
- `streamId = st_device_telemetry_{device_id}`
- 测试与文档回写

本轮不包含：

- `encode_downlink`
- `device.command` 写入主链路
- `iot-xiaozhi` 真实协议适配器对接
- 多 provider 协议路由编排

## 执行步骤

1. 先写失败测试，确认当前 `POST /backend/v3/api/iot/protocol/uplink` 返回 `404`。
2. 在 `iot.rs` 增加 uplink handler，消费当前注入的 `IotProtocolAdapter`。
3. 在 `access.rs` 增加最小访问控制，确保写入来自已绑定设备且设备已注册。
4. 归一写入 `device.telemetry` stream，并复用现有 realtime stream-frame side effect。
5. 回写 `step / 架构 / review` 与文档测试。

## 验证

- `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- `cargo fmt --all --check`

## 退出标准

- `POST /backend/v3/api/iot/protocol/uplink` 返回 `200`
- 响应是统一 `device.telemetry` frame
- 默认 `iot-mqtt` uplink 可落到 `streamType = device.telemetry`
- 注入式 adapter 测试证明 route 真实消费 `build_default_app_with_iot_protocol_adapter`
- 文档明确声明：本轮只闭环 `uplink -> device.telemetry`，不是 `encode_downlink` 或完整 IoT 网关
