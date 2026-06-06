# 09AK - 实施计划 - IoT protocol provider health HTTP surface

## 目标

让 `local-minimal-node` 暴露当前 `IotProtocolAdapter` 的统一健康视图，形成第一条真实 `IoT protocol external HTTP surface`。

## 输入

- `docs/step/08-AI-Agent-IoT统一扩展层落地.md`
- `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`
- `docs/step/08-G-IoT-access-provider-health-http-surface-2026-04-09.md`
- `adapters/iot-mqtt/src/lib.rs`
- `services/local-minimal-node/src/node/build.rs`

## 范围

本轮包含：

- `GET /app/v3/api/iot/protocol/provider_health`
- `IotProtocolAdapter::provider_health_snapshot()` 的 HTTP 暴露
- `build_default_app_with_iot_protocol_adapter`
- 默认 `iot-mqtt` health 可见性
- 测试与文档回写

本轮不包含：

- `decode_uplink / encode_downlink` HTTP API
- 设备注册、禁用、命令下发等写接口
- `iot-xiaozhi` 真实适配器对接

## 执行步骤

1. 先写失败测试，确认当前 route 返回 `404`。
2. 为 `local-minimal-node` 增加 `IotProtocolAdapter` 注入端口。
3. 让 `AppState` 保存当前注入 adapter，并暴露 health owner seam。
4. 在 `iot.rs` 增加 protocol health handler，在 `build.rs` 装配 route。
5. 回写 `step / 架构 / review` 与文档测试。

## 验证

- `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test provider_plugin_docs_test -- --nocapture`

## 退出标准

- `GET /app/v3/api/iot/protocol/provider_health` 返回 `200`
- 响应包含当前协议插件的真实 `ProviderHealthSnapshot`
- 默认 `iot-mqtt` 响应可见：
  - `providerKind = mqtt`
  - `protocolKey = mqtt`
- 文档明确声明：本轮只闭环 `protocol health`，不是 `decode_uplink / encode_downlink` API
