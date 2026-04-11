# external

本目录用于存放外部真源代码，不直接替代业务实现，也不伪造第三方源码镜像。

## xiaozhi 标准

- 官方源码真源：`https://github.com/78/xiaozhi-esp32.git`
- 标准落位：`external/xiaozhi-esp32`
- 标准命令：`git submodule add https://github.com/78/xiaozhi-esp32.git external/xiaozhi-esp32`

## 对齐原则

- `xiaozhi` 相关实现必须反复阅读官方源码，再决定如何拆分：
  - `DeviceAccessProvider`
  - `IotProtocolAdapter`
- 本目录是对齐真源，不是把 `xiaozhi` 私有实现直接拷进主业务链。
