# iot-mqtt

通用 `MQTT` 协议插件基线。

## 统一契约

- 实现 `IotProtocolAdapter`
- `protocol_key() = "mqtt"`
- `decode_uplink(...)`
- `encode_downlink(...)`
- `provider_health_snapshot()`

## 运行时边界

- provider 选择由 `ProviderRegistry` 决定。
- 上行统一归一化到 `device.telemetry`。
- 下行统一归一化到 `device.command`。
- `topic / qos / broker endpoint` 只保留在 adapter metadata，不向业务主链泄漏 MQTT 私有协议分支。

## SDKWork Documentation Contract

Domain: communication
Capability: im
Package type: rust-crate
Status: standardizing

### Public API

Public exports are declared in `specs/component.spec.json` under `contracts.publicExports`.

### Required SDK Surface

- None declared in `specs/component.spec.json`.

### Configuration

Configuration keys and runtime entrypoints are declared in `specs/component.spec.json`.

### SaaS/Private/Local Behavior

This module follows the canonical standards linked from `specs/component.spec.json`, including deployment and runtime configuration rules where applicable.

### Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module.

### Extension Points

Extension points are limited to declared public exports, runtime entrypoints, SDK clients, events, and config keys.

### Verification

- `cargo test --manifest-path apps/craw-chat/adapters/iot-mqtt/Cargo.toml`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
