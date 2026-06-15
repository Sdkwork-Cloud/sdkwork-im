-- Migration 011: RTC Sessions, Signals, Audit, Notifications, Automations, Projections
-- 继续重建剩余表，引入 organization_id

-- ============================================================
-- 15. RTC 会话
-- ============================================================

DROP TABLE IF EXISTS im_rtc_sessions CASCADE;
CREATE TABLE im_rtc_sessions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    rtc_session_id TEXT NOT NULL,
    conversation_id TEXT,
    rtc_mode TEXT NOT NULL,
    initiator_principal_kind TEXT NOT NULL,
    initiator_principal_id TEXT NOT NULL,
    provider_plugin_id TEXT,
    provider_session_id TEXT,
    provider_region TEXT,
    access_endpoint TEXT,
    session_state TEXT NOT NULL,
    latest_signal_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_signal_seq >= 0),
    signaling_stream_id TEXT,
    artifact_message_id TEXT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_sessions PRIMARY KEY (tenant_id, organization_id, rtc_session_id),
    CONSTRAINT chk_im_rtc_sessions_state CHECK (session_state IN ('started', 'accepted', 'rejected', 'ended'))
);

CREATE INDEX idx_im_rtc_sessions_conversation
    ON im_rtc_sessions (tenant_id, organization_id, conversation_id, updated_at DESC)
    WHERE conversation_id IS NOT NULL;

CREATE INDEX idx_im_rtc_sessions_state
    ON im_rtc_sessions (tenant_id, organization_id, session_state, updated_at DESC, rtc_session_id);

CREATE INDEX idx_im_rtc_sessions_provider_session
    ON im_rtc_sessions (tenant_id, organization_id, provider_plugin_id, provider_session_id)
    WHERE provider_plugin_id IS NOT NULL AND provider_session_id IS NOT NULL;

CREATE INDEX idx_im_rtc_sessions_retention_until
    ON im_rtc_sessions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 16. RTC 信令
-- ============================================================

DROP TABLE IF EXISTS im_rtc_signals CASCADE;
CREATE TABLE im_rtc_signals (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    rtc_session_id TEXT NOT NULL,
    signal_seq BIGINT NOT NULL CHECK (signal_seq > 0),
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    receiver_principal_kind TEXT,
    receiver_principal_id TEXT,
    signal_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_signals PRIMARY KEY (tenant_id, organization_id, rtc_session_id, signal_seq)
);

CREATE INDEX idx_im_rtc_signals_session_seq
    ON im_rtc_signals (tenant_id, organization_id, rtc_session_id, signal_seq);

