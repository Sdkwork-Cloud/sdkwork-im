# artifacts/releases

本目录用于存放 `craw-chat` 的正式 release bundle 归档物。

当前约定目标不是伪造发布流水线，而是先把“哪些文件构成一个可审计、可回滚、可归档的发布物”冻结成稳定目录结构。

## 当前目录约定

- `artifacts/releases/<bundle-id>/`
  - 一个 bundle 对应一次可归档发布快照
- `<bundle-id>` 当前推荐格式：
  - `<wave-or-track>-<yyyy-mm-dd>`
  - 示例：`wave-d-2026-04-08`

## 每个 bundle 至少包含

- `bundle-manifest.md`
  - 记录本 bundle 对应的 step / wave / 日期 / go / no-go / 验证命令
- 对应的 review 证据引用
  - 例如 `step-13-release-readiness-2026-04-08.md`
  - 例如 `wave-d-93-总验收-2026-04-08.md`
- 升级 / 回滚入口说明
- 当前发布边界与遗留清单

## 可选增强资产

- machine-readable evidence index
  - 例如 `local-default-post-release-evidence-index.json`
  - 用于把 operator 模板、样本文档与待收集证据槽位固定为结构化归档物
- machine-readable SDK release catalog
  - 例如 `artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json`
  - 用于把多语言 SDK 的语言入口、audience 边界与当前生成/发布状态冻结为 bundle 内可审计目录清单
  - 如当前尚未冻结真实版本，则应显式保留：
    - `plannedVersion = null`
    - `versionStatus = version_unassigned_pending_freeze`
    - `versionDecisionSourcePath = null`
- SDK release catalog schema
  - 固定路径：`artifacts/releases/schemas/sdk-release-catalog.schema.json`
  - `sdk-release-catalog.json` 应通过 `$schema` 指向该 contract，避免 bundle 间 SDK 目录字段漂移
- evidence index schema
  - 固定路径：`artifacts/releases/schemas/post-release-evidence-index.schema.json`
  - evidence index 产物应通过 `$schema` 指向该 contract，避免后续 bundle 结构漂移
  - evidence index 顶层还应固定 `artifactRoot`
    - 例如 `artifacts/releases/wave-d-2026-04-08/evidence/local-default`
  - 如需冻结 bundle 级 checksum 归档入口，可继续固定 `checksumManifestPath`
    - 例如 `artifacts/releases/wave-d-2026-04-08/evidence/local-default/checksum-manifest.txt`
  - 如需冻结 bundle 级 artifact 名单入口，可继续固定 `artifactFileListPath`
    - 例如 `artifacts/releases/wave-d-2026-04-08/evidence/local-default/artifact-file-list.txt`
  - 如需机器可读地表达当前采集完成度，可继续固定 `collectionSummary`
    - 例如 `totalSlots / requiredSlots / optionalSlots / collectedSlots / pendingSlots / skippedOptionalSlots`
    - `collectionSummary` 应始终由 `evidenceSlots[*].required` 与 `evidenceSlots[*].status` 推导，避免摘要计数与 slot 明细漂移
  - slot 级 metadata 当前建议至少保留：
    - `artifactPath`
    - `suggestedRelativePath`
    - `collectedAt`
    - `sizeBytes`
    - `checksumSha256`

## 当前状态

- 当前仓库已先落最小 bundle 归档约定与 `wave-d-2026-04-08` 示例。
- 当前 `wave-d-2026-04-08` 还额外示例化了 `sdk-release-catalog.json`，但明确状态仍为 `template_only_pending_generation`，不伪造真实 SDK 发布物。
- 当前 `sdk-release-catalog.json` 已通过 `sdk-release-catalog.schema.json` 冻结结构化 contract，避免后续 bundle 对 SDK 目录各自扩写。
- 当前版本冻结状态也已显式模板化：
  - `plannedVersion = null`
  - `versionStatus = version_unassigned_pending_freeze`
  - `versionDecisionSourcePath = null`
- 这批 release bundle 归档物的目标是：
  - 可审计
  - 可回滚
  - 可追溯
- 后续若形成真实生成产物，可继续在同一目录结构下追加机器产物与版本清单。
