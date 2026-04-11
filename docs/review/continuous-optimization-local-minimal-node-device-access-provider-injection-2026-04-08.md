# continuous-optimization - local-minimal-node device access provider injection

## 本轮结论

本轮把 `DeviceAccessProvider` 从 adapter baseline 推进到了 `local-minimal-node` 的真实运行时主链路。

## 已交付事实

- `local-minimal-node` 新增：
  - `build_default_app_with_device_access_provider`
  - `build_default_app_with_runtime_dir_and_device_access_provider`
- 默认运行时已真实装配 `im-adapter-iot-access-local::LocalDeviceAccessProvider`
- `/api/v1/devices/register` 已真实调用：
  - `register_device`
  - `bind_owner`
- 当前冻结的首次注册请求常量为：
  - `product_id = local-minimal-device`
  - `credential_kind = session`

## TDD 证据

- 红灯：
  - `cargo test -p local-minimal-node --offline --test device_access_provider_mainline_test -- --nocapture`
  - 初始失败点：`build_default_app_with_runtime_dir_and_device_access_provider` 不存在
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test device_access_provider_mainline_test -- --nocapture`

## 关键设计判断

- provider 调用不能挂在所有 route preflight 上，否则会把 heartbeat / realtime sync / websocket 也变成设备接入调用。
- 当前实现只在 projection 尚未存在设备注册记录时调用 provider，满足“首次注册触发 provider，后续续连复用本地状态”的最小闭环。

## 当前剩余缺口

- `session-gateway` 尚未接入 `DeviceAccessProvider`
- IoT provider external HTTP surface 仍未落地
- `iot-xiaozhi` 真实 adapter 仍依赖 external 源码进一步对齐

## 真实性声明

- 本轮只交付了 `local-minimal-node` 注入闭环
- 未宣称 `session-gateway` 已接入
- 未宣称 `iot-xiaozhi` 已进入运行时
