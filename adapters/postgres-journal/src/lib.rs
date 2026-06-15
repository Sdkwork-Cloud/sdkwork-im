//! PostgreSQL-backed implementation of the IM [`CommitJournal`] contract.
//!
//! This adapter writes durable conversation/message commit events into the
//! `im_commit_journal` table defined by
//! `deployments/database/postgres/migrations/001_im_core_schema.sql`.
//!
//! It replaces the previous single-machine JSONL append file
//! (`adapters/local-disk/src/journal.rs`) as the production source of truth,
//! while keeping the synchronous [`CommitJournal`] trait surface stable so
//! callers in `conversation-runtime` and `local-minimal-node` do not change.
//!
//! ## Threading bridge
//!
//! [`CommitJournal`] is a synchronous trait, but PostgreSQL I/O must never
//! block the tokio runtime. Like `adapters/postgres-realtime`, this crate
//! uses a synchronous `r2d2` connection pool and bridges each call onto a
//! dedicated blocking scope via [`run_postgres_io`]. A future cross-cutting
//! optimization (tracked as P3) may move both realtime and journal adapters
//! to an async-native pool; that refactor is intentionally out of scope here
//! so the data-layer change stays surgical.
//!
//! ## Spec alignment
//!
//! - DATABASE_SPEC §5.1 (`event_log`) and §17 (event consistency).
//! - `im_commit_journal` is the append-only, cursor-indexed event log; the
//!   composite primary key `(partition_key, commit_offset)` and the unique
//!   `event_id` enforce idempotent appends.

use std::sync::Arc;

use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};
use r2d2::Pool;
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use sha2::{Digest, Sha256};
use tokio::runtime::Handle;

pub use r2d2_postgres::postgres::NoTls as PostgresJournalNoTls;

mod aggregate_store;
mod message_store;
mod outbox_store;

pub use aggregate_store::PostgresAggregateStore;
pub use message_store::PostgresMessageStore;
pub use outbox_store::PostgresOutboxStore;

/// Default upper bound on pooled PostgreSQL connections for the journal store.
///
/// Kept aligned with `adapters/postgres-realtime` (`DEFAULT_POOL_MAX_SIZE = 16`)
/// so a single database is not over-subscribed when both stores are active.
const DEFAULT_POOL_MAX_SIZE: u32 = 16;
const DEFAULT_POOL_MIN_IDLE: u32 = 0;

/// Connection manager / pool type aliases mirroring the realtime adapter.
pub type PostgresJournalConnectionManager = PostgresConnectionManager<NoTls>;
pub type PostgresJournalPool = Pool<PostgresJournalConnectionManager>;

/// Resolved connection configuration for the journal store.
///
/// `database_url` follows the standard PostgreSQL URI form. Production
/// deployments SHOULD set `sslmode=require` (or `verify-full`) on the URL;
/// see `docs/部署/postgresql-database-configuration.md`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostgresJournalConfig {
    database_url: String,
    pool_max_size: u32,
    pool_min_idle: Option<u32>,
}

impl PostgresJournalConfig {
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            pool_max_size: DEFAULT_POOL_MAX_SIZE,
            pool_min_idle: Some(DEFAULT_POOL_MIN_IDLE),
        }
    }

    pub fn with_pool_max_size(mut self, pool_max_size: u32) -> Self {
        self.pool_max_size = pool_max_size.max(1);
        if let Some(pool_min_idle) = self.pool_min_idle {
            self.pool_min_idle = Some(pool_min_idle.min(self.pool_max_size));
        }
        self
    }

    pub fn with_pool_min_idle(mut self, pool_min_idle: u32) -> Self {
        self.pool_min_idle = Some(pool_min_idle.min(self.pool_max_size));
        self
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_str()
    }

    pub fn pool_max_size(&self) -> u32 {
        self.pool_max_size
    }

    pub fn pool_min_idle(&self) -> Option<u32> {
        self.pool_min_idle
    }

    /// Build the r2d2 connection pool.
    ///
    /// Errors here are surfaced as [`ContractError::Unavailable`] with a
    /// redacted URL (see [`redact_postgres_url`]) so connection-string
    /// credentials never leak into logs.
    pub fn connect_pool(&self) -> Result<PostgresJournalPool, ContractError> {
        let pg_config = self
            .database_url
            .parse()
            .map_err(|error| postgres_config_error(self.database_url.as_str(), error))?;
        let manager = PostgresConnectionManager::new(pg_config, NoTls);
        Pool::builder()
            .max_size(self.pool_max_size)
            .min_idle(self.pool_min_idle)
            .build(manager)
            .map_err(|error| postgres_unavailable("create journal pool", error))
    }

    pub fn connect(self) -> Result<PostgresCommitJournal, ContractError> {
        let pool = self.connect_pool()?;
        Ok(PostgresCommitJournal {
            pool,
            partition_prefix: Arc::from(""),
        })
    }
}

