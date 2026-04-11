# 09AI - 实施计划 - session-gateway 设备接入提供商注入

## 目标

在不新增伪接口的前提下，把 `DeviceAccessProvider` 注入到 `session-gateway` 的真实首设备接入主链路 `POST /api/v1/sessions/resume`。

## 输入

- `docs/step/08-AI-Agent-IoT统一扩展层落地.md`
- `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`
- `docs/step/08-E-IoT-DeviceAccessProvider接入local-minimal-node-2026-04-08.md`
- `services/session-gateway/src/lib.rs`
- `services/session-gateway/src/device_registration.rs`
- `services/session-gateway/src/session_state.rs`

## 范围

本轮包含：

- `session-gateway` provider 注入入口
- `build_app_with_device_access_provider`
- `build_app_with_cluster_and_device_access_provider`
- 默认 `iot-access-local` 装配
- `SessionDeviceRegistration` 消费 `DeviceAccessProvider`
- `SessionSyncState` 首注册判断
- `ContractError -> ApiError` 映射
- 主链路测试与文档回写

本轮不包含：

- 新增设备管理 HTTP API
- `iot-xiaozhi` 真实 adapter
- IoT provider external HTTP surface

## 执行步骤

1. 先写失败测试，覆盖 `POST /api/v1/sessions/resume` 调用注入 provider。
2. 补 builder seam，支持传入 `Arc<dyn DeviceAccessProvider>`。
3. 把默认 provider 固定为 `iot-access-local`。
4. 在 `SessionDeviceRegistration` 中接入：
   - `register_device`
   - `bind_owner`
5. 用 `SessionSyncState::has_registered_device` 做首注册保护，避免 heartbeat / route preflight 重复触发。
6. 补 provider 错误映射。
7. 回写 `step / 架构 / review` 与文档测试。

## 验证

- `cargo test -p session-gateway --offline --test device_access_provider_mainline_test -- --nocapture`
- `cargo test -p session-gateway --offline --test http_smoke_test -- --nocapture`
- `cargo test -p session-gateway --offline --test lib_structure_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`

## 退出标准

- `session-gateway` 真实支持 provider 注入
- `POST /api/v1/sessions/resume` 真实触发 `register_device / bind_owner`
- heartbeat 不重复调用 provider
- 默认运行时仍能直接启动
- 文档与测试同步闭环

## 下一步

`DeviceAccessProvider` 运行时注入闭环完成后，转入 IoT provider external HTTP surface。
