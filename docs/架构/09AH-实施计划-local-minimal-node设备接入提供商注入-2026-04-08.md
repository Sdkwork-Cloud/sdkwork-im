# 09AH - 实施计划 - local-minimal-node 设备接入提供商注入

## 目标

让 `local-minimal-node` 的设备注册主链路真实消费 `DeviceAccessProvider`，并保持默认运行时继续可启动、可测试。

## 实施范围

- `services/local-minimal-node/src/node/build.rs`
- `services/local-minimal-node/src/node/device_registration.rs`
- `services/local-minimal-node/src/node.rs`
- `services/local-minimal-node/Cargo.toml`
- `services/local-minimal-node/tests/device_access_provider_mainline_test.rs`

## 实施步骤

1. 新增 `DeviceAccessProvider` 注入构建入口
   - `build_default_app_with_device_access_provider`
   - `build_default_app_with_runtime_dir_and_device_access_provider`
2. 默认装配 `im-adapter-iot-access-local::LocalDeviceAccessProvider`
3. 扩展 `LocalNodeDeviceRegistration`
   - 保存 `Arc<dyn DeviceAccessProvider>`
   - 只在首次注册时执行 provider 调用
   - 固定当前本地基线请求：
     - `product_id = local-minimal-device`
     - `credential_kind = device_route`
4. 对 `/im/v3/api/devices/register` 写主链路测试
5. 同步 Step / 架构 / Review 文档

## 关键约束

- 不能把 provider 调用放进所有 route preflight，否则 heartbeat / sync / websocket 会重复触发外部设备接入逻辑。
- 不能伪造 `session-gateway` 已接入。
- 不能绕过默认本地 provider，必须让 `iot-access-local` 成为真实 runtime default。

## 成功标准

- `local-minimal-node` 默认启动时已装配 `iot-access-local`
- 主链路测试能证明 `/im/v3/api/devices/register` 调用了：
  - `register_device`
  - `bind_owner`
- 文档明确标注：
  - `local-minimal-node` 已闭环
  - `session-gateway` 未闭环
