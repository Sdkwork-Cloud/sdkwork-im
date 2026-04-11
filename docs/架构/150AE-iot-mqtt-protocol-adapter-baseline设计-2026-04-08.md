# 150AE - iot-mqtt protocol adapter baseline 设计

## 设计目标

`iot-mqtt` 需要先从 provider registry 中的静态声明，升级成一个可编译、可测试、可被后续 runtime 复用的 `IotProtocolAdapter` baseline。

## 当前最小 contract

`iot-mqtt` 至少实现：

- `protocol_key`
- `decode_uplink`
- `encode_downlink`
- `provider_health_snapshot`

## 归一化规则

- uplink 输入是 MQTT 侧 `topic + payload`
- 输出统一归一化为：
  - `device.telemetry`
  - `IotProtocolEnvelope`
- downlink 输入是统一 `device.command` 语义
- 输出统一归一化为 MQTT 侧 payload

MQTT 私有差异只允许停留在：

- `topic`
- `qos`
- `brokerEndpoint`

这些信息可以保留在 `attributes` 或 health snapshot，但不得泄漏成业务主链的条件分支。

## 设计原则

- `iot-mqtt` 只负责协议映射，不负责设备注册、owner 绑定或 twin 生命周期
- `DeviceAccessProvider` 与 `IotProtocolAdapter` 继续保持双层分离
- `ProviderRegistry` 继续是选择 `iot-mqtt` 的唯一治理入口

## 非目标

- 不补 `iot-xiaozhi`
- 不补完整 MQTT broker integration
- 不补 HTTP provider surface
- 不补 `DeviceAccessProvider` runtime 装配
