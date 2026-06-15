# Sdkwork IM Component Specs

This directory is the local standards index for `sdkwork-im`.

Root SDKWork standards remain authoritative. Local component specs can narrow or document this component, but they must not contradict [the root standards](../sdkwork-specs/README.md).

## Component

| Field | Value |
| --- | --- |
| Name | `sdkwork-im` |
| Type | `app` |
| Root | `sdkwork-im` |
| Domain | `communication` |
| Capability | `chat` |
| Languages | `javascript, rust` |
| Status | `ACTIVE` |

## Contract Manifest

- [component.spec.json](./component.spec.json) is the machine-readable component contract.
- Shared foundation API composition targets `sdkwork-api-gateway` through the existing
  `SDKWORK_IM_SERVER_API_BASE_URL` server common SDK root and
  `VITE_SDKWORK_IM_APP_API_BASE_URL` browser app-api root. `services/web-gateway` and
  `crates/sdkwork-im-gateway-config` keep product-owned IM routing only; foundation API routing is
  owned by the shared gateway boundary.
- Local PC development starts the sibling `sdkwork-api-gateway` Cargo service as the shared
  foundation gateway. Product-local server env defaults route Drive and Notary app-api traffic to
  that gateway root; dependency-specific upstream env keys are split-deployment overrides only.
- `crates/sdkwork-im-gateway-config` defaults Appbase, Drive, and Notary service upstreams to the
  shared gateway root. Direct module URLs remain explicit split-deployment overrides.
- Consumers should integrate through public exports, runtime entrypoints, SDK clients, or adapters declared in the manifest.
- Generated SDK language outputs are represented at their SDK family root instead of duplicating local specs in generated folders.

## Canonical Specs

| Spec | Applies Because |
| --- | --- |
| [APP_MANIFEST_SPEC.md](../sdkwork-specs/APP_MANIFEST_SPEC.md) | sdkwork.app.config.json application registration rules. |
| [APPLICATION_SPEC.md](../sdkwork-specs/APPLICATION_SPEC.md) | Application shell and module composition. |
| [COMPONENT_SPEC.md](../sdkwork-specs/COMPONENT_SPEC.md) | Local component specs directory and manifest rules. |
| [CONFIG_SPEC.md](../sdkwork-specs/CONFIG_SPEC.md) | Runtime configuration, environment, SDK bootstrap, and feature flag rules. |
| [DATABASE_SPEC.md](../sdkwork-specs/DATABASE_SPEC.md) | Database table naming, table profiles, schema registry, and prefix governance. |
| [DEPENDENCY_MANAGEMENT_SPEC.md](../sdkwork-specs/DEPENDENCY_MANAGEMENT_SPEC.md) | Native workspace dependency declarations, sibling SDKWork source paths, and Git-backed release dependency refs. |
| [DEPLOYMENT_SPEC.md](../sdkwork-specs/DEPLOYMENT_SPEC.md) | SaaS/private/local runtime parity and deployment rules. |
| [DOCUMENTATION_SPEC.md](../sdkwork-specs/DOCUMENTATION_SPEC.md) | Module README, examples, ADR, changelog, and runbook rules. |
| [DOMAIN_SPEC.md](../sdkwork-specs/DOMAIN_SPEC.md) | Canonical domain ownership and naming. |
| [FRONTEND_SPEC.md](../sdkwork-specs/FRONTEND_SPEC.md) | UI, service, SDK, accessibility, and frontend runtime rules. |
| [GOVERNANCE_SPEC.md](../sdkwork-specs/GOVERNANCE_SPEC.md) | Standard ownership, exception, compatibility, and migration rules. |
| [I18N_SPEC.md](../sdkwork-specs/I18N_SPEC.md) | User-facing language, locale, message catalog, and fallback rules. |
| [MODULE_SPEC.md](../sdkwork-specs/MODULE_SPEC.md) | Reusable package contract and dependency direction. |
| [README.md](../sdkwork-specs/README.md) | SDKWork root standards entrypoint. |
| [SDK_SPEC.md](../sdkwork-specs/SDK_SPEC.md) | SDK generation and SDK integration rules. |
| [TEST_SPEC.md](../sdkwork-specs/TEST_SPEC.md) | Contract, frontend, SDK, security, parity, and documentation verification rules. |

## Public Exports

- Public exports are not declared in the package manifest.

## SDK Clients

- No generated SDK client class is declared at this component boundary.

## Local Extension Specs

- [im-app-api-sdk-integration.spec.md](./im-app-api-sdk-integration.spec.md) defines Sdkwork IM's IM API, IM app API, IM backend API, product SDK ownership, IAM login integration, shared database, local source-link development, and git-backed release dependency rules.
- [database-prefix-registry.json](./database-prefix-registry.json) registers `im` as the controlled prefix for instant-messaging tables in the `im` app.
- [database-table-registry.json](./database-table-registry.json) lists the checked-in IM table contracts, table profiles, write owners, and migration source.
- [database-table-naming-standard.md](../docs/部署/database-table-naming-standard.md) documents the local naming policy: IM tables use `im_`; non-IM tables keep their own business prefix or approved legacy name.

## PC Client Packages

The PC client app lives under `apps/sdkwork-im-pc` and is composed of capability
packages following the SDKWork PC architecture segment. Canonical package naming:

- Console surface: `sdkwork-im-console-*` (normalized PC target `sdkwork-im-pc-console-*`).
- Admin surface: `sdkwork-im-admin-*` (normalized PC target `sdkwork-im-pc-admin-*`).
- PC-native capabilities: `sdkwork-im-pc-*`.

Historical `sdkwork-clawchat-*` package names were retired by the
`sdkwork-im → sdkwork-im` rebrand and must not be reintroduced.

## Verification

- `cargo test --workspace`
- `node scripts/dev/sdkwork-im-database-naming-standard.test.mjs`
- `node scripts/sdkwork-workspace-structure-standard.test.mjs`
