# local-minimal restart stop failure 传播实施计划

## 1. 当前阶段

- `local-minimal-node` 已完成基础服务能力、运行时目录治理和跨平台启动日志接管加固。
- 当前进入跨平台 lifecycle script 一致性收口阶段，重点是避免“脚本表面成功、真实状态失真”的运维缺陷。

## 2. 本轮问题

### 2.1 中高风险：Linux restart 脚本会吞掉 stop 失败

问题表现：
- `bin/restart-local.sh` 当前使用：
  - `bash "${ROOT_DIR}/bin/stop-local.sh" || true`
- 这意味着即使 `stop-local.sh` 因超时、权限或其它异常失败，脚本仍会继续执行 `start-local.sh`

风险：
- 旧进程未退出时继续启动新进程，可能形成双进程或端口冲突
- 启动结果会被污染成“假成功”
- Windows `restart-local.ps1` 会传播 stop 失败，Linux 却吞掉失败，跨平台契约不一致

## 3. 根因判断

- Linux restart 脚本把 `stop-local.sh` 当成“可忽略失败步骤”
- 但 `stop-local.sh` 自身已经对“未运行”做了 `exit 0` 处理，因此真正的非零退出都应视为异常，而不是静默跳过

## 4. 实施步骤

1. 先在 `deployment_profile_test.rs` 中增加红灯断言，要求 `restart-local.sh` 不得包含 `|| true`
2. 最小修复 `bin/restart-local.sh`，移除对 stop 失败的吞掉逻辑
3. 补充 review 与架构标准，冻结 restart 行为一致性
4. 运行部署脚本测试与全量离线测试

## 5. 验证命令

- `cargo test -p local-minimal-node --offline --test deployment_profile_test test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints -- --nocapture`
- `cargo test -p local-minimal-node --offline --test deployment_profile_test -- --nocapture`
- `cargo test -p local-minimal-node --offline`

## 6. 下一步

1. 继续检查 `restart-local.ps1` / `restart-local.sh` 是否还存在其它失败传播不一致点
2. 审核 `stop/status` 脚本在失败场景下的诊断输出是否足够给一线运维定位问题
3. 进入下一批 backlog 风险闭环
