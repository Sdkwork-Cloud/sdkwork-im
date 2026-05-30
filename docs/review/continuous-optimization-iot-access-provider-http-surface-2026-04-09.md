# continuous-optimization - IoT access provider HTTP surface - 2026-04-09

## 当前轮次

- step: `08`
- wave: `08-G`
- 目标：补齐第一条真实 IoT access provider external HTTP surface

## 为什么做这一轮

- `DeviceAccessProvider` 已经进入 `local-minimal-node` 和 `session-gateway` 运行时主链路
- 但外部 HTTP 观测面仍缺失 IoT access provider health
- 这使得 IoT provider 运行状态还不能和 media / rtc 一样进入统一可见面

## TDD 记录

- 先新增：
  - `services/local-minimal-node/tests/iot_provider_http_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`
- 初始失败原因：
  - `GET /backend/v3/api/iot/access/provider_health` 返回 `404`
- 绿灯后确认：
  - 返回 `200`
  - `pluginId = iot-access-local`
  - `details.providerKind = local`
  - `details.assignedProtocols = mqtt,xiaozhi`

## 实际改动

- 新增 handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 更新 route 装配：
  - `services/local-minimal-node/src/node/build.rs`
- 更新 owner seam：
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/device_registration.rs`
- 新增测试：
  - `services/local-minimal-node/tests/iot_provider_http_test.rs`

## 本轮冻结的设计事实

- 路由：
  - `GET /backend/v3/api/iot/access/provider_health`
- 响应源：
  - `DeviceAccessProvider::provider_health_snapshot()`
- 当前默认 provider：
  - `iot-access-local`
- 本轮只闭环：
  - IoT access provider health
- 本轮不闭环：
  - IoT protocol surface
  - access write API

## 验证结果

- 已通过：
  - `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`
- 本轮需要联动通过：
  - `media_provider_http_test`
  - `iot_provider_docs_test`
  - `provider_plugin_docs_test`

## 文档回写状态

- 已新增：
  - `docs/step/08-G-IoT-access-provider-health-http-surface-2026-04-09.md`
  - `docs/架构/09AJ-实施计划-IoT-access-provider-health-http-surface-2026-04-09.md`
  - `docs/架构/150AJ-iot-access-provider-health-http-surface设计-2026-04-09.md`
  - `docs/review/continuous-optimization-iot-access-provider-http-surface-2026-04-09.md`
- 已回写主文档：
  - `docs/step/08-AI-Agent-IoT统一扩展层落地.md`
  - `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`

## 剩余差距

- IoT protocol external HTTP surface 未完成
- `iot-xiaozhi` 仍未进入真实 runtime adapter 交付
- 更完整的 access 管理写 API 仍未进入本轮范围

## 下一轮动作

优先补 IoT protocol external HTTP surface。
