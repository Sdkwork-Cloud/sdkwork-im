use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::{Path, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_auth_context::{AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context};
use im_domain_core::conversation::{
    ConversationMember, ConversationReadCursorView, MembershipRole,
};
use im_domain_core::message::{ContentPart, MessageBody, MessageType};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::*;

#[derive(Clone)]
struct AppState {
    runtime: Arc<ConversationRuntime<InMemoryJournal>>,
    shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter,
}

const SHARED_CHANNEL_SYNC_PERMISSION: &str = "conversation.shared_channel.sync";
const SHARED_CHANNEL_SYNC_ACTOR_ID: &str = "control-plane-sync";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS_ENV: &str =
    "CRAW_CHAT_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS";
const SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_REQUESTS: u32 = 120;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_WINDOW_SECONDS: u64 = 60;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_BUCKETS: usize = 10_000;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS: u32 = 10_000;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS: u64 = 3_600;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_BUCKETS: usize = 200_000;
const SHARED_CHANNEL_SYNC_RATE_LIMIT_SWEEP_THRESHOLD: usize = 1024;

#[derive(Clone)]
struct SharedChannelSyncRateLimiter {
    max_requests: u32,
    window_millis: u128,
    max_buckets: usize,
    buckets: Arc<Mutex<BTreeMap<String, SharedChannelSyncRateLimitBucket>>>,
}

#[derive(Clone, Debug)]
struct SharedChannelSyncRateLimitBucket {
    window_started_at_millis: u128,
    request_count: u32,
}

impl SharedChannelSyncRateLimiter {
    fn from_env() -> Self {
        let max_requests = resolve_positive_env_u32_with_upper_bound(
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_REQUESTS,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS,
        );
        let window_seconds = resolve_positive_env_u64_with_upper_bound(
            SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_WINDOW_SECONDS,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS,
        );
        let max_buckets = resolve_positive_env_usize_with_upper_bound(
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS_ENV,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_DEFAULT_MAX_BUCKETS,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_BUCKETS,
        );
        Self {
            max_requests,
            window_millis: (window_seconds as u128) * 1000,
            max_buckets,
            buckets: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    fn try_acquire(&self, tenant_id: &str) -> bool {
        let now = current_unix_epoch_millis();
        let mut buckets =
            lock_shared_channel_rate_limit_mutex(&self.buckets, "shared-channel-sync-rate-limit");

        if buckets.len() > SHARED_CHANNEL_SYNC_RATE_LIMIT_SWEEP_THRESHOLD
            || buckets.len() >= self.max_buckets
        {
            let window_millis = self.window_millis;
            buckets.retain(|_, bucket| {
                now.saturating_sub(bucket.window_started_at_millis) < window_millis
            });
        }
        if !buckets.contains_key(tenant_id) && buckets.len() >= self.max_buckets {
            return false;
        }

        let bucket =
            buckets
                .entry(tenant_id.to_owned())
                .or_insert(SharedChannelSyncRateLimitBucket {
                    window_started_at_millis: now,
                    request_count: 0,
                });

        if now.saturating_sub(bucket.window_started_at_millis) >= self.window_millis {
            bucket.window_started_at_millis = now;
            bucket.request_count = 0;
        }

        if bucket.request_count >= self.max_requests {
            return false;
        }

        bucket.request_count = bucket.request_count.saturating_add(1);
        true
    }
}

fn lock_shared_channel_rate_limit_mutex<'a, T>(
    mutex: &'a Mutex<T>,
    lock_name: &'static str,
) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warn: recovered poisoned conversation-runtime mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

fn resolve_positive_env_u32_with_upper_bound(name: &str, default: u32, max: u32) -> u32 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
        .clamp(1, max)
}

fn resolve_positive_env_u64_with_upper_bound(name: &str, default: u64, max: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
        .clamp(1, max)
}

fn resolve_positive_env_usize_with_upper_bound(name: &str, default: usize, max: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
        .clamp(1, max)
}

fn unix_epoch_millis(now: SystemTime) -> u128 {
    now.duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn current_unix_epoch_millis() -> u128 {
    unix_epoch_millis(SystemTime::now())
}

fn shared_channel_sync_request_key(
    tenant_id: &str,
    request: &SyncSharedChannelLinkedMemberRequest,
) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        tenant_id,
        request.conversation_id,
        request.shared_channel_policy_id,
        request.external_connection_id,
        request.local_actor_id,
        request.local_actor_kind,
        request.external_member_id
    )
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostMessageRequest {
    client_msg_id: Option<String>,
    summary: Option<String>,
    text: Option<String>,
    #[serde(default)]
    parts: Vec<ContentPart>,
    #[serde(default)]
    render_hints: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EditMessageRequest {
    summary: Option<String>,
    text: Option<String>,
    #[serde(default)]
    parts: Vec<ContentPart>,
    #[serde(default)]
    render_hints: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageReactionRequest {
    reaction_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateConversationRequest {
    conversation_id: String,
    conversation_type: String,
    policy_version: Option<String>,
    capability_flags: Option<Vec<String>>,
    history_visibility: Option<String>,
    retention_policy_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateAgentDialogRequest {
    conversation_id: String,
    agent_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateAgentHandoffRequest {
    conversation_id: String,
    target_id: String,
    target_kind: String,
    handoff_session_id: String,
    handoff_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateSystemChannelRequest {
    conversation_id: String,
    subscriber_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateThreadConversationRequest {
    conversation_id: String,
    parent_conversation_id: String,
    root_message_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BindDirectChatConversationRequest {
    conversation_id: String,
    direct_chat_id: String,
    left_actor_id: String,
    left_actor_kind: String,
    right_actor_id: String,
    right_actor_kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SyncSharedChannelLinkedMemberRequest {
    conversation_id: String,
    shared_channel_policy_id: String,
    external_connection_id: String,
    local_actor_id: String,
    local_actor_kind: String,
    external_member_id: String,
    #[serde(default)]
    request_key: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddConversationMemberRequest {
    principal_id: String,
    principal_kind: String,
    role: MembershipRole,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoveConversationMemberRequest {
    member_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransferConversationOwnerRequest {
    member_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChangeConversationMemberRoleRequest {
    member_id: String,
    role: MembershipRole,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ListMembersResponse {
    items: Vec<ConversationMember>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SyncSharedChannelLinkedMemberResponse {
    proof_version: &'static str,
    request_key: String,
    status: SyncSharedChannelLinkedMemberStatus,
    #[serde(flatten)]
    member: ConversationMember,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationBindingResponse {
    conversation_id: String,
    business_type: String,
    business_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateReadCursorRequest {
    read_seq: u64,
    last_read_message_id: Option<String>,
}

impl CreateConversationRequest {
    fn conversation_policy(&self) -> Result<Option<ConversationPolicy>, ApiError> {
        validate_optional_payload_size(
            "policyVersion",
            self.policy_version.as_deref(),
            CONVERSATION_MAX_POLICY_VERSION_BYTES,
        )
        .map_err(ApiError::from)?;
        if let Some(capability_flags) = &self.capability_flags {
            validate_string_vec_payload_size(
                "capabilityFlags",
                capability_flags,
                CONVERSATION_MAX_CAPABILITY_FLAG_BYTES,
                CONVERSATION_MAX_CAPABILITY_FLAGS_TOTAL_BYTES,
            )
            .map_err(ApiError::from)?;
        }
        validate_optional_payload_size(
            "historyVisibility",
            self.history_visibility.as_deref(),
            CONVERSATION_MAX_HISTORY_VISIBILITY_BYTES,
        )
        .map_err(ApiError::from)?;
        validate_optional_payload_size(
            "retentionPolicyRef",
            self.retention_policy_ref.as_deref(),
            CONVERSATION_MAX_RETENTION_POLICY_REF_BYTES,
        )
        .map_err(ApiError::from)?;
        if self.policy_version.is_none()
            && self.capability_flags.is_none()
            && self.history_visibility.is_none()
            && self.retention_policy_ref.is_none()
        {
            return Ok(None);
        }

        let mut policy = ConversationPolicy::default();
        if let Some(policy_version) = &self.policy_version {
            policy.policy_version = policy_version.clone();
        }
        if let Some(capability_flags) = &self.capability_flags {
            policy.capability_flags = Some(capability_flags.clone());
        }
        if let Some(history_visibility) = &self.history_visibility {
            policy.history_visibility = history_visibility.clone();
        }
        if let Some(retention_policy_ref) = &self.retention_policy_ref {
            policy.retention_policy_ref = retention_policy_ref.clone();
        }

        policy
            .normalize()
            .map(Some)
            .map_err(|message| ApiError::bad_request("conversation_policy_invalid", message))
    }
}

#[derive(Debug)]
struct ApiError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl ApiError {
    fn bad_request(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::BAD_REQUEST,
            code,
            message: message.into(),
        }
    }

    fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code,
            message: message.into(),
        }
    }

    fn too_many_requests(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::TOO_MANY_REQUESTS,
            code,
            message: message.into(),
        }
    }
}

impl From<AuthContextError> for ApiError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<RuntimeError> for ApiError {
    fn from(value: RuntimeError) -> Self {
        match value {
            RuntimeError::ConversationAlreadyExists(message) => {
                Self::bad_request("conversation_exists", message)
            }
            RuntimeError::ConversationTypeInvalid(message) => {
                Self::bad_request("conversation_type_invalid", message)
            }
            RuntimeError::InvalidInput(message) => {
                Self::bad_request("conversation_request_invalid", message)
            }
            RuntimeError::PayloadTooLarge(message) => Self {
                status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                code: "payload_too_large",
                message,
            },
            RuntimeError::ConversationNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_not_found",
                message,
            },
            RuntimeError::ConversationBindingNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_binding_not_found",
                message,
            },
            RuntimeError::MessageNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "message_not_found",
                message,
            },
            RuntimeError::MessageAlreadyRecalled(message) => Self::bad_request(
                "message_already_recalled",
                format!("message already recalled: {message}"),
            ),
            RuntimeError::MemberAlreadyExists(message) => {
                Self::bad_request("conversation_member_exists", message)
            }
            RuntimeError::MemberNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_member_not_found",
                message,
            },
            RuntimeError::PermissionDenied(message) => {
                Self::forbidden("conversation_permission_denied", message)
            }
            RuntimeError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "conversation_conflict",
                message,
            },
            RuntimeError::ReadCursorInvalid(message) => {
                Self::bad_request("read_cursor_invalid", message)
            }
            RuntimeError::Contract(_) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "journal_unavailable",
                message: "commit journal unavailable".into(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({
                "code": self.code,
                "message": self.message
            })),
        )
            .into_response()
    }
}

pub fn build_default_app() -> Router {
    let state = AppState {
        runtime: Arc::new(ConversationRuntime::new(InMemoryJournal::default())),
        shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter::from_env(),
    };
    build_app(state)
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/conversations", post(create_conversation))
        .route(
            "/api/v1/conversations/threads",
            post(create_thread_conversation),
        )
        .route(
            "/api/v1/conversations/direct-chats/bindings",
            post(bind_direct_chat_conversation),
        )
        .route(
            "/api/v1/conversations/shared-channel-links/sync",
            post(sync_shared_channel_linked_member),
        )
        .route(
            "/api/v1/conversations/agent-dialogs",
            post(create_agent_dialog),
        )
        .route(
            "/api/v1/conversations/agent-handoffs",
            post(create_agent_handoff),
        )
        .route(
            "/api/v1/conversations/system-channels",
            post(create_system_channel),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff",
            get(get_agent_handoff_state),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff/accept",
            post(accept_agent_handoff),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff/resolve",
            post(resolve_agent_handoff),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff/close",
            post(close_agent_handoff),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members",
            get(list_members),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/binding",
            get(get_conversation_binding),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/add",
            post(add_member),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/remove",
            post(remove_member),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/transfer-owner",
            post(transfer_conversation_owner),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/change-role",
            post(change_conversation_member_role),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/leave",
            post(leave_conversation),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/read-cursor",
            get(get_read_cursor).post(update_read_cursor),
        )
        .route("/api/v1/messages/{message_id}/edit", post(edit_message))
        .route("/api/v1/messages/{message_id}/recall", post(recall_message))
        .route(
            "/api/v1/messages/{message_id}/reactions",
            post(add_message_reaction),
        )
        .route(
            "/api/v1/messages/{message_id}/reactions/remove",
            post(remove_message_reaction),
        )
        .route("/api/v1/messages/{message_id}/pin", post(pin_message))
        .route("/api/v1/messages/{message_id}/unpin", post(unpin_message))
        .route(
            "/api/v1/conversations/{conversation_id}/messages",
            get(list_messages).post(post_message),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/system-channel/publish",
            post(publish_system_channel_message),
        )
        .with_state(state)
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => ApiError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "conversation-runtime",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "conversation-runtime",
    })
}

async fn create_conversation(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let policy = request.conversation_policy()?;
    let result = state.runtime.create_conversation_from_auth_context(
        &auth,
        request.conversation_id,
        request.conversation_type,
    )?;
    if let Some(policy) = policy {
        if result.is_applied() {
            state.runtime.apply_conversation_policy_from_auth_context(
                &auth,
                result.conversation_id.clone(),
                policy,
            )?;
        } else {
            match state.runtime.conversation_policy_snapshot(
                auth.tenant_id.as_str(),
                result.conversation_id.as_str(),
            )? {
                Some(existing) if existing == policy => {}
                Some(_) => {
                    return Err(ApiError::from(RuntimeError::Conflict(format!(
                        "conversation create request conflicts with existing policy for conversation {}",
                        result.conversation_id
                    ))));
                }
                None => {
                    state.runtime.apply_conversation_policy_from_auth_context(
                        &auth,
                        result.conversation_id.clone(),
                        policy,
                    )?;
                }
            }
        }
    }
    Ok(Json(result))
}

async fn create_agent_dialog(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentDialogRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.create_agent_dialog_from_auth_context(
        &auth,
        request.conversation_id,
        request.agent_id,
    )?))
}

async fn create_agent_handoff(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentHandoffRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.create_agent_handoff_from_auth_context(
        &auth,
        request.conversation_id,
        request.target_id,
        request.target_kind,
        request.handoff_session_id,
        request.handoff_reason,
    )?))
}

async fn create_system_channel(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateSystemChannelRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state.runtime.create_system_channel_from_auth_context(
            &auth,
            request.conversation_id,
            request.subscriber_id,
        )?,
    ))
}

async fn create_thread_conversation(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateThreadConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state.runtime.create_thread_conversation_from_auth_context(
            &auth,
            request.conversation_id,
            request.parent_conversation_id,
            request.root_message_id,
        )?,
    ))
}

