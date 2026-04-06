# 安全、多租户、SaaS 与私有化部署设计

## 1. 安全标准

- 外部 TLS 1.3
- 内部 mTLS
- workload identity
- 最小权限
- 默认审计
- tenant-aware authz
- 风险隔离与限流

## 2. 身份模型

### 2.1 外部身份

- 用户
- 应用
- 设备
- Bot
- 服务账号

### 2.2 内部身份

- service identity
- runtime identity
- node identity

## 3. 多租户隔离

### 3.1 身份隔离

每个连接和请求必须绑定明确 `tenant_id`，且该值必须来源于已校验的认证上下文，而不是客户端业务请求体中可伪造的字段。

### 3.2 配额隔离

至少支持：

- tenant connection quota
- tenant send TPS
- tenant stream throughput
- tenant media upload quota
- tenant automation quota

### 3.3 调度隔离

- fair queue
- shuffle sharding
- noisy neighbor control
- hot tenant isolation lane

### 3.4 数据隔离

- shared logical isolation
- dedicated cell
- dedicated storage lane

### 3.5 故障隔离

- projection failure isolation
- automation worker isolation
- tenant lane throttling

## 4. SaaS 部署设计

### 4.1 Shared Cell

适用于大多数租户：

- 多租户共享基础设施
- 强治理
- 高资源利用率

### 4.2 Dedicated Cell

适用于高价值租户：

- 专属 cell
- 专属 lane
- 更高 SLA

## 5. 私有化部署设计

### 5.1 Standard

- 支持接入客户 IAM / KMS / 对象存储
- 支持本地升级与回滚

### 5.2 Restricted

- 受限联网
- 可导入离线依赖

### 5.3 Air-Gapped

- 离线镜像
- 离线许可证
- 离线升级包
- 离线诊断工具

## 6. 安装部署原则

- profile 驱动装配
- 配置模板化
- 支持最小闭环安装
- 支持滚动升级
- 支持安全下线与扩容

## 7. 备份与恢复

- metadata snapshot
- journal retention + replay
- object storage backup
- projection rebuild
- 租户级恢复预案

## 8. 运维与诊断

必须支持：

- health check
- lag diagnostics
- route ownership view
- rebalance progress
- drain status
- diagnostic bundle export

当前最小实现已落地：

- `GET /api/v1/ops/health`
- `GET /api/v1/ops/cluster`
- `GET /api/v1/ops/lag`
- `GET /api/v1/ops/diagnostics`
- `GET /api/v1/audit/export`
