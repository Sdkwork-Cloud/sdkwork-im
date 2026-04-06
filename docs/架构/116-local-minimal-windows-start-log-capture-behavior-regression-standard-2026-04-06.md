# 116. local-minimal Windows 启动日志接管行为回归标准

## 1. 目标

在已经冻结“Windows 后台启动必须接管 stdout/stderr”实现标准之后，继续冻结该能力的自动化验证标准，避免回归。

## 2. 标准

### 2.1 仅检查脚本文本不够

若测试只断言脚本中出现了：

- `-RedirectStandardOutput`
- `-RedirectStandardError`

则仍不足以证明日志链路真实可用。

### 2.2 必须有行为级验证

Windows 启动链路必须至少具备一个自动化行为测试，验证以下事实：

1. 后台子进程确实被启动
2. 子进程 stdout 会被写入约定的 `local-minimal-node.out.log`
3. 即使服务未就绪并快速退出，日志接管仍然生效

### 2.3 行为测试必须脱离真实服务依赖

行为测试应优先采用“稳定输出、快速退出”的系统可执行文件替身，以降低：

- 真实服务启动成本
- 端口依赖
- 网络依赖
- 时间敏感性

当前基线做法：

- 使用假的 `cargo.cmd`
- 使用 `whoami.exe` 作为 `local-minimal-node.exe` 替身

## 3. 验证要求

以下验证至少一项必须常驻在自动化测试中：

- `test_start_local_ps1_captures_background_process_stdout_into_documented_log_file`

其目标不是验证服务功能，而是验证启动脚本的运维契约。

## 4. 价值

这类测试的价值在于：

- 保护私有化部署脚本质量
- 保护 Windows/Linux 运维行为一致性
- 在功能测试之外，补齐 operator contract 的回归防线
