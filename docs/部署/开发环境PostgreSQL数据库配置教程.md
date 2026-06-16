# 开发环境 PostgreSQL 数据库配置教程

本文说明 SDKWork Chat 在本地开发环境中如何显式使用 PostgreSQL。当前应用标准身份为：

- app code: `chat`
- 发布访问根路径: `/sdkwork/chat`
- 环境变量前缀: `SDKWORK_IM_*`
- desktop 本地数据默认数据库: SQLite
- desktop SQLite 路径: `~/.sdkwork/chat/data/chat.sqlite`

## 1. 本地启动入口

浏览器集成开发可以使用 PostgreSQL profile：

```powershell
pnpm dev
pnpm dev:postgres
```

desktop 本地数据必须默认保留 SQLite：

```powershell
pnpm tauri:dev
```

`pnpm tauri:dev 默认使用 SQLite`，数据库文件写入 `~/.sdkwork/chat/data/chat.sqlite`，Windows 等价路径是 `%USERPROFILE%\.sdkwork\chat\data\chat.sqlite`。

只有在需要用 desktop shell 诊断 PostgreSQL 集成时，才显式执行：

```powershell
pnpm tauri:dev:postgres
```

显式 SQLite 入口保留：

```powershell
pnpm dev:sqlite
pnpm tauri:dev:sqlite
```

## 2. 配置文件位置

开发 PostgreSQL 使用仓库根目录 `.env.postgres`。仓库只提交模板 `.env.postgres.example`，本机真实配置不要提交到 Git。

创建本机配置：

```powershell
Copy-Item .env.postgres.example .env.postgres
```

## 3. 标准 PostgreSQL 配置

`.env.postgres` 使用拆分字段维护数据库连接，不把 host、database、username、password 混在一条长 URL 中。

```env
SDKWORK_IM_DEPLOYMENT_MODE=server
SDKWORK_IM_DATABASE_ENGINE=postgresql
SDKWORK_IM_DATABASE_HOST=127.0.0.1
SDKWORK_IM_DATABASE_PORT=5432
SDKWORK_IM_DATABASE_NAME=sdkwork_ai_dev
SDKWORK_IM_DATABASE_SCHEMA=sdkwork_ai_dev
SDKWORK_IM_DATABASE_USERNAME=sdkwork_ai_dev
SDKWORK_IM_DATABASE_PASSWORD=sdkworkdev123
SDKWORK_IM_DATABASE_SSL_MODE=disable
SDKWORK_IM_DATABASE_MAX_CONNECTIONS=10

SDKWORK_IM_DATABASE_ADMIN_HOST=127.0.0.1
SDKWORK_IM_DATABASE_ADMIN_PORT=5432
SDKWORK_IM_DATABASE_ADMIN_USERNAME=postgres
SDKWORK_IM_DATABASE_ADMIN_PASSWORD=postgres_admin_pass
SDKWORK_IM_DATABASE_ADMIN_DATABASE=postgres
SDKWORK_IM_DATABASE_ADMIN_SSL_MODE=disable
```

脚本会组装并桥接给当前 Rust 运行时所需的数据库 URL：

```text
postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:5432/sdkwork_ai_dev?sslmode=disable
```

## 4. 字段说明

