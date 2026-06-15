-- Migration 012: Social Relations, Organization Model, Message Interactions
-- 对齐行业最专业 IM（微信/Telegram/Discord/Slack）的数据库设计
-- 所有 ID 统一使用 Snowflake ID (BIGINT)

-- ============================================================
-- 设计原则：
-- 1. 所有主键 ID 使用 Snowflake BIGINT
-- 2. 租户和用户引用 IAM 系统（iam_tenant, iam_user）
-- 3. 组织模型（Space/Group/Channel）是 IM 专有
-- 4. 社交关系独立持久化，不依赖内存+事件溯源
-- 5. 消息互动（Reaction/Pin/Thread）独立表
-- ============================================================

-- ============================================================
-- 第一部分：社交关系真值表
-- ============================================================

-- 1. 好友请求表
CREATE TABLE IF NOT EXISTS im_friend_requests (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    request_id          BIGINT NOT NULL,           -- Snowflake ID
    requester_user_id   TEXT NOT NULL,              -- 引用 iam_user.user_id
    target_user_id      TEXT NOT NULL,              -- 引用 iam_user.user_id
    request_message     TEXT,
    status              TEXT NOT NULL DEFAULT 'pending',
    expired_at          TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_friend_requests PRIMARY KEY (tenant_id, organization_id, request_id),
    CONSTRAINT uk_im_friend_requests_pair UNIQUE (tenant_id, organization_id, requester_user_id, target_user_id, status),
    CONSTRAINT chk_im_friend_requests_status CHECK (status IN ('pending', 'accepted', 'declined', 'canceled', 'expired')),
    CONSTRAINT chk_im_friend_requests_not_self CHECK (requester_user_id != target_user_id)
);

CREATE INDEX idx_im_friend_requests_requester
    ON im_friend_requests (tenant_id, organization_id, requester_user_id, status, created_at DESC);

CREATE INDEX idx_im_friend_requests_target
    ON im_friend_requests (tenant_id, organization_id, target_user_id, status, created_at DESC);

CREATE INDEX idx_im_friend_requests_expired
    ON im_friend_requests (tenant_id, organization_id, expired_at)
    WHERE expired_at IS NOT NULL AND status = 'pending';

-- 2. 好友关系表
CREATE TABLE IF NOT EXISTS im_friendships (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    friendship_id       BIGINT NOT NULL,           -- Snowflake ID
    user_low_id         TEXT NOT NULL,              -- 规范化：较小的 user_id
    user_high_id        TEXT NOT NULL,              -- 规范化：较大的 user_id
    initiator_user_id   TEXT NOT NULL,              -- 发起好友请求的用户
    status              TEXT NOT NULL DEFAULT 'active',
    established_at      TIMESTAMPTZ,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_friendships PRIMARY KEY (tenant_id, organization_id, friendship_id),
    CONSTRAINT uk_im_friendships_pair UNIQUE (tenant_id, organization_id, user_low_id, user_high_id),
    CONSTRAINT chk_im_friendships_status CHECK (status IN ('active', 'removed')),
    CONSTRAINT chk_im_friendships_not_self CHECK (user_low_id < user_high_id)
);

CREATE INDEX idx_im_friendships_user_low
    ON im_friendships (tenant_id, organization_id, user_low_id, status, established_at DESC);

CREATE INDEX idx_im_friendships_user_high
    ON im_friendships (tenant_id, organization_id, user_high_id, status, established_at DESC);

-- 3. 用户屏蔽表
CREATE TABLE IF NOT EXISTS im_user_blocks (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    block_id            BIGINT NOT NULL,           -- Snowflake ID
    blocker_user_id     TEXT NOT NULL,              -- 屏蔽者
    blocked_user_id     TEXT NOT NULL,              -- 被屏蔽者
    scope               TEXT NOT NULL DEFAULT 'all',
    direct_chat_id      BIGINT,                    -- 仅 direct_chat 作用域
    reason              TEXT,
    expires_at          TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_user_blocks PRIMARY KEY (tenant_id, organization_id, block_id),
    CONSTRAINT uk_im_user_blocks_pair UNIQUE (tenant_id, organization_id, blocker_user_id, blocked_user_id, scope),
    CONSTRAINT chk_im_user_blocks_scope CHECK (scope IN ('all', 'friendship', 'direct_chat')),
    CONSTRAINT chk_im_user_blocks_not_self CHECK (blocker_user_id != blocked_user_id)
);

