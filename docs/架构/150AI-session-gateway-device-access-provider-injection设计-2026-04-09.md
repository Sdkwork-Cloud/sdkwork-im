# 150AI - session-gateway DeviceAccessProvider 注入设计

## 1. 目标

让 `session-gateway` 在真实首设备接入主链路中消费 `DeviceAccessProvider`，并保持与 `local-minimal-node` 一致的 IoT provider 运行时装配标准。

## 2. 设计约束

- 不新增伪造的 `/api/v1/devices/register` 接口。
- 真实入口固定为 `POST /api/v1/sessions/resume`。
- heartbeat / realtime route preflight 会复用注册路径，因此 provider 不能被重复触发。
- provider 错误必须回到统一 HTTP 错误模型。

## 3. 装配设计

`session-gateway` 新增两类装配入口：

- `build_app_with_device_access_provider`
- `build_app_with_cluster_and_device_access_provider`

默认运行时通过 `im-adapter-iot-access-local::LocalDeviceAccessProvider` 完成装配，保证无额外配置时仍然落到统一 `iot-access-local` baseline。

## 4. 调用链路

真实调用顺序冻结为：

1. `POST /api/v1/sessions/resume`
2. `SessionDeviceRegistration::register_device`
3. `SessionSyncState::has_registered_device`
4. 首次注册时调用：
   - `DeviceAccessProvider::register_device`
   - `DeviceAccessProvider::bind_owner`
5. 再进入：
   - presence register
   - realtime ensure state
   - session sync register
   - route bind

## 5. 首注册保护

`session-gateway` 没有 projection 视图，因此首注册判断不走 projection，而走 `SessionSyncState`。

判定标准：

- 若 `SessionSyncState` 中已经存在该 `tenant / principal / device`，则不再触发 provider
- 这样可以保证：
  - `resume` 首次接入会注册
  - `heartbeat`
  - realtime route preflight
  - 同一运行时内的重复 resume
  都不会重复触发外部设备接入逻辑

## 6. 请求常量

本轮冻结：

- `product_id = session-gateway-device`
- `credential_kind = session`

后续若要做 provider policy 或 deployment profile 定制，应通过控制面下发，而不是在 handler 中写分支。

## 7. 错误映射

`ContractError` 统一映射为：

- `UnsupportedCapability -> 501 / provider_capability_unsupported`
- `Conflict -> 409 / provider_conflict`
- `Unavailable -> 503 / provider_unavailable`

这样 `session-gateway` 与 `local-minimal-node` 在 provider 失败语义上保持一致。

## 8. 测试策略

采用 TDD：

- 先新增 `device_access_provider_mainline_test`
- 先让测试因缺少 builder seam 失败
- 再补最小实现
- 绿灯要求同时验证：
  - `resume` 成功
  - `heartbeat` 成功
  - provider 只被调用一次
  - 请求体携带正确的 tenant / user / device / session / product / credential 常量

## 9. 闭环判断

本轮完成后，可认为：

- `local-minimal-node`
- `session-gateway`

这两条 IoT 运行时主链路都已真实接入 `DeviceAccessProvider`。

但仍不能宣称 IoT provider 体系整体完成，因为下列缺口仍在：

- IoT provider external HTTP surface
- `iot-xiaozhi` 真实 runtime adapter
- external 源码真实对齐与 submodule 落位
