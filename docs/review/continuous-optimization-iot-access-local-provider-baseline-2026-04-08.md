# Continuous Optimization - iot-access-local provider baseline - 2026-04-08

## 1. 本轮背景

- provider/plugin 架构已经冻结：
  - `DeviceAccessProvider`
  - `IotProtocolAdapter`
  - `iot-access-local`
  - `iot-mqtt`
  - `iot-xiaozhi`
- 但仓库里仍然没有真实 `adapters/iot-access-local`
- 这意味着 IoT 仍只有协议适配层基线，没有设备管理与接入体系的 provider baseline

## 2. 实际落地

### 2.1 红灯先行

- 新增：`services/local-minimal-node/tests/iot_provider_docs_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- 初始失败点：
  - 缺少 `docs/step/08-D-IoT-DeviceAccessProvider本地运行时基线-2026-04-08.md`

### 2.2 新增真实 adapter crate

- 新增：
  - `adapters/iot-access-local/Cargo.toml`
  - `adapters/iot-access-local/src/lib.rs`
  - `adapters/iot-access-local/tests/adapter_contract_test.rs`
  - `adapters/iot-access-local/README.md`
- 当前 `iot-access-local` 已实现：
  - `register_device`
  - `bind_owner`
  - `disable_device`
  - `provider_health_snapshot`

### 2.3 当前闭环边界

- `register_device` 已提供本地 device registry 最小返回形态
- `bind_owner` 与 `disable_device` 已形成最小 owner / ban 闭环
- 默认协议分配固定为 `mqtt / xiaozhi`
- `DeviceAccessProvider` 与 `IotProtocolAdapter` 仍保持边界分离，不重新引入协议耦合

## 3. 当前判断

- IoT provider/plugin 主线现在同时拥有：
  - 第一条真实 `IotProtocolAdapter` baseline：`iot-mqtt`
  - 第一条真实 `DeviceAccessProvider` baseline：`iot-access-local`
- 当前实现仍不代表 `local-minimal-node / session-gateway` 已完成 provider-aware 注入
- 本轮正确决策是先补 `DeviceAccessProvider` baseline，再进入 `iot-xiaozhi` 与 IoT external surface
