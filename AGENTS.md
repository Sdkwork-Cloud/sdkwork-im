# Repository Guidelines

## Project Structure & Module Organization

This repository is a Rust 2024 workspace for Craw Chat, with SDK, docs, and release tooling. Core domain and contract crates live in `crates/`, runtime services in `services/`, storage/provider integrations in `adapters/`, and command-line tools in `tools/`. Local lifecycle scripts are under `bin/`, deployment helpers under `deployments/`, docs under `docs/`, SDK work under `sdks/`, and Node governance scripts under `scripts/`. Rust tests are colocated with each crate or service in `tests/`. Avoid editing `vendor/`, `.runtime/`, `target/`, and generated SDK outputs unless explicitly required.

## Build, Test, and Development Commands

- `cargo test -p local-minimal-node --tests`: run main local integration tests.
- `cargo test --workspace`: run all workspace tests; expect this to be slower.
- `cargo fmt --all --check`: verify Rust formatting.
- `cargo clippy --workspace --tests -- -D warnings`: lint with warnings as errors; for large changes, prefer narrower package scopes from `.github/workflows/im-commercial-gates.yml`.
- `node scripts/run-commercial-gates-governance-node-tests.mjs` or `pnpm test:workflow-commercial-gates`: run Node governance tests.
- `pnpm server:dev`: start the unified local web server script.
- `./bin/start-local.ps1` and `./bin/status-local.ps1`: start and inspect the default local profile.

## Coding Style & Naming Conventions

Use Rust 2024 edition and standard `rustfmt` output. Keep crates/packages kebab-case, modules/functions snake_case, types/traits PascalCase, and constants SCREAMING_SNAKE_CASE. Keep boundaries explicit: shared contracts belong in `crates/`, runtime behavior in `services/`, and provider logic in `adapters/`. Prefer workspace dependencies from `Cargo.toml` over duplicated member versions.

## Testing Guidelines

Add focused Rust integration tests under the owning package's `tests/` directory. Test names should describe behavior, for example `test_duplicate_open_stream_is_idempotent_and_conflicting_retry_is_rejected`. For HTTP or runtime behavior, update the relevant smoke or e2e test and run `cargo test -p <package> --test <file>` first, then broader checks as risk warrants.

## Commit & Pull Request Guidelines

Recent history uses Conventional Commit prefixes such as `feat:`, `fix:`, `test:`, `docs:`, and `chore:`; keep subjects imperative and scoped. Pull requests should describe the affected crate/service, list verification commands, note config or migration impacts, and link the relevant issue or design document in `docs/` or `specs/`. Include screenshots only for UI-facing changes under `apps/` or `docs/sites`.

## Security & Configuration Tips

Do not commit secrets, local runtime state, or generated caches. Keep license-sensitive changes aligned with `COMMERCIAL-LICENSE.md`, and use environment variables such as `CRAW_CHAT_POSTGRES_TEST_DATABASE_URL` for live PostgreSQL integration tests.