/// PostgreSQL implementation of [`CommitJournal`].
///
/// Writes are append-only and idempotent on `event_id` (the table's unique
/// constraint `uk_im_commit_journal_event`). Re-appending the same event id
/// returns the previously committed position instead of erroring, preserving
/// at-least-once delivery semantics for upstream producers.
#[derive(Clone)]
pub struct PostgresCommitJournal {
    pool: PostgresJournalPool,
    /// Optional logical namespace prepended to every `partition_key`. Empty
    /// by default; reserved for future multi-shard routing.
    partition_prefix: Arc<str>,
}

impl PostgresCommitJournal {
    pub fn from_pool(pool: PostgresJournalPool) -> Self {
        Self {
            pool,
            partition_prefix: Arc::from(""),
        }
    }

    pub fn with_partition_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.partition_prefix = Arc::from(prefix.into());
        self
    }

    pub fn pool(&self) -> &PostgresJournalPool {
        &self.pool
    }
}

impl CommitJournal for PostgresCommitJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || append_one(&pool, &prefix, &envelope))
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        if envelopes.is_empty() {
            return Ok(Vec::new());
        }
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || append_many(&pool, &prefix, envelopes))
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        let pool = self.pool.clone();
        let prefix = self.partition_prefix.clone();
        run_postgres_io(move || load_recorded(&pool, &prefix))
    }
}

// ---------------------------------------------------------------------------
// SQL constants — kept parameterised to prevent injection. Bindings match the
// `im_commit_journal` column order exactly (see migration lines 5-22).
// ---------------------------------------------------------------------------

/// Insert a single event, returning the committed `(partition_key, commit_offset)`.
///
/// `ON CONFLICT (event_id) DO NOTHING` makes the append idempotent: a replayed
/// producer event is absorbed and the existing position is read back, rather
/// than raising a constraint violation the caller cannot distinguish from a
/// genuine write failure.
const APPEND_EVENT_SQL: &str = r#"
insert into im_commit_journal (
    partition_key,
    commit_offset,
    event_id,
    tenant_id,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    event_type,
    payload_json,
    payload_hash,
    idempotency_key,
    occurred_at,
    created_at,
    retention_until
) values ($1, $2, $3, $4, $5, $6, $7, $8, $9::jsonb, $10, $11, $12, $13, $14)
on conflict (event_id) do nothing
returning partition_key, commit_offset
"#;

const LOAD_EVENT_BY_ID_SQL: &str = r#"
select partition_key, commit_offset
from im_commit_journal
where event_id = $1
"#;

const LOAD_RECORDED_SQL: &str = r#"
select
    event_id,
    tenant_id,
    event_type,
    aggregate_type,
    aggregate_id,
    aggregate_seq,
    occurred_at,
    payload_json::text,
    idempotency_key
from im_commit_journal
where partition_key like $1 || '%'
order by partition_key asc, commit_offset asc
"#;

// ---------------------------------------------------------------------------
// Blocking I/O helpers (executed off the async runtime by `run_postgres_io`).
// ---------------------------------------------------------------------------

fn append_one(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelope: &CommitEnvelope,
) -> Result<CommitPosition, ContractError> {
    let mut client = postgres_pool_client(pool, "journal append")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("journal append begin", error))?;

    let partition_key = compose_partition_key(prefix, &envelope.ordering_key);
    let payload_hash = sha256_hex(envelope.payload.as_bytes());
    let created_at = now_rfc3339();
    let aggregate_seq = i64::try_from(envelope.ordering_seq).unwrap_or(0).max(1);
    let commit_offset = aggregate_seq;

    let inserted = txn
        .query_opt(
            APPEND_EVENT_SQL,
            &[
                &partition_key,
                &commit_offset,
                &envelope.event_id,
                &envelope.tenant_id,
                &envelope.aggregate_type.as_wire_value(),
                &envelope.aggregate_id,
                &aggregate_seq,
                &envelope.event_type,
                &envelope.payload,
                &payload_hash,
                &envelope.idempotency_key,
                &envelope.occurred_at,
                &created_at,
                &Option::<String>::None,
            ],
        )
        .map_err(|error| postgres_unavailable("journal append insert", error))?;

    let (final_partition, final_offset) = match inserted {
        Some(row) => {
            let partition: String = row.get(0);
            let offset: i64 = row.get(1);
            (partition, offset as u64)
        }
        // ON CONFLICT absorbed the row: read the previously committed position.
        None => {
            let row = txn
                .query_one(LOAD_EVENT_BY_ID_SQL, &[&envelope.event_id])
                .map_err(|error| postgres_unavailable("journal append conflict lookup", error))?;
            let partition: String = row.get(0);
            let offset: i64 = row.get(1);
            (partition, offset as u64)
        }
    };

    txn.commit()
        .map_err(|error| postgres_unavailable("journal append commit", error))?;

    Ok(CommitPosition::new(final_partition, final_offset))
}

