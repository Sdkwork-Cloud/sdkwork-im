# im-adapters-postgres-journal

Domain: communication
Capability: im
Package type: rust-crate
Status: standardizing

Postgres journal adapter for domain event persistence. Consumes `sdkwork-database-config` for unified database configuration per `DATABASE_SPEC.md`.

## Public API

- Postgres-backed journal storage for `im-domain-events`.

## Configuration

Database connection uses `SDKWORK_IM_DATABASE_*` environment variables through `sdkwork-database-config`.

## Verification

- `cargo test -p im-adapters-postgres-journal`
