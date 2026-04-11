# 09AQ - 实施计划 - IoT protocol uplink decoded-device mismatch

## 目标

让 `/api/v1/iot/protocol/uplink` 在 decode 后发现 `envelope.device_id` 与 `preflight_device_id` 不一致时，返回 `400 device_id_mismatch`。

## 输入

- `docs/prompts/反复执行Step指令.md`
- `docs/step/08-M-IoT-protocol-uplink-request-device-mismatch-2026-04-09.md`
- `docs/架构/150AP-iot-protocol-uplink-request-device-mismatch设计-2026-04-09.md`
- `services/local-minimal-node/src/node/iot.rs`
- `services/local-minimal-node/src/node/access.rs`
- `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 范围

本轮包含：

- `/api/v1/iot/protocol/uplink`
- `ensure_iot_protocol_uplink_decoded_device_matches_preflight`
- `preflight_device_id`
- `envelope.device_id`
- `device_id_mismatch`
- `build_default_app_with_iot_protocol_adapter`
- 测试与文档回写

本轮不包含：

- payload-level 完整协议网关
- `iot-xiaozhi` runtime 对接
- 新 schema 系统

## 执行步骤

1. 先新增失败测试：
   - preflight 设备为 `d_sensor`
   - adapter decode 后返回 `envelope.device_id = d_other`
2. 确认红灯：
   - 当前返回 `403`
   - 而不是 `400 device_id_mismatch`
3. 新增 decode 后一致性 helper
4. 在 `decode_uplink()` 之后、access 二次校验之前调用
5. 回写 `step / 架构 / review` 与文档测试

## 验证

- `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- `cargo fmt --all --check`

## 退出标准

- decode 后设备不一致时返回 `400`
- 错误码为 `device_id_mismatch`
- adapter `recorded_requests() == 1`
- telemetry 主链路不回退

## 关键词冻结

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `ensure_iot_protocol_uplink_decoded_device_matches_preflight`
- `device_id_mismatch`
- `build_default_app_with_iot_protocol_adapter`