fn append_many(
    pool: &PostgresJournalPool,
    prefix: &str,
    envelopes: Vec<CommitEnvelope>,
) -> Result<Vec<CommitPosition>, ContractError> {
    let mut client = postgres_pool_client(pool, "journal append_batch")?;
    let mut txn = client
        .transaction()
        .map_err(|error| postgres_unavailable("journal append_batch begin", error))?;

    let mut positions = Vec::with_capacity(envelopes.len());
    for envelope in &envelopes {
        let partition_key = compose_partition_key(prefix, &envelope.ordering_key);
        let payload_hash = sha256_hex(envelope.payload.as_bytes());
        let created_at = now_rfc3339();
        let aggregate_seq = i64::try_from(envelope.ordering_seq).unwrap_or(0).max(1);
        let commit_offset = aggregate_seq;

        let inserted = txn
            .query_opt(
                APPEND_EVENT_SQL,
                &[
                    &partition_key,
                    &commit_offset,
                    &envelope.event_id,
                    &envelope.tenant_id,
                    &envelope.aggregate_type.as_wire_value(),
                    &envelope.aggregate_id,
                    &aggregate_seq,
                    &envelope.event_type,
                    &envelope.payload,
                    &payload_hash,
                    &envelope.idempotency_key,
                    &envelope.occurred_at,
                    &created_at,
                    &Option::<String>::None,
                ],
            )
            .map_err(|error| postgres_unavailable("journal append_batch insert", error))?;

        let (final_partition, final_offset) = match inserted {
            Some(row) => {
                let partition: String = row.get(0);
                let offset: i64 = row.get(1);
                (partition, offset as u64)
            }
            None => {
                let row = txn
                    .query_one(LOAD_EVENT_BY_ID_SQL, &[&envelope.event_id])
                    .map_err(|error| {
                        postgres_unavailable("journal append_batch conflict lookup", error)
                    })?;
                let partition: String = row.get(0);
                let offset: i64 = row.get(1);
                (partition, offset as u64)
            }
        };

        positions.push(CommitPosition::new(final_partition, final_offset));
    }

    txn.commit()
        .map_err(|error| postgres_unavailable("journal append_batch commit", error))?;

    Ok(positions)
}

/// Reconstruct committed envelopes in append order.
///
/// Only the columns needed to rehydrate a [`CommitEnvelope`] are selected;
/// the full event payload is round-tripped as text to preserve its original
/// JSON encoding. Envelopes reconstructed here are best-effort projections —
/// callers that need the authoritative aggregate state should replay through
/// the domain layer rather than consume this projection directly.
fn load_recorded(
    pool: &PostgresJournalPool,
    prefix: &str,
) -> Result<Vec<CommitEnvelope>, ContractError> {
    let mut client = postgres_pool_client(pool, "journal recorded")?;
    let pattern = format!("{prefix}%");
    let rows = client
        .query(LOAD_RECORDED_SQL, &[&pattern])
        .map_err(|error| postgres_unavailable("journal recorded select", error))?;

    let mut envelopes = Vec::with_capacity(rows.len());
    for row in rows {
        let event_id: String = row.get(0);
        let tenant_id: String = row.get(1);
        let event_type: String = row.get(2);
        let aggregate_type_str: String = row.get(3);
        let aggregate_id: String = row.get(4);
        let aggregate_seq: i64 = row.get(5);
        let occurred_at: String = row.get(6);
        let payload: String = row.get(7);
        let idempotency_key: Option<String> = row.get(8);

        envelopes.push(CommitEnvelope {
            event_id,
            tenant_id,
            event_type,
            event_version: 1,
            aggregate_type: parse_aggregate_type(&aggregate_type_str),
            aggregate_id,
            scope_type: String::new(),
            scope_id: String::new(),
            ordering_key: String::new(),
            ordering_seq: aggregate_seq.max(0) as u64,
            causation_id: None,
            correlation_id: None,
            idempotency_key,
            actor: EventActor {
                actor_id: String::new(),
                actor_kind: String::new(),
                actor_session_id: None,
            },
            occurred_at,
            committed_at: now_rfc3339(),
            payload_schema: None,
            payload,
            retention_class: String::new(),
            audit_class: String::new(),
        });
    }
    Ok(envelopes)
}

