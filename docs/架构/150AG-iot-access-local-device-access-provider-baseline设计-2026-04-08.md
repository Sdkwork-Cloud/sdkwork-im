# 150AG - iot-access-local device access provider baseline 设计

## 设计目标

`iot-access-local` 需要从 provider registry 中的静态默认项升级成一个可编译、可测试、可被后续 runtime 复用的 `DeviceAccessProvider` baseline。

## 当前最小 contract

`iot-access-local` 至少实现：

- `register_device`
- `bind_owner`
- `disable_device`
- `provider_health_snapshot`

## 本地设备管理与接入边界

- `register_device`
  - 负责本地 device registry 基线
  - 生成最小 credential secret
  - 返回默认协议分配 `mqtt / xiaozhi`
- `bind_owner`
  - 负责 owner 绑定最小闭环
- `disable_device`
  - 负责设备禁用最小闭环
- `provider_health_snapshot`
  - 负责暴露 provider 健康、默认协议与本地模式信息

## 设计原则

- `iot-access-local` 只负责设备管理与接入体系，不负责 `MQTT / xiaozhi` 协议帧编解码
- `DeviceAccessProvider` 与 `IotProtocolAdapter` 继续保持双层分离
- `ProviderRegistry` 继续是选择 `iot-access-local` 的唯一治理入口

## 非目标

- 不补 `iot-xiaozhi`
- 不补 IoT provider 外部 HTTP surface
- 不补 `local-minimal-node / session-gateway` 的 provider-aware runtime 注入
- 不把现有设备注册主路径强行改写成 provider 消费，只先交付 baseline crate
