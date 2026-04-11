# iot-access-local

通用 `iot-access-local` 设备接入插件基线。

## 统一契约

- 实现 `DeviceAccessProvider`
- `register_device(...)`
- `bind_owner(...)`
- `disable_device(...)`
- `provider_health_snapshot()`

## 运行时边界

- provider 选择由 `ProviderRegistry` 决定。
- 当前聚焦本地 device registry、owner 绑定与设备禁用基线。
- 默认协议分配为 `mqtt` 与 `xiaozhi`。
- 设备管理和接入体系停留在 `DeviceAccessProvider`，不向业务主链泄漏私有实现细节。
