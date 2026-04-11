# 08-E - IoT DeviceAccessProvider 接入 local-minimal-node

## 本轮目标

把已经落地的 `iot-access-local` baseline 从“可编译的 adapter crate”推进到“`local-minimal-node` 主链路真实消费的 provider”。

本轮只闭环：

- `local-minimal-node`
- `DeviceAccessProvider`
- `/api/v1/devices/register` 主链路
- 默认 `iot-access-local` 装配

本轮不闭环：

- `session-gateway`
- `iot-xiaozhi` 真实 adapter
- IoT provider external HTTP surface

## 发现的问题

- `im-platform-contracts` 已经定义了 `DeviceAccessProvider`。
- `adapters/iot-access-local` 已经提供了真实 baseline。
- 但 `local-minimal-node` 仍然只做本地 route / presence / projection 注册，没有消费 `Arc<dyn DeviceAccessProvider>`。
- 这意味着设备接入 provider 仍停留在 registry / adapter 层，没有进入真实运行时主链路。

## 本轮决策

- 在 `build.rs` 新增 `DeviceAccessProvider` 注入入口，模式对齐已有 `UserModuleProvider` 注入方式。
- 默认 provider 直接使用 `im-adapter-iot-access-local::LocalDeviceAccessProvider`。
- `LocalNodeDeviceRegistration` 负责消费 provider。
- provider 调用只发生在“首次设备注册”时：
  - `register_device`
  - `bind_owner`
- 如果设备已经存在于 projection 注册视图中，后续 route preflight、heartbeat、realtime 等请求不重复调用 provider。
- 当前固定本地基线参数：
  - `product_id = local-minimal-device`
  - `credential_kind = session`

## 实际落地

- 新增测试：
  - `services/local-minimal-node/tests/device_access_provider_mainline_test.rs`
- 更新装配：
  - `services/local-minimal-node/src/node/build.rs`
  - `services/local-minimal-node/src/node.rs`
- 更新注册链路：
  - `services/local-minimal-node/src/node/device_registration.rs`
- 更新依赖：
  - `services/local-minimal-node/Cargo.toml`

## 验证

- 红灯：
  - `cargo test -p local-minimal-node --offline --test device_access_provider_mainline_test -- --nocapture`
  - 初始失败点：缺少 `build_default_app_with_runtime_dir_and_device_access_provider`
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test device_access_provider_mainline_test -- --nocapture`

## 结果

- `local-minimal-node` 已支持：
  - `build_default_app_with_device_access_provider`
  - `build_default_app_with_runtime_dir_and_device_access_provider`
- 默认运行时已真实装配 `iot-access-local`
- `/api/v1/devices/register` 已真实调用：
  - `DeviceAccessProvider::register_device`
  - `DeviceAccessProvider::bind_owner`
- `session-gateway` 仍未接入，不能宣称 IoT provider 注入已全局闭环

## 下一轮建议

- 优先补 `session-gateway` 的 `DeviceAccessProvider` 注入
- 然后补 IoT provider external HTTP surface
- `iot-xiaozhi` 继续等待 external 源码真实对齐后再做运行时 adapter