async fn bind_direct_chat_conversation(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<BindDirectChatConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .runtime
            .bind_direct_chat_conversation_from_auth_context(
                &auth,
                request.conversation_id,
                request.direct_chat_id,
                request.left_actor_id,
                request.left_actor_kind,
                request.right_actor_id,
                request.right_actor_kind,
            )?,
    ))
}

async fn sync_shared_channel_linked_member(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SyncSharedChannelLinkedMemberRequest>,
) -> Result<Json<SyncSharedChannelLinkedMemberResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    if !auth.has_permission(SHARED_CHANNEL_SYNC_PERMISSION) {
        return Err(ApiError::forbidden(
            "shared_channel_sync_permission_denied",
            format!(
                "shared channel linked-member sync requires permission {SHARED_CHANNEL_SYNC_PERMISSION}"
            ),
        ));
    }
    if auth.actor_id != SHARED_CHANNEL_SYNC_ACTOR_ID {
        return Err(ApiError::forbidden(
            "shared_channel_sync_actor_invalid",
            format!(
                "shared channel linked-member sync requires actor {}",
                SHARED_CHANNEL_SYNC_ACTOR_ID
            ),
        ));
    }
    if !state
        .shared_channel_sync_rate_limiter
        .try_acquire(auth.tenant_id.as_str())
    {
        return Err(ApiError::too_many_requests(
            "shared_channel_sync_rate_limited",
            "shared channel linked-member sync exceeded per-tenant rate limit",
        ));
    }
    let expected_request_key = shared_channel_sync_request_key(auth.tenant_id.as_str(), &request);
    if let Some(request_key) = request.request_key.as_deref() {
        validate_payload_size(
            "requestKey",
            request_key,
            CONVERSATION_MAX_REQUEST_KEY_BYTES,
        )
        .map_err(ApiError::from)?;
        if request_key.trim().is_empty() {
            return Err(ApiError::bad_request(
                "shared_channel_sync_request_key_invalid",
                "shared channel linked-member sync requestKey cannot be empty when provided",
            ));
        }
        if request_key != expected_request_key.as_str() {
            return Err(ApiError::bad_request(
                "shared_channel_sync_request_key_mismatch",
                format!(
                    "shared channel linked-member sync requestKey mismatch: expected {expected_request_key}, got {request_key}"
                ),
            ));
        }
    }
    let request_key = request.request_key.clone().unwrap_or(expected_request_key);
    let sync_result = state
        .runtime
        .sync_shared_channel_linked_member_from_auth_context_with_result(
            &auth,
            request.conversation_id,
            request.shared_channel_policy_id,
            request.external_connection_id,
            request.local_actor_id,
            request.local_actor_kind,
            request.external_member_id,
        )?;
    Ok(Json(SyncSharedChannelLinkedMemberResponse {
        proof_version: "shared_channel_sync_ack.v1",
        request_key,
        status: sync_result.status,
        member: sync_result.member,
    }))
}

