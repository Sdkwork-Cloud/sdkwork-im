# Continuous Optimization Review: RTC Recording Artifact Object Storage Runtime Closure

## Current Step / Wave

- Step: `06`
- Wave: `06-C`
- Closure status: closed

## Why This Loop

- Object storage was already closed for media uploads and downloads.
- RTC recording artifacts still bypassed the object storage plugin runtime and were returning provider-specific playback URLs directly from RTC adapters.
- That left `tenant override` and `deployment_profile` ineffective on the RTC playback surface.

## What Was Completed

- Extended `RtcRecordingArtifact` with `bucket` and `storage_provider`.
- Reworked built-in RTC adapters so they emit artifact object coordinates instead of final playback URLs.
- Added built-in object storage provider installation to `RtcRuntime`.
- Froze the default object storage deployment profile at `object-storage-volcengine` for RTC runtime boot.
- Normalized `recording_artifact(...)` so playback URLs are signed by `ObjectStorageProvider`.
- Verified that tenant override and deployment profile both affect RTC artifact playback URL generation.
- Mirrored the same behavior through standalone and local-minimal HTTP surfaces.

## Files Changed

- `crates/im-platform-contracts/src/provider.rs`
- `crates/im-platform-contracts/tests/provider_registry_contract_test.rs`
- `adapters/rtc-volcengine/src/lib.rs`
- `adapters/rtc-aliyun/src/lib.rs`
- `adapters/rtc-tencent/src/lib.rs`
- `services/rtc-signaling-service/Cargo.toml`
- `services/rtc-signaling-service/src/lib.rs`
- `services/rtc-signaling-service/tests/rtc_runtime_persistence_test.rs`
- `services/rtc-signaling-service/tests/http_smoke_test.rs`
- `services/local-minimal-node/tests/http_e2e_test.rs`
- `services/local-minimal-node/tests/provider_plugin_docs_test.rs`
- `docs/review/continuous-optimization-rtc-recording-object-storage-runtime-2026-04-08.md`
- `docs/step/06-C-RTC录制对象存储运行时闭环-2026-04-08.md`
- `docs/架构/09B-实施计划-RTC录制对象存储补充-2026-04-08.md`
- `docs/架构/150B-RTC录制对象存储运行时与播放面设计-2026-04-08.md`

## Verification

- `cargo test -p rtc-signaling-service --offline --test rtc_runtime_persistence_test test_runtime_signs_recording_artifact_through_selected_object_storage_provider -- --nocapture`
- `cargo test -p rtc-signaling-service --offline --test http_smoke_test test_get_rtc_recording_artifact_over_http -- --nocapture`
- `cargo test -p local-minimal-node --offline --test http_e2e_test test_local_minimal_profile_gets_rtc_recording_artifact_over_http -- --nocapture`

## Remaining Gap

- Control-plane visibility for effective bindings is still missing for RTC and object storage.
- Playback URL TTL is still a runtime constant and has not yet been promoted into deploy governance.

## Next Loop

1. Expose effective provider binding and deployment profile views for RTC and object storage.
2. Reuse the same plugin runtime closure pattern for `iot-mqtt` and `iot-xiaozhi`.
