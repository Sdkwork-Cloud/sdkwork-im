# 08-M - IoT protocol uplink request-device mismatch

## 本轮目标

继续收敛 `/app/v3/api/iot/protocol/uplink` 的 decode 前边界：

- `local-minimal-node`
- 当 device actor 已绑定 `auth.device_id`
- 但 request body 显式提交了不同的 `request.device_id`
- 应在 `decode_uplink` 前直接返回 `device_id_mismatch`

本轮只闭环：

- request body 与 auth 绑定设备不一致的 uplink 请求
- `resolve_requested_device_id`
- `device_id_mismatch`

本轮不闭环：

- 完整 payload-level 协议防火墙
- 所有协议 schema 校验
- `iot-xiaozhi` runtime adapter

## 发现的问题

- `08-L` 之后，非 device actor 和缺失 `auth.device_id` 的请求已经会在 decode 前失败。
- 但 uplink 路由仍手工使用：
  - `request.device_id.clone().or_else(|| auth.device_id.clone())`
- 这会让 request body 与 auth.device_id 不一致时，先落到注册/权限分支，错误退化为：
  - `device_permission_denied`
- 更准确的边界应该是：
  - `device_id_mismatch`

## 本轮决策

- uplink 路由复用现有 helper：
  - `resolve_requested_device_id`
- 先统一解析 request/device 与 auth/device 的关系
- 只有解析通过后，才执行：
  - `ensure_iot_protocol_uplink_access`
  - `decode_uplink`

## 关键词冻结

- `local-minimal-node`
- `/app/v3/api/iot/protocol/uplink`
- `IotProtocolAdapter`
- `decode_uplink`
- `resolve_requested_device_id`
- `request.device_id`
- `auth.device_id`
- `device_id_mismatch`
- `build_default_app_with_iot_protocol_adapter`

## 实际落地

- 更新 uplink handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 扩展主链路测试：
  - `services/local-minimal-node/tests/iot_protocol_adapter_mainline_test.rs`

## 验证

- 红灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test test_iot_protocol_uplink_request_device_mismatch_rejects_before_adapter_decode -- --nocapture`
  - 初始失败点：
    - 返回 `403`
    - 而不是 `400 device_id_mismatch`
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test iot_protocol_adapter_mainline_test -- --nocapture`

## 结果

- request body `deviceId` 与 auth 绑定设备不一致时：
  - 先返回 `400`
  - 错误码：`device_id_mismatch`
  - adapter 不再被提前调用
- uplink 主链路仍保持：
  - 先 preflight
  - 后 `decode_uplink`
  - decode 后保留二次 access 校验

## 下一轮建议

- 继续评估 decode 后 envelope/device 与已解析 `device_id` 的一致性边界是否还需要单独冻结。
