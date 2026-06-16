# 社交服务数据库持久化实施计划

> **Goal:** 为 social-service 添加 PostgreSQL 持久化层，实现社交关系的数据库存储，支持海量并发和高性能

## 背景

- Migration 012 已定义 25 张表（社交关系、组织模型、消息互动、用户扩展）
- social-service 当前仅支持内存和文件持久化，无数据库支持
- 领域模型使用 String ID，数据库使用 BIGINT Snowflake ID
- 需要参考 postgres-journal 的成熟模式构建新的数据库适配器

## 实施步骤

### Step 1: 创建 social-postgres 适配器 crate

**文件:**
- Create: `adapters/social-postgres/Cargo.toml`
- Create: `adapters/social-postgres/src/lib.rs`
- Modify: `Cargo.toml` (workspace members)

**内容:**
```toml
[package]
name = "im-adapters-social-postgres"
edition.workspace = true
license.workspace = true
version.workspace = true

[dependencies]
chrono = "0.4"
im-domain-core = { path = "../../crates/im-domain-core" }
im-platform-contracts = { path = "../../crates/im-platform-contracts" }
postgres = { version = "0.19", features = ["with-chrono-0_4", "with-serde_json-1"] }
r2d2 = "0.8"
r2d2_postgres = "0.18"
serde_json.workspace = true
tokio.workspace = true
tracing.workspace = true
```

### Step 2: 实现好友请求持久化

**文件:**
- Create: `adapters/social-postgres/src/friend_request_store.rs`

**内容:**
- `PostgresFriendRequestStore` 结构体
- 实现 `FriendRequestStore` trait（CRUD 操作）
- SQL 常量定义
- 行映射函数
- 使用 r2d2 连接池

**关键方法:**
```rust
pub trait FriendRequestStore: Send + Sync {
    fn insert(&self, request: &FriendRequestRecord) -> Result<(), ContractError>;
    fn get_by_id(&self, tenant_id: &str, org_id: &str, request_id: i64) -> Result<Option<FriendRequestRecord>, ContractError>;
    fn list_by_requester(&self, tenant_id: &str, org_id: &str, requester_id: &str, status: &str, limit: i64) -> Result<Vec<FriendRequestRecord>, ContractError>;
    fn list_by_target(&self, tenant_id: &str, org_id: &str, target_id: &str, status: &str, limit: i64) -> Result<Vec<FriendRequestRecord>, ContractError>;
    fn update_status(&self, tenant_id: &str, org_id: &str, request_id: i64, status: &str) -> Result<(), ContractError>;
}
```

### Step 3: 实现好友关系持久化

**文件:**
- Create: `adapters/social-postgres/src/friendship_store.rs`

**内容:**
- `PostgresFriendshipStore` 结构体
- 实现 `FriendshipStore` trait
- 支持规范化 user_low_id/user_high_id 查询

### Step 4: 实现用户屏蔽持久化

**文件:**
- Create: `adapters/social-postgres/src/user_block_store.rs`

**内容:**
- `PostgresUserBlockStore` 结构体
- 实现 `UserBlockStore` trait
- 支持 scope 维度查询

### Step 5: 实现单聊会话持久化

**文件:**
- Create: `adapters/social-postgres/src/direct_chat_store.rs`

**内容:**
- `PostgresDirectChatStore` 结构体
- 实现 `DirectChatStore` trait
- 支持 pair_hash 唯一查询

### Step 6: 实现外部连接和成员链接持久化

**文件:**
- Create: `adapters/social-postgres/src/external_store.rs`

**内容:**
- `PostgresExternalConnectionStore` 和 `PostgresExternalMemberLinkStore`
- 支持连接和成员的 CRUD

### Step 7: 实现共享频道策略持久化

**文件:**
- Create: `adapters/social-postgres/src/shared_channel_store.rs`

**内容:**
- `PostgresSharedChannelPolicyStore` 结构体
- 支持策略版本管理和状态查询

### Step 8: 创建统一的 SocialPostgresAdapter

**文件:**
- Create: `adapters/social-postgres/src/adapter.rs`

**内容:**
- `SocialPostgresAdapter` 结构体，持有所有子 store
- 实现统一的初始化和连接池管理
- 参考 `PostgresJournalConfig` 的配置模式

### Step 9: 更新 social-service 集成数据库

**文件:**
- Modify: `services/social-service/Cargo.toml` (添加依赖)
- Modify: `services/social-service/src/runtime.rs` (添加 Database store 变体)

**内容:**
- 在 `SocialStateStore` 中添加 `Database` 变体
- 实现从数据库加载和保存状态的逻辑
- 保持与 Memory/File 模式的兼容性

### Step 10: 更新 workspace 配置

**文件:**
- Modify: `Cargo.toml` (添加 workspace 成员)

**内容:**
- 添加 `adapters/social-postgres` 到 workspace members

## 验证方案

1. 编译检查: `cargo check -p im-adapters-social-postgres`
2. 单元测试: `cargo test -p im-adapters-social-postgres`
3. 集成测试: 使用测试数据库运行完整的 CRUD 测试
4. 性能测试: 验证批量插入和查询性能

## 关键设计决策

1. **ID 转换**: 领域模型 String ID <-> 数据库 BIGINT 在适配器边界转换
2. **连接池**: 使用 r2d2，与 postgres-journal 保持一致
3. **错误处理**: 统一使用 `ContractError`
4. **SQL 模式**: 常量 SQL + 参数化查询，避免 SQL 注入
5. **事务**: 关键操作使用数据库事务保证一致性