async fn get_agent_handoff_state(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .runtime
            .get_agent_handoff_state_from_auth_context(&auth, conversation_id.as_str())?,
    ))
}

async fn get_conversation_binding(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ConversationBindingResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let binding = state
        .runtime
        .conversation_business_binding_from_auth_context(&auth, conversation_id.as_str())?;
    Ok(Json(ConversationBindingResponse {
        conversation_id,
        business_type: binding.business_type,
        business_id: binding.business_id,
    }))
}

async fn accept_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.accept_agent_handoff_from_auth_context(
        &auth,
        conversation_id,
    )?))
}

async fn resolve_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .runtime
            .resolve_agent_handoff_from_auth_context(&auth, conversation_id)?,
    ))
}

async fn close_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.close_agent_handoff_from_auth_context(
        &auth,
        conversation_id,
    )?))
}

async fn list_members(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ListMembersResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(ListMembersResponse {
        items: state
            .runtime
            .list_members_from_auth_context(&auth, conversation_id.as_str())?,
    }))
}

async fn add_member(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AddConversationMemberRequest>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.add_member_from_auth_context(
        &auth,
        conversation_id,
        request.principal_id,
        request.principal_kind,
        request.role,
        request.attributes,
    )?))
}

async fn remove_member(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RemoveConversationMemberRequest>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.remove_member_from_auth_context(
        &auth,
        conversation_id,
        request.member_id,
    )?))
}

