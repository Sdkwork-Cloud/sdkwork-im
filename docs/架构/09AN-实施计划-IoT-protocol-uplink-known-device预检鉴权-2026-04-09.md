# 09AN - 实施计划 - IoT protocol uplink known-device 预检鉴权

## 目标

让 `local-minimal-node` 在 `/api/v1/iot/protocol/uplink` 的目标设备已知时，先完成访问控制，再调用 `IotProtocolAdapter::decode_uplink()`。

## 输入

- `docs/prompts/反复执行Step指令.md`
- `docs/step/08-I-IoT-protocol-uplink接入device-telemetry-mainline-2026-04-09.md`
- `docs/架构/150AL-iot-protocol-uplink-device-telemetry-mainline设计-2026-04-09.md`
- `services/local-minimal-node/src/node/iot.rs`
- `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 范围

本轮包含：

- `/api/v1/iot/protocol/uplink`
- `request.device_id.clone().or_else(|| auth.device_id.clone())`
- 已知 deviceId 时的 preflight access check
- `device_permission_denied`
- `build_default_app_with_iot_protocol_adapter`
- `IotProtocolAdapter::decode_uplink()`
- 保留 decode 后二次校验
- 测试与文档回写

本轮不包含：

- `payload.deviceId` 推断场景的完全前置鉴权
- 全量协议输入校验网关
- `iot-xiaozhi` 真实协议适配器对接

## 执行步骤

1. 先扩展失败测试，确认未授权 uplink 请求虽然返回 `403`，但 adapter 仍被调用。
2. 在 `iot.rs` 中提取已知目标设备：
   - `request.device_id.clone().or_else(|| auth.device_id.clone())`
3. 若目标设备已知，则先做 preflight access check。
4. 通过后再调用 `decode_uplink()`，并保留 decode 后二次 access 校验。
5. 回写 `step / 架构 / review` 与文档测试。

## 验证

- `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- `cargo fmt --all --check`

## 退出标准

- 未授权的 known-device uplink 返回 `403`
- 注入式 adapter 的 `recorded_requests()` 长度保持 `0`
- route 仍保持 `decode_uplink -> device.telemetry` 主链路
- 文档明确声明：只收敛“已知 deviceId 先鉴权”，不是完整 payload-level 防火墙

## 关键词冻结

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `request.device_id.clone().or_else(|| auth.device_id.clone())`
- `preflight`
- `device_permission_denied`
- `build_default_app_with_iot_protocol_adapter`
