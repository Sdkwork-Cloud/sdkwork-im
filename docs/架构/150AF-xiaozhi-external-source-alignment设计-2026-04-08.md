# 150AF - xiaozhi external source alignment 设计

## 设计目标

`xiaozhi` 不是一个可以只靠名称接入的协议标签。后续所有 `iot-xiaozhi` 设计和实现都必须以官方源码仓库 `https://github.com/78/xiaozhi-esp32.git` 为真源，并统一沉淀到仓库外部源码目录 `external/xiaozhi-esp32`。

## external 标准

- 目标目录：`external/xiaozhi-esp32`
- 标准命令：`git submodule add https://github.com/78/xiaozhi-esp32.git external/xiaozhi-esp32`
- 该目录的职责不是直接被业务代码引用，而是作为协议设计、握手语义、会话行为和设备接入标准的外部真源。

## 抽象边界

### `DeviceAccessProvider`

负责：
- 设备注册
- 设备鉴权
- 设备会话
- tenant / owner 绑定
- 设备管理和接入体系

### `IotProtocolAdapter`

负责：
- `xiaozhi` 协议帧解析与编码
- topic / channel 映射
- 上行消息和下行消息归一化
- 协议错误与领域错误映射

## 对齐策略

- 每次推进 `iot-xiaozhi` 前，先反复阅读 `https://github.com/78/xiaozhi-esp32.git`
- 先识别真实实现中的：
  - 握手
  - 鉴权
  - 设备上线/离线
  - telemetry / command
  - 状态同步
  - 错误处理
- 再决定哪些归入 `DeviceAccessProvider`，哪些归入 `IotProtocolAdapter`

## 非目标

- 不在本轮伪造 `xiaozhi` submodule 已经拉取
- 不在本轮伪造 `iot-xiaozhi` 已经进入可运行 adapter 状态
- 不允许把 `xiaozhi` 私有协议字段直接扩散到业务主链
