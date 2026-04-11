# 08-G - IoT access provider health HTTP surface

## 本轮目标

在已经完成 `DeviceAccessProvider` 运行时注入的基础上，补齐第一条真实 IoT provider external HTTP surface：`local-minimal-node` 的 access provider health 查询。

本轮只闭环：

- `local-minimal-node`
- `GET /api/v1/iot/access/provider-health`
- `iot-access-local` health 可见性

本轮不闭环：

- IoT protocol HTTP surface
- IoT access write API
- `session-gateway` 的 IoT provider health surface
- `iot-xiaozhi` 真实 runtime adapter

## 发现的问题

- `local-minimal-node` 已有：
  - `media/provider-health`
  - `rtc/provider-health`
- 但 IoT access provider 虽然已经被 runtime 消费，却没有对外 health 查询入口。
- 这会导致 IoT access provider 的运行状态只能通过代码或内部调试判断，无法进入统一 HTTP 观测面。

## 本轮决策

- 只补 health surface，不顺手扩成设备管理 API。
- 路由固定为：
  - `GET /api/v1/iot/access/provider-health`
- 继续复用统一鉴权入口。
- route handler 只返回当前注入的 `DeviceAccessProvider::provider_health_snapshot()`。
- health 数据仍来自默认 `iot-access-local`，不伪造 protocol surface。

## 实际落地

- 新增测试：
  - `services/local-minimal-node/tests/iot_provider_http_test.rs`
- 新增 route handler：
  - `services/local-minimal-node/src/node/iot.rs`
- 更新路由装配：
  - `services/local-minimal-node/src/node/build.rs`
- 更新 owner seam：
  - `services/local-minimal-node/src/node.rs`
  - `services/local-minimal-node/src/node/device_registration.rs`

## 验证

- 红灯：
  - `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`
  - 初始失败点：`404`
- 绿灯：
  - `cargo test -p local-minimal-node --offline --test iot_provider_http_test -- --nocapture`

## 结果

- `local-minimal-node` 已新增：
  - `GET /api/v1/iot/access/provider-health`
- 当前返回值来自：
  - `DeviceAccessProvider::provider_health_snapshot()`
- 默认 `iot-access-local` 已可通过 HTTP 直接观测：
  - `pluginId = iot-access-local`
  - `details.providerKind = local`
  - `details.assignedProtocols = mqtt,xiaozhi`

## 下一轮建议

- 优先补 IoT protocol external HTTP surface
- 再评估是否需要把 IoT access provider health 扩到其他运行时形态
