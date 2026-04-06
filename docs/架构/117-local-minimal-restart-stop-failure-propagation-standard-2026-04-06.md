# 117. local-minimal restart stop failure 传播标准

## 1. 目标

冻结 `local-minimal` 生命周期脚本中 restart 的失败传播语义，避免出现“停止失败但重启继续”的假成功路径。

## 2. 标准

### 2.1 restart 不得吞掉 stop 的异常失败

若 restart 由两个阶段组成：

1. stop
2. start

则当 stop 返回真正的异常失败时，restart 必须立即失败，不得继续进入 start。

### 2.2 “未运行”不属于异常失败

`stop-local.sh` / `stop-local.ps1` 应自行把“进程未运行”的场景归一化为成功返回。

因此 restart 层无需也不应再额外通过：

- `|| true`
- 或等价的错误吞掉逻辑

来弱化 stop 的失败信号。

### 2.3 跨平台必须一致

- Windows restart 已传播 stop 失败
- Linux restart 必须与其保持一致

## 3. 风险说明

若 restart 吞掉 stop 失败，会带来：

- 旧实例未退干净时继续启动新实例
- 端口冲突
- 双进程
- 启动结果误判
- 排障复杂化

## 4. 验证要求

自动化测试必须至少包含：

- restart 脚本不吞掉 stop 失败的契约断言

后续可继续扩展为行为级回归测试。