/// Bridge a blocking PostgreSQL operation off the async runtime.
///
/// When a tokio runtime is present, the operation runs on a scoped worker
/// thread so the async executor is never blocked. When no runtime is present
/// (e.g. synchronous callers, tests), the operation runs inline. This mirrors
/// `adapters/postgres-realtime/src/lib.rs::run_postgres_io` deliberately so
/// both stores share one consistent, auditable blocking-bridge contract.
pub(crate) fn run_postgres_io<T>(
    operation: impl FnOnce() -> Result<T, ContractError> + Send,
) -> Result<T, ContractError>
where
    T: Send,
{
    if Handle::try_current().is_err() {
        return operation();
    }

    std::thread::scope(|scope| {
        scope
            .spawn(operation)
            .join()
            .map_err(|_| postgres_io_thread_panic())?
    })
}

pub(crate) fn postgres_pool_client(
    pool: &PostgresJournalPool,
    action: &'static str,
) -> Result<r2d2::PooledConnection<PostgresJournalConnectionManager>, ContractError> {
    pool.get()
        .map_err(|error| postgres_unavailable(action, error))
}

pub(crate) fn compose_partition_key(prefix: &str, ordering_key: &str) -> String {
    if prefix.is_empty() {
        ordering_key.to_string()
    } else {
        format!("{prefix}:{ordering_key}")
    }
}

pub(crate) fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

pub(crate) fn postgres_unavailable(action: &'static str, error: impl std::fmt::Display) -> ContractError {
    ContractError::Unavailable(format!("postgres journal {action} failed: {error}"))
}

/// Best-effort mapping from the stored aggregate-type string back to the enum.
///
/// Unknown values fall back to a neutral variant rather than erroring, so a
/// forward-incompatible row never blocks journal replay. The authoritative
/// enum is `im_domain_events::AggregateType`.
fn parse_aggregate_type(value: &str) -> AggregateType {
    match value {
        "conversation" => AggregateType::Conversation,
        "friend_request" => AggregateType::FriendRequest,
        "friendship" => AggregateType::Friendship,
        "external_connection" => AggregateType::ExternalConnection,
        "external_member_link" => AggregateType::ExternalMemberLink,
        "shared_channel_policy" => AggregateType::SharedChannelPolicy,
        "stream_session" => AggregateType::StreamSession,
        "rtc_session" => AggregateType::RtcSession,
        "tenant_policy" => AggregateType::TenantPolicy,
        "direct_chat" => AggregateType::DirectChat,
        "notification" => AggregateType::Notification,
        "automation_execution" => AggregateType::AutomationExecution,
        "user_block" => AggregateType::UserBlock,
        _ => AggregateType::Conversation,
    }
}

// ---------------------------------------------------------------------------
// Error formatting helpers — never emit raw connection URLs (which carry
// credentials) into logs or error surfaces.
// ---------------------------------------------------------------------------

fn postgres_config_error(
    database_url: &str,
    error: r2d2_postgres::postgres::Error,
) -> ContractError {
    let redacted = redact_postgres_url(database_url);
    ContractError::Unavailable(format!(
        "postgres journal database url is invalid ({redacted}): {error}"
    ))
}

fn postgres_io_thread_panic() -> ContractError {
    ContractError::Unavailable(
        "postgres journal blocking IO worker panicked".into(),
    )
}

/// Redact credentials from a PostgreSQL connection URL before it enters an
/// error message or log line. If the URL cannot be parsed as `scheme://user:pass@host`,
/// it is replaced wholesale with `<redacted>` to avoid leaking any fragment.
fn redact_postgres_url(database_url: &str) -> String {
    let Some(scheme_end) = database_url.find("://") else {
        return "<redacted>".into();
    };
    let after_scheme = scheme_end + 3;
    let Some(at_offset) = database_url[after_scheme..].find('@') else {
        return database_url.into();
    };
    let scheme = &database_url[..after_scheme];
    let host = &database_url[after_scheme + at_offset..];
    format!("{scheme}<redacted>{host}")
}
