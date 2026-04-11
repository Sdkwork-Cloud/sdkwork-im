# 09AF - 实施计划 - xiaozhi 源码 external submodule 与协议对齐

## 目标

为 `xiaozhi` 建立可反复迭代的源码真源标准，确保后续 `iot-xiaozhi` 实现始终以 `https://github.com/78/xiaozhi-esp32.git` 为依据，而不是按文档猜协议。

## 最小实施面

1. 冻结官方源码真源：`https://github.com/78/xiaozhi-esp32.git`
2. 冻结 external 标准落位：`external/xiaozhi-esp32`
3. 冻结标准命令：`git submodule add https://github.com/78/xiaozhi-esp32.git external/xiaozhi-esp32`
4. 冻结双层抽象边界：
   - `DeviceAccessProvider`
   - `IotProtocolAdapter`
5. 明确反复阅读源码并持续回写 `docs/step`、`docs/架构`、`docs/review`

## 约束

- 不伪造 `xiaozhi` submodule 已经拉取成功
- 不伪造 `iot-xiaozhi` 已完成真实运行时实现
- `xiaozhi` 相关设计只能落在：
  - `DeviceAccessProvider` 的设备管理和接入体系
  - `IotProtocolAdapter` 的协议编解码和统一映射

## 放行标准

- `external/README.md` 已冻结 `xiaozhi` submodule 标准
- `docs/step/08-C-小智源码对齐与external-submodule标准-2026-04-08.md` 已建立
- `docs/架构/150AF-xiaozhi-external-source-alignment设计-2026-04-08.md` 已建立
- `docs/review/continuous-optimization-xiaozhi-external-source-alignment-2026-04-08.md` 已记录环境阻塞和下一轮动作