async fn transfer_conversation_owner(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<TransferConversationOwnerRequest>,
) -> Result<Json<TransferConversationOwnerResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .runtime
            .transfer_conversation_owner_from_auth_context(
                &auth,
                conversation_id,
                request.member_id,
            )?,
    ))
}

async fn change_conversation_member_role(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ChangeConversationMemberRoleRequest>,
) -> Result<Json<ChangeConversationMemberRoleResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .runtime
            .change_conversation_member_role_from_auth_context(
                &auth,
                conversation_id,
                request.member_id,
                request.role,
            )?,
    ))
}

async fn leave_conversation(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.leave_conversation_from_auth_context(
        &auth,
        conversation_id,
    )?))
}

async fn get_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.read_cursor_view_from_auth_context(
        &auth,
        conversation_id.as_str(),
    )?))
}

async fn list_messages(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MessageHistoryResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.list_messages_from_auth_context(
        &auth,
        conversation_id.as_str(),
    )?))
}

async fn update_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateReadCursorRequest>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    state.runtime.update_read_cursor_from_auth_context(
        &auth,
        conversation_id.clone(),
        request.read_seq,
        request.last_read_message_id,
    )?;

    Ok(Json(state.runtime.read_cursor_view_from_auth_context(
        &auth,
        conversation_id.as_str(),
    )?))
}

