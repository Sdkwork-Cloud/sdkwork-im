# 08-I - IoT protocol uplink 接入 device.telemetry mainline

## 本轮目标

在 `iot-mqtt` 已经作为默认 `IotProtocolAdapter` 注入、且 `protocol provider-health` 已可见的前提下，补齐第一条真实 `runtime mainline consumption`：

- `local-minimal-node`
- `POST /api/v1/iot/protocol/uplink`
- `IotProtocolAdapter::decode_uplink()`
- 写入统一 `device.telemetry` stream mainline

本轮只闭环：

- `uplink -> decode_uplink -> device.telemetry`
- 默认 `iot-mqtt` uplink 主链路
- `build_default_app_with_iot_protocol_adapter` 的真实 runtime 消费

本轮不闭环：

- `encode_downlink`
- `device.command` 下行主链路
- 完整 IoT 协议网关
- `iot-xiaozhi` 真实 runtime adapter

## 发现的问题

- `IotProtocolAdapter` 虽然已经定义 `decode_uplink / encode_downlink`，但此前只停留在 contract 和 provider-health 可见性层。
- `local-minimal-node` 已经有 `device.telemetry` stream 主链路能力，但还没有任何 IoT protocol route 真实消费 `decode_uplink()`。
- 如果不把协议 uplink 写入统一 stream mainline，`iot-mqtt` 仍然只是“被注入”，不是“被运行时真实使用”。

## 本轮决策

- 最小 route 固定为：
  - `POST /api/v1/iot/protocol/uplink`
- 请求先构造 `IotProtocolDecodeRequest`，其中 `device_id` 取：
  - `request.device_id.or_else(|| auth.device_id.clone())`
- 解码后只落到一个冻结的统一目标：
  - `streamType = device.telemetry`
  - `frameType = telemetry`
  - `schemaRef = cc.device.telemetry.v1`
  - `scopeKind = device`
  - `scopeId = device_id`
- stream id 冻结为：
  - `st_device_telemetry_{device_id}`
- route 必须消费当前注入的 `IotProtocolAdapter`，不能在 handler 内部偷偷 new 默认 adapter。

## 实际落地

- 新增主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`
- 新增 uplink handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 新增 uplink access guard：
  - `services/local-minimal-node/src/node/access.rs`
- 在 `build.rs` 装配 route：
  - `POST /api/v1/iot/protocol/uplink`

## 验证

- 红灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
  - 初始失败点：`POST /api/v1/iot/protocol/uplink` 返回 `404`
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 结果

- `IotProtocolAdapter::decode_uplink()` 已首次进入真实 runtime mainline。
- 默认 `iot-mqtt` uplink 已可进入统一 `device.telemetry` stream。
- 注入式 `build_default_app_with_iot_protocol_adapter` 已被测试覆盖，证明 route 消费的是注入 adapter，而不是隐藏默认值。

## 下一轮建议

- 优先考虑 `encode_downlink -> device.command` 的首条真实 mainline consumption。
- 继续保持最小闭环，不要跳成完整 IoT 网关或设备管理写 API。
