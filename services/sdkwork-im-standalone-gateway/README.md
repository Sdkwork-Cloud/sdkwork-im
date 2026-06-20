# sdkwork-im-standalone-gateway

Domain: communication
Capability: im
Package type: rust-service
Status: active

Standalone IM gateway binary for local and packaged deployments. Composes `sdkwork-web-framework` ingress, IM route registry, and product runtime static assets without the full split-service topology.

## Public API

- Binary: `sdkwork-im-standalone-gateway`
- Config: gateway YAML/TOML resolved through `sdkwork-im-gateway-config` and `sdkwork-api-config`.

## Configuration

Reads gateway bind URLs, upstream service endpoints, and static site directories from the resolved standalone gateway config file.

## Verification

- `cargo build -p sdkwork-im-standalone-gateway`
- `pnpm gateway:build:standalone`
