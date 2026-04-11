use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path as StdPath, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use audit_service::{AuditRuntime, RecordAuditAnchor};
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use bytes::Bytes;
use craw_chat_ccp_registry::{
    BusinessPolicyVocabulary, CapabilityProfile, CcpRegistry, ClientCompatibilityDescriptor,
    EffectiveProtocolSnapshot, KillSwitchRule, ProtocolGovernanceSnapshot, QuotaProfile,
    ReleaseChannel, RolloutPolicy, SchemaDescriptor,
};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Method, Request as HyperRequest};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use im_adapters_local_disk::{FileCommitJournal, read_commit_journal_file};
use im_adapters_local_memory::MemoryCommitJournal;
use im_auth_context::{
    AuthContext, AuthContextError, PUBLIC_BEARER_HS256_SECRET_ENV, encode_hs256_bearer_token,
    resolve_auth_context, resolve_public_bearer_auth_context,
    resolve_public_bearer_required_audience, resolve_public_bearer_required_issuer,
};
use im_domain_core::social::{
    BlockScope, DirectChat, DirectChatStatus, ExternalConnection, ExternalConnectionKind,
    ExternalConnectionStatus, ExternalMemberLink, ExternalMemberLinkStatus, FriendRequest,
    FriendRequestStatus, Friendship, FriendshipStatus, SharedChannelPolicy,
    SharedChannelPolicyStatus, SocialInvariantError, UserBlock, UserBlockStatus,
    ensure_cross_tenant_connection, normalize_actor_pair, normalize_user_pair,
};
use im_domain_events::social::{
    DirectChatBoundPayload, ExternalConnectionEstablishedPayload, ExternalMemberLinkBoundPayload,
    FriendRequestSubmittedPayload, FriendshipActivatedPayload, SharedChannelPolicyAppliedPayload,
    SocialEventType, UserBlockedPayload, social_commit_envelope,
};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    CommitJournal, ContractError, EffectiveProviderBinding, PROVIDER_REGISTRY_INTERFACE_VERSION,
    ProviderDomain, ProviderPolicyCommit, ProviderPolicyDiff, ProviderPolicyHistory,
    ProviderPolicyPreview, ProviderPolicyResultStatus, ProviderRegistry, ProviderRegistrySnapshot,
    RuntimeProviderRegistry,
};
use im_time::format_unix_timestamp_millis;
use ops_service::{
    OpsRuntime, ProviderBindingItemView, ProviderBindingSnapshotView, RouteOwnershipView,
};
use serde::{Deserialize, Serialize};
use session_gateway::{
    RealtimeClusterBridge, RealtimeClusterError, RealtimeNodeLifecycleView,
    RealtimeRouteMigrationResult,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedChannelLinkedMemberSyncRequest {
    pub tenant_id: String,
    pub conversation_id: String,
    pub shared_channel_policy_id: String,
    pub external_connection_id: String,
    pub local_actor_id: String,
    pub local_actor_kind: String,
    pub external_member_id: String,
}

pub trait SharedChannelLinkedMemberSyncTrigger: Send + Sync {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String>;
}

#[derive(Clone)]
struct AppState {
    realtime_cluster: Arc<RealtimeClusterBridge>,
    protocol_registry: Arc<CcpRegistry>,
    provider_registry: Arc<dyn ProviderRegistry>,
    provider_registry_runtime: Option<Arc<RuntimeProviderRegistry>>,
    governance_loop: Option<GovernanceLoop>,
    social_runtime: Arc<SocialControlRuntime>,
    shared_channel_sync_trigger: Option<Arc<dyn SharedChannelLinkedMemberSyncTrigger>>,
}

#[derive(Clone)]
struct GovernanceLoop {
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
}

const SOCIAL_STATE_FILE_NAME: &str = "social-state.json";
const SOCIAL_COMMIT_JOURNAL_FILE_NAME: &str = "social-commit-journal.json";
const SOCIAL_TRANSACTION_MARKER_FILE_NAME: &str = "social-transaction-marker.json";
const SOCIAL_COMMIT_PARTITION: &str = "control-plane-social";
const PUBLIC_SHARED_CHANNEL_SYNC_ROUTE: &str = "/api/v1/conversations/shared-channel-links/sync";
const PUBLIC_SHARED_CHANNEL_SYNC_ACTOR_ID: &str = "control-plane-sync";
pub const SHARED_CHANNEL_SYNC_PERMISSION: &str = "conversation.shared_channel.sync";
const SHARED_CHANNEL_SYNC_DEAD_LETTER_FAILURE_THRESHOLD: u32 = 3;
const SHARED_CHANNEL_SYNC_PENDING_LEASE_WINDOW_MILLIS: u128 = 900_000;
pub const ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP_ENV: &str =
    "CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP";
pub const SHARED_CHANNEL_SYNC_TARGET_BASE_URL_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_TARGET_BASE_URL";
pub const SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS";
const SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_DEFAULT_MILLIS: u64 = 5_000;
const SHARED_CHANNEL_SYNC_RESPONSE_BODY_MAX_BYTES: usize = 16 * 1024;
pub const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED";
pub const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS";
const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_INTERVAL_MILLIS: u64 = 30_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SharedChannelSyncStaleReclaimSchedulerConfig {
    pub enabled: bool,
    pub interval_millis: u64,
}

impl SharedChannelSyncStaleReclaimSchedulerConfig {
    fn with_normalized_interval(self) -> Self {
        Self {
            enabled: self.enabled,
            interval_millis: if self.interval_millis == 0 {
                SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_INTERVAL_MILLIS
            } else {
                self.interval_millis
            },
        }
    }

    fn interval(self) -> Duration {
        Duration::from_millis(self.with_normalized_interval().interval_millis)
    }
}

struct SharedChannelSyncDispatchTask {
    request: SharedChannelLinkedMemberSyncRequest,
    response_tx: std::sync::mpsc::Sender<Result<(), String>>,
}

struct PublicSharedChannelLinkedMemberSyncTrigger {
    dispatch_tx: std::sync::mpsc::Sender<SharedChannelSyncDispatchTask>,
}

impl PublicSharedChannelLinkedMemberSyncTrigger {
    fn new(
        base_url: impl AsRef<str>,
        public_bearer_secret: impl AsRef<str>,
    ) -> Result<Self, String> {
        let base_url = validate_shared_channel_sync_target_base_url(base_url.as_ref())?;

        let public_bearer_secret = public_bearer_secret.as_ref().trim().to_owned();
        if public_bearer_secret.is_empty() {
            return Err("shared-channel sync public bearer secret cannot be empty".into());
        }

        let (dispatch_tx, dispatch_rx) =
            std::sync::mpsc::channel::<SharedChannelSyncDispatchTask>();
        std::thread::Builder::new()
            .name("shared-sync-dispatch-worker".to_owned())
            .spawn(move || {
                let runtime = match tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                {
                    Ok(runtime) => runtime,
                    Err(error) => {
                        while let Ok(task) = dispatch_rx.recv() {
                            let _ = task.response_tx.send(Err(format!(
                                "failed to build shared-channel sync worker runtime: {error}"
                            )));
                        }
                        return;
                    }
                };
                while let Ok(task) = dispatch_rx.recv() {
                    let result = runtime.block_on(Self::dispatch_request(
                        base_url.as_str(),
                        public_bearer_secret.as_str(),
                        task.request,
                    ));
                    let _ = task.response_tx.send(result);
                }
            })
            .map_err(|error| format!("failed to spawn shared-channel sync worker: {error}"))?;

        Ok(Self { dispatch_tx })
    }

    fn authorization_header(public_bearer_secret: &str, tenant_id: &str) -> Result<String, String> {
        let now = current_unix_epoch_seconds();
        let mut claims = serde_json::json!({
            "tenant_id": tenant_id,
            "sub": PUBLIC_SHARED_CHANNEL_SYNC_ACTOR_ID,
            "actor_kind": "system",
            "permissions": [SHARED_CHANNEL_SYNC_PERMISSION],
            "scope": SHARED_CHANNEL_SYNC_PERMISSION,
            "iat": now,
            "nbf": now.saturating_sub(1),
            "exp": now.saturating_add(300),
        });
        if let Some(required_issuer) = resolve_public_bearer_required_issuer() {
            claims["iss"] = serde_json::json!(required_issuer);
        }
        if let Some(required_audience) = resolve_public_bearer_required_audience() {
            claims["aud"] = serde_json::json!(required_audience);
        }
        let token = encode_hs256_bearer_token(&claims, public_bearer_secret).map_err(|error| {
            format!("failed to encode public bearer token for shared-channel sync: {error}")
        })?;

        Ok(format!("Bearer {token}"))
    }

    async fn dispatch_request(
        base_url: &str,
        public_bearer_secret: &str,
        request: SharedChannelLinkedMemberSyncRequest,
    ) -> Result<(), String> {
        let timeout = resolve_shared_channel_sync_http_timeout();
        let payload = serde_json::to_vec(&serde_json::json!({
            "conversationId": request.conversation_id,
            "sharedChannelPolicyId": request.shared_channel_policy_id,
            "externalConnectionId": request.external_connection_id,
            "localActorId": request.local_actor_id,
            "localActorKind": request.local_actor_kind,
            "externalMemberId": request.external_member_id,
        }))
        .map(Bytes::from)
        .map_err(|error| format!("failed to encode shared-channel sync payload: {error}"))?;
        let authorization =
            Self::authorization_header(public_bearer_secret, request.tenant_id.as_str())?;
        let target = format!("{}{}", base_url, PUBLIC_SHARED_CHANNEL_SYNC_ROUTE);
        let request = HyperRequest::builder()
            .method(Method::POST)
            .uri(target.as_str())
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, authorization.as_str())
            .body(Full::new(payload))
            .map_err(|error| {
                format!("failed to build shared-channel sync request for {target}: {error}")
            })?;
        let client: Client<HttpConnector, Full<Bytes>> =
            Client::builder(TokioExecutor::new()).build(HttpConnector::new());
        let response = tokio::time::timeout(timeout, client.request(request))
            .await
            .map_err(|_| {
                format!(
                    "shared-channel sync request to {target} timed out after {}ms",
                    timeout.as_millis()
                )
            })?
            .map_err(|error| format!("shared-channel sync request to {target} failed: {error}"))?;
        let status = response.status();
        let body = read_shared_channel_sync_response_body_with_limit(
            response.into_body(),
            target.as_str(),
            timeout,
        )
        .await?;
        if status.is_success() {
            return Ok(());
        }

        let body_text = String::from_utf8_lossy(body.as_ref()).trim().to_owned();
        let detail = if body_text.is_empty() {
            "empty body".into()
        } else {
            body_text
        };
        Err(format!(
            "shared-channel sync endpoint {} returned {}: {}",
            PUBLIC_SHARED_CHANNEL_SYNC_ROUTE,
            status.as_u16(),
            detail
        ))
    }
}

fn validate_shared_channel_sync_target_base_url(base_url: &str) -> Result<String, String> {
    let base_url = base_url.trim().trim_end_matches('/').to_owned();
    if base_url.is_empty() {
        return Err("shared-channel sync target base url cannot be empty".into());
    }

    let uri = base_url.parse::<hyper::Uri>().map_err(|error| {
        format!("shared-channel sync target base url is invalid: {base_url}, error: {error}")
    })?;
    let scheme = uri.scheme_str().ok_or_else(|| {
        format!(
            "shared-channel sync target base url must include scheme (https://), got {base_url}"
        )
    })?;
    let host = uri.host().ok_or_else(|| {
        format!("shared-channel sync target base url must include host, got {base_url}")
    })?;

    match scheme {
        "https" => Ok(base_url),
        "http" => {
            if is_local_shared_channel_sync_host(host) || allow_insecure_shared_channel_sync_http()
            {
                return Ok(base_url);
            }

            Err(format!(
                "shared-channel sync target base url must use https:// in non-local environments, got {base_url} (set {}=true only for controlled local testing)",
                ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP_ENV
            ))
        }
        _ => Err(format!(
            "shared-channel sync target base url must use https:// (or http://localhost for local testing), got {base_url}"
        )),
    }
}

fn is_local_shared_channel_sync_host(host: &str) -> bool {
    matches!(host, "localhost" | "127.0.0.1" | "::1")
}

fn allow_insecure_shared_channel_sync_http() -> bool {
    std::env::var(ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP_ENV)
        .ok()
        .is_some_and(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
}

fn current_unix_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs()
}

fn resolve_shared_channel_sync_http_timeout() -> Duration {
    let timeout_millis = std::env::var(SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_DEFAULT_MILLIS);
    Duration::from_millis(timeout_millis)
}

fn resolve_shared_channel_sync_stale_reclaim_scheduler_config_from_env()
-> SharedChannelSyncStaleReclaimSchedulerConfig {
    let enabled = std::env::var(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED_ENV)
        .ok()
        .is_some_and(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        });
    let interval_millis =
        std::env::var(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS_ENV)
            .ok()
            .and_then(|value| value.trim().parse::<u64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_INTERVAL_MILLIS);
    SharedChannelSyncStaleReclaimSchedulerConfig {
        enabled,
        interval_millis,
    }
}

async fn read_shared_channel_sync_response_body_with_limit(
    mut body: Incoming,
    target: &str,
    timeout: Duration,
) -> Result<Bytes, String> {
    let mut output = Vec::new();
    while let Some(frame_result) =
        tokio::time::timeout(timeout, body.frame())
            .await
            .map_err(|_| {
                format!(
                    "shared-channel sync response from {target} timed out after {}ms",
                    timeout.as_millis()
                )
            })?
    {
        let frame = frame_result.map_err(|error| {
            format!("failed to read shared-channel sync response from {target}: {error}")
        })?;
        let Ok(data) = frame.into_data() else {
            continue;
        };
        if output.len().saturating_add(data.len()) > SHARED_CHANNEL_SYNC_RESPONSE_BODY_MAX_BYTES {
            return Err(format!(
                "shared-channel sync response from {target} exceeds maximum body size {} bytes",
                SHARED_CHANNEL_SYNC_RESPONSE_BODY_MAX_BYTES
            ));
        }
        output.extend_from_slice(data.as_ref());
    }
    Ok(Bytes::from(output))
}

impl SharedChannelLinkedMemberSyncTrigger for PublicSharedChannelLinkedMemberSyncTrigger {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String> {
        let (response_tx, response_rx) = std::sync::mpsc::channel::<Result<(), String>>();
        self.dispatch_tx
            .send(SharedChannelSyncDispatchTask {
                request,
                response_tx,
            })
            .map_err(|_| "shared-channel sync worker is unavailable".to_owned())?;
        response_rx
            .recv()
            .map_err(|_| "shared-channel sync worker dropped dispatch response".to_owned())?
    }
}

