# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v1 -->

## SDKWORK Soul

Read `../sdkwork-specs/SOUL.md` before executing tasks in this root. Follow specs before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this root:

- `../sdkwork-specs/README.md`
- `../sdkwork-specs/SOUL.md`
- `../sdkwork-specs/AGENTS_SPEC.md`
- `../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this repository. If these relative paths do not resolve, stop and report the broken workspace layout.

## Application Identity

Read `sdkwork.app.config.json` before changing application behavior, runtime config, SDK wiring, release metadata, or app-owned capabilities.

## RTC Dependency Boundary

- `sdkwork-im` owns call signaling (`/im/v3/api/calls/*`) and WebSocket call workflow.
- RTC media/provider runtime comes from sibling `../sdkwork-rtc` only (`sdkwork-communication-rtc-service`, `plugins/rtc-*`, `@sdkwork/rtc-sdk`).
- Do not materialize RTC SDK packages under this repository (`sdks/` must not contain `sdkwork-rtc-sdk`).
- Canonical boundary reference: `../sdkwork-rtc/docs/rtc-im-boundary.md`.

## Local Dictionary Structure

- `AGENTS.md`: local agent entrypoint and relative SDKWORK spec index.
- `CLAUDE.md`: Claude Code compatibility shim that points to `AGENTS.md` and must not duplicate rules.
- `GEMINI.md`: Gemini CLI compatibility shim that points to `AGENTS.md` and must not duplicate rules.
- `CODEX.md`: Codex compatibility shim that points to `AGENTS.md` and must not duplicate rules.
- `sdkwork.app.config.json`: application identity and owned capability metadata.
- `.sdkwork/`: reserved local dictionary folder; create only for local skills, plugins, manifests, or AI workspace metadata.
- `specs/`: local application/component contracts and narrowing rules.
- `sdks/`: SDK families, OpenAPI authorities, route manifests, and generated SDK artifacts.
- `apis/`: authored OpenAPI and RPC contract authorities.
- `configs/`: runtime topology and deployment profile templates.
- `sdkwork.workflow.json`: GitHub packaging/release workflow manifest (`GITHUB_WORKFLOW_SPEC.md`).
- `package.json`, `Cargo.toml`: language/build manifests.
- Standard SDKWork directories: `apis/`, `apps/`, `crates/`, `sdks/`, `jobs/`, `tools/`, `plugins/`, `examples/`, `configs/`, `deployments/`, `scripts/`, `docs/`, `tests/`.
- Legacy layout retained during migration: `services/`, `adapters/`, `bin/`, `config/`, `artifacts/`, `external/`.
- Platform frameworks (sibling repos, declared in `Cargo.toml` and `sdkwork.workflow.json`):
  `../sdkwork-web-framework` for HTTP `*-api` runtimes, `../sdkwork-database` for persistence pools.
  `../sdkwork-discovery` is deferred until hosted gRPC RPC service processes ship.

## Spec Resolution Order

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `sdkwork.app.config.json` when present.
3. Read local `specs/README.md` and `specs/component.spec.json` when present.
4. Read local `.sdkwork/README.md`, `.sdkwork/skills/`, and `.sdkwork/plugins/` when relevant.
5. Read `../sdkwork-specs/README.md` and the task-specific root specs.
6. Inspect implementation files only after the relevant dictionary entries are clear.

## Required Specs By Task Type

- Agent/workflow changes: `../sdkwork-specs/SOUL.md`, `../sdkwork-specs/AGENTS_SPEC.md`, `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`.
- Any code change: `../sdkwork-specs/CODE_STYLE_SPEC.md`, `../sdkwork-specs/NAMING_SPEC.md`, plus only the touched language/framework spec.
- Rust code: `../sdkwork-specs/RUST_CODE_SPEC.md` and `../sdkwork-specs/RUST_RPC_SPEC.md` when RPC is touched.
- Java/Spring code: `../sdkwork-specs/JAVA_CODE_SPEC.md` and `../sdkwork-specs/WEB_BACKEND_SPEC.md` when HTTP backend behavior is touched.
- TypeScript/Node code: `../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`.
- Frontend/UI code: `../sdkwork-specs/FRONTEND_CODE_SPEC.md`, `../sdkwork-specs/FRONTEND_SPEC.md`, `../sdkwork-specs/UI_ARCHITECTURE_SPEC.md`, and exactly one detailed UI architecture spec.
- API, SDK, database, runtime, security, and deployment changes must follow the task matrix in `../sdkwork-specs/README.md`.

Language-specific specs are on-demand; do not load Rust, Java, TypeScript, and frontend specs for unrelated tasks.

## Code Style Rules

Read `../sdkwork-specs/CODE_STYLE_SPEC.md` and `../sdkwork-specs/NAMING_SPEC.md` before code changes.

Load language specs only when touched: Rust uses `RUST_CODE_SPEC.md`, Java/Spring uses `JAVA_CODE_SPEC.md`, TypeScript/Node uses `TYPESCRIPT_CODE_SPEC.md`, and frontend/UI uses `FRONTEND_CODE_SPEC.md`.

For Rust, keep `src/lib.rs` limited to module declarations, re-exports, light docs, and wiring; move handlers, services, repositories, DTOs, SQL, provider clients, and tests into focused modules.

For TypeScript or frontend code, prefer strict types, explicit package exports, colocated tests, and existing package/module boundaries.

## Build, Test, and Verification

Run commands from this directory unless a command explicitly targets another path.

- `pnpm install`: install dependencies for this workspace or package.
- `pnpm im:dev`: start the topology v2 browser dev stack (`self-hosted.split-services.development`).
- `pnpm server:dev`: start the Rust server and platform gateway without the PC renderer.
- `pnpm run dev`: alias of `pnpm im:dev` for workspace compatibility.
- `pnpm run check:commercial-readiness`: run repository verification or architecture checks.
- `pnpm run test:database-naming-standard`: run the configured test suite for this scope.
- `pnpm run test:component-spec-consistency`: verify component.spec.json and workspace crate README alignment.
- `pnpm run test:apis-authority-standard`: verify OpenAPI authority mirrors and SDK assembly metadata.
- `pnpm run test:runtime-standard`: run the configured test suite for this scope.
- `pnpm run test:topology-baggage`: scan active paths for retired topology vocabulary.
- `pnpm run test:rtc-signaling-boundary`: verify IM/RTC signaling boundary contracts.
- `pnpm run test:sdkwork-im-pc-dev-command`: run the configured test suite for this scope.
- `pnpm run test:sdkwork-im-pc-i18n`: run the configured test suite for this scope.
- `pnpm run test:sdkwork-im-pc-sidebar-modules`: run the configured test suite for this scope.
- `pnpm run test:workflow-commercial-gates`: run the configured test suite for this scope.
- `pnpm run test:sdkwork-workspace-structure-standard`: verify SDKWork workspace dictionary and manifests.
- `pnpm run test:web-framework-standard`: verify `sdkwork-web-framework` gateway integration.
- `pnpm run test:database-framework-standard`: verify `sdkwork-database` pool integration.
- `pnpm run test:rpc-contract`: verify RPC proto/manifest/SDK contract alignment.
- `pnpm run verify:standards`: run the full SDKWork standards verification bundle for this repository.
- `pnpm run test:runtime-id-standard`: verify runtime Snowflake ID generation standards.
- `pnpm run test:deprecated-service-boundary`: verify deprecated service crates do not mount retired HTTP surfaces.
- `cargo fmt --all --check`: verify Rust formatting across workspace crates.
- `cargo test --workspace`: run workspace Rust tests.
- `cargo clippy --workspace --tests -- -D warnings`: lint Rust tests and crates with warnings denied.

Run the narrowest relevant check first, then broader verification when API contracts, SDK generation, persistence, security, or cross-package boundaries change.

## Agent Execution Rules

Use the convention dictionary instead of broad context loading. Do not hand-edit generated SDK output unless the task is explicitly about generated artifacts and the source contract is verified. Do not replace generated SDK integration with raw HTTP. Keep changes scoped to the owning module, package, crate, or app root. Record the exact verification commands and important outputs before reporting completion.

## Human Review Rules

Request human review before breaking SDKWORK standards, changing public naming, altering security/auth behavior, changing database migrations or production deployment config, deleting data/files, or changing generated SDK ownership. Surface unresolved spec paths, app identity conflicts, component ownership conflicts, and API authority ambiguity instead of guessing.

## Existing Local Guidance

The repository-specific guidance below was preserved from the previous `AGENTS.md`. If it conflicts with the SDKWORK sections above or with `../sdkwork-specs/`, the SDKWORK standards win.

### Project Structure & Module Organization

This repository is a Rust 2024 workspace for Sdkwork IM, with SDK, docs, and release tooling. Core domain and contract crates live in `crates/`, runtime services in `services/`, storage/provider integrations in `adapters/`, and command-line tools in `tools/`. CLI wrappers are under `bin/` (`chat-cli*`, `chat-window*`); deployment templates under `deployments/`, docs under `docs/`, SDK work under `sdks/` (RTC SDK in sibling `../sdkwork-rtc`), and Node governance scripts under `scripts/`. Rust tests are colocated with each crate or service in `tests/`. Avoid editing `vendor/`, `.runtime/`, `target/`, and generated SDK outputs unless explicitly required.

### Build, Test, and Development Commands

- `cargo test -p sdkwork-im-gateway --tests`: run gateway integration tests for the default application ingress.
- `cargo test --workspace`: run all workspace tests; expect this to be slower.
- `cargo fmt --all --check`: verify Rust formatting.
- `cargo clippy --workspace --tests -- -D warnings`: lint with warnings as errors; for large changes, prefer narrower package scopes from `.github/workflows/im-commercial-gates.yml`.
- `node scripts/run-commercial-gates-governance-node-tests.mjs` or `pnpm test:workflow-commercial-gates`: run Node governance tests.
- `pnpm test:topology-baggage` / `pnpm test:rtc-signaling-boundary`: topology v2 and RTC boundary contracts.
- `pnpm im:dev` / `pnpm server:dev`: start the topology v2 development stack.

### Coding Style & Naming Conventions

Use Rust 2024 edition and standard `rustfmt` output. Keep crates/packages kebab-case, modules/functions snake_case, types/traits PascalCase, and constants SCREAMING_SNAKE_CASE. Keep boundaries explicit: shared contracts belong in `crates/`, runtime behavior in `services/`, and provider logic in `adapters/`. Prefer workspace dependencies from `Cargo.toml` over duplicated member versions.

### Testing Guidelines

Add focused Rust integration tests under the owning package's `tests/` directory. Test names should describe behavior, for example `test_duplicate_open_stream_is_idempotent_and_conflicting_retry_is_rejected`. For HTTP or runtime behavior, update the relevant smoke or e2e test and run `cargo test -p <package> --test <file>` first, then broader checks as risk warrants.

### Commit & Pull Request Guidelines

Recent history uses Conventional Commit prefixes such as `feat:`, `fix:`, `test:`, `docs:`, and `chore:`; keep subjects imperative and scoped. Pull requests should describe the affected crate/service, list verification commands, note config or migration impacts, and link the relevant issue or design document in `docs/` or `specs/`. Include screenshots only for UI-facing changes under `apps/` or `docs/sites`.

### Security & Configuration Tips

Do not commit secrets, local runtime state, or generated caches. Keep license-sensitive changes aligned with `COMMERCIAL-LICENSE.md`, and use environment variables such as `SDKWORK_IM_POSTGRES_TEST_DATABASE_URL` for live PostgreSQL integration tests.
