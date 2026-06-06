# continuous-optimization - IoT protocol uplink request-device mismatch - 2026-04-09

## 当前轮次

- step: `08`
- wave: `08-M`
- 目标：把 request/auth 设备不一致收敛为 decode 前 `device_id_mismatch`
- 节点：`local-minimal-node`

## 为什么做这一轮

- `08-L` 之后，明显无效的 actor 已经能在 decode 前被拒绝。
- 但 request body `deviceId` 与 `auth.device_id` 不一致时，route 仍先进入较粗的注册/权限路径。
- 这让边界错误被错误表达成 `device_permission_denied`。

## TDD 记录

- 先扩展：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test test_iot_protocol_uplink_request_device_mismatch_rejects_before_adapter_decode -- --nocapture`
- 初始失败原因：
  - 返回 `403`
  - 而不是 `400 device_id_mismatch`
- 绿灯确认：
  - 返回 `400`
  - 错误码 `device_id_mismatch`
  - `recorded_requests() == 0`

## 实际改动

- 更新 uplink handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 扩展主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 本轮冻结的设计事实

- 路由先复用：
  - `resolve_requested_device_id`
- request/auth 设备不一致：
  - `400`
  - `device_id_mismatch`
- 解析成功后才继续：
  - `ensure_iot_protocol_uplink_access`
  - `decode_uplink`
- 注入 seam 保持：
  - `build_default_app_with_iot_protocol_adapter`

## 边界

- 本轮只修正 request/auth device mismatch 的错误语义
- 不宣称完整 payload-level 防火墙完成
- 不宣称所有 decode 后 mismatch 都已提前到 decode 前

## 验证结果

- 已通过：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 下一轮动作

- 继续评估 decode 后 envelope.device_id 与 preflight device_id 的一致性边界是否要单独冻结

## 关键词冻结

- `local-minimal-node`
- `/app/v3/api/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `resolve_requested_device_id`
- `device_id_mismatch`
- `build_default_app_with_iot_protocol_adapter`