CREATE INDEX idx_im_user_blocks_blocker
    ON im_user_blocks (tenant_id, organization_id, blocker_user_id, scope, created_at DESC);

CREATE INDEX idx_im_user_blocks_blocked
    ON im_user_blocks (tenant_id, organization_id, blocked_user_id, scope, created_at DESC);

CREATE INDEX idx_im_user_blocks_expires
    ON im_user_blocks (tenant_id, organization_id, expires_at)
    WHERE expires_at IS NOT NULL;

-- 4. 单聊会话表
CREATE TABLE IF NOT EXISTS im_direct_chats (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    direct_chat_id      BIGINT NOT NULL,           -- Snowflake ID
    left_actor_kind     TEXT NOT NULL,
    left_actor_id       TEXT NOT NULL,
    right_actor_kind    TEXT NOT NULL,
    right_actor_id      TEXT NOT NULL,
    pair_hash           TEXT NOT NULL,              -- 规范化后的哈希
    status              TEXT NOT NULL DEFAULT 'active',
    conversation_id     TEXT,                       -- 关联的会话 ID
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_direct_chats PRIMARY KEY (tenant_id, organization_id, direct_chat_id),
    CONSTRAINT uk_im_direct_chats_pair UNIQUE (tenant_id, organization_id, pair_hash),
    CONSTRAINT chk_im_direct_chats_status CHECK (status IN ('active', 'archived', 'closed'))
);

CREATE INDEX idx_im_direct_chats_left_actor
    ON im_direct_chats (tenant_id, organization_id, left_actor_kind, left_actor_id, status);

CREATE INDEX idx_im_direct_chats_right_actor
    ON im_direct_chats (tenant_id, organization_id, right_actor_kind, right_actor_id, status);

CREATE INDEX idx_im_direct_chats_conversation
    ON im_direct_chats (tenant_id, organization_id, conversation_id)
    WHERE conversation_id IS NOT NULL;

-- 5. 外部连接表
CREATE TABLE IF NOT EXISTS im_external_connections (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    connection_id       BIGINT NOT NULL,           -- Snowflake ID
    external_tenant_id  TEXT NOT NULL,
    external_org_name   TEXT,
    connection_kind     TEXT NOT NULL DEFAULT 'shared_channel',
    status              TEXT NOT NULL DEFAULT 'active',
    established_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_external_connections PRIMARY KEY (tenant_id, organization_id, connection_id),
    CONSTRAINT uk_im_external_connections_pair UNIQUE (tenant_id, organization_id, external_tenant_id),
    CONSTRAINT chk_im_external_connections_kind CHECK (connection_kind IN ('shared_channel')),
    CONSTRAINT chk_im_external_connections_status CHECK (status IN ('active', 'suspended', 'revoked')),
    CONSTRAINT chk_im_external_connections_not_self CHECK (tenant_id != external_tenant_id)
);

-- 6. 外部成员链接表
CREATE TABLE IF NOT EXISTS im_external_member_links (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT 'default',
    link_id                 BIGINT NOT NULL,           -- Snowflake ID
    connection_id           BIGINT NOT NULL,
    local_actor_kind        TEXT NOT NULL,
    local_actor_id          TEXT NOT NULL,
    external_member_id      TEXT NOT NULL,
    external_display_name   TEXT,
    status                  TEXT NOT NULL DEFAULT 'active',
    linked_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_external_member_links PRIMARY KEY (tenant_id, organization_id, link_id),
    CONSTRAINT uk_im_external_member_links_mapping UNIQUE (tenant_id, organization_id, connection_id, local_actor_id, external_member_id),
    CONSTRAINT chk_im_external_member_links_status CHECK (status IN ('active', 'revoked'))
);

CREATE INDEX idx_im_external_member_links_connection
    ON im_external_member_links (tenant_id, organization_id, connection_id, status);

CREATE INDEX idx_im_external_member_links_local_actor
    ON im_external_member_links (tenant_id, organization_id, local_actor_kind, local_actor_id, status);

