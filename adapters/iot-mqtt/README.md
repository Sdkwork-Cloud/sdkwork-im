# iot-mqtt

通用 `MQTT` 协议插件基线。

## 统一契约

- 实现 `IotProtocolAdapter`
- `protocol_key() = "mqtt"`
- `decode_uplink(...)`
- `encode_downlink(...)`
- `provider_health_snapshot()`

## 运行时边界

- provider 选择由 `ProviderRegistry` 决定。
- 上行统一归一化到 `device.telemetry`。
- 下行统一归一化到 `device.command`。
- `topic / qos / broker endpoint` 只保留在 adapter metadata，不向业务主链泄漏 MQTT 私有协议分支。
