# continuous-optimization - IoT protocol provider HTTP surface - 2026-04-09

## 当前轮次

- step: `08`
- wave: `08-H`
- 目标：补齐第一条真实 `IoT protocol external HTTP surface`

## 为什么做这一轮

- `iot-mqtt` 已经实现 `IotProtocolAdapter`
- `local-minimal-node` 已经有 access/media/rtc 的 `provider-health`
- 但 IoT protocol 仍然缺少统一 HTTP 可见性

## TDD 记录

- 先新增：
  - `services/local-minimal-node/tests/iot_provider_http_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`
- 初始失败原因：
  - `GET /api/v1/iot/protocol/provider-health` 返回 `404`
- 绿灯确认：
  - 返回 `200`
  - `pluginId = iot-mqtt`
  - `details.providerKind = mqtt`
  - `details.protocolKey = mqtt`

## 实际改动

- 新增默认 adapter 依赖：
  - `services/local-minimal-node/Cargo.toml`
- 更新 runtime 装配：
  - `services/local-minimal-node/src/node/build.rs`
- 更新 owner seam：
  - `services/local-minimal-node/src/node.rs`
- 新增 handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 更新测试：
  - `services/local-minimal-node/tests/iot_provider_http_test.rs`

## 本轮冻结的设计事实

- route：
  - `GET /api/v1/iot/protocol/provider-health`
- 响应源：
  - `IotProtocolAdapter::provider_health_snapshot()`
- runtime 注入入口：
  - `build_default_app_with_iot_protocol_adapter`
- 默认 provider：
  - `iot-mqtt`

## 边界

- 本轮只闭环 `protocol health`
- 不是 `decode_uplink / encode_downlink` HTTP API
- 不是设备管理 API
- 不是 `iot-xiaozhi` runtime 完成交付

## 验证结果

- 已通过：
  - `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`

## 下一轮动作

- 优先收敛 `IotProtocolAdapter` 的 runtime mainline consumption
