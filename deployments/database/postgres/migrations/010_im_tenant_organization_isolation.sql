-- Migration 010: Tenant + Organization Dual Isolation
-- 为所有 im_* 业务表引入 organization_id，实现租户+组织双重隔离
-- 新应用零用户，直接重建终态 schema，不保留 001 迁移的兼容性

-- ============================================================
-- 核心设计决策：
-- 1. organization_id 为 TEXT NOT NULL DEFAULT 'default'
-- 2. 主键与索引统一前置 (tenant_id, organization_id, ...)
-- 3. 所有查询强制携带 organization_id 过滤
-- ============================================================

-- ============================================================
-- 1. 消息真值层
-- ============================================================

-- 重建 im_conversation_messages（消息真值表）
-- 主键改为 Snowflake message_id，但保留 message_seq 作为会话内序号
DROP TABLE IF EXISTS im_conversation_messages CASCADE;
CREATE TABLE im_conversation_messages (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    conversation_id     TEXT NOT NULL,
    message_id          BIGINT NOT NULL,           -- Snowflake ID，全局唯一
    message_seq         BIGINT NOT NULL,           -- 会话内严格递增
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    sender_device_id    TEXT,
    client_msg_id       TEXT,
    message_type        TEXT NOT NULL,
    payload_json        JSONB NOT NULL,
    payload_hash        TEXT NOT NULL,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ,
    retention_until     TIMESTAMPTZ,
    CONSTRAINT pk_im_conversation_messages PRIMARY KEY (tenant_id, organization_id, conversation_id, message_seq),
    CONSTRAINT uk_im_conversation_messages_id UNIQUE (tenant_id, message_id),
    CONSTRAINT chk_im_conversation_messages_seq CHECK (message_seq > 0)
);

-- 客户端幂等键（会话 + 发送者 + client_msg_id 唯一）
CREATE UNIQUE INDEX uk_im_conversation_messages_client
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, sender_principal_kind, sender_principal_id, client_msg_id)
    WHERE client_msg_id IS NOT NULL;

-- timeline 读取索引
CREATE INDEX idx_im_messages_tenant_conv_seq
    ON im_conversation_messages (tenant_id, organization_id, conversation_id, message_seq DESC);

-- 发送者消息索引
CREATE INDEX idx_im_messages_sender_created
    ON im_conversation_messages (tenant_id, organization_id, sender_principal_kind, sender_principal_id, created_at DESC);

