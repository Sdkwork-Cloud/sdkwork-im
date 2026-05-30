# Local-Minimal Ops Provider Bindings HTTP Surface Design

## Problem

`local-minimal-node` already had runtime provider binding data in `OpsRuntime`, but it still lacked the dedicated ops routes that the architecture had already frozen.

## Scope

- In scope:
  - `GET /backend/v3/api/ops/provider_bindings`
  - `GET /backend/v3/api/ops/provider_bindings/drift`
  - auth and refresh parity with the other ops routes
- Out of scope:
  - new provider snapshot assembly
  - tenant override policy writes
  - new drift rules

## Design

- Use `OpsRuntime` as the only runtime source.
- Add two local-minimal handlers:
  - `get_ops_provider_bindings`
  - `get_ops_provider_binding_drift`
- Each handler must:
  - resolve auth context
  - require `ops.read`
  - call `refresh_node_operational_view(...)`
  - return the corresponding `OpsRuntime` view
- Register both routes beside the existing ops endpoints.

## Why This Shape

- It matches standalone `ops-service`.
- It keeps `diagnostics`, `provider-bindings`, and `provider-bindings/drift` on one snapshot source.
- It avoids a second provider-binding assembly path inside local-minimal.

## Acceptance

- `local-minimal-node` no longer returns `404` for the two routes.
- The standalone routes return the same snapshot family already visible through `diagnostics`.
- Existing local-minimal tests remain green.
