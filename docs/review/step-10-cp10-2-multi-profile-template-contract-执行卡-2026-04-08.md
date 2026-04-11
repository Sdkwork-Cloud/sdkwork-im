# Step 10 / CP10-2 multi-profile template contract 执行卡 - 2026-04-08

## 当前上下文
- 当前波次：`Wave D`
- 当前 step：`Step 10`
- 当前子任务：`CP10-2`
- 前置状态：
  - `Wave C / 93` 已完成
  - `Step 10 / CP10-1` 已通过，统一命令面已冻结
  - 但 `CP10-2` 在本轮之前仍缺少三类闭环：
    - 多环境 profile 的真实边界没有形成稳定文档合同
    - `deploy-local` 在 PowerShell / CMD / Bash 上尚未统一支持 profile 选择
    - 快启文档与根 README 还没有把 `local-default` 的入口语义写实到当前脚本合同

## 本轮为什么做这个子任务
- `docs/step/10-部署脚本与多环境发布治理.md` 对 `CP10-2` 的要求是：
  - 多环境 profile 与配置模板要清晰
  - profile 不能只有名字，没有真实入口和职责边界
- `docs/step/95-架构能力闭环验收标准.md` 对 `Step 10` 的要求是：
  - 不能只存在脚本文件，必须有真实可回归的交付入口和文档语义
- 因此本轮最优决策不是假装所有未来 profile 都已经可执行，而是先冻结当前仓库里真实存在的：
  - `local-minimal`
  - `local-default`
  - profile template 入口
  - `deploy-local` 的跨平台选择合同

## 本轮实际完成

### 1. 多环境 profile 与模板边界已冻结
- 新增 `docs/部署/多环境Profile与配置模板.md`
  - 明确区分：
    - 已落地并可引用的 `local-minimal`
    - 已命名、已建 compose/template 入口但仍复用 `local-minimal` 服务合同的 `local-default`
    - 仅处于规划边界的 `private-saas-single-cell`、`cloud-shared-cell`、`cloud-dedicated-cell`
- 新增：
  - `deployments/templates/local-minimal.env.example`
  - `deployments/templates/local-default.env.example`
- 模板当前冻结的公共合同为：
  - `CRAW_CHAT_BIND_ADDR`
  - `CRAW_CHAT_RUNTIME_DIR`
  - `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`

### 2. `deploy-local` 已变成真实的 profile-aware 交付入口
- `bin/deploy-local.ps1`
  - 新增 `-ProfileName <local-minimal|local-default>`
  - 明确把 profile 选择转发给 `bootstrap-local.ps1`
- `deployments/scripts/bootstrap-local.ps1`
  - 新增 `-ProfileName`
  - 从 profile 名称动态派生 compose 文件
  - 错误信息、compose 诊断、smoke 结果均带 profile 语义
- `bin/deploy-local.sh`
  - 新增 `--profile <local-minimal|local-default>`
  - 从 profile 动态派生 compose 文件
  - 统一 profile 校验与诊断输出
- `bin/_cmd-forward-powershell.cmd`
  - 已把 `/profile`、`/profilename`、`--profile`、`--profile-name`、`--profileName`
    收口到 `-ProfileName`

### 3. 文档与回归门禁已追平当前真实脚本合同
- `services/local-minimal-node/tests/deployment_profile_test.rs`
  - 已新增：
    - `test_deployment_profiles_and_templates_document_local_minimal_and_local_default_contracts`
    - `test_deploy_local_scripts_expose_profile_selection_contract`
    - `test_deploy_local_ps1_forwards_profile_name_to_bootstrap_script`
    - `test_deploy_local_cmd_normalizes_profile_name_switch`
  - 在 fresh verification 中继续发现旧资产测试仍硬编码 `local-minimal.yml`
  - 已把旧断言修正为“profile 驱动 compose 选择”的真实契约
  - 已补充快启文档 contract 断言，确保 `deploy-local` 的 profile selector 不会再次从文档中漂移
- `docs/部署/README.md`
  - 已公开多环境 profile/template 文档入口
- `docs/部署/快速启动脚本.md`
  - 已明确：
    - `deploy-local` 支持 `local-minimal` / `local-default`
    - PowerShell / CMD 使用 `-ProfileName <local-minimal|local-default>`
    - Bash 使用 `--profile <local-minimal|local-default>`
- `README.md`
  - 已把根文档的 Docker 入口说明升级为统一 `bin/deploy-local.*` 的 profile-aware 用法

## TDD 证据

### Red
- `cargo test -p local-minimal-node --offline test_deployment_profiles_and_templates_document_local_minimal_and_local_default_contracts`
  - 初始失败，证明 profile/template 文档与模板资产尚未形成合同
- `cargo test -p local-minimal-node --offline test_deploy_local_scripts_expose_profile_selection_contract`
  - 初始失败，证明 `deploy-local` 尚未公开 profile selector
- `cargo test -p local-minimal-node --offline test_deploy_local_ps1_forwards_profile_name_to_bootstrap_script`
  - 初始失败，证明 PowerShell 入口尚未把 profile 正确转发到 bootstrap
- `cargo test -p local-minimal-node --offline test_deploy_local_cmd_normalizes_profile_name_switch`
  - 初始失败，证明 CMD 参数兼容层尚未收口 profile 语义
- `cargo test -p local-minimal-node --offline test_quick_start_doc_freezes_full_local_command_surface -- --exact`
  - 本轮补充文档 contract 后先失败，证明 `快速启动脚本.md` 还未公开 `local-default` 与 `profile selector`

### Green
- `cargo test -p local-minimal-node --offline test_deployment_profiles_and_templates_document_local_minimal_and_local_default_contracts`
- `cargo test -p local-minimal-node --offline test_deploy_local_scripts_expose_profile_selection_contract`
- `cargo test -p local-minimal-node --offline test_deploy_local_ps1_forwards_profile_name_to_bootstrap_script`
- `cargo test -p local-minimal-node --offline test_deploy_local_cmd_normalizes_profile_name_switch`
- `cargo test -p local-minimal-node --offline test_quick_start_doc_freezes_full_local_command_surface -- --exact`

## Fresh 验证
- `cargo fmt --all --check`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test`
- `powershell -NoProfile -ExecutionPolicy Bypass -File bin/deploy-local.ps1 -Help`
- `cmd /c bin\\deploy-local.cmd --help`

## 结论
- `CP10-2` 现在已不再只是“多了两个文件名”，而是具备：
  - 真实 profile 边界文档
  - 真实模板入口
  - 真实跨平台 profile 选择合同
  - 真实回归测试门禁
  - 根 README 与快启文档的一致入口语义
- `CP10-2`：通过。

## 下一轮继续做什么
1. 不停留在 `CP10-2`
2. 立刻进入 `CP10-3`，把 profile-aware `deploy-local` 作为统一入口，冻结健康检查与 smoke 的可重复执行证据
3. `Step 10` 整步仍未闭环，后续还需要继续完成：
   - `CP10-3`
   - `CP10-4`