-- retention 索引
CREATE INDEX idx_im_conversation_messages_retention_until
    ON im_conversation_messages (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 2. 消息序号分配器（会话级原子）
-- ============================================================

CREATE TABLE im_conversation_seq_counters (
    tenant_id       TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    conversation_id TEXT NOT NULL,
    next_seq        BIGINT NOT NULL DEFAULT 1,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_conversation_seq_counters PRIMARY KEY (tenant_id, organization_id, conversation_id),
    CONSTRAINT chk_im_conversation_seq_counters_seq CHECK (next_seq > 0)
);

-- ============================================================
-- 3. 消息媒体引用
-- ============================================================

DROP TABLE IF EXISTS im_message_media_refs CASCADE;
CREATE TABLE im_message_media_refs (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    conversation_id TEXT NOT NULL,
    message_seq BIGINT NOT NULL,
    message_id BIGINT NOT NULL,
    part_index INTEGER NOT NULL CHECK (part_index >= 0),
    media_role TEXT NOT NULL,
    drive_space_id TEXT NOT NULL,
    drive_node_id TEXT NOT NULL,
    drive_uri TEXT NOT NULL,
    drive_node_version TEXT,
    media_kind TEXT NOT NULL,
    media_source TEXT NOT NULL,
    mime_type TEXT,
    size_bytes TEXT,
    checksum_algorithm TEXT,
    checksum_value TEXT,
    object_blob_id TEXT,
    media_resource_snapshot JSONB NOT NULL,
    resource_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_message_media_refs PRIMARY KEY (tenant_id, organization_id, conversation_id, message_seq, part_index),
    CONSTRAINT uk_im_message_media_refs_message_part UNIQUE (tenant_id, message_id, part_index),
    CONSTRAINT fk_im_message_media_refs_message FOREIGN KEY (tenant_id, organization_id, conversation_id, message_seq)
        REFERENCES im_conversation_messages (tenant_id, organization_id, conversation_id, message_seq)
        ON DELETE CASCADE,
    CONSTRAINT chk_im_message_media_refs_drive_uri CHECK (
        drive_uri = ('drive://spaces/' || drive_space_id || '/nodes/' || drive_node_id)
    ),
    CONSTRAINT chk_im_message_media_refs_media_source CHECK (
        media_source IN ('drive', 'external_url', 'data_url', 'provider_asset', 'generated')
    ),
    CONSTRAINT chk_im_message_media_refs_size_bytes CHECK (
        size_bytes IS NULL OR size_bytes ~ '^[0-9]+$'
    )
);

CREATE INDEX idx_im_message_media_refs_drive_node
    ON im_message_media_refs (tenant_id, organization_id, drive_space_id, drive_node_id, message_seq DESC);

CREATE INDEX idx_im_message_media_refs_role
    ON im_message_media_refs (tenant_id, organization_id, conversation_id, media_role, message_seq DESC, part_index);

CREATE INDEX idx_im_message_media_refs_retention_until
    ON im_message_media_refs (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 4. Outbox 事件表（重建，支持 FOR UPDATE SKIP LOCKED）
-- ============================================================

DROP TABLE IF EXISTS im_outbox_events CASCADE;
CREATE TABLE im_outbox_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    outbox_id TEXT NOT NULL,              -- Snowflake ID
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    publish_status TEXT NOT NULL DEFAULT 'pending',
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_outbox_events PRIMARY KEY (tenant_id, organization_id, outbox_id),
    CONSTRAINT uk_im_outbox_events_event UNIQUE (tenant_id, organization_id, event_id),
    CONSTRAINT chk_im_outbox_events_publish_status CHECK (publish_status IN ('pending', 'published', 'failed'))
);

-- relay worker 用索引：FOR UPDATE SKIP LOCKED
CREATE INDEX idx_im_outbox_events_status_available
    ON im_outbox_events (tenant_id, organization_id, publish_status, available_at, outbox_id);

CREATE INDEX idx_im_outbox_events_retention_until
    ON im_outbox_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 5. Inbox 事件表（消费幂等）
-- ============================================================

DROP TABLE IF EXISTS im_inbox_events CASCADE;
CREATE TABLE im_inbox_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    inbox_id TEXT NOT NULL,
    source_system TEXT NOT NULL,
    source_event_id TEXT NOT NULL,
    consumer_name TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    process_status TEXT NOT NULL DEFAULT 'pending',
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_inbox_events PRIMARY KEY (tenant_id, organization_id, inbox_id),
    CONSTRAINT uk_im_inbox_events_source UNIQUE (tenant_id, organization_id, source_system, source_event_id),
    CONSTRAINT chk_im_inbox_events_process_status CHECK (process_status IN ('pending', 'processed', 'failed'))
);

CREATE INDEX idx_im_inbox_events_status_received
    ON im_inbox_events (tenant_id, organization_id, consumer_name, process_status, received_at, inbox_id);

CREATE INDEX idx_im_inbox_events_retention_until
    ON im_inbox_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 6. Commit Journal（重建，offset 独立于 aggregate_seq）
-- ============================================================

DROP TABLE IF EXISTS im_commit_journal CASCADE;
CREATE TABLE im_commit_journal (
    partition_key TEXT NOT NULL,           -- (tenant_id:organization_id:aggregate_type:aggregate_id)
    commit_offset BIGINT NOT NULL,         -- Snowflake ID，全局唯一，非业务序号
    event_id TEXT NOT NULL,                -- Snowflake ID
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    aggregate_seq BIGINT NOT NULL CHECK (aggregate_seq > 0),  -- 业务聚合版本号
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    idempotency_key TEXT,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_commit_journal PRIMARY KEY (partition_key, commit_offset),
    CONSTRAINT uk_im_commit_journal_event UNIQUE (event_id)
);

