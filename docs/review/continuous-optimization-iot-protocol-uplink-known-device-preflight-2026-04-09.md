# continuous-optimization - IoT protocol uplink known-device preflight - 2026-04-09

## 当前轮次

- step: `08`
- wave: `08-K`
- 目标：让已知 deviceId 的 uplink 请求先鉴权、后 decode
- 节点：`local-minimal-node`

## 为什么做这一轮

- `08-I` 已打通 `decode_uplink -> device.telemetry`
- 但 route 仍会先调用 adapter，再拒绝已知 deviceId 的未授权请求
- 这是一个真实的授权边界和成本边界缺口

## TDD 记录

- 先扩展：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
- 初始失败原因：
  - 未授权请求返回 `403`
  - 但注入式 adapter `recorded_requests()` 仍为 `1`
- 绿灯确认：
  - 未授权请求仍返回 `403`
  - `recorded_requests() == 0`
  - 其他 uplink / downlink 主链路测试保持通过

## 实际改动

- 更新 uplink handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 扩展主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 本轮冻结的设计事实

- 规则：
  - `request.device_id.clone().or_else(|| auth.device_id.clone())`
- 已知 deviceId：
  - 先 preflight
  - 后 decode
- 未知 deviceId：
  - 仍允许 decode-first，交由 adapter 从 payload.deviceId 推断
- 错误词汇：
  - `device_permission_denied`
- 注入 seam：
  - `build_default_app_with_iot_protocol_adapter`

## 边界

- 本轮只优化 known-device uplink preflight
- 不是完整 payload-level 鉴权网关
- 不是 `payload.deviceId` 推断路径已全部前置化
- 不是 `iot-xiaozhi` runtime 完成交付

## 验证结果

- 已通过：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 下一轮动作

- 优先评估 `payload.deviceId` 才能识别目标设备时的最小安全边界

## 关键词冻结

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `request.device_id.clone().or_else(|| auth.device_id.clone())`
- `preflight`
- `device_permission_denied`
- `build_default_app_with_iot_protocol_adapter`
