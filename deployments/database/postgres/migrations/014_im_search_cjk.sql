-- Migration 014: Chinese / CJK Full-Text Search
-- ============================================================
-- Replaces the simple `to_tsvector('simple', ...)` trigger with
-- proper CJK tokenization using zhparser or pg_bigm extensions.
--
-- Strategy:
--   1. If zhparser is installed → use 'chinese_zh' text search config
--   2. If pg_bigm is installed  → use bigram-based similarity + GIN trigram index
--   3. Otherwise                  → keep 'simple' config (no CJK support)
--
-- Risk: LOW (non-destructive — only modifies the search trigger function)
-- ============================================================

-- ============================================================
-- Option A: zhparser (Chinese word segmentation)
-- ============================================================
-- zhparser provides Chinese word segmentation for PostgreSQL full-text search.
-- Installation: https://github.com/amutu/zhparser
--
-- After installing zhparser, run:
--   CREATE EXTENSION IF NOT EXISTS zhparser;
--   CREATE TEXT SEARCH CONFIGURATION chinese_zh (PARSER = zhparser);
--   ALTER TEXT SEARCH CONFIGURATION chinese_zh ADD MAPPING FOR n,v,a,i,e,l WITH simple;

-- ============================================================
-- Option B: pg_bigm / pg_trgm (bigram/trigram similarity)
-- ============================================================
-- pg_bigm provides 2-gram indexing for full-text search on CJK text.
-- pg_trgm ships with PostgreSQL and provides trigram matching.
--
-- After installing pg_bigm:
--   CREATE EXTENSION IF NOT EXISTS pg_bigm;
--   CREATE INDEX idx_im_messages_search_bigm
--       ON im_conversation_messages USING gin (payload_json_text gin_bigm_ops);
--
-- With pg_trgm (bundled with PostgreSQL):
--   CREATE EXTENSION IF NOT EXISTS pg_trgm;
--   CREATE INDEX idx_im_messages_search_trgm
--       ON im_conversation_messages USING gin (
--           (payload_json->>'text') gin_trgm_ops,
--           (payload_json->>'caption') gin_trgm_ops
--       );

-- ============================================================
-- Update the search trigger to handle Chinese text
-- ============================================================

CREATE OR REPLACE FUNCTION im_messages_search_trigger() RETURNS trigger AS $$
DECLARE
    raw_text text;
BEGIN
    raw_text := COALESCE(NEW.payload_json->>'text', '') || ' ' ||
                COALESCE(NEW.payload_json->>'caption', '') || ' ' ||
                COALESCE(NEW.payload_json->>'description', '');

    -- Use zhparser if available, otherwise fall back to simple
    -- (zhparser must be installed and 'chinese_zh' config created)
    BEGIN
        NEW.search_vector := to_tsvector('chinese_zh', raw_text);
    EXCEPTION WHEN OTHERS THEN
        -- Fallback: simple config (no CJK segmentation, but works for ASCII)
        NEW.search_vector := to_tsvector('simple', raw_text);
    END;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Recreate the trigger (replace the one from migration 012)
DROP TRIGGER IF EXISTS im_messages_search_update ON im_conversation_messages;
CREATE TRIGGER im_messages_search_update
    BEFORE INSERT OR UPDATE ON im_conversation_messages
    FOR EACH ROW EXECUTE FUNCTION im_messages_search_trigger();

-- ============================================================
-- CJK search index using pg_trgm (bundled with PostgreSQL 9.4+)
-- ============================================================
-- Provides fuzzy search for Chinese/Japanese/Korean without zhparser.
-- Enable with: CREATE EXTENSION IF NOT EXISTS pg_trgm;
--
-- CREATE INDEX IF NOT EXISTS idx_im_messages_search_cjk
--     ON im_conversation_messages USING gin (
--         (COALESCE(payload_json->>'text', '') || ' ' ||
--          COALESCE(payload_json->>'caption', '') || ' ' ||
--          COALESCE(payload_json->>'description', '')) gin_trgm_ops
--     )
--     WHERE deleted_at IS NULL;

-- ============================================================
-- 搜索架构说明
-- ============================================================
-- 默认使用 PostgreSQL 原生全文搜索。后续可通过 Provider 模式
-- （参考 PushProvider / RTC adapter）扩展为可插拔的搜索后端：
--
--   trait SearchProvider {
--       fn index_message(&self, message: &StoredMessageRecord) -> Result;
--       fn search(&self, tenant: &str, query: &str) -> Result<Vec<message_id>>;
--   }
--
-- PostgreSQL 实现即为本迁移的 search_vector + GIN 索引方案。
-- 如需切换到其他后端（如 Elasticsearch），实现 SearchProvider 并
-- 通过 ProviderRegistry 切换即可，无需修改消息写入/查询路径。

-- ============================================================
-- Migration checklist (MIGRATION_SPEC §2):
--   id: MIG-2026-0014
--   type: database
--   strategy: expand-contract (new trigger coexists with old index)
--   rollback: revert trigger to 'simple' config
--   verification:
--     - SELECT to_tsvector('chinese_zh', '你好世界') @@ to_tsquery('chinese_zh', '世界');
--     - EXPLAIN ANALYZE SELECT * FROM im_conversation_messages WHERE search_vector @@ plainto_tsquery('chinese_zh', '你好');
-- ============================================================