CREATE INDEX idx_im_commit_journal_tenant_aggregate_seq
    ON im_commit_journal (tenant_id, organization_id, aggregate_type, aggregate_id, aggregate_seq);

CREATE INDEX idx_im_commit_journal_tenant_occurred
    ON im_commit_journal (tenant_id, organization_id, occurred_at, event_id);

CREATE INDEX idx_im_commit_journal_retention_until
    ON im_commit_journal (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 7. 幂等键表
-- ============================================================

DROP TABLE IF EXISTS im_idempotency_keys CASCADE;
CREATE TABLE im_idempotency_keys (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    request_scope TEXT NOT NULL,
    idempotency_key TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    response_json JSONB NOT NULL,
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_idempotency_keys PRIMARY KEY (tenant_id, organization_id, request_scope, idempotency_key)
);

CREATE INDEX idx_im_idempotency_keys_expires
    ON im_idempotency_keys (tenant_id, organization_id, expires_at);

-- ============================================================
-- 8. 实时设备事件
-- ============================================================

DROP TABLE IF EXISTS im_realtime_device_events CASCADE;
CREATE TABLE im_realtime_device_events (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    client_route_scope_key TEXT NOT NULL,
    realtime_seq BIGINT NOT NULL CHECK (realtime_seq > 0),
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    scope_type TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    delivery_class TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_device_events PRIMARY KEY (tenant_id, organization_id, client_route_scope_key, realtime_seq)
);

CREATE INDEX idx_im_realtime_device_events_scope_seq
    ON im_realtime_device_events (tenant_id, organization_id, client_route_scope_key, realtime_seq);

CREATE INDEX idx_im_realtime_device_events_scope_fanout
    ON im_realtime_device_events (tenant_id, organization_id, scope_type, scope_id, event_type, realtime_seq);

CREATE INDEX idx_im_realtime_device_events_retention_until
    ON im_realtime_device_events (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 9. 实时检查点
-- ============================================================

DROP TABLE IF EXISTS im_realtime_checkpoints CASCADE;
CREATE TABLE im_realtime_checkpoints (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    client_route_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_realtime_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_realtime_seq >= 0),
    acked_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (acked_through_seq >= 0),
    trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    capacity_trimmed_event_count BIGINT NOT NULL DEFAULT 0 CHECK (capacity_trimmed_event_count >= 0),
    capacity_trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (capacity_trimmed_through_seq >= 0),
    last_capacity_trimmed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_realtime_checkpoints PRIMARY KEY (tenant_id, organization_id, client_route_scope_key),
    CONSTRAINT chk_im_realtime_checkpoints_order CHECK (
        acked_through_seq <= latest_realtime_seq
        AND trimmed_through_seq <= latest_realtime_seq
        AND capacity_trimmed_through_seq <= trimmed_through_seq
    ),
    CONSTRAINT chk_im_realtime_checkpoints_capacity_trim_meta CHECK (
        (
            capacity_trimmed_event_count = 0
            AND capacity_trimmed_through_seq = 0
            AND last_capacity_trimmed_at IS NULL
        )
        OR (
            capacity_trimmed_event_count > 0
            AND capacity_trimmed_through_seq > 0
            AND last_capacity_trimmed_at IS NOT NULL
        )
    )
);

CREATE INDEX idx_im_realtime_checkpoints_capacity_trimmed
    ON im_realtime_checkpoints (
        tenant_id,
        organization_id,
        last_capacity_trimmed_at DESC,
        capacity_trimmed_through_seq DESC,
        client_route_scope_key
    )
    WHERE capacity_trimmed_event_count > 0;

-- ============================================================
-- 10. 实时订阅
-- ============================================================

DROP TABLE IF EXISTS im_realtime_subscriptions CASCADE;
CREATE TABLE im_realtime_subscriptions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    client_route_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    subscriptions_json JSONB NOT NULL,
    subscription_count INTEGER NOT NULL DEFAULT 0 CHECK (subscription_count >= 0),
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_subscriptions PRIMARY KEY (tenant_id, organization_id, client_route_scope_key)
);

CREATE INDEX idx_im_realtime_subscriptions_principal
    ON im_realtime_subscriptions (tenant_id, organization_id, principal_kind, principal_id, device_id);