async fn post_message(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostMessageRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let body = build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;

    let result = state
        .runtime
        .post_message(PostMessageCommand::from_auth_context(
            &auth,
            conversation_id,
            request.client_msg_id,
            MessageType::Standard,
            body,
        ))?;
    Ok(Json(result))
}

async fn publish_system_channel_message(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostMessageRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let body = build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;

    let result = state.runtime.publish_system_channel_message(
        PublishSystemChannelMessageCommand::from_auth_context(
            &auth,
            conversation_id,
            request.client_msg_id,
            body,
        ),
    )?;

    Ok(Json(result))
}

async fn edit_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<EditMessageRequest>,
) -> Result<Json<MessageMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let body = build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;
    Ok(Json(state.runtime.edit_message(
        EditMessageCommand::from_auth_context(&auth, message_id, body),
    )?))
}

async fn recall_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MessageMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.recall_message(
        RecallMessageCommand::from_auth_context(&auth, message_id),
    )?))
}

async fn add_message_reaction(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<MessageReactionRequest>,
) -> Result<Json<MessageReactionMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    if request.reaction_key.trim().is_empty() {
        return Err(ApiError::bad_request(
            "reaction_key_invalid",
            "reaction key must not be empty",
        ));
    }

    Ok(Json(state.runtime.add_message_reaction(
        AddMessageReactionCommand::from_auth_context(&auth, message_id, request.reaction_key),
    )?))
}

async fn remove_message_reaction(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<MessageReactionRequest>,
) -> Result<Json<MessageReactionMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    if request.reaction_key.trim().is_empty() {
        return Err(ApiError::bad_request(
            "reaction_key_invalid",
            "reaction key must not be empty",
        ));
    }

    Ok(Json(state.runtime.remove_message_reaction(
        RemoveMessageReactionCommand::from_auth_context(&auth, message_id, request.reaction_key),
    )?))
}