#[derive(Clone)]
enum SocialStateStore {
    Memory(Arc<Mutex<SocialControlState>>),
    File {
        file_path: Arc<PathBuf>,
        io_lock: Arc<Mutex<()>>,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
struct SocialControlState {
    friend_requests: BTreeMap<String, StoredFriendRequest>,
    friendships: BTreeMap<String, StoredFriendship>,
    user_blocks: BTreeMap<String, StoredUserBlock>,
    direct_chats: BTreeMap<String, StoredDirectChat>,
    external_connections: BTreeMap<String, StoredExternalConnection>,
    external_member_links: BTreeMap<String, StoredExternalMemberLink>,
    shared_channel_policies: BTreeMap<String, StoredSharedChannelPolicy>,
    pending_shared_channel_sync_requests: BTreeMap<String, PendingSharedChannelSyncRequest>,
    dead_letter_shared_channel_sync_requests: BTreeMap<String, PendingSharedChannelSyncRequest>,
}

struct SocialControlRuntime {
    state_store: SocialStateStore,
    commit_journal: Arc<dyn CommitJournal + Send + Sync>,
    state: RwLock<SocialControlState>,
    journal_path: Option<Arc<PathBuf>>,
    tx_marker_path: Option<Arc<PathBuf>>,
    snapshot_failpoint_path: Option<Arc<PathBuf>>,
    shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool,
}

#[derive(Clone, Debug, Default)]
struct PendingSharedChannelSyncClaimResult {
    claimed: usize,
    conflicted: usize,
    conflict_items: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialRuntimeFailpoints {
    fail_next_snapshot_save: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SocialTransactionMarkerStatus {
    PendingSnapshotRepair,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialTransactionMarker {
    status: SocialTransactionMarkerStatus,
    event_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredFriendRequest {
    friend_request: FriendRequest,
    commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug)]
struct SubmittedFriendRequest {
    friend_request: FriendRequest,
    latest_commit: CommitEnvelope,
    persistence: SocialWritePersistence,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredFriendship {
    friendship: Friendship,
    commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug)]
struct ActivatedFriendship {
    friendship: Friendship,
    latest_commit: CommitEnvelope,
    persistence: SocialWritePersistence,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredUserBlock {
    user_block: UserBlock,
    commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug)]
struct BlockedUser {
    user_block: UserBlock,
    latest_commit: CommitEnvelope,
    persistence: SocialWritePersistence,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredDirectChat {
    direct_chat: DirectChat,
    commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug)]
struct BoundDirectChat {
    direct_chat: DirectChat,
    latest_commit: CommitEnvelope,
    persistence: SocialWritePersistence,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredExternalConnection {
    external_connection: ExternalConnection,
    commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug)]
struct EstablishedExternalConnection {
    external_connection: ExternalConnection,
    latest_commit: CommitEnvelope,
    persistence: SocialWritePersistence,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredExternalMemberLink {
    external_member_link: ExternalMemberLink,
    commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug)]
struct BoundExternalMemberLink {
    external_member_link: ExternalMemberLink,
    latest_commit: CommitEnvelope,
    persistence: SocialWritePersistence,
    shared_channel_sync_requests: Vec<SharedChannelLinkedMemberSyncRequest>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct StoredSharedChannelPolicy {
    shared_channel_policy: SharedChannelPolicy,
    commits: Vec<CommitEnvelope>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PendingSharedChannelSyncRequest {
    request: SharedChannelLinkedMemberSyncRequest,
    failure_count: u32,
    last_error: String,
    owner_actor_id: Option<String>,
    owner_actor_kind: Option<String>,
    claimed_at: Option<String>,
    lease_expires_at: Option<String>,
}

impl PendingSharedChannelSyncRequest {
    fn is_owned_by(&self, actor_id: &str, actor_kind: &str) -> bool {
        self.owner_actor_id.as_deref() == Some(actor_id)
            && self.owner_actor_kind.as_deref() == Some(actor_kind)
    }

    fn is_claimed_by_other(&self, actor_id: &str, actor_kind: &str) -> bool {
        match (
            self.owner_actor_id.as_deref(),
            self.owner_actor_kind.as_deref(),
        ) {
            (Some(owner_actor_id), Some(owner_actor_kind)) => {
                owner_actor_id != actor_id || owner_actor_kind != actor_kind
            }
            (Some(owner_actor_id), None) => owner_actor_id != actor_id,
            (None, Some(owner_actor_kind)) => owner_actor_kind != actor_kind,
            (None, None) => false,
        }
    }

    fn assign_owner(&mut self, actor_id: &str, actor_kind: &str) {
        let already_owned = self.is_owned_by(actor_id, actor_kind);
        self.owner_actor_id = Some(actor_id.to_owned());
        self.owner_actor_kind = Some(actor_kind.to_owned());
        let claimed_at_epoch_millis = current_unix_epoch_millis();
        let claimed_at = format_unix_timestamp_millis(claimed_at_epoch_millis);
        let stale_same_owner_lease = already_owned
            && self
                .lease_expires_at
                .as_deref()
                .is_some_and(|lease_expires_at| lease_expires_at <= claimed_at.as_str());
        if !already_owned
            || self.claimed_at.is_none()
            || self.lease_expires_at.is_none()
            || stale_same_owner_lease
        {
            self.claimed_at = Some(claimed_at);
            self.lease_expires_at = Some(format_unix_timestamp_millis(
                claimed_at_epoch_millis
                    .saturating_add(SHARED_CHANNEL_SYNC_PENDING_LEASE_WINDOW_MILLIS),
            ));
        }
    }

    fn clear_owner(&mut self) {
        self.owner_actor_id = None;
        self.owner_actor_kind = None;
        self.claimed_at = None;
        self.lease_expires_at = None;
    }

    fn lease_status(&self, now: &str) -> SocialSharedChannelSyncLeaseStatus {
        match (
            self.owner_actor_id.as_deref(),
            self.owner_actor_kind.as_deref(),
            self.lease_expires_at.as_deref(),
        ) {
            (None, None, _) => SocialSharedChannelSyncLeaseStatus::Unclaimed,
            (Some(_), Some(_), Some(lease_expires_at)) => {
                if lease_expires_at > now {
                    SocialSharedChannelSyncLeaseStatus::Active
                } else {
                    SocialSharedChannelSyncLeaseStatus::Stale
                }
            }
            _ => SocialSharedChannelSyncLeaseStatus::Untracked,
        }
    }

    fn legacy_takeover_required_for(&self, actor_id: &str, actor_kind: &str) -> bool {
        self.is_claimed_by_other(actor_id, actor_kind) && self.lease_expires_at.is_none()
    }

    fn takeover_eligible_for(&self, actor_id: &str, actor_kind: &str, now: &str) -> bool {
        self.is_claimed_by_other(actor_id, actor_kind)
            && self
                .lease_expires_at
                .as_deref()
                .is_some_and(|lease_expires_at| lease_expires_at <= now)
    }

    fn blocks_foreign_takeover(&self, actor_id: &str, actor_kind: &str, now: &str) -> bool {
        self.is_claimed_by_other(actor_id, actor_kind)
            && self
                .lease_expires_at
                .as_deref()
                .is_some_and(|lease_expires_at| lease_expires_at > now)
    }

    fn auto_dispatch_eligible(&self, now: &str) -> bool {
        !matches!(
            self.lease_status(now),
            SocialSharedChannelSyncLeaseStatus::Active
                | SocialSharedChannelSyncLeaseStatus::Untracked
        )
    }
}

fn current_unix_epoch_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

#[derive(Clone, Debug)]
struct AppliedSharedChannelPolicy {
    shared_channel_policy: SharedChannelPolicy,
    latest_commit: CommitEnvelope,
    persistence: SocialWritePersistence,
    shared_channel_sync_requests: Vec<SharedChannelLinkedMemberSyncRequest>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialDerivedSnapshotStatus {
    Current,
    RepairRequired,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialWritePersistence {
    journal_authority: bool,
    snapshot_status: SocialDerivedSnapshotStatus,
}

#[derive(Clone, Debug)]
enum SocialCommittedEvent {
    FriendRequest {
        record: StoredFriendRequest,
        commit: CommitEnvelope,
    },
    Friendship {
        record: StoredFriendship,
        commit: CommitEnvelope,
    },
    UserBlock {
        record: StoredUserBlock,
        commit: CommitEnvelope,
    },
    DirectChat {
        record: StoredDirectChat,
        commit: CommitEnvelope,
    },
    ExternalConnection {
        record: StoredExternalConnection,
        commit: CommitEnvelope,
    },
    ExternalMemberLink {
        record: StoredExternalMemberLink,
        commit: CommitEnvelope,
    },
    SharedChannelPolicy {
        record: StoredSharedChannelPolicy,
        commit: CommitEnvelope,
    },
}

impl SocialCommittedEvent {
    fn commit(&self) -> &CommitEnvelope {
        match self {
            Self::FriendRequest { commit, .. }
            | Self::Friendship { commit, .. }
            | Self::UserBlock { commit, .. }
            | Self::DirectChat { commit, .. }
            | Self::ExternalConnection { commit, .. }
            | Self::ExternalMemberLink { commit, .. }
            | Self::SharedChannelPolicy { commit, .. } => commit,
        }
    }

    fn aggregate_label(&self) -> &'static str {
        match self {
            Self::FriendRequest { .. } => "friend_request",
            Self::Friendship { .. } => "friendship",
            Self::UserBlock { .. } => "user_block",
            Self::DirectChat { .. } => "direct_chat",
            Self::ExternalConnection { .. } => "external_connection",
            Self::ExternalMemberLink { .. } => "external_member_link",
            Self::SharedChannelPolicy { .. } => "shared_channel_policy",
        }
    }
}

impl SocialControlState {
    fn committed_event_keys(&self) -> BTreeSet<(String, String)> {
        let mut event_keys = BTreeSet::new();
        for record in self.friend_requests.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.friendships.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.user_blocks.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.direct_chats.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.external_connections.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.external_member_links.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        for record in self.shared_channel_policies.values() {
            event_keys.extend(
                record
                    .commits
                    .iter()
                    .map(|commit| (commit.tenant_id.clone(), commit.event_id.clone())),
            );
        }
        event_keys
    }

    fn committed_event(&self, tenant_id: &str, event_id: &str) -> Option<SocialCommittedEvent> {
        self.friend_requests
            .values()
            .find_map(|record| {
                find_committed_social_event(record.commits.as_slice(), tenant_id, event_id).map(
                    |commit| SocialCommittedEvent::FriendRequest {
                        record: record.clone(),
                        commit,
                    },
                )
            })
            .or_else(|| {
                self.friendships.values().find_map(|record| {
                    find_committed_social_event(record.commits.as_slice(), tenant_id, event_id).map(
                        |commit| SocialCommittedEvent::Friendship {
                            record: record.clone(),
                            commit,
                        },
                    )
                })
            })
            .or_else(|| {
                self.user_blocks.values().find_map(|record| {
                    find_committed_social_event(record.commits.as_slice(), tenant_id, event_id).map(
                        |commit| SocialCommittedEvent::UserBlock {
                            record: record.clone(),
                            commit,
                        },
                    )
                })
            })
            .or_else(|| {
                self.direct_chats.values().find_map(|record| {
                    find_committed_social_event(record.commits.as_slice(), tenant_id, event_id).map(
                        |commit| SocialCommittedEvent::DirectChat {
                            record: record.clone(),
                            commit,
                        },
                    )
                })
            })
            .or_else(|| {
                self.external_connections.values().find_map(|record| {
                    find_committed_social_event(record.commits.as_slice(), tenant_id, event_id).map(
                        |commit| SocialCommittedEvent::ExternalConnection {
                            record: record.clone(),
                            commit,
                        },
                    )
                })
            })
            .or_else(|| {
                self.external_member_links.values().find_map(|record| {
                    find_committed_social_event(record.commits.as_slice(), tenant_id, event_id).map(
                        |commit| SocialCommittedEvent::ExternalMemberLink {
                            record: record.clone(),
                            commit,
                        },
                    )
                })
            })
            .or_else(|| {
                self.shared_channel_policies.values().find_map(|record| {
                    find_committed_social_event(record.commits.as_slice(), tenant_id, event_id).map(
                        |commit| SocialCommittedEvent::SharedChannelPolicy {
                            record: record.clone(),
                            commit,
                        },
                    )
                })
            })
    }

    fn aggregate_counts(&self) -> SocialAggregateCountsResponse {
        SocialAggregateCountsResponse {
            friend_requests: self.friend_requests.len(),
            friendships: self.friendships.len(),
            user_blocks: self.user_blocks.len(),
            direct_chats: self.direct_chats.len(),
            external_connections: self.external_connections.len(),
            external_member_links: self.external_member_links.len(),
            shared_channel_policies: self.shared_channel_policies.len(),
            pending_shared_channel_sync_requests: self.pending_shared_channel_sync_requests.len(),
            dead_letter_shared_channel_sync_requests: self
                .dead_letter_shared_channel_sync_requests
                .len(),
        }
    }

    fn merge_pending_shared_channel_sync_requests_from(&mut self, other: &Self) {
        for (key, pending) in &other.pending_shared_channel_sync_requests {
            self.pending_shared_channel_sync_requests
                .entry(key.clone())
                .or_insert_with(|| pending.clone());
        }
    }

    fn merge_dead_letter_shared_channel_sync_requests_from(&mut self, other: &Self) {
        for (key, dead_letter) in &other.dead_letter_shared_channel_sync_requests {
            self.dead_letter_shared_channel_sync_requests
                .entry(key.clone())
                .or_insert_with(|| dead_letter.clone());
        }
    }

    fn pending_shared_channel_sync_count(&self) -> usize {
        self.pending_shared_channel_sync_requests.len()
    }

    fn dead_letter_shared_channel_sync_count(&self) -> usize {
        self.dead_letter_shared_channel_sync_requests.len()
    }

    fn pending_shared_channel_sync_requests(&self) -> Vec<PendingSharedChannelSyncRequest> {
        self.pending_shared_channel_sync_requests
            .values()
            .cloned()
            .collect()
    }

    fn pending_shared_channel_sync_requests_with_keys(
        &self,
    ) -> Vec<(String, PendingSharedChannelSyncRequest)> {
        self.pending_shared_channel_sync_requests
            .iter()
            .map(|(key, request)| (key.clone(), request.clone()))
            .collect()
    }

    fn dead_letter_shared_channel_sync_requests(
        &self,
    ) -> Vec<(String, PendingSharedChannelSyncRequest)> {
        self.dead_letter_shared_channel_sync_requests
            .iter()
            .map(|(key, request)| (key.clone(), request.clone()))
            .collect()
    }

    fn record_failed_shared_channel_sync_requests(
        &mut self,
        requests: &[SharedChannelLinkedMemberSyncRequest],
        error: &str,
        now: &str,
    ) -> bool {
        let mut changed = false;
        for request in requests {
            let key = shared_channel_sync_request_key(request);
            if let Some(dead_letter) = self.dead_letter_shared_channel_sync_requests.get_mut(&key) {
                dead_letter.request = request.clone();
                dead_letter.failure_count = dead_letter.failure_count.saturating_add(1);
                dead_letter.last_error = error.to_owned();
                changed = true;
                continue;
            }

            let failed_request =
                if let Some(pending) = self.pending_shared_channel_sync_requests.get_mut(&key) {
                    if pending.lease_status(now) == SocialSharedChannelSyncLeaseStatus::Stale {
                        pending.clear_owner();
                    }
                    pending.request = request.clone();
                    pending.failure_count = pending.failure_count.saturating_add(1);
                    pending.last_error = error.to_owned();
                    pending.clone()
                } else {
                    let pending = PendingSharedChannelSyncRequest {
                        request: request.clone(),
                        failure_count: 1,
                        last_error: error.to_owned(),
                        owner_actor_id: None,
                        owner_actor_kind: None,
                        claimed_at: None,
                        lease_expires_at: None,
                    };
                    self.pending_shared_channel_sync_requests
                        .insert(key.clone(), pending.clone());
                    pending
                };
            if failed_request.failure_count >= SHARED_CHANNEL_SYNC_DEAD_LETTER_FAILURE_THRESHOLD {
                let mut dead_letter_request = failed_request;
                dead_letter_request.clear_owner();
                self.pending_shared_channel_sync_requests
                    .remove(key.as_str());
                self.dead_letter_shared_channel_sync_requests
                    .insert(key, dead_letter_request);
            }
            changed = true;
        }
        changed
    }

    fn is_dead_letter_shared_channel_sync_request(
        &self,
        request: &SharedChannelLinkedMemberSyncRequest,
    ) -> bool {
        self.dead_letter_shared_channel_sync_requests
            .contains_key(shared_channel_sync_request_key(request).as_str())
    }

    fn remove_pending_shared_channel_sync_request(
        &mut self,
        request: &SharedChannelLinkedMemberSyncRequest,
    ) -> bool {
        self.pending_shared_channel_sync_requests
            .remove(shared_channel_sync_request_key(request).as_str())
            .is_some()
    }

    fn requeue_dead_letter_shared_channel_sync_requests(&mut self) -> usize {
        let request_keys = self
            .dead_letter_shared_channel_sync_requests
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        self.requeue_selected_dead_letter_shared_channel_sync_requests(&request_keys)
    }

    fn requeue_selected_dead_letter_shared_channel_sync_requests(
        &mut self,
        request_keys: &[String],
    ) -> usize {
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut requeued = 0usize;
        for key in request_keys {
            let Some(mut dead_letter) = self.dead_letter_shared_channel_sync_requests.remove(&key)
            else {
                continue;
            };
            dead_letter.failure_count = 0;
            dead_letter.clear_owner();
            self.pending_shared_channel_sync_requests
                .insert(key, dead_letter);
            requeued += 1;
        }
        requeued
    }

    fn claim_selected_pending_shared_channel_sync_requests(
        &mut self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
        now: &str,
    ) -> PendingSharedChannelSyncClaimResult {
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut result = PendingSharedChannelSyncClaimResult::default();
        for key in request_keys {
            let Some(pending) = self.pending_shared_channel_sync_requests.get_mut(&key) else {
                continue;
            };
            if pending.is_claimed_by_other(actor_id, actor_kind) {
                result.conflicted += 1;
                result
                    .conflict_items
                    .push(social_shared_channel_sync_conflict_details(
                        key.as_str(),
                        pending,
                        actor_id,
                        actor_kind,
                        now,
                    ));
                continue;
            }
            pending.assign_owner(actor_id, actor_kind);
            result.claimed += 1;
        }
        result
    }

    fn reclaim_stale_pending_shared_channel_sync_claims(&mut self, now: &str) -> usize {
        let mut reclaimed = 0usize;
        for pending in self.pending_shared_channel_sync_requests.values_mut() {
            if pending.lease_status(now) == SocialSharedChannelSyncLeaseStatus::Stale {
                pending.clear_owner();
                reclaimed += 1;
            }
        }
        reclaimed
    }

    fn release_selected_pending_shared_channel_sync_requests(
        &mut self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
    ) -> usize {
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut released = 0usize;
        for key in request_keys {
            let Some(pending) = self.pending_shared_channel_sync_requests.get_mut(&key) else {
                continue;
            };
            if !pending.is_owned_by(actor_id, actor_kind) {
                continue;
            }
            pending.clear_owner();
            released += 1;
        }
        released
    }

    fn takeover_selected_pending_shared_channel_sync_requests(
        &mut self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
    ) -> usize {
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut taken_over = 0usize;
        for key in request_keys {
            let Some(pending) = self.pending_shared_channel_sync_requests.get_mut(&key) else {
                continue;
            };
            if !pending.is_claimed_by_other(actor_id, actor_kind) {
                continue;
            }
            pending.assign_owner(actor_id, actor_kind);
            taken_over += 1;
        }
        taken_over
    }

    fn replay_commit_journal_file(&mut self, journal_path: &StdPath) -> Result<bool, String> {
        let commits = read_commit_journal_file(journal_path).map_err(|error| {
            format!(
                "failed to read social commit journal {}: {}",
                journal_path.display(),
                contract_error_message(error)
            )
        })?;
        let mut known_event_keys = self.committed_event_keys();
        let mut changed = false;
        for commit in commits {
            if !known_event_keys.insert((commit.tenant_id.clone(), commit.event_id.clone())) {
                continue;
            }
            self.apply_social_commit(commit)?;
            changed = true;
        }
        Ok(changed)
    }

    fn apply_social_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let event_type = commit.event_type.clone();
        match event_type.as_str() {
            "friend_request.submitted" => self.apply_friend_request_commit(commit),
            "friendship.activated" => self.apply_friendship_commit(commit),
            "user_block.blocked" => self.apply_user_block_commit(commit),
            "direct_chat.bound" => self.apply_direct_chat_commit(commit),
            "external_connection.established" => self.apply_external_connection_commit(commit),
            "external_member_link.bound" => self.apply_external_member_link_commit(commit),
            "shared_channel_policy.applied" => self.apply_shared_channel_policy_commit(commit),
            _ => Err(format!(
                "unsupported social replay event type {} for aggregate {}",
                event_type, commit.aggregate_id
            )),
        }
    }

    fn apply_friend_request_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: FriendRequestSubmittedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| {
                format!(
                    "failed to parse friend request replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target(
            &commit,
            AggregateType::FriendRequest,
            payload.request_id.as_str(),
        )?;
        normalize_user_pair(
            payload.requester_user_id.as_str(),
            payload.target_user_id.as_str(),
        )
        .map_err(|error| {
            format!(
                "failed to validate friend request replay payload for {}: {error}",
                commit.event_id
            )
        })?;

        let friend_request = FriendRequest {
            tenant_id: commit.tenant_id.clone(),
            request_id: payload.request_id.clone(),
            requester_user_id: payload.requester_user_id,
            target_user_id: payload.target_user_id,
            status: FriendRequestStatus::Pending,
            request_message: payload.request_message,
            expired_at: None,
            created_at: payload.requested_at.clone(),
            updated_at: payload.requested_at,
        };
        let record = self
            .friend_requests
            .entry(friend_request.request_id.clone())
            .or_insert_with(|| StoredFriendRequest {
                friend_request: friend_request.clone(),
                commits: Vec::new(),
            });
        record.friend_request = friend_request;
        record.commits.push(commit);
        Ok(())
    }

    fn apply_friendship_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: FriendshipActivatedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| {
                format!(
                    "failed to parse friendship replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target(
            &commit,
            AggregateType::Friendship,
            payload.friendship_id.as_str(),
        )?;
        let pair = normalize_user_pair(payload.user_low_id.as_str(), payload.user_high_id.as_str())
            .map_err(|error| {
                format!(
                    "failed to validate friendship replay payload for {}: {error}",
                    commit.event_id
                )
            })?;

        let friendship = Friendship {
            tenant_id: commit.tenant_id.clone(),
            friendship_id: payload.friendship_id.clone(),
            user_low_id: pair.user_low_id,
            user_high_id: pair.user_high_id,
            initiator_user_id: payload.initiator_user_id,
            status: FriendshipStatus::Active,
            established_at: Some(payload.established_at.clone()),
            updated_at: payload.established_at,
        };
        let record = self
            .friendships
            .entry(friendship.friendship_id.clone())
            .or_insert_with(|| StoredFriendship {
                friendship: friendship.clone(),
                commits: Vec::new(),
            });
        record.friendship = friendship;
        record.commits.push(commit);
        Ok(())
    }

    fn apply_user_block_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: UserBlockedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse user block replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target(
            &commit,
            AggregateType::UserBlock,
            payload.block_id.as_str(),
        )?;
        normalize_user_pair(
            payload.blocker_user_id.as_str(),
            payload.blocked_user_id.as_str(),
        )
        .map_err(|error| {
            format!(
                "failed to validate user block replay payload for {}: {error}",
                commit.event_id
            )
        })?;
        let scope: BlockScope = parse_social_replay_enum("scope", payload.scope.as_str())?;
        if matches!(scope, BlockScope::DirectChat) && payload.direct_chat_id.is_none() {
            return Err(format!(
                "user block replay payload for {} is missing directChatId for direct_chat scope",
                commit.event_id
            ));
        }

        let user_block = UserBlock {
            tenant_id: commit.tenant_id.clone(),
            block_id: payload.block_id.clone(),
            blocker_user_id: payload.blocker_user_id,
            blocked_user_id: payload.blocked_user_id,
            scope,
            status: UserBlockStatus::Active,
            direct_chat_id: payload.direct_chat_id,
            expires_at: payload.expires_at,
            created_at: payload.effective_at.clone(),
            updated_at: payload.effective_at,
        };
        let record = self
            .user_blocks
            .entry(user_block.block_id.clone())
            .or_insert_with(|| StoredUserBlock {
                user_block: user_block.clone(),
                commits: Vec::new(),
            });
        record.user_block = user_block;
        record.commits.push(commit);
        Ok(())
    }

    fn apply_direct_chat_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: DirectChatBoundPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| {
                format!(
                    "failed to parse direct chat replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target(
            &commit,
            AggregateType::DirectChat,
            payload.direct_chat_id.as_str(),
        )?;
        let pair = normalize_actor_pair(
            payload.left_actor_id.as_str(),
            payload.right_actor_id.as_str(),
        )
        .map_err(|error| {
            format!(
                "failed to validate direct chat replay payload for {}: {error}",
                commit.event_id
            )
        })?;
        if pair.pair_hash != payload.pair_hash {
            return Err(format!(
                "direct chat replay payload for {} has mismatched pairHash {}, expected {}",
                commit.event_id, payload.pair_hash, pair.pair_hash
            ));
        }

        let direct_chat = DirectChat {
            tenant_id: commit.tenant_id.clone(),
            direct_chat_id: payload.direct_chat_id.clone(),
            left_actor_id: pair.left_actor_id,
            right_actor_id: pair.right_actor_id,
            pair_hash: pair.pair_hash,
            status: DirectChatStatus::Active,
            conversation_id: Some(payload.conversation_id),
            created_at: payload.bound_at.clone(),
            updated_at: payload.bound_at,
        };
        let record = self
            .direct_chats
            .entry(direct_chat.direct_chat_id.clone())
            .or_insert_with(|| StoredDirectChat {
                direct_chat: direct_chat.clone(),
                commits: Vec::new(),
            });
        record.direct_chat = direct_chat;
        record.commits.push(commit);
        Ok(())
    }

    fn apply_external_connection_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: ExternalConnectionEstablishedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse external connection replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target(
            &commit,
            AggregateType::ExternalConnection,
            payload.connection_id.as_str(),
        )?;
        ensure_cross_tenant_connection(
            commit.tenant_id.as_str(),
            payload.external_tenant_id.as_str(),
        )
        .map_err(|error| {
            format!(
                "failed to validate external connection replay payload for {}: {error}",
                commit.event_id
            )
        })?;
        let connection_kind: ExternalConnectionKind =
            parse_social_replay_enum("connectionKind", payload.connection_kind.as_str())?;

        let external_connection = ExternalConnection {
            tenant_id: commit.tenant_id.clone(),
            connection_id: payload.connection_id.clone(),
            external_tenant_id: payload.external_tenant_id,
            external_org_name: payload.external_org_name,
            connection_kind,
            status: ExternalConnectionStatus::Active,
            established_at: payload.established_at.clone(),
            updated_at: payload.established_at,
        };
        let record = self
            .external_connections
            .entry(external_connection.connection_id.clone())
            .or_insert_with(|| StoredExternalConnection {
                external_connection: external_connection.clone(),
                commits: Vec::new(),
            });
        record.external_connection = external_connection;
        record.commits.push(commit);
        Ok(())
    }

    fn apply_external_member_link_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: ExternalMemberLinkBoundPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| {
                format!(
                    "failed to parse external member link replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target(
            &commit,
            AggregateType::ExternalMemberLink,
            payload.link_id.as_str(),
        )?;
        let connection = self
            .external_connections
            .get(payload.connection_id.as_str())
            .ok_or_else(|| {
                format!(
                    "external member link replay payload for {} references missing connection {}",
                    commit.event_id, payload.connection_id
                )
            })?;
        if connection.external_connection.tenant_id != commit.tenant_id {
            return Err(format!(
                "external member link replay payload for {} crosses tenant boundary on connection {}",
                commit.event_id, payload.connection_id
            ));
        }

        let external_member_link = ExternalMemberLink {
            tenant_id: commit.tenant_id.clone(),
            link_id: payload.link_id.clone(),
            connection_id: payload.connection_id,
            local_actor_id: payload.local_actor_id,
            local_actor_kind: payload.local_actor_kind,
            external_member_id: payload.external_member_id,
            external_display_name: payload.external_display_name,
            status: ExternalMemberLinkStatus::Active,
            linked_at: payload.linked_at.clone(),
            updated_at: payload.linked_at,
        };
        let record = self
            .external_member_links
            .entry(external_member_link.link_id.clone())
            .or_insert_with(|| StoredExternalMemberLink {
                external_member_link: external_member_link.clone(),
                commits: Vec::new(),
            });
        record.external_member_link = external_member_link;
        record.commits.push(commit);
        Ok(())
    }

    fn apply_shared_channel_policy_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: SharedChannelPolicyAppliedPayload =
            serde_json::from_str(commit.payload.as_str()).map_err(|error| {
                format!(
                    "failed to parse shared channel policy replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        validate_social_commit_target(
            &commit,
            AggregateType::SharedChannelPolicy,
            payload.policy_id.as_str(),
        )?;
        if payload.policy_version == 0 {
            return Err(format!(
                "shared channel policy replay payload for {} has invalid policyVersion 0",
                commit.event_id
            ));
        }
        let connection = self
            .external_connections
            .get(payload.connection_id.as_str())
            .ok_or_else(|| {
                format!(
                    "shared channel policy replay payload for {} references missing connection {}",
                    commit.event_id, payload.connection_id
                )
            })?;
        if connection.external_connection.tenant_id != commit.tenant_id {
            return Err(format!(
                "shared channel policy replay payload for {} crosses tenant boundary on connection {}",
                commit.event_id, payload.connection_id
            ));
        }

        let shared_channel_policy = SharedChannelPolicy {
            tenant_id: commit.tenant_id.clone(),
            policy_id: payload.policy_id.clone(),
            connection_id: payload.connection_id,
            channel_id: payload.channel_id,
            conversation_id: payload.conversation_id,
            policy_version: payload.policy_version,
            history_visibility: payload.history_visibility,
            status: SharedChannelPolicyStatus::Active,
            applied_at: payload.applied_at.clone(),
            updated_at: payload.applied_at,
        };
        let record = self
            .shared_channel_policies
            .entry(shared_channel_policy.policy_id.clone())
            .or_insert_with(|| StoredSharedChannelPolicy {
                shared_channel_policy: shared_channel_policy.clone(),
                commits: Vec::new(),
            });
        record.shared_channel_policy = shared_channel_policy;
        record.commits.push(commit);
        Ok(())
    }
}

fn validate_social_commit_target(
    commit: &CommitEnvelope,
    aggregate_type: AggregateType,
    aggregate_id: &str,
) -> Result<(), String> {
    if commit.aggregate_type != aggregate_type {
        return Err(format!(
            "social replay commit {} has aggregate type {:?}, expected {:?}",
            commit.event_id, commit.aggregate_type, aggregate_type
        ));
    }
    if commit.aggregate_id != aggregate_id {
        return Err(format!(
            "social replay commit {} has aggregate id {}, expected {}",
            commit.event_id, commit.aggregate_id, aggregate_id
        ));
    }
    if commit.scope_id != aggregate_id {
        return Err(format!(
            "social replay commit {} has scope id {}, expected {}",
            commit.event_id, commit.scope_id, aggregate_id
        ));
    }
    Ok(())
}

fn parse_social_replay_enum<T>(field_name: &str, value: &str) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_value(serde_json::Value::String(value.to_string())).map_err(|error| {
        format!("failed to parse social replay enum {field_name}={value}: {error}")
    })
}

fn find_committed_social_event(
    commits: &[CommitEnvelope],
    tenant_id: &str,
    event_id: &str,
) -> Option<CommitEnvelope> {
    commits
        .iter()
        .find(|commit| commit.tenant_id == tenant_id && commit.event_id == event_id)
        .cloned()
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MigrateRoutesRequest {
    target_node_id: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderBindingsQuery {
    tenant_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpsertProviderBindingPolicyRequest {
    tenant_id: Option<String>,
    domain: ProviderDomain,
    plugin_id: String,
    expected_base_version: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPolicyRollbackRequest {
    target_version: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubmitFriendRequestRequest {
    request_id: String,
    event_id: String,
    requester_user_id: String,
    target_user_id: String,
    request_message: Option<String>,
    requested_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivateFriendshipRequest {
    friendship_id: String,
    event_id: String,
    initiator_user_id: String,
    peer_user_id: String,
    direct_chat_id: Option<String>,
    established_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BlockUserRequest {
    block_id: String,
    event_id: String,
    blocker_user_id: String,
    blocked_user_id: String,
    scope: BlockScope,
    direct_chat_id: Option<String>,
    expires_at: Option<String>,
    effective_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BindDirectChatRequest {
    direct_chat_id: String,
    event_id: String,
    left_actor_id: String,
    right_actor_id: String,
    conversation_id: String,
    bound_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EstablishExternalConnectionRequest {
    connection_id: String,
    event_id: String,
    external_tenant_id: String,
    external_org_name: Option<String>,
    connection_kind: ExternalConnectionKind,
    established_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BindExternalMemberLinkRequest {
    link_id: String,
    event_id: String,
    connection_id: String,
    local_actor_id: String,
    local_actor_kind: String,
    external_member_id: String,
    external_display_name: Option<String>,
    linked_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApplySharedChannelPolicyRequest {
    policy_id: String,
    event_id: String,
    connection_id: String,
    channel_id: String,
    conversation_id: Option<String>,
    policy_version: u64,
    history_visibility: String,
    applied_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPolicyDiffQuery {
    from_version: u64,
    to_version: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProtocolRegistryResponse {
    protocol_version: String,
    bindings: Vec<String>,
    codecs: Vec<String>,
    schemas: Vec<ProtocolSchemaResponse>,
    compatibility_matrix: Vec<ClientCompatibilityResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProtocolGovernanceResponse {
    capability_profile: CapabilityProfileResponse,
    quota_profile: QuotaProfileResponse,
    rollout_policy: RolloutPolicyResponse,
    kill_switch: KillSwitchResponse,
    effective_snapshot: EffectiveProtocolSnapshotResponse,
    business_policy_vocabulary: BusinessPolicyVocabularyResponse,
    sdk_compatibility_baseline: SdkCompatibilityBaselineResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProtocolSchemaResponse {
    schema: String,
    kind: String,
    stage: String,
    binding_protocols: Vec<String>,
    required_capabilities: Vec<String>,
    supported_consumers: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientCompatibilityResponse {
    client_type: String,
    minimum_protocol_version: String,
    supported_bindings: Vec<String>,
    supported_codecs: Vec<String>,
    supported_capabilities: Vec<String>,
    blocked_experimental_capabilities: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CapabilityProfileResponse {
    profile_id: String,
    release_channel: String,
    enabled_capabilities: Vec<String>,
    experimental_capabilities: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct QuotaProfileResponse {
    profile_id: String,
    max_concurrent_sessions_per_tenant: u32,
    max_subscriptions_per_session: u32,
    max_inflight_messages: u32,
    max_payload_bytes: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RolloutPolicyResponse {
    policy_id: String,
    release_channel: String,
    traffic_percent: u8,
    cell_selector: String,
    region_selector: String,
    operator_override: bool,
    tenant_allowlist: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct KillSwitchResponse {
    rule_id: String,
    active: bool,
    reason: String,
    disabled_capabilities: Vec<String>,
    disabled_bindings: Vec<String>,
    disabled_codecs: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EffectiveProtocolSnapshotResponse {
    protocol_version: String,
    release_channel: String,
    enabled_capabilities: Vec<String>,
    allowed_bindings: Vec<String>,
    allowed_codecs: Vec<String>,
    quota_profile_id: String,
    kill_switch_active: bool,
    precedence: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BusinessPolicyVocabularyResponse {
    policy_version_field: String,
    capability_flags_field: String,
    history_visibility_field: String,
    history_visibility_modes: Vec<String>,
    retention_policy_ref_field: String,
    retention_policy_scopes: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SdkCompatibilityBaselineResponse {
    app_sdk_facade: &'static str,
    admin_sdk_facade: &'static str,
    matrix_client_types: Vec<String>,
    protocol_registry_path: &'static str,
    protocol_governance_path: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderBindingsResponse {
    status: ProviderSurfaceReadStatus,
    interface_version: String,
    tenant_id: Option<String>,
    effective_bindings: Vec<EffectiveProviderBinding>,
    precedence: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderBindingCommitResponse {
    status: ProviderPolicyResultStatus,
    applied: bool,
    interface_version: String,
    tenant_id: Option<String>,
    current_version: u64,
    committed_binding: EffectiveProviderBinding,
    diff: ProviderPolicyDiff,
    effective_bindings: Vec<EffectiveProviderBinding>,
    precedence: Vec<String>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ProviderSurfaceReadStatus {
    Registry,
    Bindings,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderRegistrySnapshotResponse {
    status: ProviderSurfaceReadStatus,
    #[serde(flatten)]
    snapshot: ProviderRegistrySnapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ProviderPolicyReadStatus {
    History,
    Diff,
    RolledBack,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPolicyHistoryResponse {
    status: ProviderPolicyReadStatus,
    #[serde(flatten)]
    history: ProviderPolicyHistory,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderPolicyDiffResponse {
    status: ProviderPolicyReadStatus,
    #[serde(flatten)]
    diff: ProviderPolicyDiff,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialFriendRequestWriteStatus {
    Submitted,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialFriendRequestReadStatus {
    Snapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialFriendshipWriteStatus {
    Activated,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialFriendshipReadStatus {
    Snapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialUserBlockWriteStatus {
    Blocked,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialUserBlockReadStatus {
    Snapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialDirectChatWriteStatus {
    Bound,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialDirectChatReadStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialFriendRequestCommitResponse {
    status: SocialFriendRequestWriteStatus,
    friend_request: FriendRequest,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialFriendRequestSnapshotResponse {
    status: SocialFriendRequestReadStatus,
    friend_request: FriendRequest,
    commits: Vec<CommitEnvelopeResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialFriendshipCommitResponse {
    status: SocialFriendshipWriteStatus,
    friendship: Friendship,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialFriendshipSnapshotResponse {
    status: SocialFriendshipReadStatus,
    friendship: Friendship,
    commits: Vec<CommitEnvelopeResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialUserBlockCommitResponse {
    status: SocialUserBlockWriteStatus,
    user_block: UserBlock,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialUserBlockSnapshotResponse {
    status: SocialUserBlockReadStatus,
    user_block: UserBlock,
    commits: Vec<CommitEnvelopeResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialDirectChatCommitResponse {
    status: SocialDirectChatWriteStatus,
    direct_chat: DirectChat,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialDirectChatSnapshotResponse {
    status: SocialDirectChatReadStatus,
    direct_chat: DirectChat,
    commits: Vec<CommitEnvelopeResponse>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialExternalConnectionWriteStatus {
    Established,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialExternalConnectionReadStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialExternalConnectionCommitResponse {
    status: SocialExternalConnectionWriteStatus,
    external_connection: ExternalConnection,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialExternalConnectionSnapshotResponse {
    status: SocialExternalConnectionReadStatus,
    external_connection: ExternalConnection,
    commits: Vec<CommitEnvelopeResponse>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialExternalMemberLinkWriteStatus {
    Bound,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialExternalMemberLinkReadStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialExternalMemberLinkCommitResponse {
    status: SocialExternalMemberLinkWriteStatus,
    external_member_link: ExternalMemberLink,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialExternalMemberLinkSnapshotResponse {
    status: SocialExternalMemberLinkReadStatus,
    external_member_link: ExternalMemberLink,
    commits: Vec<CommitEnvelopeResponse>,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialSharedChannelPolicyWriteStatus {
    Applied,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialSharedChannelPolicyReadStatus {
    Snapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialRuntimeRepairStatus {
    Repaired,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncRepairStatus {
    Noop,
    TriggerUnconfigured,
    Pending,
    PartiallyRepaired,
    Repaired,
    DeadLettered,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncDeadLetterRequeueStatus {
    Noop,
    Requeued,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncDeadLetterInventoryStatus {
    Snapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncPendingInventoryStatus {
    Snapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncPendingClaimStatus {
    Noop,
    Claimed,
    PartiallyClaimed,
    Conflict,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncPendingReleaseStatus {
    Noop,
    Released,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncPendingTakeoverStatus {
    Noop,
    TakenOver,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncPendingStaleReclaimStatus {
    Noop,
    Reclaimed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncLeaseStatus {
    Unclaimed,
    Active,
    Stale,
    Untracked,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncTargetedRepublishStatus {
    Noop,
    TriggerUnconfigured,
    Pending,
    PartiallyRepublished,
    Republished,
    DeadLettered,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialSharedChannelPolicyCommitResponse {
    status: SocialSharedChannelPolicyWriteStatus,
    shared_channel_policy: SharedChannelPolicy,
    latest_commit: CommitEnvelopeResponse,
    persistence: SocialWritePersistence,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialSharedChannelPolicySnapshotResponse {
    status: SocialSharedChannelPolicyReadStatus,
    shared_channel_policy: SharedChannelPolicy,
    commits: Vec<CommitEnvelopeResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAggregateCountsResponse {
    pub friend_requests: usize,
    pub friendships: usize,
    pub user_blocks: usize,
    pub direct_chats: usize,
    pub external_connections: usize,
    pub external_member_links: usize,
    pub shared_channel_policies: usize,
    pub pending_shared_channel_sync_requests: usize,
    pub dead_letter_shared_channel_sync_requests: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialRuntimeRepairResponse {
    pub status: SocialRuntimeRepairStatus,
    pub journal_authority: bool,
    pub snapshot_updated: bool,
    pub transaction_marker_cleared: bool,
    pub aggregate_counts: SocialAggregateCountsResponse,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncRepairResponse {
    pub status: SocialSharedChannelSyncRepairStatus,
    pub pending_before: usize,
    pub attempted: usize,
    pub dispatched: usize,
    pub failed: usize,
    pub reclaimed: usize,
    pub pending_after: usize,
    pub dead_letter_before: usize,
    pub dead_lettered: usize,
    pub dead_letter_after: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeadLetterRequeueResponse {
    pub status: SocialSharedChannelSyncDeadLetterRequeueStatus,
    pub pending_before: usize,
    pub dead_letter_before: usize,
    pub requeued: usize,
    pub pending_after: usize,
    pub dead_letter_after: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncInventoryItemResponse {
    pub request_key: String,
    pub request: SharedChannelLinkedMemberSyncRequest,
    pub failure_count: u32,
    pub last_error: String,
    pub owner_actor_id: Option<String>,
    pub owner_actor_kind: Option<String>,
    pub claimed_at: Option<String>,
    pub lease_expires_at: Option<String>,
    pub lease_status: SocialSharedChannelSyncLeaseStatus,
    pub takeover_eligible: bool,
    pub legacy_takeover_required: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeadLetterInventoryResponse {
    pub status: SocialSharedChannelSyncDeadLetterInventoryStatus,
    pub dead_letter_count: usize,
    pub items: Vec<SocialSharedChannelSyncInventoryItemResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncPendingInventoryResponse {
    pub status: SocialSharedChannelSyncPendingInventoryStatus,
    pub pending_count: usize,
    pub items: Vec<SocialSharedChannelSyncInventoryItemResponse>,
}

fn social_shared_channel_sync_inventory_item_response(
    request_key: String,
    request: PendingSharedChannelSyncRequest,
    actor_id: &str,
    actor_kind: &str,
    can_takeover: bool,
    now: &str,
) -> SocialSharedChannelSyncInventoryItemResponse {
    let lease_status = request.lease_status(now);
    let takeover_eligible =
        can_takeover && request.takeover_eligible_for(actor_id, actor_kind, now);
    let legacy_takeover_required =
        can_takeover && request.legacy_takeover_required_for(actor_id, actor_kind);
    SocialSharedChannelSyncInventoryItemResponse {
        request_key,
        request: request.request,
        failure_count: request.failure_count,
        last_error: request.last_error,
        owner_actor_id: request.owner_actor_id,
        owner_actor_kind: request.owner_actor_kind,
        claimed_at: request.claimed_at,
        lease_expires_at: request.lease_expires_at,
        lease_status,
        takeover_eligible,
        legacy_takeover_required,
    }
}

fn social_shared_channel_sync_conflict_details(
    request_key: &str,
    request: &PendingSharedChannelSyncRequest,
    actor_id: &str,
    actor_kind: &str,
    now: &str,
) -> serde_json::Value {
    let suggested_action = if request.legacy_takeover_required_for(actor_id, actor_kind) {
        "takeover_with_legacy_override"
    } else if request.takeover_eligible_for(actor_id, actor_kind, now) {
        "takeover_pending_request"
    } else {
        "wait_for_owner_release_or_expiry"
    };
    serde_json::json!({
        "requestKey": request_key,
        "ownerActorId": request.owner_actor_id.as_deref(),
        "leaseExpiresAt": request.lease_expires_at.as_deref(),
        "leaseStatus": request.lease_status(now),
        "takeoverEligible": request.takeover_eligible_for(actor_id, actor_kind, now),
        "legacyTakeoverRequired": request.legacy_takeover_required_for(actor_id, actor_kind),
        "suggestedAction": suggested_action
    })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncPendingClaimResponse {
    pub status: SocialSharedChannelSyncPendingClaimStatus,
    pub pending_before: usize,
    pub requested: usize,
    pub claimed: usize,
    pub conflicted: usize,
    pub conflict_items: Vec<serde_json::Value>,
    pub pending_after: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncPendingReleaseResponse {
    pub status: SocialSharedChannelSyncPendingReleaseStatus,
    pub pending_before: usize,
    pub requested: usize,
    pub released: usize,
    pub conflicted: usize,
    pub pending_after: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncPendingTakeoverResponse {
    pub status: SocialSharedChannelSyncPendingTakeoverStatus,
    pub pending_before: usize,
    pub requested: usize,
    pub taken_over: usize,
    pub pending_after: usize,
    pub legacy_override_used: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncPendingStaleReclaimResponse {
    pub status: SocialSharedChannelSyncPendingStaleReclaimStatus,
    pub pending_before: usize,
    pub reclaimed: usize,
    pub pending_after: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeadLetterTargetedRequeueResponse {
    pub status: SocialSharedChannelSyncDeadLetterRequeueStatus,
    pub pending_before: usize,
    pub dead_letter_before: usize,
    pub requested: usize,
    pub requeued: usize,
    pub pending_after: usize,
    pub dead_letter_after: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncTargetedRepublishResponse {
    pub status: SocialSharedChannelSyncTargetedRepublishStatus,
    pub pending_before: usize,
    pub requested: usize,
    pub attempted: usize,
    pub dispatched: usize,
    pub failed: usize,
    pub pending_after: usize,
    pub dead_letter_before: usize,
    pub dead_lettered: usize,
    pub dead_letter_after: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialSharedChannelSyncDeadLetterTargetedRequeueRequest {
    request_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialSharedChannelSyncPendingTargetedClaimRequest {
    request_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialSharedChannelSyncPendingTargetedReleaseRequest {
    request_keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialSharedChannelSyncPendingTargetedTakeoverRequest {
    request_keys: Vec<String>,
    #[serde(default)]
    allow_legacy_untracked: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialSharedChannelSyncTargetedRepublishRequest {
    request_keys: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CommitEnvelopeResponse {
    event_id: String,
    tenant_id: String,
    event_type: String,
    event_version: u16,
    aggregate_type: String,
    aggregate_id: String,
    scope_type: String,
    scope_id: String,
    ordering_key: String,
    ordering_seq: u64,
    causation_id: Option<String>,
    correlation_id: Option<String>,
    idempotency_key: Option<String>,
    actor: EventActorResponse,
    occurred_at: String,
    committed_at: String,
    payload_schema: Option<String>,
    payload: String,
    retention_class: String,
    audit_class: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EventActorResponse {
    actor_id: String,
    actor_kind: String,
    actor_session_id: Option<String>,
}

impl SocialStateStore {
    fn memory() -> Self {
        Self::Memory(Arc::new(Mutex::new(SocialControlState::default())))
    }

    fn file(file_path: impl Into<PathBuf>) -> Self {
        Self::File {
            file_path: Arc::new(file_path.into()),
            io_lock: Arc::new(Mutex::new(())),
        }
    }

    fn load(&self) -> Result<SocialControlState, String> {
        match self {
            Self::Memory(state) => Ok(state
                .lock()
                .expect("social state store should lock")
                .clone()),
            Self::File { file_path, io_lock } => {
                let _guard = io_lock.lock().expect("social state store should lock");
                if !file_path.exists() {
                    return Ok(SocialControlState::default());
                }
                let content = fs::read_to_string(file_path.as_path()).map_err(|error| {
                    format!(
                        "failed to read social state file {}: {error}",
                        file_path.display()
                    )
                })?;
                if content.trim().is_empty() {
                    return Ok(SocialControlState::default());
                }
                serde_json::from_str(&content).map_err(|error| {
                    format!(
                        "failed to parse social state file {}: {error}",
                        file_path.display()
                    )
                })
            }
        }
    }

    fn save(&self, state: &SocialControlState) -> Result<(), String> {
        match self {
            Self::Memory(slot) => {
                *slot.lock().expect("social state store should lock") = state.clone();
                Ok(())
            }
            Self::File { file_path, io_lock } => {
                let _guard = io_lock.lock().expect("social state store should lock");
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).map_err(|error| {
                        format!(
                            "failed to create social state parent directory {}: {error}",
                            parent.display()
                        )
                    })?;
                }
                let payload = serde_json::to_string_pretty(state)
                    .map_err(|error| format!("failed to serialize social state: {error}"))?;
                fs::write(file_path.as_path(), payload).map_err(|error| {
                    format!(
                        "failed to write social state file {}: {error}",
                        file_path.display()
                    )
                })
            }
        }
    }
}

fn non_empty_string(value: Option<&str>) -> Option<String> {
    value.and_then(|value| {
        if value.trim().is_empty() {
            None
        } else {
            Some(value.to_owned())
        }
    })
}

fn shared_channel_sync_requests_for_external_member_link(
    state: &SocialControlState,
    link: &ExternalMemberLink,
) -> Vec<SharedChannelLinkedMemberSyncRequest> {
    let Some(local_actor_kind) = non_empty_string(link.local_actor_kind.as_deref()) else {
        return Vec::new();
    };
    state
        .shared_channel_policies
        .values()
        .filter_map(|record| {
            let policy = &record.shared_channel_policy;
            let conversation_id = non_empty_string(policy.conversation_id.as_deref())?;
            if policy.tenant_id != link.tenant_id
                || !policy.status.is_active()
                || policy.connection_id != link.connection_id
                || policy.history_visibility != "shared"
            {
                return None;
            }

            Some(SharedChannelLinkedMemberSyncRequest {
                tenant_id: link.tenant_id.clone(),
                conversation_id,
                shared_channel_policy_id: policy.policy_id.clone(),
                external_connection_id: link.connection_id.clone(),
                local_actor_id: link.local_actor_id.clone(),
                local_actor_kind: local_actor_kind.clone(),
                external_member_id: link.external_member_id.clone(),
            })
        })
        .collect()
}

fn shared_channel_sync_requests_for_shared_channel_policy(
    state: &SocialControlState,
    policy: &SharedChannelPolicy,
) -> Vec<SharedChannelLinkedMemberSyncRequest> {
    let Some(conversation_id) = non_empty_string(policy.conversation_id.as_deref()) else {
        return Vec::new();
    };
    if !policy.status.is_active() || policy.history_visibility != "shared" {
        return Vec::new();
    }

    state
        .external_member_links
        .values()
        .filter_map(|record| {
            let link = &record.external_member_link;
            let local_actor_kind = non_empty_string(link.local_actor_kind.as_deref())?;
            if link.tenant_id != policy.tenant_id
                || !link.status.is_active()
                || link.connection_id != policy.connection_id
            {
                return None;
            }

            Some(SharedChannelLinkedMemberSyncRequest {
                tenant_id: policy.tenant_id.clone(),
                conversation_id: conversation_id.clone(),
                shared_channel_policy_id: policy.policy_id.clone(),
                external_connection_id: policy.connection_id.clone(),
                local_actor_id: link.local_actor_id.clone(),
                local_actor_kind,
                external_member_id: link.external_member_id.clone(),
            })
        })
        .collect()
}

fn shared_channel_sync_request_key(request: &SharedChannelLinkedMemberSyncRequest) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        request.tenant_id,
        request.conversation_id,
        request.shared_channel_policy_id,
        request.external_connection_id,
        request.local_actor_id,
        request.local_actor_kind,
        request.external_member_id
    )
}

fn write_social_transaction_marker(
    marker_path: &StdPath,
    marker: &SocialTransactionMarker,
) -> Result<(), String> {
    if let Some(parent) = marker_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "failed to create social transaction marker parent directory {}: {error}",
                parent.display()
            )
        })?;
    }
    let payload = serde_json::to_string_pretty(marker)
        .map_err(|error| format!("failed to serialize social transaction marker: {error}"))?;
    fs::write(marker_path, payload).map_err(|error| {
        format!(
            "failed to write social transaction marker file {}: {error}",
            marker_path.display()
        )
    })
}

fn clear_social_transaction_marker(marker_path: &StdPath) -> Result<bool, String> {
    if !marker_path.exists() {
        return Ok(false);
    }
    fs::remove_file(marker_path).map_err(|error| {
        format!(
            "failed to remove social transaction marker file {}: {error}",
            marker_path.display()
        )
    })?;
    Ok(true)
}

impl Default for SocialControlRuntime {
    fn default() -> Self {
        Self::new(
            SocialStateStore::memory(),
            Arc::new(MemoryCommitJournal::default()),
        )
    }
}

impl SocialControlRuntime {
    fn new(
        state_store: SocialStateStore,
        commit_journal: Arc<dyn CommitJournal + Send + Sync>,
    ) -> Self {
        Self::new_with_snapshot_failpoint(state_store, commit_journal, None)
    }

    fn new_with_snapshot_failpoint(
        state_store: SocialStateStore,
        commit_journal: Arc<dyn CommitJournal + Send + Sync>,
        snapshot_failpoint_path: Option<PathBuf>,
    ) -> Self {
        let state = state_store
            .load()
            .unwrap_or_else(|error| panic!("failed to load control-plane social state: {error}"));
        Self {
            state_store,
            commit_journal,
            state: RwLock::new(state),
            journal_path: None,
            tx_marker_path: None,
            snapshot_failpoint_path: snapshot_failpoint_path.map(Arc::new),
            shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool::new(false),
        }
    }

    fn from_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> Self {
        let state_dir = runtime_dir.as_ref().join("state");
        let journal_path = state_dir.join(SOCIAL_COMMIT_JOURNAL_FILE_NAME);
        let tx_marker_path = state_dir.join(SOCIAL_TRANSACTION_MARKER_FILE_NAME);
        let state_store = SocialStateStore::file(state_dir.join(SOCIAL_STATE_FILE_NAME));
        let commit_journal = Arc::new(FileCommitJournal::new(
            SOCIAL_COMMIT_PARTITION,
            journal_path.clone(),
        ));
        let state = Self::load_state_with_journal_replay(
            &state_store,
            journal_path.as_path(),
            Some(tx_marker_path.as_path()),
        );
        Self {
            state_store,
            commit_journal,
            state: RwLock::new(state),
            journal_path: Some(Arc::new(journal_path)),
            tx_marker_path: Some(Arc::new(tx_marker_path)),
            snapshot_failpoint_path: Some(Arc::new(state_dir.join("social-failpoints.json"))),
            shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool::new(false),
        }
    }

    fn replay_state_from_commit_journal(
        journal_path: &StdPath,
    ) -> Result<SocialControlState, String> {
        let mut replayed_state = SocialControlState::default();
        replayed_state.replay_commit_journal_file(journal_path)?;
        Ok(replayed_state)
    }

    fn load_state_with_journal_replay(
        state_store: &SocialStateStore,
        journal_path: &StdPath,
        tx_marker_path: Option<&StdPath>,
    ) -> SocialControlState {
        if journal_path.exists() {
            let snapshot_state = state_store.load().unwrap_or_default();
            let mut replayed_state = Self::replay_state_from_commit_journal(journal_path)
                .unwrap_or_else(|error| {
                    panic!("failed to replay control-plane social commit journal: {error}")
                });
            replayed_state.merge_pending_shared_channel_sync_requests_from(&snapshot_state);
            replayed_state.merge_dead_letter_shared_channel_sync_requests_from(&snapshot_state);
            state_store.save(&replayed_state).unwrap_or_else(|error| {
                panic!("failed to persist replayed control-plane social state: {error}")
            });
            if let Some(marker_path) = tx_marker_path {
                clear_social_transaction_marker(marker_path).unwrap_or_else(|error| {
                    panic!(
                        "failed to clear social transaction marker after journal replay: {error}"
                    )
                });
            }
            return replayed_state;
        }

        state_store
            .load()
            .unwrap_or_else(|error| panic!("failed to load control-plane social state: {error}"))
    }

    fn start_shared_channel_sync_stale_reclaim_scheduler(
        self: &Arc<Self>,
        config: SharedChannelSyncStaleReclaimSchedulerConfig,
    ) {
        let config = config.with_normalized_interval();
        if !config.enabled {
            return;
        }
        if self
            .shared_channel_sync_stale_reclaim_scheduler_started
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }

        let runtime = Arc::clone(self);
        let interval = config.interval();
        match std::thread::Builder::new()
            .name("shared-sync-stale-reclaim".to_owned())
            .spawn(move || {
                loop {
                    std::thread::sleep(interval);
                    if let Err(error) = runtime
                        .reclaim_stale_pending_shared_channel_sync_claims_if_any(
                            "failed to persist stale pending shared-channel sync reclaim from scheduler",
                        )
                    {
                        eprintln!(
                            "shared-channel sync stale reclaim scheduler tick failed: {error:?}"
                        );
                    }
                }
            }) {
            Ok(_) => {}
            Err(error) => {
                self.shared_channel_sync_stale_reclaim_scheduler_started
                    .store(false, Ordering::Release);
                eprintln!(
                    "failed to start shared-channel sync stale reclaim scheduler thread: {error}"
                );
            }
        }
    }

    fn consume_fail_next_snapshot_save(&self) -> Result<bool, String> {
        let Some(path) = self.snapshot_failpoint_path.as_deref() else {
            return Ok(false);
        };
        if !path.exists() {
            return Ok(false);
        }
        let content = fs::read_to_string(path).map_err(|error| {
            format!(
                "failed to read social failpoint file {}: {error}",
                path.display()
            )
        })?;
        if content.trim().is_empty() {
            return Ok(false);
        }
        let mut failpoints: SocialRuntimeFailpoints = serde_json::from_str(content.as_str())
            .map_err(|error| {
                format!(
                    "failed to parse social failpoint file {}: {error}",
                    path.display()
                )
            })?;
        if !failpoints.fail_next_snapshot_save {
            return Ok(false);
        }
        failpoints.fail_next_snapshot_save = false;
        let payload = serde_json::to_string_pretty(&failpoints)
            .map_err(|error| format!("failed to serialize social failpoints: {error}"))?;
        fs::write(path, payload).map_err(|error| {
            format!(
                "failed to consume social failpoint file {}: {error}",
                path.display()
            )
        })?;
        Ok(true)
    }

    fn persistence_with_snapshot_status(
        &self,
        snapshot_status: SocialDerivedSnapshotStatus,
    ) -> SocialWritePersistence {
        SocialWritePersistence {
            journal_authority: matches!(self.state_store, SocialStateStore::File { .. }),
            snapshot_status,
        }
    }

    fn current_persistence(&self) -> SocialWritePersistence {
        self.persistence_with_snapshot_status(SocialDerivedSnapshotStatus::Current)
    }

    fn repair_required_persistence(&self) -> SocialWritePersistence {
        self.persistence_with_snapshot_status(SocialDerivedSnapshotStatus::RepairRequired)
    }

    fn write_pending_tx_marker(&self, event_id: &str) -> Result<(), String> {
        let Some(path) = self.tx_marker_path.as_deref() else {
            return Ok(());
        };
        write_social_transaction_marker(
            path,
            &SocialTransactionMarker {
                status: SocialTransactionMarkerStatus::PendingSnapshotRepair,
                event_id: event_id.to_owned(),
            },
        )
    }

    fn clear_pending_tx_marker(&self) -> Result<bool, String> {
        let Some(path) = self.tx_marker_path.as_deref() else {
            return Ok(false);
        };
        clear_social_transaction_marker(path)
    }

    fn persist_state_transition(
        &self,
        next: &SocialControlState,
        commit: &CommitEnvelope,
    ) -> Result<SocialWritePersistence, ControlPlaneError> {
        self.commit_journal
            .append(commit.clone())
            .map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_commit_journal_unavailable",
                    format!(
                        "failed to append social commit journal before state write: {}",
                        contract_error_message(error)
                    ),
                )
            })?;
        self.write_pending_tx_marker(commit.event_id.as_str())
            .map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_state_unavailable",
                    format!("failed to write social transaction marker: {error}"),
                )
            })?;
        if self.consume_fail_next_snapshot_save().map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!("failed to consume social snapshot failpoint: {error}"),
            )
        })? {
            return Ok(self.repair_required_persistence());
        }
        if self.state_store.save(next).is_err() {
            return Ok(self.repair_required_persistence());
        }
        if self.clear_pending_tx_marker().is_err() {
            return Ok(self.repair_required_persistence());
        }
        Ok(self.current_persistence())
    }

    fn repair_derived_snapshot_best_effort(
        &self,
        state: &SocialControlState,
    ) -> SocialWritePersistence {
        if self.state_store.save(state).is_ok() && self.clear_pending_tx_marker().is_ok() {
            self.current_persistence()
        } else {
            self.repair_required_persistence()
        }
    }

    fn repair_derived_snapshot(&self) -> Result<SocialRuntimeRepairResponse, ControlPlaneError> {
        let pending_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let mut repaired_state = if let Some(journal_path) = self.journal_path.as_deref() {
            Self::replay_state_from_commit_journal(journal_path).map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_commit_journal_unavailable",
                    format!("failed to replay social commit journal during repair: {error}"),
                )
            })?
        } else {
            self.state
                .read()
                .expect("social runtime lock should not be poisoned")
                .clone()
        };
        repaired_state.merge_pending_shared_channel_sync_requests_from(&pending_state);
        repaired_state.merge_dead_letter_shared_channel_sync_requests_from(&pending_state);
        self.state_store.save(&repaired_state).map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!("failed to repair derived social state snapshot: {error}"),
            )
        })?;
        let transaction_marker_cleared = self.clear_pending_tx_marker().map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!("failed to clear social transaction marker after repair: {error}"),
            )
        })?;
        *self
            .state
            .write()
            .expect("social runtime lock should not be poisoned") = repaired_state.clone();
        Ok(SocialRuntimeRepairResponse {
            status: SocialRuntimeRepairStatus::Repaired,
            journal_authority: matches!(self.state_store, SocialStateStore::File { .. }),
            snapshot_updated: true,
            transaction_marker_cleared,
            aggregate_counts: repaired_state.aggregate_counts(),
        })
    }

    fn persist_failed_shared_channel_sync_requests(
        &self,
        requests: &[SharedChannelLinkedMemberSyncRequest],
        error: &str,
    ) -> Result<(), ControlPlaneError> {
        if requests.is_empty() {
            return Ok(());
        }

        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        let mut next_state = state.clone();
        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        if !next_state.record_failed_shared_channel_sync_requests(requests, error, now.as_str()) {
            return Ok(());
        }
        self.state_store.save(&next_state).map_err(|save_error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!(
                    "failed to persist pending shared-channel sync backlog after dispatch error: {save_error}"
                ),
            )
        })?;
        *state = next_state;
        Ok(())
    }

    fn pending_shared_channel_sync_dispatch_queue(
        &self,
        requests: &[SharedChannelLinkedMemberSyncRequest],
    ) -> Vec<SharedChannelLinkedMemberSyncRequest> {
        let state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned");
        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        let mut queue = Vec::with_capacity(
            state.pending_shared_channel_sync_requests.len()
                + state.dead_letter_shared_channel_sync_requests.len()
                + requests.len(),
        );
        let mut blocked = BTreeSet::new();
        let mut seen = BTreeSet::new();
        for pending in state.pending_shared_channel_sync_requests.values() {
            if state.is_dead_letter_shared_channel_sync_request(&pending.request) {
                continue;
            }
            let key = shared_channel_sync_request_key(&pending.request);
            if !pending.auto_dispatch_eligible(now.as_str()) {
                blocked.insert(key);
                continue;
            }
            if seen.insert(key) {
                queue.push(pending.request.clone());
            }
        }
        for request in requests {
            if state.is_dead_letter_shared_channel_sync_request(request) {
                continue;
            }
            let key = shared_channel_sync_request_key(request);
            if blocked.contains(&key) {
                continue;
            }
            if seen.insert(key) {
                queue.push(request.clone());
            }
        }
        queue
    }

    fn clear_pending_shared_channel_sync_request_best_effort(
        &self,
        request: &SharedChannelLinkedMemberSyncRequest,
    ) {
        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        let mut next_state = state.clone();
        if !next_state.remove_pending_shared_channel_sync_request(request) {
            return;
        }
        if self.state_store.save(&next_state).is_ok() {
            *state = next_state;
        }
    }

    fn repair_shared_channel_sync(
        &self,
        trigger: Option<&dyn SharedChannelLinkedMemberSyncTrigger>,
    ) -> Result<SocialSharedChannelSyncRepairResponse, ControlPlaneError> {
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let pending_before = current_state.pending_shared_channel_sync_count();
        let dead_letter_before = current_state.dead_letter_shared_channel_sync_count();
        if pending_before == 0 {
            return Ok(SocialSharedChannelSyncRepairResponse {
                status: SocialSharedChannelSyncRepairStatus::Noop,
                pending_before: 0,
                attempted: 0,
                dispatched: 0,
                failed: 0,
                reclaimed: 0,
                pending_after: 0,
                dead_letter_before,
                dead_lettered: 0,
                dead_letter_after: dead_letter_before,
            });
        }

        let mut next_state = current_state.clone();
        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        let reclaimed = next_state.reclaim_stale_pending_shared_channel_sync_claims(now.as_str());

        let Some(trigger) = trigger else {
            if reclaimed > 0 {
                self.state_store.save(&next_state).map_err(|error| {
                    ControlPlaneError::service_unavailable(
                        "social_state_unavailable",
                        format!(
                            "failed to persist shared-channel sync stale-claim reclaim before unconfigured repair: {error}"
                        ),
                    )
                })?;
                *self
                    .state
                    .write()
                    .expect("social runtime lock should not be poisoned") = next_state.clone();
            }
            return Ok(SocialSharedChannelSyncRepairResponse {
                status: SocialSharedChannelSyncRepairStatus::TriggerUnconfigured,
                pending_before,
                attempted: 0,
                dispatched: 0,
                failed: 0,
                reclaimed,
                pending_after: next_state.pending_shared_channel_sync_count(),
                dead_letter_before,
                dead_lettered: 0,
                dead_letter_after: next_state.dead_letter_shared_channel_sync_count(),
            });
        };

        let pending_items = next_state.pending_shared_channel_sync_requests();
        let mut dispatched = 0usize;
        let mut failed = 0usize;
        for pending in pending_items {
            match trigger.trigger(pending.request.clone()) {
                Ok(()) => {
                    next_state.remove_pending_shared_channel_sync_request(&pending.request);
                    dispatched += 1;
                }
                Err(error) => {
                    next_state.record_failed_shared_channel_sync_requests(
                        std::slice::from_ref(&pending.request),
                        error.as_str(),
                        now.as_str(),
                    );
                    failed += 1;
                }
            }
        }

        self.state_store.save(&next_state).map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!("failed to persist shared-channel sync repair backlog: {error}"),
            )
        })?;
        *self
            .state
            .write()
            .expect("social runtime lock should not be poisoned") = next_state.clone();

        let pending_after = next_state.pending_shared_channel_sync_count();
        let dead_letter_after = next_state.dead_letter_shared_channel_sync_count();
        let dead_lettered = dead_letter_after.saturating_sub(dead_letter_before);
        let status = if dead_lettered > 0 {
            if dispatched == 0 && pending_after == 0 {
                SocialSharedChannelSyncRepairStatus::DeadLettered
            } else {
                SocialSharedChannelSyncRepairStatus::PartiallyRepaired
            }
        } else {
            match (dispatched, pending_after) {
                (0, _) => SocialSharedChannelSyncRepairStatus::Pending,
                (_, 0) => SocialSharedChannelSyncRepairStatus::Repaired,
                _ => SocialSharedChannelSyncRepairStatus::PartiallyRepaired,
            }
        };

        Ok(SocialSharedChannelSyncRepairResponse {
            status,
            pending_before,
            attempted: pending_before,
            dispatched,
            failed,
            reclaimed,
            pending_after,
            dead_letter_before,
            dead_lettered,
            dead_letter_after,
        })
    }

    fn requeue_dead_letter_shared_channel_sync(
        &self,
    ) -> Result<SocialSharedChannelSyncDeadLetterRequeueResponse, ControlPlaneError> {
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let pending_before = current_state.pending_shared_channel_sync_count();
        let dead_letter_before = current_state.dead_letter_shared_channel_sync_count();
        if dead_letter_before == 0 {
            return Ok(SocialSharedChannelSyncDeadLetterRequeueResponse {
                status: SocialSharedChannelSyncDeadLetterRequeueStatus::Noop,
                pending_before,
                dead_letter_before,
                requeued: 0,
                pending_after: pending_before,
                dead_letter_after: dead_letter_before,
            });
        }

        let mut next_state = current_state.clone();
        let requeued = next_state.requeue_dead_letter_shared_channel_sync_requests();
        self.state_store.save(&next_state).map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!("failed to persist dead-letter shared-channel sync requeue: {error}"),
            )
        })?;
        *self
            .state
            .write()
            .expect("social runtime lock should not be poisoned") = next_state.clone();

        Ok(SocialSharedChannelSyncDeadLetterRequeueResponse {
            status: SocialSharedChannelSyncDeadLetterRequeueStatus::Requeued,
            pending_before,
            dead_letter_before,
            requeued,
            pending_after: next_state.pending_shared_channel_sync_count(),
            dead_letter_after: next_state.dead_letter_shared_channel_sync_count(),
        })
    }

    fn dead_letter_shared_channel_sync_inventory(
        &self,
        actor_id: &str,
        actor_kind: &str,
        can_takeover: bool,
    ) -> SocialSharedChannelSyncDeadLetterInventoryResponse {
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        let items = current_state
            .dead_letter_shared_channel_sync_requests()
            .into_iter()
            .map(|(request_key, request)| {
                social_shared_channel_sync_inventory_item_response(
                    request_key,
                    request,
                    actor_id,
                    actor_kind,
                    can_takeover,
                    now.as_str(),
                )
            })
            .collect::<Vec<_>>();

        SocialSharedChannelSyncDeadLetterInventoryResponse {
            status: SocialSharedChannelSyncDeadLetterInventoryStatus::Snapshot,
            dead_letter_count: items.len(),
            items,
        }
    }

    fn pending_shared_channel_sync_inventory(
        &self,
        actor_id: &str,
        actor_kind: &str,
        can_takeover: bool,
    ) -> SocialSharedChannelSyncPendingInventoryResponse {
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        let items = current_state
            .pending_shared_channel_sync_requests_with_keys()
            .into_iter()
            .map(|(request_key, request)| {
                social_shared_channel_sync_inventory_item_response(
                    request_key,
                    request,
                    actor_id,
                    actor_kind,
                    can_takeover,
                    now.as_str(),
                )
            })
            .collect::<Vec<_>>();

        SocialSharedChannelSyncPendingInventoryResponse {
            status: SocialSharedChannelSyncPendingInventoryStatus::Snapshot,
            pending_count: items.len(),
            items,
        }
    }

    fn reclaim_stale_pending_shared_channel_sync_claims_if_any(
        &self,
        persistence_error_context: &str,
    ) -> Result<usize, ControlPlaneError> {
        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        if state.pending_shared_channel_sync_count() == 0 {
            return Ok(0);
        }

        let mut next_state = state.clone();
        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        let reclaimed = next_state.reclaim_stale_pending_shared_channel_sync_claims(now.as_str());
        if reclaimed == 0 {
            return Ok(0);
        }

        self.state_store.save(&next_state).map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!("{persistence_error_context}: {error}"),
            )
        })?;
        *state = next_state;
        Ok(reclaimed)
    }

    fn reclaim_stale_pending_shared_channel_sync_claims(
        &self,
    ) -> Result<SocialSharedChannelSyncPendingStaleReclaimResponse, ControlPlaneError> {
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let pending_before = current_state.pending_shared_channel_sync_count();
        if pending_before == 0 {
            return Ok(SocialSharedChannelSyncPendingStaleReclaimResponse {
                status: SocialSharedChannelSyncPendingStaleReclaimStatus::Noop,
                pending_before: 0,
                reclaimed: 0,
                pending_after: 0,
            });
        }

        let reclaimed = self.reclaim_stale_pending_shared_channel_sync_claims_if_any(
            "failed to persist stale pending shared-channel sync reclaim",
        )?;
        if reclaimed == 0 {
            return Ok(SocialSharedChannelSyncPendingStaleReclaimResponse {
                status: SocialSharedChannelSyncPendingStaleReclaimStatus::Noop,
                pending_before,
                reclaimed: 0,
                pending_after: pending_before,
            });
        }

        Ok(SocialSharedChannelSyncPendingStaleReclaimResponse {
            status: SocialSharedChannelSyncPendingStaleReclaimStatus::Reclaimed,
            pending_before,
            reclaimed,
            pending_after: pending_before,
        })
    }

    fn claim_pending_shared_channel_sync_targeted(
        &self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
    ) -> Result<SocialSharedChannelSyncPendingClaimResponse, ControlPlaneError> {
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let pending_before = current_state.pending_shared_channel_sync_count();
        let requested = request_keys.len();
        if requested == 0 || pending_before == 0 {
            return Ok(SocialSharedChannelSyncPendingClaimResponse {
                status: SocialSharedChannelSyncPendingClaimStatus::Noop,
                pending_before,
                requested,
                claimed: 0,
                conflicted: 0,
                conflict_items: Vec::new(),
                pending_after: pending_before,
            });
        }

        let mut next_state = current_state.clone();
        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        let request_keys = request_keys.into_iter().collect::<Vec<_>>();
        let claim_result = next_state.claim_selected_pending_shared_channel_sync_requests(
            &request_keys,
            actor_id,
            actor_kind,
            now.as_str(),
        );

        if claim_result.claimed > 0 {
            self.state_store.save(&next_state).map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_state_unavailable",
                    format!(
                        "failed to persist targeted pending shared-channel sync claim: {error}"
                    ),
                )
            })?;
            *self
                .state
                .write()
                .expect("social runtime lock should not be poisoned") = next_state.clone();
        }

        let status = if claim_result.claimed == 0 && claim_result.conflicted == 0 {
            SocialSharedChannelSyncPendingClaimStatus::Noop
        } else if claim_result.claimed > 0 && claim_result.conflicted > 0 {
            SocialSharedChannelSyncPendingClaimStatus::PartiallyClaimed
        } else if claim_result.claimed > 0 {
            SocialSharedChannelSyncPendingClaimStatus::Claimed
        } else {
            SocialSharedChannelSyncPendingClaimStatus::Conflict
        };

        Ok(SocialSharedChannelSyncPendingClaimResponse {
            status,
            pending_before,
            requested,
            claimed: claim_result.claimed,
            conflicted: claim_result.conflicted,
            conflict_items: claim_result.conflict_items,
            pending_after: next_state.pending_shared_channel_sync_count(),
        })
    }

    fn release_pending_shared_channel_sync_targeted(
        &self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
    ) -> Result<SocialSharedChannelSyncPendingReleaseResponse, ControlPlaneError> {
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let pending_before = current_state.pending_shared_channel_sync_count();
        let requested = request_keys.len();
        if requested == 0 || pending_before == 0 {
            return Ok(SocialSharedChannelSyncPendingReleaseResponse {
                status: SocialSharedChannelSyncPendingReleaseStatus::Noop,
                pending_before,
                requested,
                released: 0,
                conflicted: 0,
                pending_after: pending_before,
            });
        }

        let selected_pending_items = current_state
            .pending_shared_channel_sync_requests_with_keys()
            .into_iter()
            .filter(|(request_key, _)| request_keys.contains(request_key))
            .collect::<Vec<_>>();

        if let Some((request_key, pending)) = selected_pending_items
            .iter()
            .find(|(_, pending)| pending.is_claimed_by_other(actor_id, actor_kind))
        {
            let now = format_unix_timestamp_millis(current_unix_epoch_millis());
            let message = if let Some(owner_actor_id) = pending.owner_actor_id.as_deref() {
                format!(
                    "pending shared-channel sync request {request_key} is claimed by {owner_actor_id}"
                )
            } else {
                format!(
                    "pending shared-channel sync request {request_key} is not owned by the current operator"
                )
            };
            return Err(ControlPlaneError::conflict_with_details(
                "shared_channel_sync_owner_conflict",
                message,
                social_shared_channel_sync_conflict_details(
                    request_key,
                    pending,
                    actor_id,
                    actor_kind,
                    now.as_str(),
                ),
            ));
        }

        let mut next_state = current_state.clone();
        let request_keys = request_keys.into_iter().collect::<Vec<_>>();
        let released = next_state.release_selected_pending_shared_channel_sync_requests(
            &request_keys,
            actor_id,
            actor_kind,
        );

        if released > 0 {
            self.state_store.save(&next_state).map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_state_unavailable",
                    format!(
                        "failed to persist targeted pending shared-channel sync release: {error}"
                    ),
                )
            })?;
            *self
                .state
                .write()
                .expect("social runtime lock should not be poisoned") = next_state.clone();
        }

        Ok(SocialSharedChannelSyncPendingReleaseResponse {
            status: if released == 0 {
                SocialSharedChannelSyncPendingReleaseStatus::Noop
            } else {
                SocialSharedChannelSyncPendingReleaseStatus::Released
            },
            pending_before,
            requested,
            released,
            conflicted: 0,
            pending_after: next_state.pending_shared_channel_sync_count(),
        })
    }

    fn takeover_pending_shared_channel_sync_targeted(
        &self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
        allow_legacy_untracked: bool,
    ) -> Result<SocialSharedChannelSyncPendingTakeoverResponse, ControlPlaneError> {
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let pending_before = current_state.pending_shared_channel_sync_count();
        let requested = request_keys.len();
        if requested == 0 || pending_before == 0 {
            return Ok(SocialSharedChannelSyncPendingTakeoverResponse {
                status: SocialSharedChannelSyncPendingTakeoverStatus::Noop,
                pending_before,
                requested,
                taken_over: 0,
                pending_after: pending_before,
                legacy_override_used: false,
            });
        }

        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        let selected_pending_items = current_state
            .pending_shared_channel_sync_requests_with_keys()
            .into_iter()
            .filter(|(request_key, _)| request_keys.contains(request_key))
            .collect::<Vec<_>>();

        if let Some((request_key, pending)) = selected_pending_items
            .iter()
            .find(|(_, pending)| pending.blocks_foreign_takeover(actor_id, actor_kind, &now))
        {
            let message = match (
                pending.owner_actor_id.as_deref(),
                pending.lease_expires_at.as_deref(),
            ) {
                (Some(owner_actor_id), Some(lease_expires_at)) => format!(
                    "pending shared-channel sync request {request_key} is actively claimed by {owner_actor_id} until {lease_expires_at}"
                ),
                (Some(owner_actor_id), None) => format!(
                    "pending shared-channel sync request {request_key} is claimed by {owner_actor_id}"
                ),
                _ => format!(
                    "pending shared-channel sync request {request_key} is not eligible for takeover by the current operator"
                ),
            };
            return Err(ControlPlaneError::conflict_with_details(
                "shared_channel_sync_owner_conflict",
                message,
                social_shared_channel_sync_conflict_details(
                    request_key,
                    pending,
                    actor_id,
                    actor_kind,
                    &now,
                ),
            ));
        }

        if !allow_legacy_untracked
            && let Some((request_key, pending)) = selected_pending_items
                .iter()
                .find(|(_, pending)| pending.legacy_takeover_required_for(actor_id, actor_kind))
        {
            let message = if let Some(owner_actor_id) = pending.owner_actor_id.as_deref() {
                format!(
                    "pending shared-channel sync request {request_key} is claimed by {owner_actor_id} without leaseExpiresAt; explicit legacy takeover override is required"
                )
            } else {
                format!(
                    "pending shared-channel sync request {request_key} requires explicit legacy takeover override"
                )
            };
            return Err(ControlPlaneError::conflict_with_details(
                "shared_channel_sync_legacy_takeover_override_required",
                message,
                social_shared_channel_sync_conflict_details(
                    request_key,
                    pending,
                    actor_id,
                    actor_kind,
                    &now,
                ),
            ));
        }

        let legacy_override_used = allow_legacy_untracked
            && selected_pending_items
                .iter()
                .any(|(_, pending)| pending.legacy_takeover_required_for(actor_id, actor_kind));

        let mut next_state = current_state.clone();
        let request_keys = request_keys.into_iter().collect::<Vec<_>>();
        let taken_over = next_state.takeover_selected_pending_shared_channel_sync_requests(
            &request_keys,
            actor_id,
            actor_kind,
        );

        if taken_over > 0 {
            self.state_store.save(&next_state).map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_state_unavailable",
                    format!(
                        "failed to persist targeted pending shared-channel sync takeover: {error}"
                    ),
                )
            })?;
            *self
                .state
                .write()
                .expect("social runtime lock should not be poisoned") = next_state.clone();
        }

        Ok(SocialSharedChannelSyncPendingTakeoverResponse {
            status: if taken_over == 0 {
                SocialSharedChannelSyncPendingTakeoverStatus::Noop
            } else {
                SocialSharedChannelSyncPendingTakeoverStatus::TakenOver
            },
            pending_before,
            requested,
            taken_over,
            pending_after: next_state.pending_shared_channel_sync_count(),
            legacy_override_used,
        })
    }

    fn requeue_dead_letter_shared_channel_sync_targeted(
        &self,
        request_keys: &[String],
    ) -> Result<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse, ControlPlaneError> {
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let pending_before = current_state.pending_shared_channel_sync_count();
        let dead_letter_before = current_state.dead_letter_shared_channel_sync_count();
        let requested = request_keys.len();
        if requested == 0 || dead_letter_before == 0 {
            return Ok(SocialSharedChannelSyncDeadLetterTargetedRequeueResponse {
                status: SocialSharedChannelSyncDeadLetterRequeueStatus::Noop,
                pending_before,
                dead_letter_before,
                requested,
                requeued: 0,
                pending_after: pending_before,
                dead_letter_after: dead_letter_before,
            });
        }

        let mut next_state = current_state.clone();
        let request_keys = request_keys.into_iter().collect::<Vec<_>>();
        let requeued =
            next_state.requeue_selected_dead_letter_shared_channel_sync_requests(&request_keys);
        self.state_store.save(&next_state).map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!(
                    "failed to persist targeted dead-letter shared-channel sync requeue: {error}"
                ),
            )
        })?;
        *self
            .state
            .write()
            .expect("social runtime lock should not be poisoned") = next_state.clone();

        Ok(SocialSharedChannelSyncDeadLetterTargetedRequeueResponse {
            status: if requeued == 0 {
                SocialSharedChannelSyncDeadLetterRequeueStatus::Noop
            } else {
                SocialSharedChannelSyncDeadLetterRequeueStatus::Requeued
            },
            pending_before,
            dead_letter_before,
            requested,
            requeued,
            pending_after: next_state.pending_shared_channel_sync_count(),
            dead_letter_after: next_state.dead_letter_shared_channel_sync_count(),
        })
    }

    fn republish_pending_shared_channel_sync_targeted(
        &self,
        request_keys: &[String],
        actor_id: &str,
        actor_kind: &str,
        trigger: Option<&dyn SharedChannelLinkedMemberSyncTrigger>,
    ) -> Result<SocialSharedChannelSyncTargetedRepublishResponse, ControlPlaneError> {
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .expect("social runtime lock should not be poisoned")
            .clone();
        let pending_before = current_state.pending_shared_channel_sync_count();
        let dead_letter_before = current_state.dead_letter_shared_channel_sync_count();
        let requested = request_keys.len();
        let selected_pending_items = current_state
            .pending_shared_channel_sync_requests_with_keys()
            .into_iter()
            .filter(|(request_key, _)| request_keys.contains(request_key))
            .collect::<Vec<_>>();
        let attempted = selected_pending_items.len();
        if attempted == 0 {
            return Ok(SocialSharedChannelSyncTargetedRepublishResponse {
                status: SocialSharedChannelSyncTargetedRepublishStatus::Noop,
                pending_before,
                requested,
                attempted,
                dispatched: 0,
                failed: 0,
                pending_after: pending_before,
                dead_letter_before,
                dead_lettered: 0,
                dead_letter_after: dead_letter_before,
            });
        }

        if let Some((request_key, pending)) = selected_pending_items
            .iter()
            .find(|(_, pending)| !pending.is_owned_by(actor_id, actor_kind))
        {
            let now = format_unix_timestamp_millis(current_unix_epoch_millis());
            let message = if let Some(owner_actor_id) = pending.owner_actor_id.as_deref() {
                format!(
                    "pending shared-channel sync request {request_key} is claimed by {owner_actor_id}"
                )
            } else {
                format!(
                    "pending shared-channel sync request {request_key} must be claimed by the current operator before targeted republish"
                )
            };
            return Err(ControlPlaneError::conflict_with_details(
                "shared_channel_sync_owner_conflict",
                message,
                social_shared_channel_sync_conflict_details(
                    request_key,
                    pending,
                    actor_id,
                    actor_kind,
                    now.as_str(),
                ),
            ));
        }

        let mut next_state = current_state.clone();
        let republish_started_at = format_unix_timestamp_millis(current_unix_epoch_millis());
        let mut renewed_stale_same_owner_lease = false;
        for (request_key, pending) in &selected_pending_items {
            if pending.is_owned_by(actor_id, actor_kind)
                && pending.lease_status(republish_started_at.as_str())
                    == SocialSharedChannelSyncLeaseStatus::Stale
            {
                if let Some(selected_pending) = next_state
                    .pending_shared_channel_sync_requests
                    .get_mut(request_key.as_str())
                {
                    selected_pending.assign_owner(actor_id, actor_kind);
                    renewed_stale_same_owner_lease = true;
                }
            }
        }

        let Some(trigger) = trigger else {
            if renewed_stale_same_owner_lease {
                self.state_store.save(&next_state).map_err(|error| {
                    ControlPlaneError::service_unavailable(
                        "social_state_unavailable",
                        format!(
                            "failed to persist targeted shared-channel sync republish backlog: {error}"
                        ),
                    )
                })?;
                *self
                    .state
                    .write()
                    .expect("social runtime lock should not be poisoned") = next_state;
            }
            return Ok(SocialSharedChannelSyncTargetedRepublishResponse {
                status: SocialSharedChannelSyncTargetedRepublishStatus::TriggerUnconfigured,
                pending_before,
                requested,
                attempted,
                dispatched: 0,
                failed: 0,
                pending_after: pending_before,
                dead_letter_before,
                dead_lettered: 0,
                dead_letter_after: dead_letter_before,
            });
        };

        let mut dispatched = 0usize;
        let mut failed = 0usize;
        for (_, pending) in &selected_pending_items {
            match trigger.trigger(pending.request.clone()) {
                Ok(()) => {
                    next_state.remove_pending_shared_channel_sync_request(&pending.request);
                    dispatched += 1;
                }
                Err(error) => {
                    let now = format_unix_timestamp_millis(current_unix_epoch_millis());
                    next_state.record_failed_shared_channel_sync_requests(
                        std::slice::from_ref(&pending.request),
                        error.as_str(),
                        now.as_str(),
                    );
                    failed += 1;
                }
            }
        }

        self.state_store.save(&next_state).map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!(
                    "failed to persist targeted shared-channel sync republish backlog: {error}"
                ),
            )
        })?;
        *self
            .state
            .write()
            .expect("social runtime lock should not be poisoned") = next_state.clone();

        let pending_after = next_state.pending_shared_channel_sync_count();
        let dead_letter_after = next_state.dead_letter_shared_channel_sync_count();
        let dead_lettered = dead_letter_after.saturating_sub(dead_letter_before);
        let selected_pending_after = selected_pending_items
            .iter()
            .filter(|(request_key, _)| {
                next_state
                    .pending_shared_channel_sync_requests
                    .contains_key(request_key.as_str())
            })
            .count();
        let selected_dead_letter_after = selected_pending_items
            .iter()
            .filter(|(request_key, _)| {
                next_state
                    .dead_letter_shared_channel_sync_requests
                    .contains_key(request_key.as_str())
            })
            .count();
        let status = if dead_lettered > 0
            && dispatched == 0
            && selected_pending_after == 0
            && selected_dead_letter_after > 0
        {
            SocialSharedChannelSyncTargetedRepublishStatus::DeadLettered
        } else if failed > 0 && dispatched > 0 {
            SocialSharedChannelSyncTargetedRepublishStatus::PartiallyRepublished
        } else if failed > 0 {
            SocialSharedChannelSyncTargetedRepublishStatus::Pending
        } else {
            SocialSharedChannelSyncTargetedRepublishStatus::Republished
        };

        Ok(SocialSharedChannelSyncTargetedRepublishResponse {
            status,
            pending_before,
            requested,
            attempted,
            dispatched,
            failed,
            pending_after,
            dead_letter_before,
            dead_lettered,
            dead_letter_after,
        })
    }

    fn replay_committed_social_event<T>(
        &self,
        state: &SocialControlState,
        commit: &CommitEnvelope,
        project: impl FnOnce(
            SocialCommittedEvent,
            SocialWritePersistence,
        ) -> Result<T, ControlPlaneError>,
    ) -> Result<Option<T>, ControlPlaneError> {
        let Some(existing) =
            state.committed_event(commit.tenant_id.as_str(), commit.event_id.as_str())
        else {
            return Ok(None);
        };
        if existing.commit() != commit {
            return Err(social_event_id_conflict(
                commit.event_id.as_str(),
                &existing,
            ));
        }
        let persistence = self.repair_derived_snapshot_best_effort(state);
        project(existing, persistence).map(Some)
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ControlPlaneErrorStatus {
    Unauthorized,
    Forbidden,
    Invalid,
    Conflict,
    NotFound,
    Unavailable,
}

#[derive(Debug)]
struct ControlPlaneError {
    status: StatusCode,
    code: &'static str,
    message: String,
    details: Option<serde_json::Value>,
}

fn contract_error_message(error: ContractError) -> String {
    match error {
        ContractError::UnsupportedCapability(message)
        | ContractError::Conflict(message)
        | ContractError::Unavailable(message) => message,
    }
}

fn social_event_id_conflict(event_id: &str, existing: &SocialCommittedEvent) -> ControlPlaneError {
    let committed = existing.commit();
    ControlPlaneError::conflict(
        "social_event_id_conflict",
        format!(
            "eventId {} is already committed for {} {}",
            event_id,
            existing.aggregate_label(),
            committed.aggregate_id
        ),
    )
}

impl From<RealtimeClusterError> for ControlPlaneError {
    fn from(value: RealtimeClusterError) -> Self {
        let status = match value.code {
            "node_not_found" | "target_node_not_found" | "node_runtime_missing" => {
                StatusCode::NOT_FOUND
            }
            "same_node_migration"
            | "node_not_draining"
            | "target_node_unavailable"
            | "node_draining" => StatusCode::CONFLICT,
            _ => StatusCode::BAD_REQUEST,
        };
        Self {
            status,
            code: value.code,
            message: value.message,
            details: None,
        }
    }
}

impl From<AuthContextError> for ControlPlaneError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
            details: None,
        }
    }
}

impl From<ContractError> for ControlPlaneError {
    fn from(value: ContractError) -> Self {
        match value {
            ContractError::UnsupportedCapability(message) => Self {
                status: StatusCode::BAD_REQUEST,
                code: "invalid_provider_policy",
                message,
                details: None,
            },
            ContractError::Conflict(message) => Self {
                status: StatusCode::CONFLICT,
                code: "provider_policy_conflict",
                message,
                details: None,
            },
            ContractError::Unavailable(message) => Self {
                status: StatusCode::SERVICE_UNAVAILABLE,
                code: "provider_policy_unavailable",
                message,
                details: None,
            },
        }
    }
}

impl From<SocialInvariantError> for ControlPlaneError {
    fn from(value: SocialInvariantError) -> Self {
        Self::invalid("invalid_friend_request", value.to_string())
    }
}

impl ControlPlaneError {
    fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
            details: None,
        }
    }

    fn invalid(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
            details: None,
        }
    }

    fn conflict(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code,
            message: message.into(),
            details: None,
        }
    }

    fn conflict_with_details(
        code: &'static str,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            code,
            message: message.into(),
            details: Some(details),
        }
    }

    fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code,
            message: message.into(),
            details: None,
        }
    }

    fn service_unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code,
            message: message.into(),
            details: None,
        }
    }

    fn response_status(status: StatusCode) -> ControlPlaneErrorStatus {
        match status {
            StatusCode::UNAUTHORIZED => ControlPlaneErrorStatus::Unauthorized,
            StatusCode::FORBIDDEN => ControlPlaneErrorStatus::Forbidden,
            StatusCode::CONFLICT => ControlPlaneErrorStatus::Conflict,
            StatusCode::NOT_FOUND => ControlPlaneErrorStatus::NotFound,
            StatusCode::SERVICE_UNAVAILABLE => ControlPlaneErrorStatus::Unavailable,
            _ => ControlPlaneErrorStatus::Invalid,
        }
    }
}

impl axum::response::IntoResponse for ControlPlaneError {
    fn into_response(self) -> axum::response::Response {
        let response_status = Self::response_status(self.status);
        let mut body = serde_json::json!({
            "status": response_status,
            "code": self.code,
            "message": self.message
        });
        if let Some(details) = self.details {
            body["details"] = details;
        }
        (self.status, Json(body)).into_response()
    }
}

impl From<CommitEnvelope> for CommitEnvelopeResponse {
    fn from(value: CommitEnvelope) -> Self {
        Self {
            event_id: value.event_id,
            tenant_id: value.tenant_id,
            event_type: value.event_type,
            event_version: value.event_version,
            aggregate_type: value.aggregate_type.as_wire_value().into(),
            aggregate_id: value.aggregate_id,
            scope_type: value.scope_type,
            scope_id: value.scope_id,
            ordering_key: value.ordering_key,
            ordering_seq: value.ordering_seq,
            causation_id: value.causation_id,
            correlation_id: value.correlation_id,
            idempotency_key: value.idempotency_key,
            actor: EventActorResponse {
                actor_id: value.actor.actor_id,
                actor_kind: value.actor.actor_kind,
                actor_session_id: value.actor.actor_session_id,
            },
            occurred_at: value.occurred_at,
            committed_at: value.committed_at,
            payload_schema: value.payload_schema,
            payload: value.payload,
            retention_class: value.retention_class,
            audit_class: value.audit_class,
        }
    }
}

impl SocialControlRuntime {
    fn establish_external_connection(
        &self,
        tenant_id: &str,
        auth: &AuthContext,
        request: EstablishExternalConnectionRequest,
    ) -> Result<EstablishedExternalConnection, ControlPlaneError> {
        validate_required_with_code(
            "connectionId",
            request.connection_id.as_str(),
            "invalid_external_connection",
        )?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_external_connection",
        )?;
        validate_required_with_code(
            "externalTenantId",
            request.external_tenant_id.as_str(),
            "invalid_external_connection",
        )?;
        validate_required_with_code(
            "establishedAt",
            request.established_at.as_str(),
            "invalid_external_connection",
        )?;
        ensure_cross_tenant_connection(tenant_id, request.external_tenant_id.as_str()).map_err(
            |error| ControlPlaneError::invalid("invalid_external_connection", error.to_string()),
        )?;

        let payload = ExternalConnectionEstablishedPayload {
            connection_id: request.connection_id.clone(),
            external_tenant_id: request.external_tenant_id.clone(),
            external_org_name: request.external_org_name.clone(),
            connection_kind: serde_json::to_string(&request.connection_kind)
                .expect("external connection kind should serialize")
                .trim_matches('"')
                .to_owned(),
            established_at: request.established_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("external connection payload should serialize into json");
        let commit = social_commit_envelope(
            request.event_id.as_str(),
            tenant_id,
            AggregateType::ExternalConnection,
            request.connection_id.as_str(),
            SocialEventType::ExternalConnectionEstablished,
            1,
            EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            request.established_at.as_str(),
            request.established_at.as_str(),
            payload_json.as_str(),
        );
        let external_connection = ExternalConnection {
            tenant_id: tenant_id.into(),
            connection_id: request.connection_id.clone(),
            external_tenant_id: request.external_tenant_id.clone(),
            external_org_name: request.external_org_name,
            connection_kind: request.connection_kind.clone(),
            status: ExternalConnectionStatus::Active,
            established_at: request.established_at.clone(),
            updated_at: request.established_at,
        };

        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::ExternalConnection { record, commit } => {
                        Ok(EstablishedExternalConnection {
                            external_connection: record.external_connection,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict(request.event_id.as_str(), &other)),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .external_connections
            .contains_key(external_connection.connection_id.as_str())
        {
            return Err(ControlPlaneError::conflict(
                "external_connection_conflict",
                format!(
                    "external connection {} already exists",
                    external_connection.connection_id
                ),
            ));
        }
        if next_state.external_connections.values().any(|record| {
            record.external_connection.tenant_id == tenant_id
                && record.external_connection.status.is_active()
                && record.external_connection.external_tenant_id
                    == external_connection.external_tenant_id
                && record.external_connection.connection_kind == external_connection.connection_kind
        }) {
            return Err(ControlPlaneError::conflict(
                "external_connection_target_conflict",
                format!(
                    "active external connection already exists for tenant {} and kind {:?}",
                    external_connection.external_tenant_id, external_connection.connection_kind
                ),
            ));
        }

        next_state.external_connections.insert(
            external_connection.connection_id.clone(),
            StoredExternalConnection {
                external_connection: external_connection.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(EstablishedExternalConnection {
            external_connection,
            latest_commit: commit,
            persistence,
        })
    }

    fn external_connection_snapshot(
        &self,
        tenant_id: &str,
        connection_id: &str,
    ) -> Option<StoredExternalConnection> {
        self.state
            .read()
            .expect("social runtime lock should not be poisoned")
            .external_connections
            .get(connection_id)
            .filter(|record| record.external_connection.tenant_id == tenant_id)
            .cloned()
    }

    fn bind_external_member_link(
        &self,
        tenant_id: &str,
        auth: &AuthContext,
        request: BindExternalMemberLinkRequest,
    ) -> Result<BoundExternalMemberLink, ControlPlaneError> {
        validate_required_with_code(
            "linkId",
            request.link_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "connectionId",
            request.connection_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "localActorId",
            request.local_actor_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "localActorKind",
            request.local_actor_kind.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "externalMemberId",
            request.external_member_id.as_str(),
            "invalid_external_member_link",
        )?;
        validate_required_with_code(
            "linkedAt",
            request.linked_at.as_str(),
            "invalid_external_member_link",
        )?;

        let connection = self
            .external_connection_snapshot(tenant_id, request.connection_id.as_str())
            .ok_or_else(|| {
                ControlPlaneError::not_found(
                    "external_connection_not_found",
                    format!(
                        "external connection {} was not found",
                        request.connection_id
                    ),
                )
            })?;
        if !connection.external_connection.status.is_active() {
            return Err(ControlPlaneError::conflict(
                "external_connection_inactive",
                format!(
                    "external connection {} is not active",
                    connection.external_connection.connection_id
                ),
            ));
        }

        let payload = ExternalMemberLinkBoundPayload {
            link_id: request.link_id.clone(),
            connection_id: request.connection_id.clone(),
            local_actor_id: request.local_actor_id.clone(),
            local_actor_kind: Some(request.local_actor_kind.clone()),
            external_member_id: request.external_member_id.clone(),
            external_display_name: request.external_display_name.clone(),
            linked_at: request.linked_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("external member link payload should serialize into json");
        let commit = social_commit_envelope(
            request.event_id.as_str(),
            tenant_id,
            AggregateType::ExternalMemberLink,
            request.link_id.as_str(),
            SocialEventType::ExternalMemberLinkBound,
            1,
            EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            request.linked_at.as_str(),
            request.linked_at.as_str(),
            payload_json.as_str(),
        );
        let external_member_link = ExternalMemberLink {
            tenant_id: tenant_id.into(),
            link_id: request.link_id.clone(),
            connection_id: request.connection_id.clone(),
            local_actor_id: request.local_actor_id,
            local_actor_kind: Some(request.local_actor_kind),
            external_member_id: request.external_member_id,
            external_display_name: request.external_display_name,
            status: ExternalMemberLinkStatus::Active,
            linked_at: request.linked_at.clone(),
            updated_at: request.linked_at,
        };

        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::ExternalMemberLink { record, commit } => {
                        Ok(BoundExternalMemberLink {
                            shared_channel_sync_requests:
                                shared_channel_sync_requests_for_external_member_link(
                                    &state,
                                    &record.external_member_link,
                                ),
                            external_member_link: record.external_member_link,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict(request.event_id.as_str(), &other)),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .external_member_links
            .contains_key(external_member_link.link_id.as_str())
        {
            return Err(ControlPlaneError::conflict(
                "external_member_link_conflict",
                format!(
                    "external member link {} already exists",
                    external_member_link.link_id
                ),
            ));
        }
        if next_state.external_member_links.values().any(|record| {
            record.external_member_link.tenant_id == tenant_id
                && record.external_member_link.status.is_active()
                && record.external_member_link.connection_id == external_member_link.connection_id
                && record.external_member_link.external_member_id
                    == external_member_link.external_member_id
        }) {
            return Err(ControlPlaneError::conflict(
                "external_member_mapping_conflict",
                format!(
                    "active external member mapping already exists for {} on connection {}",
                    external_member_link.external_member_id, external_member_link.connection_id
                ),
            ));
        }

        next_state.external_member_links.insert(
            external_member_link.link_id.clone(),
            StoredExternalMemberLink {
                external_member_link: external_member_link.clone(),
                commits: vec![commit.clone()],
            },
        );
        let shared_channel_sync_requests = shared_channel_sync_requests_for_external_member_link(
            &next_state,
            &external_member_link,
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(BoundExternalMemberLink {
            external_member_link,
            latest_commit: commit,
            persistence,
            shared_channel_sync_requests,
        })
    }

    fn external_member_link_snapshot(
        &self,
        tenant_id: &str,
        link_id: &str,
    ) -> Option<StoredExternalMemberLink> {
        self.state
            .read()
            .expect("social runtime lock should not be poisoned")
            .external_member_links
            .get(link_id)
            .filter(|record| record.external_member_link.tenant_id == tenant_id)
            .cloned()
    }

    fn apply_shared_channel_policy(
        &self,
        tenant_id: &str,
        auth: &AuthContext,
        request: ApplySharedChannelPolicyRequest,
    ) -> Result<AppliedSharedChannelPolicy, ControlPlaneError> {
        validate_required_with_code(
            "policyId",
            request.policy_id.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "connectionId",
            request.connection_id.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "channelId",
            request.channel_id.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "historyVisibility",
            request.history_visibility.as_str(),
            "invalid_shared_channel_policy",
        )?;
        validate_required_with_code(
            "appliedAt",
            request.applied_at.as_str(),
            "invalid_shared_channel_policy",
        )?;
        if request.policy_version == 0 {
            return Err(ControlPlaneError::invalid(
                "invalid_shared_channel_policy",
                "policyVersion must be greater than 0",
            ));
        }
        if request.history_visibility != "shared" {
            return Err(ControlPlaneError::invalid(
                "invalid_shared_channel_policy",
                format!(
                    "shared_channel_policy only supports historyVisibility=shared, got {}",
                    request.history_visibility
                ),
            ));
        }

        let connection = self
            .external_connection_snapshot(tenant_id, request.connection_id.as_str())
            .ok_or_else(|| {
                ControlPlaneError::not_found(
                    "external_connection_not_found",
                    format!(
                        "external connection {} was not found",
                        request.connection_id
                    ),
                )
            })?;
        if !connection.external_connection.status.is_active() {
            return Err(ControlPlaneError::conflict(
                "external_connection_inactive",
                format!(
                    "external connection {} is not active",
                    connection.external_connection.connection_id
                ),
            ));
        }

        let payload = SharedChannelPolicyAppliedPayload {
            policy_id: request.policy_id.clone(),
            connection_id: request.connection_id.clone(),
            channel_id: request.channel_id.clone(),
            conversation_id: request.conversation_id.clone(),
            policy_version: request.policy_version,
            history_visibility: request.history_visibility.clone(),
            applied_at: request.applied_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("shared channel policy payload should serialize into json");
        let commit = social_commit_envelope(
            request.event_id.as_str(),
            tenant_id,
            AggregateType::SharedChannelPolicy,
            request.policy_id.as_str(),
            SocialEventType::SharedChannelPolicyApplied,
            1,
            EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            request.applied_at.as_str(),
            request.applied_at.as_str(),
            payload_json.as_str(),
        );
        let shared_channel_policy = SharedChannelPolicy {
            tenant_id: tenant_id.into(),
            policy_id: request.policy_id.clone(),
            connection_id: request.connection_id.clone(),
            channel_id: request.channel_id,
            conversation_id: request.conversation_id,
            policy_version: request.policy_version,
            history_visibility: request.history_visibility,
            status: SharedChannelPolicyStatus::Active,
            applied_at: request.applied_at.clone(),
            updated_at: request.applied_at,
        };

        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::SharedChannelPolicy { record, commit } => {
                        Ok(AppliedSharedChannelPolicy {
                            shared_channel_sync_requests:
                                shared_channel_sync_requests_for_shared_channel_policy(
                                    &state,
                                    &record.shared_channel_policy,
                                ),
                            shared_channel_policy: record.shared_channel_policy,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict(request.event_id.as_str(), &other)),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .shared_channel_policies
            .contains_key(shared_channel_policy.policy_id.as_str())
        {
            return Err(ControlPlaneError::conflict(
                "shared_channel_policy_conflict",
                format!(
                    "shared channel policy {} already exists",
                    shared_channel_policy.policy_id
                ),
            ));
        }
        if next_state.shared_channel_policies.values().any(|record| {
            record.shared_channel_policy.tenant_id == tenant_id
                && record.shared_channel_policy.status.is_active()
                && record.shared_channel_policy.connection_id == shared_channel_policy.connection_id
                && record.shared_channel_policy.channel_id == shared_channel_policy.channel_id
        }) {
            return Err(ControlPlaneError::conflict(
                "shared_channel_policy_target_conflict",
                format!(
                    "active shared channel policy already exists for channel {} on connection {}",
                    shared_channel_policy.channel_id, shared_channel_policy.connection_id
                ),
            ));
        }

        next_state.shared_channel_policies.insert(
            shared_channel_policy.policy_id.clone(),
            StoredSharedChannelPolicy {
                shared_channel_policy: shared_channel_policy.clone(),
                commits: vec![commit.clone()],
            },
        );
        let shared_channel_sync_requests = shared_channel_sync_requests_for_shared_channel_policy(
            &next_state,
            &shared_channel_policy,
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(AppliedSharedChannelPolicy {
            shared_channel_policy,
            latest_commit: commit,
            persistence,
            shared_channel_sync_requests,
        })
    }

    fn shared_channel_policy_snapshot(
        &self,
        tenant_id: &str,
        policy_id: &str,
    ) -> Option<StoredSharedChannelPolicy> {
        self.state
            .read()
            .expect("social runtime lock should not be poisoned")
            .shared_channel_policies
            .get(policy_id)
            .filter(|record| record.shared_channel_policy.tenant_id == tenant_id)
            .cloned()
    }

    fn submit_friend_request(
        &self,
        tenant_id: &str,
        auth: &AuthContext,
        request: SubmitFriendRequestRequest,
    ) -> Result<SubmittedFriendRequest, ControlPlaneError> {
        validate_required("requestId", request.request_id.as_str())?;
        validate_required("eventId", request.event_id.as_str())?;
        validate_required("requesterUserId", request.requester_user_id.as_str())?;
        validate_required("targetUserId", request.target_user_id.as_str())?;
        validate_required("requestedAt", request.requested_at.as_str())?;
        normalize_user_pair(
            request.requester_user_id.as_str(),
            request.target_user_id.as_str(),
        )?;

        let payload = FriendRequestSubmittedPayload {
            request_id: request.request_id.clone(),
            requester_user_id: request.requester_user_id.clone(),
            target_user_id: request.target_user_id.clone(),
            request_message: request.request_message.clone(),
            requested_at: request.requested_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friend request payload should serialize into json");
        let commit = social_commit_envelope(
            request.event_id.as_str(),
            tenant_id,
            AggregateType::FriendRequest,
            request.request_id.as_str(),
            SocialEventType::FriendRequestSubmitted,
            1,
            EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            request.requested_at.as_str(),
            request.requested_at.as_str(),
            payload_json.as_str(),
        );
        let friend_request = FriendRequest {
            tenant_id: tenant_id.into(),
            request_id: request.request_id.clone(),
            requester_user_id: request.requester_user_id,
            target_user_id: request.target_user_id,
            status: FriendRequestStatus::Pending,
            request_message: request.request_message,
            expired_at: None,
            created_at: request.requested_at.clone(),
            updated_at: request.requested_at,
        };

        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::FriendRequest { record, commit } => {
                        Ok(SubmittedFriendRequest {
                            friend_request: record.friend_request,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict(request.event_id.as_str(), &other)),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .friend_requests
            .contains_key(friend_request.request_id.as_str())
        {
            return Err(ControlPlaneError::conflict(
                "friend_request_conflict",
                format!(
                    "friend request {} already exists",
                    friend_request.request_id
                ),
            ));
        }

        next_state.friend_requests.insert(
            friend_request.request_id.clone(),
            StoredFriendRequest {
                friend_request: friend_request.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(SubmittedFriendRequest {
            friend_request,
            latest_commit: commit,
            persistence,
        })
    }

    fn friend_request_snapshot(
        &self,
        tenant_id: &str,
        request_id: &str,
    ) -> Option<StoredFriendRequest> {
        self.state
            .read()
            .expect("social runtime lock should not be poisoned")
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.tenant_id == tenant_id)
            .cloned()
    }

    fn activate_friendship(
        &self,
        tenant_id: &str,
        auth: &AuthContext,
        request: ActivateFriendshipRequest,
    ) -> Result<ActivatedFriendship, ControlPlaneError> {
        validate_required_with_code(
            "friendshipId",
            request.friendship_id.as_str(),
            "invalid_friendship",
        )?;
        validate_required_with_code("eventId", request.event_id.as_str(), "invalid_friendship")?;
        validate_required_with_code(
            "initiatorUserId",
            request.initiator_user_id.as_str(),
            "invalid_friendship",
        )?;
        validate_required_with_code(
            "peerUserId",
            request.peer_user_id.as_str(),
            "invalid_friendship",
        )?;
        validate_required_with_code(
            "establishedAt",
            request.established_at.as_str(),
            "invalid_friendship",
        )?;
        let pair = normalize_user_pair(
            request.initiator_user_id.as_str(),
            request.peer_user_id.as_str(),
        )
        .map_err(|error| ControlPlaneError::invalid("invalid_friendship", error.to_string()))?;

        let payload = FriendshipActivatedPayload {
            friendship_id: request.friendship_id.clone(),
            user_low_id: pair.user_low_id.clone(),
            user_high_id: pair.user_high_id.clone(),
            initiator_user_id: request.initiator_user_id.clone(),
            direct_chat_id: request.direct_chat_id.clone(),
            established_at: request.established_at.clone(),
        };
        let payload_json =
            serde_json::to_string(&payload).expect("friendship payload should serialize into json");
        let commit = social_commit_envelope(
            request.event_id.as_str(),
            tenant_id,
            AggregateType::Friendship,
            request.friendship_id.as_str(),
            SocialEventType::FriendshipActivated,
            1,
            EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            request.established_at.as_str(),
            request.established_at.as_str(),
            payload_json.as_str(),
        );
        let friendship = Friendship {
            tenant_id: tenant_id.into(),
            friendship_id: request.friendship_id.clone(),
            user_low_id: pair.user_low_id.clone(),
            user_high_id: pair.user_high_id.clone(),
            initiator_user_id: request.initiator_user_id,
            status: FriendshipStatus::Active,
            established_at: Some(request.established_at.clone()),
            updated_at: request.established_at,
        };

        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::Friendship { record, commit } => {
                        Ok(ActivatedFriendship {
                            friendship: record.friendship,
                            latest_commit: commit,
                            persistence,
                        })
                    }
                    other => Err(social_event_id_conflict(request.event_id.as_str(), &other)),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .friendships
            .contains_key(friendship.friendship_id.as_str())
        {
            return Err(ControlPlaneError::conflict(
                "friendship_conflict",
                format!("friendship {} already exists", friendship.friendship_id),
            ));
        }
        if next_state.friendships.values().any(|record| {
            record.friendship.tenant_id == tenant_id
                && record.friendship.status.is_active()
                && record.friendship.user_low_id == pair.user_low_id
                && record.friendship.user_high_id == pair.user_high_id
        }) {
            return Err(ControlPlaneError::conflict(
                "friendship_pair_conflict",
                format!(
                    "active friendship already exists for pair {}:{}",
                    pair.user_low_id, pair.user_high_id
                ),
            ));
        }

        next_state.friendships.insert(
            friendship.friendship_id.clone(),
            StoredFriendship {
                friendship: friendship.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(ActivatedFriendship {
            friendship,
            latest_commit: commit,
            persistence,
        })
    }

    fn friendship_snapshot(
        &self,
        tenant_id: &str,
        friendship_id: &str,
    ) -> Option<StoredFriendship> {
        self.state
            .read()
            .expect("social runtime lock should not be poisoned")
            .friendships
            .get(friendship_id)
            .filter(|record| record.friendship.tenant_id == tenant_id)
            .cloned()
    }

    fn block_user(
        &self,
        tenant_id: &str,
        auth: &AuthContext,
        request: BlockUserRequest,
    ) -> Result<BlockedUser, ControlPlaneError> {
        validate_required_with_code("blockId", request.block_id.as_str(), "invalid_user_block")?;
        validate_required_with_code("eventId", request.event_id.as_str(), "invalid_user_block")?;
        validate_required_with_code(
            "blockerUserId",
            request.blocker_user_id.as_str(),
            "invalid_user_block",
        )?;
        validate_required_with_code(
            "blockedUserId",
            request.blocked_user_id.as_str(),
            "invalid_user_block",
        )?;
        validate_required_with_code(
            "effectiveAt",
            request.effective_at.as_str(),
            "invalid_user_block",
        )?;
        normalize_user_pair(
            request.blocker_user_id.as_str(),
            request.blocked_user_id.as_str(),
        )
        .map_err(|error| ControlPlaneError::invalid("invalid_user_block", error.to_string()))?;

        if matches!(request.scope, BlockScope::DirectChat) {
            validate_required_with_code(
                "directChatId",
                request.direct_chat_id.as_deref().unwrap_or_default(),
                "invalid_user_block",
            )?;
        }

        let scope = serde_json::to_string(&request.scope)
            .expect("user block scope should serialize")
            .trim_matches('"')
            .to_owned();
        let payload = UserBlockedPayload {
            block_id: request.block_id.clone(),
            blocker_user_id: request.blocker_user_id.clone(),
            blocked_user_id: request.blocked_user_id.clone(),
            scope,
            direct_chat_id: request.direct_chat_id.clone(),
            expires_at: request.expires_at.clone(),
            effective_at: request.effective_at.clone(),
        };
        let payload_json =
            serde_json::to_string(&payload).expect("user block payload should serialize into json");
        let commit = social_commit_envelope(
            request.event_id.as_str(),
            tenant_id,
            AggregateType::UserBlock,
            request.block_id.as_str(),
            SocialEventType::UserBlocked,
            1,
            EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            request.effective_at.as_str(),
            request.effective_at.as_str(),
            payload_json.as_str(),
        );
        let user_block = UserBlock {
            tenant_id: tenant_id.into(),
            block_id: request.block_id.clone(),
            blocker_user_id: request.blocker_user_id,
            blocked_user_id: request.blocked_user_id,
            scope: request.scope,
            status: UserBlockStatus::Active,
            direct_chat_id: request.direct_chat_id,
            expires_at: request.expires_at,
            created_at: request.effective_at.clone(),
            updated_at: request.effective_at,
        };

        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::UserBlock { record, commit } => Ok(BlockedUser {
                        user_block: record.user_block,
                        latest_commit: commit,
                        persistence,
                    }),
                    other => Err(social_event_id_conflict(request.event_id.as_str(), &other)),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .user_blocks
            .contains_key(user_block.block_id.as_str())
        {
            return Err(ControlPlaneError::conflict(
                "user_block_conflict",
                format!("user block {} already exists", user_block.block_id),
            ));
        }
        if next_state.user_blocks.values().any(|record| {
            record.user_block.tenant_id == tenant_id
                && record.user_block.status.is_active()
                && record.user_block.blocker_user_id == user_block.blocker_user_id
                && record.user_block.blocked_user_id == user_block.blocked_user_id
                && record.user_block.scope == user_block.scope
        }) {
            return Err(ControlPlaneError::conflict(
                "user_block_scope_conflict",
                format!(
                    "active user block already exists for {} -> {} scope {:?}",
                    user_block.blocker_user_id, user_block.blocked_user_id, user_block.scope
                ),
            ));
        }

        next_state.user_blocks.insert(
            user_block.block_id.clone(),
            StoredUserBlock {
                user_block: user_block.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(BlockedUser {
            user_block,
            latest_commit: commit,
            persistence,
        })
    }

    fn user_block_snapshot(&self, tenant_id: &str, block_id: &str) -> Option<StoredUserBlock> {
        self.state
            .read()
            .expect("social runtime lock should not be poisoned")
            .user_blocks
            .get(block_id)
            .filter(|record| record.user_block.tenant_id == tenant_id)
            .cloned()
    }

    fn bind_direct_chat(
        &self,
        tenant_id: &str,
        auth: &AuthContext,
        request: BindDirectChatRequest,
    ) -> Result<BoundDirectChat, ControlPlaneError> {
        validate_required_with_code(
            "directChatId",
            request.direct_chat_id.as_str(),
            "invalid_direct_chat",
        )?;
        validate_required_with_code("eventId", request.event_id.as_str(), "invalid_direct_chat")?;
        validate_required_with_code(
            "leftActorId",
            request.left_actor_id.as_str(),
            "invalid_direct_chat",
        )?;
        validate_required_with_code(
            "rightActorId",
            request.right_actor_id.as_str(),
            "invalid_direct_chat",
        )?;
        validate_required_with_code(
            "conversationId",
            request.conversation_id.as_str(),
            "invalid_direct_chat",
        )?;
        validate_required_with_code("boundAt", request.bound_at.as_str(), "invalid_direct_chat")?;
        let pair = normalize_actor_pair(
            request.left_actor_id.as_str(),
            request.right_actor_id.as_str(),
        )
        .map_err(|error| ControlPlaneError::invalid("invalid_direct_chat", error.to_string()))?;

        let payload = DirectChatBoundPayload {
            direct_chat_id: request.direct_chat_id.clone(),
            conversation_id: request.conversation_id.clone(),
            left_actor_id: pair.left_actor_id.clone(),
            right_actor_id: pair.right_actor_id.clone(),
            pair_hash: pair.pair_hash.clone(),
            bound_at: request.bound_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("direct chat payload should serialize into json");
        let commit = social_commit_envelope(
            request.event_id.as_str(),
            tenant_id,
            AggregateType::DirectChat,
            request.direct_chat_id.as_str(),
            SocialEventType::DirectChatBound,
            1,
            EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            request.bound_at.as_str(),
            request.bound_at.as_str(),
            payload_json.as_str(),
        );
        let direct_chat = DirectChat {
            tenant_id: tenant_id.into(),
            direct_chat_id: request.direct_chat_id.clone(),
            left_actor_id: pair.left_actor_id.clone(),
            right_actor_id: pair.right_actor_id.clone(),
            pair_hash: pair.pair_hash.clone(),
            status: DirectChatStatus::Active,
            conversation_id: Some(request.conversation_id),
            created_at: request.bound_at.clone(),
            updated_at: request.bound_at,
        };

        let mut state = self
            .state
            .write()
            .expect("social runtime lock should not be poisoned");
        let mut next_state = state.clone();
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::DirectChat { record, commit } => Ok(BoundDirectChat {
                        direct_chat: record.direct_chat,
                        latest_commit: commit,
                        persistence,
                    }),
                    other => Err(social_event_id_conflict(request.event_id.as_str(), &other)),
                }
            })?
        {
            return Ok(replayed);
        }
        if next_state
            .direct_chats
            .contains_key(direct_chat.direct_chat_id.as_str())
        {
            return Err(ControlPlaneError::conflict(
                "direct_chat_conflict",
                format!("direct chat {} already exists", direct_chat.direct_chat_id),
            ));
        }
        if next_state.direct_chats.values().any(|record| {
            record.direct_chat.tenant_id == tenant_id
                && record.direct_chat.status.is_active()
                && record.direct_chat.pair_hash == pair.pair_hash
        }) {
            return Err(ControlPlaneError::conflict(
                "direct_chat_pair_conflict",
                format!(
                    "active direct chat already exists for pair {}",
                    pair.pair_hash
                ),
            ));
        }

        next_state.direct_chats.insert(
            direct_chat.direct_chat_id.clone(),
            StoredDirectChat {
                direct_chat: direct_chat.clone(),
                commits: vec![commit.clone()],
            },
        );
        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(BoundDirectChat {
            direct_chat,
            latest_commit: commit,
            persistence,
        })
    }

    fn direct_chat_snapshot(
        &self,
        tenant_id: &str,
        direct_chat_id: &str,
    ) -> Option<StoredDirectChat> {
        self.state
            .read()
            .expect("social runtime lock should not be poisoned")
            .direct_chats
            .get(direct_chat_id)
            .filter(|record| record.direct_chat.tenant_id == tenant_id)
            .cloned()
    }
}

pub fn configured_runtime_dir() -> Option<PathBuf> {
    std::env::var("CRAW_CHAT_RUNTIME_DIR")
        .ok()
        .map(PathBuf::from)
}

pub fn configured_shared_channel_sync_target_base_url() -> Option<String> {
    std::env::var(SHARED_CHANNEL_SYNC_TARGET_BASE_URL_ENV)
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_owned())
        .filter(|value| !value.is_empty())
}

pub fn build_public_shared_channel_sync_trigger(
    base_url: impl AsRef<str>,
    public_bearer_secret: impl AsRef<str>,
) -> Result<Arc<dyn SharedChannelLinkedMemberSyncTrigger>, String> {
    Ok(Arc::new(PublicSharedChannelLinkedMemberSyncTrigger::new(
        base_url,
        public_bearer_secret,
    )?))
}

pub fn configured_public_shared_channel_sync_trigger()
-> Result<Option<Arc<dyn SharedChannelLinkedMemberSyncTrigger>>, String> {
    let Some(base_url) = configured_shared_channel_sync_target_base_url() else {
        return Ok(None);
    };

    let public_bearer_secret = std::env::var(PUBLIC_BEARER_HS256_SECRET_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "{} is required when {} is set",
                PUBLIC_BEARER_HS256_SECRET_ENV, SHARED_CHANNEL_SYNC_TARGET_BASE_URL_ENV
            )
        })?;

    build_public_shared_channel_sync_trigger(base_url, public_bearer_secret).map(Some)
}

pub fn repair_social_runtime_dir(
    runtime_dir: impl AsRef<StdPath>,
) -> Result<SocialRuntimeRepairResponse, String> {
    let runtime_dir = runtime_dir.as_ref();
    let state_dir = runtime_dir.join("state");
    let journal_path = state_dir.join(SOCIAL_COMMIT_JOURNAL_FILE_NAME);
    let tx_marker_path = state_dir.join(SOCIAL_TRANSACTION_MARKER_FILE_NAME);
    let state_store = SocialStateStore::file(state_dir.join(SOCIAL_STATE_FILE_NAME));
    if !journal_path.exists() {
        return Err(format!(
            "social commit journal is missing: {}",
            journal_path.display()
        ));
    }

    let snapshot_state = state_store.load().unwrap_or_default();
    let mut replayed_state =
        SocialControlRuntime::replay_state_from_commit_journal(journal_path.as_path())?;
    replayed_state.merge_pending_shared_channel_sync_requests_from(&snapshot_state);
    replayed_state.merge_dead_letter_shared_channel_sync_requests_from(&snapshot_state);
    state_store.save(&replayed_state)?;
    let transaction_marker_cleared = clear_social_transaction_marker(tx_marker_path.as_path())?;

    Ok(SocialRuntimeRepairResponse {
        status: SocialRuntimeRepairStatus::Repaired,
        journal_authority: true,
        snapshot_updated: true,
        transaction_marker_cleared,
        aggregate_counts: replayed_state.aggregate_counts(),
    })
}

pub fn format_social_runtime_dir_repair(report: &SocialRuntimeRepairResponse) -> String {
    let mut lines =
        vec![format!("social runtime repair status: {:?}", report.status).to_lowercase()];
    lines.push(format!("journal-authority: {}", report.journal_authority));
    lines.push(format!("snapshot-updated: {}", report.snapshot_updated));
    lines.push(format!(
        "transaction-marker-cleared: {}",
        report.transaction_marker_cleared
    ));
    lines.push(format!(
        "aggregate-counts: friendRequests={} friendships={} userBlocks={} directChats={} externalConnections={} externalMemberLinks={} sharedChannelPolicies={} pendingSharedChannelSyncRequests={} deadLetterSharedChannelSyncRequests={}",
        report.aggregate_counts.friend_requests,
        report.aggregate_counts.friendships,
        report.aggregate_counts.user_blocks,
        report.aggregate_counts.direct_chats,
        report.aggregate_counts.external_connections,
        report.aggregate_counts.external_member_links,
        report.aggregate_counts.shared_channel_policies,
        report.aggregate_counts.pending_shared_channel_sync_requests,
        report.aggregate_counts.dead_letter_shared_channel_sync_requests
    ));
    lines.join("\n")
}

fn build_social_runtime_from_env() -> Arc<SocialControlRuntime> {
    configured_runtime_dir()
        .map(SocialControlRuntime::from_runtime_dir)
        .map(Arc::new)
        .unwrap_or_else(|| Arc::new(SocialControlRuntime::default()))
}

pub fn build_app() -> Router {
    build_app_with_cluster(Arc::new(RealtimeClusterBridge::default()))
}

pub fn build_public_app() -> Router {
    build_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app_with_shared_channel_sync_trigger(
    shared_channel_sync_trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger>,
) -> Router {
    build_app_with_cluster_and_shared_channel_sync_trigger(
        Arc::new(RealtimeClusterBridge::default()),
        shared_channel_sync_trigger,
    )
}

pub fn build_public_app_with_shared_channel_sync_trigger(
    shared_channel_sync_trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger>,
) -> Router {
    build_app_with_shared_channel_sync_trigger(shared_channel_sync_trigger)
        .layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app_with_cluster(realtime_cluster: Arc<RealtimeClusterBridge>) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
        social_runtime: build_social_runtime_from_env(),
        shared_channel_sync_trigger: None,
    })
}

pub fn build_app_with_cluster_and_shared_channel_sync_trigger(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    shared_channel_sync_trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
        social_runtime: build_social_runtime_from_env(),
        shared_channel_sync_trigger: Some(shared_channel_sync_trigger),
    })
}

pub fn build_app_with_cluster_and_provider_registry(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<dyn ProviderRegistry>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry,
        provider_registry_runtime: None,
        governance_loop: None,
        social_runtime: build_social_runtime_from_env(),
        shared_channel_sync_trigger: None,
    })
}

pub fn build_app_with_cluster_and_runtime_provider_registry(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<RuntimeProviderRegistry>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: None,
        social_runtime: build_social_runtime_from_env(),
        shared_channel_sync_trigger: None,
    })
}

pub fn build_app_with_cluster_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
        social_runtime: build_social_runtime_from_env(),
        shared_channel_sync_trigger: None,
    })
}

pub fn build_app_with_cluster_and_governance_sinks_and_shared_channel_sync_trigger(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    shared_channel_sync_trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
        social_runtime: build_social_runtime_from_env(),
        shared_channel_sync_trigger: Some(shared_channel_sync_trigger),
    })
}

pub fn build_control_surface_with_cluster_and_governance_sinks_and_shared_channel_sync_trigger(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    shared_channel_sync_trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_control_surface_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
        social_runtime: build_social_runtime_from_env(),
        shared_channel_sync_trigger: Some(shared_channel_sync_trigger),
    })
}

pub fn build_app_with_cluster_and_governance_sinks_and_runtime_dir(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    runtime_dir: impl AsRef<StdPath>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
        social_runtime: Arc::new(SocialControlRuntime::from_runtime_dir(runtime_dir)),
        shared_channel_sync_trigger: None,
    })
}

pub fn build_app_with_cluster_and_governance_sinks_and_runtime_dir_with_shared_channel_sync_stale_reclaim_scheduler_config(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    runtime_dir: impl AsRef<StdPath>,
    scheduler_config: SharedChannelSyncStaleReclaimSchedulerConfig,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_app_with_state_and_scheduler_config(
        AppState {
            realtime_cluster,
            protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
            provider_registry: provider_registry.clone(),
            provider_registry_runtime: Some(provider_registry),
            governance_loop: Some(GovernanceLoop {
                ops_runtime,
                audit_runtime,
            }),
            social_runtime: Arc::new(SocialControlRuntime::from_runtime_dir(runtime_dir)),
            shared_channel_sync_trigger: None,
        },
        scheduler_config,
    )
}

pub fn build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    runtime_dir: impl AsRef<StdPath>,
    shared_channel_sync_trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger>,
) -> Router {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    build_control_surface_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
        social_runtime: Arc::new(SocialControlRuntime::from_runtime_dir(runtime_dir)),
        shared_channel_sync_trigger: Some(shared_channel_sync_trigger),
    })
}

pub fn build_app_with_cluster_provider_registry_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<dyn ProviderRegistry>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry,
        provider_registry_runtime: None,
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
        social_runtime: build_social_runtime_from_env(),
        shared_channel_sync_trigger: None,
    })
}

pub fn build_app_with_cluster_runtime_provider_registry_and_governance_sinks(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    provider_registry: Arc<RuntimeProviderRegistry>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
) -> Router {
    build_app_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
        social_runtime: build_social_runtime_from_env(),
        shared_channel_sync_trigger: None,
    })
}

fn build_app_with_state(state: AppState) -> Router {
    build_app_with_state_and_scheduler_config(
        state,
        resolve_shared_channel_sync_stale_reclaim_scheduler_config_from_env(),
    )
}

fn build_app_with_state_and_scheduler_config(
    state: AppState,
    scheduler_config: SharedChannelSyncStaleReclaimSchedulerConfig,
) -> Router {
    Router::new().route("/healthz", get(healthz)).merge(
        build_control_surface_with_state_and_scheduler_config(state, scheduler_config),
    )
}

fn build_control_surface_with_state(state: AppState) -> Router {
    build_control_surface_with_state_and_scheduler_config(
        state,
        resolve_shared_channel_sync_stale_reclaim_scheduler_config_from_env(),
    )
}

fn build_control_surface_with_state_and_scheduler_config(
    state: AppState,
    scheduler_config: SharedChannelSyncStaleReclaimSchedulerConfig,
) -> Router {
    state
        .social_runtime
        .start_shared_channel_sync_stale_reclaim_scheduler(scheduler_config);
    Router::new()
        .route(
            "/api/v1/control/protocol-registry",
            get(protocol_registry_snapshot),
        )
        .route(
            "/api/v1/control/protocol-governance",
            get(protocol_governance_snapshot),
        )
        .route(
            "/api/v1/control/provider-registry",
            get(provider_registry_snapshot),
        )
        .route(
            "/api/v1/control/provider-bindings",
            get(provider_bindings_snapshot).post(upsert_provider_binding_policy),
        )
        .route(
            "/api/v1/control/provider-policies",
            get(provider_policy_history),
        )
        .route(
            "/api/v1/control/provider-policies/diff",
            get(provider_policy_diff),
        )
        .route(
            "/api/v1/control/provider-policies/preview",
            post(provider_policy_preview),
        )
        .route(
            "/api/v1/control/provider-policies/rollback",
            post(rollback_provider_policy),
        )
        .route(
            "/api/v1/control/social/friend-requests",
            post(submit_friend_request),
        )
        .route(
            "/api/v1/control/social/friend-requests/{request_id}",
            get(friend_request_snapshot),
        )
        .route(
            "/api/v1/control/social/friendships",
            post(activate_friendship),
        )
        .route(
            "/api/v1/control/social/friendships/{friendship_id}",
            get(friendship_snapshot),
        )
        .route("/api/v1/control/social/user-blocks", post(block_user))
        .route(
            "/api/v1/control/social/user-blocks/{block_id}",
            get(user_block_snapshot),
        )
        .route(
            "/api/v1/control/social/direct-chats/bindings",
            post(bind_direct_chat),
        )
        .route(
            "/api/v1/control/social/direct-chats/{direct_chat_id}",
            get(direct_chat_snapshot),
        )
        .route(
            "/api/v1/control/social/external-connections",
            post(establish_external_connection),
        )
        .route(
            "/api/v1/control/social/external-connections/{connection_id}",
            get(external_connection_snapshot),
        )
        .route(
            "/api/v1/control/social/external-member-links",
            post(bind_external_member_link),
        )
        .route(
            "/api/v1/control/social/external-member-links/{link_id}",
            get(external_member_link_snapshot),
        )
        .route(
            "/api/v1/control/social/shared-channel-policies",
            post(apply_shared_channel_policy),
        )
        .route(
            "/api/v1/control/social/shared-channel-policies/{policy_id}",
            get(shared_channel_policy_snapshot),
        )
        .route(
            "/api/v1/control/social/runtime/repair-derived-snapshot",
            post(repair_social_runtime_snapshot),
        )
        .route(
            "/api/v1/control/social/runtime/dead-letter-shared-channel-sync",
            get(dead_letter_social_runtime_shared_channel_sync),
        )
        .route(
            "/api/v1/control/social/runtime/pending-shared-channel-sync",
            get(pending_social_runtime_shared_channel_sync),
        )
        .route(
            "/api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync",
            post(reclaim_stale_pending_social_runtime_shared_channel_sync),
        )
        .route(
            "/api/v1/control/social/runtime/repair-shared-channel-sync",
            post(repair_social_runtime_shared_channel_sync),
        )
        .route(
            "/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync",
            post(requeue_dead_letter_social_runtime_shared_channel_sync),
        )
        .route(
            "/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted",
            post(requeue_dead_letter_social_runtime_shared_channel_sync_targeted),
        )
        .route(
            "/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted",
            post(claim_pending_social_runtime_shared_channel_sync_targeted),
        )
        .route(
            "/api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted",
            post(release_pending_social_runtime_shared_channel_sync_targeted),
        )
        .route(
            "/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted",
            post(takeover_pending_social_runtime_shared_channel_sync_targeted),
        )
        .route(
            "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
            post(republish_pending_social_runtime_shared_channel_sync_targeted),
        )
        .route("/api/v1/control/nodes/{node_id}/drain", post(drain_node))
        .route(
            "/api/v1/control/nodes/{node_id}/activate",
            post(activate_node),
        )
        .route(
            "/api/v1/control/nodes/{node_id}/routes/migrate",
            post(migrate_node_routes),
        )
        .with_state(state)
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => ControlPlaneError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "control-plane-api",
    })
}

async fn protocol_registry_snapshot(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProtocolRegistryResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    Ok(Json(ProtocolRegistryResponse {
        protocol_version: state.protocol_registry.protocol_version().to_owned(),
        bindings: state.protocol_registry.bindings().iter().cloned().collect(),
        codecs: state.protocol_registry.codecs().iter().cloned().collect(),
        schemas: state
            .protocol_registry
            .schemas()
            .values()
            .map(schema_response)
            .collect(),
        compatibility_matrix: state
            .protocol_registry
            .compatibility_matrix()
            .values()
            .map(compatibility_response)
            .collect(),
    }))
}

async fn protocol_governance_snapshot(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProtocolGovernanceResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    let governance = state
        .protocol_registry
        .governance_snapshot()
        .ok_or_else(|| {
            ControlPlaneError::service_unavailable(
                "protocol_governance_unavailable",
                "control plane governance snapshot is not initialized",
            )
        })?;

    Ok(Json(governance_response(
        governance,
        state.protocol_registry.as_ref(),
    )))
}

async fn provider_registry_snapshot(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderRegistrySnapshotResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;
    Ok(Json(provider_registry_snapshot_response(
        state.provider_registry.snapshot(),
    )))
}

async fn provider_bindings_snapshot(
    headers: HeaderMap,
    Query(query): Query<ProviderBindingsQuery>,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingsResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    let response = provider_bindings_response(state.provider_registry.as_ref(), query.tenant_id);
    mirror_provider_bindings_into_ops_runtime(&state, &response);

    Ok(Json(response))
}

async fn upsert_provider_binding_policy(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Result<Json<ProviderBindingCommitResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_write_unavailable",
            "control plane provider policy write is not enabled for this registry",
        )
    })?;

    let (action, aggregate_id, selection_source, commit) =
        if let Some(tenant_id) = request.tenant_id.as_deref() {
            let commit = provider_registry.commit_upsert(
                Some(tenant_id),
                request.domain,
                request.plugin_id.as_str(),
                request.expected_base_version,
            )?;
            (
                "control.provider_tenant_override_updated",
                format!(
                    "tenant:{tenant_id}:{}",
                    provider_domain_name(request.domain)
                ),
                "tenant_override",
                commit,
            )
        } else {
            let commit = provider_registry.commit_upsert(
                None,
                request.domain,
                request.plugin_id.as_str(),
                request.expected_base_version,
            )?;
            (
                "control.provider_deployment_profile_updated",
                format!("deployment:{}", provider_domain_name(request.domain)),
                "deployment_profile",
                commit,
            )
        };

    if commit.applied {
        mirror_all_provider_bindings_into_ops_runtime(&state, provider_registry.as_ref());
    }
    let response = provider_bindings_response(state.provider_registry.as_ref(), request.tenant_id);
    if commit.applied {
        record_control_plane_audit(
            &state,
            &auth,
            action,
            "provider_policy",
            aggregate_id,
            serde_json::json!({
                "tenantId": response.tenant_id,
                "domain": provider_domain_name(request.domain),
                "pluginId": request.plugin_id,
                "expectedBaseVersion": request.expected_base_version,
                "selectionSource": selection_source
            }),
        );
    }

    Ok(Json(provider_binding_commit_response(response, commit)))
}

async fn provider_policy_history(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderPolicyHistoryResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_history_unavailable",
            "control plane provider policy history is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_policy_history_response(
        ProviderPolicyReadStatus::History,
        provider_registry.policy_history(),
    )))
}

async fn provider_policy_diff(
    headers: HeaderMap,
    Query(query): Query<ProviderPolicyDiffQuery>,
    State(state): State<AppState>,
) -> Result<Json<ProviderPolicyDiffResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_diff_unavailable",
            "control plane provider policy diff is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_policy_diff_response(
        ProviderPolicyReadStatus::Diff,
        provider_registry.diff_versions(query.from_version, query.to_version)?,
    )))
}

async fn provider_policy_preview(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Result<Json<ProviderPolicyPreview>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_preview_unavailable",
            "control plane provider policy preview is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_registry.preview_upsert(
        request.tenant_id.as_deref(),
        request.domain,
        request.plugin_id.as_str(),
    )?))
}

async fn rollback_provider_policy(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ProviderPolicyRollbackRequest>,
) -> Result<Json<ProviderPolicyHistoryResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_rollback_unavailable",
            "control plane provider policy rollback is not enabled for this registry",
        )
    })?;

