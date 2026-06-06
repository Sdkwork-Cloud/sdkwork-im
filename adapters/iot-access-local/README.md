# iot-access-local

通用 `iot-access-local` 设备接入插件基线。

## 统一契约

- 实现 `DeviceAccessProvider`
- `register_device(...)`
- `bind_owner(...)`
- `disable_device(...)`
- `provider_health_snapshot()`

## 运行时边界

- provider 选择由 `ProviderRegistry` 决定。
- 当前聚焦本地 device registry、owner 绑定与设备禁用基线。
- 默认协议分配为 `mqtt` 与 `xiaozhi`。
- 设备管理和接入体系停留在 `DeviceAccessProvider`，不向业务主链泄漏私有实现细节。

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

- `cargo test --manifest-path apps/craw-chat/adapters/iot-access-local/Cargo.toml`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.
