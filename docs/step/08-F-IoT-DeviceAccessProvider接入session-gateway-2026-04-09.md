# 08-F - IoT DeviceAccessProvider 接入 session-gateway

## 本轮目标

把已经落地的 `iot-access-local` 从 “adapter baseline” 推进到 `session-gateway` 主链路真实消费的 provider。

本轮只闭环：

- `session-gateway`
- `DeviceAccessProvider`
- `POST /im/v3/api/device/sessions/resume`
- route preflight 的首注册保护
- 默认 `iot-access-local` 装配

本轮不闭环：

- `iot-xiaozhi` 真实运行时 adapter
- IoT provider external HTTP surface
- `external/xiaozhi-esp32` 真实 submodule 拉取

## 发现的问题

- `session-gateway` 原先没有 `DeviceAccessProvider` 注入入口。
- `SessionDeviceRegistration` 只负责 presence / realtime / route bind，没有设备接入 provider 消费点。
- 真实首个设备接入入口不是伪造的 `/im/v3/api/devices/register`，而是 `POST /im/v3/api/device/sessions/resume`。
- heartbeat / realtime route preflight 会复用注册链路；如果不做首注册保护，会重复触发 `register_device / bind_owner`。

## 本轮决策

- 新增 provider 注入入口：
  - `build_app_with_device_access_provider`
  - `build_app_with_cluster_and_device_access_provider`
- 默认运行时继续装配 `iot-access-local`。
- `SessionDeviceRegistration` 成为 `session-gateway` 中 `DeviceAccessProvider` 的唯一消费点。
- 首注册判断不依赖 projection，而依赖 `DeviceSyncState::has_registered_device`。
- 只有首次注册时调用：
  - `register_device`
  - `bind_owner`
- provider 错误统一映射：
  - `UnsupportedCapability -> 501 / provider_capability_unsupported`
  - `Conflict -> 409 / provider_conflict`
  - `Unavailable -> 503 / provider_unavailable`
- 当前固定请求常量：
  - `product_id = session-gateway-device`
  - `credential_kind = device_route`

## 实际落地

- 新增测试：
  - `services/session-gateway/tests/device_access_provider_mainline_test.rs`
- 更新装配：
  - `services/session-gateway/src/lib.rs`
  - `services/session-gateway/Cargo.toml`
- 更新注册链路：
  - `services/session-gateway/src/device_registration.rs`
  - `services/session-gateway/src/session_state.rs`

## 验证

- 红灯：
  - `cargo test -p session-gateway --offline --test device_access_provider_mainline_test -- --nocapture`
  - 初始失败点：缺少 `build_app_with_device_access_provider`
- 绿灯：
  - `cargo test -p session-gateway --offline --test device_access_provider_mainline_test -- --nocapture`

## 结果

- `session-gateway` 已支持：
  - `build_app_with_device_access_provider`
  - `build_app_with_cluster_and_device_access_provider`
- 默认运行时已真实装配 `iot-access-local`
- `POST /im/v3/api/device/sessions/resume` 已真实调用：
  - `DeviceAccessProvider::register_device`
  - `DeviceAccessProvider::bind_owner`
- `POST /im/v3/api/presence/heartbeat` 不会重复触发 provider 注册/绑定
- `local-minimal-node + session-gateway` 的 `DeviceAccessProvider` 运行时注入已形成闭环
- 但 IoT provider external HTTP surface 仍未交付，不能把 IoT provider 整体能力说成全部完成

## 下一轮建议

- 优先补 IoT provider external HTTP surface
- 继续保持 `iot-xiaozhi` 仅在真实 external 源码对齐后再进入运行时交付
