# 150AJ - IoT access provider health HTTP surface 设计

## 1. 目标

让已经被 runtime 装配的 `DeviceAccessProvider` 进入统一 HTTP 观测面，形成第一条真实 IoT provider external HTTP surface。

## 2. 设计约束

- 只做 health surface，不做设备管理写 API。
- 不把 access provider health 和 protocol surface 混成一个不清晰的接口。
- 优先复用 `local-minimal-node` 已存在的 `media / rtc provider-health` 路由模式。

## 3. 路由设计

本轮冻结路由：

- `GET /api/v1/iot/access/provider-health`

返回体直接复用 `ProviderHealthSnapshot`，不再额外包一层自定义对象。

## 4. owner seam

路由 handler 不直接持有 provider，而是通过 `LocalNodeDeviceRegistration` 暴露：

- `provider_health_snapshot()`

这样可以保持 IoT access provider 的消费边界仍然收敛在 device registration owner seam，而不是再散落到 route 层。

## 5. 当前返回语义

当前默认 provider 为 `iot-access-local`，因此外部可见的最小健康信息冻结为：

- `pluginId = iot-access-local`
- `status = healthy`
- `details.providerKind = local`
- `details.assignedProtocols = mqtt,xiaozhi`

## 6. 为什么只做 access health

当前仓库虽然已经有：

- `DeviceAccessProvider`
- `IotProtocolAdapter`

但 runtime 真实外部 surface 只对 access provider 有稳定注入和统一 owner seam。

因此这一轮只交付 access provider health，是最小真实闭环；如果强行把 protocol surface 一起做，会把还没冻结的运行时选择面混进来。

## 7. 测试策略

采用 TDD：

- 先新增 `iot_provider_http_test`
- 确认当前返回 `404`
- 再补最小 route / handler / owner seam
- 绿灯要求验证：
  - `200`
  - `pluginId = iot-access-local`
  - `providerKind = local`
  - `assignedProtocols = mqtt,xiaozhi`

## 8. 闭环判断

本轮完成后，只能说明：

- IoT access provider external HTTP surface 已迈出第一条真实 health 路径

不能说明：

- 不是 protocol surface
- 不是完整 access 管理 API
- IoT protocol external HTTP surface 完成
- IoT provider 整体管理面完成
