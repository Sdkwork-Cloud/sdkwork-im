# Repository Guidelines

<!-- SDKWORK-AGENTS-GENERATED: v1 -->

## SDKWORK Soul

Read `../../../sdkwork-specs/SOUL.md` before executing tasks in this application root. Follow specs
before memory, dictionary before context, stop on ambiguity, and evidence before completion.

## SDKWORK Standards

Canonical SDKWORK specs path from this application root:

- `../../../sdkwork-specs/README.md`
- `../../../sdkwork-specs/SOUL.md`
- `../../../sdkwork-specs/AGENTS_SPEC.md`
- `../../../sdkwork-specs/CODE_STYLE_SPEC.md`
- `../../../sdkwork-specs/NAMING_SPEC.md`

Do not copy root standard text into this application root. If these relative paths do not resolve,
stop and report the broken workspace layout.

## Application Identity

Read `sdkwork.app.config.json` before changing PC application behavior, runtime config, SDK wiring,
release metadata, packaging, or app-owned capabilities.

## Local Dictionary Structure

- `AGENTS.md`: local application agent entrypoint and relative SDKWORK spec index.
- `CLAUDE.md`: Claude Code compatibility shim that points to `AGENTS.md` and must not duplicate
  rules.
- `GEMINI.md`: Gemini CLI compatibility shim that points to `AGENTS.md` and must not duplicate
  rules.
- `CODEX.md`: Codex compatibility shim that points to `AGENTS.md` and must not duplicate rules.
- `sdkwork.app.config.json`: PC application identity and release metadata.
- `.sdkwork/`: source-controlled application dictionary for local skills, plugins, manifests, and
  AI workspace metadata.
- `specs/`: local PC application/component contracts and narrowing rules.
- `packages/`: PC React package family for app, console, admin, shared runtime, and desktop host
  modules.
- `sdks/`: application-root SDK workspaces when present.
- `src/`: thin PC application bootstrap, providers, route assembly, and shell entry.
- `scripts/`: app-local build, validation, generation, migration, and development utilities.

## Spec Resolution Order

1. Read this `AGENTS.md` and any nearer component-level `AGENTS.md`.
2. Read `sdkwork.app.config.json`.
3. Read local `specs/README.md` and `specs/component.spec.json` when present.
4. Read local `.sdkwork/README.md`, `.sdkwork/skills/`, and `.sdkwork/plugins/` when relevant.
5. Read `../../../sdkwork-specs/README.md` and task-specific root specs.
6. Inspect implementation files only after the relevant dictionary entries are clear.

## Required Specs By Task Type

- Agent/workflow changes: `../../../sdkwork-specs/SOUL.md`,
  `../../../sdkwork-specs/AGENTS_SPEC.md`, and
  `../../../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`.
- Any code change: `../../../sdkwork-specs/CODE_STYLE_SPEC.md`,
  `../../../sdkwork-specs/NAMING_SPEC.md`, plus only the touched language/framework spec.
- TypeScript/Node code: `../../../sdkwork-specs/TYPESCRIPT_CODE_SPEC.md`.
- Frontend/UI code: `../../../sdkwork-specs/FRONTEND_CODE_SPEC.md`,
  `../../../sdkwork-specs/FRONTEND_SPEC.md`,
  `../../../sdkwork-specs/UI_ARCHITECTURE_SPEC.md`, and
  `../../../sdkwork-specs/APP_PC_REACT_UI_SPEC.md`.
- PC application architecture: `../../../sdkwork-specs/APPLICATION_SPEC.md`,
  `../../../sdkwork-specs/APP_CLIENT_ARCHITECTURE_ALIGNMENT_SPEC.md`, and
  `../../../sdkwork-specs/APP_PC_ARCHITECTURE_SPEC.md`.
- SDK wiring, runtime config, release metadata, and packaging changes must follow the task matrix in
  `../../../sdkwork-specs/README.md`.

Language-specific specs are on-demand; do not load unrelated specs for unrelated tasks.

## Code Style Rules

Read `../../../sdkwork-specs/CODE_STYLE_SPEC.md` and
`../../../sdkwork-specs/NAMING_SPEC.md` before code changes.

For TypeScript or frontend code, prefer strict types, explicit package exports, colocated tests, and
existing package/module boundaries. Root `src/` must stay thin; business pages, services, i18n,
state, and route contributions belong in packages.

Canonical PC client package naming follows the SDKWork PC architecture segment.
Console surface uses `sdkwork-im-console-*` (normalized PC target
`sdkwork-im-pc-console-*`); admin surface uses `sdkwork-im-admin-*` (normalized
PC target `sdkwork-im-pc-admin-*`); PC-native capabilities use `sdkwork-im-pc-*`.
Historical `sdkwork-clawchat-console-*` and `sdkwork-clawchat-admin-*` package
names were retired by the `sdkwork-im â†?sdkwork-im` rebrand and must not be
reintroduced.

## Build, Test, and Verification

Run commands from this application root unless a command explicitly targets the repository root.

- `pnpm install`: install PC application workspace dependencies.
- `pnpm dev`: start the PC browser development server.
- `pnpm dev:tauri`: start the PC desktop/Tauri development path.
- `pnpm build`: build the browser renderer and bundled server.
- `pnpm lint`: run TypeScript checking through the local wrapper.
- `pnpm test:notary-app-sdk-integration`: run notary app SDK integration contract checks.
- `pnpm test:qr-scan-standard`: run QR scan contract checks.

From the repository root, run `pnpm run test:sdkwork-workspace-structure-standard` after changing
application-root dictionary, package taxonomy, or workspace metadata.

## Agent Execution Rules

Use the convention dictionary instead of broad context loading. Do not hand-edit generated SDK
output unless the task is explicitly about generated artifacts and the source contract is verified.
Do not replace generated SDK integration with raw HTTP. Keep changes scoped to the owning package,
surface, or app root. Record exact verification commands and important outputs before reporting
completion.

## Human Review Rules

Request human review before breaking SDKWORK standards, changing public naming, altering
security/auth behavior, changing generated SDK ownership, changing production release metadata, or
deleting tracked runtime/cache files such as the current `.sdkwork/dart/pub-cache` migration
artifact.
