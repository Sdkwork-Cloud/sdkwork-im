# 09AG - 实施计划 - iot-access-local 设备接入运行时基线

## 目标

让 IoT provider/plugin 主线拥有第一条真实 `DeviceAccessProvider` crate，而不是继续只停留在 registry 默认项和架构文档中。

## 最小实施面

1. 先写红测，冻结 `adapters/iot-access-local` 与 step/架构/review 证据
2. 新增 `adapters/iot-access-local`
3. 真实实现 `DeviceAccessProvider`：
   - `register_device`
   - `bind_owner`
   - `disable_device`
   - `provider_health_snapshot`
4. 新增 adapter contract test
5. 回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不伪造 `local-minimal-node` 已完成 provider-aware device access 注入
- 不伪造 IoT provider 外部 HTTP surface 已交付
- 先冻结本地设备管理和接入体系 baseline，再继续推进 `iot-xiaozhi` 与运行时装配

## 放行标准

- workspace manifest 包含 `adapters/iot-access-local`
- `adapters/iot-access-local` 可编译且 contract test 通过
- step / 架构 / review 已显式说明这是 IoT 第一条真实 `DeviceAccessProvider` baseline
- 文档已显式说明 `register_device / bind_owner / disable_device / provider_health_snapshot`
