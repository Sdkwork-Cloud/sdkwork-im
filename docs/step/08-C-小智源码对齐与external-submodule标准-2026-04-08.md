# 08-C - 小智源码对齐与 external submodule 标准

## 本轮目标

把 `xiaozhi` 从“只存在于 IoT 协议矩阵中的名称”推进到“有明确官方源码真源、明确 external submodule 落位、明确双层抽象边界和反复对齐机制”的可执行标准。

## 官方真源与标准落位

- 官方源码真源固定为：`https://github.com/78/xiaozhi-esp32.git`
- 仓库内标准落位固定为：`external/xiaozhi-esp32`
- 标准命令固定为：`git submodule add https://github.com/78/xiaozhi-esp32.git external/xiaozhi-esp32`

## 为什么要补这一轮

- 当前 IoT 主线虽然已经有 `iot-mqtt` baseline，但 `xiaozhi` 仍停留在名字级声明。
- 如果没有官方源码真源，后续 `iot-xiaozhi` 很容易退化成“根据猜测写协议适配器”。
- 用户要求能够反复阅读源码并持续对齐标准实现，因此必须先冻结 external submodule 标准，再冻结设计边界。

## 双层抽象边界

- `DeviceAccessProvider`
  - 负责设备注册、鉴权、会话、tenant/owner 绑定、封禁、设备管理和接入体系
- `IotProtocolAdapter`
  - 负责 `xiaozhi` 协议帧、上行/下行消息、topic/channel 映射、错误语义映射

`xiaozhi` 不得绕开统一领域模型。所有协议差异都必须停留在 `IotProtocolAdapter`，所有接入生命周期都必须停留在 `DeviceAccessProvider`。

## 对齐要求

- 后续每一轮都必须反复阅读 `https://github.com/78/xiaozhi-esp32.git` 对应源码，再更新：
  - `docs/架构`
  - `docs/step`
  - `docs/review`
- 对齐内容至少覆盖：
  - 握手与认证
  - 设备会话
  - telemetry / command 映射
  - 设备状态、错误码、重连与恢复语义

## 当前环境结论

- 标准已经冻结到 `external/xiaozhi-esp32`
- 当前环境尚未完成真实 submodule 拉取，因此本轮不伪造源码目录、不伪造 gitlink、不伪造已对齐实现
- 下一轮以 `xiaozhi` 源码实际拉取和 `iot-xiaozhi` adapter 基线为优先
