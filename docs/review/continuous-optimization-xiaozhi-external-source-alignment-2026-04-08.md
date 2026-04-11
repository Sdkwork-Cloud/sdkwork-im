# Continuous Optimization - xiaozhi external source alignment - 2026-04-08

## 1. 本轮目标

- 冻结 `xiaozhi` 官方源码真源：`https://github.com/78/xiaozhi-esp32.git`
- 冻结 external 标准落位：`external/xiaozhi-esp32`
- 冻结标准命令：`git submodule add https://github.com/78/xiaozhi-esp32.git external/xiaozhi-esp32`
- 冻结 `DeviceAccessProvider` 与 `IotProtocolAdapter` 的双层抽象边界

## 2. 当前环境证据

- 试图验证远端可达性时，执行了：`git ls-remote https://github.com/78/xiaozhi-esp32.git HEAD`
- 当前环境返回的核心失败信息包含：`unexpected eof`
- 结论：
  - 当前环境未完成实际 submodule 拉取
  - 当前环境不能诚实地宣称 `external/xiaozhi-esp32` 已成为真实 git submodule

## 3. 本轮真实交付

- 建立 `08-C` step 文档，明确 `xiaozhi` 源码对齐与反复迭代机制
- 建立 `150AF` 架构设计文档，明确 external 真源与双层抽象
- 建立 `external/README.md`，冻结 submodule 标准而不是伪造源码副本
- 回写主架构与 Step 08，使 `xiaozhi` 不再只是协议矩阵中的名称

## 4. 当前判断

- 这一轮的正确动作是冻结标准，不是伪造结果
- 后续真正落地 `xiaozhi` 时，必须继续围绕：
  - `DeviceAccessProvider`
  - `IotProtocolAdapter`
  - `https://github.com/78/xiaozhi-esp32.git`
  - `external/xiaozhi-esp32`
展开反复阅读、对齐和回写
