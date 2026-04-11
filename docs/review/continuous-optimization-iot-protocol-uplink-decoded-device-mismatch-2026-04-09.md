# continuous-optimization - IoT protocol uplink decoded-device mismatch - 2026-04-09

## 当前轮次

- step: `08`
- wave: `08-N`
- 目标：把 decode 后设备不一致冻结为 `device_id_mismatch`
- 节点：`local-minimal-node`

## 为什么做这一轮

- `08-M` 已收敛 request/auth 设备不一致。
- 但如果 adapter decode 后返回了不同的 `envelope.device_id`，route 仍会把问题退化成 `device_permission_denied`。
- 这条边界只有在 decode 后才能发现，但仍需要更准确的错误语义。

## TDD 记录

- 先扩展：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test test_iot_protocol_uplink_decoded_device_mismatch_returns_bad_request_after_decode -- --nocapture`
- 初始失败原因：
  - 返回 `403`
  - 而不是 `400 device_id_mismatch`
- 绿灯确认：
  - 返回 `400`
  - 错误码 `device_id_mismatch`
  - adapter `recorded_requests() == 1`

## 实际改动

- 新增 decode 后一致性 helper：
  - `services/local-minimal-node/src/node/access.rs`
- 在 uplink handler 中前置 post-decode mismatch 判断：
  - `services/local-minimal-node/src/node/iot.rs`
- 扩展主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 本轮冻结的设计事实

- helper：
  - `ensure_iot_protocol_uplink_decoded_device_matches_preflight`
- mismatch 语义：
  - `400`
  - `device_id_mismatch`
- 顺序：
  - preflight
  - `decode_uplink`
  - decode 后一致性校验
  - decode 后 access 校验
- 注入 seam 保持：
  - `build_default_app_with_iot_protocol_adapter`

## 边界

- 本轮只修正 decode 后设备不一致的错误语义
- 不宣称完整 payload-level 协议网关已经完成
- 不宣称所有协议冲突都可在 decode 前发现

## 验证结果

- 已通过：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 下一轮动作

- 继续检查 `Step 08` 是否还存在未冻结的真实代码差距；若无，则按循环 Step 指令转入下一 step / wave

## 关键词冻结

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `ensure_iot_protocol_uplink_decoded_device_matches_preflight`
- `device_id_mismatch`
- `build_default_app_with_iot_protocol_adapter`
