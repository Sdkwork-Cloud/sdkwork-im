# 剩余独立服务公网认证收口与 Public Builder 补齐（2026-04-05）

## 1. 背景

在 `27-外部认证与Trusted-Identity边界标准-2026-04-05` 与
`28-多入口服务外部认证收敛与控制面保护-2026-04-05` 完成后，
`craw-chat` 仍保留多组可单独启动的服务二进制。

这些服务虽然在 handler 内已经通过 `resolve_auth_context(...)` 解析认证上下文，
但其 `main.rs` 仍直接启动 `build_default_app()`，从而把
`trusted identity headers` 的测试/内部链路兼容能力，错误带到了公网部署入口。

本轮目标是把剩余可独立部署的 app-facing 服务全部补齐为：

1. `build_default_app()` 或 `build_app(...)`
   继续保留给测试装配、内部集成与受信链路。
2. `build_public_app()` 作为公网默认入口，统一执行 `Bearer-only` 认证。
3. 所有 `main.rs` 统一绑定 `build_public_app()`。

## 2. 本轮发现

### 2.1 高风险：剩余七个独立服务仍以 default builder 对外启动

受影响服务：

- `audit-service`
- `automation-service`
- `media-service`
- `ops-service`
- `notification-service`
- `conversation-runtime`
- `projection-service`

共同根因：

- 服务仅提供 `build_default_app()` / `build_app(...)`
- handler 通过 `resolve_auth_context(...)` 兼容 Bearer 与 trusted headers
- `main.rs` 直接启动 default builder
- 一旦这些二进制被单独部署到公网，攻击者即可伪造
  `x-tenant-id`、`x-user-id`、`x-actor-id` 等 trusted headers
  越过 Bearer/JWT 边界

### 2.2 风险面

- `audit-service`
  - `GET /backend/v3/api/audit/records`
  - `GET /backend/v3/api/audit/export`
  - `POST /backend/v3/api/audit/records`
- `automation-service`
  - `POST /im/v3/api/automation/executions`
  - `GET /im/v3/api/automation/executions/{execution_id}`
- `media-service`
  - `POST /im/v3/api/media/uploads`
  - `POST /im/v3/api/media/uploads/{media_asset_id}/complete`
  - `GET /im/v3/api/media/{media_asset_id}`
- `ops-service`
  - `GET /backend/v3/api/ops/health`
  - `GET /backend/v3/api/ops/cluster`
  - `GET /backend/v3/api/ops/lag`
  - `GET /backend/v3/api/ops/diagnostics`
- `notification-service`
  - `POST /im/v3/api/notifications/requests`
  - `GET /im/v3/api/notifications`
  - `GET /im/v3/api/notifications/{notification_id}`
- `conversation-runtime`
  - `POST /im/v3/api/chat/conversations`
  - `POST /im/v3/api/chat/conversations/{conversation_id}/members/add`
  - `POST /im/v3/api/chat/conversations/{conversation_id}/members/remove`
  - `POST /im/v3/api/chat/conversations/{conversation_id}/messages`
  - `POST /im/v3/api/chat/messages/{message_id}/edit`
  - `POST /im/v3/api/chat/messages/{message_id}/recall`
- `projection-service`
  - `GET /im/v3/api/chat/inbox`
  - `GET /im/v3/api/chat/conversations/{conversation_id}`
  - `GET /im/v3/api/chat/conversations/{conversation_id}/read-cursor`
  - `GET /im/v3/api/chat/conversations/{conversation_id}/messages`

## 3. 修复标准

本轮之后，冻结以下标准：

1. 所有可单独部署的 app-facing 服务必须显式提供 `build_public_app()`。
2. `build_public_app()` 仅允许 `/healthz`、`/readyz` 匿名访问。
3. 其余所有外部路由统一通过 `resolve_bearer_auth_context(...)`
   执行 Bearer-only 预校验。
4. `resolve_auth_context(...)` 仅保留给 internal/test builder、
   受信服务链路以及离线测试装配。
5. 若测试需要注入自定义状态或 runtime，可额外提供
   `build_public_app_with_*` 变体，但公网二进制默认入口仍必须绑定
   `build_public_app()`。

## 4. 本轮落地

### 4.1 入口装配

以下服务已补齐 `build_public_app()`：

- `services/audit-service`
- `services/automation-service`
- `services/media-service`
- `services/ops-service`
- `services/notification-service`
- `services/conversation-runtime`
- `services/projection-service`

其中：

- `projection-service` 额外提供 `build_public_app_with_service(...)`
  以支持测试注入自定义 projection runtime

### 4.2 二进制启动入口

以下 `main.rs` 已全部切换为公网默认装配：

- `services/audit-service/src/main.rs`
- `services/automation-service/src/main.rs`
- `services/media-service/src/main.rs`
- `services/ops-service/src/main.rs`
- `services/notification-service/src/main.rs`
- `services/conversation-runtime/src/main.rs`
- `services/projection-service/src/main.rs`

### 4.3 回归测试

本轮新增公网认证回归测试：

- `services/audit-service/tests/public_auth_test.rs`
- `services/automation-service/tests/public_auth_test.rs`
- `services/media-service/tests/public_auth_test.rs`
- `services/ops-service/tests/public_auth_test.rs`
- `services/notification-service/tests/public_auth_test.rs`
- `services/conversation-runtime/tests/public_auth_test.rs`
- `services/projection-service/tests/public_auth_test.rs`

测试原则：

- public app 必须拒绝仅携带 trusted headers 的请求
- public app 必须接受合法 Bearer 请求

## 5. 验证结果

本轮执行并通过：

- `cargo test -p audit-service --test public_auth_test --offline`
- `cargo test -p automation-service --test public_auth_test --offline`
- `cargo test -p media-service --test public_auth_test --offline`
- `cargo test -p ops-service --test public_auth_test --offline`
- `cargo test -p notification-service --test public_auth_test --offline`
- `cargo test -p conversation-runtime --test public_auth_test --offline`
- `cargo test -p projection-service --test public_auth_test --offline`
- `cargo test -p audit-service --offline`
- `cargo test -p automation-service --offline`
- `cargo test -p media-service --offline`
- `cargo test -p ops-service --offline`
- `cargo test -p notification-service --offline`
- `cargo test -p conversation-runtime --offline`
- `cargo test -p projection-service --offline`
- `cargo fmt --all`
- `cargo test --workspace --offline`

## 6. 当前结论

截至本轮：

1. 工作区内所有可单独部署、面向外部的服务二进制，
   已全部从 `default builder` 切换到 `public builder`。
2. `trusted identity headers` 已被收敛为 internal/test 能力，
   不再作为公网默认认证来源。
3. 文档、部署入口、服务装配和测试契约已重新对齐。

## 7. 剩余工作

本轮完成的是“认证边界收口”，不是“授权模型完结”。
下一轮优先继续 review：

1. `audit-service` 与 `ops-service` 的“认证之后授权”标准，
   例如 admin/operator 角色或运维策略校验。
2. 是否将 `build_default_app()` 在语义上进一步重命名为
   `build_internal_app()`，降低后续误用概率。
3. 是否为部署脚本、容器镜像和 Helm 配置增加静态校验，
   防止未来重新绑定到 default/internal builder。
