# Server �??�?��?�置�?PostgreSQL �?��?�

`sdkwork-im-server` �??SDKWork Chat �??server �??�?��?�口�??�?�?��?�??identity �?`chat`�?�?�?�??名为 `sdkwork-chat`�?对�?SDKWork 路�?�?`/sdkwork/chat`�??

Server �??container �?认使�?� PostgreSQL�??Desktop �?��?��?�据�?认使�?� SQLite�?�??件位�?`~/.sdkwork/chat/data/chat.sqlite`�??

## �?�??�??件

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

## chat.toml �?�据�?段

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

## postgresql.yaml �?��?�??�?�?��?��?

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

## �?��?�??模�?

- `verify-only`: �?�?�已建�?�?�已建账号�?�已�?schema�?只校�?�?�置�??�?�?��??
- `bootstrap-schema`: �?�?�已�?�?PostgreSQL�?�?�?�账号可�?��?�??schema�??
- `create-db-and-schema`: 管�?�??�??�?�??�?��??建�?�据�?�??schema�??

`init-storage-server` 使�?� `/etc/sdkwork/chat/postgresql.yaml` �??平台�?价路�?�??�?码来�??`database.secret` �??平台�?�?��?不�?��??�?��?�?�??�??

## �?��?�?�?��?�??

�?�??�?�?��?��?�??`SDKWORK_IM_*`�?

```env
SDKWORK_IM_DEPLOYMENT_MODE=server
SDKWORK_IM_CONFIG_FILE=/etc/sdkwork/chat/chat.toml
SDKWORK_IM_DATA_DIR=/var/lib/sdkwork/chat
SDKWORK_IM_LOG_DIR=/var/log/sdkwork/chat
SDKWORK_IM_RUN_DIR=/run/sdkwork/chat
SDKWORK_IM_DATABASE_ENGINE=postgresql
SDKWORK_IM_DATABASE_HOST=10.10.20.15
SDKWORK_IM_DATABASE_PORT=5432
SDKWORK_IM_DATABASE_NAME=sdkwork_chat_prod
SDKWORK_IM_DATABASE_SCHEMA=sdkwork_chat_prod
SDKWORK_IM_DATABASE_USERNAME=sdkwork_chat_prod
SDKWORK_IM_DATABASE_PASSWORD_FILE=/etc/sdkwork/chat/database.secret
SDKWORK_IM_DATABASE_SSL_MODE=require
```

`DATABASE_PROVIDER` �??`DATABASE_SSLMODE` 不�?��?�??名称�??
