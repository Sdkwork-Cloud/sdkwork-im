# 多环境Profile与配置模板

本文用于冻结 `Step 10 / CP10-2` 当前阶段已经真实存在、可以被引用和验证的部署 profile 与配置模板边界。

## 1. 当前状态总览

当前仓库里已经存在的 profile 资产只有两类：

- `local-minimal`
  - 真实可执行
  - 对应单机最小闭环
  - 已有脚本、compose、smoke、runtime 运维命令
- `local-default`
  - 真实存在，但当前仍是兼容基线
  - 通过 `deployments/docker-compose/local-default.yml` 继承 `local-minimal.yml`
  - 作用是为后续“更接近默认本地开发环境”的 profile 保留稳定名字和模板入口

后续规划中的 profile 当前只做边界声明，不宣称可执行：

- `private-saas-single-cell`
- `cloud-shared-cell`
- `cloud-dedicated-cell`

## 2. Profile矩阵

| Profile | 当前状态 | 真实资产 | 主要用途 | 当前边界 |
| --- | --- | --- | --- | --- |
| `local-minimal` | 已落地 | `bin/*-local.*`、`deployments/docker-compose/local-minimal.yml`、`tools/smoke/local_stack_smoke.*` | 单机开发、最小闭环、恢复演练 | 唯一完整脚本闭环 |
| `local-default` | 已命名、已建 compose 入口 | `deployments/docker-compose/local-default.yml`、`deployments/templates/local-default.env.example` | 为默认本地开发 profile 预留稳定入口 | 当前仍复用 `local-minimal` 服务合同 |
| `private-saas-single-cell` | 规划中 | 文档边界 | 私有化单 cell | 当前无 compose / 脚本 / 模板 |
| `cloud-shared-cell` | 规划中 | 文档边界 | 共享 SaaS cell | 当前无 compose / 脚本 / 模板 |
| `cloud-dedicated-cell` | 规划中 | 文档边界 | 独占租户 cell | 当前无 compose / 脚本 / 模板 |

## 3. 当前已冻结的模板资产

当前模板文件统一放在：

- `deployments/templates/local-minimal.env.example`
- `deployments/templates/local-default.env.example`

它们当前只冻结最小本地运行所需的公共合同：

- `CRAW_CHAT_BIND_ADDR`
- `CRAW_CHAT_RUNTIME_DIR`
- `CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET`

这些模板的定位是：

- 作为 profile 配置占位模板
- 作为后续私有化/多环境参数层的起点
- 作为 README 与部署文档引用的稳定资产

这些模板当前不承担生产配置真源角色，也不替代：

- 密钥管理系统
- 多环境配置中心
- control-plane 下发的 effective snapshot

## 4. `local-minimal` 与 `local-default` 的当前关系

### 4.1 `local-minimal`

- 当前唯一完整闭环 profile
- 对应：
  - 本地脚本安装/初始化/启动/状态/重启/停止
  - Docker Compose 启动
  - smoke 验证
  - runtime inspect / repair / backup / archive / restore 运维闭环

### 4.2 `local-default`

- 当前不是新的依赖拓扑
- 当前通过 `local-default.yml` 继承 `local-minimal.yml`
- 这个名字已经冻结，但当前语义仍是：
  - 默认本地开发 profile 的兼容占位名
  - 为后续外部依赖、默认开发拓扑、更多本地基础设施预留扩展位

因此当前阶段必须明确：

- `local-default` 已经是受支持的 profile 名称
- 但 `local-default` 还不是独立的脚本闭环或独立服务编排集合
- 若需要完整本地闭环，仍应优先使用 `local-minimal`

## 5. 规划中的多环境Profile边界

### 5.1 `private-saas-single-cell`

- 目标：私有化单 cell 交付
- 预期后续补齐：
  - 私有化模板
  - 外部依赖地址
  - secrets/证书参数
  - bootstrap / smoke / 恢复脚本

### 5.2 `cloud-shared-cell`

- 目标：共享 SaaS cell
- 预期后续补齐：
  - cell 级配置模板
  - 多节点编排入口
  - 共享基础设施依赖
  - SLO / smoke / drain 操作口径

### 5.3 `cloud-dedicated-cell`

- 目标：独占租户 cell
- 预期后续补齐：
  - tenant/cell 级模板
  - 独占资源边界
  - 发布与回滚口径
  - 审计与恢复演练入口

## 6. 模板使用原则

- 只把模板文件当作 example，不把真实密钥写入仓库
- profile 模板命名必须和 profile 名字保持一一对应
- 当前阶段允许 `local-default` 暂时复用 `local-minimal` 的服务合同
- 后续如果 `local-default` 演进出独立依赖拓扑，必须继续沿用同名模板入口，而不是重新造一个别名

## 7. 当前结论

`CP10-2` 当前阶段先冻结的不是“所有 profile 都可执行”，而是：

- 哪些 profile 已经存在真实资产
- 哪些 profile 只是规划边界
- 哪些模板文件已经形成稳定入口
- `local-minimal` 与 `local-default` 当前各自承担什么责任

这样后续继续推进 compose、smoke、恢复演练时，就不会把“存在一个文件名”误判成“已经有一个完整 profile”。
