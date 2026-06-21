# Deprecated

Canonical database lifecycle assets live in the application-root `database/` directory.

The SQL files in this directory are **read-only archives** consolidated into
`database/ddl/baseline/postgres/0001_im_legacy_baseline.sql`. Runtime code, tests, and bootstrap
commands must not load schema from here.

Do not add new schema files here. Migrate remaining changes into:

- `database/contract/schema.yaml`
- `database/migrations/{engine}/`
- `database/ddl/baseline/{engine}/`

See `DATABASE_FRAMEWORK_SPEC.md` and `database/README.md`.
