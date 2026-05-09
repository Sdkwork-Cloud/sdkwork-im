-- Core IM PostgreSQL schema.
-- This migration is intentionally append-first and cursor-indexed. It defines
-- the durable contracts that the Rust in-memory/local stores must preserve.

CREATE TABLE IF NOT EXISTS im_commit_journal (
    partition_key TEXT NOT NULL,
    commit_offset BIGINT NOT NULL CHECK (commit_offset > 0),
    event_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    aggregate_seq BIGINT NOT NULL CHECK (aggregate_seq > 0),
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    idempotency_key TEXT,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_commit_journal PRIMARY KEY (partition_key, commit_offset),
    CONSTRAINT uk_im_commit_journal_event UNIQUE (event_id)
);

CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_aggregate_seq
    ON im_commit_journal (tenant_id, aggregate_type, aggregate_id, aggregate_seq);

CREATE INDEX IF NOT EXISTS idx_im_commit_journal_tenant_occurred
    ON im_commit_journal (tenant_id, occurred_at, event_id);

CREATE TABLE IF NOT EXISTS im_outbox_events (
    tenant_id TEXT NOT NULL,
    outbox_id TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    publish_status TEXT NOT NULL,
    attempt_count INTEGER NOT NULL DEFAULT 0 CHECK (attempt_count >= 0),
    available_at TIMESTAMPTZ NOT NULL,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_outbox_events PRIMARY KEY (tenant_id, outbox_id),
    CONSTRAINT uk_im_outbox_events_event UNIQUE (tenant_id, event_id)
);

CREATE INDEX IF NOT EXISTS idx_im_outbox_events_status_available
    ON im_outbox_events (tenant_id, publish_status, available_at, outbox_id);

CREATE TABLE IF NOT EXISTS im_inbox_events (
    tenant_id TEXT NOT NULL,
    inbox_id TEXT NOT NULL,
    source_system TEXT NOT NULL,
    source_event_id TEXT NOT NULL,
    consumer_name TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    process_status TEXT NOT NULL,
    received_at TIMESTAMPTZ NOT NULL,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_inbox_events PRIMARY KEY (tenant_id, inbox_id),
    CONSTRAINT uk_im_inbox_events_source UNIQUE (tenant_id, source_system, source_event_id)
);

CREATE INDEX IF NOT EXISTS idx_im_inbox_events_status_received
    ON im_inbox_events (tenant_id, consumer_name, process_status, received_at, inbox_id);

CREATE TABLE IF NOT EXISTS im_idempotency_keys (
    tenant_id TEXT NOT NULL,
    request_scope TEXT NOT NULL,
    idempotency_key TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    response_json JSONB NOT NULL,
    first_seen_at TIMESTAMPTZ NOT NULL,
    last_seen_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT pk_im_idempotency_keys PRIMARY KEY (tenant_id, request_scope, idempotency_key),
    CONSTRAINT uk_im_idempotency_keys_scope UNIQUE (tenant_id, request_scope, idempotency_key)
);

CREATE INDEX IF NOT EXISTS idx_im_idempotency_keys_expires
    ON im_idempotency_keys (expires_at);