    let rollback_snapshot = provider_registry.rollback_to(request.target_version)?;
    mirror_all_provider_bindings_into_ops_runtime(&state, provider_registry.as_ref());
    let history = provider_registry.policy_history();
    record_control_plane_audit(
        &state,
        &auth,
        "control.provider_policy_rolled_back",
        "provider_policy",
        format!("version:{}", rollback_snapshot.version),
        serde_json::json!({
            "targetVersion": request.target_version,
            "currentVersion": history.current_version,
            "rollbackFromVersion": rollback_snapshot.rollback_from_version
        }),
    );

    Ok(Json(provider_policy_history_response(
        ProviderPolicyReadStatus::RolledBack,
        history,
    )))
}

async fn submit_friend_request(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SubmitFriendRequestRequest>,
) -> Result<Json<SocialFriendRequestCommitResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let submitted =
        state
            .social_runtime
            .submit_friend_request(auth.tenant_id.as_str(), &auth, request)?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.friend_request_submitted",
        "friend_request",
        submitted.friend_request.request_id.clone(),
        serde_json::json!({
            "requestId": submitted.friend_request.request_id,
            "requesterUserId": submitted.friend_request.requester_user_id,
            "targetUserId": submitted.friend_request.target_user_id,
            "eventId": submitted.latest_commit.event_id
        }),
    );

    Ok(Json(SocialFriendRequestCommitResponse {
        status: SocialFriendRequestWriteStatus::Submitted,
        friend_request: submitted.friend_request,
        latest_commit: submitted.latest_commit.into(),
        persistence: submitted.persistence,
    }))
}

