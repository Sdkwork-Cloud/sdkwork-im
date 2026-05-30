# 150AN - IoT protocol uplink known-device preflight 设计

## 1. 目标

把 `local-minimal-node` 上 `/backend/v3/api/iot/protocol/uplink` 的最小授权边界前移：当目标设备已知时，先鉴权，再调用 `decode_uplink()`。

## 2. 设计约束

- 只处理“已知 deviceId”的 preflight 场景。
- 不要求在 `payload.deviceId` 才能确定设备时也做到完全前置鉴权。
- 不改变 `decode_uplink -> device.telemetry` 的统一主链路。

## 3. 规则冻结

本轮冻结已知目标设备的计算规则：

- `request.device_id.clone().or_else(|| auth.device_id.clone())`

如果上式能得到 `deviceId`，则：

- 先鉴权
- 后 decode
- 未授权错误词汇保持：
  - `device_permission_denied`

如果上式仍然得不到 `deviceId`，则保留 decode-first 路径，由 adapter 从 `payload.deviceId` 推断目标设备。

## 4. 为什么要做这个 preflight

`08-I` 的原始实现中，即使请求最终会被拒绝，route 仍会先调用 `decode_uplink()`。

这会带来两个问题：

- 未授权请求先消耗协议解析成本
- 注入式 adapter 无法保证“先授权再消费”

因此本轮最小修正是：

- 对已知 deviceId 的请求前移 access guard
- decode 后仍保留二次校验，防止 payload.deviceId 与预期设备不一致
- 继续冻结注入 seam：
  - `build_default_app_with_iot_protocol_adapter`
  - `IotProtocolAdapter`

## 5. 边界说明

本轮可以声明：

- 已知 deviceId 的 uplink 请求已经具备 preflight
- 未授权请求可在 decode 前被拒绝

本轮不能声明：

- 不是所有 uplink 请求都完全先鉴权
- 不是已经解决 `payload.deviceId` 才可识别的场景
- 不是完整协议防火墙

## 6. 测试策略

采用 TDD：

- 先扩展 `iot_protocol_adapter_mainline_test`
- 构造一个 known-device 的未授权请求
- 先确认路由虽然返回 `403`，但 adapter 仍被调用 1 次
- 再补最小 preflight 逻辑
- 绿灯验证：
  - 返回 `403`
  - `recorded_requests() == 0`
  - 已授权主链路仍保持通过

## 7. 关键词冻结

- `local-minimal-node`
- `/backend/v3/api/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `request.device_id.clone().or_else(|| auth.device_id.clone())`
- `preflight`
- `device_permission_denied`
- `build_default_app_with_iot_protocol_adapter`
