# 08-N - IoT protocol uplink decoded-device mismatch

## 本轮目标

补齐 `/api/v1/iot/protocol/uplink` 在 decode 后的最后一条最小一致性边界：

- `local-minimal-node`
- preflight 已得到 `preflight_device_id`
- adapter `decode_uplink()` 返回了 `envelope.device_id`
- 若两者不一致，应直接返回 `device_id_mismatch`

本轮只闭环：

- `envelope.device_id` 与 `preflight_device_id` 不一致
- `ensure_iot_protocol_uplink_decoded_device_matches_preflight`
- `device_id_mismatch`

本轮不闭环：

- 完整 payload-level 协议网关
- 所有 schema 校验
- `iot-xiaozhi` runtime adapter

## 发现的问题

- `08-M` 已把 request/auth 设备不一致前移为 `device_id_mismatch`。
- 但如果 adapter 忽略 preflight 设备，decode 后返回了不同的 `envelope.device_id`，当前错误仍会退化成：
  - `device_permission_denied`
- 这不是最准确的边界表达。

## 本轮决策

- 在 `decode_uplink()` 之后、access 二次校验之前，新增：
  - `ensure_iot_protocol_uplink_decoded_device_matches_preflight`
- 如果 `envelope.device_id != preflight_device_id`：
  - 返回 `400`
  - 错误码：`device_id_mismatch`
- 只有一致时，才继续进入 decode 后 access 校验与 telemetry 写入主链路

## 关键词冻结

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `ensure_iot_protocol_uplink_decoded_device_matches_preflight`
- `preflight_device_id`
- `envelope.device_id`
- `device_id_mismatch`
- `build_default_app_with_iot_protocol_adapter`

## 实际落地

- 更新 access guard：
  - `services/local-minimal-node/src/node/access.rs`
- 更新 uplink handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 扩展主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 验证

- 红灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test test_iot_protocol_uplink_decoded_device_mismatch_returns_bad_request_after_decode -- --nocapture`
  - 初始失败点：
    - 返回 `403`
    - 而不是 `400 device_id_mismatch`
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 结果

- decode 后 envelope/device 与 preflight/device 不一致时：
  - 返回 `400`
  - 错误码：`device_id_mismatch`
- adapter 仍只会被调用一次，因为该边界必须在 decode 后才能发现
- 成功路径仍保持：
  - preflight
  - `decode_uplink`
  - decode 后校验
  - `device.telemetry`

## 下一轮建议

- 继续检查 Step 08 是否已无新的真实代码差距，若已闭环，则按提示转入下一 step 或下一波总复核入口。
