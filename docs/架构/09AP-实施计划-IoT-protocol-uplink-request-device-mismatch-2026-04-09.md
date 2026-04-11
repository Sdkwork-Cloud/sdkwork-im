# 09AP - 实施计划 - IoT protocol uplink request-device mismatch

## 目标

让 `/api/v1/iot/protocol/uplink` 在 request body `deviceId` 与 `auth.device_id` 不一致时，于 `decode_uplink()` 前返回 `device_id_mismatch`。

## 输入

- `docs/prompts/反复执行Step指令.md`
- `docs/step/08-L-IoT-protocol-uplink-actor-preflight-2026-04-09.md`
- `docs/架构/150AO-iot-protocol-uplink-actor-preflight设计-2026-04-09.md`
- `services/local-minimal-node/src/node/iot.rs`
- `services/local-minimal-node/src/node/access.rs`
- `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 范围

本轮包含：

- `/api/v1/iot/protocol/uplink`
- `resolve_requested_device_id`
- `request.device_id`
- `auth.device_id`
- `device_id_mismatch`
- `build_default_app_with_iot_protocol_adapter`
- 测试与文档回写

本轮不包含：

- 完整 payload-level 防火墙
- `iot-xiaozhi` runtime 对接
- 新协议 schema 体系

## 执行步骤

1. 先新增失败测试：
   - device actor 已绑定 `d_sensor`
   - request body 传 `deviceId = d_other`
2. 确认红灯：
   - 当前返回 `403`
   - 而不是 `400 device_id_mismatch`
3. 在 uplink route 中复用：
   - `resolve_requested_device_id(&auth, request.device_id)`
4. 解析成功后再执行：
   - `ensure_iot_protocol_uplink_access`
   - `decode_uplink`
5. 回写 `step / 架构 / review` 与文档测试

## 验证

- `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- `cargo fmt --all --check`

## 退出标准

- request/auth 设备不一致时返回 `400`
- 错误码为 `device_id_mismatch`
- `recorded_requests() == 0`
- 现有 uplink/downlink 主链路不回退

## 关键词冻结

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `resolve_requested_device_id`
- `device_id_mismatch`
- `build_default_app_with_iot_protocol_adapter`