async fn friend_request_snapshot(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendRequestSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    let snapshot = state
        .social_runtime
        .friend_request_snapshot(auth.tenant_id.as_str(), request_id.as_str())
        .ok_or_else(|| {
            ControlPlaneError::not_found(
                "friend_request_not_found",
                format!("friend request {request_id} was not found"),
            )
        })?;

    Ok(Json(SocialFriendRequestSnapshotResponse {
        status: SocialFriendRequestReadStatus::Snapshot,
        friend_request: snapshot.friend_request,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

async fn activate_friendship(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ActivateFriendshipRequest>,
) -> Result<Json<SocialFriendshipCommitResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let activated =
        state
            .social_runtime
            .activate_friendship(auth.tenant_id.as_str(), &auth, request)?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.friendship_activated",
        "friendship",
        activated.friendship.friendship_id.clone(),
        serde_json::json!({
            "friendshipId": activated.friendship.friendship_id,
            "userLowId": activated.friendship.user_low_id,
            "userHighId": activated.friendship.user_high_id,
            "initiatorUserId": activated.friendship.initiator_user_id,
            "eventId": activated.latest_commit.event_id
        }),
    );

    Ok(Json(SocialFriendshipCommitResponse {
        status: SocialFriendshipWriteStatus::Activated,
        friendship: activated.friendship,
        latest_commit: activated.latest_commit.into(),
        persistence: activated.persistence,
    }))
}

async fn friendship_snapshot(
    Path(friendship_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendshipSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    let snapshot = state
        .social_runtime
        .friendship_snapshot(auth.tenant_id.as_str(), friendship_id.as_str())
        .ok_or_else(|| {
            ControlPlaneError::not_found(
                "friendship_not_found",
                format!("friendship {friendship_id} was not found"),
            )
        })?;

    Ok(Json(SocialFriendshipSnapshotResponse {
        status: SocialFriendshipReadStatus::Snapshot,
        friendship: snapshot.friendship,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

async fn block_user(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<BlockUserRequest>,
) -> Result<Json<SocialUserBlockCommitResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let blocked = state
        .social_runtime
        .block_user(auth.tenant_id.as_str(), &auth, request)?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.user_block_blocked",
        "user_block",
        blocked.user_block.block_id.clone(),
        serde_json::json!({
            "blockId": blocked.user_block.block_id,
            "blockerUserId": blocked.user_block.blocker_user_id,
            "blockedUserId": blocked.user_block.blocked_user_id,
            "scope": blocked.user_block.scope,
            "directChatId": blocked.user_block.direct_chat_id,
            "eventId": blocked.latest_commit.event_id
        }),
    );

    Ok(Json(SocialUserBlockCommitResponse {
        status: SocialUserBlockWriteStatus::Blocked,
        user_block: blocked.user_block,
        latest_commit: blocked.latest_commit.into(),
        persistence: blocked.persistence,
    }))
}

async fn user_block_snapshot(
    Path(block_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialUserBlockSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    let snapshot = state
        .social_runtime
        .user_block_snapshot(auth.tenant_id.as_str(), block_id.as_str())
        .ok_or_else(|| {
            ControlPlaneError::not_found(
                "user_block_not_found",
                format!("user block {block_id} was not found"),
            )
        })?;

    Ok(Json(SocialUserBlockSnapshotResponse {
        status: SocialUserBlockReadStatus::Snapshot,
        user_block: snapshot.user_block,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

async fn bind_direct_chat(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<BindDirectChatRequest>,
) -> Result<Json<SocialDirectChatCommitResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let bound = state
        .social_runtime
        .bind_direct_chat(auth.tenant_id.as_str(), &auth, request)?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.direct_chat_bound",
        "direct_chat",
        bound.direct_chat.direct_chat_id.clone(),
        serde_json::json!({
            "directChatId": bound.direct_chat.direct_chat_id,
            "leftActorId": bound.direct_chat.left_actor_id,
            "rightActorId": bound.direct_chat.right_actor_id,
            "pairHash": bound.direct_chat.pair_hash,
            "conversationId": bound.direct_chat.conversation_id,
            "eventId": bound.latest_commit.event_id
        }),
    );

    Ok(Json(SocialDirectChatCommitResponse {
        status: SocialDirectChatWriteStatus::Bound,
        direct_chat: bound.direct_chat,
        latest_commit: bound.latest_commit.into(),
        persistence: bound.persistence,
    }))
}

async fn direct_chat_snapshot(
    Path(direct_chat_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialDirectChatSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    let snapshot = state
        .social_runtime
        .direct_chat_snapshot(auth.tenant_id.as_str(), direct_chat_id.as_str())
        .ok_or_else(|| {
            ControlPlaneError::not_found(
                "direct_chat_not_found",
                format!("direct chat {direct_chat_id} was not found"),
            )
        })?;

    Ok(Json(SocialDirectChatSnapshotResponse {
        status: SocialDirectChatReadStatus::Snapshot,
        direct_chat: snapshot.direct_chat,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

async fn establish_external_connection(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<EstablishExternalConnectionRequest>,
) -> Result<Json<SocialExternalConnectionCommitResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let established = state.social_runtime.establish_external_connection(
        auth.tenant_id.as_str(),
        &auth,
        request,
    )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.external_connection_established",
        "external_connection",
        established.external_connection.connection_id.clone(),
        serde_json::json!({
            "connectionId": established.external_connection.connection_id,
            "externalTenantId": established.external_connection.external_tenant_id,
            "connectionKind": established.external_connection.connection_kind,
            "eventId": established.latest_commit.event_id
        }),
    );

    Ok(Json(SocialExternalConnectionCommitResponse {
        status: SocialExternalConnectionWriteStatus::Established,
        external_connection: established.external_connection,
        latest_commit: established.latest_commit.into(),
        persistence: established.persistence,
    }))
}

async fn external_connection_snapshot(
    Path(connection_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialExternalConnectionSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    let snapshot = state
        .social_runtime
        .external_connection_snapshot(auth.tenant_id.as_str(), connection_id.as_str())
        .ok_or_else(|| {
            ControlPlaneError::not_found(
                "external_connection_not_found",
                format!("external connection {connection_id} was not found"),
            )
        })?;

    Ok(Json(SocialExternalConnectionSnapshotResponse {
        status: SocialExternalConnectionReadStatus::Snapshot,
        external_connection: snapshot.external_connection,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

async fn bind_external_member_link(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<BindExternalMemberLinkRequest>,
) -> Result<Json<SocialExternalMemberLinkCommitResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let bound =
        state
            .social_runtime
            .bind_external_member_link(auth.tenant_id.as_str(), &auth, request)?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.external_member_link_bound",
        "external_member_link",
        bound.external_member_link.link_id.clone(),
        serde_json::json!({
            "linkId": bound.external_member_link.link_id,
            "connectionId": bound.external_member_link.connection_id,
            "localActorId": bound.external_member_link.local_actor_id,
            "localActorKind": bound.external_member_link.local_actor_kind,
            "externalMemberId": bound.external_member_link.external_member_id,
            "eventId": bound.latest_commit.event_id
        }),
    );
    dispatch_shared_channel_sync_requests(&state, &auth, &bound.shared_channel_sync_requests)?;

    Ok(Json(SocialExternalMemberLinkCommitResponse {
        status: SocialExternalMemberLinkWriteStatus::Bound,
        external_member_link: bound.external_member_link,
        latest_commit: bound.latest_commit.into(),
        persistence: bound.persistence,
    }))
}

async fn external_member_link_snapshot(
    Path(link_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialExternalMemberLinkSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    let snapshot = state
        .social_runtime
        .external_member_link_snapshot(auth.tenant_id.as_str(), link_id.as_str())
        .ok_or_else(|| {
            ControlPlaneError::not_found(
                "external_member_link_not_found",
                format!("external member link {link_id} was not found"),
            )
        })?;

    Ok(Json(SocialExternalMemberLinkSnapshotResponse {
        status: SocialExternalMemberLinkReadStatus::Snapshot,
        external_member_link: snapshot.external_member_link,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

async fn apply_shared_channel_policy(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ApplySharedChannelPolicyRequest>,
) -> Result<Json<SocialSharedChannelPolicyCommitResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let applied = state.social_runtime.apply_shared_channel_policy(
        auth.tenant_id.as_str(),
        &auth,
        request,
    )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.shared_channel_policy_applied",
        "shared_channel_policy",
        applied.shared_channel_policy.policy_id.clone(),
        serde_json::json!({
            "policyId": applied.shared_channel_policy.policy_id,
            "connectionId": applied.shared_channel_policy.connection_id,
            "channelId": applied.shared_channel_policy.channel_id,
            "historyVisibility": applied.shared_channel_policy.history_visibility,
            "policyVersion": applied.shared_channel_policy.policy_version,
            "eventId": applied.latest_commit.event_id
        }),
    );
    dispatch_shared_channel_sync_requests(&state, &auth, &applied.shared_channel_sync_requests)?;

    Ok(Json(SocialSharedChannelPolicyCommitResponse {
        status: SocialSharedChannelPolicyWriteStatus::Applied,
        shared_channel_policy: applied.shared_channel_policy,
        latest_commit: applied.latest_commit.into(),
        persistence: applied.persistence,
    }))
}

async fn shared_channel_policy_snapshot(
    Path(policy_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelPolicySnapshotResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;

    let snapshot = state
        .social_runtime
        .shared_channel_policy_snapshot(auth.tenant_id.as_str(), policy_id.as_str())
        .ok_or_else(|| {
            ControlPlaneError::not_found(
                "shared_channel_policy_not_found",
                format!("shared channel policy {policy_id} was not found"),
            )
        })?;

    Ok(Json(SocialSharedChannelPolicySnapshotResponse {
        status: SocialSharedChannelPolicyReadStatus::Snapshot,
        shared_channel_policy: snapshot.shared_channel_policy,
        commits: snapshot.commits.into_iter().map(Into::into).collect(),
    }))
}

async fn repair_social_runtime_snapshot(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialRuntimeRepairResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let repair = state.social_runtime.repair_derived_snapshot()?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.social_runtime_derived_snapshot_repaired",
        "social_runtime",
        "derived_snapshot".into(),
        serde_json::json!({
            "journalAuthority": repair.journal_authority,
            "snapshotUpdated": repair.snapshot_updated,
            "transactionMarkerCleared": repair.transaction_marker_cleared,
            "aggregateCounts": &repair.aggregate_counts
        }),
    );

    Ok(Json(repair))
}

async fn dead_letter_social_runtime_shared_channel_sync(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeadLetterInventoryResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;
    let can_takeover = auth.has_permission("control.write");

    Ok(Json(
        state
            .social_runtime
            .dead_letter_shared_channel_sync_inventory(
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                can_takeover,
            ),
    ))
}

async fn pending_social_runtime_shared_channel_sync(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncPendingInventoryResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_read_access(&auth)?;
    let can_takeover = auth.has_permission("control.write");

    Ok(Json(
        state.social_runtime.pending_shared_channel_sync_inventory(
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            can_takeover,
        ),
    ))
}

async fn reclaim_stale_pending_social_runtime_shared_channel_sync(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncPendingStaleReclaimResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let reclaim = state
        .social_runtime
        .reclaim_stale_pending_shared_channel_sync_claims()?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.social_runtime_shared_channel_sync_pending_stale_reclaimed",
        "social_runtime",
        "shared_channel_sync_pending".into(),
        serde_json::json!({
            "status": reclaim.status,
            "pendingBefore": reclaim.pending_before,
            "reclaimed": reclaim.reclaimed,
            "pendingAfter": reclaim.pending_after
        }),
    );

    Ok(Json(reclaim))
}

async fn repair_social_runtime_shared_channel_sync(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncRepairResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let repair = state.social_runtime.repair_shared_channel_sync(
        state
            .shared_channel_sync_trigger
            .as_ref()
            .map(|trigger| trigger.as_ref()),
    )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.social_runtime_shared_channel_sync_repaired",
        "social_runtime",
        "shared_channel_sync".into(),
        serde_json::json!({
            "status": repair.status,
            "pendingBefore": repair.pending_before,
            "attempted": repair.attempted,
            "dispatched": repair.dispatched,
            "failed": repair.failed,
            "reclaimed": repair.reclaimed,
            "pendingAfter": repair.pending_after,
            "deadLetterBefore": repair.dead_letter_before,
            "deadLettered": repair.dead_lettered,
            "deadLetterAfter": repair.dead_letter_after
        }),
    );

    Ok(Json(repair))
}

async fn requeue_dead_letter_social_runtime_shared_channel_sync(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeadLetterRequeueResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let requeue = state
        .social_runtime
        .requeue_dead_letter_shared_channel_sync()?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.social_runtime_shared_channel_sync_dead_letter_requeued",
        "social_runtime",
        "shared_channel_sync_dead_letter".into(),
        serde_json::json!({
            "status": requeue.status,
            "pendingBefore": requeue.pending_before,
            "deadLetterBefore": requeue.dead_letter_before,
            "requeued": requeue.requeued,
            "pendingAfter": requeue.pending_after,
            "deadLetterAfter": requeue.dead_letter_after
        }),
    );

    Ok(Json(requeue))
}

async fn requeue_dead_letter_social_runtime_shared_channel_sync_targeted(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncDeadLetterTargetedRequeueRequest>,
) -> Result<Json<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let requeue = state
        .social_runtime
        .requeue_dead_letter_shared_channel_sync_targeted(&request.request_keys)?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.social_runtime_shared_channel_sync_dead_letter_targeted_requeued",
        "social_runtime",
        "shared_channel_sync_dead_letter".into(),
        serde_json::json!({
            "status": requeue.status,
            "pendingBefore": requeue.pending_before,
            "deadLetterBefore": requeue.dead_letter_before,
            "requested": requeue.requested,
            "requeued": requeue.requeued,
            "pendingAfter": requeue.pending_after,
            "deadLetterAfter": requeue.dead_letter_after,
            "requestKeys": request.request_keys
        }),
    );

    Ok(Json(requeue))
}

async fn claim_pending_social_runtime_shared_channel_sync_targeted(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncPendingTargetedClaimRequest>,
) -> Result<Json<SocialSharedChannelSyncPendingClaimResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let claim = state
        .social_runtime
        .claim_pending_shared_channel_sync_targeted(
            &request.request_keys,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.social_runtime_shared_channel_sync_pending_targeted_claimed",
        "social_runtime",
        "shared_channel_sync_pending".into(),
        serde_json::json!({
            "status": claim.status,
            "pendingBefore": claim.pending_before,
            "requested": claim.requested,
            "claimed": claim.claimed,
            "conflicted": claim.conflicted,
            "conflictItems": claim.conflict_items.clone(),
            "pendingAfter": claim.pending_after,
            "requestKeys": request.request_keys
        }),
    );

    Ok(Json(claim))
}

async fn release_pending_social_runtime_shared_channel_sync_targeted(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncPendingTargetedReleaseRequest>,
) -> Result<Json<SocialSharedChannelSyncPendingReleaseResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let release = state
        .social_runtime
        .release_pending_shared_channel_sync_targeted(
            &request.request_keys,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
        )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.social_runtime_shared_channel_sync_pending_targeted_released",
        "social_runtime",
        "shared_channel_sync_pending".into(),
        serde_json::json!({
            "status": release.status,
            "pendingBefore": release.pending_before,
            "requested": release.requested,
            "released": release.released,
            "conflicted": release.conflicted,
            "pendingAfter": release.pending_after,
            "requestKeys": request.request_keys
        }),
    );

    Ok(Json(release))
}

async fn takeover_pending_social_runtime_shared_channel_sync_targeted(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncPendingTargetedTakeoverRequest>,
) -> Result<Json<SocialSharedChannelSyncPendingTakeoverResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let takeover = state
        .social_runtime
        .takeover_pending_shared_channel_sync_targeted(
            &request.request_keys,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            request.allow_legacy_untracked,
        )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.social_runtime_shared_channel_sync_pending_targeted_taken_over",
        "social_runtime",
        "shared_channel_sync_pending".into(),
        serde_json::json!({
            "status": takeover.status,
            "pendingBefore": takeover.pending_before,
            "requested": takeover.requested,
            "takenOver": takeover.taken_over,
            "pendingAfter": takeover.pending_after,
            "legacyOverrideUsed": takeover.legacy_override_used,
            "allowLegacyUntracked": request.allow_legacy_untracked,
            "requestKeys": request.request_keys
        }),
    );

    Ok(Json(takeover))
}

async fn republish_pending_social_runtime_shared_channel_sync_targeted(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncTargetedRepublishRequest>,
) -> Result<Json<SocialSharedChannelSyncTargetedRepublishResponse>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;

    let republish = state
        .social_runtime
        .republish_pending_shared_channel_sync_targeted(
            &request.request_keys,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            state
                .shared_channel_sync_trigger
                .as_ref()
                .map(|trigger| trigger.as_ref()),
        )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.social_runtime_shared_channel_sync_pending_targeted_republished",
        "social_runtime",
        "shared_channel_sync_pending".into(),
        serde_json::json!({
            "status": republish.status,
            "pendingBefore": republish.pending_before,
            "requested": republish.requested,
            "attempted": republish.attempted,
            "dispatched": republish.dispatched,
            "failed": republish.failed,
            "pendingAfter": republish.pending_after,
            "deadLetterBefore": republish.dead_letter_before,
            "deadLettered": republish.dead_lettered,
            "deadLetterAfter": republish.dead_letter_after,
            "requestKeys": request.request_keys
        }),
    );

    Ok(Json(republish))
}

async fn drain_node(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;
    let lifecycle = state
        .realtime_cluster
        .mark_node_draining(node_id.as_str())?;
    mirror_node_into_ops_runtime(&state, node_id.as_str());
    record_control_plane_audit(
        &state,
        &auth,
        "control.node_draining_marked",
        "control_node",
        node_id.clone(),
        serde_json::json!({
            "nodeId": node_id,
            "drainStatus": lifecycle.drain_status,
            "rebalanceState": lifecycle.rebalance_state,
            "ownedRouteCount": lifecycle.owned_route_count
        }),
    );
    Ok(Json(lifecycle))
}

async fn activate_node(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;
    let lifecycle = state.realtime_cluster.activate_node(node_id.as_str())?;
    mirror_node_into_ops_runtime(&state, node_id.as_str());
    record_control_plane_audit(
        &state,
        &auth,
        "control.node_activated",
        "control_node",
        node_id.clone(),
        serde_json::json!({
            "nodeId": node_id,
            "drainStatus": lifecycle.drain_status,
            "rebalanceState": lifecycle.rebalance_state,
            "ownedRouteCount": lifecycle.owned_route_count
        }),
    );
    Ok(Json(lifecycle))
}

async fn migrate_node_routes(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<MigrateRoutesRequest>,
) -> Result<Json<RealtimeRouteMigrationResult>, ControlPlaneError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_control_write_access(&auth)?;
    let migration = state
        .realtime_cluster
        .migrate_node_routes(node_id.as_str(), request.target_node_id.as_str())?;
    mirror_node_into_ops_runtime(&state, node_id.as_str());
    mirror_node_into_ops_runtime(&state, request.target_node_id.as_str());
    record_control_plane_audit(
        &state,
        &auth,
        "control.node_routes_migrated",
        "control_node",
        node_id.clone(),
        serde_json::json!({
            "sourceNodeId": migration.source_node_id,
            "targetNodeId": migration.target_node_id,
            "migratedRouteCount": migration.migrated_route_count,
            "sourceDrainStatus": migration.source_drain_status,
            "sourceRebalanceState": migration.source_rebalance_state,
            "targetDrainStatus": migration.target_drain_status,
            "targetRebalanceState": migration.target_rebalance_state
        }),
    );
    Ok(Json(migration))
}

fn mirror_node_into_ops_runtime(state: &AppState, node_id: &str) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };
    if governance_loop.ops_runtime.node_id() != node_id {
        return;
    }

    let Some(lifecycle) = state.realtime_cluster.node_lifecycle(node_id) else {
        return;
    };
    governance_loop.ops_runtime.set_node_lifecycle(
        lifecycle.drain_status.as_str(),
        lifecycle.rebalance_state.as_str(),
    );
    governance_loop.ops_runtime.update_route_ownership(
        state
            .realtime_cluster
            .routes_for_node(node_id)
            .into_iter()
            .map(|route| RouteOwnershipView {
                tenant_id: route.tenant_id,
                principal_id: route.principal_id,
                device_id: route.device_id,
                owner_node_id: route.owner_node_id,
                connection_kind: route.connection_kind,
                bound_at: route.bound_at,
            })
            .collect(),
    );
}

fn mirror_provider_bindings_into_ops_runtime(
    state: &AppState,
    response: &ProviderBindingsResponse,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };

    governance_loop
        .ops_runtime
        .update_provider_binding_snapshot(provider_binding_snapshot_view(response));
}

fn mirror_all_provider_bindings_into_ops_runtime(
    state: &AppState,
    provider_registry: &RuntimeProviderRegistry,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };

    let mut snapshots = vec![provider_binding_snapshot_view(&provider_bindings_response(
        provider_registry,
        None,
    ))];
    let mut tenant_ids = provider_registry.tenant_ids_with_overrides();
    tenant_ids.sort();
    snapshots.extend(tenant_ids.into_iter().map(|tenant_id| {
        provider_binding_snapshot_view(&provider_bindings_response(
            provider_registry,
            Some(tenant_id),
        ))
    }));
    governance_loop
        .ops_runtime
        .replace_provider_binding_snapshots(snapshots);
}

