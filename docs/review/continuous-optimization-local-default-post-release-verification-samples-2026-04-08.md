# Continuous Optimization - local-default post-release verification samples - 2026-04-08

## 1. 本轮背景

- `Step 13` 与 `Wave D / 93` 已收口，仓库已经具备：
  - `local-default` 的 profile 名称
  - `deploy-local` 的 profile 入口
  - `status-local` / runtime ops 的 profile-aware 合同
- 但 backlog 仍明确指出：`local-default` 还缺少与 `local-minimal` 对称的关键发布后验证样本，导致该 profile 更像“有入口、少验收口径”的兼容名。

## 2. 实际落地

### 2.1 新增 local-default 发布后验证样本文档

- 新增：`docs/部署/local-default发布后验证样本.md`
- 当前已固定：
  - PowerShell post-release 样本
  - Bash post-release 样本
  - deploy / status / smoke / chat / runtime ops 的最小对称核对路径
  - 证据采集建议
  - 当前边界说明

### 2.2 明确当前边界，不伪造独立拓扑

- 文档明确保留：
  - 当前 `local-default` 仍复用 `local-minimal` 的 compose 服务合同与 smoke 链路
  - 本轮只是补“发布后怎么验证”的样本，不宣称 `local-default` 已拥有独立部署拓扑

### 2.3 公开入口与归档物同步

- 更新：`docs/部署/README.md`
- 更新：`README.md`
- 更新：`artifacts/releases/wave-d-2026-04-08/bundle-manifest.md`
- 当前这份样本已经：
  - 进入部署文档导航
  - 进入仓库根 README 文档入口
  - 进入 `Wave D` release bundle 归档证据

### 2.4 contract gate 已冻结

- 更新：`services/local-minimal-node/tests/deployment_profile_test.rs`
- 新增：
  - `test_local_default_post_release_verification_samples_are_documented_and_archived`
- 当前门禁会冻结：
  - 文档存在
  - 样本中包含 PowerShell / Bash 对称命令
  - README / 部署 README 回链存在
  - `Wave D` bundle manifest 已引用该文档

## 3. 当前判断

- `local-default` 现在不再只在模板、profile 名和帮助面层对齐，也具备了明确的 post-release 验证样本。
- 当前实现的是“最小可信样本”，不是自动化发布后验收流水线。
- 后续仍可继续深化：
  - 把样本变成真实 operator 执行记录模板
  - 在 release bundle 中追加实际执行输出或截图索引
  - 在可用 Bash runtime 的节点补对称 shell 证据

## 4. fresh evidence

- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_local_default_post_release_verification_samples_are_documented_and_archived -- --nocapture`
