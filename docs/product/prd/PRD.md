# Sdkwork IM PRD

Status: active
Owner: SDKWork maintainers
Application: chat
Updated: 2026-06-24
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## Document Map

- [PRD-01-baseline-audit.md](PRD-01-baseline-audit.md)
- [PRD-01-productdesignrequirementsscope.md](PRD-01-productdesignrequirementsscope.md)

## 1. Background And Problem

Sdkwork IM is an enterprise-oriented instant messaging platform with PC web/desktop client,
multi-tenant console/admin surfaces, Rust microservice backend, generated SDK contracts, and
SDKWork-standard deployment profiles (`standalone` / `cloud`).

Product detail lives in the linked PRD shards below.

## 8. Commercial Readiness Status

As of 2026-06-25:

### Backend, API, and Admin

- OpenAPI authorities for `/im/v3/api`, `/app/v3/api`, and `/backend/v3/api` are checked in with generated TypeScript and Flutter SDK families.
- PostgreSQL/SQLite migrations live under `database/migrations/` with framework contract tests (`pnpm run test:database-framework-standard`).
- Admin/console surfaces ship through `apps/sdkwork-im-pc` package families (`sdkwork-im-console-*`, `sdkwork-im-admin-*`) with generated backend SDK integration.
- `pnpm check:commercial-readiness` passes locally (PC build/lint, SDK contracts including Flutter parity, Rust gates, Step-11 capacity evidence, Playwright shell + authenticated chat e2e).

### Client Delivery Matrix

| Surface | Root | Status | Notes |
| --- | --- | --- | --- |
| PC web/desktop | `apps/sdkwork-im-pc` | **Production pilot ready** | Playwright shell + authenticated chat e2e (mock IAM/IM in CI) |
| Console/admin | `apps/sdkwork-im-pc` (`sdkwork-im-console-*`, `sdkwork-im-admin-*`) | **Production pilot ready** | i18n migrated; module packages split from monolithic core |
| H5 mobile | `apps/sdkwork-im-h5` | **Production pilot ready** | IAM `platform: "h5"`, inbox + conversation REST, WebSocket live inbox (user scope) + conversation updates via `@sdkwork/im-sdk`, dev port `3010` |
| Flutter mobile | `apps/sdkwork-im-flutter-mobile` | **Production pilot ready** | Inbox + conversation REST, WebSocket CCP live inbox (user scope) + conversation updates via `im_sdk_composed` with shared live hub, Appbase/dev auth |

### Operations and Evidence

- CI `im-commercial-gates.yml` runs `pnpm verify`, `pnpm check:commercial-readiness`, Playwright Chromium install, and split-service tests on `main`.
- Push delivery supports FCM HTTP v1 OAuth (`SDKWORK_IM_FCM_CREDENTIALS_PATH`) with legacy server-key fallback.
- Kubernetes reference manifests cover gateway, realtime, conversation, governance, notification, projection, media, and streaming services with Ingress, PDB, and HPA templates.
- Staging topology profile: `cloud.split-services.staging`.
- Customer operations and data protection guides: `docs/product/compliance/`.
- Observability runbook: `deployments/observability/README.md`.
- Commercial deployment contract: `pnpm run test:commercial-deployment-contract` (included in `pnpm check:commercial-readiness`).
- Step 11 scenario catalog contract: `pnpm run test:step11-scenario-catalog` (validates repo assets and tier evidence states).
- IM H5 architecture standard: `pnpm run test:sdkwork-im-h5-architecture-standard` (included in `pnpm check:commercial-readiness`).
- IM Flutter mobile architecture standard: `pnpm run test:sdkwork-im-flutter-mobile-architecture-standard` (included in `pnpm check:commercial-readiness`).
- IM app SDK Flutter parity: `pnpm run test:im-app-sdk-flutter-parity` (included in `pnpm check:commercial-readiness`).

### Remaining Enterprise Rollout Items

- Staging-backed Playwright runs against real split-service topology (mock-based chat e2e ships in CI today).
- Multi-region DR automation and published SDK artifact registry (git materialization remains the default today).


## 9. Open Questions