fn dispatch_shared_channel_sync_requests(
    state: &AppState,
    auth: &AuthContext,
    requests: &[SharedChannelLinkedMemberSyncRequest],
) -> Result<(), ControlPlaneError> {
    if requests.is_empty() {
        return Ok(());
    }

    state
        .social_runtime
        .reclaim_stale_pending_shared_channel_sync_claims_if_any(
            "failed to persist shared-channel sync stale-claim reclaim before dispatch",
        )?;

    let Some(trigger) = state.shared_channel_sync_trigger.as_ref() else {
        state
            .social_runtime
            .persist_failed_shared_channel_sync_requests(
                requests,
                "shared-channel sync trigger is not configured",
            )?;
        return Ok(());
    };

    let dispatch_queue = state
        .social_runtime
        .pending_shared_channel_sync_dispatch_queue(requests);
    for (index, request) in dispatch_queue.iter().enumerate() {
        match trigger.trigger(request.clone()) {
            Ok(()) => {
                state
                    .social_runtime
                    .clear_pending_shared_channel_sync_request_best_effort(request);
                record_control_plane_audit(
                    state,
                    auth,
                    "control.shared_channel_linked_member_sync_triggered",
                    "shared_channel_sync",
                    format!(
                        "{}:{}:{}",
                        request.shared_channel_policy_id,
                        request.conversation_id,
                        request.local_actor_id
                    ),
                    serde_json::to_value(request)
                        .expect("shared channel sync request should serialize into audit payload"),
                );
            }
            Err(error) => {
                state
                    .social_runtime
                    .persist_failed_shared_channel_sync_requests(
                        &dispatch_queue[index..],
                        error.as_str(),
                    )?;
                return Err(ControlPlaneError::service_unavailable(
                    "shared_channel_sync_unavailable",
                    format!(
                        "failed to dispatch shared-channel linked-member sync for policy {} and actor {}: {error}",
                        request.shared_channel_policy_id, request.local_actor_id
                    ),
                ));
            }
        }
    }

    Ok(())
}

