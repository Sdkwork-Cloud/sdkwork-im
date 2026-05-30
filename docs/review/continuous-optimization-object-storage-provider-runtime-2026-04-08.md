# Continuous Optimization Review: Object Storage Provider Runtime Closure

## Summary

- Completed the first real `ObjectStorageProvider` runtime loop.
- Added a reusable `object-storage-s3` adapter crate for:
  - `object-storage-aliyun`
  - `object-storage-tencent`
  - `object-storage-volcengine`
  - `object-storage-aws`
  - `object-storage-google`
  - `object-storage-microsoft`
- Closed the `media-service` seam so uploads and signed download URLs now go through provider selection instead of stringly typed `"local" / "s3"` branches.

## What Changed

- `crates/im-platform-contracts`
  - `StaticProviderRegistry` now supports `deployment_profile` selection in addition to tenant override and global default.
- `adapters/object-storage-s3`
  - Added a real S3-compatible provider adapter crate.
  - Added adapter contract tests for Volcengine and Google gateway mode.
- `services/media-service`
  - Added built-in object storage provider map.
  - Added deployment-profile default selection to `object-storage-volcengine`.
  - `complete_upload(...)` now canonicalizes storage provider and resource URL through `ObjectStorageProvider`.
  - Added `GET /im/v3/api/media/{media_asset_id}/download-url`.
  - Added `GET /backend/v3/api/media/provider_health`.
- `services/local-minimal-node`
  - Mirrored media provider health and signed download URL HTTP surface.

## Verification

- `cargo test -p im-platform-contracts --offline --test provider_registry_contract_test test_provider_registry_supports_deployment_profile_selection_between_tenant_override_and_global_default -- --nocapture`
- `cargo test -p im-adapter-object-storage-s3 --offline --test adapter_contract_test -- --nocapture`
- `cargo test -p media-service --offline --test provider_integration_test -- --nocapture`
- `cargo test -p media-service --offline --test media_asset_test -- --nocapture`
- `cargo test -p media-service --offline --test media_event_test -- --nocapture`
- `cargo test -p media-service --offline --test public_auth_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test media_provider_http_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test http_e2e_test -- --nocapture`
- `cargo test -p local-minimal-node --offline --test public_auth_e2e_test -- --nocapture`

## Remaining Gaps

- RTC recording artifact metadata is still emitted directly by RTC providers and has not yet been normalized onto the new object storage runtime.
- IoT still needs the same closure pattern:
  - real adapter crate
  - runtime provider selection
  - provider health / protocol surface
- User module plugin selection is already modeled, but deployment-profile wiring and control-plane governance can still be tightened.

## Next Suggested Loop

1. Rebind RTC recording artifact export onto `ObjectStorageProvider`.
2. Add a control-plane view for media/object-storage deployment profile and effective provider binding.
3. Reuse the same plugin runtime pattern for `iot-mqtt` and `iot-xiaozhi`.
