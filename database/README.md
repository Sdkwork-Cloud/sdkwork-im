# IM Database Module

Canonical lifecycle assets for `sdkwork-im` per `DATABASE_FRAMEWORK_SPEC.md`.

- moduleId: `im`
- serviceCode: `IM`
- tablePrefix: `im_`

## Commands

```bash
pnpm run db:validate
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```

## Migration status

Legacy SQL was consolidated into `ddl/baseline/postgres/0001_*_legacy_baseline.sql` for bootstrap review.
Author contract-first tables in `contract/schema.yaml`, then split baseline into versioned `migrations/` pairs.

Imported legacy sources:
- `deployments/database/postgres/migrations/001_im_core_schema.sql`
- `deployments/database/postgres/migrations/010_im_tenant_organization_isolation.sql`
- `deployments/database/postgres/migrations/011_im_projections_rtc_streams.sql`
- `deployments/database/postgres/migrations/012_im_social_org_interactions.sql`
- `deployments/database/postgres/migrations/014_im_search_cjk.sql`

Runtime services MUST create pools through `sdkwork-database-sqlx` and register `DefaultDatabaseModule` at bootstrap.

## Runtime integration

- Bootstrap crate: `crates/sdkwork-im-database-host`
- Pool facade: `crates/sdkwork-im-database-pool`
- Entrypoints: `bootstrap_im_database_from_env()` / `bootstrap_im_database(pool)`
- Wired from: `services/sdkwork-im-standalone-gateway`
- Dev migrate: `pnpm db:postgres:migrate` delegates to `sdkwork-database-cli bootstrap`
- Contract materialization: `pnpm run db:materialize:contract`
- IAM schema is owned by `sdkwork-appbase/database/` and bootstrapped by embedded IAM routes