fn record_control_plane_audit(
    state: &AppState,
    auth: &AuthContext,
    action: &str,
    aggregate_type: &str,
    aggregate_id: String,
    payload: serde_json::Value,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };
    let record_id = format!("{aggregate_type}:{aggregate_id}:{action}");
    let payload =
        serde_json::to_string(&payload).expect("control plane audit payload should serialize");
    governance_loop.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id,
            aggregate_type: aggregate_type.into(),
            aggregate_id,
            action: action.into(),
            payload: Some(payload),
        },
    );
}

fn provider_bindings_response(
    provider_registry: &dyn ProviderRegistry,
    tenant_id: Option<String>,
) -> ProviderBindingsResponse {
    let precedence = provider_registry.snapshot().precedence;
    let effective_bindings = ProviderDomain::ALL
        .into_iter()
        .filter_map(|domain| provider_registry.effective_binding(domain, tenant_id.as_deref()))
        .collect();
    ProviderBindingsResponse {
        status: ProviderSurfaceReadStatus::Bindings,
        interface_version: PROVIDER_REGISTRY_INTERFACE_VERSION.into(),
        tenant_id,
        effective_bindings,
        precedence,
    }
}

fn provider_binding_commit_response(
    response: ProviderBindingsResponse,
    commit: ProviderPolicyCommit,
) -> ProviderBindingCommitResponse {
    ProviderBindingCommitResponse {
        status: commit.status,
        applied: commit.applied,
        interface_version: response.interface_version,
        tenant_id: response.tenant_id,
        current_version: commit.current_version,
        committed_binding: commit.committed_binding,
        diff: commit.diff,
        effective_bindings: response.effective_bindings,
        precedence: response.precedence,
    }
}