-- 7. 共享频道策略表
CREATE TABLE IF NOT EXISTS im_shared_channel_policies (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT 'default',
    policy_id               BIGINT NOT NULL,           -- Snowflake ID
    connection_id           BIGINT NOT NULL,
    channel_id              TEXT NOT NULL,
    conversation_id         TEXT,
    policy_version          BIGINT NOT NULL DEFAULT 1,
    history_visibility      TEXT NOT NULL DEFAULT 'shared',
    status                  TEXT NOT NULL DEFAULT 'active',
    applied_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_shared_channel_policies PRIMARY KEY (tenant_id, organization_id, policy_id),
    CONSTRAINT uk_im_shared_channel_policies_target UNIQUE (tenant_id, organization_id, connection_id, channel_id),
    CONSTRAINT chk_im_shared_channel_policies_visibility CHECK (history_visibility IN ('shared', 'isolated')),
    CONSTRAINT chk_im_shared_channel_policies_status CHECK (status IN ('active', 'suspended'))
);

CREATE INDEX idx_im_shared_channel_policies_connection
    ON im_shared_channel_policies (tenant_id, organization_id, connection_id, status);

-- ============================================================
-- 第二部分：组织模型（IM 专有）
-- ============================================================

-- 8. 空间/组织表
CREATE TABLE IF NOT EXISTS im_spaces (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    space_id            BIGINT NOT NULL,           -- Snowflake ID
    space_name          TEXT NOT NULL,
    space_type          TEXT NOT NULL DEFAULT 'organization',
    owner_user_id       TEXT NOT NULL,              -- 引用 iam_user.user_id
    description         TEXT,
    avatar_url          TEXT,
    max_members         INTEGER NOT NULL DEFAULT 10000,
    settings_json       JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_spaces PRIMARY KEY (tenant_id, organization_id, space_id),
    CONSTRAINT chk_im_spaces_type CHECK (space_type IN ('organization', 'team', 'project', 'community'))
);

CREATE INDEX idx_im_spaces_owner
    ON im_spaces (tenant_id, organization_id, owner_user_id, created_at DESC);

CREATE INDEX idx_im_spaces_type
    ON im_spaces (tenant_id, organization_id, space_type, created_at DESC);

-- 9. 空间成员表
CREATE TABLE IF NOT EXISTS im_space_members (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    space_id            BIGINT NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    role                TEXT NOT NULL DEFAULT 'member',
    nickname            TEXT,
    joined_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_space_members PRIMARY KEY (tenant_id, organization_id, space_id, user_id),
    CONSTRAINT chk_im_space_members_role CHECK (role IN ('owner', 'admin', 'member', 'guest'))
);

CREATE INDEX idx_im_space_members_user
    ON im_space_members (tenant_id, organization_id, user_id, role);

-- 10. 群组表
CREATE TABLE IF NOT EXISTS im_chat_groups (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    group_id            BIGINT NOT NULL,           -- Snowflake ID
    space_id            BIGINT,                    -- 所属空间（可选）
    group_name          TEXT NOT NULL,
    group_type          TEXT NOT NULL DEFAULT 'normal',
    owner_user_id       TEXT NOT NULL,              -- 引用 iam_user.user_id
    conversation_id     TEXT,                       -- 关联的会话 ID
    max_members         INTEGER NOT NULL DEFAULT 500,
    description         TEXT,
    avatar_url          TEXT,
    announcement        TEXT,                       -- 群公告
    settings_json       JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_chat_groups PRIMARY KEY (tenant_id, organization_id, group_id),
    CONSTRAINT chk_im_chat_groups_type CHECK (group_type IN ('normal', 'announcement', 'project', 'department'))
);

CREATE INDEX idx_im_chat_groups_space
    ON im_chat_groups (tenant_id, organization_id, space_id, created_at DESC)
    WHERE space_id IS NOT NULL;

CREATE INDEX idx_im_chat_groups_owner
    ON im_chat_groups (tenant_id, organization_id, owner_user_id, created_at DESC);

CREATE INDEX idx_im_chat_groups_conversation
    ON im_chat_groups (tenant_id, organization_id, conversation_id)
    WHERE conversation_id IS NOT NULL;