CREATE INDEX idx_im_rtc_signals_retention_until
    ON im_rtc_signals (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 17. 审计记录
-- ============================================================

DROP TABLE IF EXISTS im_audit_records CASCADE;
CREATE TABLE im_audit_records (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    audit_seq BIGINT NOT NULL CHECK (audit_seq > 0),
    audit_id TEXT NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    action TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    request_id TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    previous_hash TEXT,
    record_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_audit_records PRIMARY KEY (tenant_id, organization_id, audit_seq),
    CONSTRAINT uk_im_audit_records_id UNIQUE (tenant_id, organization_id, audit_id)
);

CREATE INDEX idx_im_audit_records_tenant_seq
    ON im_audit_records (tenant_id, organization_id, audit_seq);

CREATE INDEX idx_im_audit_records_target
    ON im_audit_records (tenant_id, organization_id, target_type, target_id, audit_seq);

CREATE INDEX idx_im_audit_records_retention_until
    ON im_audit_records (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 18. 通知任务
-- ============================================================

DROP TABLE IF EXISTS im_notification_tasks CASCADE;
CREATE TABLE im_notification_tasks (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    notification_id TEXT NOT NULL,
    source_event_id TEXT NOT NULL,
    source_event_type TEXT NOT NULL,
    category TEXT NOT NULL,
    channel TEXT NOT NULL,
    recipient_kind TEXT NOT NULL,
    recipient_id TEXT NOT NULL,
    notification_status TEXT NOT NULL DEFAULT 'requested',
    title TEXT,
    body TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    dispatched_at TIMESTAMPTZ,
    failure_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_notification_tasks PRIMARY KEY (tenant_id, organization_id, notification_id),
    CONSTRAINT uk_im_notification_tasks_source UNIQUE (tenant_id, organization_id, source_event_id, recipient_kind, recipient_id, category, channel),
    CONSTRAINT chk_im_notification_tasks_status CHECK (notification_status IN ('requested', 'dispatched', 'failed'))
);

CREATE INDEX idx_im_notification_tasks_recipient_updated
    ON im_notification_tasks (tenant_id, organization_id, recipient_kind, recipient_id, updated_at DESC, notification_id);

CREATE INDEX idx_im_notification_tasks_status
    ON im_notification_tasks (tenant_id, organization_id, notification_status, updated_at DESC);

CREATE INDEX idx_im_notification_tasks_retention_until
    ON im_notification_tasks (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 19. 自动化执行
-- ============================================================

DROP TABLE IF EXISTS im_automation_executions CASCADE;
CREATE TABLE im_automation_executions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    execution_id TEXT NOT NULL,
    trigger_type TEXT NOT NULL,
    target_kind TEXT NOT NULL,
    target_ref TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    input_payload_json JSONB,
    input_payload_hash TEXT,
    output_payload_json JSONB,
    output_payload_hash TEXT,
    execution_state TEXT NOT NULL DEFAULT 'requested',
    retry_count INTEGER NOT NULL DEFAULT 0 CHECK (retry_count >= 0),
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    failure_reason TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_automation_executions PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, execution_id),
    CONSTRAINT uk_im_automation_executions_request UNIQUE (tenant_id, organization_id, principal_kind, principal_id, execution_id, request_hash),
    CONSTRAINT chk_im_automation_executions_state CHECK (execution_state IN ('requested', 'running', 'succeeded', 'failed'))
);

CREATE INDEX idx_im_automation_executions_principal_updated
    ON im_automation_executions (tenant_id, organization_id, principal_kind, principal_id, updated_at DESC, execution_id);

CREATE INDEX idx_im_automation_executions_state
    ON im_automation_executions (tenant_id, organization_id, execution_state, updated_at DESC);

CREATE INDEX idx_im_automation_executions_retention_until
    ON im_automation_executions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 20. 投影：Timeline 条目
-- ============================================================

DROP TABLE IF EXISTS im_projection_timeline_entries CASCADE;
CREATE TABLE im_projection_timeline_entries (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    conversation_id TEXT NOT NULL,
    message_seq BIGINT NOT NULL CHECK (message_seq > 0),
    message_id BIGINT NOT NULL,
    summary TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_timeline_entries PRIMARY KEY (tenant_id, organization_id, conversation_id, message_seq),
    CONSTRAINT uk_im_projection_timeline_entries_message UNIQUE (tenant_id, organization_id, message_id)
);

CREATE INDEX idx_im_projection_timeline_entries_message
    ON im_projection_timeline_entries (tenant_id, organization_id, message_id);

CREATE INDEX idx_im_projection_timeline_entries_retention_until
    ON im_projection_timeline_entries (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 21. 投影：会话摘要
-- ============================================================

DROP TABLE IF EXISTS im_projection_conversation_summaries CASCADE;
CREATE TABLE im_projection_conversation_summaries (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    conversation_id TEXT NOT NULL,
    conversation_type TEXT,
    message_count BIGINT NOT NULL DEFAULT 0 CHECK (message_count >= 0),
    last_message_id BIGINT,
    last_message_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_message_seq >= 0),
    last_sender_kind TEXT,
    last_sender_id TEXT,
    last_summary TEXT,
    last_message_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    agent_handoff_json JSONB,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_conversation_summaries PRIMARY KEY (tenant_id, organization_id, conversation_id)
);

CREATE INDEX idx_im_projection_conversation_summaries_activity
    ON im_projection_conversation_summaries (tenant_id, organization_id, last_activity_at DESC, conversation_id);

CREATE INDEX idx_im_projection_conversation_summaries_retention_until
    ON im_projection_conversation_summaries (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 22. 投影：会话成员
-- ============================================================

DROP TABLE IF EXISTS im_projection_conversation_members CASCADE;
CREATE TABLE im_projection_conversation_members (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    conversation_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    member_id BIGINT NOT NULL,             -- Snowflake ID
    membership_role TEXT NOT NULL,
    membership_state TEXT NOT NULL,
    invited_by TEXT,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    removed_at TIMESTAMPTZ,
    attributes_json JSONB NOT NULL DEFAULT '{}'::JSONB,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_conversation_members PRIMARY KEY (tenant_id, organization_id, conversation_id, principal_kind, principal_id),
    CONSTRAINT uk_im_projection_conversation_members_member UNIQUE (tenant_id, organization_id, conversation_id, member_id),
    CONSTRAINT chk_im_projection_conversation_members_state CHECK (membership_state IN ('invited', 'joined', 'linked', 'removed', 'left'))
);

CREATE INDEX idx_im_projection_conversation_members_principal
    ON im_projection_conversation_members (tenant_id, organization_id, principal_kind, principal_id, membership_state, conversation_id);

CREATE INDEX idx_im_projection_conversation_members_active
    ON im_projection_conversation_members (tenant_id, organization_id, conversation_id, principal_kind, principal_id)
    WHERE membership_state = 'joined';

CREATE INDEX idx_im_projection_conversation_members_retention_until
    ON im_projection_conversation_members (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 23. 投影：已读游标
-- ============================================================

DROP TABLE IF EXISTS im_projection_read_cursors CASCADE;
CREATE TABLE im_projection_read_cursors (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    conversation_id TEXT NOT NULL,
    member_id BIGINT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    read_seq BIGINT NOT NULL DEFAULT 0 CHECK (read_seq >= 0),
    last_read_message_id BIGINT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_read_cursors PRIMARY KEY (tenant_id, organization_id, conversation_id, member_id)
);

CREATE INDEX idx_im_projection_read_cursors_principal
    ON im_projection_read_cursors (tenant_id, organization_id, principal_kind, principal_id, conversation_id);

CREATE INDEX idx_im_projection_read_cursors_retention_until
    ON im_projection_read_cursors (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 24. 投影：注册客户端路由
-- ============================================================

DROP TABLE IF EXISTS im_projection_registered_client_routes CASCADE;
CREATE TABLE im_projection_registered_client_routes (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    registered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_registered_client_routes PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id)
);

CREATE INDEX idx_im_projection_registered_client_routes_retention_until
    ON im_projection_registered_client_routes (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 25. 投影：客户端路由同步 Feed
-- ============================================================

DROP TABLE IF EXISTS im_projection_client_route_sync_feeds CASCADE;
CREATE TABLE im_projection_client_route_sync_feeds (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    sync_seq BIGINT NOT NULL CHECK (sync_seq > 0),
    origin_event_id TEXT NOT NULL,
    origin_event_type TEXT NOT NULL,
    conversation_id TEXT,
    message_id BIGINT,
    message_seq BIGINT CHECK (message_seq IS NULL OR message_seq > 0),
    member_id BIGINT,
    read_seq BIGINT CHECK (read_seq IS NULL OR read_seq >= 0),
    last_read_message_id BIGINT,
    actor_kind TEXT,
    actor_id TEXT,
    actor_device_id TEXT,
    summary TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_client_route_sync_feeds PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id, sync_seq)
);

CREATE INDEX idx_im_projection_client_route_sync_feeds_window
    ON im_projection_client_route_sync_feeds (tenant_id, organization_id, principal_kind, principal_id, device_id, sync_seq);

CREATE INDEX idx_im_projection_client_route_sync_feeds_conversation
    ON im_projection_client_route_sync_feeds (tenant_id, organization_id, conversation_id, sync_seq)
    WHERE conversation_id IS NOT NULL;

CREATE INDEX idx_im_projection_client_route_sync_feeds_retention_until
    ON im_projection_client_route_sync_feeds (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 26. 投影：客户端路由同步检查点
-- ============================================================

DROP TABLE IF EXISTS im_projection_client_route_sync_checkpoints CASCADE;
CREATE TABLE im_projection_client_route_sync_checkpoints (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_sync_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_sync_seq >= 0),
    trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_client_route_sync_checkpoints PRIMARY KEY (tenant_id, organization_id, principal_kind, principal_id, device_id),
    CONSTRAINT chk_im_projection_client_route_sync_checkpoints_order CHECK (trimmed_through_seq <= latest_sync_seq)
);

CREATE INDEX idx_im_projection_client_route_sync_checkpoints_retention_until
    ON im_projection_client_route_sync_checkpoints (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 27. 投影：联系人
-- ============================================================

DROP TABLE IF EXISTS im_projection_contacts CASCADE;
CREATE TABLE im_projection_contacts (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    owner_user_id TEXT NOT NULL,
    contact_type TEXT NOT NULL,
    target_user_id TEXT NOT NULL,
    relationship_state TEXT NOT NULL,
    friendship_id TEXT NOT NULL,
    direct_chat_id TEXT,
    conversation_id TEXT,
    established_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_interaction_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_contacts PRIMARY KEY (tenant_id, organization_id, owner_user_id, contact_type, target_user_id)
);

CREATE INDEX idx_im_projection_contacts_owner_activity
    ON im_projection_contacts (tenant_id, organization_id, owner_user_id, last_interaction_at DESC, target_user_id);

CREATE INDEX idx_im_projection_contacts_retention_until
    ON im_projection_contacts (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 28. 投影：直接聊天绑定
-- ============================================================

DROP TABLE IF EXISTS im_projection_direct_chat_bindings CASCADE;
CREATE TABLE im_projection_direct_chat_bindings (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    direct_chat_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    direct_chat_status TEXT NOT NULL DEFAULT 'active',
    bound_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_direct_chat_bindings PRIMARY KEY (tenant_id, organization_id, direct_chat_id),
    CONSTRAINT uk_im_projection_direct_chat_bindings_conversation UNIQUE (tenant_id, organization_id, conversation_id),
    CONSTRAINT chk_im_projection_direct_chat_bindings_status CHECK (direct_chat_status IN ('active', 'archived'))
);

CREATE INDEX idx_im_projection_direct_chat_bindings_conversation
    ON im_projection_direct_chat_bindings (tenant_id, organization_id, conversation_id, direct_chat_status);

CREATE INDEX idx_im_projection_direct_chat_bindings_retention_until
    ON im_projection_direct_chat_bindings (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 29. Stream Sessions
-- ============================================================

DROP TABLE IF EXISTS im_stream_sessions CASCADE;
CREATE TABLE im_stream_sessions (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    stream_id TEXT NOT NULL,
    owner_principal_kind TEXT NOT NULL,
    owner_principal_id TEXT NOT NULL,
    stream_type TEXT NOT NULL,
    scope_kind TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    durability_class TEXT NOT NULL,
    ordering_scope TEXT NOT NULL,
    schema_ref TEXT,
    stream_state TEXT NOT NULL DEFAULT 'created',
    last_frame_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_frame_seq >= 0),
    last_checkpoint_seq BIGINT CHECK (last_checkpoint_seq >= 0),
    result_message_id BIGINT,
    complete_frame_seq BIGINT CHECK (complete_frame_seq >= 0),
    abort_frame_seq BIGINT CHECK (abort_frame_seq >= 0),
    abort_reason TEXT,
    opened_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_stream_sessions PRIMARY KEY (tenant_id, organization_id, stream_id),
    CONSTRAINT chk_im_stream_sessions_state CHECK (stream_state IN ('created', 'opened', 'active', 'checkpointed', 'completed', 'aborted', 'expired')),
    CONSTRAINT chk_im_stream_sessions_seq_order CHECK (
        COALESCE(last_checkpoint_seq, 0) <= last_frame_seq
        AND COALESCE(complete_frame_seq, 0) <= last_frame_seq
        AND COALESCE(abort_frame_seq, 0) <= last_frame_seq
    )
);

CREATE INDEX idx_im_stream_sessions_scope
    ON im_stream_sessions (tenant_id, organization_id, scope_kind, scope_id, updated_at DESC);

CREATE INDEX idx_im_stream_sessions_updated
    ON im_stream_sessions (tenant_id, organization_id, updated_at DESC, stream_id);

CREATE INDEX idx_im_stream_sessions_retention_until
    ON im_stream_sessions (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 30. Stream Frames
-- ============================================================

DROP TABLE IF EXISTS im_stream_frames CASCADE;
CREATE TABLE im_stream_frames (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    stream_id TEXT NOT NULL,
    frame_seq BIGINT NOT NULL CHECK (frame_seq > 0),
    producer_principal_kind TEXT NOT NULL,
    producer_principal_id TEXT NOT NULL,
    schema_ref TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_stream_frames PRIMARY KEY (tenant_id, organization_id, stream_id, frame_seq)
);

CREATE INDEX idx_im_stream_frames_stream_seq
    ON im_stream_frames (tenant_id, organization_id, stream_id, frame_seq);

CREATE INDEX idx_im_stream_frames_retention_until
    ON im_stream_frames (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;
