# 09AO - 实施计划 - IoT protocol uplink actor preflight

## 目标

让 `local-minimal-node` 在 `/backend/v3/api/iot/protocol/uplink` 中，先完成 actor 级预检，再调用 `IotProtocolAdapter::decode_uplink()`。

## 输入

- `docs/prompts/反复执行Step指令.md`
- `docs/step/08-K-IoT-protocol-uplink-known-device预检鉴权-2026-04-09.md`
- `docs/架构/150AN-iot-protocol-uplink-known-device-preflight设计-2026-04-09.md`
- `services/local-minimal-node/src/node/iot.rs`
- `services/local-minimal-node/src/node/access.rs`
- `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 范围

本轮包含：

- `/backend/v3/api/iot/protocol/uplink`
- `ensure_iot_protocol_uplink_actor_preflight`
- `auth.actor_kind`
- `auth.device_id`
- `device_permission_denied`
- `device_id_missing`
- `build_default_app_with_iot_protocol_adapter`
- 测试与文档回写

本轮不包含：

- `payload.deviceId` 的完整前置鉴权
- 完整协议输入网关
- `iot-xiaozhi` runtime 对接

## 执行步骤

1. 先扩展失败测试，构造：
   - 非 device actor
   - request body 不带 `deviceId`
   - adapter 通过 `payload.deviceId` 推断设备
2. 确认红灯：
   - 请求返回 `403`
   - 但 `decode_uplink()` 仍被调用
3. 在 `access.rs` 增加 `ensure_iot_protocol_uplink_actor_preflight`
4. 在 `iot.rs` 中把 actor preflight 放到 `decode_uplink()` 之前
5. 保留现有：
   - `request.device_id.clone().or_else(|| auth.device_id.clone())`
   - known-device preflight
   - decode 后二次 access 校验
6. 回写 `step / 架构 / review` 与文档测试

## 验证

- `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- `cargo fmt --all --check`

## 退出标准

- 非 device actor 的 uplink 请求在 decode 前返回 `403`
- 缺失 `auth.device_id` 的 device actor 请求在 decode 前返回 `device_id_missing`
- `recorded_requests() == 0`
- `decode_uplink -> device.telemetry` 主链路保持不回退
- 文档明确声明：本轮只前移 actor 级边界，不宣称 payload-level 防火墙完成

## 关键词冻结

- `local-minimal-node`
- `/backend/v3/api/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `ensure_iot_protocol_uplink_actor_preflight`
- `device_permission_denied`
- `device_id_missing`
- `build_default_app_with_iot_protocol_adapter`
