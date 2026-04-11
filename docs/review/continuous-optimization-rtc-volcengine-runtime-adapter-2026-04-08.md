# Continuous Optimization - RTC Volcengine Runtime Adapter - 2026-04-08

## 本轮目标

在 Step 05 的 `UserModuleProvider` 主链闭环后，进入 Step 06 的最小可闭环增量：

- 把 `RtcProviderPort` 从契约层推进到真实 `RtcRuntime`
- 落默认 `rtc-volcengine` provider adapter
- 保持 `rtc-signaling-service` 与 `local-minimal-node` 的现有 RTC 主链稳定

## 已完成

- 新增 `adapters/rtc-volcengine`
  - 提供默认 `VolcengineRtcProvider`
  - 实现 `create_session / close_session / issue_participant_credential / refresh_participant_credential / map_provider_callback / export_recording_artifact / provider_health_snapshot`
- `services/rtc-signaling-service/src/lib.rs`
  - `RtcRuntime` 新增 provider registry 与 runtime provider map
  - 保留 `with_store(...)`，但默认改为装配 `rtc-volcengine`
  - 新增 `with_store_and_provider_registry(...)`
  - `create_session(...)` 现在真实调用 `RtcProviderPort::create_session(...)`
  - 新增 `issue_participant_credential(...)`
  - 新增 `provider_health_snapshot(...)`
  - `reject_session(...)` 与 `end_session(...)` 现在真实调用 `RtcProviderPort::close_session(...)`
- `crates/im-domain-core/src/rtc.rs`
  - `RtcSession` 新增通用 provider 元数据：
    - `provider_plugin_id`
    - `provider_session_id`
    - `access_endpoint`
    - `provider_region`
  - 这些字段保持通用，不暴露厂商私有结构

## TDD 证据

先新增失败测试，再补实现：

- 失败命令
  - `cargo test -p rtc-signaling-service --offline test_runtime_routes_create_credential_and_end_through_selected_rtc_provider -- --nocapture`
  - 失败原因：`RtcRuntime::with_store_and_provider_registry` 不存在
- 通过后验证
  - 同一条命令转绿

## 验证结果

以下命令在本轮代码落地后重新执行并通过：

- `cargo fmt --all`
- `cargo fmt --all --check`
- `cargo test -p im-domain-core --offline test_rtc_session_serializes_signal_binding_fields -- --nocapture`
- `cargo test -p rtc-signaling-service --offline test_runtime_routes_create_credential_and_end_through_selected_rtc_provider -- --nocapture`
- `cargo test -p rtc-signaling-service --offline -- --nocapture`
- `cargo test -p local-minimal-node --offline --test rtc_runtime_persistence_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test provider_plugin_docs_test -- --nocapture`

## 架构判断

本轮后可以确认：

- `RtcProviderPort` 不再只停留在契约测试，已经进入真实 runtime 链路
- 默认 RTC provider 已冻结到 `rtc-volcengine`
- RTC 会话已具备通用 provider 元数据，后续可以继续承接 callback / artifact / credential surface
- `local-minimal-node` 没有因为 provider 接线破坏 RTC 持久化恢复链路

## 仍未完成

- 还没有开放统一的 credential / callback / artifact HTTP surface
- 还没有落 `rtc-aliyun / rtc-tencent` 真实 adapter
- 还没有进入 `object-storage-s3` 与 `iot-mqtt` 的真实 runtime adapter

## 下一轮建议

优先继续 Step 06 / Wave B：

1. 为 `rtc-signaling-service / local-minimal-node` 增加标准化 RTC credential surface，完成 `session + credential + health` 的外部可见闭环。
2. 若保持最小增量策略，也可以直接进入 `object-storage-s3` baseline，继续推进 provider/plugin 三条主线中的下一条。
