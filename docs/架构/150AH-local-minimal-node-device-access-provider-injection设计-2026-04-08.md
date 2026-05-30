# 150AH - local-minimal-node DeviceAccessProvider 注入设计

## 设计目标

把 `DeviceAccessProvider` 从静态 provider 矩阵推进到 `local-minimal-node` 的真实设备注册运行时。

## 设计边界

- 运行时：`local-minimal-node`
- provider：`DeviceAccessProvider`
- 默认实现：`iot-access-local`
- 非目标：`session-gateway`、`iot-xiaozhi` 真实 adapter、IoT provider external HTTP surface

## 装配设计

`local-minimal-node` 新增两条注入入口：

- `build_default_app_with_device_access_provider`
- `build_default_app_with_runtime_dir_and_device_access_provider`

默认路径继续自动装配：

- `im-adapter-iot-access-local::LocalDeviceAccessProvider`

这保证：

- 测试可以替换 provider
- 默认运行时仍然不需要额外配置
- provider 注入模式与 `PrincipalProfileProvider` 保持一致

## 运行时调用设计

`LocalNodeDeviceRegistration` 成为 `DeviceAccessProvider` 在 `local-minimal-node` 中的唯一消费点。

调用顺序固定为：

1. route / resume preflight
2. projection 中首次注册判定
3. 若不存在，则调用：
   - `register_device`
   - `bind_owner`
4. 再进入本地：
   - presence registration
   - realtime device state
   - projection register_device
   - route bind

## 首次注册判定

首次注册的判定标准为：

- `projection_service.registered_devices(tenant_id, principal_id)` 中不存在当前 `device_id`

这样做的原因：

- 避免每次 heartbeat / realtime sync / websocket preflight 都重复触发 provider
- 保持 provider 只承担“设备接入”和“owner 绑定”的真实职责
- 后续可以平滑扩展到 `session-gateway`

## 当前固定基线

在 `local-minimal-node` 当前基线中，provider 请求固定为：

- `product_id = local-minimal-device`
- `credential_kind = device_route`

这是本轮为了最小闭环做的运行时冻结，后续如果引入 product registry 再提升为可配置选择。

## 失败语义

- `register_device` 或 `bind_owner` 返回 `ContractError` 时，直接映射为现有 provider 错误语义。
- 如果 `bind_owner` 返回 `false`，运行时视为 `provider_conflict`。
- provider 失败时，不继续执行本地 presence / projection 注册，避免本地状态先于设备接入状态落库。

## 闭环结论

本轮完成后：

- `iot-access-local` 已成为 `local-minimal-node` 的真实 runtime default
- `DeviceAccessProvider` 已进入设备注册主链路
- 但 IoT provider 注入仍不是全局闭环，因为 `session-gateway` 尚未接入
