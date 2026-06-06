# 150AQ - IoT protocol uplink decoded-device mismatch 设计

## 1. 目标

冻结 `/app/v3/api/iot/protocol/uplink` 在 decode 后的设备一致性边界：`envelope.device_id` 必须与 preflight 已解析的 `preflight_device_id` 一致。

## 2. 设计约束

- request/auth 设备一致性已经由 `08-M` 前移到 decode 前。
- 但 adapter 仍可能在 decode 后返回不同的 `envelope.device_id`。
- 该场景只能在 decode 后发现，因此必须保留一条 post-decode bad-request 边界。
- 该边界应先于 decode 后 access 校验与 telemetry 写入。

## 3. 规则冻结

新增 decode 后 helper：

- `ensure_iot_protocol_uplink_decoded_device_matches_preflight`

冻结语义：

- `envelope.device_id == preflight_device_id`：
  - 继续执行
- `envelope.device_id != preflight_device_id`：
  - 返回 `400`
  - 错误码：`device_id_mismatch`

随后再执行：

- decode 后 `ensure_iot_protocol_uplink_access`
- `device.telemetry` stream mainline

## 4. 为什么这是最小真实修正

如果 adapter 在 decode 后返回了与 preflight 不一致的设备，而 route 只是继续走 access guard，就会把问题模糊成：

- `device_permission_denied`

但这里的根因不是权限不足，而是协议解析结果与已冻结边界不一致。因此更准确的边界是：

- `device_id_mismatch`

## 5. 边界说明

本轮可以声明：

- decode 后 envelope/device mismatch 已冻结为 bad-request
- `preflight_device_id` 与 `envelope.device_id` 的一致性不再依赖权限路径隐式表达
- `build_default_app_with_iot_protocol_adapter` 注入 seam 保持不变

本轮不能声明：

- 完整 payload-level 协议防火墙已完成
- 所有协议语义冲突都被前移到 decode 前

## 6. TDD 策略

- 新增红测：
  - preflight 为 `d_sensor`
  - adapter 返回 `envelope.device_id = d_other`
- 红灯证明：
  - 当前返回 `403`
  - 而不是 `400 device_id_mismatch`
- 绿灯证明：
  - `400`
  - `device_id_mismatch`
  - adapter 只被调用一次
  - 其它 uplink/downlink mainline 继续通过

## 7. 关键词冻结

- `local-minimal-node`
- `/app/v3/api/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `ensure_iot_protocol_uplink_decoded_device_matches_preflight`
- `preflight_device_id`
- `envelope.device_id`
- `device_id_mismatch`
- `400`
- `decode 后`
- `build_default_app_with_iot_protocol_adapter`
