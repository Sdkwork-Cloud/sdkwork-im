# 150AO - IoT protocol uplink actor preflight 设计

## 1. 目标

把 `/api/v1/iot/protocol/uplink` 的最外层授权边界继续前移：在还不知道最终 `payload.deviceId` 之前，只要 auth 已经足够证明请求不合法，就先拒绝，再决定是否进入 `decode_uplink()`。

## 2. 设计约束

- `device.telemetry` 写入只允许 bound device actor。
- 因此下列信息足以做 actor-only preflight：
  - `auth.actor_kind`
  - `auth.device_id`
- 不能破坏既有主链路：
  - `decode_uplink -> device.telemetry`
- 不能移除 decode 后 access 二次校验，因为 `payload.deviceId` 仍可能与预期设备不一致。

## 3. 规则冻结

新增入口规则：

- `ensure_iot_protocol_uplink_actor_preflight(auth)`

冻结语义：

- `auth.actor_kind != "device"`：
  - 返回 `403`
  - 错误码：`device_permission_denied`
- `auth.device_id.is_none()`：
  - 返回 `400`
  - 错误码：`device_id_missing`

通过 actor preflight 后，继续执行现有规则：

- `request.device_id.clone().or_else(|| auth.device_id.clone())`
- 如果 deviceId 已知：
  - 先做 known-device preflight
  - 后 decode
- 如果 deviceId 仍未知：
  - 允许 decode-first
  - 由 adapter 从 `payload.deviceId` 推断
- decode 后仍保留 access 校验

## 4. 为什么这是真实边界

`08-K` 之后，已知目标设备的请求已经能做到先鉴权后 decode。

但还有一类请求仍然多消耗一次协议解析：

- auth.actor_kind 不是 `device`
- 或 auth.device_id 根本不存在
- 这种请求即使稍后从 `payload.deviceId` 推断出目标设备，也不可能合法写入 `device.telemetry`

因此本轮最小修正是：

- 先用 `auth.actor_kind` 和 `auth.device_id` 做快速失败
- 不等到 `payload.deviceId` 解析完成后才拒绝

## 5. 边界说明

本轮可以声明：

- actor-only 非法请求可在 decode 前拒绝
- known-device 与 actor-only 两类 preflight 已同时存在
- `build_default_app_with_iot_protocol_adapter` 仍是冻结的注入 seam

本轮不能声明：

- 所有 uplink 都已完全先鉴权
- `payload.deviceId` 场景已全部前置化
- 完整协议防火墙已经完成

## 6. TDD 策略

- 先补红测：
  - 非 device actor
  - request body 无 `deviceId`
  - adapter 可通过 `payload.deviceId` 推断设备
- 红灯证明：
  - 请求虽然返回 `403`
  - 但 `decode_uplink()` 已经先执行一次
- 绿灯证明：
  - `403`
  - `recorded_requests() == 0`
  - 既有 uplink/downlink mainline 继续通过

## 7. 关键词冻结

- `local-minimal-node`
- `/api/v1/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `ensure_iot_protocol_uplink_actor_preflight`
- `auth.actor_kind`
- `auth.device_id`
- `payload.deviceId`
- `device_permission_denied`
- `device_id_missing`
- `build_default_app_with_iot_protocol_adapter`
- `后 decode`
