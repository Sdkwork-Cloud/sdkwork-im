# Continuous Optimization - local-default machine-readable evidence index - 2026-04-08

## 1. 本轮背景

- 上一轮已经补齐 `local-default` 的 post-release 验证样本与 operator 执行记录模板。
- 但 release bundle 里仍缺少一份 machine-readable 归档物来固定：
  - 当前 bundle 属于哪个 wave / profile
  - 当前仍处于模板态还是已完成真实采集
  - 样本、模板和待收集证据槽位的对应关系
- 当前环境没有真实的 `local-default` 发布后执行窗口，因此本轮不伪造一份已填写完成的 operator 记录，而是先把“结构化证据索引”冻结下来。

## 2. 实际落地

### 2.1 新增 machine-readable evidence index

- 新增：`artifacts/releases/wave-d-2026-04-08/local-default-post-release-evidence-index.json`
- 当前 JSON 已固定：
  - `bundleId = wave-d-2026-04-08`
  - `wave = Wave D`
  - `profile = local-default`
  - `artifact = post-release-evidence-index`
  - `state = template_only_pending_collection`
  - 当前边界说明
  - `sampleDoc` / `recordTemplate` 回链
  - 最小 `evidenceSlots`

### 2.2 冻结最小证据槽位，不伪造已收集结果

- 当前 `evidenceSlots` 已覆盖：
  - `deploy_local_ps1_log`
  - `status_local_ps1_output`
  - `local_stack_smoke_ps1_output`
  - `open_chat_test_operator_record`
  - `inspect_runtime_ps1_output`
  - 可选 `screenshot_archive`
- 每个槽位都保持 `pending_collection`，明确表示：
  - 本轮只是固定结构化归档位
  - 当前并未声称这些 `local-default` 证据已经在本机真实采集完成

### 2.3 样本、模板、bundle 归档同步接线

- 更新：`docs/部署/local-default发布后验证样本.md`
- 更新：`docs/部署/local-default发布后验证执行记录模板.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 更新：`artifacts/releases/README.md`
- 当前关系已经明确：
  - 样本文档说明“怎么验证”
  - 执行记录模板承接“怎么留痕”
  - evidence index 固定“有哪些证据槽位、当前处于什么状态”

### 2.4 contract gate 已冻结

- 更新：`services/local-minimal-node/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_release_bundle_contains_machine_readable_evidence_index`
- 当前门禁会冻结：
  - JSON 产物存在且可解析
  - `bundleId / profile / state / boundary / sampleDoc / recordTemplate` 合同存在
  - 样本与模板文档回指 evidence index
  - `Wave D` bundle manifest 已归档该 JSON

## 3. 当前判断

- `local-default` 的 post-release 资产已从“样本 + 人工记录模板”继续推进到“可机读证据索引”。
- 当前实现仍是结构化模板态，不是真实 operator 执行结果归档。
- 本轮的关键决策是不伪造“已执行成功”的示例，而是把真实采集前必须冻结的 evidence schema 先落下来。
- 后续仍可继续深化：
  - 在真实 `local-default` 发布后验证窗口回填 evidence slots
  - 为 evidence index 追加实际文件路径 / checksum / 时间戳
  - 把这份索引接入单一审计/验证导航页

## 4. fresh evidence

- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_local_default_release_bundle_contains_machine_readable_evidence_index -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