-- 11. 群组成员表
CREATE TABLE IF NOT EXISTS im_group_members (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    group_id            BIGINT NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    role                TEXT NOT NULL DEFAULT 'member',
    nickname            TEXT,                       -- 群内昵称
    mute_until          TIMESTAMPTZ,               -- 禁言截止时间
    joined_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_group_members PRIMARY KEY (tenant_id, organization_id, group_id, user_id),
    CONSTRAINT chk_im_group_members_role CHECK (role IN ('owner', 'admin', 'member', 'muted'))
);

CREATE INDEX idx_im_group_members_user
    ON im_group_members (tenant_id, organization_id, user_id, role);

CREATE INDEX idx_im_group_members_role
    ON im_group_members (tenant_id, organization_id, group_id, role, joined_at);

-- 12. 频道表
CREATE TABLE IF NOT EXISTS im_chat_channels (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    channel_id          BIGINT NOT NULL,           -- Snowflake ID
    space_id            BIGINT NOT NULL,
    channel_name        TEXT NOT NULL,
    channel_type        TEXT NOT NULL DEFAULT 'text',
    description         TEXT,
    conversation_id     TEXT,                       -- 关联的会话 ID
    position            INTEGER NOT NULL DEFAULT 0,
    is_nsfw             BOOLEAN NOT NULL DEFAULT FALSE,
    is_pinned           BOOLEAN NOT NULL DEFAULT FALSE,
    topic               TEXT,                       -- 频道话题
    settings_json       JSONB NOT NULL DEFAULT '{}',
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_chat_channels PRIMARY KEY (tenant_id, organization_id, channel_id),
    CONSTRAINT chk_im_chat_channels_type CHECK (channel_type IN ('text', 'voice', 'announcement', 'forum'))
);

CREATE INDEX idx_im_chat_channels_space
    ON im_chat_channels (tenant_id, organization_id, space_id, position, channel_name);

CREATE INDEX idx_im_chat_channels_conversation
    ON im_chat_channels (tenant_id, organization_id, conversation_id)
    WHERE conversation_id IS NOT NULL;

-- 13. 频道访问规则表
CREATE TABLE IF NOT EXISTS im_channel_access_rules (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    rule_id             BIGINT NOT NULL,           -- Snowflake ID
    channel_id          BIGINT NOT NULL,
    rule_type           TEXT NOT NULL,
    principal_kind      TEXT,                       -- user/role/group
    principal_id        TEXT,
    permission          TEXT NOT NULL,              -- view/send/manage
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_channel_access_rules PRIMARY KEY (tenant_id, organization_id, rule_id),
    CONSTRAINT uk_im_channel_access_rules_target UNIQUE (tenant_id, organization_id, channel_id, rule_type, principal_kind, principal_id, permission),
    CONSTRAINT chk_im_channel_access_rules_type CHECK (rule_type IN ('allow', 'deny'))
);

CREATE INDEX idx_im_channel_access_rules_channel
    ON im_channel_access_rules (tenant_id, organization_id, channel_id, rule_type);

-- ============================================================
-- 第三部分：消息互动表
-- ============================================================

-- 14. 消息 Reaction 表
CREATE TABLE IF NOT EXISTS im_message_reactions (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    conversation_id     TEXT NOT NULL,
    message_id          BIGINT NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    reaction_type       TEXT NOT NULL,              -- emoji 类型（如 👍, ❤️, 😂）
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_message_reactions PRIMARY KEY (tenant_id, organization_id, conversation_id, message_id, user_id, reaction_type)
);

CREATE INDEX idx_im_message_reactions_message
    ON im_message_reactions (tenant_id, organization_id, conversation_id, message_id, reaction_type);

CREATE INDEX idx_im_message_reactions_user
    ON im_message_reactions (tenant_id, organization_id, user_id, created_at DESC);

-- 15. 消息 Pin 表
CREATE TABLE IF NOT EXISTS im_message_pins (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    conversation_id     TEXT NOT NULL,
    message_id          BIGINT NOT NULL,
    pinned_by_user_id   TEXT NOT NULL,              -- 引用 iam_user.user_id
    pin_reason          TEXT,
    pinned_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_message_pins PRIMARY KEY (tenant_id, organization_id, conversation_id, message_id)
);

CREATE INDEX idx_im_message_pins_conversation
    ON im_message_pins (tenant_id, organization_id, conversation_id, pinned_at DESC);

