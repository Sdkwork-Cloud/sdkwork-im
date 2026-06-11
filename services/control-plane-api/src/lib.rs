use std::cmp::Ordering as CmpOrdering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path as StdPath, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use audit_service::{AuditRuntime, RecordAuditAnchor};
use axum::extract::{DefaultBodyLimit, Extension, Path, Query, State};
use axum::http::{HeaderMap, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use base64::Engine as _;
use bytes::Bytes;
use craw_chat_ccp_registry::{
    BusinessPolicyVocabulary, CapabilityProfile, CcpRegistry, ClientCompatibilityDescriptor,
    EffectiveProtocolSnapshot, KillSwitchRule, ProtocolGovernanceSnapshot, QuotaProfile,
    ReleaseChannel, RolloutPolicy, SchemaDescriptor,
};
use craw_chat_openapi::{OpenApiServiceSpec, render_docs_html};
use fs4::fs_std::FileExt;
use getrandom::fill as fill_random;
use hmac::{Hmac, Mac};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::header::CONTENT_TYPE;
use hyper::{Method, Request as HyperRequest};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use im_adapters_local_disk::{FileCommitJournal, read_commit_journal_file};
use im_adapters_local_memory::MemoryCommitJournal;
use im_app_context::{
    AppContext, AppContextError, build_dual_token_headers_for_context, resolve_app_context,
    resolve_app_context_for_request,
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
    FriendRequestAcceptedPayload, FriendRequestCanceledPayload, FriendRequestDeclinedPayload,
    FriendRequestSubmittedPayload, FriendshipActivatedPayload, FriendshipRemovedPayload,
    SharedChannelPolicyAppliedPayload, SocialCommitEnvelopeInput, SocialEventType,
    UserBlockedPayload, social_commit_envelope,
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
use serde_json::{Map as JsonMap, Value as JsonValue};
use session_gateway::{
    RealtimeClusterBridge, RealtimeClusterError, RealtimeNodeLifecycleView,
    RealtimeRouteMigrationResult,
};
use sha2::{Digest, Sha256};
use tokio::sync::Semaphore;

const CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "CRAW_CHAT_CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS";
const CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 50_000;
const CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "CRAW_CHAT_CONTROL_PLANE_MAX_REQUEST_BODY_BYTES";
const CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const CONTROL_PLANE_REQUIRE_DUAL_TOKEN_HEADERS_ENV: &str =
    "CRAW_CHAT_CONTROL_PLANE_REQUIRE_DUAL_TOKEN_HEADERS";

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
    require_dual_token_headers: bool,
}

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedChannelSyncDeliveryProofStatus {
    TransportAccepted,
    Applied,
    AlreadyLinked,
    Replayed,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedChannelSyncDeliveryProof {
    pub request_key: String,
    pub status: SharedChannelSyncDeliveryProofStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl SharedChannelSyncDeliveryProof {
    fn transport_accepted(request_key: String) -> Self {
        Self {
            request_key,
            status: SharedChannelSyncDeliveryProofStatus::TransportAccepted,
            proof_version: None,
            target: None,
        }
    }
}

pub trait SharedChannelLinkedMemberSyncTrigger: Send + Sync {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String>;

    fn trigger_with_delivery_proof(
        &self,
        request: SharedChannelLinkedMemberSyncRequest,
    ) -> Result<SharedChannelSyncDeliveryProof, String> {
        let request_key = shared_channel_sync_request_key(&request);
        self.trigger(request)?;
        Ok(SharedChannelSyncDeliveryProof::transport_accepted(
            request_key,
        ))
    }
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
pub struct SocialControlQuery {
    social_runtime: Arc<SocialControlRuntime>,
}

impl SocialControlQuery {
    pub fn direct_chat_snapshot(
        &self,
        tenant_id: &str,
        direct_chat_id: &str,
    ) -> Option<DirectChat> {
        self.social_runtime
            .direct_chat_snapshot(tenant_id, direct_chat_id)
            .map(|record| record.direct_chat)
    }

    pub fn active_direct_chat_access_block(
        &self,
        tenant_id: &str,
        direct_chat_id: &str,
    ) -> Option<UserBlock> {
        self.social_runtime
            .active_direct_chat_access_block(tenant_id, direct_chat_id)
    }

    pub fn active_friendship_access_block_for_pair(
        &self,
        tenant_id: &str,
        user_a: &str,
        user_b: &str,
    ) -> Option<UserBlock> {
        self.social_runtime
            .active_friendship_access_block_for_pair(tenant_id, user_a, user_b)
    }

    pub fn authoritative_active_friendships_for_user(
        &self,
        tenant_id: &str,
        user_id: &str,
    ) -> Result<Vec<Friendship>, String> {
        self.social_runtime
            .authoritative_active_friendships_for_user(tenant_id, user_id)
    }

    pub fn authoritative_active_direct_chat_for_pair(
        &self,
        tenant_id: &str,
        user_low_id: &str,
        user_high_id: &str,
    ) -> Result<Option<DirectChat>, String> {
        self.social_runtime
            .authoritative_active_direct_chat_for_pair(tenant_id, user_low_id, user_high_id)
    }
}

#[derive(Clone)]
struct GovernanceLoop {
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
}

const SOCIAL_STATE_FILE_NAME: &str = "social-state.json";
const SOCIAL_COMMIT_JOURNAL_FILE_NAME: &str = "social-commit-journal.json";
const SOCIAL_TRANSACTION_MARKER_FILE_NAME: &str = "social-transaction-marker.json";
const SOCIAL_WRITE_LOCK_FILE_NAME: &str = "social-write.lock";
const SOCIAL_COMMIT_PARTITION: &str = "control-plane-social";
const PUBLIC_SHARED_CHANNEL_SYNC_ROUTE: &str =
    "/im/v3/api/chat/conversations/shared_channel_links/sync";
const PUBLIC_SHARED_CHANNEL_SYNC_ACTOR_ID: &str = "control-plane-sync";
pub const SHARED_CHANNEL_SYNC_PERMISSION: &str = "conversation.shared_channel.sync";
const SHARED_CHANNEL_SYNC_DEAD_LETTER_FAILURE_THRESHOLD: u32 = 3;
const SHARED_CHANNEL_SYNC_PENDING_LEASE_WINDOW_MILLIS: u128 = 900_000;
const SHARED_CHANNEL_SYNC_DISPATCH_DELIVERY_DEDUP_WINDOW_MILLIS: u128 = 300_000;
pub const ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP_ENV: &str =
    "CRAW_CHAT_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP";
pub const SHARED_CHANNEL_SYNC_RUNTIME_PROFILE_ENV: &str = "CRAW_CHAT_RUNTIME_PROFILE";
pub const SHARED_CHANNEL_SYNC_TARGET_BASE_URL_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_TARGET_BASE_URL";
pub const SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS";
const SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_DEFAULT_MILLIS: u64 = 5_000;
const SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MAX_MILLIS: u64 = 60_000;
const SHARED_CHANNEL_SYNC_RESPONSE_BODY_MAX_BYTES: usize = 16 * 1024;
pub const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED";
pub const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS";
pub const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS";
const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED_DEFAULT: bool = true;
const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_INTERVAL_MILLIS: u64 = 30_000;
const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MIN_INTERVAL_MILLIS: u64 = 1_000;
const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MAX_INTERVAL_MILLIS: u64 = 600_000;
const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_JITTER_MILLIS: u64 = 250;
const SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MAX_JITTER_MILLIS: u64 = 5_000;
pub const SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT";
pub const SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY";
pub const SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS";
pub const SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES";
pub const SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MILLIS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MILLIS";
const SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT_DEFAULT: usize = 4;
const SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT_MAX: usize = 128;
const SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY_DEFAULT: usize = 1024;
const SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY_MAX: usize = 65_536;
const SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_DEFAULT_MILLIS: u128 = 2_592_000_000;
const SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MAX_MILLIS: u128 = 31_536_000_000;
const SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_DEFAULT: usize = 200_000;
const SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_MAX: usize = 2_000_000;
const SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_DEFAULT_MILLIS: u128 = 0;
const SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MAX_MILLIS: u128 = 60_000;
const SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION: &str = "shared_channel_sync_ack.v1";
const CONTROL_PLANE_MAX_ID_BYTES: usize = 256;
const CONTROL_PLANE_MAX_ACTOR_KIND_BYTES: usize = 64;
const CONTROL_PLANE_MAX_TIMESTAMP_BYTES: usize = 64;
const CONTROL_PLANE_MAX_REQUEST_MESSAGE_BYTES: usize = 8 * 1024;
const CONTROL_PLANE_MAX_HISTORY_VISIBILITY_BYTES: usize = 32;
const CONTROL_PLANE_MAX_EXTERNAL_ORG_NAME_BYTES: usize = 256;
const CONTROL_PLANE_MAX_EXTERNAL_DISPLAY_NAME_BYTES: usize = 512;
const CONTROL_PLANE_MAX_REQUEST_KEY_BYTES: usize = 512;
const CONTROL_PLANE_MAX_REQUEST_KEYS: usize = 1024;
const CONTROL_PLANE_MAX_REQUEST_KEYS_TOTAL_BYTES: usize = 64 * 1024;
const SOCIAL_FRIEND_REQUEST_LIST_DEFAULT_LIMIT: usize = 100;
const SOCIAL_FRIEND_REQUEST_LIST_MAX_LIMIT: usize = 200;
const SOCIAL_FRIEND_REQUEST_LIST_MAX_CURSOR_BYTES: usize = 1024;
const SOCIAL_FRIEND_REQUEST_CURSOR_VERSION: u64 = 1;
const SHARED_CHANNEL_SYNC_INVENTORY_DEFAULT_LIMIT: usize = 100;
const SHARED_CHANNEL_SYNC_INVENTORY_MAX_LIMIT: usize = 200;
const SHARED_CHANNEL_SYNC_INVENTORY_MAX_CURSOR_BYTES: usize = 1024;
const SHARED_CHANNEL_SYNC_INVENTORY_CURSOR_VERSION: u64 = 1;
const FRIEND_REQUEST_CURSOR_HS256_SECRET_ENV: &str = "CRAW_CHAT_FRIEND_REQUEST_CURSOR_HS256_SECRET";
static CONTROL_PLANE_AUDIT_RECORD_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SharedChannelSyncStaleReclaimSchedulerConfig {
    pub enabled: bool,
    pub interval_millis: u64,
    pub jitter_millis: u64,
}

impl SharedChannelSyncStaleReclaimSchedulerConfig {
    fn with_normalized_values(self) -> Self {
        let interval_millis = if self.interval_millis == 0 {
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_INTERVAL_MILLIS
        } else {
            self.interval_millis
        }
        .clamp(
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MIN_INTERVAL_MILLIS,
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MAX_INTERVAL_MILLIS,
        );
        Self {
            enabled: self.enabled,
            interval_millis,
            jitter_millis: self
                .jitter_millis
                .min(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MAX_JITTER_MILLIS),
        }
    }

    fn tick_sleep_duration(self) -> Duration {
        self.tick_sleep_duration_at(SystemTime::now())
    }

    fn tick_sleep_duration_at(self, now: SystemTime) -> Duration {
        let normalized = self.with_normalized_values();
        let jitter_offset_millis = if normalized.jitter_millis == 0 {
            0
        } else {
            let now_millis = now
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            (now_millis % ((normalized.jitter_millis as u128) + 1)) as u64
        };
        Duration::from_millis(
            normalized
                .interval_millis
                .saturating_add(jitter_offset_millis),
        )
    }
}

struct SharedChannelSyncDispatchTask {
    request: SharedChannelLinkedMemberSyncRequest,
    response_tx: std::sync::mpsc::Sender<Result<SharedChannelSyncDeliveryProof, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SharedChannelSyncAckStatus {
    Applied,
    AlreadyLinked,
    Replayed,
}

impl SharedChannelSyncAckStatus {
    fn into_delivery_status(self) -> SharedChannelSyncDeliveryProofStatus {
        match self {
            Self::Applied => SharedChannelSyncDeliveryProofStatus::Applied,
            Self::AlreadyLinked => SharedChannelSyncDeliveryProofStatus::AlreadyLinked,
            Self::Replayed => SharedChannelSyncDeliveryProofStatus::Replayed,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SharedChannelSyncAckResponse {
    request_key: String,
    status: SharedChannelSyncAckStatus,
    proof_version: Option<String>,
    principal_id: String,
    principal_kind: String,
    role: String,
    state: String,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
}

struct DualTokenSharedChannelLinkedMemberSyncTrigger {
    dispatch_tx: std::sync::mpsc::SyncSender<SharedChannelSyncDispatchTask>,
    dispatch_queue_capacity: usize,
}

impl DualTokenSharedChannelLinkedMemberSyncTrigger {
    fn new(base_url: impl AsRef<str>) -> Result<Self, String> {
        let base_url = validate_shared_channel_sync_target_base_url(base_url.as_ref())?;

        let dispatch_queue_capacity = resolve_shared_channel_sync_dispatch_queue_capacity();
        let (dispatch_tx, dispatch_rx) =
            std::sync::mpsc::sync_channel::<SharedChannelSyncDispatchTask>(dispatch_queue_capacity);
        let dispatch_rx = Arc::new(Mutex::new(dispatch_rx));
        let worker_count = resolve_shared_channel_sync_dispatch_worker_count();

        for worker_index in 0..worker_count {
            let dispatch_rx = Arc::clone(&dispatch_rx);
            let base_url = base_url.clone();
            std::thread::Builder::new()
                .name(format!("shared-sync-dispatch-worker-{worker_index}"))
                .spawn(move || {
                    let runtime = match tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                    {
                        Ok(runtime) => Some(runtime),
                        Err(error) => {
                            tracing::warn!(
                                "failed to build shared-channel sync worker runtime: {error}"
                            );
                            None
                        }
                    };

                    loop {
                        let recv = dispatch_rx
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner())
                            .recv();
                        let Ok(task) = recv else {
                            break;
                        };
                        let result = if let Some(runtime) = runtime.as_ref() {
                            runtime
                                .block_on(Self::dispatch_request(base_url.as_str(), task.request))
                        } else {
                            Err("shared-channel sync worker runtime unavailable".to_owned())
                        };
                        let _ = task.response_tx.send(result);
                    }
                })
                .map_err(|error| format!("failed to spawn shared-channel sync worker: {error}"))?;
        }

        Ok(Self {
            dispatch_tx,
            dispatch_queue_capacity,
        })
    }

    async fn dispatch_request(
        base_url: &str,
        request: SharedChannelLinkedMemberSyncRequest,
    ) -> Result<SharedChannelSyncDeliveryProof, String> {
        let timeout = resolve_shared_channel_sync_http_timeout();
        let ack_request = request.clone();
        let request_key = shared_channel_sync_request_key(&request);
        let payload = serde_json::to_vec(&serde_json::json!({
            "conversationId": request.conversation_id,
            "sharedChannelPolicyId": request.shared_channel_policy_id,
            "externalConnectionId": request.external_connection_id,
            "localActorId": request.local_actor_id,
            "localActorKind": request.local_actor_kind,
            "externalMemberId": request.external_member_id,
            "requestKey": request_key.clone(),
        }))
        .map(Bytes::from)
        .map_err(|error| format!("failed to encode shared-channel sync payload: {error}"))?;
        let target = format!("{}{}", base_url, PUBLIC_SHARED_CHANNEL_SYNC_ROUTE);
        let auth_context = AppContext {
            tenant_id: request.tenant_id.clone(),
            organization_id: None,
            user_id: PUBLIC_SHARED_CHANNEL_SYNC_ACTOR_ID.to_owned(),
            session_id: Some("shared-channel-sync".to_owned()),
            app_id: Some("craw-chat".to_owned()),
            environment: Some("local".to_owned()),
            deployment_mode: Some("local".to_owned()),
            auth_level: Some("system".to_owned()),
            data_scope: BTreeSet::from(["tenant".to_owned()]),
            permission_scope: BTreeSet::from([SHARED_CHANNEL_SYNC_PERMISSION.to_owned()]),
            actor_id: PUBLIC_SHARED_CHANNEL_SYNC_ACTOR_ID.to_owned(),
            actor_kind: "system".to_owned(),
            device_id: None,
        };
        let auth_headers = build_dual_token_headers_for_context(
            &auth_context,
            [SHARED_CHANNEL_SYNC_PERMISSION],
        );
        let mut builder = HyperRequest::builder()
            .method(Method::POST)
            .uri(target.as_str())
            .header(CONTENT_TYPE, "application/json");
        for (name, value) in auth_headers.iter() {
            builder = builder.header(name, value);
        }
        let request = builder
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
            return validate_shared_channel_sync_ack_response(
                body.as_ref(),
                target.as_str(),
                &ack_request,
                request_key.as_str(),
            );
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
            if is_local_shared_channel_sync_host(host) {
                return Ok(base_url);
            }
            if allow_insecure_shared_channel_sync_http() {
                let Some(runtime_profile) = resolve_shared_channel_sync_runtime_profile() else {
                    return Err(format!(
                        "shared-channel sync remote http override requires explicit local runtime profile via {} (for example: local-minimal or local-default)",
                        SHARED_CHANNEL_SYNC_RUNTIME_PROFILE_ENV
                    ));
                };
                if !is_local_shared_channel_sync_runtime_profile(runtime_profile.as_str()) {
                    return Err(format!(
                        "shared-channel sync remote http override is only allowed for local runtime profiles; got {}={} (allowed: local-minimal, local-default, local, dev, development, test, ci)",
                        SHARED_CHANNEL_SYNC_RUNTIME_PROFILE_ENV, runtime_profile
                    ));
                }
                tracing::warn!(
                    "allowing insecure shared-channel sync target over remote http because {} is enabled for local runtime profile {}={}",
                    ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP_ENV,
                    SHARED_CHANNEL_SYNC_RUNTIME_PROFILE_ENV,
                    runtime_profile
                );
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

fn resolve_shared_channel_sync_runtime_profile() -> Option<String> {
    std::env::var(SHARED_CHANNEL_SYNC_RUNTIME_PROFILE_ENV)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
}

fn is_local_shared_channel_sync_runtime_profile(profile: &str) -> bool {
    matches!(
        profile,
        "local-minimal" | "local-default" | "local" | "dev" | "development" | "test" | "ci"
    )
}

fn resolve_shared_channel_sync_http_timeout() -> Duration {
    let timeout_millis = std::env::var(SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_DEFAULT_MILLIS)
        .min(SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MAX_MILLIS);
    Duration::from_millis(timeout_millis)
}

fn resolve_shared_channel_sync_dispatch_worker_count() -> usize {
    std::env::var(SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT_DEFAULT)
        .min(SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT_MAX)
}

fn resolve_shared_channel_sync_dispatch_queue_capacity() -> usize {
    std::env::var(SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY_DEFAULT)
        .min(SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY_MAX)
}

fn resolve_shared_channel_sync_delivered_ledger_retention_millis() -> u128 {
    std::env::var(SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u128>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_DEFAULT_MILLIS)
        .min(SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MAX_MILLIS)
}

fn resolve_shared_channel_sync_delivered_ledger_max_entries() -> usize {
    std::env::var(SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_DEFAULT)
        .min(SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_MAX)
}

fn resolve_shared_channel_sync_pending_retry_cooldown_millis() -> u128 {
    std::env::var(SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MILLIS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u128>().ok())
        .unwrap_or(SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_DEFAULT_MILLIS)
        .min(SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MAX_MILLIS)
}

fn resolve_shared_channel_sync_stale_reclaim_scheduler_config_from_env()
-> SharedChannelSyncStaleReclaimSchedulerConfig {
    let enabled = resolve_env_bool_with_default(
        SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED_ENV,
        SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED_DEFAULT,
    );
    let interval_millis =
        std::env::var(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS_ENV)
            .ok()
            .and_then(|value| value.trim().parse::<u64>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_INTERVAL_MILLIS)
            .clamp(
                SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MIN_INTERVAL_MILLIS,
                SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MAX_INTERVAL_MILLIS,
            );
    let jitter_millis =
        std::env::var(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS_ENV)
            .ok()
            .and_then(|value| value.trim().parse::<u64>().ok())
            .unwrap_or(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_JITTER_MILLIS)
            .min(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MAX_JITTER_MILLIS);
    SharedChannelSyncStaleReclaimSchedulerConfig {
        enabled,
        interval_millis,
        jitter_millis,
    }
}

fn resolve_env_bool_with_default(name: &str, default: bool) -> bool {
    let Ok(value) = std::env::var(name) else {
        return default;
    };
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => true,
        "0" | "false" | "no" | "off" => false,
        invalid => {
            tracing::warn!("invalid boolean env {name}={invalid}, falling back to default");
            default
        }
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

fn validate_shared_channel_sync_ack_response(
    body: &[u8],
    target: &str,
    request: &SharedChannelLinkedMemberSyncRequest,
    expected_request_key: &str,
) -> Result<SharedChannelSyncDeliveryProof, String> {
    if body.is_empty() {
        return Err(format!(
            "shared-channel sync endpoint {target} returned success without ack payload"
        ));
    }
    let ack: SharedChannelSyncAckResponse = serde_json::from_slice(body).map_err(|error| {
        format!("shared-channel sync endpoint {target} returned invalid ack json: {error}")
    })?;
    if ack.request_key != expected_request_key {
        return Err(format!(
            "shared-channel sync endpoint {target} ack requestKey mismatch: expected {expected_request_key}, got {}",
            ack.request_key
        ));
    }
    if ack.proof_version.as_deref() != Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION) {
        return Err(format!(
            "shared-channel sync endpoint {target} ack proofVersion mismatch: expected {SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION}, got {:?}",
            ack.proof_version
        ));
    }
    let status = ack.status.into_delivery_status();
    if ack.principal_id != request.local_actor_id {
        return Err(format!(
            "shared-channel sync endpoint {target} ack principalId mismatch: expected {}, got {}",
            request.local_actor_id, ack.principal_id
        ));
    }
    if ack.principal_kind != request.local_actor_kind {
        return Err(format!(
            "shared-channel sync endpoint {target} ack principalKind mismatch: expected {}, got {}",
            request.local_actor_kind, ack.principal_kind
        ));
    }
    if ack.role.as_str() != "guest" {
        return Err(format!(
            "shared-channel sync endpoint {target} ack role mismatch: expected guest, got {}",
            ack.role
        ));
    }
    if ack.state.as_str() != "linked" {
        return Err(format!(
            "shared-channel sync endpoint {target} ack state mismatch: expected linked, got {}",
            ack.state
        ));
    }
    let expected_attributes = [
        (
            "sharedChannelPolicyId",
            request.shared_channel_policy_id.as_str(),
        ),
        (
            "externalConnectionId",
            request.external_connection_id.as_str(),
        ),
        ("externalMemberId", request.external_member_id.as_str()),
    ];
    for (key, expected_value) in expected_attributes {
        let Some(actual_value) = ack.attributes.get(key).map(String::as_str) else {
            return Err(format!(
                "shared-channel sync endpoint {target} ack attributes missing key {key}"
            ));
        };
        if actual_value != expected_value {
            return Err(format!(
                "shared-channel sync endpoint {target} ack attributes[{key}] mismatch: expected {expected_value}, got {actual_value}"
            ));
        }
    }
    let Some(actual_request_key) = ack
        .attributes
        .get("sharedChannelSyncRequestKey")
        .map(String::as_str)
    else {
        return Err(format!(
            "shared-channel sync endpoint {target} ack attributes missing key sharedChannelSyncRequestKey"
        ));
    };
    if actual_request_key != expected_request_key {
        return Err(format!(
            "shared-channel sync endpoint {target} ack attributes[sharedChannelSyncRequestKey] mismatch: expected {expected_request_key}, got {actual_request_key}"
        ));
    }
    Ok(SharedChannelSyncDeliveryProof {
        request_key: ack.request_key,
        status,
        proof_version: ack.proof_version,
        target: Some(target.to_owned()),
    })
}

impl SharedChannelLinkedMemberSyncTrigger for DualTokenSharedChannelLinkedMemberSyncTrigger {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String> {
        self.trigger_with_delivery_proof(request).map(|_| ())
    }

    fn trigger_with_delivery_proof(
        &self,
        request: SharedChannelLinkedMemberSyncRequest,
    ) -> Result<SharedChannelSyncDeliveryProof, String> {
        let (response_tx, response_rx) =
            std::sync::mpsc::channel::<Result<SharedChannelSyncDeliveryProof, String>>();
        match self.dispatch_tx.try_send(SharedChannelSyncDispatchTask {
            request,
            response_tx,
        }) {
            Ok(()) => {}
            Err(std::sync::mpsc::TrySendError::Full(_)) => {
                return Err(format!(
                    "shared-channel sync dispatch queue is full (capacity: {}), retry later",
                    self.dispatch_queue_capacity
                ));
            }
            Err(std::sync::mpsc::TrySendError::Disconnected(_)) => {
                return Err("shared-channel sync worker is unavailable".to_owned());
            }
        }
        response_rx
            .recv()
            .map_err(|_| "shared-channel sync worker dropped dispatch response".to_owned())?
    }
}

fn social_query_handle(social_runtime: Arc<SocialControlRuntime>) -> Arc<SocialControlQuery> {
    Arc::new(SocialControlQuery { social_runtime })
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
    delivered_shared_channel_sync_requests: BTreeMap<String, String>,
    delivered_shared_channel_sync_delivery_proofs:
        BTreeMap<String, StoredSharedChannelSyncDeliveryProof>,
    recent_shared_channel_sync_deliveries: BTreeMap<String, String>,
    #[serde(skip)]
    pending_friend_request_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    accepted_friend_request_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    friend_request_user_index: BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    active_friendship_pair_index: BTreeMap<SocialPairIndexKey, String>,
    #[serde(skip)]
    active_friendship_user_index: BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    friendship_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    active_direct_chat_pair_index: BTreeMap<SocialPairIndexKey, String>,
    #[serde(skip)]
    direct_chat_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    active_user_block_scope_index: BTreeMap<SocialUserBlockScopeIndexKey, String>,
    #[serde(skip)]
    active_friendship_block_pair_index: BTreeMap<SocialPairIndexKey, String>,
    #[serde(skip)]
    active_direct_chat_block_pair_index: BTreeMap<SocialPairIndexKey, String>,
    #[serde(skip)]
    active_direct_chat_block_chat_index: BTreeMap<SocialDirectChatBlockIndexKey, String>,
    #[serde(skip)]
    committed_event_index: BTreeMap<SocialCommittedEventIndexKey, SocialCommittedEventPointer>,
    #[serde(skip)]
    active_external_connection_target_index:
        BTreeMap<SocialExternalConnectionTargetIndexKey, String>,
    #[serde(skip)]
    active_external_member_mapping_index: BTreeMap<SocialExternalMemberMappingIndexKey, String>,
    #[serde(skip)]
    active_external_member_connection_index: BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    active_shared_channel_policy_target_index:
        BTreeMap<SocialSharedChannelPolicyTargetIndexKey, String>,
    #[serde(skip)]
    active_shared_channel_policy_connection_index:
        BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pending_shared_channel_retry_index: BTreeMap<SharedChannelRetryIndexKey, BTreeSet<String>>,
    #[serde(skip)]
    pending_shared_channel_lease_index: BTreeMap<SharedChannelLeaseIndexKey, BTreeSet<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SocialPairIndexKey {
    tenant_id: String,
    left_id: String,
    right_id: String,
}

impl SocialPairIndexKey {
    fn new(tenant_id: &str, left_id: &str, right_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            left_id: left_id.to_owned(),
            right_id: right_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SocialUserIndexKey {
    tenant_id: String,
    user_id: String,
}

impl SocialUserIndexKey {
    fn new(tenant_id: &str, user_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            user_id: user_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SocialUserBlockScopeIndexKey {
    tenant_id: String,
    blocker_user_id: String,
    blocked_user_id: String,
    scope: String,
    direct_chat_id: Option<String>,
}

impl SocialUserBlockScopeIndexKey {
    fn new(user_block: &UserBlock) -> Self {
        let direct_chat_id = if matches!(user_block.scope, BlockScope::DirectChat) {
            user_block.direct_chat_id.clone()
        } else {
            None
        };
        Self {
            tenant_id: user_block.tenant_id.clone(),
            blocker_user_id: user_block.blocker_user_id.clone(),
            blocked_user_id: user_block.blocked_user_id.clone(),
            scope: block_scope_index_label(&user_block.scope).to_owned(),
            direct_chat_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SocialDirectChatBlockIndexKey {
    tenant_id: String,
    direct_chat_id: String,
}

impl SocialDirectChatBlockIndexKey {
    fn new(tenant_id: &str, direct_chat_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            direct_chat_id: direct_chat_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SocialExternalConnectionTargetIndexKey {
    tenant_id: String,
    external_tenant_id: String,
    connection_kind: String,
}

impl SocialExternalConnectionTargetIndexKey {
    fn new(
        tenant_id: &str,
        external_tenant_id: &str,
        connection_kind: &ExternalConnectionKind,
    ) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            external_tenant_id: external_tenant_id.to_owned(),
            connection_kind: external_connection_kind_index_label(connection_kind).to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SocialExternalMemberMappingIndexKey {
    tenant_id: String,
    connection_id: String,
    external_member_id: String,
}

impl SocialExternalMemberMappingIndexKey {
    fn new(tenant_id: &str, connection_id: &str, external_member_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            connection_id: connection_id.to_owned(),
            external_member_id: external_member_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SocialConnectionIndexKey {
    tenant_id: String,
    connection_id: String,
}

impl SocialConnectionIndexKey {
    fn new(tenant_id: &str, connection_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            connection_id: connection_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SharedChannelRetryIndexKey {
    last_failed_at: String,
}

impl SharedChannelRetryIndexKey {
    fn new(last_failed_at: &str) -> Self {
        Self {
            last_failed_at: last_failed_at.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SharedChannelLeaseIndexKey {
    lease_expires_at: String,
}

impl SharedChannelLeaseIndexKey {
    fn new(lease_expires_at: &str) -> Self {
        Self {
            lease_expires_at: lease_expires_at.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SocialSharedChannelPolicyTargetIndexKey {
    tenant_id: String,
    connection_id: String,
    channel_id: String,
}

impl SocialSharedChannelPolicyTargetIndexKey {
    fn new(tenant_id: &str, connection_id: &str, channel_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            connection_id: connection_id.to_owned(),
            channel_id: channel_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SocialCommittedEventIndexKey {
    tenant_id: String,
    event_id: String,
}

impl SocialCommittedEventIndexKey {
    fn new(tenant_id: &str, event_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_owned(),
            event_id: event_id.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
enum SocialCommittedEventPointer {
    FriendRequest {
        request_id: String,
        commit_index: usize,
    },
    Friendship {
        friendship_id: String,
        commit_index: usize,
    },
    UserBlock {
        block_id: String,
        commit_index: usize,
    },
    DirectChat {
        direct_chat_id: String,
        commit_index: usize,
    },
    ExternalConnection {
        connection_id: String,
        commit_index: usize,
    },
    ExternalMemberLink {
        link_id: String,
        commit_index: usize,
    },
    SharedChannelPolicy {
        policy_id: String,
        commit_index: usize,
    },
}

impl SocialCommittedEventPointer {
    fn with_commit_index(&self, commit_index: usize) -> Self {
        match self {
            Self::FriendRequest { request_id, .. } => Self::FriendRequest {
                request_id: request_id.clone(),
                commit_index,
            },
            Self::Friendship { friendship_id, .. } => Self::Friendship {
                friendship_id: friendship_id.clone(),
                commit_index,
            },
            Self::UserBlock { block_id, .. } => Self::UserBlock {
                block_id: block_id.clone(),
                commit_index,
            },
            Self::DirectChat { direct_chat_id, .. } => Self::DirectChat {
                direct_chat_id: direct_chat_id.clone(),
                commit_index,
            },
            Self::ExternalConnection { connection_id, .. } => Self::ExternalConnection {
                connection_id: connection_id.clone(),
                commit_index,
            },
            Self::ExternalMemberLink { link_id, .. } => Self::ExternalMemberLink {
                link_id: link_id.clone(),
                commit_index,
            },
            Self::SharedChannelPolicy { policy_id, .. } => Self::SharedChannelPolicy {
                policy_id: policy_id.clone(),
                commit_index,
            },
        }
    }
}

struct SocialControlRuntime {
    state_store: SocialStateStore,
    commit_journal: Arc<dyn CommitJournal + Send + Sync>,
    state: RwLock<SocialControlState>,
    authority_replay_error: RwLock<Option<String>>,
    journal_path: Option<Arc<PathBuf>>,
    tx_marker_path: Option<Arc<PathBuf>>,
    write_lock_path: Option<Arc<PathBuf>>,
    snapshot_failpoint_path: Option<Arc<PathBuf>>,
    shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool,
}

struct SocialWriteLockGuard {
    file: fs::File,
}

struct SocialAuthorityLoad {
    state: SocialControlState,
    replay_error: Option<String>,
}

impl Drop for SocialWriteLockGuard {
    fn drop(&mut self) {
        if let Err(error) = self.file.unlock() {
            tracing::warn!("failed to unlock control-plane social write lock: {error}");
        }
    }
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

#[derive(Clone, Debug)]
struct AcceptedFriendRequest {
    friend_request: FriendRequest,
    latest_commit: CommitEnvelope,
    persistence: SocialWritePersistence,
    friendship: Option<Friendship>,
    friendship_materialized_commit: Option<CommitEnvelope>,
    direct_chat: Option<DirectChat>,
    direct_chat_materialized_commit: Option<CommitEnvelope>,
}

#[derive(Clone, Debug)]
struct DeclinedFriendRequest {
    friend_request: FriendRequest,
    latest_commit: CommitEnvelope,
    persistence: SocialWritePersistence,
}

#[derive(Clone, Debug)]
struct CanceledFriendRequest {
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

#[derive(Clone, Debug)]
struct RemovedFriendship {
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredSharedChannelSyncDeliveryProof {
    delivered_at: String,
    status: SharedChannelSyncDeliveryProofStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    proof_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PendingSharedChannelSyncRequest {
    request: SharedChannelLinkedMemberSyncRequest,
    failure_count: u32,
    last_error: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_failed_at: Option<String>,
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
                .is_some_and(|lease_expires_at| {
                    timestamp_at_or_before(lease_expires_at, claimed_at.as_str())
                });
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
                if timestamp_after(lease_expires_at, now) {
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
                .is_some_and(|lease_expires_at| timestamp_at_or_before(lease_expires_at, now))
    }

    fn blocks_foreign_takeover(&self, actor_id: &str, actor_kind: &str, now: &str) -> bool {
        self.is_claimed_by_other(actor_id, actor_kind)
            && self
                .lease_expires_at
                .as_deref()
                .is_some_and(|lease_expires_at| timestamp_after(lease_expires_at, now))
    }

    fn auto_dispatch_eligible(&self, now: &str, retry_window_start: &str) -> bool {
        if self
            .last_failed_at
            .as_deref()
            .is_some_and(|last_failed_at| timestamp_after(last_failed_at, retry_window_start))
        {
            return false;
        }
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

#[cfg(test)]
fn unix_epoch_seconds(at: SystemTime) -> u64 {
    at.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}

fn is_canonical_rfc3339_millis_utc(timestamp: &str) -> bool {
    let bytes = timestamp.as_bytes();
    if bytes.len() != 24 {
        return false;
    }
    for index in [4, 7] {
        if bytes[index] != b'-' {
            return false;
        }
    }
    if bytes[10] != b'T' || bytes[13] != b':' || bytes[16] != b':' || bytes[19] != b'.' {
        return false;
    }
    if bytes[23] != b'Z' {
        return false;
    }
    for index in [0, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18, 20, 21, 22] {
        if !bytes[index].is_ascii_digit() {
            return false;
        }
    }
    true
}

fn compare_canonical_rfc3339_millis_utc(left: &str, right: &str) -> Option<CmpOrdering> {
    if !is_canonical_rfc3339_millis_utc(left) || !is_canonical_rfc3339_millis_utc(right) {
        return None;
    }
    Some(left.cmp(right))
}

fn timestamp_recency_cmp(left: &str, right: &str) -> CmpOrdering {
    match (
        is_canonical_rfc3339_millis_utc(left),
        is_canonical_rfc3339_millis_utc(right),
    ) {
        (true, true) => left.cmp(right),
        (true, false) => CmpOrdering::Greater,
        (false, true) => CmpOrdering::Less,
        (false, false) => left.cmp(right),
    }
}

fn timestamp_newer_for_recency(candidate: &str, existing: &str) -> bool {
    matches!(
        timestamp_recency_cmp(candidate, existing),
        CmpOrdering::Greater
    )
}

fn timestamp_not_before_for_dedup(value: &str, window_start: &str) -> bool {
    compare_canonical_rfc3339_millis_utc(value, window_start)
        .is_some_and(|ordering| !matches!(ordering, CmpOrdering::Less))
}

fn timestamp_before_or_noncanonical_for_retention(value: &str, window_start: &str) -> bool {
    match compare_canonical_rfc3339_millis_utc(value, window_start) {
        Some(ordering) => matches!(ordering, CmpOrdering::Less),
        None => true,
    }
}

fn timestamp_after(left: &str, right: &str) -> bool {
    compare_canonical_rfc3339_millis_utc(left, right)
        .is_some_and(|ordering| matches!(ordering, CmpOrdering::Greater))
}

fn timestamp_at_or_before(left: &str, right: &str) -> bool {
    match compare_canonical_rfc3339_millis_utc(left, right) {
        Some(ordering) => !matches!(ordering, CmpOrdering::Greater),
        None => true,
    }
}

fn shared_channel_sync_delivered_ledger_retention_window_start(now_epoch_millis: u128) -> String {
    let retention_millis = resolve_shared_channel_sync_delivered_ledger_retention_millis();
    format_unix_timestamp_millis(now_epoch_millis.saturating_sub(retention_millis))
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
    fn rebuild_social_friend_request_indexes(&mut self) {
        self.pending_friend_request_pair_index.clear();
        self.accepted_friend_request_pair_index.clear();
        self.friend_request_user_index.clear();

        for record in self.friend_requests.values() {
            index_friend_request_record(
                &mut self.pending_friend_request_pair_index,
                &mut self.accepted_friend_request_pair_index,
                &mut self.friend_request_user_index,
                record,
            );
        }
    }

    fn rebuild_social_pair_indexes(&mut self) {
        self.active_friendship_pair_index.clear();
        self.active_friendship_user_index.clear();
        self.friendship_pair_index.clear();
        self.active_direct_chat_pair_index.clear();
        self.direct_chat_pair_index.clear();

        for record in self.friendships.values() {
            index_friendship_record(
                &mut self.active_friendship_pair_index,
                &mut self.active_friendship_user_index,
                &mut self.friendship_pair_index,
                record,
            );
        }
        for record in self.direct_chats.values() {
            index_direct_chat_record(
                &mut self.active_direct_chat_pair_index,
                &mut self.direct_chat_pair_index,
                record,
            );
        }
    }

    fn rebuild_social_user_block_indexes(&mut self) {
        self.active_user_block_scope_index.clear();
        self.active_friendship_block_pair_index.clear();
        self.active_direct_chat_block_pair_index.clear();
        self.active_direct_chat_block_chat_index.clear();

        for record in self.user_blocks.values() {
            index_user_block_record(
                &mut self.active_user_block_scope_index,
                &mut self.active_friendship_block_pair_index,
                &mut self.active_direct_chat_block_pair_index,
                &mut self.active_direct_chat_block_chat_index,
                record,
            );
        }
    }

    fn rebuild_social_external_collaboration_indexes(&mut self) {
        self.active_external_connection_target_index.clear();
        self.active_external_member_mapping_index.clear();
        self.active_external_member_connection_index.clear();
        self.active_shared_channel_policy_target_index.clear();
        self.active_shared_channel_policy_connection_index.clear();

        for record in self.external_connections.values() {
            index_external_connection_record(
                &mut self.active_external_connection_target_index,
                record,
            );
        }
        for record in self.external_member_links.values() {
            index_external_member_link_record(
                &mut self.active_external_member_mapping_index,
                &mut self.active_external_member_connection_index,
                record,
            );
        }
        for record in self.shared_channel_policies.values() {
            index_shared_channel_policy_record(
                &mut self.active_shared_channel_policy_target_index,
                &mut self.active_shared_channel_policy_connection_index,
                record,
            );
        }
    }

    fn rebuild_social_indexes(&mut self) {
        self.rebuild_social_friend_request_indexes();
        self.rebuild_social_pair_indexes();
        self.rebuild_social_user_block_indexes();
        self.rebuild_social_external_collaboration_indexes();
        self.rebuild_shared_channel_pending_indexes();
        self.rebuild_social_committed_event_index();
    }

    fn rebuild_shared_channel_pending_indexes(&mut self) {
        self.pending_shared_channel_retry_index.clear();
        self.pending_shared_channel_lease_index.clear();
        for (request_key, pending) in &self.pending_shared_channel_sync_requests {
            index_pending_shared_channel_sync_request(
                &mut self.pending_shared_channel_retry_index,
                &mut self.pending_shared_channel_lease_index,
                request_key.as_str(),
                pending,
            );
        }
    }

    fn rebuild_social_committed_event_index(&mut self) {
        self.committed_event_index.clear();
        for record in self.friend_requests.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::FriendRequest {
                    request_id: record.friend_request.request_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.friendships.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::Friendship {
                    friendship_id: record.friendship.friendship_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.user_blocks.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::UserBlock {
                    block_id: record.user_block.block_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.direct_chats.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::DirectChat {
                    direct_chat_id: record.direct_chat.direct_chat_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.external_connections.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::ExternalConnection {
                    connection_id: record.external_connection.connection_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.external_member_links.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::ExternalMemberLink {
                    link_id: record.external_member_link.link_id.clone(),
                    commit_index: 0,
                },
            );
        }
        for record in self.shared_channel_policies.values() {
            index_social_commits(
                &mut self.committed_event_index,
                record.commits.as_slice(),
                SocialCommittedEventPointer::SharedChannelPolicy {
                    policy_id: record.shared_channel_policy.policy_id.clone(),
                    commit_index: 0,
                },
            );
        }
    }

    fn index_friend_request_commits(&mut self, request_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::FriendRequest {
                request_id: request_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_friendship_commits(&mut self, friendship_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::Friendship {
                friendship_id: friendship_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_user_block_commits(&mut self, block_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::UserBlock {
                block_id: block_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_direct_chat_commits(&mut self, direct_chat_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::DirectChat {
                direct_chat_id: direct_chat_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_external_connection_commits(
        &mut self,
        connection_id: &str,
        commits: &[CommitEnvelope],
    ) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::ExternalConnection {
                connection_id: connection_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_external_member_link_commits(&mut self, link_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::ExternalMemberLink {
                link_id: link_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn index_shared_channel_policy_commits(&mut self, policy_id: &str, commits: &[CommitEnvelope]) {
        index_social_commits(
            &mut self.committed_event_index,
            commits,
            SocialCommittedEventPointer::SharedChannelPolicy {
                policy_id: policy_id.to_owned(),
                commit_index: 0,
            },
        );
    }

    fn insert_friend_request_record(&mut self, request_id: String, record: StoredFriendRequest) {
        if let Some(previous) = self.friend_requests.insert(request_id, record.clone()) {
            unindex_friend_request_record(
                &mut self.pending_friend_request_pair_index,
                &mut self.accepted_friend_request_pair_index,
                &mut self.friend_request_user_index,
                &previous,
            );
        }
        index_friend_request_record(
            &mut self.pending_friend_request_pair_index,
            &mut self.accepted_friend_request_pair_index,
            &mut self.friend_request_user_index,
            &record,
        );
        self.index_friend_request_commits(
            record.friend_request.request_id.as_str(),
            record.commits.as_slice(),
        );
    }

    fn insert_friendship_record(&mut self, friendship_id: String, record: StoredFriendship) {
        if let Some(previous) = self.friendships.insert(friendship_id, record.clone()) {
            unindex_friendship_record(
                &mut self.active_friendship_pair_index,
                &mut self.active_friendship_user_index,
                &mut self.friendship_pair_index,
                &previous,
            );
        }
        index_friendship_record(
            &mut self.active_friendship_pair_index,
            &mut self.active_friendship_user_index,
            &mut self.friendship_pair_index,
            &record,
        );
        self.index_friendship_commits(
            record.friendship.friendship_id.as_str(),
            record.commits.as_slice(),
        );
    }

    fn insert_user_block_record(&mut self, block_id: String, record: StoredUserBlock) {
        if let Some(previous) = self.user_blocks.insert(block_id, record.clone()) {
            unindex_user_block_record(
                &mut self.active_user_block_scope_index,
                &mut self.active_friendship_block_pair_index,
                &mut self.active_direct_chat_block_pair_index,
                &mut self.active_direct_chat_block_chat_index,
                &previous,
            );
        }
        index_user_block_record(
            &mut self.active_user_block_scope_index,
            &mut self.active_friendship_block_pair_index,
            &mut self.active_direct_chat_block_pair_index,
            &mut self.active_direct_chat_block_chat_index,
            &record,
        );
        self.index_user_block_commits(
            record.user_block.block_id.as_str(),
            record.commits.as_slice(),
        );
    }

    fn insert_direct_chat_record(&mut self, direct_chat_id: String, record: StoredDirectChat) {
        if let Some(previous) = self.direct_chats.insert(direct_chat_id, record.clone()) {
            unindex_direct_chat_record(
                &mut self.active_direct_chat_pair_index,
                &mut self.direct_chat_pair_index,
                &previous,
            );
        }
        index_direct_chat_record(
            &mut self.active_direct_chat_pair_index,
            &mut self.direct_chat_pair_index,
            &record,
        );
        self.index_direct_chat_commits(
            record.direct_chat.direct_chat_id.as_str(),
            record.commits.as_slice(),
        );
    }

    fn insert_external_connection_record(
        &mut self,
        connection_id: String,
        record: StoredExternalConnection,
    ) {
        if let Some(previous) = self
            .external_connections
            .insert(connection_id, record.clone())
        {
            unindex_external_connection_record(
                &mut self.active_external_connection_target_index,
                &previous,
            );
        }
        index_external_connection_record(
            &mut self.active_external_connection_target_index,
            &record,
        );
        self.index_external_connection_commits(
            record.external_connection.connection_id.as_str(),
            record.commits.as_slice(),
        );
    }

    fn insert_external_member_link_record(
        &mut self,
        link_id: String,
        record: StoredExternalMemberLink,
    ) {
        if let Some(previous) = self.external_member_links.insert(link_id, record.clone()) {
            unindex_external_member_link_record(
                &mut self.active_external_member_mapping_index,
                &mut self.active_external_member_connection_index,
                &previous,
            );
        }
        index_external_member_link_record(
            &mut self.active_external_member_mapping_index,
            &mut self.active_external_member_connection_index,
            &record,
        );
        self.index_external_member_link_commits(
            record.external_member_link.link_id.as_str(),
            record.commits.as_slice(),
        );
    }

    fn insert_shared_channel_policy_record(
        &mut self,
        policy_id: String,
        record: StoredSharedChannelPolicy,
    ) {
        if let Some(previous) = self
            .shared_channel_policies
            .insert(policy_id, record.clone())
        {
            unindex_shared_channel_policy_record(
                &mut self.active_shared_channel_policy_target_index,
                &mut self.active_shared_channel_policy_connection_index,
                &previous,
            );
        }
        index_shared_channel_policy_record(
            &mut self.active_shared_channel_policy_target_index,
            &mut self.active_shared_channel_policy_connection_index,
            &record,
        );
        self.index_shared_channel_policy_commits(
            record.shared_channel_policy.policy_id.as_str(),
            record.commits.as_slice(),
        );
    }

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
        let pointer = self
            .committed_event_index
            .get(&SocialCommittedEventIndexKey::new(tenant_id, event_id))?;
        match pointer {
            SocialCommittedEventPointer::FriendRequest {
                request_id,
                commit_index,
            } => {
                let record = self.friend_requests.get(request_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::FriendRequest { record, commit })
            }
            SocialCommittedEventPointer::Friendship {
                friendship_id,
                commit_index,
            } => {
                let record = self.friendships.get(friendship_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::Friendship { record, commit })
            }
            SocialCommittedEventPointer::UserBlock {
                block_id,
                commit_index,
            } => {
                let record = self.user_blocks.get(block_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::UserBlock { record, commit })
            }
            SocialCommittedEventPointer::DirectChat {
                direct_chat_id,
                commit_index,
            } => {
                let record = self.direct_chats.get(direct_chat_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::DirectChat { record, commit })
            }
            SocialCommittedEventPointer::ExternalConnection {
                connection_id,
                commit_index,
            } => {
                let record = self.external_connections.get(connection_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::ExternalConnection { record, commit })
            }
            SocialCommittedEventPointer::ExternalMemberLink {
                link_id,
                commit_index,
            } => {
                let record = self.external_member_links.get(link_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::ExternalMemberLink { record, commit })
            }
            SocialCommittedEventPointer::SharedChannelPolicy {
                policy_id,
                commit_index,
            } => {
                let record = self.shared_channel_policies.get(policy_id)?.clone();
                let commit = record.commits.get(*commit_index)?.clone();
                if commit.tenant_id != tenant_id || commit.event_id != event_id {
                    return None;
                }
                Some(SocialCommittedEvent::SharedChannelPolicy { record, commit })
            }
        }
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
            delivered_shared_channel_sync_requests: self
                .delivered_shared_channel_sync_requests
                .len(),
            recent_shared_channel_sync_deliveries: self.recent_shared_channel_sync_deliveries.len(),
        }
    }

    fn merge_pending_shared_channel_sync_requests_from(&mut self, other: &Self) {
        for (key, pending) in &other.pending_shared_channel_sync_requests {
            if !self.pending_shared_channel_sync_requests.contains_key(key) {
                self.upsert_pending_shared_channel_sync_request(key.clone(), pending.clone());
            }
        }
    }

    fn merge_dead_letter_shared_channel_sync_requests_from(&mut self, other: &Self) {
        for (key, dead_letter) in &other.dead_letter_shared_channel_sync_requests {
            self.dead_letter_shared_channel_sync_requests
                .entry(key.clone())
                .or_insert_with(|| dead_letter.clone());
        }
    }

    fn merge_delivered_shared_channel_sync_requests_from(&mut self, other: &Self) {
        for (key, delivered_at) in &other.delivered_shared_channel_sync_requests {
            self.delivered_shared_channel_sync_requests
                .entry(key.clone())
                .and_modify(|existing| {
                    if timestamp_newer_for_recency(delivered_at, existing) {
                        *existing = delivered_at.clone();
                    }
                })
                .or_insert_with(|| delivered_at.clone());
        }
    }

    fn merge_delivered_shared_channel_sync_delivery_proofs_from(&mut self, other: &Self) {
        for (key, proof) in &other.delivered_shared_channel_sync_delivery_proofs {
            self.delivered_shared_channel_sync_delivery_proofs
                .entry(key.clone())
                .and_modify(|existing| {
                    if timestamp_newer_for_recency(
                        proof.delivered_at.as_str(),
                        existing.delivered_at.as_str(),
                    ) || (proof.delivered_at == existing.delivered_at
                        && existing.status
                            == SharedChannelSyncDeliveryProofStatus::TransportAccepted
                        && proof.status != SharedChannelSyncDeliveryProofStatus::TransportAccepted)
                    {
                        *existing = proof.clone();
                    }
                })
                .or_insert_with(|| proof.clone());
        }
    }

    fn merge_recent_shared_channel_sync_deliveries_from(&mut self, other: &Self) {
        for (key, delivered_at) in &other.recent_shared_channel_sync_deliveries {
            self.recent_shared_channel_sync_deliveries
                .entry(key.clone())
                .and_modify(|existing| {
                    if timestamp_newer_for_recency(delivered_at, existing) {
                        *existing = delivered_at.clone();
                    }
                })
                .or_insert_with(|| delivered_at.clone());
        }
    }

    fn pending_shared_channel_sync_count(&self) -> usize {
        self.pending_shared_channel_sync_requests.len()
    }

    fn dead_letter_shared_channel_sync_count(&self) -> usize {
        self.dead_letter_shared_channel_sync_requests.len()
    }

    fn prune_delivered_shared_channel_sync_backlog(&mut self) -> usize {
        if self.delivered_shared_channel_sync_requests.is_empty() {
            return 0;
        }
        let pending_keys = self
            .pending_shared_channel_sync_requests
            .keys()
            .filter(|key| {
                self.delivered_shared_channel_sync_requests
                    .contains_key(key.as_str())
            })
            .cloned()
            .collect::<Vec<_>>();
        let dead_letter_keys = self
            .dead_letter_shared_channel_sync_requests
            .keys()
            .filter(|key| {
                self.delivered_shared_channel_sync_requests
                    .contains_key(key.as_str())
            })
            .cloned()
            .collect::<Vec<_>>();
        for key in &pending_keys {
            self.remove_pending_shared_channel_sync_request_by_key(key.as_str());
        }
        for key in &dead_letter_keys {
            self.dead_letter_shared_channel_sync_requests
                .remove(key.as_str());
        }
        pending_keys.len() + dead_letter_keys.len()
    }

    fn prune_delivered_shared_channel_sync_requests(
        &mut self,
        retention_window_start: &str,
        max_entries: usize,
    ) -> usize {
        if self.delivered_shared_channel_sync_requests.is_empty() {
            return 0;
        }
        let protected_keys = self
            .pending_shared_channel_sync_requests
            .keys()
            .chain(self.dead_letter_shared_channel_sync_requests.keys())
            .cloned()
            .collect::<BTreeSet<_>>();

        let stale_keys = self
            .delivered_shared_channel_sync_requests
            .iter()
            .filter(|(key, delivered_at)| {
                timestamp_before_or_noncanonical_for_retention(
                    delivered_at.as_str(),
                    retention_window_start,
                ) && !protected_keys.contains(key.as_str())
            })
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>();
        let mut removed = 0usize;
        for key in stale_keys {
            if self
                .delivered_shared_channel_sync_requests
                .remove(key.as_str())
                .is_some()
            {
                self.delivered_shared_channel_sync_delivery_proofs
                    .remove(key.as_str());
                removed = removed.saturating_add(1);
            }
        }

        if self.delivered_shared_channel_sync_requests.len() <= max_entries {
            return removed;
        }

        let overflow = self
            .delivered_shared_channel_sync_requests
            .len()
            .saturating_sub(max_entries);
        if overflow == 0 {
            return removed;
        }

        let mut oldest_candidates = self
            .delivered_shared_channel_sync_requests
            .iter()
            .filter(|(key, _)| !protected_keys.contains(key.as_str()))
            .map(|(key, delivered_at)| (key.clone(), delivered_at.clone()))
            .collect::<Vec<_>>();
        oldest_candidates.sort_by(|left, right| {
            left.1
                .cmp(&right.1)
                .then_with(|| left.0.as_str().cmp(right.0.as_str()))
        });
        for (key, _) in oldest_candidates.into_iter().take(overflow) {
            if self
                .delivered_shared_channel_sync_requests
                .remove(key.as_str())
                .is_some()
            {
                self.delivered_shared_channel_sync_delivery_proofs
                    .remove(key.as_str());
                removed = removed.saturating_add(1);
            }
        }
        removed
    }

    fn pending_shared_channel_sync_requests(&self) -> Vec<PendingSharedChannelSyncRequest> {
        self.pending_shared_channel_sync_requests
            .values()
            .cloned()
            .collect()
    }

    fn selected_pending_shared_channel_sync_requests(
        &self,
        request_keys: &BTreeSet<String>,
    ) -> Vec<(String, PendingSharedChannelSyncRequest)> {
        request_keys
            .iter()
            .filter_map(|request_key| {
                self.pending_shared_channel_sync_requests
                    .get(request_key.as_str())
                    .map(|pending| (request_key.clone(), pending.clone()))
            })
            .collect()
    }

    fn selected_undelivered_pending_shared_channel_sync_requests(
        &self,
        request_keys: &BTreeSet<String>,
    ) -> Vec<(String, PendingSharedChannelSyncRequest)> {
        request_keys
            .iter()
            .filter_map(|request_key| {
                let pending = self
                    .pending_shared_channel_sync_requests
                    .get(request_key.as_str())?;
                if self
                    .delivered_shared_channel_sync_requests
                    .contains_key(request_key.as_str())
                {
                    None
                } else {
                    Some((request_key.clone(), pending.clone()))
                }
            })
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
            if self
                .delivered_shared_channel_sync_requests
                .contains_key(key.as_str())
            {
                let removed_pending =
                    self.remove_pending_shared_channel_sync_request_by_key(key.as_str());
                let removed_dead_letter = self
                    .dead_letter_shared_channel_sync_requests
                    .remove(key.as_str())
                    .is_some();
                changed |= removed_pending || removed_dead_letter;
                continue;
            }
            if let Some(dead_letter) = self.dead_letter_shared_channel_sync_requests.get_mut(&key) {
                dead_letter.request = request.clone();
                dead_letter.failure_count = dead_letter.failure_count.saturating_add(1);
                dead_letter.last_error = error.to_owned();
                dead_letter.last_failed_at = Some(now.to_owned());
                self.record_failed_shared_channel_sync_delivery_proof(key.as_str(), now);
                changed = true;
                continue;
            }

            let failed_request = if let Some(existing) = self
                .pending_shared_channel_sync_requests
                .get(key.as_str())
                .cloned()
            {
                let mut pending = existing;
                if pending.lease_status(now) == SocialSharedChannelSyncLeaseStatus::Stale {
                    pending.clear_owner();
                }
                pending.request = request.clone();
                pending.failure_count = pending.failure_count.saturating_add(1);
                pending.last_error = error.to_owned();
                pending.last_failed_at = Some(now.to_owned());
                self.upsert_pending_shared_channel_sync_request(key.clone(), pending.clone());
                pending
            } else {
                let pending = PendingSharedChannelSyncRequest {
                    request: request.clone(),
                    failure_count: 1,
                    last_error: error.to_owned(),
                    last_failed_at: Some(now.to_owned()),
                    owner_actor_id: None,
                    owner_actor_kind: None,
                    claimed_at: None,
                    lease_expires_at: None,
                };
                self.upsert_pending_shared_channel_sync_request(key.clone(), pending.clone());
                pending
            };
            if failed_request.failure_count >= SHARED_CHANNEL_SYNC_DEAD_LETTER_FAILURE_THRESHOLD {
                let mut dead_letter_request = failed_request;
                dead_letter_request.clear_owner();
                self.remove_pending_shared_channel_sync_request_by_key(key.as_str());
                self.dead_letter_shared_channel_sync_requests
                    .insert(key, dead_letter_request);
            }
            self.record_failed_shared_channel_sync_delivery_proof(
                shared_channel_sync_request_key(request).as_str(),
                now,
            );
            changed = true;
        }
        changed
    }

    fn record_failed_shared_channel_sync_delivery_proof(
        &mut self,
        request_key: &str,
        failed_at: &str,
    ) {
        let failed_proof = StoredSharedChannelSyncDeliveryProof {
            delivered_at: failed_at.to_owned(),
            status: SharedChannelSyncDeliveryProofStatus::Failed,
            proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
            target: None,
        };
        let should_update = self
            .delivered_shared_channel_sync_delivery_proofs
            .get(request_key)
            .is_none_or(|existing| {
                timestamp_newer_for_recency(
                    failed_proof.delivered_at.as_str(),
                    existing.delivered_at.as_str(),
                ) || (failed_proof.delivered_at == existing.delivered_at
                    && existing.status == SharedChannelSyncDeliveryProofStatus::TransportAccepted)
            });
        if should_update {
            self.delivered_shared_channel_sync_delivery_proofs
                .insert(request_key.to_owned(), failed_proof);
        }
    }

    fn is_dead_letter_shared_channel_sync_request(
        &self,
        request: &SharedChannelLinkedMemberSyncRequest,
    ) -> bool {
        self.dead_letter_shared_channel_sync_requests
            .contains_key(shared_channel_sync_request_key(request).as_str())
    }

    fn is_delivered_shared_channel_sync_request(
        &self,
        request: &SharedChannelLinkedMemberSyncRequest,
    ) -> bool {
        self.delivered_shared_channel_sync_requests
            .contains_key(shared_channel_sync_request_key(request).as_str())
    }

    fn remove_pending_shared_channel_sync_request(
        &mut self,
        request: &SharedChannelLinkedMemberSyncRequest,
    ) -> bool {
        self.remove_pending_shared_channel_sync_request_by_key(
            shared_channel_sync_request_key(request).as_str(),
        )
    }

    fn upsert_pending_shared_channel_sync_request(
        &mut self,
        request_key: String,
        pending: PendingSharedChannelSyncRequest,
    ) {
        if let Some(previous) = self
            .pending_shared_channel_sync_requests
            .insert(request_key.clone(), pending.clone())
        {
            unindex_pending_shared_channel_sync_request(
                &mut self.pending_shared_channel_retry_index,
                &mut self.pending_shared_channel_lease_index,
                request_key.as_str(),
                &previous,
            );
        }
        index_pending_shared_channel_sync_request(
            &mut self.pending_shared_channel_retry_index,
            &mut self.pending_shared_channel_lease_index,
            request_key.as_str(),
            &pending,
        );
    }

    fn remove_pending_shared_channel_sync_request_by_key(&mut self, request_key: &str) -> bool {
        if let Some(previous) = self
            .pending_shared_channel_sync_requests
            .remove(request_key)
        {
            unindex_pending_shared_channel_sync_request(
                &mut self.pending_shared_channel_retry_index,
                &mut self.pending_shared_channel_lease_index,
                request_key,
                &previous,
            );
            true
        } else {
            false
        }
    }

    fn retryable_pending_shared_channel_sync_requests(
        &self,
        retry_window_start: &str,
    ) -> Vec<PendingSharedChannelSyncRequest> {
        self.pending_shared_channel_retry_index
            .range(..=SharedChannelRetryIndexKey::new(retry_window_start))
            .flat_map(|(_, request_keys)| request_keys.iter())
            .filter_map(|request_key| {
                self.pending_shared_channel_sync_requests
                    .get(request_key.as_str())
                    .cloned()
            })
            .collect()
    }

    fn pending_shared_channel_sync_request_blocks_dispatch(
        &self,
        request: &SharedChannelLinkedMemberSyncRequest,
        now: &str,
        retry_window_start: &str,
    ) -> bool {
        let request_key = shared_channel_sync_request_key(request);
        self.pending_shared_channel_sync_requests
            .get(request_key.as_str())
            .is_some_and(|pending| !pending.auto_dispatch_eligible(now, retry_window_start))
    }

    fn stale_pending_shared_channel_sync_requests(
        &self,
        now: &str,
    ) -> Vec<(String, PendingSharedChannelSyncRequest)> {
        self.pending_shared_channel_lease_index
            .range(..=SharedChannelLeaseIndexKey::new(now))
            .flat_map(|(_, request_keys)| request_keys.iter())
            .filter_map(|request_key| {
                self.pending_shared_channel_sync_requests
                    .get(request_key.as_str())
                    .filter(|pending| {
                        pending.lease_status(now) == SocialSharedChannelSyncLeaseStatus::Stale
                    })
                    .cloned()
                    .map(|pending| (request_key.clone(), pending))
            })
            .collect()
    }

    fn recently_dispatched_shared_channel_sync_request(
        &self,
        request: &SharedChannelLinkedMemberSyncRequest,
        dedup_window_start: &str,
    ) -> bool {
        self.recent_shared_channel_sync_deliveries
            .get(shared_channel_sync_request_key(request).as_str())
            .is_some_and(|delivered_at| {
                timestamp_not_before_for_dedup(delivered_at.as_str(), dedup_window_start)
            })
    }

    fn record_dispatched_shared_channel_sync_request(
        &mut self,
        request: &SharedChannelLinkedMemberSyncRequest,
        delivered_at: &str,
        dedup_window_start: &str,
        proof: Option<&SharedChannelSyncDeliveryProof>,
        replayed: bool,
    ) -> bool {
        let key = shared_channel_sync_request_key(request);
        self.delivered_shared_channel_sync_requests
            .entry(key.clone())
            .and_modify(|existing| {
                if timestamp_newer_for_recency(delivered_at, existing.as_str()) {
                    *existing = delivered_at.to_owned();
                }
            })
            .or_insert_with(|| delivered_at.to_owned());

        let mut proof_record = proof
            .cloned()
            .unwrap_or_else(|| SharedChannelSyncDeliveryProof::transport_accepted(key.clone()));
        if replayed
            && matches!(
                proof_record.status,
                SharedChannelSyncDeliveryProofStatus::TransportAccepted
                    | SharedChannelSyncDeliveryProofStatus::Applied
                    | SharedChannelSyncDeliveryProofStatus::AlreadyLinked
            )
        {
            proof_record.status = SharedChannelSyncDeliveryProofStatus::Replayed;
        }
        let delivered_proof = StoredSharedChannelSyncDeliveryProof {
            delivered_at: delivered_at.to_owned(),
            status: proof_record.status,
            proof_version: proof_record.proof_version,
            target: proof_record.target,
        };
        let proof_should_update = self
            .delivered_shared_channel_sync_delivery_proofs
            .get(key.as_str())
            .is_none_or(|existing| {
                timestamp_newer_for_recency(
                    delivered_proof.delivered_at.as_str(),
                    existing.delivered_at.as_str(),
                ) || (delivered_proof.delivered_at == existing.delivered_at
                    && existing.status == SharedChannelSyncDeliveryProofStatus::TransportAccepted
                    && delivered_proof.status
                        != SharedChannelSyncDeliveryProofStatus::TransportAccepted)
            });
        if proof_should_update {
            self.delivered_shared_channel_sync_delivery_proofs
                .insert(key.clone(), delivered_proof);
        }

        self.recent_shared_channel_sync_deliveries
            .retain(|_, existing_delivered_at| {
                timestamp_not_before_for_dedup(existing_delivered_at.as_str(), dedup_window_start)
            });
        let should_update = self
            .recent_shared_channel_sync_deliveries
            .get(key.as_str())
            .map(String::as_str)
            != Some(delivered_at);
        if should_update {
            self.recent_shared_channel_sync_deliveries
                .insert(key, delivered_at.to_owned());
        }
        should_update || proof_should_update
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
            if self
                .delivered_shared_channel_sync_requests
                .contains_key(key.as_str())
            {
                self.remove_pending_shared_channel_sync_request_by_key(key.as_str());
                continue;
            }
            dead_letter.failure_count = 0;
            dead_letter.clear_owner();
            self.upsert_pending_shared_channel_sync_request(key, dead_letter);
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
            if self
                .delivered_shared_channel_sync_requests
                .contains_key(key.as_str())
            {
                self.remove_pending_shared_channel_sync_request_by_key(key.as_str());
                continue;
            }
            let Some(mut pending) = self
                .pending_shared_channel_sync_requests
                .get(key.as_str())
                .cloned()
            else {
                continue;
            };
            if pending.is_claimed_by_other(actor_id, actor_kind) {
                result.conflicted += 1;
                result
                    .conflict_items
                    .push(social_shared_channel_sync_conflict_details(
                        key.as_str(),
                        &pending,
                        actor_id,
                        actor_kind,
                        now,
                    ));
                continue;
            }
            pending.assign_owner(actor_id, actor_kind);
            self.upsert_pending_shared_channel_sync_request(key, pending);
            result.claimed += 1;
        }
        result
    }

    fn reclaim_stale_pending_shared_channel_sync_claims(&mut self, now: &str) -> usize {
        let mut reclaimed = 0usize;
        for (request_key, mut pending) in self.stale_pending_shared_channel_sync_requests(now) {
            pending.clear_owner();
            self.upsert_pending_shared_channel_sync_request(request_key, pending);
            reclaimed += 1;
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
            if self
                .delivered_shared_channel_sync_requests
                .contains_key(key.as_str())
            {
                self.remove_pending_shared_channel_sync_request_by_key(key.as_str());
                continue;
            }
            let Some(mut pending) = self
                .pending_shared_channel_sync_requests
                .get(key.as_str())
                .cloned()
            else {
                continue;
            };
            if !pending.is_owned_by(actor_id, actor_kind) {
                continue;
            }
            pending.clear_owner();
            self.upsert_pending_shared_channel_sync_request(key, pending);
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
            if self
                .delivered_shared_channel_sync_requests
                .contains_key(key.as_str())
            {
                self.remove_pending_shared_channel_sync_request_by_key(key.as_str());
                continue;
            }
            let Some(mut pending) = self
                .pending_shared_channel_sync_requests
                .get(key.as_str())
                .cloned()
            else {
                continue;
            };
            if !pending.is_claimed_by_other(actor_id, actor_kind) {
                continue;
            }
            pending.assign_owner(actor_id, actor_kind);
            self.upsert_pending_shared_channel_sync_request(key, pending);
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
            "friend_request.accepted" => self.apply_friend_request_accepted_commit(commit),
            "friend_request.declined" => self.apply_friend_request_declined_commit(commit),
            "friend_request.canceled" => self.apply_friend_request_canceled_commit(commit),
            "friendship.activated" => self.apply_friendship_commit(commit),
            "friendship.removed" => self.apply_friendship_removed_commit(commit),
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
        let request_id = friend_request.request_id.clone();
        let mut record = self
            .friend_requests
            .get(request_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredFriendRequest {
                friend_request: friend_request.clone(),
                commits: Vec::new(),
            });
        record.friend_request = friend_request;
        record.commits.push(commit);
        self.insert_friend_request_record(request_id, record);
        Ok(())
    }

    fn apply_friend_request_accepted_commit(
        &mut self,
        commit: CommitEnvelope,
    ) -> Result<(), String> {
        let payload: FriendRequestAcceptedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| {
            format!(
                "failed to parse friend request accept replay payload for {}: {error}",
                commit.event_id
            )
        })?;
        validate_social_commit_target(
            &commit,
            AggregateType::FriendRequest,
            payload.request_id.as_str(),
        )?;
        let mut record = self
            .friend_requests
            .get(payload.request_id.as_str())
            .cloned()
            .ok_or_else(|| {
                format!(
                    "friend request accept replay payload for {} references missing request {}",
                    commit.event_id, payload.request_id
                )
            })?;
        if !matches!(record.friend_request.status, FriendRequestStatus::Pending) {
            return Err(format!(
                "friend request accept replay payload for {} cannot transition request {} from {:?}",
                commit.event_id, payload.request_id, record.friend_request.status
            ));
        }
        if payload.accepted_by_user_id != record.friend_request.target_user_id {
            return Err(format!(
                "friend request accept replay payload for {} must be accepted by target user {}",
                commit.event_id, record.friend_request.target_user_id
            ));
        }

        let request_id = record.friend_request.request_id.clone();
        record.friend_request.status = FriendRequestStatus::Accepted;
        record.friend_request.updated_at = payload.accepted_at;
        record.commits.push(commit);
        self.insert_friend_request_record(request_id, record);
        Ok(())
    }

    fn apply_friend_request_declined_commit(
        &mut self,
        commit: CommitEnvelope,
    ) -> Result<(), String> {
        let payload: FriendRequestDeclinedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| {
            format!(
                "failed to parse friend request decline replay payload for {}: {error}",
                commit.event_id
            )
        })?;
        validate_social_commit_target(
            &commit,
            AggregateType::FriendRequest,
            payload.request_id.as_str(),
        )?;
        let mut record = self
            .friend_requests
            .get(payload.request_id.as_str())
            .cloned()
            .ok_or_else(|| {
                format!(
                    "friend request decline replay payload for {} references missing request {}",
                    commit.event_id, payload.request_id
                )
            })?;
        if !matches!(
            record.friend_request.status,
            FriendRequestStatus::Pending | FriendRequestStatus::Declined
        ) {
            return Err(format!(
                "friend request decline replay payload for {} cannot transition request {} from {:?}",
                commit.event_id, payload.request_id, record.friend_request.status
            ));
        }
        if payload.declined_by_user_id != record.friend_request.target_user_id {
            return Err(format!(
                "friend request decline replay payload for {} must be declined by target user {}",
                commit.event_id, record.friend_request.target_user_id
            ));
        }

        let request_id = record.friend_request.request_id.clone();
        record.friend_request.status = FriendRequestStatus::Declined;
        record.friend_request.updated_at = payload.declined_at;
        record.commits.push(commit);
        self.insert_friend_request_record(request_id, record);
        Ok(())
    }

    fn apply_friend_request_canceled_commit(
        &mut self,
        commit: CommitEnvelope,
    ) -> Result<(), String> {
        let payload: FriendRequestCanceledPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| {
            format!(
                "failed to parse friend request cancel replay payload for {}: {error}",
                commit.event_id
            )
        })?;
        validate_social_commit_target(
            &commit,
            AggregateType::FriendRequest,
            payload.request_id.as_str(),
        )?;
        let mut record = self
            .friend_requests
            .get(payload.request_id.as_str())
            .cloned()
            .ok_or_else(|| {
                format!(
                    "friend request cancel replay payload for {} references missing request {}",
                    commit.event_id, payload.request_id
                )
            })?;
        if !matches!(
            record.friend_request.status,
            FriendRequestStatus::Pending | FriendRequestStatus::Canceled
        ) {
            return Err(format!(
                "friend request cancel replay payload for {} cannot transition request {} from {:?}",
                commit.event_id, payload.request_id, record.friend_request.status
            ));
        }
        if payload.canceled_by_user_id != record.friend_request.requester_user_id {
            return Err(format!(
                "friend request cancel replay payload for {} must be canceled by requester user {}",
                commit.event_id, record.friend_request.requester_user_id
            ));
        }

        let request_id = record.friend_request.request_id.clone();
        record.friend_request.status = FriendRequestStatus::Canceled;
        record.friend_request.updated_at = payload.canceled_at;
        record.commits.push(commit);
        self.insert_friend_request_record(request_id, record);
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
        let friendship_id = friendship.friendship_id.clone();
        let mut record = self
            .friendships
            .get(friendship_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredFriendship {
                friendship: friendship.clone(),
                commits: Vec::new(),
            });
        record.friendship = friendship;
        record.commits.push(commit);
        self.insert_friendship_record(friendship_id, record);
        Ok(())
    }

    fn apply_friendship_removed_commit(&mut self, commit: CommitEnvelope) -> Result<(), String> {
        let payload: FriendshipRemovedPayload = serde_json::from_str(commit.payload.as_str())
            .map_err(|error| {
                format!(
                    "failed to parse friendship removal replay payload for {}: {error}",
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
                    "failed to validate friendship removal replay payload for {}: {error}",
                    commit.event_id
                )
            })?;
        if payload.removed_by_user_id != pair.user_low_id
            && payload.removed_by_user_id != pair.user_high_id
        {
            return Err(format!(
                "friendship removal replay payload for {} has foreign remover {}",
                commit.event_id, payload.removed_by_user_id
            ));
        }

        let friendship = Friendship {
            tenant_id: commit.tenant_id.clone(),
            friendship_id: payload.friendship_id.clone(),
            user_low_id: pair.user_low_id.clone(),
            user_high_id: pair.user_high_id.clone(),
            initiator_user_id: payload.removed_by_user_id,
            status: FriendshipStatus::Removed,
            established_at: None,
            updated_at: payload.removed_at,
        };
        let archived_at;
        let tenant_id = commit.tenant_id.clone();
        {
            let mut record = self
                .friendships
                .get(friendship.friendship_id.as_str())
                .cloned()
                .unwrap_or_else(|| StoredFriendship {
                    friendship: friendship.clone(),
                    commits: Vec::new(),
                });
            if record.friendship.user_low_id != pair.user_low_id
                || record.friendship.user_high_id != pair.user_high_id
            {
                return Err(format!(
                    "friendship removal replay payload for {} does not match stored pair",
                    commit.event_id
                ));
            }
            record.friendship.status = FriendshipStatus::Removed;
            record.friendship.updated_at = friendship.updated_at.clone();
            if record.friendship.established_at.is_none() {
                record.friendship.established_at = friendship.established_at.clone();
            }
            if record.friendship.initiator_user_id.trim().is_empty() {
                record.friendship.initiator_user_id = friendship.initiator_user_id.clone();
            }
            archived_at = record.friendship.updated_at.clone();
            record.commits.push(commit);
            self.insert_friendship_record(friendship.friendship_id.clone(), record);
        }
        archive_active_direct_chats_for_pair(
            self,
            tenant_id.as_str(),
            pair.user_low_id.as_str(),
            pair.user_high_id.as_str(),
            archived_at.as_str(),
        );
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
        let block_id = user_block.block_id.clone();
        let mut record = self
            .user_blocks
            .get(block_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredUserBlock {
                user_block: user_block.clone(),
                commits: Vec::new(),
            });
        record.user_block = user_block;
        record.commits.push(commit);
        self.insert_user_block_record(block_id, record);
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
        let direct_chat_id = direct_chat.direct_chat_id.clone();
        let mut record = self
            .direct_chats
            .get(direct_chat_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredDirectChat {
                direct_chat: direct_chat.clone(),
                commits: Vec::new(),
            });
        record.direct_chat = direct_chat;
        record.commits.push(commit);
        self.insert_direct_chat_record(direct_chat_id, record);
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
        let connection_id = external_connection.connection_id.clone();
        let mut record = self
            .external_connections
            .get(connection_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredExternalConnection {
                external_connection: external_connection.clone(),
                commits: Vec::new(),
            });
        record.external_connection = external_connection;
        record.commits.push(commit);
        self.insert_external_connection_record(connection_id, record);
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
        let link_id = external_member_link.link_id.clone();
        let mut record = self
            .external_member_links
            .get(link_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredExternalMemberLink {
                external_member_link: external_member_link.clone(),
                commits: Vec::new(),
            });
        record.external_member_link = external_member_link;
        record.commits.push(commit);
        self.insert_external_member_link_record(link_id, record);
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
        let policy_id = shared_channel_policy.policy_id.clone();
        let mut record = self
            .shared_channel_policies
            .get(policy_id.as_str())
            .cloned()
            .unwrap_or_else(|| StoredSharedChannelPolicy {
                shared_channel_policy: shared_channel_policy.clone(),
                commits: Vec::new(),
            });
        record.shared_channel_policy = shared_channel_policy;
        record.commits.push(commit);
        self.insert_shared_channel_policy_record(policy_id, record);
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
struct AcceptFriendRequestRequest {
    event_id: String,
    accepted_by_user_id: String,
    accepted_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeclineFriendRequestRequest {
    event_id: String,
    declined_by_user_id: String,
    declined_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CancelFriendRequestRequest {
    event_id: String,
    canceled_by_user_id: String,
    canceled_at: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FriendRequestInventoryDirectionQuery {
    Incoming,
    Outgoing,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FriendRequestInventoryStatusQuery {
    #[default]
    Pending,
    Accepted,
    Declined,
    Canceled,
    Expired,
    All,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FriendRequestInventoryQuery {
    user_id: String,
    direction: FriendRequestInventoryDirectionQuery,
    #[serde(default)]
    status: FriendRequestInventoryStatusQuery,
    limit: Option<usize>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SharedChannelSyncInventoryQuery {
    limit: Option<usize>,
    cursor: Option<String>,
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
struct RemoveFriendshipRequest {
    event_id: String,
    removed_by_user_id: String,
    removed_at: String,
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
    im_sdk_family: &'static str,
    app_sdk_family: &'static str,
    backend_sdk_family: &'static str,
    rtc_sdk_family: &'static str,
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
    Accepted,
    Declined,
    Canceled,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialFriendRequestReadStatus {
    Inventory,
    Snapshot,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum SocialFriendshipWriteStatus {
    Activated,
    Removed,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    friendship: Option<Friendship>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    friendship_latest_commit: Option<CommitEnvelopeResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    direct_chat: Option<DirectChat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    direct_chat_latest_commit: Option<CommitEnvelopeResponse>,
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
struct SocialFriendRequestInventoryResponse {
    status: SocialFriendRequestReadStatus,
    items: Vec<FriendRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FriendRequestInventoryCursor {
    v: u64,
    updated_at: String,
    created_at: String,
    request_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SharedChannelSyncInventoryCursor {
    v: u64,
    request_key: String,
}

#[derive(Clone, Debug)]
struct SharedChannelSyncInventoryPageSpec {
    limit: usize,
    cursor: Option<SharedChannelSyncInventoryCursor>,
}

#[derive(Debug)]
struct FriendRequestInventoryPage {
    items: Vec<FriendRequest>,
    next_cursor: Option<String>,
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
pub enum SocialSharedChannelSyncDeliveredInventoryStatus {
    Snapshot,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SocialSharedChannelSyncDeliveryStateInventoryStatus {
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
    pub delivered_shared_channel_sync_requests: usize,
    pub recent_shared_channel_sync_deliveries: usize,
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
    pub last_failed_at: Option<String>,
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
    pub next_cursor: Option<String>,
    pub items: Vec<SocialSharedChannelSyncInventoryItemResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncPendingInventoryResponse {
    pub status: SocialSharedChannelSyncPendingInventoryStatus,
    pub pending_count: usize,
    pub next_cursor: Option<String>,
    pub items: Vec<SocialSharedChannelSyncInventoryItemResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeliveredInventoryItemResponse {
    pub request_key: String,
    pub delivered_at: String,
    pub status: SharedChannelSyncDeliveryProofStatus,
    pub proof_version: Option<String>,
    pub target: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeliveredInventoryResponse {
    pub status: SocialSharedChannelSyncDeliveredInventoryStatus,
    pub delivered_count: usize,
    pub next_cursor: Option<String>,
    pub items: Vec<SocialSharedChannelSyncDeliveredInventoryItemResponse>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeliveryStateInventoryItemResponse {
    pub request_key: String,
    pub status: SharedChannelSyncDeliveryProofStatus,
    pub updated_at: Option<String>,
    pub proof_version: Option<String>,
    pub target: Option<String>,
    pub failure_count: u32,
    pub last_error: Option<String>,
    pub pending: bool,
    pub dead_letter: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialSharedChannelSyncDeliveryStateInventoryResponse {
    pub status: SocialSharedChannelSyncDeliveryStateInventoryStatus,
    pub delivered_count: usize,
    pub pending_count: usize,
    pub dead_letter_count: usize,
    pub total_count: usize,
    pub next_cursor: Option<String>,
    pub items: Vec<SocialSharedChannelSyncDeliveryStateInventoryItemResponse>,
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
        last_failed_at: request.last_failed_at,
        owner_actor_id: request.owner_actor_id,
        owner_actor_kind: request.owner_actor_kind,
        claimed_at: request.claimed_at,
        lease_expires_at: request.lease_expires_at,
        lease_status,
        takeover_eligible,
        legacy_takeover_required,
    }
}

fn social_shared_channel_sync_delivered_inventory_item_response(
    request_key: String,
    delivered_at: String,
    proof: Option<StoredSharedChannelSyncDeliveryProof>,
) -> SocialSharedChannelSyncDeliveredInventoryItemResponse {
    let fallback = StoredSharedChannelSyncDeliveryProof {
        delivered_at: delivered_at.clone(),
        status: SharedChannelSyncDeliveryProofStatus::TransportAccepted,
        proof_version: None,
        target: None,
    };
    let resolved = proof.unwrap_or(fallback);
    SocialSharedChannelSyncDeliveredInventoryItemResponse {
        request_key,
        delivered_at: resolved.delivered_at,
        status: resolved.status,
        proof_version: resolved.proof_version,
        target: resolved.target,
    }
}

fn social_shared_channel_sync_delivery_state_item_from_proof(
    request_key: String,
    delivered_at: String,
    proof: Option<StoredSharedChannelSyncDeliveryProof>,
) -> SocialSharedChannelSyncDeliveryStateInventoryItemResponse {
    let fallback = StoredSharedChannelSyncDeliveryProof {
        delivered_at: delivered_at.clone(),
        status: SharedChannelSyncDeliveryProofStatus::TransportAccepted,
        proof_version: None,
        target: None,
    };
    let resolved = proof.unwrap_or(fallback);
    SocialSharedChannelSyncDeliveryStateInventoryItemResponse {
        request_key,
        status: resolved.status,
        updated_at: Some(resolved.delivered_at),
        proof_version: resolved.proof_version,
        target: resolved.target,
        failure_count: 0,
        last_error: None,
        pending: false,
        dead_letter: false,
    }
}

fn social_shared_channel_sync_delivery_state_item_from_pending(
    request_key: String,
    request: PendingSharedChannelSyncRequest,
    dead_letter: bool,
) -> SocialSharedChannelSyncDeliveryStateInventoryItemResponse {
    SocialSharedChannelSyncDeliveryStateInventoryItemResponse {
        request_key,
        status: SharedChannelSyncDeliveryProofStatus::Failed,
        updated_at: request.last_failed_at,
        proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
        target: None,
        failure_count: request.failure_count,
        last_error: if request.last_error.is_empty() {
            None
        } else {
            Some(request.last_error)
        },
        pending: !dead_letter,
        dead_letter,
    }
}

#[derive(Debug)]
struct SharedChannelSyncInventoryMapPage<T> {
    items: Vec<(String, T)>,
    next_key: Option<String>,
}

fn shared_channel_sync_inventory_map_page<T: Clone>(
    items_by_key: &BTreeMap<String, T>,
    limit: usize,
    after_request_key: Option<&str>,
) -> SharedChannelSyncInventoryMapPage<T> {
    let mut items: Vec<(String, T)> = Vec::with_capacity(limit.min(items_by_key.len()));
    let mut next_key = None;
    for (request_key, item) in items_by_key {
        if after_request_key.is_some_and(|cursor_key| request_key.as_str() <= cursor_key) {
            continue;
        }
        if items.len() == limit {
            next_key = items.last().map(|(request_key, _)| request_key.clone());
            break;
        }
        items.push((request_key.clone(), item.clone()));
    }
    SharedChannelSyncInventoryMapPage { items, next_key }
}

fn shared_channel_sync_inventory_cursor_for(request_key: &str) -> String {
    let cursor = SharedChannelSyncInventoryCursor {
        v: SHARED_CHANNEL_SYNC_INVENTORY_CURSOR_VERSION,
        request_key: request_key.to_owned(),
    };
    let payload = serde_json::to_value(&cursor)
        .expect("shared-channel sync inventory cursor should serialize into json");
    let secret = resolve_friend_request_cursor_signing_secret();
    encode_signed_cursor_payload(&payload, secret.as_str())
        .expect("shared-channel sync inventory cursor should encode into signed compact token")
}

fn parse_shared_channel_sync_inventory_cursor(
    cursor: &str,
) -> Result<SharedChannelSyncInventoryCursor, ControlPlaneError> {
    let payload = decode_signed_friend_request_cursor_payload(cursor)?;
    let cursor: SharedChannelSyncInventoryCursor =
        serde_json::from_value(payload).map_err(|_| {
            ControlPlaneError::invalid(
                "cursor_invalid",
                "shared-channel sync inventory cursor payload is not valid",
            )
        })?;
    if cursor.v != SHARED_CHANNEL_SYNC_INVENTORY_CURSOR_VERSION {
        return Err(ControlPlaneError::invalid(
            "cursor_invalid",
            format!(
                "shared-channel sync inventory cursor version {} is not supported",
                cursor.v
            ),
        ));
    }
    validate_payload_size(
        "cursor.requestKey",
        cursor.request_key.as_str(),
        CONTROL_PLANE_MAX_REQUEST_KEY_BYTES,
    )?;
    Ok(cursor)
}

fn parse_shared_channel_sync_inventory_page_spec(
    query: &SharedChannelSyncInventoryQuery,
) -> Result<SharedChannelSyncInventoryPageSpec, ControlPlaneError> {
    let limit = query
        .limit
        .unwrap_or(SHARED_CHANNEL_SYNC_INVENTORY_DEFAULT_LIMIT);
    if limit == 0 || limit > SHARED_CHANNEL_SYNC_INVENTORY_MAX_LIMIT {
        return Err(ControlPlaneError::invalid(
            "limit_invalid",
            format!("limit must be between 1 and {SHARED_CHANNEL_SYNC_INVENTORY_MAX_LIMIT}"),
        ));
    }
    let cursor = if let Some(cursor) = query.cursor.as_deref() {
        validate_payload_size(
            "cursor",
            cursor,
            SHARED_CHANNEL_SYNC_INVENTORY_MAX_CURSOR_BYTES,
        )?;
        Some(parse_shared_channel_sync_inventory_cursor(cursor)?)
    } else {
        None
    };
    Ok(SharedChannelSyncInventoryPageSpec { limit, cursor })
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

fn openapi_string_schema() -> JsonValue {
    serde_json::json!({
        "type": "string"
    })
}

fn openapi_nullable_string_schema() -> JsonValue {
    serde_json::json!({
        "type": "string",
        "nullable": true
    })
}

fn openapi_integer_schema() -> JsonValue {
    serde_json::json!({
        "type": "integer",
        "format": "int64"
    })
}

fn openapi_nullable_integer_schema() -> JsonValue {
    serde_json::json!({
        "type": "integer",
        "format": "int64",
        "nullable": true
    })
}

fn openapi_boolean_schema() -> JsonValue {
    serde_json::json!({
        "type": "boolean"
    })
}

fn openapi_string_array_schema() -> JsonValue {
    serde_json::json!({
        "type": "array",
        "items": openapi_string_schema()
    })
}

fn openapi_array_schema(items: JsonValue) -> JsonValue {
    serde_json::json!({
        "type": "array",
        "items": items
    })
}

fn openapi_object_schema(
    required: &[&str],
    properties: Vec<(&str, JsonValue)>,
    additional_properties: bool,
) -> JsonValue {
    let mut property_map = JsonMap::new();
    for (name, schema) in properties {
        property_map.insert(name.to_owned(), schema);
    }

    let mut schema = JsonMap::new();
    schema.insert("type".to_owned(), serde_json::json!("object"));
    schema.insert("properties".to_owned(), JsonValue::Object(property_map));
    schema.insert(
        "additionalProperties".to_owned(),
        JsonValue::Bool(additional_properties),
    );
    if !required.is_empty() {
        schema.insert("required".to_owned(), serde_json::json!(required));
    }

    JsonValue::Object(schema)
}

fn openapi_describe(mut schema: JsonValue, description: &str) -> JsonValue {
    if !description.is_empty()
        && let JsonValue::Object(object) = &mut schema
    {
        object.insert(
            "description".to_owned(),
            JsonValue::String(description.to_owned()),
        );
    }
    schema
}

fn openapi_generic_object_schema(description: &str) -> JsonValue {
    openapi_describe(openapi_object_schema(&[], Vec::new(), true), description)
}

fn openapi_component_ref(name: &str) -> JsonValue {
    serde_json::json!({
        "$ref": format!("#/components/schemas/{name}")
    })
}

fn insert_openapi_schema(schemas: &mut JsonMap<String, JsonValue>, name: &str, schema: JsonValue) {
    schemas.insert(name.to_owned(), schema);
}

fn openapi_json_response(description: &str, schema_name: &str) -> JsonValue {
    serde_json::json!({
        "description": description,
        "content": {
            "application/json": {
                "schema": openapi_component_ref(schema_name)
            }
        }
    })
}

fn openapi_problem_response(description: &str, schema_name: &str) -> JsonValue {
    serde_json::json!({
        "description": description,
        "content": {
            "application/problem+json": {
                "schema": openapi_component_ref(schema_name)
            }
        }
    })
}

fn openapi_standard_responses(success_schema_name: &str) -> JsonValue {
    let mut responses = JsonMap::new();
    responses.insert(
        "200".to_owned(),
        openapi_json_response("Successful response.", success_schema_name),
    );

    for (status, description) in [
        ("400", "Invalid request."),
        ("401", "Authentication required."),
        ("403", "Permission denied."),
        ("404", "Resource not found."),
        ("409", "Request conflicts with current control-plane state."),
        ("503", "Control-plane dependency is unavailable."),
    ] {
        responses.insert(
            status.to_owned(),
            openapi_problem_response(description, "ControlPlaneErrorResponse"),
        );
    }

    JsonValue::Object(responses)
}

fn openapi_request_body(schema_name: &str, description: &str) -> JsonValue {
    serde_json::json!({
        "required": true,
        "description": description,
        "content": {
            "application/json": {
                "schema": openapi_component_ref(schema_name)
            }
        }
    })
}

fn openapi_query_parameter(
    name: &str,
    required: bool,
    schema: JsonValue,
    description: &str,
) -> JsonValue {
    serde_json::json!({
        "name": name,
        "in": "query",
        "required": required,
        "description": description,
        "schema": schema
    })
}

fn openapi_path_parameter(name: &str, description: &str) -> JsonValue {
    serde_json::json!({
        "name": name,
        "in": "path",
        "required": true,
        "description": description,
        "schema": openapi_string_schema()
    })
}

fn openapi_operation(
    summary: &str,
    operation_id: &str,
    tag: &str,
    parameters: Vec<JsonValue>,
    request_body: Option<JsonValue>,
    response_schema_name: &str,
    secure: bool,
) -> JsonValue {
    let mut operation = JsonMap::new();
    operation.insert("summary".to_owned(), JsonValue::String(summary.to_owned()));
    operation.insert(
        "operationId".to_owned(),
        JsonValue::String(operation_id.to_owned()),
    );
    operation.insert("tags".to_owned(), serde_json::json!([tag]));
    operation.insert(
        "responses".to_owned(),
        openapi_standard_responses(response_schema_name),
    );
    if secure {
        operation.insert(
            "security".to_owned(),
            serde_json::json!([{ "AuthToken": [], "AccessToken": [] }]),
        );
    } else {
        operation.insert("security".to_owned(), serde_json::json!([]));
    }
    if !parameters.is_empty() {
        operation.insert("parameters".to_owned(), JsonValue::Array(parameters));
    }
    if let Some(request_body) = request_body {
        operation.insert("requestBody".to_owned(), request_body);
    }

    JsonValue::Object(operation)
}

fn control_plane_openapi_components() -> JsonValue {
    let mut schemas = JsonMap::new();

    insert_openapi_schema(
        &mut schemas,
        "HealthResponse",
        openapi_object_schema(
            &["status", "service"],
            vec![
                ("status", openapi_string_schema()),
                ("service", openapi_string_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ControlPlaneErrorResponse",
        openapi_object_schema(
            &["type", "title", "status"],
            vec![
                (
                    "type",
                    serde_json::json!({
                        "type": "string",
                        "format": "uri-reference"
                    }),
                ),
                ("title", openapi_string_schema()),
                (
                    "status",
                    serde_json::json!({
                        "type": "integer",
                        "minimum": 100,
                        "maximum": 599
                    }),
                ),
                ("detail", openapi_string_schema()),
                ("instance", openapi_string_schema()),
                ("code", openapi_string_schema()),
                ("message", openapi_string_schema()),
                ("errorStatus", openapi_string_schema()),
                (
                    "details",
                    serde_json::json!({
                        "type": "object",
                        "nullable": true,
                        "additionalProperties": true
                    }),
                ),
            ],
            true,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ProtocolSchemaResponse",
        openapi_object_schema(
            &[
                "schema",
                "kind",
                "stage",
                "bindingProtocols",
                "requiredCapabilities",
                "supportedConsumers",
            ],
            vec![
                ("schema", openapi_string_schema()),
                ("kind", openapi_string_schema()),
                ("stage", openapi_string_schema()),
                ("bindingProtocols", openapi_string_array_schema()),
                ("requiredCapabilities", openapi_string_array_schema()),
                ("supportedConsumers", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ClientCompatibilityResponse",
        openapi_object_schema(
            &[
                "clientType",
                "minimumProtocolVersion",
                "supportedBindings",
                "supportedCodecs",
                "supportedCapabilities",
                "blockedExperimentalCapabilities",
            ],
            vec![
                ("clientType", openapi_string_schema()),
                ("minimumProtocolVersion", openapi_string_schema()),
                ("supportedBindings", openapi_string_array_schema()),
                ("supportedCodecs", openapi_string_array_schema()),
                ("supportedCapabilities", openapi_string_array_schema()),
                (
                    "blockedExperimentalCapabilities",
                    openapi_string_array_schema(),
                ),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ProtocolRegistryResponse",
        openapi_object_schema(
            &[
                "protocolVersion",
                "bindings",
                "codecs",
                "schemas",
                "compatibilityMatrix",
            ],
            vec![
                ("protocolVersion", openapi_string_schema()),
                ("bindings", openapi_string_array_schema()),
                ("codecs", openapi_string_array_schema()),
                (
                    "schemas",
                    openapi_array_schema(openapi_component_ref("ProtocolSchemaResponse")),
                ),
                (
                    "compatibilityMatrix",
                    openapi_array_schema(openapi_component_ref("ClientCompatibilityResponse")),
                ),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "CapabilityProfileResponse",
        openapi_object_schema(
            &[
                "profileId",
                "releaseChannel",
                "enabledCapabilities",
                "experimentalCapabilities",
            ],
            vec![
                ("profileId", openapi_string_schema()),
                ("releaseChannel", openapi_string_schema()),
                ("enabledCapabilities", openapi_string_array_schema()),
                ("experimentalCapabilities", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "QuotaProfileResponse",
        openapi_object_schema(
            &[
                "profileId",
                "maxConcurrentSessionsPerTenant",
                "maxSubscriptionsPerSession",
                "maxInflightMessages",
                "maxPayloadBytes",
            ],
            vec![
                ("profileId", openapi_string_schema()),
                ("maxConcurrentSessionsPerTenant", openapi_integer_schema()),
                ("maxSubscriptionsPerSession", openapi_integer_schema()),
                ("maxInflightMessages", openapi_integer_schema()),
                ("maxPayloadBytes", openapi_integer_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "RolloutPolicyResponse",
        openapi_object_schema(
            &[
                "policyId",
                "releaseChannel",
                "trafficPercent",
                "cellSelector",
                "regionSelector",
                "operatorOverride",
                "tenantAllowlist",
            ],
            vec![
                ("policyId", openapi_string_schema()),
                ("releaseChannel", openapi_string_schema()),
                ("trafficPercent", openapi_integer_schema()),
                ("cellSelector", openapi_string_schema()),
                ("regionSelector", openapi_string_schema()),
                ("operatorOverride", openapi_boolean_schema()),
                ("tenantAllowlist", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "KillSwitchResponse",
        openapi_object_schema(
            &[
                "ruleId",
                "active",
                "reason",
                "disabledCapabilities",
                "disabledBindings",
                "disabledCodecs",
            ],
            vec![
                ("ruleId", openapi_string_schema()),
                ("active", openapi_boolean_schema()),
                ("reason", openapi_string_schema()),
                ("disabledCapabilities", openapi_string_array_schema()),
                ("disabledBindings", openapi_string_array_schema()),
                ("disabledCodecs", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "EffectiveProtocolSnapshotResponse",
        openapi_object_schema(
            &[
                "protocolVersion",
                "releaseChannel",
                "enabledCapabilities",
                "allowedBindings",
                "allowedCodecs",
                "quotaProfileId",
                "killSwitchActive",
                "precedence",
            ],
            vec![
                ("protocolVersion", openapi_string_schema()),
                ("releaseChannel", openapi_string_schema()),
                ("enabledCapabilities", openapi_string_array_schema()),
                ("allowedBindings", openapi_string_array_schema()),
                ("allowedCodecs", openapi_string_array_schema()),
                ("quotaProfileId", openapi_string_schema()),
                ("killSwitchActive", openapi_boolean_schema()),
                ("precedence", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "BusinessPolicyVocabularyResponse",
        openapi_object_schema(
            &[
                "policyVersionField",
                "capabilityFlagsField",
                "historyVisibilityField",
                "historyVisibilityModes",
                "retentionPolicyRefField",
                "retentionPolicyScopes",
            ],
            vec![
                ("policyVersionField", openapi_string_schema()),
                ("capabilityFlagsField", openapi_string_schema()),
                ("historyVisibilityField", openapi_string_schema()),
                ("historyVisibilityModes", openapi_string_array_schema()),
                ("retentionPolicyRefField", openapi_string_schema()),
                ("retentionPolicyScopes", openapi_string_array_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "SdkCompatibilityBaselineResponse",
        openapi_object_schema(
            &[
                "imSdkFamily",
                "appSdkFamily",
                "backendSdkFamily",
                "rtcSdkFamily",
                "matrixClientTypes",
                "protocolRegistryPath",
                "protocolGovernancePath",
            ],
            vec![
                ("imSdkFamily", openapi_string_schema()),
                ("appSdkFamily", openapi_string_schema()),
                ("backendSdkFamily", openapi_string_schema()),
                ("rtcSdkFamily", openapi_string_schema()),
                ("matrixClientTypes", openapi_string_array_schema()),
                ("protocolRegistryPath", openapi_string_schema()),
                ("protocolGovernancePath", openapi_string_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "ProtocolGovernanceResponse",
        openapi_object_schema(
            &[
                "capabilityProfile",
                "quotaProfile",
                "rolloutPolicy",
                "killSwitch",
                "effectiveSnapshot",
                "businessPolicyVocabulary",
                "sdkCompatibilityBaseline",
            ],
            vec![
                (
                    "capabilityProfile",
                    openapi_component_ref("CapabilityProfileResponse"),
                ),
                (
                    "quotaProfile",
                    openapi_component_ref("QuotaProfileResponse"),
                ),
                (
                    "rolloutPolicy",
                    openapi_component_ref("RolloutPolicyResponse"),
                ),
                ("killSwitch", openapi_component_ref("KillSwitchResponse")),
                (
                    "effectiveSnapshot",
                    openapi_component_ref("EffectiveProtocolSnapshotResponse"),
                ),
                (
                    "businessPolicyVocabulary",
                    openapi_component_ref("BusinessPolicyVocabularyResponse"),
                ),
                (
                    "sdkCompatibilityBaseline",
                    openapi_component_ref("SdkCompatibilityBaselineResponse"),
                ),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "RouteNodeLifecycle",
        openapi_object_schema(
            &["nodeId", "drainStatus", "rebalanceState", "ownedRouteCount"],
            vec![
                ("nodeId", openapi_string_schema()),
                ("drainStatus", openapi_string_schema()),
                ("rebalanceState", openapi_string_schema()),
                ("ownedRouteCount", openapi_integer_schema()),
            ],
            false,
        ),
    );
    insert_openapi_schema(
        &mut schemas,
        "RouteMigrationResult",
        openapi_object_schema(
            &[
                "sourceNodeId",
                "targetNodeId",
                "migratedRouteCount",
                "sourceDrainStatus",
                "sourceRebalanceState",
                "targetDrainStatus",
                "targetRebalanceState",
            ],
            vec![
                ("sourceNodeId", openapi_string_schema()),
                ("targetNodeId", openapi_string_schema()),
                ("migratedRouteCount", openapi_integer_schema()),
                ("sourceDrainStatus", openapi_string_schema()),
                ("sourceRebalanceState", openapi_string_schema()),
                ("targetDrainStatus", openapi_string_schema()),
                ("targetRebalanceState", openapi_string_schema()),
            ],
            false,
        ),
    );

    for (name, description) in [
        (
            "ProviderRegistrySnapshotResponse",
            "Provider registry snapshot for the current control-plane view.",
        ),
        (
            "ProviderBindingsResponse",
            "Effective provider bindings resolved for the current tenant scope.",
        ),
        (
            "ProviderBindingCommitResponse",
            "Provider binding mutation result after applying a control-plane policy change.",
        ),
        (
            "ProviderPolicyHistoryResponse",
            "Provider policy history snapshot for the current tenant scope.",
        ),
        (
            "ProviderPolicyDiffResponse",
            "Provider policy diff between two committed versions.",
        ),
        (
            "SocialFriendRequestCommitResponse",
            "Friend request write result plus persistence metadata.",
        ),
        (
            "SocialFriendRequestSnapshotResponse",
            "Friend request snapshot plus commit history.",
        ),
        (
            "SocialFriendshipCommitResponse",
            "Friendship write result plus persistence metadata.",
        ),
        (
            "SocialFriendshipSnapshotResponse",
            "Friendship snapshot plus commit history.",
        ),
        (
            "SocialUserBlockCommitResponse",
            "User block write result plus persistence metadata.",
        ),
        (
            "SocialUserBlockSnapshotResponse",
            "User block snapshot plus commit history.",
        ),
        (
            "SocialDirectChatCommitResponse",
            "Direct chat binding result plus persistence metadata.",
        ),
        (
            "SocialDirectChatSnapshotResponse",
            "Direct chat snapshot plus commit history.",
        ),
        (
            "SocialExternalConnectionCommitResponse",
            "External connection write result plus persistence metadata.",
        ),
        (
            "SocialExternalConnectionSnapshotResponse",
            "External connection snapshot plus commit history.",
        ),
        (
            "SocialExternalMemberLinkCommitResponse",
            "External member link write result plus persistence metadata.",
        ),
        (
            "SocialExternalMemberLinkSnapshotResponse",
            "External member link snapshot plus commit history.",
        ),
        (
            "SocialSharedChannelPolicyCommitResponse",
            "Shared-channel policy write result plus persistence metadata.",
        ),
        (
            "SocialSharedChannelPolicySnapshotResponse",
            "Shared-channel policy snapshot plus commit history.",
        ),
        (
            "SocialRuntimeRepairResponse",
            "Derived social runtime repair report.",
        ),
        (
            "SocialSharedChannelSyncDeadLetterInventoryResponse",
            "Dead-letter shared-channel sync queue snapshot.",
        ),
        (
            "SocialSharedChannelSyncPendingInventoryResponse",
            "Pending shared-channel sync queue snapshot.",
        ),
        (
            "SocialSharedChannelSyncDeliveredInventoryResponse",
            "Delivered shared-channel sync ledger snapshot.",
        ),
        (
            "SocialSharedChannelSyncDeliveryStateInventoryResponse",
            "Merged shared-channel sync delivery-state inventory snapshot.",
        ),
        (
            "SocialSharedChannelSyncRepairResponse",
            "Repair result for shared-channel sync backlog processing.",
        ),
        (
            "SocialSharedChannelSyncDeadLetterRequeueResponse",
            "Bulk requeue result for dead-letter shared-channel sync entries.",
        ),
        (
            "SocialSharedChannelSyncPendingClaimResponse",
            "Targeted claim result for pending shared-channel sync entries.",
        ),
        (
            "SocialSharedChannelSyncPendingReleaseResponse",
            "Targeted release result for pending shared-channel sync entries.",
        ),
        (
            "SocialSharedChannelSyncPendingTakeoverResponse",
            "Targeted takeover result for pending shared-channel sync entries.",
        ),
        (
            "SocialSharedChannelSyncPendingStaleReclaimResponse",
            "Automatic stale reclaim result for pending shared-channel sync entries.",
        ),
        (
            "SocialSharedChannelSyncDeadLetterTargetedRequeueResponse",
            "Targeted requeue result for selected dead-letter shared-channel sync entries.",
        ),
        (
            "SocialSharedChannelSyncTargetedRepublishResponse",
            "Targeted republish result for selected shared-channel sync entries.",
        ),
    ] {
        insert_openapi_schema(
            &mut schemas,
            name,
            openapi_generic_object_schema(description),
        );
    }

    for (name, schema) in [
        (
            "UpsertProviderBindingPolicyRequest",
            openapi_object_schema(
                &["domain", "pluginId"],
                vec![
                    ("tenantId", openapi_nullable_string_schema()),
                    ("domain", openapi_string_schema()),
                    ("pluginId", openapi_string_schema()),
                    ("expectedBaseVersion", openapi_nullable_integer_schema()),
                ],
                false,
            ),
        ),
        (
            "ProviderPolicyRollbackRequest",
            openapi_object_schema(
                &["targetVersion"],
                vec![("targetVersion", openapi_integer_schema())],
                false,
            ),
        ),
        (
            "SubmitFriendRequestRequest",
            openapi_object_schema(
                &[
                    "requestId",
                    "eventId",
                    "requesterUserId",
                    "targetUserId",
                    "requestedAt",
                ],
                vec![
                    ("requestId", openapi_string_schema()),
                    ("eventId", openapi_string_schema()),
                    ("requesterUserId", openapi_string_schema()),
                    ("targetUserId", openapi_string_schema()),
                    ("requestMessage", openapi_nullable_string_schema()),
                    ("requestedAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "AcceptFriendRequestRequest",
            openapi_object_schema(
                &["eventId", "acceptedByUserId", "acceptedAt"],
                vec![
                    ("eventId", openapi_string_schema()),
                    ("acceptedByUserId", openapi_string_schema()),
                    ("acceptedAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "DeclineFriendRequestRequest",
            openapi_object_schema(
                &["eventId", "declinedByUserId", "declinedAt"],
                vec![
                    ("eventId", openapi_string_schema()),
                    ("declinedByUserId", openapi_string_schema()),
                    ("declinedAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "CancelFriendRequestRequest",
            openapi_object_schema(
                &["eventId", "canceledByUserId", "canceledAt"],
                vec![
                    ("eventId", openapi_string_schema()),
                    ("canceledByUserId", openapi_string_schema()),
                    ("canceledAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "ActivateFriendshipRequest",
            openapi_object_schema(
                &[
                    "friendshipId",
                    "eventId",
                    "initiatorUserId",
                    "peerUserId",
                    "establishedAt",
                ],
                vec![
                    ("friendshipId", openapi_string_schema()),
                    ("eventId", openapi_string_schema()),
                    ("initiatorUserId", openapi_string_schema()),
                    ("peerUserId", openapi_string_schema()),
                    ("directChatId", openapi_nullable_string_schema()),
                    ("establishedAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "RemoveFriendshipRequest",
            openapi_object_schema(
                &["eventId", "removedByUserId", "removedAt"],
                vec![
                    ("eventId", openapi_string_schema()),
                    ("removedByUserId", openapi_string_schema()),
                    ("removedAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "BlockUserRequest",
            openapi_object_schema(
                &[
                    "blockId",
                    "eventId",
                    "blockerUserId",
                    "blockedUserId",
                    "scope",
                    "effectiveAt",
                ],
                vec![
                    ("blockId", openapi_string_schema()),
                    ("eventId", openapi_string_schema()),
                    ("blockerUserId", openapi_string_schema()),
                    ("blockedUserId", openapi_string_schema()),
                    ("scope", openapi_string_schema()),
                    ("directChatId", openapi_nullable_string_schema()),
                    ("expiresAt", openapi_nullable_string_schema()),
                    ("effectiveAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "BindDirectChatRequest",
            openapi_object_schema(
                &[
                    "directChatId",
                    "eventId",
                    "leftActorId",
                    "rightActorId",
                    "conversationId",
                    "boundAt",
                ],
                vec![
                    ("directChatId", openapi_string_schema()),
                    ("eventId", openapi_string_schema()),
                    ("leftActorId", openapi_string_schema()),
                    ("rightActorId", openapi_string_schema()),
                    ("conversationId", openapi_string_schema()),
                    ("boundAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "EstablishExternalConnectionRequest",
            openapi_object_schema(
                &[
                    "connectionId",
                    "eventId",
                    "externalTenantId",
                    "connectionKind",
                    "establishedAt",
                ],
                vec![
                    ("connectionId", openapi_string_schema()),
                    ("eventId", openapi_string_schema()),
                    ("externalTenantId", openapi_string_schema()),
                    ("externalOrgName", openapi_nullable_string_schema()),
                    ("connectionKind", openapi_string_schema()),
                    ("establishedAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "BindExternalMemberLinkRequest",
            openapi_object_schema(
                &[
                    "linkId",
                    "eventId",
                    "connectionId",
                    "localActorId",
                    "localActorKind",
                    "externalMemberId",
                    "linkedAt",
                ],
                vec![
                    ("linkId", openapi_string_schema()),
                    ("eventId", openapi_string_schema()),
                    ("connectionId", openapi_string_schema()),
                    ("localActorId", openapi_string_schema()),
                    ("localActorKind", openapi_string_schema()),
                    ("externalMemberId", openapi_string_schema()),
                    ("externalDisplayName", openapi_nullable_string_schema()),
                    ("linkedAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "ApplySharedChannelPolicyRequest",
            openapi_object_schema(
                &[
                    "policyId",
                    "eventId",
                    "connectionId",
                    "channelId",
                    "policyVersion",
                    "historyVisibility",
                    "appliedAt",
                ],
                vec![
                    ("policyId", openapi_string_schema()),
                    ("eventId", openapi_string_schema()),
                    ("connectionId", openapi_string_schema()),
                    ("channelId", openapi_string_schema()),
                    ("conversationId", openapi_nullable_string_schema()),
                    ("policyVersion", openapi_integer_schema()),
                    ("historyVisibility", openapi_string_schema()),
                    ("appliedAt", openapi_string_schema()),
                ],
                false,
            ),
        ),
        (
            "SocialSharedChannelSyncDeadLetterTargetedRequeueRequest",
            openapi_object_schema(
                &["requestKeys"],
                vec![("requestKeys", openapi_string_array_schema())],
                false,
            ),
        ),
        (
            "SocialSharedChannelSyncPendingTargetedClaimRequest",
            openapi_object_schema(
                &["requestKeys"],
                vec![("requestKeys", openapi_string_array_schema())],
                false,
            ),
        ),
        (
            "SocialSharedChannelSyncPendingTargetedReleaseRequest",
            openapi_object_schema(
                &["requestKeys"],
                vec![("requestKeys", openapi_string_array_schema())],
                false,
            ),
        ),
        (
            "SocialSharedChannelSyncPendingTargetedTakeoverRequest",
            openapi_object_schema(
                &["requestKeys"],
                vec![
                    ("requestKeys", openapi_string_array_schema()),
                    ("allowLegacyUntracked", openapi_boolean_schema()),
                ],
                false,
            ),
        ),
        (
            "SocialSharedChannelSyncTargetedRepublishRequest",
            openapi_object_schema(
                &["requestKeys"],
                vec![("requestKeys", openapi_string_array_schema())],
                false,
            ),
        ),
        (
            "MigrateRoutesRequest",
            openapi_object_schema(
                &["targetNodeId"],
                vec![("targetNodeId", openapi_string_schema())],
                false,
            ),
        ),
    ] {
        insert_openapi_schema(&mut schemas, name, schema);
    }

    serde_json::json!({
        "securitySchemes": {
            "AuthToken": {
                "type": "http",
                "scheme": "bearer",
                "bearerFormat": "JWT"
            },
            "AccessToken": {
                "type": "apiKey",
                "in": "header",
                "name": "Access-Token"
            }
        },
        "schemas": JsonValue::Object(schemas)
    })
}

fn control_plane_openapi_paths() -> JsonValue {
    let mut paths = JsonMap::new();

    paths.insert(
        "/healthz".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Check control-plane process health.",
                "getHealthz",
                "meta",
                Vec::new(),
                None,
                "HealthResponse",
                false
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/protocol_registry".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read the control-plane protocol registry snapshot.",
                "getProtocolRegistry",
                "protocol",
                Vec::new(),
                None,
                "ProtocolRegistryResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/protocol_governance".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read the control-plane protocol governance snapshot.",
                "getProtocolGovernance",
                "protocol",
                Vec::new(),
                None,
                "ProtocolGovernanceResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_registry".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read the provider registry snapshot.",
                "getProviderRegistry",
                "providers",
                Vec::new(),
                None,
                "ProviderRegistrySnapshotResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_bindings".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read effective provider bindings.",
                "getProviderBindings",
                "providers",
                vec![
                    openapi_query_parameter(
                        "tenantId",
                        false,
                        openapi_string_schema(),
                        "Optional tenant scope for effective provider bindings."
                    )
                ],
                None,
                "ProviderBindingsResponse",
                true
            ),
            "post": openapi_operation(
                "Upsert a provider binding policy.",
                "upsertProviderBindingPolicy",
                "providers",
                Vec::new(),
                Some(openapi_request_body(
                    "UpsertProviderBindingPolicyRequest",
                    "Provider binding mutation payload."
                )),
                "ProviderBindingCommitResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read provider policy history.",
                "getProviderPolicyHistory",
                "providers",
                Vec::new(),
                None,
                "ProviderPolicyHistoryResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies/diff".to_owned(),
        serde_json::json!({
            "get": openapi_operation(
                "Read provider policy diff between two versions.",
                "getProviderPolicyDiff",
                "providers",
                vec![
                    openapi_query_parameter(
                        "fromVersion",
                        true,
                        openapi_integer_schema(),
                        "Base provider policy version."
                    ),
                    openapi_query_parameter(
                        "toVersion",
                        true,
                        openapi_integer_schema(),
                        "Target provider policy version."
                    )
                ],
                None,
                "ProviderPolicyDiffResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies/preview".to_owned(),
        serde_json::json!({
            "post": openapi_operation(
                "Preview the effective provider policy result before commit.",
                "previewProviderPolicy",
                "providers",
                Vec::new(),
                Some(openapi_request_body(
                    "UpsertProviderBindingPolicyRequest",
                    "Provider binding preview payload."
                )),
                "ProviderBindingCommitResponse",
                true
            )
        }),
    );
    paths.insert(
        "/backend/v3/api/control/provider_policies/rollback".to_owned(),
        serde_json::json!({
            "post": openapi_operation(
                "Rollback provider policy history to a target version.",
                "rollbackProviderPolicy",
                "providers",
                Vec::new(),
                Some(openapi_request_body(
                    "ProviderPolicyRollbackRequest",
                    "Provider policy rollback payload."
                )),
                "ProviderBindingCommitResponse",
                true
            )
        }),
    );

    for (path, summary, operation_id, request_schema, response_schema) in [
        (
            "/backend/v3/api/control/social/friend_requests",
            "Submit a friend request event.",
            "submitFriendRequest",
            "SubmitFriendRequestRequest",
            "SocialFriendRequestCommitResponse",
        ),
        (
            "/backend/v3/api/control/social/friendships",
            "Activate a friendship event.",
            "activateFriendship",
            "ActivateFriendshipRequest",
            "SocialFriendshipCommitResponse",
        ),
        (
            "/backend/v3/api/control/social/user_blocks",
            "Block a user in the social graph.",
            "blockUser",
            "BlockUserRequest",
            "SocialUserBlockCommitResponse",
        ),
        (
            "/backend/v3/api/control/social/direct_chats/bindings",
            "Bind a direct chat to a conversation.",
            "bindDirectChat",
            "BindDirectChatRequest",
            "SocialDirectChatCommitResponse",
        ),
        (
            "/backend/v3/api/control/social/external_connections",
            "Establish an external collaboration connection.",
            "establishExternalConnection",
            "EstablishExternalConnectionRequest",
            "SocialExternalConnectionCommitResponse",
        ),
        (
            "/backend/v3/api/control/social/external_member_links",
            "Bind an external member link.",
            "bindExternalMemberLink",
            "BindExternalMemberLinkRequest",
            "SocialExternalMemberLinkCommitResponse",
        ),
        (
            "/backend/v3/api/control/social/shared_channel_policies",
            "Apply a shared-channel policy.",
            "applySharedChannelPolicy",
            "ApplySharedChannelPolicyRequest",
            "SocialSharedChannelPolicyCommitResponse",
        ),
    ] {
        paths.insert(
            path.to_owned(),
            serde_json::json!({
                "post": openapi_operation(
                    summary,
                    operation_id,
                    "social",
                    Vec::new(),
                    Some(openapi_request_body(request_schema, "Social mutation payload.")),
                    response_schema,
                    true
                )
            }),
        );
    }

    for (path, param_name, summary, operation_id, response_schema) in [
        (
            "/backend/v3/api/control/social/friend_requests/{request_id}",
            "request_id",
            "Read a friend request snapshot.",
            "getFriendRequestSnapshot",
            "SocialFriendRequestSnapshotResponse",
        ),
        (
            "/backend/v3/api/control/social/friendships/{friendship_id}",
            "friendship_id",
            "Read a friendship snapshot.",
            "getFriendshipSnapshot",
            "SocialFriendshipSnapshotResponse",
        ),
        (
            "/backend/v3/api/control/social/user_blocks/{block_id}",
            "block_id",
            "Read a user block snapshot.",
            "getUserBlockSnapshot",
            "SocialUserBlockSnapshotResponse",
        ),
        (
            "/backend/v3/api/control/social/direct_chats/{direct_chat_id}",
            "direct_chat_id",
            "Read a direct chat snapshot.",
            "getDirectChatSnapshot",
            "SocialDirectChatSnapshotResponse",
        ),
        (
            "/backend/v3/api/control/social/external_connections/{connection_id}",
            "connection_id",
            "Read an external connection snapshot.",
            "getExternalConnectionSnapshot",
            "SocialExternalConnectionSnapshotResponse",
        ),
        (
            "/backend/v3/api/control/social/external_member_links/{link_id}",
            "link_id",
            "Read an external member link snapshot.",
            "getExternalMemberLinkSnapshot",
            "SocialExternalMemberLinkSnapshotResponse",
        ),
        (
            "/backend/v3/api/control/social/shared_channel_policies/{policy_id}",
            "policy_id",
            "Read a shared-channel policy snapshot.",
            "getSharedChannelPolicySnapshot",
            "SocialSharedChannelPolicySnapshotResponse",
        ),
    ] {
        paths.insert(
            path.to_owned(),
            serde_json::json!({
                "get": openapi_operation(
                    summary,
                    operation_id,
                    "social",
                    vec![openapi_path_parameter(param_name, "Aggregate identifier.")],
                    None,
                    response_schema,
                    true
                )
            }),
        );
    }

    for (path, param_name, summary, operation_id, request_schema, response_schema) in [
        (
            "/backend/v3/api/control/social/friend_requests/{request_id}/accept",
            "request_id",
            "Accept a friend request.",
            "acceptFriendRequest",
            "AcceptFriendRequestRequest",
            "SocialFriendRequestCommitResponse",
        ),
        (
            "/backend/v3/api/control/social/friend_requests/{request_id}/decline",
            "request_id",
            "Decline a friend request.",
            "declineFriendRequest",
            "DeclineFriendRequestRequest",
            "SocialFriendRequestCommitResponse",
        ),
        (
            "/backend/v3/api/control/social/friend_requests/{request_id}/cancel",
            "request_id",
            "Cancel a friend request.",
            "cancelFriendRequest",
            "CancelFriendRequestRequest",
            "SocialFriendRequestCommitResponse",
        ),
        (
            "/backend/v3/api/control/social/friendships/{friendship_id}/remove",
            "friendship_id",
            "Remove a friendship.",
            "removeFriendship",
            "RemoveFriendshipRequest",
            "SocialFriendshipCommitResponse",
        ),
    ] {
        paths.insert(
            path.to_owned(),
            serde_json::json!({
                "post": openapi_operation(
                    summary,
                    operation_id,
                    "social",
                    vec![openapi_path_parameter(param_name, "Aggregate identifier.")],
                    Some(openapi_request_body(request_schema, "Social action payload.")),
                    response_schema,
                    true
                )
            }),
        );
    }

    for (path, summary, operation_id, response_schema) in [
        (
            "/backend/v3/api/control/social/runtime/repair_derived_snapshot",
            "Repair the persisted social runtime derived snapshot.",
            "repairSocialRuntimeSnapshot",
            "SocialRuntimeRepairResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/reclaim_stale_pending_shared_channel_sync",
            "Reclaim stale shared-channel sync pending ownership.",
            "reclaimStalePendingSharedChannelSync",
            "SocialSharedChannelSyncPendingStaleReclaimResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/repair_shared_channel_sync",
            "Repair shared-channel sync backlog state.",
            "repairSharedChannelSync",
            "SocialSharedChannelSyncRepairResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync",
            "Requeue all dead-letter shared-channel sync entries.",
            "requeueDeadLetterSharedChannelSync",
            "SocialSharedChannelSyncDeadLetterRequeueResponse",
        ),
    ] {
        paths.insert(
            path.to_owned(),
            serde_json::json!({
                "post": openapi_operation(
                    summary,
                    operation_id,
                    "social-runtime",
                    Vec::new(),
                    None,
                    response_schema,
                    true
                )
            }),
        );
    }

    for (path, summary, operation_id, response_schema) in [
        (
            "/backend/v3/api/control/social/runtime/dead_letter_shared_channel_sync",
            "Read the dead-letter shared-channel sync queue.",
            "getDeadLetterSharedChannelSyncInventory",
            "SocialSharedChannelSyncDeadLetterInventoryResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/pending_shared_channel_sync",
            "Read the pending shared-channel sync queue.",
            "getPendingSharedChannelSyncInventory",
            "SocialSharedChannelSyncPendingInventoryResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/delivered_shared_channel_sync",
            "Read the delivered shared-channel sync ledger.",
            "getDeliveredSharedChannelSyncInventory",
            "SocialSharedChannelSyncDeliveredInventoryResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync",
            "Read merged shared-channel sync delivery state.",
            "getSharedChannelSyncDeliveryStateInventory",
            "SocialSharedChannelSyncDeliveryStateInventoryResponse",
        ),
    ] {
        paths.insert(
            path.to_owned(),
            serde_json::json!({
                "get": openapi_operation(
                    summary,
                    operation_id,
                    "social-runtime",
                    Vec::new(),
                    None,
                    response_schema,
                    true
                )
            }),
        );
    }

    for (path, summary, operation_id, request_schema, response_schema) in [
        (
            "/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted",
            "Requeue selected dead-letter shared-channel sync entries.",
            "requeueDeadLetterSharedChannelSyncTargeted",
            "SocialSharedChannelSyncDeadLetterTargetedRequeueRequest",
            "SocialSharedChannelSyncDeadLetterTargetedRequeueResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/claim_pending_shared_channel_sync_targeted",
            "Claim selected pending shared-channel sync entries.",
            "claimPendingSharedChannelSyncTargeted",
            "SocialSharedChannelSyncPendingTargetedClaimRequest",
            "SocialSharedChannelSyncPendingClaimResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/release_pending_shared_channel_sync_targeted",
            "Release selected pending shared-channel sync entries.",
            "releasePendingSharedChannelSyncTargeted",
            "SocialSharedChannelSyncPendingTargetedReleaseRequest",
            "SocialSharedChannelSyncPendingReleaseResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/takeover_pending_shared_channel_sync_targeted",
            "Take over selected pending shared-channel sync entries.",
            "takeoverPendingSharedChannelSyncTargeted",
            "SocialSharedChannelSyncPendingTargetedTakeoverRequest",
            "SocialSharedChannelSyncPendingTakeoverResponse",
        ),
        (
            "/backend/v3/api/control/social/runtime/republish_pending_shared_channel_sync_targeted",
            "Republish selected pending shared-channel sync entries.",
            "republishPendingSharedChannelSyncTargeted",
            "SocialSharedChannelSyncTargetedRepublishRequest",
            "SocialSharedChannelSyncTargetedRepublishResponse",
        ),
    ] {
        paths.insert(
            path.to_owned(),
            serde_json::json!({
                "post": openapi_operation(
                    summary,
                    operation_id,
                    "social-runtime",
                    Vec::new(),
                    Some(openapi_request_body(request_schema, "Targeted request-key payload.")),
                    response_schema,
                    true
                )
            }),
        );
    }

    for (path, operation_id, summary) in [
        (
            "/backend/v3/api/control/nodes/{node_id}/drain",
            "drainNode",
            "Mark a realtime node as draining.",
        ),
        (
            "/backend/v3/api/control/nodes/{node_id}/activate",
            "activateNode",
            "Activate a realtime node and clear drain state.",
        ),
    ] {
        paths.insert(
            path.to_owned(),
            serde_json::json!({
                "post": openapi_operation(
                    summary,
                    operation_id,
                    "nodes",
                    vec![openapi_path_parameter("node_id", "Realtime node identifier.")],
                    None,
                    "RouteNodeLifecycle",
                    true
                )
            }),
        );
    }

    paths.insert(
        "/backend/v3/api/control/nodes/{node_id}/routes/migrate".to_owned(),
        serde_json::json!({
            "post": openapi_operation(
                "Migrate owned routes from the source node to the target node.",
                "migrateNodeRoutes",
                "nodes",
                vec![openapi_path_parameter("node_id", "Source realtime node identifier.")],
                Some(openapi_request_body(
                    "MigrateRoutesRequest",
                    "Route migration target payload."
                )),
                "RouteMigrationResult",
                true
            )
        }),
    );

    JsonValue::Object(paths)
}

fn control_plane_openapi_document() -> JsonValue {
    serde_json::json!({
        "openapi": "3.1.2",
        "info": {
            "title": "Control Plane API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Live OpenAPI contract for the control-plane control-plane runtime. This document is emitted by the running service and is intended to be captured into the admin SDK workspace before generation."
        },
        "servers": [
            {
                "url": "/"
            }
        ],
        "tags": [
            {
                "name": "meta",
                "description": "Service metadata and runtime health endpoints."
            },
            {
                "name": "protocol",
                "description": "Protocol registry and protocol governance surfaces."
            },
            {
                "name": "providers",
                "description": "Provider registry, binding, history, diff, preview, and rollback surfaces."
            },
            {
                "name": "social",
                "description": "Social graph and external collaboration control surfaces."
            },
            {
                "name": "social-runtime",
                "description": "Shared-channel sync backlog, repair, and delivery-state runtime surfaces."
            },
            {
                "name": "nodes",
                "description": "Realtime node lifecycle and route migration control surfaces."
            }
        ],
        "components": control_plane_openapi_components(),
        "paths": control_plane_openapi_paths()
    })
}

pub fn render_openapi_document() -> serde_json::Value {
    control_plane_openapi_document()
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
            Self::Memory(state) => {
                let mut loaded =
                    lock_social_state_mutex(state, "social-state-store.memory").clone();
                loaded.rebuild_social_indexes();
                Ok(loaded)
            }
            Self::File { file_path, io_lock } => {
                let _guard = lock_social_state_mutex(io_lock, "social-state-store.file-io");
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
                    return Err(format!(
                        "social state file {} is empty",
                        file_path.display()
                    ));
                }
                let mut loaded: SocialControlState =
                    serde_json::from_str(&content).map_err(|error| {
                        format!(
                            "failed to parse social state file {}: {error}",
                            file_path.display()
                        )
                    })?;
                loaded.rebuild_social_indexes();
                Ok(loaded)
            }
        }
    }

    fn save(&self, state: &SocialControlState) -> Result<(), String> {
        match self {
            Self::Memory(slot) => {
                *lock_social_state_mutex(slot, "social-state-store.memory") = state.clone();
                Ok(())
            }
            Self::File { file_path, io_lock } => {
                let _guard = lock_social_state_mutex(io_lock, "social-state-store.file-io");
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
                write_file_atomically(file_path.as_path(), payload.as_bytes(), "social state file")
            }
        }
    }
}

fn write_file_atomically(
    file_path: &StdPath,
    payload: &[u8],
    store_name: &str,
) -> Result<(), String> {
    let parent = file_path
        .parent()
        .ok_or_else(|| format!("{store_name} path has no parent: {}", file_path.display()))?;
    fs::create_dir_all(parent).map_err(|error| {
        format!(
            "failed to create {store_name} parent directory {}: {error}",
            parent.display()
        )
    })?;

    let temp_path = atomic_temp_path(file_path)?;
    let write_result = (|| {
        let mut temp_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(temp_path.as_path())
            .map_err(|error| {
                format!(
                    "failed to create {store_name} temp file {}: {error}",
                    temp_path.display()
                )
            })?;
        temp_file.write_all(payload).map_err(|error| {
            format!(
                "failed to write {store_name} temp file {}: {error}",
                temp_path.display()
            )
        })?;
        temp_file.sync_all().map_err(|error| {
            format!(
                "failed to sync {store_name} temp file {}: {error}",
                temp_path.display()
            )
        })?;
        drop(temp_file);
        replace_file_atomically(temp_path.as_path(), file_path).map_err(|error| {
            format!(
                "failed to atomically replace {store_name} {} from temp file {}: {error}",
                file_path.display(),
                temp_path.display()
            )
        })?;
        sync_parent_directory(parent, store_name)?;
        Ok(())
    })();

    if write_result.is_err() {
        let _ = fs::remove_file(temp_path.as_path());
    }
    write_result
}

fn atomic_temp_path(file_path: &StdPath) -> Result<PathBuf, String> {
    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("file path has no valid file name: {}", file_path.display()))?;
    let mut random = [0_u8; 8];
    fill_random(&mut random).map_err(|error| {
        format!(
            "failed to generate temporary file suffix for {}: {error}",
            file_path.display()
        )
    })?;
    let suffix = u64::from_le_bytes(random);
    Ok(file_path.with_file_name(format!(
        ".{file_name}.{}.{}.tmp",
        std::process::id(),
        suffix
    )))
}

#[cfg(windows)]
fn replace_file_atomically(temp_path: &StdPath, file_path: &StdPath) -> Result<(), String> {
    use std::os::windows::ffi::OsStrExt;

    const MOVEFILE_REPLACE_EXISTING: u32 = 0x0000_0001;
    const MOVEFILE_WRITE_THROUGH: u32 = 0x0000_0008;

    #[link(name = "Kernel32")]
    unsafe extern "system" {
        fn MoveFileExW(
            lp_existing_file_name: *const u16,
            lp_new_file_name: *const u16,
            dw_flags: u32,
        ) -> i32;
    }

    let existing = temp_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let new = file_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let replaced = unsafe {
        MoveFileExW(
            existing.as_ptr(),
            new.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if replaced == 0 {
        return Err(std::io::Error::last_os_error().to_string());
    }
    Ok(())
}

#[cfg(not(windows))]
fn replace_file_atomically(temp_path: &StdPath, file_path: &StdPath) -> Result<(), String> {
    fs::rename(temp_path, file_path).map_err(|error| error.to_string())
}

fn sync_parent_directory(parent: &StdPath, store_name: &str) -> Result<(), String> {
    #[cfg(unix)]
    {
        fs::File::open(parent)
            .and_then(|file| file.sync_all())
            .map_err(|error| {
                format!(
                    "failed to sync {store_name} parent directory {}: {error}",
                    parent.display()
                )
            })?;
    }
    let _ = (parent, store_name);
    Ok(())
}

fn lock_social_state_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovering poisoned {lock_name} lock");
            poisoned.into_inner()
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
    active_shared_channel_policy_records_for_connection(
        state,
        link.tenant_id.as_str(),
        link.connection_id.as_str(),
    )
    .into_iter()
    .filter_map(|record| {
        let policy = &record.shared_channel_policy;
        let conversation_id = non_empty_string(policy.conversation_id.as_deref())?;
        if policy.history_visibility != "shared" {
            return None;
        }

        Some(SharedChannelLinkedMemberSyncRequest {
            tenant_id: link.tenant_id.clone(),
            conversation_id,
            shared_channel_policy_id: policy.policy_id.clone(),
            external_connection_id: link.connection_id.clone(),
            local_actor_id: link.local_actor_id.clone(),
            local_actor_kind: link.local_actor_kind.clone(),
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

    active_external_member_link_records_for_connection(
        state,
        policy.tenant_id.as_str(),
        policy.connection_id.as_str(),
    )
    .into_iter()
    .map(|record| {
        let link = &record.external_member_link;
        SharedChannelLinkedMemberSyncRequest {
            tenant_id: policy.tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            shared_channel_policy_id: policy.policy_id.clone(),
            external_connection_id: policy.connection_id.clone(),
            local_actor_id: link.local_actor_id.clone(),
            local_actor_kind: link.local_actor_kind.clone(),
            external_member_id: link.external_member_id.clone(),
        }
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

fn shared_channel_sync_audit_aggregate_id(
    request: &SharedChannelLinkedMemberSyncRequest,
) -> String {
    let request_key = shared_channel_sync_request_key(request);
    let digest = Sha256::digest(request_key.as_bytes());
    format!("shared-channel-sync:{digest:x}")
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
        let authority_load = Self::load_social_state_for_authority(
            &state_store,
            "failed to load control-plane social state during runtime bootstrap",
        );
        Self {
            state_store,
            commit_journal,
            state: RwLock::new(authority_load.state),
            authority_replay_error: RwLock::new(authority_load.replay_error),
            journal_path: None,
            tx_marker_path: None,
            write_lock_path: None,
            snapshot_failpoint_path: snapshot_failpoint_path.map(Arc::new),
            shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool::new(false),
        }
    }

    fn recover_poisoned_social_runtime_lock<T>(poisoned: std::sync::PoisonError<T>) -> T {
        tracing::warn!(
            "control-plane social runtime lock was poisoned by a prior panic; continuing with inner state"
        );
        poisoned.into_inner()
    }

    fn from_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> Self {
        let state_dir = runtime_dir.as_ref().join("state");
        let journal_path = state_dir.join(SOCIAL_COMMIT_JOURNAL_FILE_NAME);
        let tx_marker_path = state_dir.join(SOCIAL_TRANSACTION_MARKER_FILE_NAME);
        let write_lock_path = state_dir.join(SOCIAL_WRITE_LOCK_FILE_NAME);
        let state_store = SocialStateStore::file(state_dir.join(SOCIAL_STATE_FILE_NAME));
        let commit_journal = Arc::new(FileCommitJournal::new(
            SOCIAL_COMMIT_PARTITION,
            journal_path.clone(),
        ));
        let authority_load = Self::load_state_with_journal_replay(
            &state_store,
            journal_path.as_path(),
            Some(tx_marker_path.as_path()),
        );
        Self {
            state_store,
            commit_journal,
            state: RwLock::new(authority_load.state),
            authority_replay_error: RwLock::new(authority_load.replay_error),
            journal_path: Some(Arc::new(journal_path)),
            tx_marker_path: Some(Arc::new(tx_marker_path)),
            write_lock_path: Some(Arc::new(write_lock_path)),
            snapshot_failpoint_path: Some(Arc::new(state_dir.join("social-failpoints.json"))),
            shared_channel_sync_stale_reclaim_scheduler_started: AtomicBool::new(false),
        }
    }

    fn replay_state_from_commit_journal(
        journal_path: &StdPath,
    ) -> Result<SocialControlState, String> {
        let mut replayed_state = SocialControlState::default();
        replayed_state.replay_commit_journal_file(journal_path)?;
        replayed_state.rebuild_social_indexes();
        Ok(replayed_state)
    }

    fn load_social_state_for_authority(
        state_store: &SocialStateStore,
        context: &str,
    ) -> SocialAuthorityLoad {
        match state_store.load() {
            Ok(state) => SocialAuthorityLoad {
                state,
                replay_error: None,
            },
            Err(error) => {
                let replay_error = format!("{context}: {error}");
                tracing::warn!(
                    "{replay_error}. control-plane social authority is unavailable until the snapshot is repaired"
                );
                SocialAuthorityLoad {
                    state: SocialControlState::default(),
                    replay_error: Some(replay_error),
                }
            }
        }
    }

    fn load_state_with_journal_replay(
        state_store: &SocialStateStore,
        journal_path: &StdPath,
        tx_marker_path: Option<&StdPath>,
    ) -> SocialAuthorityLoad {
        if journal_path.exists() {
            let snapshot_load = Self::load_social_state_for_authority(
                state_store,
                "failed to load control-plane social snapshot during journal replay bootstrap",
            );
            let snapshot_state = snapshot_load.state;
            let mut replayed_state = match Self::replay_state_from_commit_journal(journal_path) {
                Ok(state) => state,
                Err(error) => {
                    let replay_error = format!(
                        "failed to replay control-plane social commit journal {}: {error}",
                        journal_path.display()
                    );
                    tracing::warn!(
                        "{replay_error}. control-plane social authority is unavailable until the journal is repaired"
                    );
                    return SocialAuthorityLoad {
                        state: snapshot_state,
                        replay_error: Some(replay_error),
                    };
                }
            };
            replayed_state.merge_pending_shared_channel_sync_requests_from(&snapshot_state);
            replayed_state.merge_dead_letter_shared_channel_sync_requests_from(&snapshot_state);
            replayed_state.merge_delivered_shared_channel_sync_requests_from(&snapshot_state);
            replayed_state
                .merge_delivered_shared_channel_sync_delivery_proofs_from(&snapshot_state);
            replayed_state.merge_recent_shared_channel_sync_deliveries_from(&snapshot_state);
            if let Err(error) = state_store.save(&replayed_state) {
                tracing::warn!(
                    "failed to persist replayed control-plane social state {}: {error}. continuing with in-memory replayed state",
                    journal_path.display()
                );
            }
            if let Some(marker_path) = tx_marker_path
                && let Err(error) = clear_social_transaction_marker(marker_path)
            {
                tracing::warn!(
                    "failed to clear social transaction marker after journal replay {}: {error}",
                    marker_path.display()
                );
            }
            return SocialAuthorityLoad {
                state: replayed_state,
                replay_error: None,
            };
        }

        Self::load_social_state_for_authority(
            state_store,
            "failed to load control-plane social state without commit journal",
        )
    }

    fn start_shared_channel_sync_stale_reclaim_scheduler(
        self: &Arc<Self>,
        config: SharedChannelSyncStaleReclaimSchedulerConfig,
    ) {
        let config = config.with_normalized_values();
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
        match std::thread::Builder::new()
            .name("shared-sync-stale-reclaim".to_owned())
            .spawn(move || {
                loop {
                    std::thread::sleep(config.tick_sleep_duration());
                    if let Err(error) = runtime
                        .reclaim_stale_pending_shared_channel_sync_claims_if_any(
                            "failed to persist stale pending shared-channel sync reclaim from scheduler",
                        )
                    {
                        tracing::warn!(
                            "shared-channel sync stale reclaim scheduler tick failed: {error:?}"
                        );
                    }
                    if let Err(error) = runtime.prune_delivered_shared_channel_sync_backlog_if_any(
                        "failed to persist shared-channel sync delivered-ledger pruning from scheduler",
                    ) {
                        tracing::warn!(
                            "shared-channel sync delivered-ledger pruning scheduler tick failed: {error:?}"
                        );
                    }
                }
            }) {
            Ok(_) => {}
            Err(error) => {
                self.shared_channel_sync_stale_reclaim_scheduler_started
                    .store(false, Ordering::Release);
                tracing::warn!(
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

    fn persist_state_transition_batch(
        &self,
        next: &SocialControlState,
        commits: &[CommitEnvelope],
    ) -> Result<SocialWritePersistence, ControlPlaneError> {
        let Some(marker_event_id) = commits.first().map(|commit| commit.event_id.as_str()) else {
            return Ok(self.current_persistence());
        };
        self.commit_journal
            .append_batch(commits.to_vec())
            .map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_commit_journal_unavailable",
                    format!(
                        "failed to append social commit journal batch before state write: {}",
                        contract_error_message(error)
                    ),
                )
            })?;
        self.write_pending_tx_marker(marker_event_id)
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
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
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
                .clone()
        };
        repaired_state.merge_pending_shared_channel_sync_requests_from(&pending_state);
        repaired_state.merge_dead_letter_shared_channel_sync_requests_from(&pending_state);
        repaired_state.merge_delivered_shared_channel_sync_requests_from(&pending_state);
        repaired_state.merge_delivered_shared_channel_sync_delivery_proofs_from(&pending_state);
        repaired_state.merge_recent_shared_channel_sync_deliveries_from(&pending_state);
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = repaired_state.clone();
        Ok(SocialRuntimeRepairResponse {
            status: SocialRuntimeRepairStatus::Repaired,
            journal_authority: matches!(self.state_store, SocialStateStore::File { .. }),
            snapshot_updated: true,
            transaction_marker_cleared,
            aggregate_counts: repaired_state.aggregate_counts(),
        })
    }

    fn acquire_cross_instance_write_lock(
        &self,
    ) -> Result<Option<SocialWriteLockGuard>, ControlPlaneError> {
        let Some(path) = self.write_lock_path.as_deref() else {
            return Ok(None);
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_state_unavailable",
                    format!(
                        "failed to create control-plane social lock directory {}: {error}",
                        parent.display()
                    ),
                )
            })?;
        }
        let file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(path)
            .map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_state_unavailable",
                    format!(
                        "failed to open control-plane social write lock {}: {error}",
                        path.display()
                    ),
                )
            })?;
        file.lock_exclusive().map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!(
                    "failed to acquire control-plane social write lock {}: {error}",
                    path.display()
                ),
            )
        })?;
        Ok(Some(SocialWriteLockGuard { file }))
    }

    fn acquire_cross_instance_read_lock(
        &self,
    ) -> Result<Option<SocialWriteLockGuard>, ControlPlaneError> {
        let Some(path) = self.write_lock_path.as_deref() else {
            return Ok(None);
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_state_unavailable",
                    format!(
                        "failed to create control-plane social lock directory {}: {error}",
                        parent.display()
                    ),
                )
            })?;
        }
        let file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(path)
            .map_err(|error| {
                ControlPlaneError::service_unavailable(
                    "social_state_unavailable",
                    format!(
                        "failed to open control-plane social read lock {}: {error}",
                        path.display()
                    ),
                )
            })?;
        file.lock_shared().map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!(
                    "failed to acquire control-plane social read lock {}: {error}",
                    path.display()
                ),
            )
        })?;
        Ok(Some(SocialWriteLockGuard { file }))
    }

    fn refresh_state_from_authority_for_write(&self) -> Result<(), ControlPlaneError> {
        let Some(journal_path) = self.journal_path.as_deref() else {
            self.ensure_social_authority_available()?;
            return Ok(());
        };

        let mut authoritative_state = if journal_path.exists() {
            match Self::replay_state_from_commit_journal(journal_path) {
                Ok(replayed) => replayed,
                Err(error) => {
                    let replay_error = format!(
                        "failed to replay control-plane social commit journal {} during cross-instance refresh: {error}",
                        journal_path.display()
                    );
                    *self
                        .authority_replay_error
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) =
                        Some(replay_error.clone());
                    return Err(ControlPlaneError::service_unavailable(
                        "social_commit_journal_unavailable",
                        replay_error,
                    ));
                }
            }
        } else {
            match self.state_store.load() {
                Ok(snapshot_state) => snapshot_state,
                Err(error) => {
                    let replay_error = format!(
                        "failed to load control-plane social snapshot during cross-instance write refresh without commit journal: {error}"
                    );
                    *self
                        .authority_replay_error
                        .write()
                        .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) =
                        Some(replay_error.clone());
                    return Err(ControlPlaneError::service_unavailable(
                        "social_state_unavailable",
                        replay_error,
                    ));
                }
            }
        };
        let snapshot_state = match self.state_store.load() {
            Ok(snapshot_state) => snapshot_state,
            Err(error) if journal_path.exists() => {
                tracing::warn!(
                    "failed to load control-plane social snapshot during cross-instance write refresh: {error}. continuing from commit journal authority"
                );
                SocialControlState::default()
            }
            Err(error) => {
                let replay_error = format!(
                    "failed to load control-plane social snapshot during cross-instance write refresh without commit journal: {error}"
                );
                *self
                    .authority_replay_error
                    .write()
                    .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) =
                    Some(replay_error.clone());
                return Err(ControlPlaneError::service_unavailable(
                    "social_state_unavailable",
                    replay_error,
                ));
            }
        };
        authoritative_state.merge_pending_shared_channel_sync_requests_from(&snapshot_state);
        authoritative_state.merge_dead_letter_shared_channel_sync_requests_from(&snapshot_state);
        authoritative_state.merge_delivered_shared_channel_sync_requests_from(&snapshot_state);
        authoritative_state
            .merge_delivered_shared_channel_sync_delivery_proofs_from(&snapshot_state);
        authoritative_state.merge_recent_shared_channel_sync_deliveries_from(&snapshot_state);
        *self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = authoritative_state;
        *self
            .authority_replay_error
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = None;
        Ok(())
    }

    fn ensure_social_authority_available(&self) -> Result<(), ControlPlaneError> {
        let replay_error = self
            .authority_replay_error
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone();
        if let Some(error) = replay_error {
            return Err(ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                error,
            ));
        }
        Ok(())
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let now_epoch_millis = current_unix_epoch_millis();
        let now = format_unix_timestamp_millis(now_epoch_millis);
        let dedup_window_start = format_unix_timestamp_millis(
            now_epoch_millis
                .saturating_sub(SHARED_CHANNEL_SYNC_DISPATCH_DELIVERY_DEDUP_WINDOW_MILLIS),
        );
        let retry_window_start = format_unix_timestamp_millis(
            now_epoch_millis
                .saturating_sub(resolve_shared_channel_sync_pending_retry_cooldown_millis()),
        );
        let mut queue = Vec::with_capacity(
            state.pending_shared_channel_retry_index.len()
                + state.dead_letter_shared_channel_sync_requests.len()
                + requests.len(),
        );
        let mut blocked = BTreeSet::new();
        let mut seen = BTreeSet::new();
        for pending in
            state.retryable_pending_shared_channel_sync_requests(retry_window_start.as_str())
        {
            if state.is_delivered_shared_channel_sync_request(&pending.request) {
                continue;
            }
            if state.is_dead_letter_shared_channel_sync_request(&pending.request) {
                continue;
            }
            let key = shared_channel_sync_request_key(&pending.request);
            if !pending.auto_dispatch_eligible(now.as_str(), retry_window_start.as_str()) {
                blocked.insert(key);
                continue;
            }
            if seen.insert(key) {
                queue.push(pending.request.clone());
            }
        }
        for request in requests {
            if state.is_delivered_shared_channel_sync_request(request) {
                continue;
            }
            if state.is_dead_letter_shared_channel_sync_request(request) {
                continue;
            }
            if state.recently_dispatched_shared_channel_sync_request(
                request,
                dedup_window_start.as_str(),
            ) {
                continue;
            }
            if state.pending_shared_channel_sync_request_blocks_dispatch(
                request,
                now.as_str(),
                retry_window_start.as_str(),
            ) {
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

    fn clear_pending_shared_channel_sync_request_and_record_delivery(
        &self,
        request: &SharedChannelLinkedMemberSyncRequest,
        proof: Option<&SharedChannelSyncDeliveryProof>,
    ) -> Result<(), ControlPlaneError> {
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut next_state = state.clone();
        let now_epoch_millis = current_unix_epoch_millis();
        let delivered_at = format_unix_timestamp_millis(now_epoch_millis);
        let dedup_window_start = format_unix_timestamp_millis(
            now_epoch_millis
                .saturating_sub(SHARED_CHANNEL_SYNC_DISPATCH_DELIVERY_DEDUP_WINDOW_MILLIS),
        );
        let retention_window_start =
            shared_channel_sync_delivered_ledger_retention_window_start(now_epoch_millis);
        let max_entries = resolve_shared_channel_sync_delivered_ledger_max_entries();
        let removed_pending = next_state.remove_pending_shared_channel_sync_request(request);
        let removed_dead_letter = next_state
            .dead_letter_shared_channel_sync_requests
            .remove(shared_channel_sync_request_key(request).as_str())
            .is_some();
        let recorded_delivery = next_state.record_dispatched_shared_channel_sync_request(
            request,
            delivered_at.as_str(),
            dedup_window_start.as_str(),
            proof,
            removed_pending || removed_dead_letter,
        );
        let pruned_delivered = next_state.prune_delivered_shared_channel_sync_requests(
            retention_window_start.as_str(),
            max_entries,
        );
        if !removed_pending && !removed_dead_letter && !recorded_delivery && pruned_delivered == 0 {
            return Ok(());
        }
        self.state_store.save(&next_state).map_err(|error| {
            let request_key = shared_channel_sync_request_key(request);
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!(
                    "failed to persist shared-channel sync delivered state for requestKey {request_key}: {error}"
                ),
            )
        })?;
        *state = next_state;
        Ok(())
    }

    fn repair_shared_channel_sync(
        &self,
        trigger: Option<&dyn SharedChannelLinkedMemberSyncTrigger>,
    ) -> Result<SocialSharedChannelSyncRepairResponse, ControlPlaneError> {
        let current_state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
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
        let now_epoch_millis = current_unix_epoch_millis();
        let now = format_unix_timestamp_millis(now_epoch_millis);
        let dedup_window_start = format_unix_timestamp_millis(
            now_epoch_millis
                .saturating_sub(SHARED_CHANNEL_SYNC_DISPATCH_DELIVERY_DEDUP_WINDOW_MILLIS),
        );
        let retention_window_start =
            shared_channel_sync_delivered_ledger_retention_window_start(now_epoch_millis);
        let max_entries = resolve_shared_channel_sync_delivered_ledger_max_entries();
        let reclaimed = next_state.reclaim_stale_pending_shared_channel_sync_claims(now.as_str());
        let pruned = next_state.prune_delivered_shared_channel_sync_backlog();
        let delivered_pruned = next_state.prune_delivered_shared_channel_sync_requests(
            retention_window_start.as_str(),
            max_entries,
        );

        let Some(trigger) = trigger else {
            if reclaimed > 0 || pruned > 0 || delivered_pruned > 0 {
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
                    .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) =
                    next_state.clone();
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
        let attempted = pending_items.len();
        let mut dispatched = 0usize;
        let mut failed = 0usize;
        for pending in pending_items {
            match trigger.trigger_with_delivery_proof(pending.request.clone()) {
                Ok(delivery_proof) => {
                    next_state.remove_pending_shared_channel_sync_request(&pending.request);
                    next_state
                        .dead_letter_shared_channel_sync_requests
                        .remove(shared_channel_sync_request_key(&pending.request).as_str());
                    next_state.record_dispatched_shared_channel_sync_request(
                        &pending.request,
                        now.as_str(),
                        dedup_window_start.as_str(),
                        Some(&delivery_proof),
                        true,
                    );
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
        next_state.prune_delivered_shared_channel_sync_requests(
            retention_window_start.as_str(),
            max_entries,
        );

        self.state_store.save(&next_state).map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!("failed to persist shared-channel sync repair backlog: {error}"),
            )
        })?;
        *self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = next_state.clone();

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
            match (attempted, dispatched, pending_after) {
                (0, _, 0) => SocialSharedChannelSyncRepairStatus::Repaired,
                (0, _, _) => SocialSharedChannelSyncRepairStatus::Pending,
                (_, 0, _) => SocialSharedChannelSyncRepairStatus::Pending,
                (_, _, 0) => SocialSharedChannelSyncRepairStatus::Repaired,
                _ => SocialSharedChannelSyncRepairStatus::PartiallyRepaired,
            }
        };

        Ok(SocialSharedChannelSyncRepairResponse {
            status,
            pending_before,
            attempted,
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = next_state.clone();

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
        page: &SharedChannelSyncInventoryPageSpec,
    ) -> SocialSharedChannelSyncDeadLetterInventoryResponse {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        let page_entries = shared_channel_sync_inventory_map_page(
            &state.dead_letter_shared_channel_sync_requests,
            page.limit,
            page.cursor
                .as_ref()
                .map(|cursor| cursor.request_key.as_str()),
        );
        let next_cursor = page_entries
            .next_key
            .as_deref()
            .map(shared_channel_sync_inventory_cursor_for);
        let items = page_entries
            .items
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
            dead_letter_count: state.dead_letter_shared_channel_sync_requests.len(),
            next_cursor,
            items,
        }
    }

    fn pending_shared_channel_sync_inventory(
        &self,
        actor_id: &str,
        actor_kind: &str,
        can_takeover: bool,
        page: &SharedChannelSyncInventoryPageSpec,
    ) -> SocialSharedChannelSyncPendingInventoryResponse {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let now = format_unix_timestamp_millis(current_unix_epoch_millis());
        let page_entries = shared_channel_sync_inventory_map_page(
            &state.pending_shared_channel_sync_requests,
            page.limit,
            page.cursor
                .as_ref()
                .map(|cursor| cursor.request_key.as_str()),
        );
        let next_cursor = page_entries
            .next_key
            .as_deref()
            .map(shared_channel_sync_inventory_cursor_for);
        let items = page_entries
            .items
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
            pending_count: state.pending_shared_channel_sync_requests.len(),
            next_cursor,
            items,
        }
    }

    fn delivered_shared_channel_sync_inventory(
        &self,
        page: &SharedChannelSyncInventoryPageSpec,
    ) -> SocialSharedChannelSyncDeliveredInventoryResponse {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let page_entries = shared_channel_sync_inventory_map_page(
            &state.delivered_shared_channel_sync_requests,
            page.limit,
            page.cursor
                .as_ref()
                .map(|cursor| cursor.request_key.as_str()),
        );
        let next_cursor = page_entries
            .next_key
            .as_deref()
            .map(shared_channel_sync_inventory_cursor_for);
        let items = page_entries
            .items
            .into_iter()
            .map(|(request_key, delivered_at)| {
                let proof = state
                    .delivered_shared_channel_sync_delivery_proofs
                    .get(request_key.as_str())
                    .cloned();
                social_shared_channel_sync_delivered_inventory_item_response(
                    request_key,
                    delivered_at,
                    proof,
                )
            })
            .collect::<Vec<_>>();
        SocialSharedChannelSyncDeliveredInventoryResponse {
            status: SocialSharedChannelSyncDeliveredInventoryStatus::Snapshot,
            delivered_count: state.delivered_shared_channel_sync_requests.len(),
            next_cursor,
            items,
        }
    }

    fn shared_channel_sync_delivery_state_inventory(
        &self,
        page: &SharedChannelSyncInventoryPageSpec,
    ) -> SocialSharedChannelSyncDeliveryStateInventoryResponse {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let delivered_count = state.delivered_shared_channel_sync_requests.len();
        let pending_count = state.pending_shared_channel_sync_requests.len();
        let dead_letter_count = state.dead_letter_shared_channel_sync_requests.len();
        let mut items_by_key =
            BTreeMap::<String, SocialSharedChannelSyncDeliveryStateInventoryItemResponse>::new();
        for (request_key, delivered_at) in &state.delivered_shared_channel_sync_requests {
            items_by_key.insert(
                request_key.clone(),
                social_shared_channel_sync_delivery_state_item_from_proof(
                    request_key.clone(),
                    delivered_at.clone(),
                    state
                        .delivered_shared_channel_sync_delivery_proofs
                        .get(request_key.as_str())
                        .cloned(),
                ),
            );
        }
        for (request_key, request) in &state.pending_shared_channel_sync_requests {
            items_by_key.insert(
                request_key.clone(),
                social_shared_channel_sync_delivery_state_item_from_pending(
                    request_key.clone(),
                    request.clone(),
                    false,
                ),
            );
        }
        for (request_key, request) in &state.dead_letter_shared_channel_sync_requests {
            items_by_key.insert(
                request_key.clone(),
                social_shared_channel_sync_delivery_state_item_from_pending(
                    request_key.clone(),
                    request.clone(),
                    true,
                ),
            );
        }
        let page_entries = shared_channel_sync_inventory_map_page(
            &items_by_key,
            page.limit,
            page.cursor
                .as_ref()
                .map(|cursor| cursor.request_key.as_str()),
        );
        let next_cursor = page_entries
            .next_key
            .as_deref()
            .map(shared_channel_sync_inventory_cursor_for);
        let items = page_entries
            .items
            .into_iter()
            .map(|(_, item)| item)
            .collect::<Vec<_>>();
        SocialSharedChannelSyncDeliveryStateInventoryResponse {
            status: SocialSharedChannelSyncDeliveryStateInventoryStatus::Snapshot,
            delivered_count,
            pending_count,
            dead_letter_count,
            total_count: items_by_key.len(),
            next_cursor,
            items,
        }
    }

    fn prune_delivered_shared_channel_sync_backlog_if_any(
        &self,
        persistence_error_context: &str,
    ) -> Result<usize, ControlPlaneError> {
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        if state.delivered_shared_channel_sync_requests.is_empty() {
            return Ok(0);
        }

        let mut next_state = state.clone();
        let now_epoch_millis = current_unix_epoch_millis();
        let retention_window_start =
            shared_channel_sync_delivered_ledger_retention_window_start(now_epoch_millis);
        let max_entries = resolve_shared_channel_sync_delivered_ledger_max_entries();
        let pruned_backlog = next_state.prune_delivered_shared_channel_sync_backlog();
        let pruned_delivered = next_state.prune_delivered_shared_channel_sync_requests(
            retention_window_start.as_str(),
            max_entries,
        );
        let pruned = pruned_backlog.saturating_add(pruned_delivered);
        if pruned == 0 {
            return Ok(0);
        }

        self.state_store.save(&next_state).map_err(|error| {
            ControlPlaneError::service_unavailable(
                "social_state_unavailable",
                format!("{persistence_error_context}: {error}"),
            )
        })?;
        *state = next_state;
        Ok(pruned)
    }

    fn reclaim_stale_pending_shared_channel_sync_claims_if_any(
        &self,
        persistence_error_context: &str,
    ) -> Result<usize, ControlPlaneError> {
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
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
        validate_request_keys_payload("requestKeys", request_keys)?;
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
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
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = next_state.clone();
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
        validate_request_keys_payload("requestKeys", request_keys)?;
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
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

        let selected_pending_items =
            current_state.selected_pending_shared_channel_sync_requests(&request_keys);

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
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = next_state.clone();
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
        validate_request_keys_payload("requestKeys", request_keys)?;
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
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
        let selected_pending_items =
            current_state.selected_pending_shared_channel_sync_requests(&request_keys);

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
                .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = next_state.clone();
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
        validate_request_keys_payload("requestKeys", request_keys)?;
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = next_state.clone();

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
        validate_request_keys_payload("requestKeys", request_keys)?;
        let request_keys = request_keys
            .iter()
            .filter(|key| !key.is_empty())
            .cloned()
            .collect::<BTreeSet<_>>();
        let current_state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .clone();
        let pending_before = current_state.pending_shared_channel_sync_count();
        let dead_letter_before = current_state.dead_letter_shared_channel_sync_count();
        let requested = request_keys.len();
        let selected_pending_items =
            current_state.selected_undelivered_pending_shared_channel_sync_requests(&request_keys);
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
        let republish_started_epoch_millis = current_unix_epoch_millis();
        let republish_started_at = format_unix_timestamp_millis(republish_started_epoch_millis);
        let dedup_window_start = format_unix_timestamp_millis(
            republish_started_epoch_millis
                .saturating_sub(SHARED_CHANNEL_SYNC_DISPATCH_DELIVERY_DEDUP_WINDOW_MILLIS),
        );
        let retention_window_start = shared_channel_sync_delivered_ledger_retention_window_start(
            republish_started_epoch_millis,
        );
        let max_entries = resolve_shared_channel_sync_delivered_ledger_max_entries();
        let mut renewed_stale_same_owner_lease = false;
        for (request_key, pending) in &selected_pending_items {
            if pending.is_owned_by(actor_id, actor_kind)
                && pending.lease_status(republish_started_at.as_str())
                    == SocialSharedChannelSyncLeaseStatus::Stale
                && let Some(mut selected_pending) = next_state
                    .pending_shared_channel_sync_requests
                    .get(request_key.as_str())
                    .cloned()
            {
                selected_pending.assign_owner(actor_id, actor_kind);
                next_state.upsert_pending_shared_channel_sync_request(
                    request_key.clone(),
                    selected_pending,
                );
                renewed_stale_same_owner_lease = true;
            }
        }

        let Some(trigger) = trigger else {
            let delivered_pruned = next_state.prune_delivered_shared_channel_sync_requests(
                retention_window_start.as_str(),
                max_entries,
            );
            if renewed_stale_same_owner_lease || delivered_pruned > 0 {
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
                    .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = next_state;
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
            match trigger.trigger_with_delivery_proof(pending.request.clone()) {
                Ok(delivery_proof) => {
                    next_state.remove_pending_shared_channel_sync_request(&pending.request);
                    next_state
                        .dead_letter_shared_channel_sync_requests
                        .remove(shared_channel_sync_request_key(&pending.request).as_str());
                    next_state.record_dispatched_shared_channel_sync_request(
                        &pending.request,
                        republish_started_at.as_str(),
                        dedup_window_start.as_str(),
                        Some(&delivery_proof),
                        true,
                    );
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
        next_state.prune_delivered_shared_channel_sync_requests(
            retention_window_start.as_str(),
            max_entries,
        );

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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock) = next_state.clone();

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

impl From<AppContextError> for ControlPlaneError {
    fn from(value: AppContextError) -> Self {
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

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
            details: None,
        }
    }

    fn payload_too_many_items(field: &'static str, max_items: usize, actual_items: usize) -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_items} items, actual={actual_items} items"
            ),
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
        let status = self.status;
        let response_status = Self::response_status(status);
        let detail = self.message;
        let message = detail.clone();
        let title = status.canonical_reason().unwrap_or("Unknown Error");
        let mut body = serde_json::json!({
            "type": "about:blank",
            "title": title,
            "status": status.as_u16(),
            "detail": detail,
            "code": self.code,
            "message": message,
            "errorStatus": response_status
        });
        if let Some(details) = self.details {
            body["details"] = details;
        }
        (
            status,
            [(CONTENT_TYPE, "application/problem+json; charset=utf-8")],
            Json(body),
        )
            .into_response()
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
        auth: &AppContext,
        request: EstablishExternalConnectionRequest,
    ) -> Result<EstablishedExternalConnection, ControlPlaneError> {
        validate_payload_size(
            "connectionId",
            request.connection_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "externalTenantId",
            request.external_tenant_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "externalOrgName",
            request.external_org_name.as_deref(),
            CONTROL_PLANE_MAX_EXTERNAL_ORG_NAME_BYTES,
        )?;
        validate_payload_size(
            "establishedAt",
            request.established_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
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
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::ExternalConnection,
            aggregate_id: request.connection_id.as_str(),
            event_type: SocialEventType::ExternalConnectionEstablished,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.established_at.as_str(),
            committed_at: request.established_at.as_str(),
            payload: payload_json.as_str(),
        });
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

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
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
        if active_external_connection_record_for_target(
            &next_state,
            tenant_id,
            external_connection.external_tenant_id.as_str(),
            &external_connection.connection_kind,
        )
        .is_some()
        {
            return Err(ControlPlaneError::conflict(
                "external_connection_target_conflict",
                format!(
                    "active external connection already exists for tenant {} and kind {:?}",
                    external_connection.external_tenant_id, external_connection.connection_kind
                ),
            ));
        }

        next_state.insert_external_connection_record(
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .external_connections
            .get(connection_id)
            .filter(|record| record.external_connection.tenant_id == tenant_id)
            .cloned()
    }

    fn bind_external_member_link(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: BindExternalMemberLinkRequest,
    ) -> Result<BoundExternalMemberLink, ControlPlaneError> {
        validate_payload_size(
            "linkId",
            request.link_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "connectionId",
            request.connection_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "localActorId",
            request.local_actor_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "localActorKind",
            request.local_actor_kind.as_str(),
            CONTROL_PLANE_MAX_ACTOR_KIND_BYTES,
        )?;
        validate_payload_size(
            "externalMemberId",
            request.external_member_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "externalDisplayName",
            request.external_display_name.as_deref(),
            CONTROL_PLANE_MAX_EXTERNAL_DISPLAY_NAME_BYTES,
        )?;
        validate_payload_size(
            "linkedAt",
            request.linked_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
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

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
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
            local_actor_kind: request.local_actor_kind.clone(),
            external_member_id: request.external_member_id.clone(),
            external_display_name: request.external_display_name.clone(),
            linked_at: request.linked_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("external member link payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::ExternalMemberLink,
            aggregate_id: request.link_id.as_str(),
            event_type: SocialEventType::ExternalMemberLinkBound,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.linked_at.as_str(),
            committed_at: request.linked_at.as_str(),
            payload: payload_json.as_str(),
        });
        let external_member_link = ExternalMemberLink {
            tenant_id: tenant_id.into(),
            link_id: request.link_id.clone(),
            connection_id: request.connection_id.clone(),
            local_actor_id: request.local_actor_id,
            local_actor_kind: request.local_actor_kind,
            external_member_id: request.external_member_id,
            external_display_name: request.external_display_name,
            status: ExternalMemberLinkStatus::Active,
            linked_at: request.linked_at.clone(),
            updated_at: request.linked_at,
        };

        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
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
        if active_external_member_link_record_for_mapping(
            &next_state,
            tenant_id,
            external_member_link.connection_id.as_str(),
            external_member_link.external_member_id.as_str(),
        )
        .is_some()
        {
            return Err(ControlPlaneError::conflict(
                "external_member_mapping_conflict",
                format!(
                    "active external member mapping already exists for {} on connection {}",
                    external_member_link.external_member_id, external_member_link.connection_id
                ),
            ));
        }

        next_state.insert_external_member_link_record(
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .external_member_links
            .get(link_id)
            .filter(|record| record.external_member_link.tenant_id == tenant_id)
            .cloned()
    }

    fn apply_shared_channel_policy(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: ApplySharedChannelPolicyRequest,
    ) -> Result<AppliedSharedChannelPolicy, ControlPlaneError> {
        validate_payload_size(
            "policyId",
            request.policy_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "connectionId",
            request.connection_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "channelId",
            request.channel_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "conversationId",
            request.conversation_id.as_deref(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "historyVisibility",
            request.history_visibility.as_str(),
            CONTROL_PLANE_MAX_HISTORY_VISIBILITY_BYTES,
        )?;
        validate_payload_size(
            "appliedAt",
            request.applied_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
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

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
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
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::SharedChannelPolicy,
            aggregate_id: request.policy_id.as_str(),
            event_type: SocialEventType::SharedChannelPolicyApplied,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.applied_at.as_str(),
            committed_at: request.applied_at.as_str(),
            payload: payload_json.as_str(),
        });
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
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
        if active_shared_channel_policy_record_for_target(
            &next_state,
            tenant_id,
            shared_channel_policy.connection_id.as_str(),
            shared_channel_policy.channel_id.as_str(),
        )
        .is_some()
        {
            return Err(ControlPlaneError::conflict(
                "shared_channel_policy_target_conflict",
                format!(
                    "active shared channel policy already exists for channel {} on connection {}",
                    shared_channel_policy.channel_id, shared_channel_policy.connection_id
                ),
            ));
        }

        next_state.insert_shared_channel_policy_record(
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .shared_channel_policies
            .get(policy_id)
            .filter(|record| record.shared_channel_policy.tenant_id == tenant_id)
            .cloned()
    }

    fn submit_friend_request(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: SubmitFriendRequestRequest,
    ) -> Result<SubmittedFriendRequest, ControlPlaneError> {
        validate_payload_size(
            "requestId",
            request.request_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "requesterUserId",
            request.requester_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "targetUserId",
            request.target_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "requestMessage",
            request.request_message.as_deref(),
            CONTROL_PLANE_MAX_REQUEST_MESSAGE_BYTES,
        )?;
        validate_payload_size(
            "requestedAt",
            request.requested_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
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
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::FriendRequest,
            aggregate_id: request.request_id.as_str(),
            event_type: SocialEventType::FriendRequestSubmitted,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.requested_at.as_str(),
            committed_at: request.requested_at.as_str(),
            payload: payload_json.as_str(),
        });
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

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
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
            return Err(ControlPlaneError::conflict_with_details(
                "friend_request_conflict",
                format!(
                    "friend request {} already exists",
                    friend_request.request_id
                ),
                serde_json::json!({
                    "existingRequestId": friend_request.request_id,
                    "existingStatus": FriendRequestStatus::Pending,
                    "existingRequesterUserId": friend_request.requester_user_id,
                    "existingTargetUserId": friend_request.target_user_id
                }),
            ));
        }
        let requested_pair = friend_request
            .user_pair()
            .expect("validated friend request should expose normalized user pair");
        if let Some(user_block) = active_friendship_scoped_user_block(
            &next_state,
            tenant_id,
            friend_request.requester_user_id.as_str(),
            friend_request.target_user_id.as_str(),
        ) {
            return Err(ControlPlaneError::conflict_with_details(
                "friend_request_blocked",
                format!(
                    "friend request pair {} is blocked by {}",
                    requested_pair.pair_key(),
                    user_block.block_id
                ),
                social_pair_block_conflict_details(&user_block),
            ));
        }
        if let Some(existing_friendship) = active_friendship_record_for_pair(
            &next_state,
            tenant_id,
            requested_pair.user_low_id.as_str(),
            requested_pair.user_high_id.as_str(),
        ) {
            return Err(ControlPlaneError::conflict_with_details(
                "friendship_pair_conflict",
                format!(
                    "active friendship already exists for pair {}",
                    requested_pair.pair_key()
                ),
                serde_json::json!({
                    "existingFriendshipId": existing_friendship.friendship.friendship_id,
                    "existingStatus": existing_friendship.friendship.status,
                    "userLowId": existing_friendship.friendship.user_low_id,
                    "userHighId": existing_friendship.friendship.user_high_id
                }),
            ));
        }
        let pair_has_materialized_friendship = friendship_pair_has_materialized_record(
            &next_state,
            tenant_id,
            requested_pair.user_low_id.as_str(),
            requested_pair.user_high_id.as_str(),
        );
        if let Some(existing) = open_friend_request_record_for_pair(
            &next_state,
            tenant_id,
            requested_pair.user_low_id.as_str(),
            requested_pair.user_high_id.as_str(),
            pair_has_materialized_friendship,
        ) {
            return Err(ControlPlaneError::conflict_with_details(
                "friend_request_pair_conflict",
                format!(
                    "open friend request already exists for pair {}",
                    requested_pair.pair_key()
                ),
                serde_json::json!({
                    "existingRequestId": existing.friend_request.request_id,
                    "existingStatus": existing.friend_request.status,
                    "existingRequesterUserId": existing.friend_request.requester_user_id,
                    "existingTargetUserId": existing.friend_request.target_user_id
                }),
            ));
        }

        next_state.insert_friend_request_record(
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.tenant_id == tenant_id)
            .cloned()
    }

    fn list_friend_requests(
        &self,
        tenant_id: &str,
        user_id: &str,
        direction: FriendRequestInventoryDirectionQuery,
        status: FriendRequestInventoryStatusQuery,
        limit: usize,
        cursor: Option<&FriendRequestInventoryCursor>,
    ) -> FriendRequestInventoryPage {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let mut items = friend_request_records_for_user(&state, tenant_id, user_id)
            .into_iter()
            .filter(|record| {
                friend_request_matches_inventory_direction(
                    &record.friend_request,
                    user_id,
                    direction,
                )
            })
            .filter(|record| {
                friend_request_matches_inventory_status(&record.friend_request, status)
            })
            .map(|record| record.friend_request.clone())
            .collect::<Vec<_>>();
        items.sort_by(compare_friend_request_inventory_order);
        if let Some(cursor) = cursor {
            items.retain(|item| compare_friend_request_inventory_with_cursor(item, cursor).is_gt());
        }
        let next_cursor = if items.len() > limit {
            items
                .get(limit - 1)
                .map(friend_request_inventory_cursor_for)
        } else {
            None
        };
        items.truncate(limit);
        FriendRequestInventoryPage { items, next_cursor }
    }

    fn accept_friend_request(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request_id: &str,
        request: AcceptFriendRequestRequest,
    ) -> Result<AcceptedFriendRequest, ControlPlaneError> {
        validate_payload_size("requestId", request_id, CONTROL_PLANE_MAX_ID_BYTES)?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "acceptedByUserId",
            request.accepted_by_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "acceptedAt",
            request.accepted_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("requestId", request_id, "invalid_friend_request")?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "acceptedByUserId",
            request.accepted_by_user_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "acceptedAt",
            request.accepted_at.as_str(),
            "invalid_friend_request",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let stored = state
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.tenant_id == tenant_id)
            .cloned()
            .ok_or_else(|| {
                ControlPlaneError::not_found(
                    "friend_request_not_found",
                    format!("friend request {request_id} was not found"),
                )
            })?;
        let existing_committed_event = state.committed_event(tenant_id, request.event_id.as_str());
        let existing_ordering_seq = existing_committed_event
            .as_ref()
            .map(|existing| existing.commit().ordering_seq);
        if stored.friend_request.target_user_id != request.accepted_by_user_id {
            return Err(ControlPlaneError::invalid(
                "invalid_friend_request",
                format!("acceptedByUserId must match target user for {request_id}"),
            ));
        }
        if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            return Err(ControlPlaneError::conflict(
                "friend_request_not_pending",
                format!("friend request {request_id} is not pending"),
            ));
        }

        let user_pair = stored
            .friend_request
            .user_pair()
            .expect("validated friend request should expose normalized user pair");
        let actor_pair = normalize_actor_pair(
            stored.friend_request.requester_user_id.as_str(),
            stored.friend_request.target_user_id.as_str(),
        )
        .expect("validated friend request participants should normalize into direct chat pair");
        let accepted_at = request.accepted_at.clone();
        let friendship_id = deterministic_social_id("fs_", request_id);
        let friendship_event_id = deterministic_social_id("evt_fs_activate_", request_id);
        let direct_chat_id = deterministic_social_id("dc_", request_id);
        let direct_chat_event_id = deterministic_social_id("evt_dc_bind_", request_id);
        let conversation_id = deterministic_social_id("c_direct_", request_id);
        let payload = FriendRequestAcceptedPayload {
            request_id: request_id.into(),
            accepted_by_user_id: request.accepted_by_user_id.clone(),
            accepted_at: accepted_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friend request accept payload should serialize into json");
        let accept_commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::FriendRequest,
            aggregate_id: request_id,
            event_type: SocialEventType::FriendRequestAccepted,
            ordering_seq: existing_ordering_seq.unwrap_or(stored.commits.len() as u64 + 1),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: accepted_at.as_str(),
            committed_at: accepted_at.as_str(),
            payload: payload_json.as_str(),
        });
        let accept_commit_already_committed = if let Some(existing) = existing_committed_event {
            if existing.commit() != &accept_commit {
                return Err(social_event_id_conflict(
                    request.event_id.as_str(),
                    &existing,
                ));
            }
            true
        } else {
            false
        };
        if let Some(user_block) = active_friendship_scoped_user_block(
            &state,
            tenant_id,
            stored.friend_request.requester_user_id.as_str(),
            stored.friend_request.target_user_id.as_str(),
        ) {
            let pair = stored
                .friend_request
                .user_pair()
                .expect("validated friend request should expose normalized user pair");
            return Err(ControlPlaneError::conflict_with_details(
                "friend_request_blocked",
                format!(
                    "friend request pair {} is blocked by {}",
                    pair.pair_key(),
                    user_block.block_id
                ),
                social_pair_block_conflict_details(&user_block),
            ));
        }

        let mut next_state = state.clone();
        let mut commits_to_persist = Vec::new();
        let friend_request = if accept_commit_already_committed {
            next_state
                .friend_requests
                .get(request_id)
                .expect("friend request should exist after replay validation")
                .friend_request
                .clone()
        } else {
            let mut record = next_state
                .friend_requests
                .get(request_id)
                .cloned()
                .expect("friend request should exist after validation");
            record.friend_request.status = FriendRequestStatus::Accepted;
            record.friend_request.updated_at = accepted_at.clone();
            record.commits.push(accept_commit.clone());
            commits_to_persist.push(accept_commit.clone());
            let friend_request = record.friend_request.clone();
            next_state.insert_friend_request_record(request_id.to_owned(), record);
            friend_request
        };

        let existing_friendship = active_friendship_record_for_pair(
            &next_state,
            tenant_id,
            &user_pair.user_low_id,
            &user_pair.user_high_id,
        );
        let existing_direct_chat = active_direct_chat_record_for_pair(
            &next_state,
            tenant_id,
            actor_pair.left_actor_id.as_str(),
            actor_pair.right_actor_id.as_str(),
        );

        let planned_direct_chat_id = existing_direct_chat
            .as_ref()
            .map(|record| record.direct_chat.direct_chat_id.clone())
            .unwrap_or_else(|| direct_chat_id.clone());

        let (friendship, friendship_materialized_commit) = if let Some(record) = existing_friendship
        {
            (Some(record.friendship), None)
        } else {
            let friendship_payload = FriendshipActivatedPayload {
                friendship_id: friendship_id.clone(),
                user_low_id: user_pair.user_low_id.clone(),
                user_high_id: user_pair.user_high_id.clone(),
                initiator_user_id: stored.friend_request.requester_user_id.clone(),
                direct_chat_id: Some(planned_direct_chat_id.clone()),
                established_at: accepted_at.clone(),
            };
            let friendship_payload_json = serde_json::to_string(&friendship_payload)
                .expect("friendship payload should serialize into json");
            let friendship_commit = social_commit_envelope(SocialCommitEnvelopeInput {
                event_id: friendship_event_id.as_str(),
                tenant_id,
                aggregate_type: AggregateType::Friendship,
                aggregate_id: friendship_id.as_str(),
                event_type: SocialEventType::FriendshipActivated,
                ordering_seq: 1,
                actor: EventActor {
                    actor_id: auth.actor_id.clone(),
                    actor_kind: auth.actor_kind.clone(),
                    actor_session_id: auth.session_id.clone(),
                },
                occurred_at: accepted_at.as_str(),
                committed_at: accepted_at.as_str(),
                payload: friendship_payload_json.as_str(),
            });
            if let Some(existing) =
                next_state.committed_event(tenant_id, friendship_event_id.as_str())
            {
                if existing.commit() != &friendship_commit {
                    return Err(social_event_id_conflict(
                        friendship_event_id.as_str(),
                        &existing,
                    ));
                }
                match existing {
                    SocialCommittedEvent::Friendship { record, .. }
                        if record.friendship.status.is_active() =>
                    {
                        (Some(record.friendship), None)
                    }
                    SocialCommittedEvent::Friendship { .. } => (None, None),
                    other => {
                        return Err(social_event_id_conflict(
                            friendship_event_id.as_str(),
                            &other,
                        ));
                    }
                }
            } else {
                if next_state.friendships.contains_key(friendship_id.as_str()) {
                    return Err(ControlPlaneError::conflict(
                        "friendship_conflict",
                        format!("friendship {friendship_id} already exists"),
                    ));
                }
                let friendship = Friendship {
                    tenant_id: tenant_id.into(),
                    friendship_id: friendship_id.clone(),
                    user_low_id: user_pair.user_low_id.clone(),
                    user_high_id: user_pair.user_high_id.clone(),
                    initiator_user_id: stored.friend_request.requester_user_id.clone(),
                    status: FriendshipStatus::Active,
                    established_at: Some(accepted_at.clone()),
                    updated_at: accepted_at.clone(),
                };
                next_state.insert_friendship_record(
                    friendship.friendship_id.clone(),
                    StoredFriendship {
                        friendship: friendship.clone(),
                        commits: vec![friendship_commit.clone()],
                    },
                );
                commits_to_persist.push(friendship_commit.clone());
                (Some(friendship), Some(friendship_commit))
            }
        };

        let (direct_chat, direct_chat_materialized_commit) =
            if let Some(record) = existing_direct_chat {
                (Some(record.direct_chat), None)
            } else {
                let direct_chat_payload = DirectChatBoundPayload {
                    direct_chat_id: direct_chat_id.clone(),
                    conversation_id: conversation_id.clone(),
                    left_actor_id: actor_pair.left_actor_id.clone(),
                    right_actor_id: actor_pair.right_actor_id.clone(),
                    pair_hash: actor_pair.pair_hash.clone(),
                    bound_at: accepted_at.clone(),
                };
                let direct_chat_payload_json = serde_json::to_string(&direct_chat_payload)
                    .expect("direct chat payload should serialize into json");
                let direct_chat_commit = social_commit_envelope(SocialCommitEnvelopeInput {
                    event_id: direct_chat_event_id.as_str(),
                    tenant_id,
                    aggregate_type: AggregateType::DirectChat,
                    aggregate_id: direct_chat_id.as_str(),
                    event_type: SocialEventType::DirectChatBound,
                    ordering_seq: 1,
                    actor: EventActor {
                        actor_id: auth.actor_id.clone(),
                        actor_kind: auth.actor_kind.clone(),
                        actor_session_id: auth.session_id.clone(),
                    },
                    occurred_at: accepted_at.as_str(),
                    committed_at: accepted_at.as_str(),
                    payload: direct_chat_payload_json.as_str(),
                });
                if let Some(existing) =
                    next_state.committed_event(tenant_id, direct_chat_event_id.as_str())
                {
                    if existing.commit() != &direct_chat_commit {
                        return Err(social_event_id_conflict(
                            direct_chat_event_id.as_str(),
                            &existing,
                        ));
                    }
                    match existing {
                        SocialCommittedEvent::DirectChat { record, .. }
                            if record.direct_chat.status.is_active() =>
                        {
                            (Some(record.direct_chat), None)
                        }
                        SocialCommittedEvent::DirectChat { .. } => (None, None),
                        other => {
                            return Err(social_event_id_conflict(
                                direct_chat_event_id.as_str(),
                                &other,
                            ));
                        }
                    }
                } else {
                    if next_state
                        .direct_chats
                        .contains_key(direct_chat_id.as_str())
                    {
                        return Err(ControlPlaneError::conflict(
                            "direct_chat_conflict",
                            format!("direct chat {direct_chat_id} already exists"),
                        ));
                    }
                    let direct_chat = DirectChat {
                        tenant_id: tenant_id.into(),
                        direct_chat_id: direct_chat_id.clone(),
                        left_actor_id: actor_pair.left_actor_id.clone(),
                        right_actor_id: actor_pair.right_actor_id.clone(),
                        pair_hash: actor_pair.pair_hash.clone(),
                        status: DirectChatStatus::Active,
                        conversation_id: Some(conversation_id.clone()),
                        created_at: accepted_at.clone(),
                        updated_at: accepted_at.clone(),
                    };
                    next_state.insert_direct_chat_record(
                        direct_chat.direct_chat_id.clone(),
                        StoredDirectChat {
                            direct_chat: direct_chat.clone(),
                            commits: vec![direct_chat_commit.clone()],
                        },
                    );
                    commits_to_persist.push(direct_chat_commit.clone());
                    (Some(direct_chat), Some(direct_chat_commit))
                }
            };

        let persistence = if commits_to_persist.is_empty() {
            self.repair_derived_snapshot_best_effort(&next_state)
        } else {
            self.persist_state_transition_batch(&next_state, commits_to_persist.as_slice())?
        };
        *state = next_state;

        Ok(AcceptedFriendRequest {
            friend_request,
            latest_commit: accept_commit,
            persistence,
            friendship,
            friendship_materialized_commit,
            direct_chat,
            direct_chat_materialized_commit,
        })
    }

    fn decline_friend_request(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request_id: &str,
        request: DeclineFriendRequestRequest,
    ) -> Result<DeclinedFriendRequest, ControlPlaneError> {
        validate_payload_size("requestId", request_id, CONTROL_PLANE_MAX_ID_BYTES)?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "declinedByUserId",
            request.declined_by_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "declinedAt",
            request.declined_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("requestId", request_id, "invalid_friend_request")?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "declinedByUserId",
            request.declined_by_user_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "declinedAt",
            request.declined_at.as_str(),
            "invalid_friend_request",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let stored = state
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.tenant_id == tenant_id)
            .cloned()
            .ok_or_else(|| {
                ControlPlaneError::not_found(
                    "friend_request_not_found",
                    format!("friend request {request_id} was not found"),
                )
            })?;
        let existing_ordering_seq = state
            .committed_event(tenant_id, request.event_id.as_str())
            .map(|existing| existing.commit().ordering_seq);
        if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            return Err(ControlPlaneError::conflict(
                "friend_request_not_pending",
                format!("friend request {request_id} is not pending"),
            ));
        }
        if stored.friend_request.target_user_id != request.declined_by_user_id {
            return Err(ControlPlaneError::invalid(
                "invalid_friend_request",
                format!("declinedByUserId must match target user for {request_id}"),
            ));
        }

        let payload = FriendRequestDeclinedPayload {
            request_id: request_id.into(),
            declined_by_user_id: request.declined_by_user_id.clone(),
            declined_at: request.declined_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friend request decline payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::FriendRequest,
            aggregate_id: request_id,
            event_type: SocialEventType::FriendRequestDeclined,
            ordering_seq: existing_ordering_seq.unwrap_or(stored.commits.len() as u64 + 1),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.declined_at.as_str(),
            committed_at: request.declined_at.as_str(),
            payload: payload_json.as_str(),
        });
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::FriendRequest { record, commit } => {
                        Ok(DeclinedFriendRequest {
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

        let mut next_state = state.clone();
        let mut record = next_state
            .friend_requests
            .get(request_id)
            .cloned()
            .expect("friend request should exist after validation");
        record.friend_request.status = FriendRequestStatus::Declined;
        record.friend_request.updated_at = request.declined_at;
        let friend_request = record.friend_request.clone();
        record.commits.push(commit.clone());
        next_state.insert_friend_request_record(request_id.to_owned(), record);

        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(DeclinedFriendRequest {
            friend_request,
            latest_commit: commit,
            persistence,
        })
    }

    fn cancel_friend_request(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request_id: &str,
        request: CancelFriendRequestRequest,
    ) -> Result<CanceledFriendRequest, ControlPlaneError> {
        validate_payload_size("requestId", request_id, CONTROL_PLANE_MAX_ID_BYTES)?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "canceledByUserId",
            request.canceled_by_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "canceledAt",
            request.canceled_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("requestId", request_id, "invalid_friend_request")?;
        validate_required_with_code(
            "eventId",
            request.event_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "canceledByUserId",
            request.canceled_by_user_id.as_str(),
            "invalid_friend_request",
        )?;
        validate_required_with_code(
            "canceledAt",
            request.canceled_at.as_str(),
            "invalid_friend_request",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let stored = state
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.tenant_id == tenant_id)
            .cloned()
            .ok_or_else(|| {
                ControlPlaneError::not_found(
                    "friend_request_not_found",
                    format!("friend request {request_id} was not found"),
                )
            })?;
        let existing_ordering_seq = state
            .committed_event(tenant_id, request.event_id.as_str())
            .map(|existing| existing.commit().ordering_seq);
        if !matches!(stored.friend_request.status, FriendRequestStatus::Pending)
            && existing_ordering_seq.is_none()
        {
            return Err(ControlPlaneError::conflict(
                "friend_request_not_pending",
                format!("friend request {request_id} is not pending"),
            ));
        }
        if stored.friend_request.requester_user_id != request.canceled_by_user_id {
            return Err(ControlPlaneError::invalid(
                "invalid_friend_request",
                format!("canceledByUserId must match requester user for {request_id}"),
            ));
        }

        let payload = FriendRequestCanceledPayload {
            request_id: request_id.into(),
            canceled_by_user_id: request.canceled_by_user_id.clone(),
            canceled_at: request.canceled_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friend request cancel payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::FriendRequest,
            aggregate_id: request_id,
            event_type: SocialEventType::FriendRequestCanceled,
            ordering_seq: existing_ordering_seq.unwrap_or(stored.commits.len() as u64 + 1),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.canceled_at.as_str(),
            committed_at: request.canceled_at.as_str(),
            payload: payload_json.as_str(),
        });
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::FriendRequest { record, commit } => {
                        Ok(CanceledFriendRequest {
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

        let mut next_state = state.clone();
        let mut record = next_state
            .friend_requests
            .get(request_id)
            .cloned()
            .expect("friend request should exist after validation");
        record.friend_request.status = FriendRequestStatus::Canceled;
        record.friend_request.updated_at = request.canceled_at;
        let friend_request = record.friend_request.clone();
        record.commits.push(commit.clone());
        next_state.insert_friend_request_record(request_id.to_owned(), record);

        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(CanceledFriendRequest {
            friend_request,
            latest_commit: commit,
            persistence,
        })
    }

    fn activate_friendship(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: ActivateFriendshipRequest,
    ) -> Result<ActivatedFriendship, ControlPlaneError> {
        validate_payload_size(
            "friendshipId",
            request.friendship_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "initiatorUserId",
            request.initiator_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "peerUserId",
            request.peer_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "directChatId",
            request.direct_chat_id.as_deref(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "establishedAt",
            request.established_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
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
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::Friendship,
            aggregate_id: request.friendship_id.as_str(),
            event_type: SocialEventType::FriendshipActivated,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.established_at.as_str(),
            committed_at: request.established_at.as_str(),
            payload: payload_json.as_str(),
        });
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

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
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
        if let Some(user_block) = active_friendship_scoped_user_block(
            &next_state,
            tenant_id,
            friendship.user_low_id.as_str(),
            friendship.user_high_id.as_str(),
        ) {
            return Err(ControlPlaneError::conflict_with_details(
                "friendship_blocked",
                format!(
                    "friendship pair {}:{} is blocked by {}",
                    pair.user_low_id, pair.user_high_id, user_block.block_id
                ),
                social_pair_block_conflict_details(&user_block),
            ));
        }
        if let Some(existing_friendship) = active_friendship_record_for_pair(
            &next_state,
            tenant_id,
            pair.user_low_id.as_str(),
            pair.user_high_id.as_str(),
        ) {
            return Err(ControlPlaneError::conflict_with_details(
                "friendship_pair_conflict",
                format!(
                    "active friendship already exists for pair {}:{}",
                    pair.user_low_id, pair.user_high_id
                ),
                serde_json::json!({
                    "existingFriendshipId": existing_friendship.friendship.friendship_id,
                    "existingStatus": existing_friendship.friendship.status,
                    "userLowId": existing_friendship.friendship.user_low_id,
                    "userHighId": existing_friendship.friendship.user_high_id
                }),
            ));
        }

        next_state.insert_friendship_record(
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .friendships
            .get(friendship_id)
            .filter(|record| record.friendship.tenant_id == tenant_id)
            .cloned()
    }

    fn remove_friendship(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        friendship_id: &str,
        request: RemoveFriendshipRequest,
    ) -> Result<RemovedFriendship, ControlPlaneError> {
        validate_payload_size("friendshipId", friendship_id, CONTROL_PLANE_MAX_ID_BYTES)?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "removedByUserId",
            request.removed_by_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "removedAt",
            request.removed_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
        validate_required_with_code("friendshipId", friendship_id, "invalid_friendship")?;
        validate_required_with_code("eventId", request.event_id.as_str(), "invalid_friendship")?;
        validate_required_with_code(
            "removedByUserId",
            request.removed_by_user_id.as_str(),
            "invalid_friendship",
        )?;
        validate_required_with_code(
            "removedAt",
            request.removed_at.as_str(),
            "invalid_friendship",
        )?;

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        let stored = state
            .friendships
            .get(friendship_id)
            .filter(|record| record.friendship.tenant_id == tenant_id)
            .cloned()
            .ok_or_else(|| {
                ControlPlaneError::not_found(
                    "friendship_not_found",
                    format!("friendship {friendship_id} was not found"),
                )
            })?;
        let existing_ordering_seq = state
            .committed_event(tenant_id, request.event_id.as_str())
            .map(|existing| existing.commit().ordering_seq);
        if !stored.friendship.status.is_active() && existing_ordering_seq.is_none() {
            return Err(ControlPlaneError::conflict(
                "friendship_not_active",
                format!("friendship {friendship_id} is not active"),
            ));
        }
        if request.removed_by_user_id != stored.friendship.user_low_id
            && request.removed_by_user_id != stored.friendship.user_high_id
        {
            return Err(ControlPlaneError::invalid(
                "invalid_friendship",
                format!("removedByUserId must be a friendship participant for {friendship_id}"),
            ));
        }

        let payload = FriendshipRemovedPayload {
            friendship_id: stored.friendship.friendship_id.clone(),
            user_low_id: stored.friendship.user_low_id.clone(),
            user_high_id: stored.friendship.user_high_id.clone(),
            removed_by_user_id: request.removed_by_user_id.clone(),
            removed_at: request.removed_at.clone(),
        };
        let payload_json = serde_json::to_string(&payload)
            .expect("friendship removal payload should serialize into json");
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::Friendship,
            aggregate_id: friendship_id,
            event_type: SocialEventType::FriendshipRemoved,
            ordering_seq: existing_ordering_seq.unwrap_or(stored.commits.len() as u64 + 1),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.removed_at.as_str(),
            committed_at: request.removed_at.as_str(),
            payload: payload_json.as_str(),
        });
        if let Some(replayed) =
            self.replay_committed_social_event(&state, &commit, |existing, persistence| {
                match existing {
                    SocialCommittedEvent::Friendship { record, commit } => Ok(RemovedFriendship {
                        friendship: record.friendship,
                        latest_commit: commit,
                        persistence,
                    }),
                    other => Err(social_event_id_conflict(request.event_id.as_str(), &other)),
                }
            })?
        {
            return Ok(replayed);
        }

        let mut next_state = state.clone();
        let mut record = next_state
            .friendships
            .get(friendship_id)
            .cloned()
            .expect("friendship should exist after validation");
        record.friendship.status = FriendshipStatus::Removed;
        record.friendship.updated_at = request.removed_at;
        record.commits.push(commit.clone());
        let friendship = record.friendship.clone();
        next_state.insert_friendship_record(friendship_id.to_owned(), record);
        archive_active_direct_chats_for_pair(
            &mut next_state,
            tenant_id,
            friendship.user_low_id.as_str(),
            friendship.user_high_id.as_str(),
            friendship.updated_at.as_str(),
        );

        let persistence = self.persist_state_transition(&next_state, &commit)?;
        *state = next_state;

        Ok(RemovedFriendship {
            friendship,
            latest_commit: commit,
            persistence,
        })
    }

    fn block_user(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: BlockUserRequest,
    ) -> Result<BlockedUser, ControlPlaneError> {
        validate_payload_size(
            "blockId",
            request.block_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "blockerUserId",
            request.blocker_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "blockedUserId",
            request.blocked_user_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "directChatId",
            request.direct_chat_id.as_deref(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_optional_payload_size(
            "expiresAt",
            request.expires_at.as_deref(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
        validate_payload_size(
            "effectiveAt",
            request.effective_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
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
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::UserBlock,
            aggregate_id: request.block_id.as_str(),
            event_type: SocialEventType::UserBlocked,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.effective_at.as_str(),
            committed_at: request.effective_at.as_str(),
            payload: payload_json.as_str(),
        });
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

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
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
        if let Some(direct_chat_id) = user_block.direct_chat_id.as_deref() {
            let direct_chat = next_state
                .direct_chats
                .get(direct_chat_id)
                .filter(|record| record.direct_chat.tenant_id == tenant_id)
                .filter(|record| record.direct_chat.status.is_active())
                .ok_or_else(|| {
                    ControlPlaneError::invalid(
                        "invalid_user_block",
                        format!("direct chat {direct_chat_id} does not exist or is not active"),
                    )
                })?;
            let direct_chat_pair = normalize_user_pair(
                direct_chat.direct_chat.left_actor_id.as_str(),
                direct_chat.direct_chat.right_actor_id.as_str(),
            )
            .map_err(|error| {
                ControlPlaneError::invalid(
                    "invalid_user_block",
                    format!("direct chat {direct_chat_id} cannot be used for user block: {error}"),
                )
            })?;
            let block_pair = user_block.user_pair().map_err(|error| {
                ControlPlaneError::invalid("invalid_user_block", error.to_string())
            })?;
            if direct_chat_pair != block_pair {
                return Err(ControlPlaneError::invalid(
                    "invalid_user_block",
                    format!(
                        "direct chat {direct_chat_id} does not match block pair {}",
                        block_pair.pair_key()
                    ),
                ));
            }
        }
        if active_user_block_for_scope(
            &next_state,
            tenant_id,
            user_block.blocker_user_id.as_str(),
            user_block.blocked_user_id.as_str(),
            &user_block.scope,
            user_block.direct_chat_id.as_deref(),
        )
        .is_some()
        {
            return Err(ControlPlaneError::conflict(
                "user_block_scope_conflict",
                format!(
                    "active user block already exists for {} -> {} scope {:?}",
                    user_block.blocker_user_id, user_block.blocked_user_id, user_block.scope
                ),
            ));
        }

        next_state.insert_user_block_record(
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .user_blocks
            .get(block_id)
            .filter(|record| record.user_block.tenant_id == tenant_id)
            .cloned()
    }

    fn active_direct_chat_access_block(
        &self,
        tenant_id: &str,
        direct_chat_id: &str,
    ) -> Option<UserBlock> {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        active_direct_chat_scoped_user_block(&state, tenant_id, direct_chat_id)
    }

    fn active_friendship_access_block_for_pair(
        &self,
        tenant_id: &str,
        user_a: &str,
        user_b: &str,
    ) -> Option<UserBlock> {
        let state = self
            .state
            .read()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
        active_friendship_scoped_user_block(&state, tenant_id, user_a, user_b)
    }

    fn bind_direct_chat(
        &self,
        tenant_id: &str,
        auth: &AppContext,
        request: BindDirectChatRequest,
    ) -> Result<BoundDirectChat, ControlPlaneError> {
        validate_payload_size(
            "directChatId",
            request.direct_chat_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "eventId",
            request.event_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "leftActorId",
            request.left_actor_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "rightActorId",
            request.right_actor_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "conversationId",
            request.conversation_id.as_str(),
            CONTROL_PLANE_MAX_ID_BYTES,
        )?;
        validate_payload_size(
            "boundAt",
            request.bound_at.as_str(),
            CONTROL_PLANE_MAX_TIMESTAMP_BYTES,
        )?;
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
        let commit = social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: request.event_id.as_str(),
            tenant_id,
            aggregate_type: AggregateType::DirectChat,
            aggregate_id: request.direct_chat_id.as_str(),
            event_type: SocialEventType::DirectChatBound,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: request.bound_at.as_str(),
            committed_at: request.bound_at.as_str(),
            payload: payload_json.as_str(),
        });
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

        let _write_lock = self.acquire_cross_instance_write_lock()?;
        self.refresh_state_from_authority_for_write()?;
        let mut state = self
            .state
            .write()
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock);
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
        if let Some(existing_direct_chat) = active_direct_chat_record_for_pair(
            &next_state,
            tenant_id,
            pair.left_actor_id.as_str(),
            pair.right_actor_id.as_str(),
        ) {
            return Err(ControlPlaneError::conflict_with_details(
                "direct_chat_pair_conflict",
                format!(
                    "active direct chat already exists for pair {}",
                    pair.pair_hash
                ),
                serde_json::json!({
                    "existingDirectChatId": existing_direct_chat.direct_chat.direct_chat_id,
                    "existingStatus": existing_direct_chat.direct_chat.status,
                    "leftActorId": existing_direct_chat.direct_chat.left_actor_id,
                    "rightActorId": existing_direct_chat.direct_chat.right_actor_id,
                    "conversationId": existing_direct_chat.direct_chat.conversation_id
                }),
            ));
        }

        next_state.insert_direct_chat_record(
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
            .unwrap_or_else(Self::recover_poisoned_social_runtime_lock)
            .direct_chats
            .get(direct_chat_id)
            .filter(|record| record.direct_chat.tenant_id == tenant_id)
            .cloned()
    }

    fn authoritative_state_for_query(&self) -> Result<SocialControlState, String> {
        match self.journal_path.as_deref() {
            Some(journal_path) => {
                let authority_load = Self::load_state_with_journal_replay(
                    &self.state_store,
                    journal_path,
                    self.tx_marker_path.as_deref().map(|path| path.as_path()),
                );
                if let Some(error) = authority_load.replay_error {
                    return Err(error);
                }
                Ok(authority_load.state)
            }
            None => self.state_store.load(),
        }
    }

    fn authoritative_active_friendships_for_user(
        &self,
        tenant_id: &str,
        user_id: &str,
    ) -> Result<Vec<Friendship>, String> {
        let state = self.authoritative_state_for_query()?;
        let mut friendships = active_friendship_records_for_user(&state, tenant_id, user_id)
            .into_iter()
            .map(|record| record.friendship)
            .collect::<Vec<_>>();
        friendships.sort_by(|left, right| {
            right
                .updated_at
                .cmp(&left.updated_at)
                .then_with(|| left.friendship_id.cmp(&right.friendship_id))
        });
        Ok(friendships)
    }

    fn authoritative_active_direct_chat_for_pair(
        &self,
        tenant_id: &str,
        user_low_id: &str,
        user_high_id: &str,
    ) -> Result<Option<DirectChat>, String> {
        let state = self.authoritative_state_for_query()?;
        Ok(
            active_direct_chat_record_for_pair(&state, tenant_id, user_low_id, user_high_id)
                .map(|record| record.direct_chat),
        )
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

pub fn build_dual_token_shared_channel_sync_trigger(
    base_url: impl AsRef<str>,
) -> Result<Arc<dyn SharedChannelLinkedMemberSyncTrigger>, String> {
    Ok(Arc::new(DualTokenSharedChannelLinkedMemberSyncTrigger::new(
        base_url,
    )?))
}

pub fn configured_dual_token_shared_channel_sync_trigger()
-> Result<Option<Arc<dyn SharedChannelLinkedMemberSyncTrigger>>, String> {
    let Some(base_url) = configured_shared_channel_sync_target_base_url() else {
        return Ok(None);
    };

    build_dual_token_shared_channel_sync_trigger(base_url).map(Some)
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

    let snapshot_state = match state_store.load() {
        Ok(state) => state,
        Err(error) => {
            tracing::warn!(
                "failed to load control-plane social snapshot during operator repair: {error}. continuing from commit journal authority"
            );
            SocialControlState::default()
        }
    };
    let mut replayed_state =
        SocialControlRuntime::replay_state_from_commit_journal(journal_path.as_path())?;
    replayed_state.merge_pending_shared_channel_sync_requests_from(&snapshot_state);
    replayed_state.merge_dead_letter_shared_channel_sync_requests_from(&snapshot_state);
    replayed_state.merge_delivered_shared_channel_sync_requests_from(&snapshot_state);
    replayed_state.merge_delivered_shared_channel_sync_delivery_proofs_from(&snapshot_state);
    replayed_state.merge_recent_shared_channel_sync_deliveries_from(&snapshot_state);
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
        "aggregate-counts: friendRequests={} friendships={} userBlocks={} directChats={} externalConnections={} externalMemberLinks={} sharedChannelPolicies={} pendingSharedChannelSyncRequests={} deadLetterSharedChannelSyncRequests={} deliveredSharedChannelSyncRequests={} recentSharedChannelSyncDeliveries={}",
        report.aggregate_counts.friend_requests,
        report.aggregate_counts.friendships,
        report.aggregate_counts.user_blocks,
        report.aggregate_counts.direct_chats,
        report.aggregate_counts.external_connections,
        report.aggregate_counts.external_member_links,
        report.aggregate_counts.shared_channel_policies,
        report.aggregate_counts.pending_shared_channel_sync_requests,
        report.aggregate_counts.dead_letter_shared_channel_sync_requests,
        report.aggregate_counts.delivered_shared_channel_sync_requests,
        report.aggregate_counts.recent_shared_channel_sync_deliveries
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
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
        require_dual_token_headers: resolve_require_dual_token_headers(),
    };
    build_app()
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            require_app_context,
        ))
}

pub fn export_openapi_document() -> Result<serde_json::Value, String> {
    Ok(control_plane_openapi_document())
}

pub fn export_openapi_spec() -> OpenApiServiceSpec<'static> {
    control_plane_openapi_spec()
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
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
        require_dual_token_headers: resolve_require_dual_token_headers(),
    };
    build_app_with_shared_channel_sync_trigger(shared_channel_sync_trigger)
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            require_app_context,
        ))
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

pub fn build_control_surface_with_cluster_and_governance_sinks_and_shared_channel_sync_trigger_with_social_query(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    shared_channel_sync_trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger>,
) -> (Router, Arc<SocialControlQuery>) {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    let social_runtime = build_social_runtime_from_env();
    let social_query = social_query_handle(social_runtime.clone());
    let router = build_control_surface_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
        social_runtime,
        shared_channel_sync_trigger: Some(shared_channel_sync_trigger),
    });
    (router, social_query)
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

pub fn build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger_with_social_query(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    runtime_dir: impl AsRef<StdPath>,
    shared_channel_sync_trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger>,
) -> (Router, Arc<SocialControlQuery>) {
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());
    let social_runtime = Arc::new(SocialControlRuntime::from_runtime_dir(runtime_dir));
    let social_query = social_query_handle(social_runtime.clone());
    let router = build_control_surface_with_state(AppState {
        realtime_cluster,
        protocol_registry: Arc::new(CcpRegistry::control_plane_v1()),
        provider_registry: provider_registry.clone(),
        provider_registry_runtime: Some(provider_registry),
        governance_loop: Some(GovernanceLoop {
            ops_runtime,
            audit_runtime,
        }),
        social_runtime,
        shared_channel_sync_trigger: Some(shared_channel_sync_trigger),
    });
    (router, social_query)
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
    Router::new()
        .route("/healthz", get(healthz))
        .route("/openapi.json", get(openapi_document))
        .route(
            "/backend/v3/api/control/openapi.json",
            get(openapi_document),
        )
        .route("/docs", get(docs))
        .merge(build_control_surface_with_state_and_scheduler_config(
            state,
            scheduler_config,
        ))
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
            "/backend/v3/api/control/protocol_registry",
            get(protocol_registry_snapshot),
        )
        .route(
            "/backend/v3/api/control/protocol_governance",
            get(protocol_governance_snapshot),
        )
        .route(
            "/backend/v3/api/control/provider_registry",
            get(provider_registry_snapshot),
        )
        .route(
            "/backend/v3/api/control/provider_bindings",
            get(provider_bindings_snapshot).post(upsert_provider_binding_policy),
        )
        .route(
            "/backend/v3/api/control/provider_policies",
            get(provider_policy_history),
        )
        .route(
            "/backend/v3/api/control/provider_policies/diff",
            get(provider_policy_diff),
        )
        .route(
            "/backend/v3/api/control/provider_policies/preview",
            post(provider_policy_preview),
        )
        .route(
            "/backend/v3/api/control/provider_policies/rollback",
            post(rollback_provider_policy),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests",
            get(list_friend_requests).post(submit_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}",
            get(friend_request_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}/accept",
            post(accept_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}/decline",
            post(decline_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friend_requests/{request_id}/cancel",
            post(cancel_friend_request),
        )
        .route(
            "/backend/v3/api/control/social/friendships",
            post(activate_friendship),
        )
        .route(
            "/backend/v3/api/control/social/friendships/{friendship_id}",
            get(friendship_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/friendships/{friendship_id}/remove",
            post(remove_friendship),
        )
        .route("/backend/v3/api/control/social/user_blocks", post(block_user))
        .route(
            "/backend/v3/api/control/social/user_blocks/{block_id}",
            get(user_block_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/direct_chats/bindings",
            post(bind_direct_chat),
        )
        .route(
            "/backend/v3/api/control/social/direct_chats/{direct_chat_id}",
            get(direct_chat_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/external_connections",
            post(establish_external_connection),
        )
        .route(
            "/backend/v3/api/control/social/external_connections/{connection_id}",
            get(external_connection_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/external_member_links",
            post(bind_external_member_link),
        )
        .route(
            "/backend/v3/api/control/social/external_member_links/{link_id}",
            get(external_member_link_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/shared_channel_policies",
            post(apply_shared_channel_policy),
        )
        .route(
            "/backend/v3/api/control/social/shared_channel_policies/{policy_id}",
            get(shared_channel_policy_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/runtime/repair_derived_snapshot",
            post(repair_social_runtime_snapshot),
        )
        .route(
            "/backend/v3/api/control/social/runtime/dead_letter_shared_channel_sync",
            get(dead_letter_social_runtime_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/pending_shared_channel_sync",
            get(pending_social_runtime_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/delivered_shared_channel_sync",
            get(delivered_social_runtime_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync",
            get(delivery_state_social_runtime_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/reclaim_stale_pending_shared_channel_sync",
            post(reclaim_stale_pending_social_runtime_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/repair_shared_channel_sync",
            post(repair_social_runtime_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync",
            post(requeue_dead_letter_social_runtime_shared_channel_sync),
        )
        .route(
            "/backend/v3/api/control/social/runtime/requeue_dead_letter_shared_channel_sync_targeted",
            post(requeue_dead_letter_social_runtime_shared_channel_sync_targeted),
        )
        .route(
            "/backend/v3/api/control/social/runtime/claim_pending_shared_channel_sync_targeted",
            post(claim_pending_social_runtime_shared_channel_sync_targeted),
        )
        .route(
            "/backend/v3/api/control/social/runtime/release_pending_shared_channel_sync_targeted",
            post(release_pending_social_runtime_shared_channel_sync_targeted),
        )
        .route(
            "/backend/v3/api/control/social/runtime/takeover_pending_shared_channel_sync_targeted",
            post(takeover_pending_social_runtime_shared_channel_sync_targeted),
        )
        .route(
            "/backend/v3/api/control/social/runtime/republish_pending_shared_channel_sync_targeted",
            post(republish_pending_social_runtime_shared_channel_sync_targeted),
        )
        .route("/backend/v3/api/control/nodes/{node_id}/drain", post(drain_node))
        .route(
            "/backend/v3/api/control/nodes/{node_id}/activate",
            post(activate_node),
        )
        .route(
            "/backend/v3/api/control/nodes/{node_id}/routes/migrate",
            post(migrate_node_routes),
        )
        .with_state(state)
}

async fn require_app_context(
    State(guardrails): State<PublicAppGuardrails>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    match request.uri().path() {
        "/healthz" | "/openapi.json" | "/backend/v3/api/control/openapi.json" | "/docs" => {
            next.run(request).await
        }
        _ => {
            let permit = match guardrails.request_gate.clone().try_acquire_owned() {
                Ok(permit) => permit,
                Err(_) => {
                    return ControlPlaneError::service_unavailable(
                        "http_overloaded",
                        "server is at maximum in-flight request capacity, please retry later",
                    )
                    .into_response();
                }
            };
            if guardrails.require_dual_token_headers
                && let Err(error) = require_dual_token_headers(request.headers())
            {
                return error.into_response();
            }
            let resolved = match resolve_app_context_for_request(
                request.headers(),
                request.uri().path(),
                request.method().as_str(),
            ) {
                Ok(resolved) => resolved,
                Err(error) => return ControlPlaneError::from(error).into_response(),
            };
            request
                .extensions_mut()
                .insert(resolved.app_request_context);
            request.extensions_mut().insert(resolved.app_context);
            let response = next.run(request).await;
            drop(permit);
            response
        }
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "control-plane-api",
    })
}

async fn openapi_document() -> Json<JsonValue> {
    Json(control_plane_openapi_document())
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&control_plane_openapi_spec()))
}

fn control_plane_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Control Plane API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Detailed OpenAPI contract for the Craw Chat control-plane runtime, including protocol governance, provider policy, social control workflows, and node lifecycle operations.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

async fn protocol_registry_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProtocolRegistryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProtocolGovernanceResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProviderRegistrySnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    Ok(Json(provider_registry_snapshot_response(
        state.provider_registry.snapshot(),
    )))
}

async fn provider_bindings_snapshot(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ProviderBindingsQuery>,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingsResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let tenant_id = validate_optional_tenant_id(query.tenant_id)?;

    let response = provider_bindings_response(state.provider_registry.as_ref(), tenant_id);
    mirror_provider_bindings_into_ops_runtime(&state, &response);

    Ok(Json(response))
}

async fn upsert_provider_binding_policy(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Result<Json<ProviderBindingCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let tenant_id = validate_optional_tenant_id(request.tenant_id.clone())?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_write_unavailable",
            "control plane provider policy write is not enabled for this registry",
        )
    })?;

    let (action, aggregate_id, selection_source, commit) =
        if let Some(tenant_id) = tenant_id.as_deref() {
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
    let response = provider_bindings_response(state.provider_registry.as_ref(), tenant_id);
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
                "currentVersion": commit.current_version,
                "selectionSource": selection_source
            }),
        );
    }

    Ok(Json(provider_binding_commit_response(response, commit)))
}

async fn provider_policy_history(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProviderPolicyHistoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    Query(query): Query<ProviderPolicyDiffQuery>,
    State(state): State<AppState>,
) -> Result<Json<ProviderPolicyDiffResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpsertProviderBindingPolicyRequest>,
) -> Result<Json<ProviderPolicyPreview>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;
    let tenant_id = validate_optional_tenant_id(request.tenant_id.clone())?;
    let provider_registry = state.provider_registry_runtime.as_ref().ok_or_else(|| {
        ControlPlaneError::service_unavailable(
            "provider_policy_preview_unavailable",
            "control plane provider policy preview is not enabled for this registry",
        )
    })?;

    Ok(Json(provider_registry.preview_upsert(
        tenant_id.as_deref(),
        request.domain,
        request.plugin_id.as_str(),
    )?))
}

async fn rollback_provider_policy(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<ProviderPolicyRollbackRequest>,
) -> Result<Json<ProviderPolicyHistoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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

fn friend_request_matches_inventory_direction(
    friend_request: &FriendRequest,
    user_id: &str,
    direction: FriendRequestInventoryDirectionQuery,
) -> bool {
    match direction {
        FriendRequestInventoryDirectionQuery::Incoming => friend_request.target_user_id == user_id,
        FriendRequestInventoryDirectionQuery::Outgoing => {
            friend_request.requester_user_id == user_id
        }
    }
}

fn friend_request_matches_inventory_status(
    friend_request: &FriendRequest,
    status: FriendRequestInventoryStatusQuery,
) -> bool {
    match status {
        FriendRequestInventoryStatusQuery::Pending => {
            friend_request.status == FriendRequestStatus::Pending
        }
        FriendRequestInventoryStatusQuery::Accepted => {
            friend_request.status == FriendRequestStatus::Accepted
        }
        FriendRequestInventoryStatusQuery::Declined => {
            friend_request.status == FriendRequestStatus::Declined
        }
        FriendRequestInventoryStatusQuery::Canceled => {
            friend_request.status == FriendRequestStatus::Canceled
        }
        FriendRequestInventoryStatusQuery::Expired => {
            friend_request.status == FriendRequestStatus::Expired
        }
        FriendRequestInventoryStatusQuery::All => true,
    }
}

fn compare_friend_request_inventory_order(
    left: &FriendRequest,
    right: &FriendRequest,
) -> CmpOrdering {
    compare_friend_request_inventory_sort_key(
        left.updated_at.as_str(),
        left.created_at.as_str(),
        left.request_id.as_str(),
        right.updated_at.as_str(),
        right.created_at.as_str(),
        right.request_id.as_str(),
    )
}

fn compare_friend_request_inventory_with_cursor(
    friend_request: &FriendRequest,
    cursor: &FriendRequestInventoryCursor,
) -> CmpOrdering {
    compare_friend_request_inventory_sort_key(
        friend_request.updated_at.as_str(),
        friend_request.created_at.as_str(),
        friend_request.request_id.as_str(),
        cursor.updated_at.as_str(),
        cursor.created_at.as_str(),
        cursor.request_id.as_str(),
    )
}

fn compare_friend_request_inventory_sort_key(
    left_updated_at: &str,
    left_created_at: &str,
    left_request_id: &str,
    right_updated_at: &str,
    right_created_at: &str,
    right_request_id: &str,
) -> CmpOrdering {
    right_updated_at
        .cmp(left_updated_at)
        .then_with(|| right_created_at.cmp(left_created_at))
        .then_with(|| left_request_id.cmp(right_request_id))
}

fn friend_request_inventory_cursor_for(friend_request: &FriendRequest) -> String {
    let cursor = FriendRequestInventoryCursor {
        v: SOCIAL_FRIEND_REQUEST_CURSOR_VERSION,
        updated_at: friend_request.updated_at.clone(),
        created_at: friend_request.created_at.clone(),
        request_id: friend_request.request_id.clone(),
    };
    let payload = serde_json::to_value(&cursor)
        .expect("friend request inventory cursor should serialize into json");
    let secret = resolve_friend_request_cursor_signing_secret();
    encode_signed_cursor_payload(&payload, secret.as_str())
        .expect("friend request inventory cursor should encode into signed compact token")
}

fn encode_signed_cursor_payload(
    payload: &serde_json::Value,
    secret: &str,
) -> Result<String, ControlPlaneError> {
    let header = serde_json::json!({
        "alg": "HS256",
        "typ": "cursor"
    });
    let header_bytes = serde_json::to_vec(&header).map_err(|_| {
        ControlPlaneError::service_unavailable(
            "cursor_encoding_failed",
            "cursor header could not be encoded",
        )
    })?;
    let payload_bytes = serde_json::to_vec(payload).map_err(|_| {
        ControlPlaneError::service_unavailable(
            "cursor_encoding_failed",
            "cursor payload could not be encoded",
        )
    })?;
    let header_segment = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(header_bytes);
    let payload_segment = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(payload_bytes);
    let signing_input = format!("{header_segment}.{payload_segment}");
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| {
        ControlPlaneError::service_unavailable(
            "cursor_signing_secret_invalid",
            "cursor signing secret is invalid",
        )
    })?;
    mac.update(signing_input.as_bytes());
    let signature_segment =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
    Ok(format!("{signing_input}.{signature_segment}"))
}

fn parse_friend_request_inventory_cursor(
    cursor: &str,
) -> Result<FriendRequestInventoryCursor, ControlPlaneError> {
    let payload = decode_signed_friend_request_cursor_payload(cursor)?;
    let cursor: FriendRequestInventoryCursor = serde_json::from_value(payload).map_err(|_| {
        ControlPlaneError::invalid(
            "cursor_invalid",
            "friend request cursor payload is not valid",
        )
    })?;
    if cursor.v != SOCIAL_FRIEND_REQUEST_CURSOR_VERSION {
        return Err(ControlPlaneError::invalid(
            "cursor_invalid",
            format!(
                "friend request cursor version {} is not supported",
                cursor.v
            ),
        ));
    }
    Ok(cursor)
}

fn decode_signed_friend_request_cursor_payload(
    cursor: &str,
) -> Result<serde_json::Value, ControlPlaneError> {
    let segments = cursor.split('.').collect::<Vec<_>>();
    if segments.len() != 3 {
        return Err(ControlPlaneError::invalid(
            "cursor_invalid",
            "friend request cursor must be a signed compact token",
        ));
    }
    let header_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(segments[0])
        .map_err(|_| {
            ControlPlaneError::invalid(
                "cursor_invalid",
                "friend request cursor header must be valid base64url",
            )
        })?;
    let header: serde_json::Value = serde_json::from_slice(&header_bytes).map_err(|_| {
        ControlPlaneError::invalid(
            "cursor_invalid",
            "friend request cursor header must be valid json",
        )
    })?;
    let algorithm = header
        .get("alg")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ControlPlaneError::invalid(
                "cursor_invalid",
                "friend request cursor algorithm must be HS256",
            )
        })?;
    if algorithm != "HS256" {
        return Err(ControlPlaneError::invalid(
            "cursor_invalid",
            "friend request cursor algorithm must be HS256",
        ));
    }

    let signature = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(segments[2])
        .map_err(|_| {
            ControlPlaneError::invalid(
                "cursor_invalid",
                "friend request cursor signature must be valid base64url",
            )
        })?;
    let secret = resolve_friend_request_cursor_signing_secret();
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| {
        ControlPlaneError::service_unavailable(
            "cursor_signing_secret_invalid",
            "friend request cursor signing secret is invalid",
        )
    })?;
    mac.update(format!("{}.{}", segments[0], segments[1]).as_bytes());
    mac.verify_slice(signature.as_slice()).map_err(|_| {
        ControlPlaneError::invalid(
            "cursor_invalid",
            "friend request cursor signature is invalid",
        )
    })?;

    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(segments[1])
        .map_err(|_| {
            ControlPlaneError::invalid(
                "cursor_invalid",
                "friend request cursor payload must be valid base64url",
            )
        })?;
    serde_json::from_slice(&payload).map_err(|_| {
        ControlPlaneError::invalid(
            "cursor_invalid",
            "friend request cursor payload is not valid json",
        )
    })
}

fn resolve_friend_request_cursor_signing_secret() -> String {
    if let Some(configured) = resolve_non_empty_env_secret(FRIEND_REQUEST_CURSOR_HS256_SECRET_ENV) {
        return configured;
    }

    static EPHEMERAL_SECRET: OnceLock<String> = OnceLock::new();
    EPHEMERAL_SECRET
        .get_or_init(|| {
            let mut bytes = [0u8; 32];
            if fill_random(&mut bytes).is_ok() {
                tracing::warn!(
                    "{} is unset; using ephemeral in-memory friend request cursor signing secret",
                    FRIEND_REQUEST_CURSOR_HS256_SECRET_ENV
                );
                return base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
            }
            let fallback = format!(
                "ephemeral-friend-request-cursor-secret-{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            );
            tracing::warn!(
                "failed to generate random friend request cursor signing secret; using process-local time-derived fallback"
            );
            fallback
        })
        .clone()
}

fn resolve_non_empty_env_secret(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn active_friendship_scoped_user_block(
    state: &SocialControlState,
    tenant_id: &str,
    user_a: &str,
    user_b: &str,
) -> Option<UserBlock> {
    let pair = normalize_user_pair(user_a, user_b).ok()?;
    let pair_key = SocialPairIndexKey::new(
        tenant_id,
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    );
    active_user_block_by_id(
        state,
        state.active_friendship_block_pair_index.get(&pair_key)?,
    )
}

fn active_direct_chat_scoped_user_block(
    state: &SocialControlState,
    tenant_id: &str,
    direct_chat_id: &str,
) -> Option<UserBlock> {
    let direct_chat = state
        .direct_chats
        .get(direct_chat_id)
        .filter(|record| record.direct_chat.tenant_id == tenant_id)
        .map(|record| &record.direct_chat)?;
    let pair = normalize_user_pair(
        direct_chat.left_actor_id.as_str(),
        direct_chat.right_actor_id.as_str(),
    )
    .ok()?;
    let chat_key = SocialDirectChatBlockIndexKey::new(tenant_id, direct_chat_id);
    if let Some(block_id) = state.active_direct_chat_block_chat_index.get(&chat_key)
        && let Some(user_block) = active_user_block_by_id(state, block_id)
    {
        return Some(user_block);
    }

    let pair_key = SocialPairIndexKey::new(
        tenant_id,
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    );
    active_user_block_by_id(
        state,
        state.active_direct_chat_block_pair_index.get(&pair_key)?,
    )
}

fn social_pair_block_conflict_details(user_block: &UserBlock) -> serde_json::Value {
    serde_json::json!({
        "blockId": user_block.block_id.clone(),
        "blockerUserId": user_block.blocker_user_id.clone(),
        "blockedUserId": user_block.blocked_user_id.clone(),
        "scope": user_block.scope.clone(),
        "directChatId": user_block.direct_chat_id.clone(),
    })
}

fn block_scope_index_label(scope: &BlockScope) -> &'static str {
    match scope {
        BlockScope::All => "all",
        BlockScope::Friendship => "friendship",
        BlockScope::DirectChat => "direct_chat",
    }
}

fn external_connection_kind_index_label(kind: &ExternalConnectionKind) -> &'static str {
    match kind {
        ExternalConnectionKind::SharedChannel => "shared_channel",
    }
}

fn deterministic_social_id(prefix: &str, seed: &str) -> String {
    let digest = Sha256::digest(seed.as_bytes());
    let digest = format!("{digest:x}");
    format!("{prefix}{}", &digest[..24])
}

fn friendship_pair_index_key(friendship: &Friendship) -> SocialPairIndexKey {
    SocialPairIndexKey::new(
        friendship.tenant_id.as_str(),
        friendship.user_low_id.as_str(),
        friendship.user_high_id.as_str(),
    )
}

fn direct_chat_pair_index_key(direct_chat: &DirectChat) -> SocialPairIndexKey {
    SocialPairIndexKey::new(
        direct_chat.tenant_id.as_str(),
        direct_chat.left_actor_id.as_str(),
        direct_chat.right_actor_id.as_str(),
    )
}

fn user_block_pair_index_key(user_block: &UserBlock) -> Option<SocialPairIndexKey> {
    let pair = user_block.user_pair().ok()?;
    Some(SocialPairIndexKey::new(
        user_block.tenant_id.as_str(),
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    ))
}

fn friend_request_pair_index_key(friend_request: &FriendRequest) -> Option<SocialPairIndexKey> {
    let pair = friend_request.user_pair().ok()?;
    Some(SocialPairIndexKey::new(
        friend_request.tenant_id.as_str(),
        pair.user_low_id.as_str(),
        pair.user_high_id.as_str(),
    ))
}

fn external_connection_target_index_key(
    external_connection: &ExternalConnection,
) -> SocialExternalConnectionTargetIndexKey {
    SocialExternalConnectionTargetIndexKey::new(
        external_connection.tenant_id.as_str(),
        external_connection.external_tenant_id.as_str(),
        &external_connection.connection_kind,
    )
}

fn external_member_mapping_index_key(
    external_member_link: &ExternalMemberLink,
) -> SocialExternalMemberMappingIndexKey {
    SocialExternalMemberMappingIndexKey::new(
        external_member_link.tenant_id.as_str(),
        external_member_link.connection_id.as_str(),
        external_member_link.external_member_id.as_str(),
    )
}

fn external_member_connection_index_key(
    external_member_link: &ExternalMemberLink,
) -> SocialConnectionIndexKey {
    SocialConnectionIndexKey::new(
        external_member_link.tenant_id.as_str(),
        external_member_link.connection_id.as_str(),
    )
}

fn shared_channel_policy_target_index_key(
    shared_channel_policy: &SharedChannelPolicy,
) -> SocialSharedChannelPolicyTargetIndexKey {
    SocialSharedChannelPolicyTargetIndexKey::new(
        shared_channel_policy.tenant_id.as_str(),
        shared_channel_policy.connection_id.as_str(),
        shared_channel_policy.channel_id.as_str(),
    )
}

fn shared_channel_policy_connection_index_key(
    shared_channel_policy: &SharedChannelPolicy,
) -> SocialConnectionIndexKey {
    SocialConnectionIndexKey::new(
        shared_channel_policy.tenant_id.as_str(),
        shared_channel_policy.connection_id.as_str(),
    )
}

fn remove_id_from_pair_index_bucket(
    index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    key: &SocialPairIndexKey,
    record_id: &str,
) {
    if let Some(ids) = index.get_mut(key) {
        ids.remove(record_id);
        if ids.is_empty() {
            index.remove(key);
        }
    }
}

fn remove_id_from_user_index_bucket(
    index: &mut BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    key: &SocialUserIndexKey,
    record_id: &str,
) {
    if let Some(ids) = index.get_mut(key) {
        ids.remove(record_id);
        if ids.is_empty() {
            index.remove(key);
        }
    }
}

fn remove_id_from_connection_index_bucket(
    index: &mut BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    key: &SocialConnectionIndexKey,
    record_id: &str,
) {
    if let Some(ids) = index.get_mut(key) {
        ids.remove(record_id);
        if ids.is_empty() {
            index.remove(key);
        }
    }
}

fn pending_shared_channel_retry_index_key(
    pending: &PendingSharedChannelSyncRequest,
) -> SharedChannelRetryIndexKey {
    let retry_at = pending
        .last_failed_at
        .as_deref()
        .filter(|last_failed_at| is_canonical_rfc3339_millis_utc(last_failed_at))
        .unwrap_or("");
    SharedChannelRetryIndexKey::new(retry_at)
}

fn pending_shared_channel_lease_index_key(
    pending: &PendingSharedChannelSyncRequest,
) -> Option<SharedChannelLeaseIndexKey> {
    let lease_expires_at = pending.lease_expires_at.as_deref()?;
    if is_canonical_rfc3339_millis_utc(lease_expires_at) {
        Some(SharedChannelLeaseIndexKey::new(lease_expires_at))
    } else {
        Some(SharedChannelLeaseIndexKey::new(""))
    }
}

fn remove_id_from_shared_channel_retry_index_bucket(
    index: &mut BTreeMap<SharedChannelRetryIndexKey, BTreeSet<String>>,
    key: &SharedChannelRetryIndexKey,
    request_key: &str,
) {
    if let Some(ids) = index.get_mut(key) {
        ids.remove(request_key);
        if ids.is_empty() {
            index.remove(key);
        }
    }
}

fn remove_id_from_shared_channel_lease_index_bucket(
    index: &mut BTreeMap<SharedChannelLeaseIndexKey, BTreeSet<String>>,
    key: &SharedChannelLeaseIndexKey,
    request_key: &str,
) {
    if let Some(ids) = index.get_mut(key) {
        ids.remove(request_key);
        if ids.is_empty() {
            index.remove(key);
        }
    }
}

fn index_pending_shared_channel_sync_request(
    retry_index: &mut BTreeMap<SharedChannelRetryIndexKey, BTreeSet<String>>,
    lease_index: &mut BTreeMap<SharedChannelLeaseIndexKey, BTreeSet<String>>,
    request_key: &str,
    pending: &PendingSharedChannelSyncRequest,
) {
    retry_index
        .entry(pending_shared_channel_retry_index_key(pending))
        .or_default()
        .insert(request_key.to_owned());
    if let Some(lease_key) = pending_shared_channel_lease_index_key(pending) {
        lease_index
            .entry(lease_key)
            .or_default()
            .insert(request_key.to_owned());
    }
}

fn unindex_pending_shared_channel_sync_request(
    retry_index: &mut BTreeMap<SharedChannelRetryIndexKey, BTreeSet<String>>,
    lease_index: &mut BTreeMap<SharedChannelLeaseIndexKey, BTreeSet<String>>,
    request_key: &str,
    pending: &PendingSharedChannelSyncRequest,
) {
    remove_id_from_shared_channel_retry_index_bucket(
        retry_index,
        &pending_shared_channel_retry_index_key(pending),
        request_key,
    );
    if let Some(lease_key) = pending_shared_channel_lease_index_key(pending) {
        remove_id_from_shared_channel_lease_index_bucket(lease_index, &lease_key, request_key);
    }
}

fn index_friend_request_record(
    pending_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    accepted_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    user_index: &mut BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    record: &StoredFriendRequest,
) {
    for user_id in [
        record.friend_request.requester_user_id.as_str(),
        record.friend_request.target_user_id.as_str(),
    ] {
        user_index
            .entry(SocialUserIndexKey::new(
                record.friend_request.tenant_id.as_str(),
                user_id,
            ))
            .or_default()
            .insert(record.friend_request.request_id.clone());
    }

    let Some(key) = friend_request_pair_index_key(&record.friend_request) else {
        return;
    };
    match record.friend_request.status {
        FriendRequestStatus::Pending => {
            pending_index
                .entry(key)
                .or_default()
                .insert(record.friend_request.request_id.clone());
        }
        FriendRequestStatus::Accepted => {
            accepted_index
                .entry(key)
                .or_default()
                .insert(record.friend_request.request_id.clone());
        }
        FriendRequestStatus::Declined
        | FriendRequestStatus::Canceled
        | FriendRequestStatus::Expired => {}
    }
}

fn unindex_friend_request_record(
    pending_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    accepted_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    user_index: &mut BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    record: &StoredFriendRequest,
) {
    for user_id in [
        record.friend_request.requester_user_id.as_str(),
        record.friend_request.target_user_id.as_str(),
    ] {
        remove_id_from_user_index_bucket(
            user_index,
            &SocialUserIndexKey::new(record.friend_request.tenant_id.as_str(), user_id),
            record.friend_request.request_id.as_str(),
        );
    }

    let Some(key) = friend_request_pair_index_key(&record.friend_request) else {
        return;
    };
    remove_id_from_pair_index_bucket(
        pending_index,
        &key,
        record.friend_request.request_id.as_str(),
    );
    remove_id_from_pair_index_bucket(
        accepted_index,
        &key,
        record.friend_request.request_id.as_str(),
    );
}

fn index_friendship_record(
    active_index: &mut BTreeMap<SocialPairIndexKey, String>,
    active_user_index: &mut BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    all_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    record: &StoredFriendship,
) {
    let key = friendship_pair_index_key(&record.friendship);
    all_index
        .entry(key.clone())
        .or_default()
        .insert(record.friendship.friendship_id.clone());
    if record.friendship.status.is_active() {
        active_index.insert(key, record.friendship.friendship_id.clone());
        for user_id in [
            record.friendship.user_low_id.as_str(),
            record.friendship.user_high_id.as_str(),
        ] {
            active_user_index
                .entry(SocialUserIndexKey::new(
                    record.friendship.tenant_id.as_str(),
                    user_id,
                ))
                .or_default()
                .insert(record.friendship.friendship_id.clone());
        }
    }
}

fn unindex_friendship_record(
    active_index: &mut BTreeMap<SocialPairIndexKey, String>,
    active_user_index: &mut BTreeMap<SocialUserIndexKey, BTreeSet<String>>,
    all_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    record: &StoredFriendship,
) {
    let key = friendship_pair_index_key(&record.friendship);
    if active_index
        .get(&key)
        .is_some_and(|friendship_id| friendship_id == &record.friendship.friendship_id)
    {
        active_index.remove(&key);
    }
    for user_id in [
        record.friendship.user_low_id.as_str(),
        record.friendship.user_high_id.as_str(),
    ] {
        remove_id_from_user_index_bucket(
            active_user_index,
            &SocialUserIndexKey::new(record.friendship.tenant_id.as_str(), user_id),
            record.friendship.friendship_id.as_str(),
        );
    }
    remove_id_from_pair_index_bucket(all_index, &key, record.friendship.friendship_id.as_str());
}

fn index_direct_chat_record(
    active_index: &mut BTreeMap<SocialPairIndexKey, String>,
    all_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    record: &StoredDirectChat,
) {
    let key = direct_chat_pair_index_key(&record.direct_chat);
    all_index
        .entry(key.clone())
        .or_default()
        .insert(record.direct_chat.direct_chat_id.clone());
    if record.direct_chat.status.is_active() {
        active_index.insert(key, record.direct_chat.direct_chat_id.clone());
    }
}

fn unindex_direct_chat_record(
    active_index: &mut BTreeMap<SocialPairIndexKey, String>,
    all_index: &mut BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    record: &StoredDirectChat,
) {
    let key = direct_chat_pair_index_key(&record.direct_chat);
    if active_index
        .get(&key)
        .is_some_and(|direct_chat_id| direct_chat_id == &record.direct_chat.direct_chat_id)
    {
        active_index.remove(&key);
    }
    remove_id_from_pair_index_bucket(all_index, &key, record.direct_chat.direct_chat_id.as_str());
}

fn index_social_commits(
    index: &mut BTreeMap<SocialCommittedEventIndexKey, SocialCommittedEventPointer>,
    commits: &[CommitEnvelope],
    pointer: SocialCommittedEventPointer,
) {
    for (commit_index, commit) in commits.iter().enumerate() {
        index.insert(
            SocialCommittedEventIndexKey::new(commit.tenant_id.as_str(), commit.event_id.as_str()),
            pointer.with_commit_index(commit_index),
        );
    }
}

fn index_user_block_record(
    active_scope_index: &mut BTreeMap<SocialUserBlockScopeIndexKey, String>,
    friendship_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    direct_chat_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    direct_chat_chat_index: &mut BTreeMap<SocialDirectChatBlockIndexKey, String>,
    record: &StoredUserBlock,
) {
    if !record.user_block.status.is_active() {
        return;
    }

    active_scope_index.insert(
        SocialUserBlockScopeIndexKey::new(&record.user_block),
        record.user_block.block_id.clone(),
    );

    let Some(pair_key) = user_block_pair_index_key(&record.user_block) else {
        return;
    };
    match record.user_block.scope {
        BlockScope::All => {
            friendship_pair_index.insert(pair_key.clone(), record.user_block.block_id.clone());
            direct_chat_pair_index.insert(pair_key, record.user_block.block_id.clone());
        }
        BlockScope::Friendship => {
            friendship_pair_index.insert(pair_key, record.user_block.block_id.clone());
        }
        BlockScope::DirectChat => {
            if let Some(direct_chat_id) = record.user_block.direct_chat_id.as_deref() {
                direct_chat_chat_index.insert(
                    SocialDirectChatBlockIndexKey::new(
                        record.user_block.tenant_id.as_str(),
                        direct_chat_id,
                    ),
                    record.user_block.block_id.clone(),
                );
            }
        }
    }
}

fn unindex_user_block_record(
    active_scope_index: &mut BTreeMap<SocialUserBlockScopeIndexKey, String>,
    friendship_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    direct_chat_pair_index: &mut BTreeMap<SocialPairIndexKey, String>,
    direct_chat_chat_index: &mut BTreeMap<SocialDirectChatBlockIndexKey, String>,
    record: &StoredUserBlock,
) {
    if !record.user_block.status.is_active() {
        return;
    }

    let scope_key = SocialUserBlockScopeIndexKey::new(&record.user_block);
    if active_scope_index
        .get(&scope_key)
        .is_some_and(|block_id| block_id == &record.user_block.block_id)
    {
        active_scope_index.remove(&scope_key);
    }

    let Some(pair_key) = user_block_pair_index_key(&record.user_block) else {
        return;
    };
    if friendship_pair_index
        .get(&pair_key)
        .is_some_and(|block_id| block_id == &record.user_block.block_id)
    {
        friendship_pair_index.remove(&pair_key);
    }
    if direct_chat_pair_index
        .get(&pair_key)
        .is_some_and(|block_id| block_id == &record.user_block.block_id)
    {
        direct_chat_pair_index.remove(&pair_key);
    }
    if let Some(direct_chat_id) = record.user_block.direct_chat_id.as_deref() {
        let chat_key = SocialDirectChatBlockIndexKey::new(
            record.user_block.tenant_id.as_str(),
            direct_chat_id,
        );
        if direct_chat_chat_index
            .get(&chat_key)
            .is_some_and(|block_id| block_id == &record.user_block.block_id)
        {
            direct_chat_chat_index.remove(&chat_key);
        }
    }
}

fn index_external_connection_record(
    active_target_index: &mut BTreeMap<SocialExternalConnectionTargetIndexKey, String>,
    record: &StoredExternalConnection,
) {
    if !record.external_connection.status.is_active() {
        return;
    }
    active_target_index.insert(
        external_connection_target_index_key(&record.external_connection),
        record.external_connection.connection_id.clone(),
    );
}

fn unindex_external_connection_record(
    active_target_index: &mut BTreeMap<SocialExternalConnectionTargetIndexKey, String>,
    record: &StoredExternalConnection,
) {
    if !record.external_connection.status.is_active() {
        return;
    }
    let key = external_connection_target_index_key(&record.external_connection);
    if active_target_index
        .get(&key)
        .is_some_and(|connection_id| connection_id == &record.external_connection.connection_id)
    {
        active_target_index.remove(&key);
    }
}

fn index_external_member_link_record(
    active_mapping_index: &mut BTreeMap<SocialExternalMemberMappingIndexKey, String>,
    active_connection_index: &mut BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    record: &StoredExternalMemberLink,
) {
    if !record.external_member_link.status.is_active() {
        return;
    }
    active_mapping_index.insert(
        external_member_mapping_index_key(&record.external_member_link),
        record.external_member_link.link_id.clone(),
    );
    active_connection_index
        .entry(external_member_connection_index_key(
            &record.external_member_link,
        ))
        .or_default()
        .insert(record.external_member_link.link_id.clone());
}

fn unindex_external_member_link_record(
    active_mapping_index: &mut BTreeMap<SocialExternalMemberMappingIndexKey, String>,
    active_connection_index: &mut BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    record: &StoredExternalMemberLink,
) {
    if !record.external_member_link.status.is_active() {
        return;
    }
    let key = external_member_mapping_index_key(&record.external_member_link);
    if active_mapping_index
        .get(&key)
        .is_some_and(|link_id| link_id == &record.external_member_link.link_id)
    {
        active_mapping_index.remove(&key);
    }
    remove_id_from_connection_index_bucket(
        active_connection_index,
        &external_member_connection_index_key(&record.external_member_link),
        record.external_member_link.link_id.as_str(),
    );
}

fn index_shared_channel_policy_record(
    active_target_index: &mut BTreeMap<SocialSharedChannelPolicyTargetIndexKey, String>,
    active_connection_index: &mut BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    record: &StoredSharedChannelPolicy,
) {
    if !record.shared_channel_policy.status.is_active() {
        return;
    }
    active_target_index.insert(
        shared_channel_policy_target_index_key(&record.shared_channel_policy),
        record.shared_channel_policy.policy_id.clone(),
    );
    active_connection_index
        .entry(shared_channel_policy_connection_index_key(
            &record.shared_channel_policy,
        ))
        .or_default()
        .insert(record.shared_channel_policy.policy_id.clone());
}

fn unindex_shared_channel_policy_record(
    active_target_index: &mut BTreeMap<SocialSharedChannelPolicyTargetIndexKey, String>,
    active_connection_index: &mut BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>,
    record: &StoredSharedChannelPolicy,
) {
    if !record.shared_channel_policy.status.is_active() {
        return;
    }
    let key = shared_channel_policy_target_index_key(&record.shared_channel_policy);
    if active_target_index
        .get(&key)
        .is_some_and(|policy_id| policy_id == &record.shared_channel_policy.policy_id)
    {
        active_target_index.remove(&key);
    }
    remove_id_from_connection_index_bucket(
        active_connection_index,
        &shared_channel_policy_connection_index_key(&record.shared_channel_policy),
        record.shared_channel_policy.policy_id.as_str(),
    );
}

fn first_indexed_friend_request_record_for_pair(
    state: &SocialControlState,
    index: &BTreeMap<SocialPairIndexKey, BTreeSet<String>>,
    key: &SocialPairIndexKey,
    expected_status: FriendRequestStatus,
) -> Option<StoredFriendRequest> {
    index.get(key)?.iter().find_map(|request_id| {
        state
            .friend_requests
            .get(request_id)
            .filter(|record| record.friend_request.status == expected_status)
            .cloned()
    })
}

fn open_friend_request_record_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    user_low_id: &str,
    user_high_id: &str,
    pair_has_materialized_friendship: bool,
) -> Option<StoredFriendRequest> {
    let key = SocialPairIndexKey::new(tenant_id, user_low_id, user_high_id);
    first_indexed_friend_request_record_for_pair(
        state,
        &state.pending_friend_request_pair_index,
        &key,
        FriendRequestStatus::Pending,
    )
    .or_else(|| {
        if pair_has_materialized_friendship {
            None
        } else {
            first_indexed_friend_request_record_for_pair(
                state,
                &state.accepted_friend_request_pair_index,
                &key,
                FriendRequestStatus::Accepted,
            )
        }
    })
}

fn friend_request_records_for_user(
    state: &SocialControlState,
    tenant_id: &str,
    user_id: &str,
) -> Vec<StoredFriendRequest> {
    let key = SocialUserIndexKey::new(tenant_id, user_id);
    state
        .friend_request_user_index
        .get(&key)
        .into_iter()
        .flat_map(|request_ids| request_ids.iter())
        .filter_map(|request_id| {
            state
                .friend_requests
                .get(request_id)
                .filter(|record| record.friend_request.tenant_id == tenant_id)
                .cloned()
        })
        .collect()
}

fn active_external_connection_record_for_target(
    state: &SocialControlState,
    tenant_id: &str,
    external_tenant_id: &str,
    connection_kind: &ExternalConnectionKind,
) -> Option<StoredExternalConnection> {
    let key =
        SocialExternalConnectionTargetIndexKey::new(tenant_id, external_tenant_id, connection_kind);
    state
        .external_connections
        .get(state.active_external_connection_target_index.get(&key)?)
        .filter(|record| record.external_connection.status.is_active())
        .cloned()
}

fn active_external_member_link_record_for_mapping(
    state: &SocialControlState,
    tenant_id: &str,
    connection_id: &str,
    external_member_id: &str,
) -> Option<StoredExternalMemberLink> {
    let key =
        SocialExternalMemberMappingIndexKey::new(tenant_id, connection_id, external_member_id);
    state
        .external_member_links
        .get(state.active_external_member_mapping_index.get(&key)?)
        .filter(|record| record.external_member_link.status.is_active())
        .cloned()
}

fn active_shared_channel_policy_record_for_target(
    state: &SocialControlState,
    tenant_id: &str,
    connection_id: &str,
    channel_id: &str,
) -> Option<StoredSharedChannelPolicy> {
    let key = SocialSharedChannelPolicyTargetIndexKey::new(tenant_id, connection_id, channel_id);
    state
        .shared_channel_policies
        .get(state.active_shared_channel_policy_target_index.get(&key)?)
        .filter(|record| record.shared_channel_policy.status.is_active())
        .cloned()
}

fn active_external_member_link_records_for_connection(
    state: &SocialControlState,
    tenant_id: &str,
    connection_id: &str,
) -> Vec<StoredExternalMemberLink> {
    let key = SocialConnectionIndexKey::new(tenant_id, connection_id);
    state
        .active_external_member_connection_index
        .get(&key)
        .into_iter()
        .flat_map(|link_ids| link_ids.iter())
        .filter_map(|link_id| {
            state
                .external_member_links
                .get(link_id)
                .filter(|record| record.external_member_link.status.is_active())
                .cloned()
        })
        .collect()
}

fn active_shared_channel_policy_records_for_connection(
    state: &SocialControlState,
    tenant_id: &str,
    connection_id: &str,
) -> Vec<StoredSharedChannelPolicy> {
    let key = SocialConnectionIndexKey::new(tenant_id, connection_id);
    state
        .active_shared_channel_policy_connection_index
        .get(&key)
        .into_iter()
        .flat_map(|policy_ids| policy_ids.iter())
        .filter_map(|policy_id| {
            state
                .shared_channel_policies
                .get(policy_id)
                .filter(|record| record.shared_channel_policy.status.is_active())
                .cloned()
        })
        .collect()
}

fn active_friendship_record_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    user_low_id: &str,
    user_high_id: &str,
) -> Option<StoredFriendship> {
    let key = SocialPairIndexKey::new(tenant_id, user_low_id, user_high_id);
    state
        .friendships
        .get(state.active_friendship_pair_index.get(&key)?.as_str())
        .filter(|record| record.friendship.status.is_active())
        .cloned()
}

fn active_friendship_records_for_user(
    state: &SocialControlState,
    tenant_id: &str,
    user_id: &str,
) -> Vec<StoredFriendship> {
    let key = SocialUserIndexKey::new(tenant_id, user_id);
    state
        .active_friendship_user_index
        .get(&key)
        .into_iter()
        .flat_map(|friendship_ids| friendship_ids.iter())
        .filter_map(|friendship_id| {
            state
                .friendships
                .get(friendship_id)
                .filter(|record| record.friendship.status.is_active())
                .cloned()
        })
        .collect()
}

fn friendship_pair_has_materialized_record(
    state: &SocialControlState,
    tenant_id: &str,
    user_low_id: &str,
    user_high_id: &str,
) -> bool {
    state
        .friendship_pair_index
        .contains_key(&SocialPairIndexKey::new(
            tenant_id,
            user_low_id,
            user_high_id,
        ))
}

fn active_direct_chat_record_for_pair(
    state: &SocialControlState,
    tenant_id: &str,
    left_actor_id: &str,
    right_actor_id: &str,
) -> Option<StoredDirectChat> {
    let actor_pair = SocialPairIndexKey::new(tenant_id, left_actor_id, right_actor_id);
    state
        .direct_chats
        .get(
            state
                .active_direct_chat_pair_index
                .get(&actor_pair)?
                .as_str(),
        )
        .filter(|record| record.direct_chat.status.is_active())
        .cloned()
}

fn active_user_block_for_scope(
    state: &SocialControlState,
    tenant_id: &str,
    blocker_user_id: &str,
    blocked_user_id: &str,
    scope: &BlockScope,
    direct_chat_id: Option<&str>,
) -> Option<StoredUserBlock> {
    let probe = UserBlock {
        tenant_id: tenant_id.to_owned(),
        block_id: String::new(),
        blocker_user_id: blocker_user_id.to_owned(),
        blocked_user_id: blocked_user_id.to_owned(),
        scope: scope.clone(),
        status: UserBlockStatus::Active,
        direct_chat_id: direct_chat_id.map(ToOwned::to_owned),
        expires_at: None,
        created_at: String::new(),
        updated_at: String::new(),
    };
    state
        .user_blocks
        .get(
            state
                .active_user_block_scope_index
                .get(&SocialUserBlockScopeIndexKey::new(&probe))?,
        )
        .filter(|record| record.user_block.status.is_active())
        .cloned()
}

fn active_user_block_by_id(state: &SocialControlState, block_id: &str) -> Option<UserBlock> {
    state
        .user_blocks
        .get(block_id)
        .filter(|record| record.user_block.status.is_active())
        .map(|record| record.user_block.clone())
}

fn archive_active_direct_chats_for_pair(
    state: &mut SocialControlState,
    tenant_id: &str,
    user_low_id: &str,
    user_high_id: &str,
    archived_at: &str,
) {
    let pair_hash = normalize_actor_pair(user_low_id, user_high_id)
        .expect("validated friendship pair should normalize into direct chat pair")
        .pair_hash;
    let actor_pair = normalize_actor_pair(user_low_id, user_high_id)
        .expect("validated friendship pair should normalize into direct chat pair");
    let index_key = SocialPairIndexKey::new(
        tenant_id,
        actor_pair.left_actor_id.as_str(),
        actor_pair.right_actor_id.as_str(),
    );
    let direct_chat_ids = state
        .direct_chat_pair_index
        .get(&index_key)
        .cloned()
        .unwrap_or_default();
    for direct_chat_id in direct_chat_ids {
        let Some(mut record) = state.direct_chats.get(direct_chat_id.as_str()).cloned() else {
            continue;
        };
        if record.direct_chat.pair_hash != pair_hash || !record.direct_chat.status.is_active() {
            continue;
        }
        record.direct_chat.status = DirectChatStatus::Archived;
        record.direct_chat.updated_at = archived_at.to_owned();
        state.insert_direct_chat_record(direct_chat_id, record);
    }
}

async fn list_friend_requests(
    Query(query): Query<FriendRequestInventoryQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendRequestInventoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    validate_payload_size("userId", query.user_id.as_str(), CONTROL_PLANE_MAX_ID_BYTES)?;
    validate_required_with_code(
        "userId",
        query.user_id.as_str(),
        "invalid_friend_request_query",
    )?;
    let limit = query
        .limit
        .unwrap_or(SOCIAL_FRIEND_REQUEST_LIST_DEFAULT_LIMIT);
    if limit == 0 || limit > SOCIAL_FRIEND_REQUEST_LIST_MAX_LIMIT {
        return Err(ControlPlaneError::invalid(
            "limit_invalid",
            format!("limit must be between 1 and {SOCIAL_FRIEND_REQUEST_LIST_MAX_LIMIT}"),
        ));
    }
    let cursor = if let Some(cursor) = query.cursor.as_deref() {
        validate_payload_size(
            "cursor",
            cursor,
            SOCIAL_FRIEND_REQUEST_LIST_MAX_CURSOR_BYTES,
        )?;
        Some(parse_friend_request_inventory_cursor(cursor)?)
    } else {
        None
    };

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
    let page = state.social_runtime.list_friend_requests(
        auth.tenant_id.as_str(),
        query.user_id.as_str(),
        query.direction,
        query.status,
        limit,
        cursor.as_ref(),
    );

    Ok(Json(SocialFriendRequestInventoryResponse {
        status: SocialFriendRequestReadStatus::Inventory,
        items: page.items,
        next_cursor: page.next_cursor,
    }))
}

async fn submit_friend_request(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<SubmitFriendRequestRequest>,
) -> Result<Json<SocialFriendRequestCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
        friendship: None,
        friendship_latest_commit: None,
        direct_chat: None,
        direct_chat_latest_commit: None,
    }))
}

async fn accept_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<AcceptFriendRequestRequest>,
) -> Result<Json<SocialFriendRequestCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;

    let accepted = state.social_runtime.accept_friend_request(
        auth.tenant_id.as_str(),
        &auth,
        request_id.as_str(),
        request,
    )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.friend_request_accepted",
        "friend_request",
        accepted.friend_request.request_id.clone(),
        serde_json::json!({
            "requestId": accepted.friend_request.request_id,
            "requesterUserId": accepted.friend_request.requester_user_id,
            "targetUserId": accepted.friend_request.target_user_id,
            "eventId": accepted.latest_commit.event_id
        }),
    );
    if let (Some(friendship), Some(friendship_commit)) = (
        accepted.friendship.as_ref(),
        accepted.friendship_materialized_commit.as_ref(),
    ) {
        record_control_plane_audit(
            &state,
            &auth,
            "control.friendship_activated",
            "friendship",
            friendship.friendship_id.clone(),
            serde_json::json!({
                "friendshipId": friendship.friendship_id,
                "userLowId": friendship.user_low_id,
                "userHighId": friendship.user_high_id,
                "eventId": friendship_commit.event_id
            }),
        );
    }
    if let (Some(direct_chat), Some(direct_chat_commit)) = (
        accepted.direct_chat.as_ref(),
        accepted.direct_chat_materialized_commit.as_ref(),
    ) {
        record_control_plane_audit(
            &state,
            &auth,
            "control.direct_chat_bound",
            "direct_chat",
            direct_chat.direct_chat_id.clone(),
            serde_json::json!({
                "directChatId": direct_chat.direct_chat_id,
                "leftActorId": direct_chat.left_actor_id,
                "rightActorId": direct_chat.right_actor_id,
                "conversationId": direct_chat.conversation_id,
                "eventId": direct_chat_commit.event_id
            }),
        );
    }

    Ok(Json(SocialFriendRequestCommitResponse {
        status: SocialFriendRequestWriteStatus::Accepted,
        friend_request: accepted.friend_request,
        latest_commit: accepted.latest_commit.into(),
        persistence: accepted.persistence,
        friendship: accepted.friendship,
        friendship_latest_commit: accepted.friendship_materialized_commit.map(Into::into),
        direct_chat: accepted.direct_chat,
        direct_chat_latest_commit: accepted.direct_chat_materialized_commit.map(Into::into),
    }))
}

async fn decline_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<DeclineFriendRequestRequest>,
) -> Result<Json<SocialFriendRequestCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;

    let declined = state.social_runtime.decline_friend_request(
        auth.tenant_id.as_str(),
        &auth,
        request_id.as_str(),
        request,
    )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.friend_request_declined",
        "friend_request",
        declined.friend_request.request_id.clone(),
        serde_json::json!({
            "requestId": declined.friend_request.request_id,
            "requesterUserId": declined.friend_request.requester_user_id,
            "targetUserId": declined.friend_request.target_user_id,
            "eventId": declined.latest_commit.event_id
        }),
    );

    Ok(Json(SocialFriendRequestCommitResponse {
        status: SocialFriendRequestWriteStatus::Declined,
        friend_request: declined.friend_request,
        latest_commit: declined.latest_commit.into(),
        persistence: declined.persistence,
        friendship: None,
        friendship_latest_commit: None,
        direct_chat: None,
        direct_chat_latest_commit: None,
    }))
}

async fn cancel_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<CancelFriendRequestRequest>,
) -> Result<Json<SocialFriendRequestCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;

    let canceled = state.social_runtime.cancel_friend_request(
        auth.tenant_id.as_str(),
        &auth,
        request_id.as_str(),
        request,
    )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.friend_request_canceled",
        "friend_request",
        canceled.friend_request.request_id.clone(),
        serde_json::json!({
            "requestId": canceled.friend_request.request_id,
            "requesterUserId": canceled.friend_request.requester_user_id,
            "targetUserId": canceled.friend_request.target_user_id,
            "eventId": canceled.latest_commit.event_id
        }),
    );

    Ok(Json(SocialFriendRequestCommitResponse {
        status: SocialFriendRequestWriteStatus::Canceled,
        friend_request: canceled.friend_request,
        latest_commit: canceled.latest_commit.into(),
        persistence: canceled.persistence,
        friendship: None,
        friendship_latest_commit: None,
        direct_chat: None,
        direct_chat_latest_commit: None,
    }))
}

async fn friend_request_snapshot(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendRequestSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<ActivateFriendshipRequest>,
) -> Result<Json<SocialFriendshipCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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

async fn remove_friendship(
    Path(friendship_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<RemoveFriendshipRequest>,
) -> Result<Json<SocialFriendshipCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_write_access(&auth)?;

    let removed = state.social_runtime.remove_friendship(
        auth.tenant_id.as_str(),
        &auth,
        friendship_id.as_str(),
        request,
    )?;
    record_control_plane_audit(
        &state,
        &auth,
        "control.friendship_removed",
        "friendship",
        removed.friendship.friendship_id.clone(),
        serde_json::json!({
            "friendshipId": removed.friendship.friendship_id,
            "userLowId": removed.friendship.user_low_id,
            "userHighId": removed.friendship.user_high_id,
            "updatedAt": removed.friendship.updated_at,
            "eventId": removed.latest_commit.event_id
        }),
    );

    Ok(Json(SocialFriendshipCommitResponse {
        status: SocialFriendshipWriteStatus::Removed,
        friendship: removed.friendship,
        latest_commit: removed.latest_commit.into(),
        persistence: removed.persistence,
    }))
}

async fn friendship_snapshot(
    Path(friendship_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendshipSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<BlockUserRequest>,
) -> Result<Json<SocialUserBlockCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialUserBlockSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<BindDirectChatRequest>,
) -> Result<Json<SocialDirectChatCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialDirectChatSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<EstablishExternalConnectionRequest>,
) -> Result<Json<SocialExternalConnectionCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialExternalConnectionSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<BindExternalMemberLinkRequest>,
) -> Result<Json<SocialExternalMemberLinkCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialExternalMemberLinkSnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<ApplySharedChannelPolicyRequest>,
) -> Result<Json<SocialSharedChannelPolicyCommitResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelPolicySnapshotResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;

    let _read_lock = state.social_runtime.acquire_cross_instance_read_lock()?;
    state
        .social_runtime
        .refresh_state_from_authority_for_write()?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialRuntimeRepairResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    Query(query): Query<SharedChannelSyncInventoryQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeadLetterInventoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let can_takeover = auth.has_permission("control.write");
    let page = parse_shared_channel_sync_inventory_page_spec(&query)?;

    Ok(Json(
        state
            .social_runtime
            .dead_letter_shared_channel_sync_inventory(
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                can_takeover,
                &page,
            ),
    ))
}

async fn pending_social_runtime_shared_channel_sync(
    Query(query): Query<SharedChannelSyncInventoryQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncPendingInventoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let can_takeover = auth.has_permission("control.write");
    let page = parse_shared_channel_sync_inventory_page_spec(&query)?;

    Ok(Json(
        state.social_runtime.pending_shared_channel_sync_inventory(
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            can_takeover,
            &page,
        ),
    ))
}

async fn delivered_social_runtime_shared_channel_sync(
    Query(query): Query<SharedChannelSyncInventoryQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeliveredInventoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let page = parse_shared_channel_sync_inventory_page_spec(&query)?;

    Ok(Json(
        state
            .social_runtime
            .delivered_shared_channel_sync_inventory(&page),
    ))
}

async fn delivery_state_social_runtime_shared_channel_sync(
    Query(query): Query<SharedChannelSyncInventoryQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeliveryStateInventoryResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_control_read_access(&auth)?;
    let page = parse_shared_channel_sync_inventory_page_spec(&query)?;

    Ok(Json(
        state
            .social_runtime
            .shared_channel_sync_delivery_state_inventory(&page),
    ))
}

async fn reclaim_stale_pending_social_runtime_shared_channel_sync(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncPendingStaleReclaimResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncRepairResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialSharedChannelSyncDeadLetterRequeueResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncDeadLetterTargetedRequeueRequest>,
) -> Result<Json<SocialSharedChannelSyncDeadLetterTargetedRequeueResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncPendingTargetedClaimRequest>,
) -> Result<Json<SocialSharedChannelSyncPendingClaimResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncPendingTargetedReleaseRequest>,
) -> Result<Json<SocialSharedChannelSyncPendingReleaseResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncPendingTargetedTakeoverRequest>,
) -> Result<Json<SocialSharedChannelSyncPendingTakeoverResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<SocialSharedChannelSyncTargetedRepublishRequest>,
) -> Result<Json<SocialSharedChannelSyncTargetedRepublishResponse>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<RealtimeNodeLifecycleView>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<MigrateRoutesRequest>,
) -> Result<Json<RealtimeRouteMigrationResult>, ControlPlaneError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: &AppContext,
    requests: &[SharedChannelLinkedMemberSyncRequest],
) -> Result<(), ControlPlaneError> {
    if requests.is_empty() {
        return Ok(());
    }

    state
        .social_runtime
        .prune_delivered_shared_channel_sync_backlog_if_any(
            "failed to persist shared-channel sync delivered-backlog pruning before dispatch",
        )?;

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
        match trigger.trigger_with_delivery_proof(request.clone()) {
            Ok(delivery_proof) => {
                state
                    .social_runtime
                    .clear_pending_shared_channel_sync_request_and_record_delivery(
                        request,
                        Some(&delivery_proof),
                    )
                    .map_err(|error| {
                        let request_key = shared_channel_sync_request_key(request);
                        ControlPlaneError::service_unavailable(
                            "shared_channel_sync_delivery_state_unavailable",
                            format!(
                                "shared-channel sync request {request_key} was dispatched but delivered state persistence failed: {}",
                                error.message
                            ),
                        )
                    })?;
                record_control_plane_audit(
                    state,
                    auth,
                    "control.shared_channel_linked_member_sync_triggered",
                    "shared_channel_sync",
                    shared_channel_sync_audit_aggregate_id(request),
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
    auth: &AppContext,
    action: &str,
    aggregate_type: &str,
    aggregate_id: String,
    payload: serde_json::Value,
) {
    let Some(governance_loop) = &state.governance_loop else {
        return;
    };
    let record_id = control_plane_audit_record_id();
    let payload =
        serde_json::to_string(&payload).expect("control plane audit payload should serialize");
    if let Err(error) = governance_loop.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id,
            aggregate_type: aggregate_type.into(),
            aggregate_id,
            action: action.into(),
            payload: Some(payload),
        },
    ) {
        tracing::warn!("control-plane audit write failed for {aggregate_type}/{action}: {error:?}");
    }
}

fn control_plane_audit_record_id() -> String {
    let recorded_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let sequence = CONTROL_PLANE_AUDIT_RECORD_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    format!("control-audit-{recorded_nanos:x}-{sequence:x}")
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

fn ensure_control_write_access(auth: &AppContext) -> Result<(), ControlPlaneError> {
    if auth.has_permission("control.write") {
        return Ok(());
    }

    Err(ControlPlaneError::forbidden("control.write"))
}

fn ensure_control_read_access(auth: &AppContext) -> Result<(), ControlPlaneError> {
    if auth.has_permission("control.read") || auth.has_permission("control.write") {
        return Ok(());
    }

    Err(ControlPlaneError::forbidden("control.read"))
}

fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, ControlPlaneError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(ControlPlaneError::from),
    }
}

fn require_dual_token_headers(headers: &HeaderMap) -> Result<(), ControlPlaneError> {
    if !has_bearer_auth_token(headers) {
        return Err(ControlPlaneError {
            status: StatusCode::UNAUTHORIZED,
            code: "auth_token_missing",
            message: "authorization header must provide a bearer token".to_owned(),
            details: None,
        });
    }
    if !has_access_token_header(headers) {
        return Err(ControlPlaneError {
            status: StatusCode::UNAUTHORIZED,
            code: "access_token_missing",
            message: "access-token header is required".to_owned(),
            details: None,
        });
    }
    Ok(())
}

fn has_bearer_auth_token(headers: &HeaderMap) -> bool {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .and_then(|value| {
            let (scheme, token) = value.split_once(' ')?;
            if scheme.eq_ignore_ascii_case("bearer") && !token.trim().is_empty() {
                return Some(());
            }
            None
        })
        .is_some()
}

fn has_access_token_header(headers: &HeaderMap) -> bool {
    headers
        .get("access-token")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .is_some_and(|value| !value.is_empty())
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(CONTROL_PLANE_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(CONTROL_PLANE_MAX_REQUEST_BODY_BYTES_MAX)
}

fn resolve_require_dual_token_headers() -> bool {
    std::env::var(CONTROL_PLANE_REQUIRE_DUAL_TOKEN_HEADERS_ENV)
        .ok()
        .map(|value| parse_truthy_env_flag(Some(value)))
        .unwrap_or(true)
}

fn parse_truthy_env_flag(raw: Option<String>) -> bool {
    raw.as_deref().map(str::trim).is_some_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
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
        im_sdk_family: "sdkwork-im-sdk",
        app_sdk_family: "sdkwork-im-app-sdk",
        backend_sdk_family: "sdkwork-im-backend-sdk",
        rtc_sdk_family: "sdkwork-rtc-sdk",
        matrix_client_types: registry.compatibility_matrix().keys().cloned().collect(),
        protocol_registry_path: "/backend/v3/api/control/protocol_registry",
        protocol_governance_path: "/backend/v3/api/control/protocol_governance",
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

fn validate_optional_tenant_id(
    tenant_id: Option<String>,
) -> Result<Option<String>, ControlPlaneError> {
    if let Some(tenant_id) = tenant_id {
        validate_required_with_code("tenantId", tenant_id.as_str(), "invalid_provider_policy")?;
        validate_payload_size("tenantId", tenant_id.as_str(), CONTROL_PLANE_MAX_ID_BYTES)?;
        return Ok(Some(tenant_id));
    }

    Ok(None)
}

fn validate_payload_size(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), ControlPlaneError> {
    let actual_bytes = value.len();
    if actual_bytes > max_bytes {
        return Err(ControlPlaneError::payload_too_large(
            field,
            max_bytes,
            actual_bytes,
        ));
    }

    Ok(())
}

fn validate_optional_payload_size(
    field: &'static str,
    value: Option<&str>,
    max_bytes: usize,
) -> Result<(), ControlPlaneError> {
    if let Some(value) = value {
        validate_payload_size(field, value, max_bytes)?;
    }

    Ok(())
}

fn validate_request_keys_payload(
    field: &'static str,
    request_keys: &[String],
) -> Result<(), ControlPlaneError> {
    if request_keys.len() > CONTROL_PLANE_MAX_REQUEST_KEYS {
        return Err(ControlPlaneError::payload_too_many_items(
            field,
            CONTROL_PLANE_MAX_REQUEST_KEYS,
            request_keys.len(),
        ));
    }

    let total_bytes = request_keys.iter().fold(0usize, |total, request_key| {
        total.saturating_add(request_key.len())
    });
    if total_bytes > CONTROL_PLANE_MAX_REQUEST_KEYS_TOTAL_BYTES {
        return Err(ControlPlaneError::payload_too_large(
            field,
            CONTROL_PLANE_MAX_REQUEST_KEYS_TOTAL_BYTES,
            total_bytes,
        ));
    }

    for request_key in request_keys {
        validate_payload_size(
            field,
            request_key.as_str(),
            CONTROL_PLANE_MAX_REQUEST_KEY_BYTES,
        )?;
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue};

    use super::*;
    use std::sync::{Mutex, OnceLock};

    struct ScopedEnvVar {
        name: &'static str,
        previous: Option<String>,
    }

    impl ScopedEnvVar {
        fn set(name: &'static str, value: &str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::set_var(name, value);
            }
            Self { name, previous }
        }

        fn remove(name: &'static str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::remove_var(name);
            }
            Self { name, previous }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                unsafe {
                    std::env::set_var(self.name, previous);
                }
            } else {
                unsafe {
                    std::env::remove_var(self.name);
                }
            }
        }
    }

    fn scheduler_env_guard() -> std::sync::MutexGuard<'static, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("scheduler env guard should lock")
    }

    #[test]
    fn test_unix_epoch_seconds_clamps_pre_epoch_time_to_zero() {
        let before_epoch = UNIX_EPOCH
            .checked_sub(std::time::Duration::from_secs(1))
            .expect("test pre-epoch timestamp should construct");
        assert_eq!(unix_epoch_seconds(before_epoch), 0);
    }

    #[test]
    fn test_unix_epoch_seconds_preserves_post_epoch_time() {
        let after_epoch = UNIX_EPOCH + std::time::Duration::from_secs(42);
        assert_eq!(unix_epoch_seconds(after_epoch), 42);
    }

    #[test]
    fn test_shared_channel_stale_reclaim_scheduler_is_enabled_by_default() {
        let _guard = scheduler_env_guard();
        let _enabled =
            ScopedEnvVar::remove(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED_ENV);
        let _interval =
            ScopedEnvVar::remove(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS_ENV);
        let _jitter =
            ScopedEnvVar::remove(SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS_ENV);

        let config = resolve_shared_channel_sync_stale_reclaim_scheduler_config_from_env();
        assert!(config.enabled);
        assert_eq!(
            config.interval_millis,
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_INTERVAL_MILLIS
        );
        assert_eq!(
            config.jitter_millis,
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_DEFAULT_JITTER_MILLIS
        );
    }

    #[test]
    fn parse_truthy_env_flag_accepts_common_truthy_values() {
        for value in ["1", "true", "TRUE", " yes ", "On"] {
            assert!(parse_truthy_env_flag(Some(value.to_owned())));
        }
        for value in ["0", "false", "off", "no", "", "  "] {
            assert!(!parse_truthy_env_flag(Some(value.to_owned())));
        }
        assert!(!parse_truthy_env_flag(None));
    }

    #[test]
    fn dual_token_header_helpers_validate_auth_and_access_headers() {
        let mut headers = HeaderMap::new();
        assert!(!has_bearer_auth_token(&headers));
        assert!(!has_access_token_header(&headers));

        headers.insert(
            axum::http::header::AUTHORIZATION,
            HeaderValue::from_static("Bearer token"),
        );
        assert!(has_bearer_auth_token(&headers));
        assert!(!has_access_token_header(&headers));
        let error =
            require_dual_token_headers(&headers).expect_err("access-token should be required");
        assert_eq!(error.status, StatusCode::UNAUTHORIZED);
        assert_eq!(error.code, "access_token_missing");

        headers.insert("access-token", HeaderValue::from_static("access"));
        assert!(has_access_token_header(&headers));
        require_dual_token_headers(&headers).expect("dual token headers should pass");
    }

    #[test]
    fn test_shared_channel_stale_reclaim_scheduler_can_be_disabled_with_env() {
        let _guard = scheduler_env_guard();
        let _enabled = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED_ENV,
            "false",
        );
        let _interval = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS_ENV,
            "1200",
        );
        let _jitter = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS_ENV,
            "33",
        );

        let config = resolve_shared_channel_sync_stale_reclaim_scheduler_config_from_env();
        assert!(!config.enabled);
        assert_eq!(config.interval_millis, 1200);
        assert_eq!(config.jitter_millis, 33);
    }

    #[test]
    fn test_shared_channel_stale_reclaim_scheduler_interval_is_clamped_to_minimum() {
        let _guard = scheduler_env_guard();
        let _interval = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS_ENV,
            "1",
        );

        let config = resolve_shared_channel_sync_stale_reclaim_scheduler_config_from_env();
        assert_eq!(config.interval_millis, 1_000);
    }

    #[test]
    fn test_shared_channel_stale_reclaim_scheduler_interval_is_clamped_to_maximum() {
        let _guard = scheduler_env_guard();
        let _interval = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS_ENV,
            "99999999",
        );

        let config = resolve_shared_channel_sync_stale_reclaim_scheduler_config_from_env();
        assert_eq!(config.interval_millis, 600_000);
    }

    #[test]
    fn test_shared_channel_stale_reclaim_scheduler_jitter_is_clamped_to_maximum() {
        let _guard = scheduler_env_guard();
        let _jitter = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS_ENV,
            "999999",
        );

        let config = resolve_shared_channel_sync_stale_reclaim_scheduler_config_from_env();
        assert_eq!(
            config.jitter_millis,
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MAX_JITTER_MILLIS
        );
    }

    #[test]
    fn test_shared_channel_stale_reclaim_scheduler_sleep_duration_includes_bounded_jitter() {
        let config = SharedChannelSyncStaleReclaimSchedulerConfig {
            enabled: true,
            interval_millis: 30_000,
            jitter_millis: 200,
        };
        let now = UNIX_EPOCH + std::time::Duration::from_millis(30_123);
        let expected_jitter = 30_123 % (200 + 1);
        let with_jitter = config.tick_sleep_duration_at(now);
        assert_eq!(with_jitter.as_millis(), 30_000 + expected_jitter as u128);

        let without_jitter = SharedChannelSyncStaleReclaimSchedulerConfig {
            enabled: true,
            interval_millis: 30_000,
            jitter_millis: 0,
        }
        .tick_sleep_duration_at(UNIX_EPOCH + std::time::Duration::from_millis(30_123));
        assert_eq!(without_jitter.as_millis(), 30_000);
    }

    #[test]
    fn test_shared_channel_stale_reclaim_scheduler_explicit_config_is_clamped_to_safe_bounds() {
        let normalized = SharedChannelSyncStaleReclaimSchedulerConfig {
            enabled: true,
            interval_millis: 1,
            jitter_millis: 999_999,
        }
        .with_normalized_values();
        assert_eq!(normalized.interval_millis, 1_000);
        assert_eq!(
            normalized.jitter_millis,
            SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_MAX_JITTER_MILLIS
        );

        let normalized_high = SharedChannelSyncStaleReclaimSchedulerConfig {
            enabled: true,
            interval_millis: 99_999_999,
            jitter_millis: 20,
        }
        .with_normalized_values();
        assert_eq!(normalized_high.interval_millis, 600_000);
        assert_eq!(normalized_high.jitter_millis, 20);
    }

    #[test]
    fn test_shared_channel_delivered_ledger_limits_resolve_from_env() {
        let _guard = scheduler_env_guard();
        let _retention =
            ScopedEnvVar::remove(SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS_ENV);
        let _max_entries =
            ScopedEnvVar::remove(SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_ENV);

        assert_eq!(
            resolve_shared_channel_sync_delivered_ledger_retention_millis(),
            SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_DEFAULT_MILLIS
        );
        assert_eq!(
            resolve_shared_channel_sync_delivered_ledger_max_entries(),
            SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_DEFAULT
        );

        let _retention = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS_ENV,
            "12345",
        );
        let _max_entries =
            ScopedEnvVar::set(SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_ENV, "77");
        assert_eq!(
            resolve_shared_channel_sync_delivered_ledger_retention_millis(),
            12345
        );
        assert_eq!(
            resolve_shared_channel_sync_delivered_ledger_max_entries(),
            77
        );

        let _retention = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS_ENV,
            "999999999999",
        );
        let _max_entries = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_ENV,
            "999999999",
        );
        assert_eq!(
            resolve_shared_channel_sync_delivered_ledger_retention_millis(),
            SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MAX_MILLIS
        );
        assert_eq!(
            resolve_shared_channel_sync_delivered_ledger_max_entries(),
            SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES_MAX
        );
    }

    #[test]
    fn test_shared_channel_dispatch_timeout_is_capped_to_safe_upper_bound() {
        let _guard = scheduler_env_guard();
        let _timeout = ScopedEnvVar::set(SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS_ENV, "600000");

        assert_eq!(
            resolve_shared_channel_sync_http_timeout().as_millis(),
            60_000
        );
    }

    #[test]
    fn test_shared_channel_dispatch_worker_and_queue_limits_are_capped() {
        let _guard = scheduler_env_guard();
        let _workers = ScopedEnvVar::set(SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT_ENV, "1000");
        let _queue = ScopedEnvVar::set(SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY_ENV, "1000000");

        assert_eq!(resolve_shared_channel_sync_dispatch_worker_count(), 128);
        assert_eq!(
            resolve_shared_channel_sync_dispatch_queue_capacity(),
            65_536
        );
    }

    #[test]
    fn test_shared_channel_pending_retry_cooldown_limits_resolve_from_env() {
        let _guard = scheduler_env_guard();
        let _cooldown = ScopedEnvVar::remove(SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MILLIS_ENV);
        assert_eq!(
            resolve_shared_channel_sync_pending_retry_cooldown_millis(),
            SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_DEFAULT_MILLIS
        );

        let _cooldown = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MILLIS_ENV,
            "2500",
        );
        assert_eq!(
            resolve_shared_channel_sync_pending_retry_cooldown_millis(),
            2500
        );

        let _cooldown = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MILLIS_ENV,
            "999999",
        );
        assert_eq!(
            resolve_shared_channel_sync_pending_retry_cooldown_millis(),
            SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MAX_MILLIS
        );
    }

    #[test]
    fn test_pending_shared_channel_sync_dispatch_queue_defers_recent_failures_until_retry_cooldown()
    {
        let _guard = scheduler_env_guard();
        let _cooldown = ScopedEnvVar::set(
            SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MILLIS_ENV,
            "60000",
        );
        let runtime = SocialControlRuntime::default();
        let request = SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".to_owned(),
            conversation_id: "c_retry_cooldown".to_owned(),
            shared_channel_policy_id: "scp_retry_cooldown".to_owned(),
            external_connection_id: "ec_retry_cooldown".to_owned(),
            local_actor_id: "u_retry_cooldown".to_owned(),
            local_actor_kind: "user".to_owned(),
            external_member_id: "partner::retry-cooldown".to_owned(),
        };
        let request_key = shared_channel_sync_request_key(&request);
        let failed_at = format_unix_timestamp_millis(current_unix_epoch_millis());
        {
            let mut state = runtime
                .state
                .write()
                .unwrap_or_else(SocialControlRuntime::recover_poisoned_social_runtime_lock);
            state.upsert_pending_shared_channel_sync_request(
                request_key.clone(),
                PendingSharedChannelSyncRequest {
                    request: request.clone(),
                    failure_count: 1,
                    last_error: "dispatch timeout".to_owned(),
                    last_failed_at: Some(failed_at),
                    owner_actor_id: None,
                    owner_actor_kind: None,
                    claimed_at: None,
                    lease_expires_at: None,
                },
            );
        }

        let queue_during_cooldown = runtime.pending_shared_channel_sync_dispatch_queue(&[]);
        assert!(
            queue_during_cooldown.is_empty(),
            "recently failed request should be deferred until retry cooldown elapses"
        );

        {
            let mut state = runtime
                .state
                .write()
                .unwrap_or_else(SocialControlRuntime::recover_poisoned_social_runtime_lock);
            let mut pending = state
                .pending_shared_channel_sync_requests
                .get(request_key.as_str())
                .cloned()
                .expect("pending request should still exist");
            pending.last_failed_at = Some("1970-01-01T00:00:00.000Z".to_owned());
            state.upsert_pending_shared_channel_sync_request(request_key.clone(), pending);
        }
        let queue_after_cooldown = runtime.pending_shared_channel_sync_dispatch_queue(&[]);
        assert_eq!(
            queue_after_cooldown,
            vec![request],
            "request should re-enter dispatch queue after cooldown window"
        );
    }

    #[test]
    fn test_pending_shared_channel_sync_inventory_is_limited_and_emits_cursor() {
        let runtime = SocialControlRuntime::default();
        {
            let mut state = runtime
                .state
                .write()
                .unwrap_or_else(SocialControlRuntime::recover_poisoned_social_runtime_lock);
            for index in 1..=3 {
                let request = SharedChannelLinkedMemberSyncRequest {
                    tenant_id: "t_demo".to_owned(),
                    conversation_id: format!("c_inventory_page_{index:03}"),
                    shared_channel_policy_id: format!("scp_inventory_page_{index:03}"),
                    external_connection_id: format!("ec_inventory_page_{index:03}"),
                    local_actor_id: "u_inventory_page".to_owned(),
                    local_actor_kind: "user".to_owned(),
                    external_member_id: format!("partner::inventory-page-{index:03}"),
                };
                state.upsert_pending_shared_channel_sync_request(
                    shared_channel_sync_request_key(&request),
                    PendingSharedChannelSyncRequest {
                        request,
                        failure_count: 1,
                        last_error: "dispatch timeout".to_owned(),
                        last_failed_at: Some("2026-04-12T01:02:03.000Z".to_owned()),
                        owner_actor_id: None,
                        owner_actor_kind: None,
                        claimed_at: None,
                        lease_expires_at: None,
                    },
                );
            }
        }

        let page = SharedChannelSyncInventoryPageSpec {
            limit: 2,
            cursor: None,
        };
        let first_page =
            runtime.pending_shared_channel_sync_inventory("u_admin", "admin", true, &page);

        assert_eq!(first_page.pending_count, 3);
        assert_eq!(first_page.items.len(), 2);
        assert!(
            first_page.next_cursor.is_some(),
            "limited inventory page should include a cursor for the next page"
        );

        let cursor = parse_shared_channel_sync_inventory_cursor(
            first_page
                .next_cursor
                .as_deref()
                .expect("first page should expose next cursor"),
        )
        .expect("next cursor should parse");
        let second_page = runtime.pending_shared_channel_sync_inventory(
            "u_admin",
            "admin",
            true,
            &SharedChannelSyncInventoryPageSpec {
                limit: 2,
                cursor: Some(cursor),
            },
        );

        assert_eq!(second_page.pending_count, 3);
        assert_eq!(second_page.items.len(), 1);
        assert!(second_page.next_cursor.is_none());
    }

    #[test]
    fn test_pending_shared_channel_sync_non_canonical_lease_timestamp_is_treated_as_stale() {
        let pending = PendingSharedChannelSyncRequest {
            request: SharedChannelLinkedMemberSyncRequest {
                tenant_id: "t_demo".to_owned(),
                conversation_id: "c_non_canonical_lease".to_owned(),
                shared_channel_policy_id: "scp_non_canonical_lease".to_owned(),
                external_connection_id: "ec_non_canonical_lease".to_owned(),
                local_actor_id: "u_non_canonical_lease".to_owned(),
                local_actor_kind: "user".to_owned(),
                external_member_id: "partner::non-canonical-lease".to_owned(),
            },
            failure_count: 0,
            last_error: String::new(),
            last_failed_at: None,
            owner_actor_id: Some("operator_a".to_owned()),
            owner_actor_kind: Some("system".to_owned()),
            claimed_at: Some("2026-04-12T04:00:00.000Z".to_owned()),
            lease_expires_at: Some("2026-04-12T12:00:00+08:00".to_owned()),
        };
        let now = "2026-04-12T05:00:00.000Z";
        assert_eq!(
            pending.lease_status(now),
            SocialSharedChannelSyncLeaseStatus::Stale
        );
        assert!(pending.takeover_eligible_for("operator_b", "system", now));
        assert!(!pending.blocks_foreign_takeover("operator_b", "system", now));
    }

    #[test]
    fn test_pending_shared_channel_sync_auto_dispatch_ignores_non_canonical_failure_timestamp() {
        let pending = PendingSharedChannelSyncRequest {
            request: SharedChannelLinkedMemberSyncRequest {
                tenant_id: "t_demo".to_owned(),
                conversation_id: "c_non_canonical_failure".to_owned(),
                shared_channel_policy_id: "scp_non_canonical_failure".to_owned(),
                external_connection_id: "ec_non_canonical_failure".to_owned(),
                local_actor_id: "u_non_canonical_failure".to_owned(),
                local_actor_kind: "user".to_owned(),
                external_member_id: "partner::non-canonical-failure".to_owned(),
            },
            failure_count: 1,
            last_error: "dispatch timeout".to_owned(),
            last_failed_at: Some("2026-04-12T12:00:00+08:00".to_owned()),
            owner_actor_id: None,
            owner_actor_kind: None,
            claimed_at: None,
            lease_expires_at: None,
        };
        assert!(
            pending.auto_dispatch_eligible("2026-04-12T05:00:00.000Z", "2026-04-12T04:30:00.000Z"),
            "non-canonical failure timestamp must not block retry dispatch queue"
        );
    }

    #[test]
    fn test_shared_channel_failed_delivery_proof_is_recorded_for_pending_requests() {
        let request = SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".to_owned(),
            conversation_id: "c_fail_record".to_owned(),
            shared_channel_policy_id: "scp_fail_record".to_owned(),
            external_connection_id: "ec_fail_record".to_owned(),
            local_actor_id: "u_fail_record".to_owned(),
            local_actor_kind: "user".to_owned(),
            external_member_id: "partner::fail-record".to_owned(),
        };
        let failed_at = "2026-04-12T01:02:03.000Z";
        let mut state = SocialControlState::default();
        assert!(state.record_failed_shared_channel_sync_requests(
            std::slice::from_ref(&request),
            "dispatch_failed",
            failed_at
        ));

        let key = shared_channel_sync_request_key(&request);
        let pending = state
            .pending_shared_channel_sync_requests
            .get(key.as_str())
            .expect("failed request should be tracked in pending backlog");
        assert_eq!(pending.last_failed_at.as_deref(), Some(failed_at));
        assert_eq!(pending.failure_count, 1);

        let proof = state
            .delivered_shared_channel_sync_delivery_proofs
            .get(key.as_str())
            .expect("failed delivery should be reflected in delivery proof ledger");
        assert_eq!(proof.delivered_at, failed_at);
        assert_eq!(proof.status, SharedChannelSyncDeliveryProofStatus::Failed);
        assert_eq!(
            proof.proof_version.as_deref(),
            Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION)
        );
    }

    #[test]
    fn test_shared_channel_replayed_delivery_status_is_recorded_for_replayed_dispatch() {
        let request = SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".to_owned(),
            conversation_id: "c_replay_record".to_owned(),
            shared_channel_policy_id: "scp_replay_record".to_owned(),
            external_connection_id: "ec_replay_record".to_owned(),
            local_actor_id: "u_replay_record".to_owned(),
            local_actor_kind: "user".to_owned(),
            external_member_id: "partner::replay-record".to_owned(),
        };
        let key = shared_channel_sync_request_key(&request);
        let mut state = SocialControlState::default();
        state.upsert_pending_shared_channel_sync_request(
            key.clone(),
            PendingSharedChannelSyncRequest {
                request: request.clone(),
                failure_count: 2,
                last_error: "timeout".to_owned(),
                last_failed_at: Some("2026-04-12T01:09:00.000Z".to_owned()),
                owner_actor_id: None,
                owner_actor_kind: None,
                claimed_at: None,
                lease_expires_at: None,
            },
        );

        let proof = SharedChannelSyncDeliveryProof {
            request_key: key.clone(),
            status: SharedChannelSyncDeliveryProofStatus::Applied,
            proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
            target: Some("https://runtime.example.com".to_owned()),
        };
        assert!(state.record_dispatched_shared_channel_sync_request(
            &request,
            "2026-04-12T01:10:00.000Z",
            "2026-04-12T00:55:00.000Z",
            Some(&proof),
            true,
        ));

        let delivered = state
            .delivered_shared_channel_sync_delivery_proofs
            .get(key.as_str())
            .expect("replayed delivery should be persisted in proof ledger");
        assert_eq!(
            delivered.status,
            SharedChannelSyncDeliveryProofStatus::Replayed
        );
    }

    #[test]
    fn test_shared_channel_record_dispatched_overrides_non_canonical_existing_timestamps() {
        let request = SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".to_owned(),
            conversation_id: "c_invalid_timestamp_recover".to_owned(),
            shared_channel_policy_id: "scp_invalid_timestamp_recover".to_owned(),
            external_connection_id: "ec_invalid_timestamp_recover".to_owned(),
            local_actor_id: "u_invalid_timestamp_recover".to_owned(),
            local_actor_kind: "user".to_owned(),
            external_member_id: "partner::invalid-timestamp-recover".to_owned(),
        };
        let key = shared_channel_sync_request_key(&request);
        let mut state = SocialControlState::default();
        state
            .delivered_shared_channel_sync_requests
            .insert(key.clone(), "zzzz-invalid".to_owned());
        state.delivered_shared_channel_sync_delivery_proofs.insert(
            key.clone(),
            StoredSharedChannelSyncDeliveryProof {
                delivered_at: "zzzz-invalid".to_owned(),
                status: SharedChannelSyncDeliveryProofStatus::Failed,
                proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
                target: Some("https://invalid.example.com".to_owned()),
            },
        );

        let proof = SharedChannelSyncDeliveryProof {
            request_key: key.clone(),
            status: SharedChannelSyncDeliveryProofStatus::Applied,
            proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
            target: Some("https://runtime.example.com".to_owned()),
        };
        assert!(state.record_dispatched_shared_channel_sync_request(
            &request,
            "2026-04-12T01:20:00.000Z",
            "2026-04-12T01:00:00.000Z",
            Some(&proof),
            false,
        ));

        assert_eq!(
            state
                .delivered_shared_channel_sync_requests
                .get(key.as_str())
                .expect("delivered ledger item should exist"),
            "2026-04-12T01:20:00.000Z"
        );
        let stored_proof = state
            .delivered_shared_channel_sync_delivery_proofs
            .get(key.as_str())
            .expect("delivery proof should exist");
        assert_eq!(stored_proof.delivered_at, "2026-04-12T01:20:00.000Z");
        assert_eq!(
            stored_proof.status,
            SharedChannelSyncDeliveryProofStatus::Applied
        );
        assert_eq!(
            stored_proof.target.as_deref(),
            Some("https://runtime.example.com")
        );
    }

    #[test]
    fn test_prune_delivered_shared_channel_sync_requests_respects_protected_keys_and_capacity() {
        fn request(local_actor_id: &str) -> SharedChannelLinkedMemberSyncRequest {
            SharedChannelLinkedMemberSyncRequest {
                tenant_id: "t_demo".to_owned(),
                conversation_id: "c_partner_ops".to_owned(),
                shared_channel_policy_id: "scp_demo".to_owned(),
                external_connection_id: "ec_demo".to_owned(),
                local_actor_id: local_actor_id.to_owned(),
                local_actor_kind: "user".to_owned(),
                external_member_id: format!("partner::{local_actor_id}"),
            }
        }

        let mut state = SocialControlState::default();
        let protected_request = request("actor_protected");
        let protected_key = shared_channel_sync_request_key(&protected_request);
        state.upsert_pending_shared_channel_sync_request(
            protected_key.clone(),
            PendingSharedChannelSyncRequest {
                request: protected_request.clone(),
                failure_count: 0,
                last_error: String::new(),
                last_failed_at: None,
                owner_actor_id: None,
                owner_actor_kind: None,
                claimed_at: None,
                lease_expires_at: None,
            },
        );

        let old_a = request("actor_old_a");
        let old_b = request("actor_old_b");
        let fresh = request("actor_fresh");
        let old_a_key = shared_channel_sync_request_key(&old_a);
        let old_b_key = shared_channel_sync_request_key(&old_b);
        state
            .delivered_shared_channel_sync_requests
            .insert(old_a_key.clone(), "2026-01-01T00:00:00.000Z".to_owned());
        state
            .delivered_shared_channel_sync_requests
            .insert(old_b_key.clone(), "2026-01-02T00:00:00.000Z".to_owned());
        state
            .delivered_shared_channel_sync_requests
            .insert(protected_key.clone(), "2026-01-03T00:00:00.000Z".to_owned());
        state.delivered_shared_channel_sync_delivery_proofs.insert(
            old_a_key.clone(),
            StoredSharedChannelSyncDeliveryProof {
                delivered_at: "2026-01-01T00:00:00.000Z".to_owned(),
                status: SharedChannelSyncDeliveryProofStatus::Applied,
                proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
                target: Some("https://runtime-a".to_owned()),
            },
        );
        state.delivered_shared_channel_sync_delivery_proofs.insert(
            old_b_key.clone(),
            StoredSharedChannelSyncDeliveryProof {
                delivered_at: "2026-01-02T00:00:00.000Z".to_owned(),
                status: SharedChannelSyncDeliveryProofStatus::AlreadyLinked,
                proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
                target: Some("https://runtime-b".to_owned()),
            },
        );
        state.delivered_shared_channel_sync_delivery_proofs.insert(
            protected_key.clone(),
            StoredSharedChannelSyncDeliveryProof {
                delivered_at: "2026-01-03T00:00:00.000Z".to_owned(),
                status: SharedChannelSyncDeliveryProofStatus::Applied,
                proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
                target: Some("https://runtime-protected".to_owned()),
            },
        );
        let fresh_key = shared_channel_sync_request_key(&fresh);
        state
            .delivered_shared_channel_sync_requests
            .insert(fresh_key.clone(), "2026-04-12T00:00:00.000Z".to_owned());
        state.delivered_shared_channel_sync_delivery_proofs.insert(
            fresh_key.clone(),
            StoredSharedChannelSyncDeliveryProof {
                delivered_at: "2026-04-12T00:00:00.000Z".to_owned(),
                status: SharedChannelSyncDeliveryProofStatus::Applied,
                proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
                target: Some("https://runtime-fresh".to_owned()),
            },
        );

        let removed_by_age =
            state.prune_delivered_shared_channel_sync_requests("2026-02-01T00:00:00.000Z", 2);
        assert_eq!(removed_by_age, 2);
        assert!(
            state
                .delivered_shared_channel_sync_requests
                .contains_key(protected_key.as_str())
        );
        assert!(
            state
                .delivered_shared_channel_sync_requests
                .contains_key(fresh_key.as_str())
        );
        assert!(
            !state
                .delivered_shared_channel_sync_delivery_proofs
                .contains_key(old_a_key.as_str())
        );
        assert!(
            !state
                .delivered_shared_channel_sync_delivery_proofs
                .contains_key(old_b_key.as_str())
        );
        assert!(
            state
                .delivered_shared_channel_sync_delivery_proofs
                .contains_key(protected_key.as_str())
        );
        assert!(
            state
                .delivered_shared_channel_sync_delivery_proofs
                .contains_key(fresh_key.as_str())
        );
        assert_eq!(state.delivered_shared_channel_sync_requests.len(), 2);
        assert_eq!(state.delivered_shared_channel_sync_delivery_proofs.len(), 2);

        state.pending_shared_channel_sync_requests.clear();
        state.delivered_shared_channel_sync_requests.clear();
        state.delivered_shared_channel_sync_delivery_proofs.clear();
        let k1 = shared_channel_sync_request_key(&request("actor_cap_1"));
        let k2 = shared_channel_sync_request_key(&request("actor_cap_2"));
        let k3 = shared_channel_sync_request_key(&request("actor_cap_3"));
        state
            .delivered_shared_channel_sync_requests
            .insert(k1.clone(), "2026-03-01T00:00:00.000Z".to_owned());
        state
            .delivered_shared_channel_sync_requests
            .insert(k2.clone(), "2026-03-02T00:00:00.000Z".to_owned());
        state
            .delivered_shared_channel_sync_requests
            .insert(k3.clone(), "2026-03-03T00:00:00.000Z".to_owned());
        state.delivered_shared_channel_sync_delivery_proofs.insert(
            k1.clone(),
            StoredSharedChannelSyncDeliveryProof {
                delivered_at: "2026-03-01T00:00:00.000Z".to_owned(),
                status: SharedChannelSyncDeliveryProofStatus::Applied,
                proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
                target: Some("https://runtime-cap-1".to_owned()),
            },
        );
        state.delivered_shared_channel_sync_delivery_proofs.insert(
            k2.clone(),
            StoredSharedChannelSyncDeliveryProof {
                delivered_at: "2026-03-02T00:00:00.000Z".to_owned(),
                status: SharedChannelSyncDeliveryProofStatus::Applied,
                proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
                target: Some("https://runtime-cap-2".to_owned()),
            },
        );
        state.delivered_shared_channel_sync_delivery_proofs.insert(
            k3.clone(),
            StoredSharedChannelSyncDeliveryProof {
                delivered_at: "2026-03-03T00:00:00.000Z".to_owned(),
                status: SharedChannelSyncDeliveryProofStatus::AlreadyLinked,
                proof_version: Some(SHARED_CHANNEL_SYNC_ACK_PROOF_VERSION.to_owned()),
                target: Some("https://runtime-cap-3".to_owned()),
            },
        );

        let removed_by_capacity =
            state.prune_delivered_shared_channel_sync_requests("2026-01-01T00:00:00.000Z", 2);
        assert_eq!(removed_by_capacity, 1);
        assert!(
            !state
                .delivered_shared_channel_sync_requests
                .contains_key(k1.as_str())
        );
        assert!(
            state
                .delivered_shared_channel_sync_requests
                .contains_key(k2.as_str())
        );
        assert!(
            state
                .delivered_shared_channel_sync_requests
                .contains_key(k3.as_str())
        );
        assert!(
            !state
                .delivered_shared_channel_sync_delivery_proofs
                .contains_key(k1.as_str())
        );
        assert!(
            state
                .delivered_shared_channel_sync_delivery_proofs
                .contains_key(k2.as_str())
        );
        assert!(
            state
                .delivered_shared_channel_sync_delivery_proofs
                .contains_key(k3.as_str())
        );
    }
}
