# 08-K - IoT protocol uplink known-device 预检鉴权

## 本轮目标

在 `08-I` 已打通 `decode_uplink -> device.telemetry` 的前提下，补齐一个最小但真实的安全/成本边界：

- `local-minimal-node`
- 当 `/app/v3/api/iot/protocol/uplink` 的目标 `deviceId` 已知时
- 先做 access preflight
- 再调用 `IotProtocolAdapter::decode_uplink()`

本轮只闭环：

- `request.device_id` 或 `auth.device_id` 已知时的预检鉴权
- 未授权请求不再先消耗 adapter decode

本轮不闭环：

- `deviceId` 只能从 `payload.deviceId` 推断时的 decode-first 路径
- 完整 IoT 协议防火墙
- `iot-xiaozhi` 真实 runtime adapter

## 发现的问题

- `08-I` 的 uplink route 会先调用 `decode_uplink()`，再检查设备访问权限。
- 这意味着即使请求最终会被拒绝，只要 `deviceId` 已在请求体或 auth 中明确给出，未授权请求仍会先消耗 adapter。
- 对注入式 adapter 而言，这会把不必要的协议解析成本放在授权前。

## 本轮决策

- 对 `local-minimal-node` 的 `/app/v3/api/iot/protocol/uplink` 增加一个最小 preflight 规则：
  - `request.device_id.clone().or_else(|| auth.device_id.clone())`
- 如果上式得到 `deviceId`，则：
  - 先调用 access guard
  - 通过后才调用 `decode_uplink()`
- decode 后仍保留二次校验，防止 `payload.deviceId` 与预期设备不一致。
- 未授权拒绝继续保持现有错误词汇：
  - `device_permission_denied`
- 注入边界继续冻结为：
  - `build_default_app_with_iot_protocol_adapter`
  - `IotProtocolAdapter`
  - `decode_uplink`

## 关键词冻结

- `local-minimal-node`
- `/app/v3/api/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `request.device_id.clone().or_else(|| auth.device_id.clone())`
- `preflight`
- `device_permission_denied`
- `build_default_app_with_iot_protocol_adapter`

## 实际落地

- 扩展主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`
- 更新 uplink handler：
  - `services/local-minimal-node/src/node/iot.rs`

## 验证

- 红灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`
  - 新失败点：未授权 uplink 请求虽然返回 `403`，但 adapter 仍被调用 1 次
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 结果

- `/app/v3/api/iot/protocol/uplink` 在已知 `deviceId` 时已具备：
  - 先鉴权
  - 后 decode
- 注入式 adapter 测试已证明：
  - 未授权请求返回 `403`
  - adapter `decode_uplink()` 不再被提前调用

## 下一轮建议

- 继续收敛 `payload.deviceId` 才能确定目标设备时的最小安全边界。
- 继续避免把这次优化误写成“完整协议防火墙已完成”。
