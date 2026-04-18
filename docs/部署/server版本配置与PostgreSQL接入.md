# Server 版本配置与 PostgreSQL 接入

`craw-chat-server` 默认采用 PostgreSQL 作为 server 版本数据库基线。

## 已部署 PostgreSQL 的接入方式

如果 PostgreSQL 已经由 DBA 或现有平台安装部署完成，推荐直接使用 configuration file 方式接入，而不是让安装器强制创建数据库。

核心文件：

- `server.yaml`
- `server.env`
- `storage/postgresql.yaml`
- `secrets/postgresql.password`

## `storage/postgresql.yaml` 示例

```yaml
provider: postgresql

connection:
  host: 10.10.20.15
  port: 5432
  database: craw_chat
  username: craw_chat_app
  passwordFile: ./secrets/postgresql.password
  sslmode: require
  applicationName: craw-chat-server
  connectTimeoutSeconds: 10

schema:
  name: craw_chat
  provisioningMode: none
  migrationMode: apply
  expectedVersion: latest

pool:
  minConnections: 5
  maxConnections: 30
  idleTimeoutSeconds: 300
  maxLifetimeSeconds: 1800
```

## 初始化模式

- `verify-only`
- `bootstrap-schema`
- `create-db-and-schema`

推荐关系：

- 外部已建库已建账号: `verify-only`
- 外部已装 PostgreSQL 但 schema 未初始化: `bootstrap-schema`
- 授权应用创建数据库与 schema: `create-db-and-schema`

## 关键字段

- `passwordFile`
- `migrationMode`
- `provisioningMode`

## 注意事项

- `external-managed` 场景下，直接维护 configuration file 即可。
- 第一阶段 `init-storage-server` 会真实校验 file-based contract 并生成 report，不会伪造 live connectivity 成功。
- 后续阶段再补 PostgreSQL 真连接验证、migrate apply 和 schema version gate。
