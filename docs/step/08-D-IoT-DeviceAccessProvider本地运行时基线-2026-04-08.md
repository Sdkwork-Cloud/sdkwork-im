# 08-D - IoT DeviceAccessProvider 本地运行时基线

## 本轮目标

把 `iot-access-local` 从 provider registry 中的静态声明推进到第一条真实 `DeviceAccessProvider` adapter baseline，补上 IoT 设备管理与接入体系的本地基线。

## 发现的问题

- `DeviceAccessProvider` 契约和 `iot-access-local` 默认项已经冻结在 `im-platform-contracts` 与 `ProviderRegistry` 中。
- 但仓库里还没有：
  - `adapters/iot-access-local`
  - 真实 `register_device / bind_owner / disable_device / provider_health_snapshot`
- 结果是 IoT 当前只有 `IotProtocolAdapter` 基线，没有设备管理与接入体系的真实 provider baseline。

## 本轮决策

- 先交付最小但真实的 `iot-access-local` adapter crate。
- 本轮只闭环：
  - `adapters/iot-access-local`
  - `DeviceAccessProvider` 的真实最小实现
  - adapter contract test
  - step / 架构 / review 回写
- 暂不伪造：
  - `local-minimal-node / session-gateway` 的 provider-aware runtime 注入
  - IoT provider external HTTP surface
  - `iot-xiaozhi` 真实 adapter

## 实际落地

- 新增：
  - `adapters/iot-access-local/Cargo.toml`
  - `adapters/iot-access-local/src/lib.rs`
  - `adapters/iot-access-local/tests/adapter_contract_test.rs`
  - `adapters/iot-access-local/README.md`
- 当前 `iot-access-local` 已实现统一 `DeviceAccessProvider` contract：
  - `register_device`
  - `bind_owner`
  - `disable_device`
  - `provider_health_snapshot`
- 本地设备接入基线当前冻结为：
  - device registry
  - owner binding
  - disable / ban baseline
  - 默认协议分配 `mqtt / xiaozhi`

## 验证

- 红灯：
  - `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
  - 初始失败点：缺少 `docs/step/08-D-IoT-DeviceAccessProvider本地运行时基线-2026-04-08.md`
- 绿灯：
  - `cargo fmt --all --check`
  - `cargo test -p im-adapter-iot-access-local --offline --test adapter_contract_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
  - `cargo test -p local-minimal-node --offline --test provider_plugin_docs_test -- --nocapture`

## 下一轮建议

- IoT 主线下一条更真实的缺口是：
  - `iot-xiaozhi` 真实 adapter
  - `DeviceAccessProvider` 注入 `local-minimal-node / session-gateway`
  - IoT provider health / access / protocol 的外部 HTTP surface
