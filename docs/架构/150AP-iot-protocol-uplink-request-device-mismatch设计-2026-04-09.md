# 150AP - IoT protocol uplink request-device mismatch 设计

## 1. 目标

把 request body `deviceId` 与 `auth.device_id` 的一致性判断前移到 `/backend/v3/api/iot/protocol/uplink` 的 decode 前边界。

## 2. 设计约束

- device actor 已有 actor preflight。
- uplink 写入仍只能发生在 bound device actor 上。
- request body 如果显式给出 `request.device_id`，它必须与 `auth.device_id` 一致。
- 该判断应先于：
  - 注册/权限检查
  - `decode_uplink`

## 3. 规则冻结

入口先做：

- `resolve_requested_device_id(&auth, request.device_id)`

冻结语义：

- `request.device_id` 与 `auth.device_id` 一致：
  - 返回解析后的 device id
- `request.device_id` 缺失：
  - 退回 `auth.device_id`
- request/auth 不一致：
  - 返回 `400`
  - 错误码：`device_id_mismatch`

随后再执行：

- `ensure_iot_protocol_uplink_access`
- `decode_uplink`
- decode 后 access 二次校验

## 4. 为什么这是最小真实修正

此前 uplink 路由直接手工拼：

- `request.device_id.clone().or_else(|| auth.device_id.clone())`

这会让 mismatch 情况先掉入注册/权限分支，从而得到较粗糙的：

- `device_permission_denied`

但 request/auth 设备不一致，本质不是权限不足，而是边界输入错误。因此应在更靠前的位置直接报：

- `device_id_mismatch`

## 5. 边界说明

本轮可以声明：

- request/auth 设备不一致已在 decode 前冻结为 bad-request 边界
- `resolve_requested_device_id` 成为 uplink 路由的统一 request/auth 设备解析入口
- `build_default_app_with_iot_protocol_adapter` 注入 seam 保持不变

本轮不能声明：

- 完整 payload-level 防火墙已经完成
- 所有 envelope/device mismatch 都已提前到 decode 前

## 6. TDD 策略

- 新增红测：
  - device actor 已绑定 `d_sensor`
  - request body 传 `d_other`
- 红灯证明：
  - 当前返回 `403`
  - 而不是 `400 device_id_mismatch`
- 绿灯证明：
  - `400`
  - `device_id_mismatch`
  - adapter `recorded_requests() == 0`
  - 其它 uplink/downlink mainline 继续通过

## 7. 关键词冻结

- `local-minimal-node`
- `/backend/v3/api/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `resolve_requested_device_id`
- `request.device_id`
- `auth.device_id`
- `device_id_mismatch`
- `400`
- `后 decode`
- `build_default_app_with_iot_protocol_adapter`
