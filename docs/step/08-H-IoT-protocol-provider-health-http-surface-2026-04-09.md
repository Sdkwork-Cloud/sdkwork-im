# 08-H - IoT protocol provider health HTTP surface

## 本轮目标

在 `iot-mqtt` 已实现 `IotProtocolAdapter` baseline 的前提下，补齐第一条真实 `IoT protocol external HTTP surface`：

- `local-minimal-node`
- `GET /app/v3/api/iot/protocol/provider_health`
- 当前注入 `IotProtocolAdapter` 的健康可见性

本轮只闭环：

- `local-minimal-node`
- `IotProtocolAdapter::provider_health_snapshot()`
- 默认 `iot-mqtt` health 对外可见

本轮不闭环：

- `decode_uplink / encode_downlink` HTTP API
- 设备管理写接口
- `iot-xiaozhi` 真实 runtime adapter
- 多运行时统一协议路由

## 发现的问题

- `iot-mqtt` 已经实现 `IotProtocolAdapter`，但只存在于 crate 内部 contract。
- `local-minimal-node` 已经有：
  - `GET /app/v3/api/iot/access/provider_health`
  - `GET /app/v3/api/media/provider_health`
  - `GET /backend/v3/api/rtc/provider_health`
- IoT protocol 仍然缺少第一条统一 HTTP 观察路径，无法直接确认当前 runtime 注入的协议适配器是谁。

## 本轮决策

- 只补 `protocol health surface`，不扩成协议收发 API。
- route 固定为：
  - `GET /app/v3/api/iot/protocol/provider_health`
- 响应直接返回当前注入的 `IotProtocolAdapter::provider_health_snapshot()`。
- `local-minimal-node` 显式增加 `build_default_app_with_iot_protocol_adapter` 注入端口，避免 route 隐式 new 默认适配器。

## 实际落地

- 新增测试：
  - `services/local-minimal-node/tests/iot_provider_http_test.rs`
- 新增默认 adapter 装配：
  - `services/local-minimal-node/Cargo.toml`
  - `services/local-minimal-node/src/node/build.rs`
- 新增 owner seam：
  - `services/local-minimal-node/src/node.rs`
- 新增 route handler：
  - `services/local-minimal-node/src/node/iot.rs`

## 验证

- 红灯：
  - `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`
  - 初始失败点：`GET /app/v3/api/iot/protocol/provider_health` 返回 `404`
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`

## 结果

- `local-minimal-node` 已新增：
  - `GET /app/v3/api/iot/protocol/provider_health`
- 当前响应来自：
  - `IotProtocolAdapter::provider_health_snapshot()`
- 默认 `iot-mqtt` 已可通过 HTTP 直接观察：
  - `pluginId = iot-mqtt`
  - `details.providerKind = mqtt`
  - `details.protocolKey = mqtt`

## 下一轮建议

- 优先进入 `IotProtocolAdapter` 的真实 mainline runtime consumption。
- 最小候选不是设备管理 API，而是 `decode_uplink / encode_downlink` 在运行时主链路中的首次真实消费。
