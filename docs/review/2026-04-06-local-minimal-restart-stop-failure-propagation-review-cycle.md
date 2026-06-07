# local-minimal restart stop failure 传播 Review Cycle

## 1. Review 范围

- [restart-local.sh](<workspace-root>/craw-chat/bin/restart-local.sh)
- [deployment_profile_test.rs](<workspace-root>/craw-chat/services/local-minimal-node/tests/deployment_profile_test.rs)
- 本轮 `/docs/review/` 与 `/docs/架构/` 文档沉淀

## 2. 问题列表

### 2.1 中高风险：restart 脚本吞掉 stop 失败

问题表现：
- Linux restart 脚本中存在 `|| true`
- 这会让“停止失败”和“停止成功”在控制流上没有区别

影响：
- 旧实例仍在运行时，新实例可能继续被启动
- 端口冲突、双实例和错误定位复杂化
- 同一产品在 Windows/Linux 上生命周期语义不一致

## 3. 根因

- `stop-local.sh` 的“未运行”场景已经被内部吸收为 `exit 0`
- 因此 restart 层继续吞掉 stop 失败，不是在增强容错，而是在掩盖异常

## 4. 修复策略

- 不修改 `stop-local.sh`
- 不修改 `restart-local.ps1`
- 只移除 Linux restart 对 stop 失败的吞掉逻辑，使其和 Windows 行为对齐

## 5. 实施记录

### 5.1 红灯

- 新增部署契约断言：`restart-local.sh` 不得包含 `|| true`
- 红灯结果：针对性测试失败

### 5.2 绿灯

- 将：
  - `bash "${ROOT_DIR}/bin/stop-local.sh" || true`
- 修改为：
  - `bash "${ROOT_DIR}/bin/stop-local.sh"`

## 6. 当前结论

- 本轮已关闭“Linux restart 吞掉 stop 失败”的运维一致性缺口
- 这是 lifecycle contract 修复，不是业务功能变更

## 7. 下一步计划

1. 继续收敛 restart/status/stop 的失败诊断输出
2. 继续扩展行为级回归测试，覆盖更多脚本异常场景
3. 回到 backlog 中剩余的高优先级缺口继续闭环