async fn pin_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MessagePinMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.pin_message(
        PinMessageCommand::from_auth_context(&auth, message_id),
    )?))
}

async fn unpin_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MessagePinMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.unpin_message(
        UnpinMessageCommand::from_auth_context(&auth, message_id),
    )?))
}

fn build_message_body(
    summary: Option<String>,
    text: Option<String>,
    parts: Vec<ContentPart>,
    render_hints: BTreeMap<String, String>,
) -> Result<MessageBody, ApiError> {
    let mut resolved_parts = Vec::new();
    if let Some(text) = text
        && !text.trim().is_empty()
    {
        resolved_parts.push(ContentPart::text(text));
    }
    resolved_parts.extend(parts);

    if resolved_parts.is_empty() {
        return Err(ApiError::bad_request(
            "message_body_empty",
            "message body must contain text or parts",
        ));
    }

    Ok(MessageBody {
        summary,
        parts: resolved_parts,
        render_hints,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};
    use std::time::Duration;

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
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                unsafe {
                    std::env::set_var(self.name, previous);
                }
                return;
            }
            unsafe {
                std::env::remove_var(self.name);
            }
        }
    }

    fn rate_limit_env_guard<'a>() -> std::sync::MutexGuard<'a, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock")
    }

    #[test]
    fn test_unix_epoch_millis_clamps_pre_epoch_time_to_zero() {
        let before_epoch = UNIX_EPOCH
            .checked_sub(Duration::from_millis(1))
            .expect("test pre-epoch timestamp should construct");
        assert_eq!(unix_epoch_millis(before_epoch), 0);
    }

    #[test]
    fn test_unix_epoch_millis_preserves_post_epoch_time() {
        let after_epoch = UNIX_EPOCH + Duration::from_millis(1_234);
        assert_eq!(unix_epoch_millis(after_epoch), 1_234);
    }

    #[test]
    fn test_shared_channel_sync_rate_limiter_clamps_env_values_to_safe_bounds() {
        let _guard = rate_limit_env_guard();
        let _max_requests =
            ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV, "999999");
        let _window_seconds =
            ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV, "999999");
        let _max_buckets =
            ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS_ENV, "999999");

        let limiter = SharedChannelSyncRateLimiter::from_env();
        assert_eq!(
            limiter.max_requests,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS
        );
        assert_eq!(
            limiter.window_millis,
            (SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS as u128) * 1000
        );
        assert_eq!(
            limiter.max_buckets,
            SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_BUCKETS
        );
    }

    #[test]
    fn test_shared_channel_sync_rate_limiter_rejects_new_tenant_when_bucket_cap_is_reached() {
        let limiter = SharedChannelSyncRateLimiter {
            max_requests: 2,
            window_millis: 60_000,
            max_buckets: 2,
            buckets: Arc::new(Mutex::new(BTreeMap::new())),
        };

        assert!(limiter.try_acquire("tenant_a"));
        assert!(limiter.try_acquire("tenant_b"));
        assert!(
            !limiter.try_acquire("tenant_c"),
            "new tenant should be rejected when rate-limit bucket cap is reached"
        );
        assert!(
            limiter.try_acquire("tenant_a"),
            "existing tenant should still be serviceable when cap is reached"
        );
    }

    #[test]
    fn test_shared_channel_sync_rate_limiter_prunes_expired_buckets_before_rejecting_new_tenant() {
        let limiter = SharedChannelSyncRateLimiter {
            max_requests: 1,
            window_millis: 1,
            max_buckets: 2,
            buckets: Arc::new(Mutex::new(BTreeMap::new())),
        };
        {
            let mut buckets = lock_shared_channel_rate_limit_mutex(
                &limiter.buckets,
                "shared-channel-sync-rate-limit",
            );
            buckets.insert(
                "tenant_expired_a".into(),
                SharedChannelSyncRateLimitBucket {
                    window_started_at_millis: 0,
                    request_count: 1,
                },
            );
            buckets.insert(
                "tenant_expired_b".into(),
                SharedChannelSyncRateLimitBucket {
                    window_started_at_millis: 0,
                    request_count: 1,
                },
            );
        }

        assert!(
            limiter.try_acquire("tenant_new"),
            "expired buckets should be swept before enforcing max bucket cap"
        );
    }
}
