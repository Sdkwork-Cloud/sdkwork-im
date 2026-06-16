-- DEFERRED (Phase 2): Do not apply via pnpm db:postgres:migrate.
-- See docs/superpowers/specs/2026-06-16-im-phase0-approach-a-design.md
--
-- Migration 013: Table Partitioning for Scale
-- ============================================================
-- Strategy: Expand-Contract (DATABASE_SPEC §3.5, MIGRATION_SPEC §3)
--   1. CREATE new partitioned tables alongside existing ones
--   2. CREATE indexes on partitioned tables (matching existing indexes)
--   3. Dual-write period: application writes to both old and new
--   4. Backfill: COPY old data into partitioned tables
--   5. Validation: compare row counts and checksums
--   6. Cutover: point application reads to partitioned tables
--   7. Contract: DROP old tables after verification window
--
-- Risk level: HIGH (requires DBA review, dual-write window, rollback plan)
-- Compliance: L2 Service Ready (multi-tenant, partitioning, retention)
-- ============================================================

-- ============================================================
-- 1. Conversation Messages — HASH(conversation_id) 64 partitions
-- ============================================================
-- Rationale: Messages are always queried by (tenant, org, conversation_id, message_seq).
-- HASH partitioning on conversation_id ensures even distribution and eliminates
-- the need for application-level sharding for up to ~10B messages.

CREATE TABLE IF NOT EXISTS im_conversation_messages_partitioned (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    conversation_id     TEXT NOT NULL,
    message_id          BIGINT NOT NULL,
    message_seq         BIGINT NOT NULL,
    sender_principal_kind TEXT NOT NULL,
    sender_principal_id TEXT NOT NULL,
    sender_device_id    TEXT,
    client_msg_id       TEXT,
    message_type        TEXT NOT NULL,
    payload_json        JSONB NOT NULL,
    payload_hash        TEXT NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at          TIMESTAMPTZ,
    retention_until     TIMESTAMPTZ,
    search_vector       tsvector
) PARTITION BY HASH (conversation_id);

-- Create 64 hash partitions (p00-p63)
-- In production, automate with: SELECT 'CREATE TABLE im_conversation_messages_p' || LPAD(i::text, 2, '0') || ' PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER ' || i || ');' FROM generate_series(0,63) i;
CREATE TABLE IF NOT EXISTS im_conversation_messages_p00 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 0);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p01 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 1);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p02 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 2);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p03 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 3);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p04 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 4);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p05 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 5);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p06 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 6);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p07 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 7);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p08 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 8);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p09 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 9);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p10 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 10);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p11 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 11);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p12 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 12);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p13 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 13);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p14 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 14);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p15 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 15);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p16 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 16);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p17 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 17);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p18 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 18);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p19 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 19);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p20 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 20);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p21 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 21);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p22 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 22);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p23 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 23);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p24 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 24);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p25 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 25);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p26 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 26);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p27 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 27);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p28 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 28);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p29 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 29);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p30 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 30);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p31 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 31);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p32 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 32);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p33 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 33);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p34 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 34);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p35 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 35);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p36 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 36);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p37 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 37);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p38 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 38);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p39 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 39);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p40 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 40);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p41 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 41);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p42 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 42);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p43 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 43);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p44 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 44);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p45 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 45);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p46 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 46);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p47 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 47);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p48 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 48);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p49 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 49);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p50 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 50);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p51 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 51);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p52 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 52);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p53 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 53);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p54 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 54);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p55 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 55);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p56 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 56);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p57 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 57);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p58 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 58);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p59 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 59);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p60 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 60);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p61 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 61);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p62 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 62);
CREATE TABLE IF NOT EXISTS im_conversation_messages_p63 PARTITION OF im_conversation_messages_partitioned FOR VALUES WITH (MODULUS 64, REMAINDER 63);

-- Re-apply indexes on the partitioned table (they propagate to partitions)
CREATE INDEX IF NOT EXISTS idx_im_msgs_p_tenant_conv_seq
    ON im_conversation_messages_partitioned (tenant_id, organization_id, conversation_id, message_seq DESC);
CREATE UNIQUE INDEX IF NOT EXISTS uk_im_msgs_p_message
    ON im_conversation_messages_partitioned (tenant_id, message_id);
