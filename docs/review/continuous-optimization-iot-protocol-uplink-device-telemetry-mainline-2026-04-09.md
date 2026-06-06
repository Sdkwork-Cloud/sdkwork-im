# continuous-optimization - IoT protocol uplink device.telemetry mainline - 2026-04-09

## 当前轮次

- step: `08`
- wave: `08-I`
- 目标：让 `IotProtocolAdapter::decode_uplink()` 首次进入真实 runtime mainline

## 为什么做这一轮

- `iot-mqtt` 已经实现 `IotProtocolAdapter`
- `local-minimal-node` 已有 `device.telemetry` stream 主链路
- 但此前还没有任何 route 真实消费 `decode_uplink()`

## TDD 记录

- 先新增：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
- 初始失败原因：
  - `POST /app/v3/api/iot/protocol/uplink` 返回 `404`
- 中间修正：
  - 首个最小实现后出现 `409`
  - 注入式 adapter 测试暴露 `device_id` 没有从 `auth.device_id` 回退
  - 最终冻结为 `request.device_id.or_else(|| auth.device_id.clone())`
- 绿灯确认：
  - 返回 `200`
  - `streamType = device.telemetry`
  - `frameType = telemetry`
  - `schemaRef = cc.device.telemetry.v1`
  - 注入式 `build_default_app_with_iot_protocol_adapter` 被真实消费

## 实际改动

- 更新 route handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 更新 access guard：
  - `services/local-minimal-node/src/node/access.rs`
- 更新 route 装配：
  - `services/local-minimal-node/src/node/build.rs`
- 新增主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 本轮冻结的设计事实

- route：
  - `POST /app/v3/api/iot/protocol/uplink`
- runtime seam：
  - `IotProtocolAdapter::decode_uplink()`
- 统一目标：
  - `device.telemetry`
- stream id：
  - `st_device_telemetry_{device_id}`
- schema：
  - `cc.device.telemetry.v1`

## 边界

- 本轮只闭环 `uplink -> device.telemetry`
- 不是 `encode_downlink`
- 不是 `device.command`
- 不是完整 IoT 协议网关
- 不是 `iot-xiaozhi` runtime 完成交付

## 验证结果

- 已通过：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 下一轮动作

- 优先收敛 `encode_downlink -> device.command` 的首条真实 mainline consumption
