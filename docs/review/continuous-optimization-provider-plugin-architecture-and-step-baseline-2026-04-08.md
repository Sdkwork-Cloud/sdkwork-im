# 持续优化：provider/plugin 架构与 step 基线回写 - 2026-04-08

## 1. 当前 step / 波次

- 当前仓库状态：`Wave D / Step 13` 已闭环，处于持续优化模式。
- 本轮定位：持续优化第十七轮增量。
- 本轮主题：冻结 RTC、对象存储、IoT 的 provider/plugin 架构标准，并同步回写 step 与循环执行提示词。

## 2. 本轮为什么做

当前架构文档已经明确了 `AI / Agent / IoT`、`RTC`、`S3-compatible Object Storage` 和控制面治理方向，但还缺一套统一的 provider/plugin 标准，导致：

- RTC 厂商接入边界还不够清晰
- 对象存储多云接入缺少统一 `S3` 契约口径
- 用户模块缺少“本地实现 / 外部系统集成”的统一 plugin 标准
- IoT 的 `MQTT / 小智协议 / 设备管理与接入体系` 还没有收敛成同一张标准图
- `docs/prompts/反复执行Step指令.md` 过长，不利于持续迭代

因此本轮先做文档与执行基线收口，而不是直接散写实现分支。

## 3. 本轮实际完成

- 新增 `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`
  - 冻结 `rtc / object-storage / principal-profile / iot-access / iot-protocol` 五类插件域
  - 冻结 RTC provider 矩阵：`火山引擎 / 阿里云 / 腾讯云`
  - 冻结 RTC 全局默认 provider：`火山引擎`
  - 冻结对象存储通过 `S3` 标准接入 `阿里云 / 腾讯云 / 火山引擎 / AWS / Google / Microsoft`
  - 冻结用户模块两种形态：`本地实现 / 外部系统集成`，默认 `本地实现`
  - 冻结 IoT 首批协议插件：`MQTT / 小智协议`
  - 冻结设备管理与接入体系边界
- 回写架构索引与基线文档：
  - `docs/架构/README.md`
  - `docs/架构/04-技术选型与可插拔策略.md`
  - `docs/架构/07-缓存-流式通信-RTC-通知设计.md`
  - `docs/架构/134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md`
  - `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
  - `docs/架构/142-控制面与配置治理设计-2026-04-06.md`
- 回写 step 基线：
  - `docs/step/README.md`
  - `docs/step/00-总实施原则与执行门禁.md`
  - `docs/step/05-消息与会话主链路重构.md`
  - `docs/step/06-流式与RTC实时能力重构.md`
  - `docs/step/08-AI-Agent-IoT统一扩展层落地.md`
  - `docs/step/10-部署脚本与多环境发布治理.md`
  - `docs/step/13-发布就绪与持续迭代闭环.md`
- 精简 `docs/prompts/反复执行Step指令.md`
  - 保留 step 判断、闭环、波次验收、持续优化、架构回写五类核心要求
  - 去掉重复冗长表述
- 新增文档契约测试：
  - `services/local-minimal-node/tests/provider_plugin_docs_test.rs`

## 4. 改动文件

- `docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md`
- `docs/架构/README.md`
- `docs/架构/04-技术选型与可插拔策略.md`
- `docs/架构/07-缓存-流式通信-RTC-通知设计.md`
- `docs/架构/09-实施计划.md`
- `docs/架构/134-AI-Agent-IoT统一实时通信模型设计-2026-04-06.md`
- `docs/架构/137-部署拓扑与容量规划设计-2026-04-06.md`
- `docs/架构/142-控制面与配置治理设计-2026-04-06.md`
- `docs/step/README.md`
- `docs/step/00-总实施原则与执行门禁.md`
- `docs/step/05-消息与会话主链路重构.md`
- `docs/step/06-流式与RTC实时能力重构.md`
- `docs/step/08-AI-Agent-IoT统一扩展层落地.md`
- `docs/step/10-部署脚本与多环境发布治理.md`
- `docs/step/13-发布就绪与持续迭代闭环.md`
- `docs/prompts/反复执行Step指令.md`
- `services/local-minimal-node/tests/provider_plugin_docs_test.rs`

## 5. 架构回写判断

- `docs/架构` 回写：已完成。
- `docs/step` 回写：已完成。
- `docs/review` 证据：本文件已归档。
- 当前未偏离现有 `130-149` 总纲，新增 `150` 作为 provider/plugin 与设备接入专项标准。

## 6. 当前仍未闭环的内容

本轮完成的是标准冻结与执行口径收口，不是最终代码实现。仍未闭环的内容包括：

- `RtcProviderPort / provider registry` 的真实代码落地
- `PrincipalProfileProvider` 的真实代码落地
- `rtc-volcengine / rtc-aliyun / rtc-tencent` 的 conformance 测试资产
- `ObjectStorageProvider` 的多云 `S3` 实现与配置模板
- `iot-mqtt / iot-xiaozhi` 的协议适配与设备管理主路径
- provider 级 deployment profile 与 control-plane snapshot 的真实运行时接入

## 7. 下一轮建议

建议按最小闭环顺序继续推进：

1. 先做 `principal-profile-upstream-context` 与 `principal-profile-external-catalog` 的统一 `PrincipalProfileProvider`
2. 再做 `rtc-volcengine` 基线，实现 `RtcProviderPort + default provider + artifact 回流边界`
3. 再做 `object-storage-s3` 公共契约与多云 endpoint/profile 模板
4. 再做 `iot-mqtt` 主路径和设备管理 owner seam
5. 最后补 `rtc-aliyun / rtc-tencent / iot-xiaozhi` 适配增量

每一轮都继续复用：

- `docs/review`
- `docs/架构/09-实施计划.md`
- `docs/prompts/反复执行Step指令.md`
- 对应 conformance / smoke / 代码验证