CREATE INDEX IF NOT EXISTS idx_im_msgs_p_sender_created
    ON im_conversation_messages_partitioned (tenant_id, organization_id, sender_principal_kind, sender_principal_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_im_msgs_p_retention
    ON im_conversation_messages_partitioned (tenant_id, organization_id, retention_until)
    WHERE retention_until IS NOT NULL;

-- ============================================================
-- 2. Realtime Device Events — RANGE(created_at) monthly
-- ============================================================
-- Rationale: Events are short-lived (retention_until < 7 days typical).
-- Monthly RANGE partitions allow efficient DROP of expired partitions
-- instead of expensive DELETE with retention_until filtering.

CREATE TABLE IF NOT EXISTS im_realtime_device_events_partitioned (
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
    retention_until TIMESTAMPTZ
) PARTITION BY RANGE (created_at);

-- Create initial monthly partitions for current + next 3 months
-- In production, automate with pg_partman or a cron job.
-- Manual creation for the first few months:
CREATE TABLE IF NOT EXISTS im_realtime_device_events_202606 PARTITION OF im_realtime_device_events_partitioned
    FOR VALUES FROM ('2026-06-01') TO ('2026-07-01');
CREATE TABLE IF NOT EXISTS im_realtime_device_events_202607 PARTITION OF im_realtime_device_events_partitioned
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');
CREATE TABLE IF NOT EXISTS im_realtime_device_events_202608 PARTITION OF im_realtime_device_events_partitioned
    FOR VALUES FROM ('2026-08-01') TO ('2026-09-01');
CREATE TABLE IF NOT EXISTS im_realtime_device_events_202609 PARTITION OF im_realtime_device_events_partitioned
    FOR VALUES FROM ('2026-09-01') TO ('2026-10-01');

CREATE INDEX IF NOT EXISTS idx_im_rdev_p_scope_seq
    ON im_realtime_device_events_partitioned (tenant_id, organization_id, client_route_scope_key, realtime_seq);
CREATE INDEX IF NOT EXISTS idx_im_rdev_p_fanout
    ON im_realtime_device_events_partitioned (tenant_id, organization_id, scope_type, scope_id, event_type, realtime_seq);

-- ============================================================
-- 3. Outbox Events — RANGE(created_at) monthly
-- ============================================================
CREATE TABLE IF NOT EXISTS im_outbox_events_partitioned (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    outbox_id TEXT NOT NULL,
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
    retention_until TIMESTAMPTZ
) PARTITION BY RANGE (created_at);

CREATE TABLE IF NOT EXISTS im_outbox_events_202606 PARTITION OF im_outbox_events_partitioned
    FOR VALUES FROM ('2026-06-01') TO ('2026-07-01');
CREATE TABLE IF NOT EXISTS im_outbox_events_202607 PARTITION OF im_outbox_events_partitioned
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');
CREATE TABLE IF NOT EXISTS im_outbox_events_202608 PARTITION OF im_outbox_events_partitioned
    FOR VALUES FROM ('2026-08-01') TO ('2026-09-01');

CREATE INDEX IF NOT EXISTS idx_im_outbox_p_status
    ON im_outbox_events_partitioned (tenant_id, organization_id, publish_status, available_at, outbox_id);

-- ============================================================
-- 4. Commit Journal — RANGE(created_at) monthly
-- ============================================================
CREATE TABLE IF NOT EXISTS im_commit_journal_partitioned (
    partition_key TEXT NOT NULL,
    commit_offset BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL DEFAULT 'default',
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    aggregate_seq BIGINT NOT NULL CHECK (aggregate_seq > 0),
    event_type TEXT NOT NULL,
    payload_json JSONB NOT NULL,
    payload_hash TEXT NOT NULL,
    idempotency_key TEXT,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    retention_until TIMESTAMPTZ
) PARTITION BY RANGE (created_at);

CREATE TABLE IF NOT EXISTS im_commit_journal_202606 PARTITION OF im_commit_journal_partitioned
    FOR VALUES FROM ('2026-06-01') TO ('2026-07-01');
CREATE TABLE IF NOT EXISTS im_commit_journal_202607 PARTITION OF im_commit_journal_partitioned
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');
CREATE TABLE IF NOT EXISTS im_commit_journal_202608 PARTITION OF im_commit_journal_partitioned
    FOR VALUES FROM ('2026-08-01') TO ('2026-09-01');

CREATE INDEX IF NOT EXISTS idx_im_journal_p_tenant_aggregate
    ON im_commit_journal_partitioned (tenant_id, organization_id, aggregate_type, aggregate_id, aggregate_seq);
CREATE UNIQUE INDEX IF NOT EXISTS uk_im_journal_p_event
    ON im_commit_journal_partitioned (event_id);

-- ============================================================
-- 5. Retention cleanup automation (recommended pg_partman config)
-- ============================================================
-- After deploying, configure pg_partman for automatic partition management:
--
-- SELECT partman.create_parent(
--     p_parent_table := 'public.im_realtime_device_events_partitioned',
--     p_control := 'created_at',
--     p_type := 'native',
--     p_interval := '1 month',
--     p_premake := 3,
--     p_retention := '3 months',
--     p_retention_keep_table := false
-- );
--
-- Manual retention cleanup for tables without pg_partman:
-- DROP TABLE IF EXISTS im_realtime_device_events_202603;  -- drops old partition

-- ============================================================
-- Migration checklist (MIGRATION_SPEC §2):
--   id: MIG-2026-0013
--   type: database
--   strategy: expand-contract (dual-write → backfill → validate → cutover → contract)
--   rollback: DROP partitioned tables, app continues using original tables
--   verification:
--     - SELECT count(*) FROM im_conversation_messages;
--     - SELECT count(*) FROM im_conversation_messages_partitioned;
--     - pgbench -T 60 -f query.sql (verify no regression)
-- ============================================================
