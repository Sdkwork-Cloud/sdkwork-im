# Server 版本配置与 PostgreSQL 接入

`craw-chat-server` 是 SDKWork Chat 的 server 版本入口。应用标准 identity 为 `chat`，发布包名为 `sdkwork-chat`，对外 SDKWork 路径为 `/sdkwork/chat`。

Server 和 container 默认使用 PostgreSQL。Desktop 本地数据默认使用 SQLite，文件位于 `~/.sdkwork/chat/data/chat.sqlite`。

## 标准文件

Linux server/service/container:

```text
/etc/sdkwork/chat/chat.toml
/etc/sdkwork/chat/server.env
/etc/sdkwork/chat/postgresql.yaml
/etc/sdkwork/chat/database.secret
/var/lib/sdkwork/chat
/var/log/sdkwork/chat
/run/sdkwork/chat
```

Windows Service:

```text
%ProgramFiles%/sdkwork/chat
%ProgramData%/sdkwork/chat/chat.toml
%ProgramData%/sdkwork/chat/server.env
%ProgramData%/sdkwork/chat/postgresql.yaml
%ProgramData%/sdkwork/chat/database.secret
%ProgramData%/sdkwork/chat/Data
%ProgramData%/sdkwork/chat/Logs
%ProgramData%/sdkwork/chat/Run
```

macOS service:

```text
/usr/lib/sdkwork/chat
/Library/Application Support/sdkwork/chat/chat.toml
/Library/Application Support/sdkwork/chat/server.env
/Library/Application Support/sdkwork/chat/postgresql.yaml
/Library/Application Support/sdkwork/chat/database.secret
/Library/Application Support/sdkwork/chat/Data
/Library/Logs/sdkwork/chat
/Library/Application Support/sdkwork/chat/Run
```

## chat.toml 数据库段

```toml
[database]
engine = "postgresql"
host = "10.10.20.15"
port = 5432
database = "sdkwork_chat_prod"
schema = "sdkwork_chat_prod"
username = "sdkwork_chat_prod"
password_file = "/etc/sdkwork/chat/database.secret"
ssl_mode = "require"
max_connections = 20
```

## postgresql.yaml 初始化辅助配置

```yaml
provider: postgresql

connection:
  host: 10.10.20.15
  port: 5432
  database: sdkwork_chat_prod
  username: sdkwork_chat_prod
  passwordFile: /etc/sdkwork/chat/database.secret
  sslmode: require
  applicationName: sdkwork-chat-server
  connectTimeoutSeconds: 10

schema:
  name: sdkwork_chat_prod
  provisioningMode: none
  migrationMode: apply
  expectedVersion: latest

pool:
  minConnections: 5
  maxConnections: 20
  idleTimeoutSeconds: 300
  maxLifetimeSeconds: 1800
```

## External configuration file contract

`postgresql.yaml` is the server-side external configuration file for PostgreSQL install modes. It freezes the connection secret reference `passwordFile`, schema lifecycle field `migrationMode`, and lifecycle modes `verify-only`, `bootstrap-schema`, and `create-db-and-schema`.

## 初始化模式

- `verify-only`: 外部已建库、已建账号、已建 schema，只校验配置和连接。
- `bootstrap-schema`: 外部已安装 PostgreSQL，应用账号可初始化 schema。
- `create-db-and-schema`: 管理员授权脚本创建数据库和 schema。

`init-storage-server` 使用 `/etc/sdkwork/chat/postgresql.yaml` 或平台等价路径。密码来自 `database.secret` 或平台密钥，不能写入发布包。

## 环境变量覆盖

标准变量前缀是 `SDKWORK_CHAT_*`：

```env
SDKWORK_CHAT_DEPLOYMENT_MODE=server
SDKWORK_CHAT_CONFIG_FILE=/etc/sdkwork/chat/chat.toml
SDKWORK_CHAT_DATA_DIR=/var/lib/sdkwork/chat
SDKWORK_CHAT_LOG_DIR=/var/log/sdkwork/chat
SDKWORK_CHAT_RUN_DIR=/run/sdkwork/chat
SDKWORK_CHAT_DATABASE_ENGINE=postgresql
SDKWORK_CHAT_DATABASE_HOST=10.10.20.15
SDKWORK_CHAT_DATABASE_PORT=5432
SDKWORK_CHAT_DATABASE_NAME=sdkwork_chat_prod
SDKWORK_CHAT_DATABASE_SCHEMA=sdkwork_chat_prod
SDKWORK_CHAT_DATABASE_USERNAME=sdkwork_chat_prod
SDKWORK_CHAT_DATABASE_PASSWORD_FILE=/etc/sdkwork/chat/database.secret
SDKWORK_CHAT_DATABASE_SSL_MODE=require
```

`DATABASE_PROVIDER` 和 `DATABASE_SSLMODE` 不是标准名称。
