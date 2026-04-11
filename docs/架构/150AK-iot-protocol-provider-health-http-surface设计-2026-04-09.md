# 150AK - IoT protocol provider health HTTP surface 设计

## 1. 目标

把已经存在的 `IotProtocolAdapter` 运行时注入结果，变成 `local-minimal-node` 上一条可直接观测的 HTTP 路由。

## 2. 设计约束

- 只做 `protocol health surface`，不做 `decode_uplink / encode_downlink` HTTP API。
- 不把协议健康接口混成设备管理 API。
- 继续复用 `provider-health` 的统一路径模式。

## 3. 路由设计

本轮冻结路由：

- `GET /api/v1/iot/protocol/provider-health`

返回体直接复用 `ProviderHealthSnapshot`，不额外包装。

## 4. owner seam

`local-minimal-node` 必须保存当前注入的 `Arc<dyn IotProtocolAdapter>`，而不是在 handler 内部临时构造默认实现。

因此本轮冻结以下注入与消费边界：

- `build_default_app_with_iot_protocol_adapter`
- `build_default_app_with_runtime_dir_and_iot_protocol_adapter`
- `AppState::iot_protocol_provider_health()`
- `IotProtocolAdapter::provider_health_snapshot()`

这样 route 读取的是实际 runtime adapter，而不是隐藏默认值。

## 5. 默认返回语义

当前默认 adapter 为 `iot-mqtt`，因此最小健康视图冻结为：

- `pluginId = iot-mqtt`
- `status = healthy`
- `details.providerKind = mqtt`
- `details.protocolKey = mqtt`

## 6. 为什么只做 health

仓库里虽然已经有：

- `decode_uplink`
- `encode_downlink`
- `device.telemetry`
- `device.command`

但这些能力仍主要停留在 adapter contract 层，还没有一个稳定、冻结的运行时主消费路径。

所以这轮只能诚实声称：

- IoT protocol external HTTP surface 已有第一条真实路径

不能声称：

- 不是 `decode_uplink / encode_downlink` HTTP API
- 不是完整 IoT 协议网关
- 不是设备管理 API
- 不是 `iot-xiaozhi` 已完成 runtime 对接

## 7. 测试策略

采用 TDD：

- 先新增 `iot_provider_http_test`
- 确认 `GET /api/v1/iot/protocol/provider-health` 初始返回 `404`
- 再补最小 route / injection / owner seam
- 绿灯验证：
  - `pluginId = iot-mqtt`
  - `providerKind = mqtt`
  - `protocolKey = mqtt`

## 8. 闭环判断

本轮完成后，只能说明：

- `local-minimal-node` 已具备第一条 `IotProtocolAdapter` 健康可见路径

下一真实缺口应转向：

- `IotProtocolAdapter` 在 runtime mainline 中的首次真实消费
