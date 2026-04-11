# Wave D 2026-04-08 Bundle Manifest

## 基本信息

- bundle id: `wave-d-2026-04-08`
- 对应波次：`Wave D`
- 对应闭环：`Step 13`、`Wave D / 93`
- 当前结论：`go / no-go = Go`

## 归档证据

- `docs/review/step-13-release-readiness-2026-04-08.md`
- `docs/review/step-13-go-no-go清单-2026-04-08.md`
- `docs/review/step-13-next-wave-backlog-2026-04-08.md`
- `docs/review/wave-d-93-总验收-2026-04-08.md`
- `docs/review/step-13-架构兑现-2026-04-08.md`
- `docs/review/step-13-架构回写决议-2026-04-08.md`
- `docs/review/continuous-optimization-local-default-machine-readable-evidence-index-2026-04-08.md`
- `docs/review/continuous-optimization-release-bundle-evidence-index-schema-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-release-catalog-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-release-catalog-schema-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-leaf-readmes-release-boundary-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-container-readmes-release-boundary-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-release-version-placeholder-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-release-version-decision-source-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-leaf-readmes-version-decision-source-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-container-readmes-version-decision-source-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-leaf-readmes-version-placeholder-2026-04-08.md`
- `docs/review/continuous-optimization-sdk-container-readmes-version-placeholder-2026-04-08.md`
- `docs/review/continuous-optimization-evidence-slot-metadata-contract-2026-04-08.md`
- `docs/review/continuous-optimization-evidence-artifact-root-contract-2026-04-08.md`
- `docs/review/continuous-optimization-evidence-slot-suggested-path-contract-2026-04-08.md`
- `docs/review/continuous-optimization-evidence-slot-size-bytes-contract-2026-04-08.md`
- `docs/review/continuous-optimization-checksum-manifest-contract-2026-04-08.md`
- `docs/review/continuous-optimization-artifact-file-list-contract-2026-04-08.md`
- `docs/review/continuous-optimization-collection-summary-contract-2026-04-08.md`
- `docs/review/continuous-optimization-collection-summary-slot-consistency-contract-2026-04-08.md`
- `docs/部署/local-default发布后验证样本.md`
- `docs/部署/local-default发布后验证执行记录模板.md`
- `artifacts/releases/wave-d-2026-04-08/evidence/local-default/README.md`
- `artifacts/releases/wave-d-2026-04-08/evidence/local-default/checksum-manifest.txt`
- `artifacts/releases/wave-d-2026-04-08/evidence/local-default/artifact-file-list.txt`
- `artifacts/releases/schemas/post-release-evidence-index.schema.json`
- `artifacts/releases/schemas/sdk-release-catalog.schema.json`
- `artifacts/releases/wave-d-2026-04-08/local-default-post-release-evidence-index.json`
- `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`

## 验证命令

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features --offline -- -D warnings`
- `cargo test --workspace --offline`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/deploy-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/start-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/status-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/status-local.ps1 -ProfileName local-default`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/restore-runtime-local.ps1 -Help`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/open-chat-test.ps1 -Help`

## 升级 / 回滚入口

- 升级入口：
  - `bin/deploy-local.ps1`
  - `bin/start-local.ps1`
  - `bin/status-local.ps1`
- 回滚/恢复入口：
  - `bin/restore-runtime-local.ps1`

## 当前边界

- 当前 bundle 是最小归档物，不等于完整自动发布流水线。
- `local-default` 当前仍复用 `local-minimal` 服务合同；本 bundle 只新增对称 post-release 验证样本，不宣称已拥有独立拓扑。
- SDK 目录当前只冻结 machine-readable release catalog：
  - `sdk-release-catalog.schema.json`
  - `sdk-release-catalog.json`
  - `state = template_only_pending_generation`
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`
- 当前 bundle 已满足：
  - 可审计
  - 可回滚
  - 可追溯
- 后续仍可在同一 bundle 目录下补：
  - 机器生成版本清单
  - checksum
  - release note 导出物