fn provider_registry_snapshot_response(
    snapshot: ProviderRegistrySnapshot,
) -> ProviderRegistrySnapshotResponse {
    ProviderRegistrySnapshotResponse {
        status: ProviderSurfaceReadStatus::Registry,
        snapshot,
    }
}

fn provider_policy_history_response(
    status: ProviderPolicyReadStatus,
    history: ProviderPolicyHistory,
) -> ProviderPolicyHistoryResponse {
    ProviderPolicyHistoryResponse { status, history }
}

fn provider_policy_diff_response(
    status: ProviderPolicyReadStatus,
    diff: ProviderPolicyDiff,
) -> ProviderPolicyDiffResponse {
    ProviderPolicyDiffResponse { status, diff }
}

fn provider_binding_snapshot_view(
    response: &ProviderBindingsResponse,
) -> ProviderBindingSnapshotView {
    ProviderBindingSnapshotView {
        interface_version: response.interface_version.clone(),
        tenant_id: response.tenant_id.clone(),
        effective_bindings: response
            .effective_bindings
            .iter()
            .map(|binding| ProviderBindingItemView {
                domain: provider_domain_name(binding.domain).into(),
                default_plugin_id: binding.default_plugin_id.clone(),
                selected_plugin_id: binding.selected_plugin_id.clone(),
                selection_source: binding.selection_source.clone(),
                tenant_override_allowed: binding.tenant_override_allowed,
            })
            .collect(),
        precedence: response.precedence.clone(),
    }
}

