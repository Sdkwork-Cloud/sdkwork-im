# 08-L - IoT protocol uplink actor preflight

## 本轮目标

在 `08-K` 已把 known-device uplink 的预检鉴权前移之后，再收敛一个更小但真实的边界：

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- 当 `request.deviceId` 为空，目标设备只能后续由 `payload.deviceId` 推断时
- 仍然可以只依据 auth 先拒绝明显不合法的 actor
- 避免未授权请求先进入 `IotProtocolAdapter::decode_uplink()`

本轮只闭环：

- `auth.actor_kind` 不是 `device` 的 uplink 请求
- `auth.device_id` 缺失的 device actor uplink 请求
- `ensure_iot_protocol_uplink_actor_preflight`

本轮不闭环：

- `payload.deviceId` 的完整 payload-level 防火墙
- 所有协议输入的 schema 校验
- `iot-xiaozhi` 真实 runtime adapter

## 发现的问题

- `08-K` 只解决了已知 `deviceId` 的预检路径。
- 如果请求体没有 `deviceId`，但 auth 本身已经能证明当前 actor 不可能执行 telemetry write，route 仍会先调用 `decode_uplink()`。
- 这让明显无效的请求仍然先消耗 adapter 解析成本。

## 本轮决策

- 在 `/api/v1/iot/protocol/uplink` 入口先执行：
  - `ensure_iot_protocol_uplink_actor_preflight`
- 规则冻结为：
  - `auth.actor_kind != "device"` -> `device_permission_denied`
  - `auth.device_id.is_none()` -> `device_id_missing`
- actor 预检通过后，继续沿用已有主链路：
  - `request.device_id.clone().or_else(|| auth.device_id.clone())`
  - known-device preflight
  - `decode_uplink`
  - decode 后 access 二次校验

## 关键词冻结

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `ensure_iot_protocol_uplink_actor_preflight`
- `device_permission_denied`
- `device_id_missing`
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
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test test_iot_protocol_uplink_non_device_actor_without_request_device_id_rejects_before_decode -- --nocapture`
  - 失败点：请求最终返回 `403`，但 adapter `recorded_requests()` 仍为 `1`
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 结果

- `/api/v1/iot/protocol/uplink` 现在具备两层 decode 前收敛：
  - actor preflight
  - known-device preflight
- 未授权的非 device actor 请求不再先进入 `decode_uplink()`
- decode 后校验仍保留，用于覆盖 `payload.deviceId` 与预期设备不一致的场景

## 下一轮建议

- 继续评估只依赖 `payload.deviceId` 才能识别目标设备时，还能否诚实前移更多最小边界。