CREATE INDEX idx_im_message_pins_user
    ON im_message_pins (tenant_id, organization_id, pinned_by_user_id, pinned_at DESC);

-- 16. Thread 表
CREATE TABLE IF NOT EXISTS im_threads (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    thread_id           BIGINT NOT NULL,           -- Snowflake ID
    conversation_id     TEXT NOT NULL,
    root_message_id     BIGINT NOT NULL,
    thread_title        TEXT,
    reply_count         INTEGER NOT NULL DEFAULT 0 CHECK (reply_count >= 0),
    last_reply_at       TIMESTAMPTZ,
    last_reply_user_id  TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_threads PRIMARY KEY (tenant_id, organization_id, thread_id),
    CONSTRAINT uk_im_threads_root UNIQUE (tenant_id, organization_id, conversation_id, root_message_id)
);

CREATE INDEX idx_im_threads_conversation
    ON im_threads (tenant_id, organization_id, conversation_id, last_reply_at DESC);

CREATE INDEX idx_im_threads_root_message
    ON im_threads (tenant_id, organization_id, root_message_id);

-- 17. Thread 订阅表
CREATE TABLE IF NOT EXISTS im_thread_subscriptions (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    thread_id           BIGINT NOT NULL,
    user_id             TEXT NOT NULL,              -- 引用 iam_user.user_id
    last_read_seq       BIGINT NOT NULL DEFAULT 0,
    notification_level  TEXT NOT NULL DEFAULT 'all',
    subscribed_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_thread_subscriptions PRIMARY KEY (tenant_id, organization_id, thread_id, user_id),
    CONSTRAINT chk_im_thread_subscriptions_level CHECK (notification_level IN ('all', 'mentions', 'none'))
);

CREATE INDEX idx_im_thread_subscriptions_user
    ON im_thread_subscriptions (tenant_id, organization_id, user_id, subscribed_at DESC);

-- ============================================================
-- 第四部分：IM 用户扩展表
-- ============================================================

-- 18. IM 用户资料扩展表
CREATE TABLE IF NOT EXISTS im_user_profiles (
    tenant_id               TEXT NOT NULL,
    organization_id         TEXT NOT NULL DEFAULT 'default',
    user_id                 TEXT NOT NULL,              -- 引用 iam_user.user_id
    im_nickname             TEXT,                       -- IM 专属昵称
    im_avatar_url           TEXT,                       -- IM 专属头像
    im_status_message       TEXT,                       -- 状态消息
    im_notification_prefs   JSONB NOT NULL DEFAULT '{}', -- 通知偏好
    im_mute_settings        JSONB NOT NULL DEFAULT '{}', -- 免打扰设置
    im_privacy_settings     JSONB NOT NULL DEFAULT '{}', -- 隐私设置
    im_online_status        TEXT NOT NULL DEFAULT 'online',
    last_active_at          TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_user_profiles PRIMARY KEY (tenant_id, organization_id, user_id),
    CONSTRAINT chk_im_user_profiles_online_status CHECK (im_online_status IN ('online', 'away', 'busy', 'invisible', 'offline'))
);

-- 19. 用户设置表
CREATE TABLE IF NOT EXISTS im_user_settings (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    user_id             TEXT NOT NULL,
    setting_key         TEXT NOT NULL,
    setting_value       JSONB NOT NULL,
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_user_settings PRIMARY KEY (tenant_id, organization_id, user_id, setting_key)
);

-- 20. 会话设置表（用户对特定会话的设置）
CREATE TABLE IF NOT EXISTS im_conversation_settings (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    conversation_id     TEXT NOT NULL,
    user_id             TEXT NOT NULL,
    is_muted            BOOLEAN NOT NULL DEFAULT FALSE,
    mute_until          TIMESTAMPTZ,
    is_pinned           BOOLEAN NOT NULL DEFAULT FALSE,
    is_archived         BOOLEAN NOT NULL DEFAULT FALSE,
    is_blocked          BOOLEAN NOT NULL DEFAULT FALSE,
    notification_level  TEXT NOT NULL DEFAULT 'all',
    custom_name         TEXT,                       -- 用户自定义会话名称
    settings_json       JSONB NOT NULL DEFAULT '{}',
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_conversation_settings PRIMARY KEY (tenant_id, organization_id, conversation_id, user_id),
    CONSTRAINT chk_im_conversation_settings_notification CHECK (notification_level IN ('all', 'mentions', 'none'))
);

