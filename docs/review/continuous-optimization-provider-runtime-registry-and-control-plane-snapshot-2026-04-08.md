# 持续优化：provider runtime 契约与 control-plane snapshot - 2026-04-08

## 1. 当前 step / 波次

- 当前仓库状态：`Wave D / Step 13` 已闭环，处于持续优化模式。
- 本轮定位：持续优化第十八轮增量。
- 本轮主题：把 provider/plugin 从纯文档标准推进到真实运行时代码契约，并补齐控制面的 provider registry 只读快照。

## 2. 本轮为什么做

上一轮已经冻结了 `rtc / object-storage / user-module / iot-access / iot-protocol` 的 provider/plugin 文档标准，但真实代码仍停留在：

- 有架构标准，没有统一 runtime contract
- 有 provider 矩阵，没有控制面可读的 provider registry snapshot
- 有默认策略，没有 tenant override 优先级的代码表达

这会导致后续直接接 `rtc-volcengine`、`object-storage-s3`、`iot-mqtt` 时，仍然需要先返工底座。

## 3. 本轮实际完成

- 在 `crates/im-platform-contracts` 新增 provider runtime contract：
  - `ProviderDomain`
  - `ProviderPluginDescriptor`
  - `ProviderHealthSnapshot`
  - `EffectiveProviderBinding`
  - `ProviderRegistrySnapshot`
  - `ProviderRegistry`
  - `StaticProviderRegistry`
  - `RtcProviderPort`
  - `ObjectStorageProvider`
  - `UserModuleProvider`
  - `DeviceAccessProvider`
  - `IotProtocolAdapter`
- `StaticProviderRegistry::platform_default()` 已冻结默认运行时快照：
  - RTC 默认 `rtc-volcengine`
  - 用户模块默认 `user-module-local`
  - 设备接入默认 `iot-access-local`
  - IoT 协议默认 `iot-mqtt`
  - 对象存储保留多云矩阵，但默认选择保持 `deployment_required`
- 在 `services/control-plane-api` 新增：
  - `GET /api/v1/control/provider-registry`
  - 权限模型沿用 `control.read / control.write`
  - 返回 `ProviderRegistrySnapshot`
- 新增契约测试：
  - `crates/im-platform-contracts/tests/provider_registry_contract_test.rs`
  - `services/control-plane-api/tests/provider_registry_test.rs`

## 4. 改动文件

- `crates/im-platform-contracts/src/provider.rs`
- `crates/im-platform-contracts/src/lib.rs`
- `crates/im-platform-contracts/tests/provider_registry_contract_test.rs`
- `services/control-plane-api/Cargo.toml`
- `services/control-plane-api/src/lib.rs`
- `services/control-plane-api/tests/provider_registry_test.rs`
- `docs/架构/09-实施计划.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`
- `docs/review/continuous-optimization-provider-runtime-registry-and-control-plane-snapshot-2026-04-08.md`

## 5. docs/review 与 docs/架构 是否已回写

- `docs/review`：已回写，本文件已归档。
- `docs/架构`：已回写到 `09-实施计划` 与 `142-控制面与配置治理设计`。

## 6. 验证

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test -- --nocapture`
- `cargo test -p control-plane-api --offline --test provider_registry_test -- --nocapture`

## 7. 当前还差什么

- `RtcProviderPort` 还没有真正接入 `rtc-signaling-service / local-minimal-node`
- `ObjectStorageProvider` 还没有替换现有 `ObjectStore / BlobStore` 装配路径
- `UserModuleProvider` 还没有进入 conversation / member / sender 主链
- `DeviceAccessProvider / IotProtocolAdapter` 还没有接管现有设备注册与协议上下行主路径
- provider registry 还没有持久化真源，也没有 tenant override 写接口

## 8. 下一轮做什么

1. 先做 `user-module-local / user-module-external` 的最小 provider 实现，并接入 `sender / member` 主链。
2. 再把 `RtcProviderPort` 接入 `rtc-signaling-service` 或 `local-minimal-node`，落 `rtc-volcengine` 默认 provider。
3. 再推进 `object-storage-s3` 和 `iot-mqtt` 的最小实现与 conformance 测试。
