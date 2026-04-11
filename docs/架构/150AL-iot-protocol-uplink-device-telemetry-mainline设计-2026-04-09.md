# 150AL - IoT protocol uplink -> device.telemetry mainline 设计

## 1. 目标

把已经存在的 `IotProtocolAdapter::decode_uplink()` 从“仅有 contract”推进到“被 `local-minimal-node` 真实 runtime 消费”，并归一接入 `device.telemetry` stream mainline。

## 2. 设计约束

- 只实现 `uplink` 入口，不实现 `encode_downlink`。
- 不扩成完整 IoT 协议网关。
- 不直接暴露一个抽象的 `decode_uplink / encode_downlink` HTTP 调试 API。
- 继续复用现有 `streaming_runtime`、`device.telemetry` 与 realtime stream-frame side effect。

## 3. 路由与数据流

本轮冻结 route：

- `POST /api/v1/iot/protocol/uplink`

冻结数据流：

1. 解析 auth context。
2. 构造 `IotProtocolDecodeRequest`。
3. 调用当前注入的 `IotProtocolAdapter::decode_uplink()`。
4. 校验 `device_id` 属于已注册且已绑定设备。
5. 打开或复用 `st_device_telemetry_{device_id}`。
6. 追加 `device.telemetry` frame。
7. 发布已有 realtime stream-frame event。

## 4. 请求归一规则

`IotProtocolDecodeRequest.device_id` 冻结为：

- `request.device_id.or_else(|| auth.device_id.clone())`

这样既支持显式请求体，也支持设备 actor 直接从 auth context 传入绑定 `device_id`。

## 5. Stream 语义冻结

本轮冻结统一目标：

- `streamType = device.telemetry`
- `frameType = telemetry`
- `schemaRef = cc.device.telemetry.v1`
- `scopeKind = device`
- `scopeId = envelope.device_id`

`attributes` 直接沿用 `IotProtocolEnvelope.attributes`，因此 `protocol`、`topic` 等协议侧元数据可以继续透传。

## 6. 注入边界

`local-minimal-node` 必须消费运行时注入的 `Arc<dyn IotProtocolAdapter>`，因此本轮设计冻结：

- `build_default_app_with_iot_protocol_adapter`
- `build_default_app_with_runtime_dir_and_iot_protocol_adapter`
- `AppState.iot_protocol_adapter`
- route 直接调用 `decode_uplink()`

这样测试可以证明：route 消费的是被注入 provider，而不是隐藏默认值。

## 7. 边界与未完成项

本轮可以声明：

- `IotProtocolAdapter` 已有首条真实 runtime mainline consumption
- 默认 `iot-mqtt` uplink 已能进入统一 `device.telemetry`

本轮不能声明：

- 不是 `encode_downlink` 已打通
- 不是 `device.command` 已接通
- 不是完整设备管理 API
- 不是 `iot-xiaozhi` runtime adapter 已完成对接

## 8. 测试策略

采用 TDD：

- 先新增 `iot_protocol_adapter_mainline_test`
- 确认 `POST /api/v1/iot/protocol/uplink` 初始返回 `404`
- 再补最小 route / access / stream ingestion
- 绿灯验证：
  - `streamType = device.telemetry`
  - `frameType = telemetry`
  - `schemaRef = cc.device.telemetry.v1`
  - 注入 adapter 被真实消费