| 变量 | 说明 | 示例 |
| --- | --- | --- |
| `SDKWORK_IM_DATABASE_ENGINE` | 数据库类型。开发 PostgreSQL 固定为 `postgresql`。 | `postgresql` |
| `SDKWORK_IM_DATABASE_HOST` | PostgreSQL 主机名或 IP。 | `127.0.0.1` |
| `SDKWORK_IM_DATABASE_PORT` | PostgreSQL 端口。 | `5432` |
| `SDKWORK_IM_DATABASE_NAME` | 应用使用的数据库名。 | `sdkwork_ai_dev` |
| `SDKWORK_IM_DATABASE_SCHEMA` | 应用表结构所在 schema。建议与数据库名保持一致。 | `sdkwork_ai_dev` |
| `SDKWORK_IM_DATABASE_USERNAME` | 应用数据库账号。 | `sdkwork_ai_dev` |
| `SDKWORK_IM_DATABASE_PASSWORD` | 应用数据库密码。 | `sdkworkdev123` |
| `SDKWORK_IM_DATABASE_SSL_MODE` | PostgreSQL SSL 模式。本地通常用 `disable`。 | `disable` |
| `SDKWORK_IM_DATABASE_MAX_CONNECTIONS` | 本地连接池最大连接数。 | `10` |
| `SDKWORK_IM_DATABASE_ADMIN_USERNAME` | 初始化数据库时使用的 PostgreSQL 管理账号。 | `postgres` |
| `SDKWORK_IM_DATABASE_ADMIN_PASSWORD` | 管理账号密码。只用于 `pnpm db:postgres:init`。 | `postgres_admin_pass` |
| `SDKWORK_IM_DATABASE_ADMIN_DATABASE` | 管理账号先连接的维护库。通常是 `postgres`。 | `postgres` |
| `SDKWORK_IM_DATABASE_ADMIN_URL` | 可选完整管理员连接串。设置后优先于拆分管理员字段。 | `postgresql://postgres:postgres_admin_pass@127.0.0.1:5432/postgres?sslmode=disable` |

`DATABASE_PROVIDER` 和 `DATABASE_SSLMODE` 不是标准名称。新配置必须使用 `DATABASE_ENGINE` 和 `DATABASE_SSL_MODE`。

## 5. 初始化和升级数据库

执行前确认本机已安装 PostgreSQL 客户端工具，命令行可以访问 `psql`：

```powershell
psql --version
```

查看计划，不连接或修改数据库：

```powershell
pnpm db:postgres:plan
```

初始化数据库、应用账号、schema、授权和默认权限：

```powershell
pnpm db:postgres:init
```

执行仓库内 PostgreSQL 迁移 SQL：

```powershell
pnpm db:postgres:migrate
```

`pnpm db:postgres:init` 需要 `SDKWORK_IM_DATABASE_ADMIN_PASSWORD` 或 `SDKWORK_IM_DATABASE_ADMIN_URL`。`pnpm db:postgres:migrate` 使用应用账号连接目标数据库，并执行 `deployments/database/postgres/migrations/001_im_core_schema.sql`。

脚本通过 `PGPASSWORD` 把密码传给 `psql`，不会把密码拼进命令行参数。`pnpm db:postgres:plan` 输出中的密码会被替换为 `***`。

## 6. Windows 应用访问 WSL Ubuntu PostgreSQL

如果 SDKWork Chat 和 `pnpm` 在 Windows PowerShell 中运行，而 PostgreSQL 安装在 WSL Ubuntu 中，`.env.postgres` 的 host 必须填写 Windows 能访问到的 TCP 地址。优先使用：

```env
SDKWORK_IM_DATABASE_HOST=127.0.0.1
```

验证端口：

```powershell
Test-NetConnection 127.0.0.1 -Port 5432
```

如果需要使用 WSL IP：

```powershell
wsl hostname -I
```

然后把 `SDKWORK_IM_DATABASE_HOST` 改成对应 IP。Windows 应用不应该使用 WSL 内部的 Unix socket。

## 7. 手工连接验证

```powershell
psql "host=127.0.0.1 port=5432 dbname=sdkwork_ai_dev user=sdkwork_ai_dev password=sdkworkdev123 sslmode=disable"
```

成功后可检查当前库、用户和 schema：

```sql
SELECT current_database(), current_user, current_schema();
```

## 8. 与 production 的边界

`.env.postgres` 仅用于本地开发和集成测试。server 发布包使用 `/etc/sdkwork/chat/chat.toml`、`/etc/sdkwork/chat/server.env`、`/etc/sdkwork/chat/postgresql.yaml` 和 `/etc/sdkwork/chat/database.secret`。desktop 发布包和 desktop 默认开发入口不依赖 PostgreSQL，默认 SQLite 数据文件是 `~/.sdkwork/chat/data/chat.sqlite`。
