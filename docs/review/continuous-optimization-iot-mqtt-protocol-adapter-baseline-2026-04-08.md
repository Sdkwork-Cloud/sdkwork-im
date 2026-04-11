# Continuous Optimization - IoT MQTT protocol adapter baseline - 2026-04-08

## 1. 本轮背景

- provider/plugin 架构已经冻结：
  - `DeviceAccessProvider`
  - `IotProtocolAdapter`
  - `iot-mqtt`
  - `iot-xiaozhi`
- 但仓库里仍然没有真实 `adapters/iot-mqtt`。
- 这意味着 IoT 仍停留在 registry 与文档层，没有第一条真实 protocol adapter baseline。

## 2. 实际落地

### 2.1 红灯先行

- 新增：`services/local-minimal-node/tests/iot_provider_docs_test.rs`
- 红灯命令：
  - `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
- 初始失败点：
  - 缺少 `docs/step/08-B-IoT-MQTT协议插件运行时基线-2026-04-08.md`

### 2.2 新增真实 adapter crate

- 新增：
  - `adapters/iot-mqtt/Cargo.toml`
  - `adapters/iot-mqtt/src/lib.rs`
  - `adapters/iot-mqtt/tests/adapter_contract_test.rs`
  - `adapters/iot-mqtt/README.md`
- 当前 `iot-mqtt` 已实现：
  - `protocol_key`
  - `decode_uplink`
  - `encode_downlink`
  - `provider_health_snapshot`

### 2.3 当前闭环边界

- `decode_uplink` 已把 MQTT uplink 归一化到 `IotProtocolEnvelope`
- `encode_downlink` 已把统一下行语义编码成 MQTT payload
- `topic / qos / brokerEndpoint` 只保留在 adapter metadata
- `device.telemetry / device.command` 仍是业务主链的统一表达，不重新引入 MQTT 分支

## 3. 当前判断

- IoT provider/plugin 主线已经拥有第一条真实 protocol adapter，而不再只有 registry 声明。
- 当前实现仍然不代表完整 IoT runtime、设备注册或 broker 集成已经完成。
- 本轮关键决策是先补齐 `iot-mqtt` baseline，而不是一次性把 `iot-xiaozhi`、`DeviceAccessProvider`、HTTP surface 全部堆进来。

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p im-adapter-iot-mqtt --offline --test adapter_contract_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test iot_provider_docs_test -- --nocapture`