CREATE INDEX idx_im_realtime_subscriptions_synced_at
    ON im_realtime_subscriptions (tenant_id, organization_id, client_route_scope_key, synced_at);

CREATE INDEX idx_im_realtime_subscriptions_items_gin
    ON im_realtime_subscriptions USING GIN (subscriptions_json);

CREATE INDEX idx_im_realtime_subscriptions_retention_until
    ON im_realtime_subscriptions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 11. 实时订阅范围
-- ============================================================

DROP TABLE IF EXISTS im_realtime_subscription_scopes CASCADE;
CREATE TABLE im_realtime_subscription_scopes (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    scope_type TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    event_type TEXT NOT NULL DEFAULT '*',
    client_route_scope_key TEXT NOT NULL,
    device_id TEXT NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_realtime_subscription_scopes PRIMARY KEY (
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        scope_type,
        scope_id,
        event_type,
        client_route_scope_key
    ),
    CONSTRAINT fk_im_realtime_subscription_scopes_device
        FOREIGN KEY (tenant_id, organization_id, client_route_scope_key)
        REFERENCES im_realtime_subscriptions (tenant_id, organization_id, client_route_scope_key)
        ON DELETE CASCADE
);

CREATE INDEX idx_im_realtime_subscription_scopes_fanout
    ON im_realtime_subscription_scopes (
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        scope_type,
        scope_id,
        event_type,
        device_id
    );

CREATE INDEX idx_im_realtime_subscription_scopes_device
    ON im_realtime_subscription_scopes (tenant_id, organization_id, client_route_scope_key, synced_at);

-- ============================================================
-- 12. Presence 状态
-- ============================================================

DROP TABLE IF EXISTS im_presence_states CASCADE;
CREATE TABLE im_presence_states (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    session_id TEXT,
    presence_status TEXT NOT NULL,
    last_sync_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_sync_seq >= 0),
    last_resume_at TIMESTAMPTZ,
    last_seen_at TIMESTAMPTZ,
    resume_required BOOLEAN NOT NULL DEFAULT FALSE,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_presence_states PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_presence_states_status CHECK (presence_status IN ('online', 'offline'))
);

CREATE INDEX idx_im_presence_states_principal
    ON im_presence_states (tenant_id, organization_id, principal_kind, principal_id, device_id);

CREATE INDEX idx_im_presence_states_online_seen_at
    ON im_presence_states (
        last_seen_at,
        tenant_id,
        organization_id,
        principal_kind,
        principal_id,
        device_id
    )
    WHERE presence_status = 'online' AND last_seen_at IS NOT NULL;

CREATE INDEX idx_im_presence_states_retention_until
    ON im_presence_states (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 13. 路由绑定
-- ============================================================

DROP TABLE IF EXISTS im_route_bindings CASCADE;
CREATE TABLE im_route_bindings (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    owner_node_id TEXT NOT NULL,
    session_id TEXT,
    connection_kind TEXT NOT NULL,
    route_epoch BIGINT NOT NULL CHECK (route_epoch > 0),
    bound_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_route_bindings PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_route_bindings_connection_kind CHECK (connection_kind IN ('websocket', 'http'))
);

CREATE INDEX idx_im_route_bindings_owner_node
    ON im_route_bindings (owner_node_id, tenant_id, organization_id, principal_kind, principal_id, device_id);

-- ============================================================
-- 14. 断线围栏
-- ============================================================

DROP TABLE IF EXISTS im_realtime_disconnect_fences CASCADE;
CREATE TABLE im_realtime_disconnect_fences (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    session_id TEXT,
    owner_node_id TEXT NOT NULL,
    disconnected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    fence_token TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_disconnect_fences PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id),
    CONSTRAINT uk_im_realtime_disconnect_fences_token UNIQUE (tenant_id, organization_id, fence_token)
);

CREATE INDEX idx_im_realtime_disconnect_fences_disconnected_at
    ON im_realtime_disconnect_fences (tenant_id, organization_id, disconnected_at, principal_kind, principal_id, device_id);

CREATE INDEX idx_im_realtime_disconnect_fences_retention_until
    ON im_realtime_disconnect_fences (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;
