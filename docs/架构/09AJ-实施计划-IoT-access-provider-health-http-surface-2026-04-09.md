# 09AJ - 实施计划 - IoT access provider health HTTP surface

## 目标

补齐第一条真实 IoT provider external HTTP surface，让 `local-minimal-node` 对外暴露当前 access provider 的 health 状态。

## 输入

- `docs/step/08-AI-Agent-IoT统一扩展层落地.md`
- `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`
- `docs/step/08-F-IoT-DeviceAccessProvider接入session-gateway-2026-04-09.md`
- `services/local-minimal-node/src/node/build.rs`
- `services/local-minimal-node/src/node/device_registration.rs`

## 范围

本轮包含：

- `GET /api/v1/iot/access/provider-health`
- `DeviceAccessProvider::provider_health_snapshot()` 的 HTTP 暴露
- `local-minimal-node` 运行时 health 可见性
- 默认 `iot-access-local` 的 health 可见性
- 测试与文档回写

本轮不包含：

- IoT protocol HTTP API
- 设备注册/禁用等写接口
- `session-gateway` health surface

## 执行步骤

1. 先写失败测试，确认当前路由返回 `404`。
2. 新增 IoT health route handler。
3. 通过现有 `LocalNodeDeviceRegistration` 暴露 `provider_health_snapshot`。
4. 在 `build.rs` 装配新路由。
5. 回写 `step / 架构 / review` 与文档测试。

## 验证

- `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test media_provider_http_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test provider_plugin_docs_test -- --nocapture`

## 退出标准

- `GET /api/v1/iot/access/provider-health` 返回 `200`
- 响应包含当前 access provider 的真实 `ProviderHealthSnapshot`
- 默认 `iot-access-local` 响应可见：
  - `assignedProtocols = mqtt,xiaozhi`
- 文档明确记录“只闭环 access health，不闭环 protocol surface”

## 下一步

转入 IoT protocol external HTTP surface。