CREATE INDEX idx_im_conversation_settings_user
    ON im_conversation_settings (tenant_id, organization_id, user_id, is_pinned DESC, updated_at DESC);

-- ============================================================
-- 第五部分：消息搜索索引
-- ============================================================

-- 21. 消息搜索向量列
ALTER TABLE im_conversation_messages ADD COLUMN IF NOT EXISTS search_vector tsvector;

-- 22. 消息搜索索引
CREATE INDEX IF NOT EXISTS idx_im_messages_search
    ON im_conversation_messages USING GIN(search_vector)
    WHERE deleted_at IS NULL;

-- 23. 消息搜索触发器
CREATE OR REPLACE FUNCTION im_messages_search_trigger() RETURNS trigger AS $$
BEGIN
    NEW.search_vector := to_tsvector('simple',
        COALESCE(NEW.payload_json->>'text', '') || ' ' ||
        COALESCE(NEW.payload_json->>'caption', '') || ' ' ||
        COALESCE(NEW.payload_json->>'description', '')
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS im_messages_search_update ON im_conversation_messages;
CREATE TRIGGER im_messages_search_update
    BEFORE INSERT OR UPDATE ON im_conversation_messages
    FOR EACH ROW EXECUTE FUNCTION im_messages_search_trigger();

-- ============================================================
-- 第六部分：邀请和封禁
-- ============================================================

-- 24. 邀请表
CREATE TABLE IF NOT EXISTS im_invitations (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    invitation_id       BIGINT NOT NULL,           -- Snowflake ID
    inviter_user_id     TEXT NOT NULL,
    invitee_user_id     TEXT,
    invitee_email       TEXT,
    invitee_phone       TEXT,
    target_type         TEXT NOT NULL,              -- space/group/channel
    target_id           BIGINT NOT NULL,
    role                TEXT NOT NULL DEFAULT 'member',
    status              TEXT NOT NULL DEFAULT 'pending',
    message             TEXT,
    expires_at          TIMESTAMPTZ,
    accepted_at         TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_invitations PRIMARY KEY (tenant_id, organization_id, invitation_id),
    CONSTRAINT chk_im_invitations_target_type CHECK (target_type IN ('space', 'group', 'channel')),
    CONSTRAINT chk_im_invitations_status CHECK (status IN ('pending', 'accepted', 'declined', 'expired', 'canceled'))
);

CREATE INDEX idx_im_invitations_invitee
    ON im_invitations (tenant_id, organization_id, invitee_user_id, status, created_at DESC)
    WHERE invitee_user_id IS NOT NULL;

CREATE INDEX idx_im_invitations_target
    ON im_invitations (tenant_id, organization_id, target_type, target_id, status);

-- 25. 封禁记录表
CREATE TABLE IF NOT EXISTS im_ban_records (
    tenant_id           TEXT NOT NULL,
    organization_id     TEXT NOT NULL DEFAULT 'default',
    ban_id              BIGINT NOT NULL,           -- Snowflake ID
    target_type         TEXT NOT NULL,              -- space/group/channel
    target_id           BIGINT NOT NULL,
    banned_user_id      TEXT NOT NULL,
    banned_by_user_id   TEXT NOT NULL,
    reason              TEXT,
    expires_at          TIMESTAMPTZ,
    unbanned_at         TIMESTAMPTZ,
    unbanned_by_user_id TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_im_ban_records PRIMARY KEY (tenant_id, organization_id, ban_id),
    CONSTRAINT chk_im_ban_records_target_type CHECK (target_type IN ('space', 'group', 'channel'))
);

CREATE INDEX idx_im_ban_records_target
    ON im_ban_records (tenant_id, organization_id, target_type, target_id, banned_user_id)
    WHERE unbanned_at IS NULL;

CREATE INDEX idx_im_ban_records_user
    ON im_ban_records (tenant_id, organization_id, banned_user_id, created_at DESC);

-- ============================================================
-- 完成
-- ============================================================

-- 注册新表到 database-table-registry.json
-- 注册新表到 database-prefix-registry.json