CREATE TABLE IF NOT EXISTS im_conversation_messages (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    message_seq BIGINT NOT NULL CHECK (message_seq > 0),
    message_id TEXT NOT NULL,
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    sender_device_id TEXT,
    client_msg_id TEXT,
    message_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    deleted_at TIMESTAMPTZ,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_conversation_messages PRIMARY KEY (tenant_id, conversation_id, message_seq),
    CONSTRAINT uk_im_conversation_messages_message UNIQUE (tenant_id, message_id)
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_im_conversation_messages_client
    ON im_conversation_messages (tenant_id, conversation_id, sender_principal_kind, sender_principal_id, client_msg_id)
    WHERE client_msg_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_messages_tenant_conversation_seq
    ON im_conversation_messages (tenant_id, conversation_id, message_seq DESC);

CREATE INDEX IF NOT EXISTS idx_im_messages_sender_created
    ON im_conversation_messages (tenant_id, sender_principal_kind, sender_principal_id, created_at DESC);

CREATE TABLE IF NOT EXISTS im_realtime_device_events (
    tenant_id TEXT NOT NULL,
    device_scope_key TEXT NOT NULL,
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
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_device_events PRIMARY KEY (tenant_id, device_scope_key, realtime_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_device_events_scope_seq
    ON im_realtime_device_events (tenant_id, device_scope_key, realtime_seq);

CREATE INDEX IF NOT EXISTS idx_im_realtime_device_events_scope_fanout
    ON im_realtime_device_events (tenant_id, scope_type, scope_id, event_type, realtime_seq);

CREATE TABLE IF NOT EXISTS im_realtime_checkpoints (
    tenant_id TEXT NOT NULL,
    device_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_realtime_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_realtime_seq >= 0),
    acked_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (acked_through_seq >= 0),
    trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    capacity_trimmed_event_count BIGINT NOT NULL DEFAULT 0 CHECK (capacity_trimmed_event_count >= 0),
    capacity_trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (capacity_trimmed_through_seq >= 0),
    last_capacity_trimmed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT pk_im_realtime_checkpoints PRIMARY KEY (tenant_id, device_scope_key),
    CONSTRAINT ck_im_realtime_checkpoints_order CHECK (
        acked_through_seq <= latest_realtime_seq
        AND trimmed_through_seq <= latest_realtime_seq
        AND capacity_trimmed_through_seq <= trimmed_through_seq
    ),
    CONSTRAINT ck_im_realtime_checkpoints_capacity_trim_meta CHECK (
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

CREATE INDEX IF NOT EXISTS idx_im_realtime_checkpoints_capacity_trimmed
    ON im_realtime_checkpoints (
        tenant_id,
        last_capacity_trimmed_at DESC,
        capacity_trimmed_through_seq DESC,
        device_scope_key
    )
    WHERE capacity_trimmed_event_count > 0;

CREATE TABLE IF NOT EXISTS im_realtime_subscriptions (
    tenant_id TEXT NOT NULL,
    device_scope_key TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    subscriptions_json JSONB NOT NULL,
    subscription_count INTEGER NOT NULL DEFAULT 0 CHECK (subscription_count >= 0),
    synced_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_subscriptions PRIMARY KEY (tenant_id, device_scope_key)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_principal
    ON im_realtime_subscriptions (tenant_id, principal_kind, principal_id, device_id);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_synced_at
    ON im_realtime_subscriptions (tenant_id, device_scope_key, synced_at);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscriptions_items_gin
    ON im_realtime_subscriptions USING GIN (subscriptions_json);

CREATE TABLE IF NOT EXISTS im_realtime_subscription_scopes (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    scope_type TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    event_type TEXT NOT NULL DEFAULT '*',
    device_scope_key TEXT NOT NULL,
    device_id TEXT NOT NULL,
    synced_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT pk_im_realtime_subscription_scopes PRIMARY KEY (
        tenant_id,
        principal_kind,
        principal_id,
        scope_type,
        scope_id,
        event_type,
        device_scope_key
    ),
    CONSTRAINT fk_im_realtime_subscription_scopes_device FOREIGN KEY (tenant_id, device_scope_key)
        REFERENCES im_realtime_subscriptions (tenant_id, device_scope_key)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscription_scopes_fanout
    ON im_realtime_subscription_scopes (
        tenant_id,
        principal_kind,
        principal_id,
        scope_type,
        scope_id,
        event_type,
        device_id
    );

CREATE INDEX IF NOT EXISTS idx_im_realtime_subscription_scopes_device
    ON im_realtime_subscription_scopes (tenant_id, device_scope_key, synced_at);

CREATE TABLE IF NOT EXISTS im_presence_states (
    tenant_id TEXT NOT NULL,
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
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_presence_states PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id)
);

CREATE INDEX IF NOT EXISTS idx_im_presence_states_principal
    ON im_presence_states (tenant_id, principal_kind, principal_id, device_id);

CREATE INDEX IF NOT EXISTS idx_im_presence_states_online_seen_at
    ON im_presence_states (
        last_seen_at,
        tenant_id,
        principal_kind,
        principal_id,
        device_id
    )
    WHERE presence_status = 'online' AND last_seen_at IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_route_bindings (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    owner_node_id TEXT NOT NULL,
    session_id TEXT,
    connection_kind TEXT NOT NULL,
    route_epoch BIGINT NOT NULL CHECK (route_epoch > 0),
    bound_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    CONSTRAINT pk_im_route_bindings PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id)
);

CREATE INDEX IF NOT EXISTS idx_im_route_bindings_owner_node
    ON im_route_bindings (owner_node_id, tenant_id, principal_kind, principal_id, device_id);

CREATE TABLE IF NOT EXISTS im_realtime_disconnect_fences (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    session_id TEXT,
    owner_node_id TEXT NOT NULL,
    disconnected_at TIMESTAMPTZ NOT NULL,
    fence_token TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_realtime_disconnect_fences PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id),
    CONSTRAINT uk_im_realtime_disconnect_fences_token UNIQUE (tenant_id, fence_token)
);

CREATE INDEX IF NOT EXISTS idx_im_realtime_disconnect_fences_disconnected_at
    ON im_realtime_disconnect_fences (tenant_id, disconnected_at, principal_kind, principal_id, device_id);

CREATE TABLE IF NOT EXISTS im_rtc_sessions (
    tenant_id TEXT NOT NULL,
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
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_sessions PRIMARY KEY (tenant_id, rtc_session_id),
    CONSTRAINT ck_im_rtc_sessions_state CHECK (session_state IN ('started', 'accepted', 'rejected', 'ended'))
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_conversation
    ON im_rtc_sessions (tenant_id, conversation_id, updated_at DESC)
    WHERE conversation_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_state
    ON im_rtc_sessions (tenant_id, session_state, updated_at DESC, rtc_session_id);

CREATE INDEX IF NOT EXISTS idx_im_rtc_sessions_provider_session
    ON im_rtc_sessions (tenant_id, provider_plugin_id, provider_session_id)
    WHERE provider_plugin_id IS NOT NULL AND provider_session_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_rtc_signals (
    tenant_id TEXT NOT NULL,
    rtc_session_id TEXT NOT NULL,
    signal_seq BIGINT NOT NULL CHECK (signal_seq > 0),
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    receiver_principal_kind TEXT,
    receiver_principal_id TEXT,
    signal_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_rtc_signals PRIMARY KEY (tenant_id, rtc_session_id, signal_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_rtc_signals_session_seq
    ON im_rtc_signals (tenant_id, rtc_session_id, signal_seq);

CREATE TABLE IF NOT EXISTS im_audit_records (
    tenant_id TEXT NOT NULL,
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
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_audit_records PRIMARY KEY (tenant_id, audit_seq),
    CONSTRAINT uk_im_audit_records_id UNIQUE (tenant_id, audit_id)
);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_tenant_seq
    ON im_audit_records (tenant_id, audit_seq);

CREATE INDEX IF NOT EXISTS idx_im_audit_records_target
    ON im_audit_records (tenant_id, target_type, target_id, audit_seq);

CREATE TABLE IF NOT EXISTS im_notification_tasks (
    tenant_id TEXT NOT NULL,
    notification_id TEXT NOT NULL,
    source_event_id TEXT NOT NULL,
    source_event_type TEXT NOT NULL,
    category TEXT NOT NULL,
    channel TEXT NOT NULL,
    recipient_kind TEXT NOT NULL,
    recipient_id TEXT NOT NULL,
    notification_status TEXT NOT NULL,
    title TEXT,
    body TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    requested_at TIMESTAMPTZ NOT NULL,
    dispatched_at TIMESTAMPTZ,
    failure_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_notification_tasks PRIMARY KEY (tenant_id, notification_id),
    CONSTRAINT uk_im_notification_tasks_source UNIQUE (tenant_id, source_event_id, recipient_kind, recipient_id, category, channel),
    CONSTRAINT ck_im_notification_tasks_status CHECK (notification_status IN ('requested', 'dispatched', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_notification_tasks_recipient_updated
    ON im_notification_tasks (tenant_id, recipient_kind, recipient_id, updated_at DESC, notification_id);

CREATE INDEX IF NOT EXISTS idx_im_notification_tasks_status
    ON im_notification_tasks (tenant_id, notification_status, updated_at DESC);

CREATE TABLE IF NOT EXISTS im_automation_executions (
    tenant_id TEXT NOT NULL,
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
    execution_state TEXT NOT NULL,
    retry_count INTEGER NOT NULL DEFAULT 0 CHECK (retry_count >= 0),
    requested_at TIMESTAMPTZ NOT NULL,
    completed_at TIMESTAMPTZ,
    failure_reason TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_automation_executions PRIMARY KEY (tenant_id, principal_kind, principal_id, execution_id),
    CONSTRAINT uk_im_automation_executions_request UNIQUE (tenant_id, principal_kind, principal_id, execution_id, request_hash),
    CONSTRAINT ck_im_automation_executions_state CHECK (execution_state IN ('requested', 'running', 'succeeded', 'failed'))
);

CREATE INDEX IF NOT EXISTS idx_im_automation_executions_principal_updated
    ON im_automation_executions (tenant_id, principal_kind, principal_id, updated_at DESC, execution_id);

CREATE INDEX IF NOT EXISTS idx_im_automation_executions_state
    ON im_automation_executions (tenant_id, execution_state, updated_at DESC);

CREATE TABLE IF NOT EXISTS im_projection_timeline_entries (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    message_seq BIGINT NOT NULL CHECK (message_seq > 0),
    message_id TEXT NOT NULL,
    summary TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_timeline_entries PRIMARY KEY (tenant_id, conversation_id, message_seq),
    CONSTRAINT uk_im_projection_timeline_entries_message UNIQUE (tenant_id, message_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_timeline_entries_message
    ON im_projection_timeline_entries (tenant_id, message_id);

CREATE TABLE IF NOT EXISTS im_projection_conversation_summaries (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    conversation_type TEXT,
    message_count BIGINT NOT NULL DEFAULT 0 CHECK (message_count >= 0),
    last_message_id TEXT,
    last_message_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_message_seq >= 0),
    last_sender_kind TEXT,
    last_sender_id TEXT,
    last_summary TEXT,
    last_message_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ NOT NULL,
    agent_handoff_json JSONB,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_conversation_summaries PRIMARY KEY (tenant_id, conversation_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_summaries_activity
    ON im_projection_conversation_summaries (tenant_id, last_activity_at DESC, conversation_id);

CREATE TABLE IF NOT EXISTS im_projection_conversation_members (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    member_id TEXT NOT NULL,
    membership_role TEXT NOT NULL,
    membership_state TEXT NOT NULL,
    invited_by TEXT,
    joined_at TIMESTAMPTZ NOT NULL,
    removed_at TIMESTAMPTZ,
    attributes_json JSONB NOT NULL DEFAULT '{}'::JSONB,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_conversation_members PRIMARY KEY (tenant_id, conversation_id, principal_kind, principal_id),
    CONSTRAINT uk_im_projection_conversation_members_member UNIQUE (tenant_id, conversation_id, member_id),
    CONSTRAINT ck_im_projection_conversation_members_state CHECK (membership_state IN ('invited', 'joined', 'removed', 'left'))
);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_members_principal
    ON im_projection_conversation_members (tenant_id, principal_kind, principal_id, membership_state, conversation_id);

CREATE INDEX IF NOT EXISTS idx_im_projection_conversation_members_active
    ON im_projection_conversation_members (tenant_id, conversation_id, principal_kind, principal_id)
    WHERE membership_state = 'joined';

CREATE TABLE IF NOT EXISTS im_projection_read_cursors (
    tenant_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    member_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    read_seq BIGINT NOT NULL DEFAULT 0 CHECK (read_seq >= 0),
    last_read_message_id TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_read_cursors PRIMARY KEY (tenant_id, conversation_id, member_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_read_cursors_principal
    ON im_projection_read_cursors (tenant_id, principal_kind, principal_id, conversation_id);

CREATE TABLE IF NOT EXISTS im_projection_registered_devices (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    registered_at TIMESTAMPTZ NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_registered_devices PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id)
);

CREATE TABLE IF NOT EXISTS im_projection_device_sync_feeds (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    sync_seq BIGINT NOT NULL CHECK (sync_seq > 0),
    origin_event_id TEXT NOT NULL,
    origin_event_type TEXT NOT NULL,
    conversation_id TEXT,
    message_id TEXT,
    message_seq BIGINT CHECK (message_seq IS NULL OR message_seq > 0),
    member_id TEXT,
    read_seq BIGINT CHECK (read_seq IS NULL OR read_seq >= 0),
    last_read_message_id TEXT,
    actor_kind TEXT,
    actor_id TEXT,
    actor_device_id TEXT,
    summary TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_device_sync_feeds PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id, sync_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_device_sync_feeds_window
    ON im_projection_device_sync_feeds (tenant_id, principal_kind, principal_id, device_id, sync_seq);

CREATE INDEX IF NOT EXISTS idx_im_projection_device_sync_feeds_conversation
    ON im_projection_device_sync_feeds (tenant_id, conversation_id, sync_seq)
    WHERE conversation_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS im_projection_device_sync_checkpoints (
    tenant_id TEXT NOT NULL,
    principal_kind TEXT NOT NULL,
    principal_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    latest_sync_seq BIGINT NOT NULL DEFAULT 0 CHECK (latest_sync_seq >= 0),
    trimmed_through_seq BIGINT NOT NULL DEFAULT 0 CHECK (trimmed_through_seq >= 0),
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_device_sync_checkpoints PRIMARY KEY (tenant_id, principal_kind, principal_id, device_id),
    CONSTRAINT ck_im_projection_device_sync_checkpoints_order CHECK (trimmed_through_seq <= latest_sync_seq)
);

CREATE TABLE IF NOT EXISTS im_projection_contacts (
    tenant_id TEXT NOT NULL,
    owner_user_id TEXT NOT NULL,
    contact_type TEXT NOT NULL,
    target_user_id TEXT NOT NULL,
    relationship_state TEXT NOT NULL,
    friendship_id TEXT NOT NULL,
    direct_chat_id TEXT,
    conversation_id TEXT,
    established_at TIMESTAMPTZ NOT NULL,
    last_interaction_at TIMESTAMPTZ NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_contacts PRIMARY KEY (tenant_id, owner_user_id, contact_type, target_user_id)
);

CREATE INDEX IF NOT EXISTS idx_im_projection_contacts_owner_activity
    ON im_projection_contacts (tenant_id, owner_user_id, last_interaction_at DESC, target_user_id);

CREATE TABLE IF NOT EXISTS im_projection_direct_chat_bindings (
    tenant_id TEXT NOT NULL,
    direct_chat_id TEXT NOT NULL,
    conversation_id TEXT NOT NULL,
    direct_chat_status TEXT NOT NULL,
    bound_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_projection_direct_chat_bindings PRIMARY KEY (tenant_id, direct_chat_id),
    CONSTRAINT uk_im_projection_direct_chat_bindings_conversation UNIQUE (tenant_id, conversation_id),
    CONSTRAINT ck_im_projection_direct_chat_bindings_status CHECK (direct_chat_status IN ('active', 'archived'))
);

CREATE INDEX IF NOT EXISTS idx_im_projection_direct_chat_bindings_conversation
    ON im_projection_direct_chat_bindings (tenant_id, conversation_id, direct_chat_status);

CREATE TABLE IF NOT EXISTS im_stream_sessions (
    tenant_id TEXT NOT NULL,
    stream_id TEXT NOT NULL,
    owner_principal_kind TEXT NOT NULL,
    owner_principal_id TEXT NOT NULL,
    stream_type TEXT NOT NULL,
    scope_kind TEXT NOT NULL,
    scope_id TEXT NOT NULL,
    durability_class TEXT NOT NULL,
    ordering_scope TEXT NOT NULL,
    schema_ref TEXT,
    stream_state TEXT NOT NULL,
    last_frame_seq BIGINT NOT NULL DEFAULT 0 CHECK (last_frame_seq >= 0),
    last_checkpoint_seq BIGINT CHECK (last_checkpoint_seq >= 0),
    result_message_id TEXT,
    complete_frame_seq BIGINT CHECK (complete_frame_seq >= 0),
    abort_frame_seq BIGINT CHECK (abort_frame_seq >= 0),
    abort_reason TEXT,
    opened_at TIMESTAMPTZ NOT NULL,
    closed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_stream_sessions PRIMARY KEY (tenant_id, stream_id),
    CONSTRAINT ck_im_stream_sessions_state CHECK (stream_state IN ('created', 'opened', 'active', 'checkpointed', 'completed', 'aborted', 'expired')),
    CONSTRAINT ck_im_stream_sessions_seq_order CHECK (
        COALESCE(last_checkpoint_seq, 0) <= last_frame_seq
        AND COALESCE(complete_frame_seq, 0) <= last_frame_seq
        AND COALESCE(abort_frame_seq, 0) <= last_frame_seq
    )
);

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_scope
    ON im_stream_sessions (tenant_id, scope_kind, scope_id, updated_at DESC);

CREATE INDEX IF NOT EXISTS idx_im_stream_sessions_updated
    ON im_stream_sessions (tenant_id, updated_at DESC, stream_id);

CREATE TABLE IF NOT EXISTS im_stream_frames (
    tenant_id TEXT NOT NULL,
    stream_id TEXT NOT NULL,
    frame_seq BIGINT NOT NULL CHECK (frame_seq > 0),
    producer_principal_kind TEXT NOT NULL,
    producer_principal_id TEXT NOT NULL,
    schema_ref TEXT,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    retention_until TIMESTAMPTZ,
    CONSTRAINT pk_im_stream_frames PRIMARY KEY (tenant_id, stream_id, frame_seq)
);

CREATE INDEX IF NOT EXISTS idx_im_stream_frames_stream_seq
    ON im_stream_frames (tenant_id, stream_id, frame_seq);