fn ensure_control_write_access(auth: &AuthContext) -> Result<(), ControlPlaneError> {
    if auth.has_permission("control.write") {
        return Ok(());
    }

    Err(ControlPlaneError::forbidden("control.write"))
}

fn ensure_control_read_access(auth: &AuthContext) -> Result<(), ControlPlaneError> {
    if auth.has_permission("control.read") || auth.has_permission("control.write") {
        return Ok(());
    }

    Err(ControlPlaneError::forbidden("control.read"))
}

fn schema_response(schema: &SchemaDescriptor) -> ProtocolSchemaResponse {
    ProtocolSchemaResponse {
        schema: schema.schema.clone(),
        kind: schema.kind.clone(),
        stage: schema.stage.as_str().to_owned(),
        binding_protocols: schema.binding_protocols.iter().cloned().collect(),
        required_capabilities: schema.required_capabilities.iter().cloned().collect(),
        supported_consumers: schema.supported_consumers.iter().cloned().collect(),
    }
}

fn compatibility_response(
    descriptor: &ClientCompatibilityDescriptor,
) -> ClientCompatibilityResponse {
    ClientCompatibilityResponse {
        client_type: descriptor.client_type.clone(),
        minimum_protocol_version: descriptor.minimum_protocol_version.clone(),
        supported_bindings: descriptor.supported_bindings.iter().cloned().collect(),
        supported_codecs: descriptor.supported_codecs.iter().cloned().collect(),
        supported_capabilities: descriptor.supported_capabilities.iter().cloned().collect(),
        blocked_experimental_capabilities: descriptor
            .blocked_experimental_capabilities
            .iter()
            .cloned()
            .collect(),
    }
}

fn governance_response(
    governance: &ProtocolGovernanceSnapshot,
    registry: &CcpRegistry,
) -> ProtocolGovernanceResponse {
    ProtocolGovernanceResponse {
        capability_profile: capability_profile_response(&governance.capability_profile),
        quota_profile: quota_profile_response(&governance.quota_profile),
        rollout_policy: rollout_policy_response(&governance.rollout_policy),
        kill_switch: kill_switch_response(&governance.kill_switch),
        effective_snapshot: effective_snapshot_response(&governance.effective_snapshot),
        business_policy_vocabulary: business_policy_vocabulary_response(
            &governance.business_policy_vocabulary,
        ),
        sdk_compatibility_baseline: sdk_compatibility_baseline_response(registry),
    }
}

fn capability_profile_response(profile: &CapabilityProfile) -> CapabilityProfileResponse {
    CapabilityProfileResponse {
        profile_id: profile.profile_id.clone(),
        release_channel: release_channel(profile.release_channel.clone()).to_owned(),
        enabled_capabilities: profile.enabled_capabilities.iter().cloned().collect(),
        experimental_capabilities: profile.experimental_capabilities.iter().cloned().collect(),
    }
}

fn quota_profile_response(profile: &QuotaProfile) -> QuotaProfileResponse {
    QuotaProfileResponse {
        profile_id: profile.profile_id.clone(),
        max_concurrent_sessions_per_tenant: profile.max_concurrent_sessions_per_tenant,
        max_subscriptions_per_session: profile.max_subscriptions_per_session,
        max_inflight_messages: profile.max_inflight_messages,
        max_payload_bytes: profile.max_payload_bytes,
    }
}

fn rollout_policy_response(policy: &RolloutPolicy) -> RolloutPolicyResponse {
    RolloutPolicyResponse {
        policy_id: policy.policy_id.clone(),
        release_channel: release_channel(policy.release_channel.clone()).to_owned(),
        traffic_percent: policy.traffic_percent,
        cell_selector: policy.cell_selector.clone(),
        region_selector: policy.region_selector.clone(),
        operator_override: policy.operator_override,
        tenant_allowlist: policy.tenant_allowlist.iter().cloned().collect(),
    }
}

fn kill_switch_response(kill_switch: &KillSwitchRule) -> KillSwitchResponse {
    KillSwitchResponse {
        rule_id: kill_switch.rule_id.clone(),
        active: kill_switch.active,
        reason: kill_switch.reason.clone(),
        disabled_capabilities: kill_switch.disabled_capabilities.iter().cloned().collect(),
        disabled_bindings: kill_switch.disabled_bindings.iter().cloned().collect(),
        disabled_codecs: kill_switch.disabled_codecs.iter().cloned().collect(),
    }
}

fn effective_snapshot_response(
    snapshot: &EffectiveProtocolSnapshot,
) -> EffectiveProtocolSnapshotResponse {
    EffectiveProtocolSnapshotResponse {
        protocol_version: snapshot.protocol_version.clone(),
        release_channel: release_channel(snapshot.release_channel.clone()).to_owned(),
        enabled_capabilities: snapshot.enabled_capabilities.iter().cloned().collect(),
        allowed_bindings: snapshot.allowed_bindings.iter().cloned().collect(),
        allowed_codecs: snapshot.allowed_codecs.iter().cloned().collect(),
        quota_profile_id: snapshot.quota_profile_id.clone(),
        kill_switch_active: snapshot.kill_switch_active,
        precedence: snapshot.precedence.clone(),
    }
}

fn business_policy_vocabulary_response(
    vocabulary: &BusinessPolicyVocabulary,
) -> BusinessPolicyVocabularyResponse {
    BusinessPolicyVocabularyResponse {
        policy_version_field: vocabulary.policy_version_field.clone(),
        capability_flags_field: vocabulary.capability_flags_field.clone(),
        history_visibility_field: vocabulary.history_visibility_field.clone(),
        history_visibility_modes: vocabulary.history_visibility_modes.clone(),
        retention_policy_ref_field: vocabulary.retention_policy_ref_field.clone(),
        retention_policy_scopes: vocabulary.retention_policy_scopes.clone(),
    }
}

fn sdk_compatibility_baseline_response(registry: &CcpRegistry) -> SdkCompatibilityBaselineResponse {
    SdkCompatibilityBaselineResponse {
        app_sdk_facade: "sdkwork-craw-chat-sdk",
        admin_sdk_facade: "sdkwork-craw-chat-sdk-admin",
        matrix_client_types: registry.compatibility_matrix().keys().cloned().collect(),
        protocol_registry_path: "/api/v1/control/protocol-registry",
        protocol_governance_path: "/api/v1/control/protocol-governance",
    }
}

fn release_channel(channel: ReleaseChannel) -> &'static str {
    channel.as_str()
}

fn provider_domain_name(domain: ProviderDomain) -> &'static str {
    domain.as_str()
}

fn validate_required(field: &'static str, value: &str) -> Result<(), ControlPlaneError> {
    validate_required_with_code(field, value, "invalid_friend_request")
}

fn validate_required_with_code(
    field: &'static str,
    value: &str,
    code: &'static str,
) -> Result<(), ControlPlaneError> {
    if value.trim().is_empty() {
        return Err(ControlPlaneError::invalid(
            code,
            format!("{field} cannot be empty"),
        ));
    }

    Ok(())
}
