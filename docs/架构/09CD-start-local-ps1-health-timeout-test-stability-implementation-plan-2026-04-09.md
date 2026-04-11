# start-local.ps1 Health Timeout Test Stability Implementation Plan

## Goal

- 修复健康超时回滚回归测试的时序脆弱性，恢复可重复验证能力。

## Steps

- 用现有失败回归固定问题。
- 为 marker 落盘增加短暂轮询。
- 把测试加速窗口从 `2 x 100ms` 放宽到 `5 x 100ms`。
- 回跑重复单测、全量部署契约测试、格式检查和包级回归。

## Boundary

- 不改生产脚本。
- 只收敛测试时序模型。
