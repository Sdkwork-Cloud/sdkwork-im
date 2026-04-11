# continuous-optimization - IoT protocol uplink actor preflight - 2026-04-09

## 当前轮次

- step: `08`
- wave: `08-L`
- 目标：让明显不合法的 uplink actor 在 `decode_uplink()` 前被拒绝
- 节点：`local-minimal-node`

## 为什么做这一轮

- `08-K` 已收敛 known-device preflight。
- 但 request body 不带 `deviceId` 时，非 device actor 仍会先进入 `IotProtocolAdapter::decode_uplink()`。
- 这是真实的授权和成本边界缺口。

## TDD 记录

- 先扩展：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test test_iot_protocol_uplink_non_device_actor_without_request_device_id_rejects_before_decode -- --nocapture`
- 初始失败原因：
  - 请求返回 `403`
  - 但注入式 adapter `recorded_requests()` 仍为 `1`
- 绿灯确认：
  - 非 device actor 请求仍返回 `403`
  - `recorded_requests() == 0`
  - 既有 uplink / downlink mainline 测试继续通过

## 实际改动

- 新增 actor preflight：
  - `services/local-minimal-node/src/node/access.rs`
- 在 uplink handler 前移调用：
  - `services/local-minimal-node/src/node/iot.rs`
- 扩展主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 本轮冻结的设计事实

- 新规则：
  - `ensure_iot_protocol_uplink_actor_preflight`
- 失败路径：
  - `auth.actor_kind != "device"` -> `device_permission_denied`
  - `auth.device_id.is_none()` -> `device_id_missing`
- 主链路保留：
  - `request.device_id.clone().or_else(|| auth.device_id.clone())`
  - known-device preflight
  - `decode_uplink`
  - decode 后二次 access 校验
- 注入 seam 继续冻结为：
  - `build_default_app_with_iot_protocol_adapter`

## 边界

- 本轮只收敛 actor-only preflight
- 不宣称 `payload.deviceId` 路径已经全部前置鉴权
- 不宣称完整协议防火墙已经完成
- 不宣称 `iot-xiaozhi` runtime 已交付

## 验证结果

- 已通过：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 下一轮动作

- 继续评估 `payload.deviceId` 才能识别目标设备时，还能否再前移最小诚实边界

## 关键词冻结

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `ensure_iot_protocol_uplink_actor_preflight`
- `device_permission_denied`
- `device_id_missing`
- `build_default_app_with_iot_protocol_adapter`
