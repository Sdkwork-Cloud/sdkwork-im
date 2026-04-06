use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path as StdPath, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use audit_service::{AuditExportBundle, AuditRecord, AuditRuntime, RecordAuditAnchor};
use automation_service::{AutomationExecution, AutomationRuntime, RequestAutomationExecution};
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{
    Json, Router,
    routing::{get, post},
};
use conversation_runtime::{
    AcceptAgentHandoffCommand, AddConversationMemberCommand, AgentHandoffStateView,
    ChangeConversationMemberRoleCommand, ChangeConversationMemberRoleResult,
    CloseAgentHandoffCommand, ConversationRuntime, CreateAgentDialogCommand,
    CreateAgentHandoffCommand, CreateConversationCommand, CreateConversationResult,
    CreateSystemChannelCommand, EditMessageCommand, LeaveConversationCommand,
    MessageMutationResult, PostMessageCommand, PostMessageResult,
    PublishSystemChannelMessageCommand, RecallMessageCommand, RemoveConversationMemberCommand,
    ResolveAgentHandoffCommand, TransferConversationOwnerCommand, TransferConversationOwnerResult,
    UpdateReadCursorCommand,
};
use im_adapters_local_disk::{
    FileAutomationExecutionStore, FileCommitJournal, FileNotificationTaskStore,
    FilePresenceStateStore, FileRealtimeCheckpointStore, FileRealtimeDisconnectFenceStore,
    FileRealtimeSubscriptionStore, FileRtcStateStore, FileStreamStateStore,
    read_commit_journal_file, validate_automation_execution_store_file,
    validate_commit_journal_file, validate_notification_task_store_file,
    validate_presence_state_store_file, validate_realtime_checkpoint_store_file,
    validate_realtime_disconnect_fence_store_file, validate_realtime_subscription_store_file,
    validate_rtc_state_store_file, validate_stream_state_store_file,
};
use im_adapters_local_memory::MemoryCommitJournal;
use im_adapters_local_memory::MemoryRealtimeCheckpointStore;
use im_auth_context::AuthContext;
use im_auth_context::{AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context};
use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationMember, ConversationReadCursorView, DeviceSyncFeedEntry,
    MembershipRole,
};
use im_domain_core::media::MediaProcessingState;
use im_domain_core::message::{
    ContentPart, MediaPart, MessageBody, MessageType, Sender, SignalPart,
};
use im_domain_core::realtime::RealtimeSubscription;
use im_domain_core::session::{PresenceSnapshotView, SessionResumeView};
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    CommitJournal, CommitPosition, ContractError, RealtimeCheckpointRecord,
    RealtimeDisconnectFenceRecord, RealtimeSubscriptionRecord, RtcStateRecord, StreamStateRecord,
};
use media_service::{CompleteUploadRequest, CreateUploadRequest, MediaRuntime};
use notification_service::{NotificationRuntime, NotificationTask, RequestNotification};
use ops_service::{
    ClusterView, DiagnosticBundle, LagView, OpsHealthResponse, OpsRuntime, RouteOwnershipView,
    RuntimeDirInspectionItem, RuntimeDirInspectionView,
};
use projection_service::TimelineProjectionService;
use rtc_signaling_service::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, PostRtcSignalRequest, RtcRuntime,
    UpdateRtcSessionRequest,
};
use serde::{Deserialize, Serialize};
use session_gateway::{
    AckRealtimeEventsRequest, ListRealtimeEventsQuery, PresenceRuntimeError, RealtimeClusterBridge,
    RealtimeClusterError, RealtimeDeliveryRuntime, RealtimeRuntimeError, SessionPresenceRuntime,
    SyncRealtimeSubscriptionsRequest, serve_realtime_websocket,
};
use streaming_service::{
    AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest,
    ListStreamFramesQuery, OpenStreamRequest, StreamFrameWindow, StreamingRuntime,
};

#[derive(Clone)]
struct AppState {
    node_id: String,
    runtime_dir: Option<PathBuf>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    conversation_runtime: Arc<ConversationRuntime<ProjectionJournal>>,
    projection_service: Arc<TimelineProjectionService>,
    session_presence_runtime: Arc<SessionPresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    media_runtime: Arc<MediaRuntime>,
    streaming_runtime: Arc<StreamingRuntime>,
    rtc_runtime: Arc<RtcRuntime>,
    notification_runtime: Arc<NotificationRuntime>,
    automation_runtime: Arc<AutomationRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    ops_runtime: Arc<OpsRuntime>,
}

#[derive(Clone)]
struct ProjectionJournal {
    inner: ProjectionJournalInner,
    projection_service: Arc<TimelineProjectionService>,
}

#[derive(Clone)]
enum ProjectionJournalInner {
    Memory(MemoryCommitJournal),
    File(FileCommitJournal),
}

impl ProjectionJournal {
    fn new_memory(projection_service: Arc<TimelineProjectionService>) -> Self {
        Self {
            inner: ProjectionJournalInner::Memory(MemoryCommitJournal::with_partition(
                "local-minimal",
            )),
            projection_service,
        }
    }

    fn new_file(
        projection_service: Arc<TimelineProjectionService>,
        file_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            inner: ProjectionJournalInner::File(FileCommitJournal::new("local-minimal", file_path)),
            projection_service,
        }
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        match &self.inner {
            ProjectionJournalInner::Memory(inner) => Ok(inner.recorded()),
            ProjectionJournalInner::File(inner) => inner.recorded(),
        }
    }
}

impl CommitJournal for ProjectionJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let position = match &self.inner {
            ProjectionJournalInner::Memory(inner) => inner.append(envelope.clone())?,
            ProjectionJournalInner::File(inner) => inner.append(envelope.clone())?,
        };

        let _ = self.projection_service.apply(&envelope);

        Ok(position)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
    profile: &'static str,
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
struct CreateConversationRequest {
    conversation_id: String,
    conversation_type: String,
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
struct AddConversationMemberRequest {
    principal_id: String,
    principal_kind: String,
    role: MembershipRole,
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
struct InboxResponse {
    items: Vec<ConversationInboxEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DeviceSyncFeedResponse {
    items: Vec<DeviceSyncFeedEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterDeviceRequest {
    device_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SyncFeedQuery {
    after_seq: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResumeSessionRequest {
    device_id: Option<String>,
    last_seen_sync_seq: Option<u64>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PresenceDeviceRequest {
    device_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateReadCursorRequest {
    read_seq: u64,
    last_read_message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AttachMediaRequest {
    conversation_id: String,
    client_msg_id: Option<String>,
    summary: Option<String>,
    text: Option<String>,
    #[serde(default)]
    render_hints: BTreeMap<String, String>,
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
}

impl From<conversation_runtime::RuntimeError> for ApiError {
    fn from(value: conversation_runtime::RuntimeError) -> Self {
        match value {
            conversation_runtime::RuntimeError::ConversationAlreadyExists(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "conversation_exists",
                message,
            },
            conversation_runtime::RuntimeError::ConversationTypeInvalid(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "conversation_type_invalid",
                message,
            },
            conversation_runtime::RuntimeError::ConversationNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_not_found",
                message,
            },
            conversation_runtime::RuntimeError::MessageNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "message_not_found",
                message,
            },
            conversation_runtime::RuntimeError::MessageAlreadyRecalled(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "message_already_recalled",
                message,
            },
            conversation_runtime::RuntimeError::MemberAlreadyExists(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "conversation_member_exists",
                message,
            },
            conversation_runtime::RuntimeError::MemberNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_member_not_found",
                message,
            },
            conversation_runtime::RuntimeError::PermissionDenied(message) => Self {
                status: axum::http::StatusCode::FORBIDDEN,
                code: "conversation_permission_denied",
                message,
            },
            conversation_runtime::RuntimeError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "conversation_conflict",
                message,
            },
            conversation_runtime::RuntimeError::ReadCursorInvalid(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "read_cursor_invalid",
                message,
            },
            conversation_runtime::RuntimeError::Contract(_) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "journal_unavailable",
                message: "commit journal unavailable".into(),
            },
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

impl From<RealtimeClusterError> for ApiError {
    fn from(value: RealtimeClusterError) -> Self {
        Self {
            status: if value.code == "disconnect_fence_store_unavailable"
                || value.code == "checkpoint_store_unavailable"
                || value.code == "subscription_store_unavailable"
            {
                axum::http::StatusCode::SERVICE_UNAVAILABLE
            } else {
                axum::http::StatusCode::CONFLICT
            },
            code: value.code,
            message: value.message,
        }
    }
}

impl From<RealtimeRuntimeError> for ApiError {
    fn from(value: RealtimeRuntimeError) -> Self {
        let status = match value.code {
            "checkpoint_store_unavailable" | "subscription_store_unavailable" => {
                axum::http::StatusCode::SERVICE_UNAVAILABLE
            }
            "checkpoint_store_conflict" | "subscription_store_conflict" => {
                axum::http::StatusCode::CONFLICT
            }
            "checkpoint_store_unsupported" | "subscription_store_unsupported" => {
                axum::http::StatusCode::NOT_IMPLEMENTED
            }
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status,
            code: value.code,
            message: value.message,
        }
    }
}

impl From<PresenceRuntimeError> for ApiError {
    fn from(value: PresenceRuntimeError) -> Self {
        let status = match value.code() {
            "presence_store_unavailable" => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "presence_store_conflict" | "reconnect_required" => axum::http::StatusCode::CONFLICT,
            "presence_store_unsupported" => axum::http::StatusCode::NOT_IMPLEMENTED,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<streaming_service::StreamingError> for ApiError {
    fn from(value: streaming_service::StreamingError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<rtc_signaling_service::RtcError> for ApiError {
    fn from(value: rtc_signaling_service::RtcError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<media_service::MediaError> for ApiError {
    fn from(value: media_service::MediaError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(serde_json::json!({
                "code": self.code,
                "message": self.message
            })),
        )
            .into_response()
    }
}

pub fn resolve_bind_addr() -> String {
    std::env::var("CRAW_CHAT_BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:18090".into())
}

pub fn resolve_runtime_dir() -> PathBuf {
    std::env::var("CRAW_CHAT_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".runtime").join("local-minimal"))
}

const EXPECTED_RUNTIME_STATE_FILES: [&str; 9] = [
    "commit-journal.json",
    "realtime-disconnect-fences.json",
    "realtime-checkpoints.json",
    "realtime-subscriptions.json",
    "presence-state.json",
    "stream-state.json",
    "rtc-state.json",
    "notification-tasks.json",
    "automation-executions.json",
];

struct RuntimeStateValidationFailure {
    parseable: bool,
    error: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRepairActionView {
    pub file_name: String,
    pub path: String,
    pub status: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRepairView {
    pub status: String,
    pub runtime_dir: String,
    pub backup_dir: Option<String>,
    pub repaired_file_count: usize,
    pub skipped_file_count: usize,
    pub before: RuntimeDirInspectionView,
    pub after: RuntimeDirInspectionView,
    pub actions: Vec<RuntimeDirRepairActionView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestoreView {
    pub status: String,
    pub runtime_dir: String,
    pub source_backup_dir: String,
    pub confirmed_preview_fingerprint: Option<String>,
    pub pre_restore_backup_dir: Option<String>,
    pub restored_file_count: usize,
    pub skipped_file_count: usize,
    pub before: RuntimeDirInspectionView,
    pub after: RuntimeDirInspectionView,
    pub actions: Vec<RuntimeDirRepairActionView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirBackupCatalogItemView {
    pub backup_name: String,
    pub backup_dir: String,
    pub operation: String,
    pub has_state_dir: bool,
    pub snapshot_quality: String,
    pub managed_file_count: usize,
    pub missing_file_count: usize,
    pub report_type: Option<String>,
    pub report_status: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirBackupCatalogView {
    pub status: String,
    pub runtime_dir: String,
    pub backups_dir: String,
    pub backup_count: usize,
    pub items: Vec<RuntimeDirBackupCatalogItemView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestorePreviewChangeSummaryView {
    pub summary_kind: String,
    pub source_key_count: usize,
    pub target_key_count: usize,
    pub added_keys: Vec<String>,
    pub removed_keys: Vec<String>,
    pub modified_keys: Vec<String>,
    pub unchanged_key_count: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestorePreviewDomainSummaryView {
    pub summary_kind: String,
    pub added_keys: Vec<String>,
    pub removed_keys: Vec<String>,
    pub owner_node_changed_keys: Vec<String>,
    pub session_changed_keys: Vec<String>,
    pub other_modified_keys: Vec<String>,
    pub unchanged_key_count: usize,
    pub latest_advanced_keys: Option<Vec<String>>,
    pub latest_rewound_keys: Option<Vec<String>>,
    pub acked_advanced_keys: Option<Vec<String>>,
    pub acked_rewound_keys: Option<Vec<String>>,
    pub trimmed_advanced_keys: Option<Vec<String>>,
    pub trimmed_rewound_keys: Option<Vec<String>>,
    pub timestamp_only_changed_keys: Option<Vec<String>>,
    pub added_scope_keys: Option<Vec<String>>,
    pub removed_scope_keys: Option<Vec<String>>,
    pub event_types_added_scope_keys: Option<Vec<String>>,
    pub event_types_removed_scope_keys: Option<Vec<String>>,
    pub subscribed_at_only_changed_scope_keys: Option<Vec<String>>,
    pub unchanged_scope_count: Option<usize>,
    pub stream_state_changed_keys: Option<Vec<String>>,
    pub stream_last_frame_advanced_keys: Option<Vec<String>>,
    pub stream_last_frame_rewound_keys: Option<Vec<String>>,
    pub stream_checkpoint_advanced_keys: Option<Vec<String>>,
    pub stream_checkpoint_rewound_keys: Option<Vec<String>>,
    pub stream_result_message_changed_keys: Option<Vec<String>>,
    pub added_frame_keys: Option<Vec<String>>,
    pub removed_frame_keys: Option<Vec<String>>,
    pub modified_frame_keys: Option<Vec<String>>,
    pub unchanged_frame_count: Option<usize>,
    pub rtc_state_changed_keys: Option<Vec<String>>,
    pub rtc_signaling_stream_changed_keys: Option<Vec<String>>,
    pub rtc_artifact_message_changed_keys: Option<Vec<String>>,
    pub added_signal_keys: Option<Vec<String>>,
    pub removed_signal_keys: Option<Vec<String>>,
    pub modified_signal_keys: Option<Vec<String>>,
    pub unchanged_signal_count: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestorePreviewActionView {
    pub file_name: String,
    pub source_path: String,
    pub target_path: String,
    pub source_exists: bool,
    pub target_exists: bool,
    pub action: String,
    pub detail: String,
    pub change_summary: Option<RuntimeDirRestorePreviewChangeSummaryView>,
    pub domain_summary: Option<RuntimeDirRestorePreviewDomainSummaryView>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirRestorePreviewView {
    pub status: String,
    pub runtime_dir: String,
    pub source_backup_dir: String,
    pub preview_fingerprint: String,
    pub source_snapshot_quality: String,
    pub source_managed_file_count: usize,
    pub source_missing_file_count: usize,
    pub source_report_type: Option<String>,
    pub source_report_status: Option<String>,
    pub would_restore_file_count: usize,
    pub unchanged_file_count: usize,
    pub skipped_file_count: usize,
    pub before: RuntimeDirInspectionView,
    pub actions: Vec<RuntimeDirRestorePreviewActionView>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeDirRestorePreviewFingerprintMaterial<'a> {
    status: &'a str,
    runtime_dir: &'a str,
    source_backup_dir: &'a str,
    source_snapshot_quality: &'a str,
    source_managed_file_count: usize,
    source_missing_file_count: usize,
    source_report_type: Option<&'a str>,
    source_report_status: Option<&'a str>,
    would_restore_file_count: usize,
    unchanged_file_count: usize,
    skipped_file_count: usize,
    before: &'a RuntimeDirInspectionView,
    actions: &'a [RuntimeDirRestorePreviewActionView],
}

#[derive(Clone, Debug)]
struct RuntimeBackupSnapshotSummary {
    backup_name: String,
    backup_dir: String,
    operation: String,
    has_state_dir: bool,
    snapshot_quality: String,
    managed_file_count: usize,
    missing_file_count: usize,
    report_type: Option<String>,
    report_status: Option<String>,
}

fn contract_error_message(error: ContractError) -> String {
    match error {
        ContractError::UnsupportedCapability(message)
        | ContractError::Conflict(message)
        | ContractError::Unavailable(message) => message,
    }
}

fn runtime_state_parse_failure(error: ContractError) -> RuntimeStateValidationFailure {
    RuntimeStateValidationFailure {
        parseable: false,
        error: contract_error_message(error),
    }
}

fn apply_projection_journal_envelopes(
    recorded: &[CommitEnvelope],
    projection_service: &TimelineProjectionService,
    conversation_runtime: &ConversationRuntime<ProjectionJournal>,
    surface: &str,
) -> Result<(), String> {
    for envelope in recorded {
        projection_service.apply(envelope).map_err(|error| {
            format!(
                "failed to replay projection event {} during {surface}: {error:?}",
                envelope.event_id
            )
        })?;
        conversation_runtime
            .apply_recovered_envelope(envelope)
            .map_err(|error| {
                format!(
                    "failed to replay conversation event {} during {surface}: {error:?}",
                    envelope.event_id
                )
            })?;
    }

    Ok(())
}

fn validate_projection_journal_file(
    file_path: &StdPath,
) -> Result<(), RuntimeStateValidationFailure> {
    validate_commit_journal_file(file_path).map_err(runtime_state_parse_failure)?;
    let recorded = read_commit_journal_file(file_path).map_err(runtime_state_parse_failure)?;
    let projection_service = Arc::new(TimelineProjectionService::default());
    let conversation_runtime =
        ConversationRuntime::new(ProjectionJournal::new_memory(projection_service.clone()));

    apply_projection_journal_envelopes(
        recorded.as_slice(),
        projection_service.as_ref(),
        &conversation_runtime,
        "runtime-dir inspection",
    )
    .map_err(|error| RuntimeStateValidationFailure {
        parseable: true,
        error,
    })
}

fn validate_runtime_state_file(
    file_name: &str,
    file_path: &StdPath,
) -> Result<(), RuntimeStateValidationFailure> {
    match file_name {
        "commit-journal.json" => validate_projection_journal_file(file_path),
        "realtime-disconnect-fences.json" => {
            validate_realtime_disconnect_fence_store_file(file_path)
                .map_err(runtime_state_parse_failure)
        }
        "realtime-checkpoints.json" => {
            validate_realtime_checkpoint_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "realtime-subscriptions.json" => validate_realtime_subscription_store_file(file_path)
            .map_err(runtime_state_parse_failure),
        "presence-state.json" => {
            validate_presence_state_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "stream-state.json" => {
            validate_stream_state_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "rtc-state.json" => {
            validate_rtc_state_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "notification-tasks.json" => {
            validate_notification_task_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        "automation-executions.json" => {
            validate_automation_execution_store_file(file_path).map_err(runtime_state_parse_failure)
        }
        _ => Ok(()),
    }
}

fn empty_runtime_state_file_content(file_name: &str) -> &'static str {
    match file_name {
        "commit-journal.json" => "[]\n",
        _ => "{}\n",
    }
}

fn runtime_dir_operation_backup_dir(runtime_dir: &StdPath, operation: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    runtime_dir
        .join("backups")
        .join(format!("runtime-dir-{operation}-{timestamp}"))
}

fn runtime_dir_repair_backup_dir(runtime_dir: &StdPath) -> PathBuf {
    runtime_dir_operation_backup_dir(runtime_dir, "repair")
}

fn runtime_dir_restore_backup_dir(runtime_dir: &StdPath) -> PathBuf {
    runtime_dir_operation_backup_dir(runtime_dir, "restore")
}

fn runtime_backup_operation(backup_name: &str) -> &'static str {
    if backup_name.starts_with("runtime-dir-repair-") {
        "repair"
    } else if backup_name.starts_with("runtime-dir-restore-") {
        "restore"
    } else {
        "unknown"
    }
}

fn runtime_backup_snapshot_quality(managed_file_count: usize) -> &'static str {
    if managed_file_count == 0 {
        "empty_snapshot"
    } else if managed_file_count == EXPECTED_RUNTIME_STATE_FILES.len() {
        "full_snapshot"
    } else {
        "partial_snapshot"
    }
}

fn runtime_backup_report_preview(backup_dir: &StdPath) -> (Option<String>, Option<String>) {
    for (report_type, report_file_name) in [
        ("restore", "restore-report.json"),
        ("repair", "repair-report.json"),
    ] {
        let report_path = backup_dir.join(report_file_name);
        if !report_path.exists() {
            continue;
        }

        let report_status = fs::read(&report_path)
            .ok()
            .and_then(|payload| serde_json::from_slice::<serde_json::Value>(&payload).ok())
            .and_then(|value| {
                value
                    .get("status")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned)
            });
        return (Some(report_type.into()), report_status);
    }

    (None, None)
}

fn stable_runtime_dir_restore_preview_fingerprint(
    material: &RuntimeDirRestorePreviewFingerprintMaterial<'_>,
) -> String {
    let payload = serde_json::to_vec(material)
        .expect("runtime-dir restore preview fingerprint material should serialize");
    let mut hash = 14695981039346656037u64;
    for byte in payload {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(1099511628211u64);
    }
    format!("rvp1-{hash:016x}")
}

fn summarize_runtime_restore_preview_change(
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewChangeSummaryView> {
    let source_value = serde_json::from_slice::<serde_json::Value>(source_payload).ok()?;
    let target_value = serde_json::from_slice::<serde_json::Value>(target_payload).ok()?;

    match (source_value, target_value) {
        (serde_json::Value::Object(source_map), serde_json::Value::Object(target_map)) => {
            let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
            let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
            let added_keys = source_keys
                .difference(&target_keys)
                .cloned()
                .collect::<Vec<_>>();
            let removed_keys = target_keys
                .difference(&source_keys)
                .cloned()
                .collect::<Vec<_>>();

            let mut modified_keys = Vec::new();
            let mut unchanged_key_count = 0usize;
            for key in source_keys.intersection(&target_keys) {
                if source_map.get(key) == target_map.get(key) {
                    unchanged_key_count += 1;
                } else {
                    modified_keys.push(key.clone());
                }
            }

            Some(RuntimeDirRestorePreviewChangeSummaryView {
                summary_kind: "json_object_keys".into(),
                source_key_count: source_map.len(),
                target_key_count: target_map.len(),
                added_keys,
                removed_keys,
                modified_keys,
                unchanged_key_count,
            })
        }
        _ => None,
    }
}

fn summarize_disconnect_fence_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "realtime-disconnect-fences.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeDisconnectFenceRecord>>(source_payload)
            .ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeDisconnectFenceRecord>>(target_payload)
            .ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut owner_node_changed_keys = Vec::new();
    let mut session_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source disconnect fence entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target disconnect fence entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let owner_changed = source_entry.owner_node_id != target_entry.owner_node_id;
        let session_changed = source_entry.session_id != target_entry.session_id;
        if owner_changed {
            owner_node_changed_keys.push(key.clone());
        }
        if session_changed {
            session_changed_keys.push(key.clone());
        }
        if !owner_changed && !session_changed {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "disconnect_fences".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys,
        session_changed_keys,
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: None,
        latest_rewound_keys: None,
        acked_advanced_keys: None,
        acked_rewound_keys: None,
        trimmed_advanced_keys: None,
        trimmed_rewound_keys: None,
        timestamp_only_changed_keys: None,
        added_scope_keys: None,
        removed_scope_keys: None,
        event_types_added_scope_keys: None,
        event_types_removed_scope_keys: None,
        subscribed_at_only_changed_scope_keys: None,
        unchanged_scope_count: None,
        stream_state_changed_keys: None,
        stream_last_frame_advanced_keys: None,
        stream_last_frame_rewound_keys: None,
        stream_checkpoint_advanced_keys: None,
        stream_checkpoint_rewound_keys: None,
        stream_result_message_changed_keys: None,
        added_frame_keys: None,
        removed_frame_keys: None,
        modified_frame_keys: None,
        unchanged_frame_count: None,
        rtc_state_changed_keys: None,
        rtc_signaling_stream_changed_keys: None,
        rtc_artifact_message_changed_keys: None,
        added_signal_keys: None,
        removed_signal_keys: None,
        modified_signal_keys: None,
        unchanged_signal_count: None,
    })
}

fn summarize_realtime_checkpoint_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "realtime-checkpoints.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeCheckpointRecord>>(source_payload)
            .ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeCheckpointRecord>>(target_payload)
            .ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut latest_advanced_keys = Vec::new();
    let mut latest_rewound_keys = Vec::new();
    let mut acked_advanced_keys = Vec::new();
    let mut acked_rewound_keys = Vec::new();
    let mut trimmed_advanced_keys = Vec::new();
    let mut trimmed_rewound_keys = Vec::new();
    let mut timestamp_only_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source realtime checkpoint entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target realtime checkpoint entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let latest_advanced = source_entry.latest_realtime_seq > target_entry.latest_realtime_seq;
        let latest_rewound = source_entry.latest_realtime_seq < target_entry.latest_realtime_seq;
        let acked_advanced = source_entry.acked_through_seq > target_entry.acked_through_seq;
        let acked_rewound = source_entry.acked_through_seq < target_entry.acked_through_seq;
        let trimmed_advanced = source_entry.trimmed_through_seq > target_entry.trimmed_through_seq;
        let trimmed_rewound = source_entry.trimmed_through_seq < target_entry.trimmed_through_seq;

        if latest_advanced {
            latest_advanced_keys.push(key.clone());
        }
        if latest_rewound {
            latest_rewound_keys.push(key.clone());
        }
        if acked_advanced {
            acked_advanced_keys.push(key.clone());
        }
        if acked_rewound {
            acked_rewound_keys.push(key.clone());
        }
        if trimmed_advanced {
            trimmed_advanced_keys.push(key.clone());
        }
        if trimmed_rewound {
            trimmed_rewound_keys.push(key.clone());
        }

        let sequence_changed = latest_advanced
            || latest_rewound
            || acked_advanced
            || acked_rewound
            || trimmed_advanced
            || trimmed_rewound;
        let identity_changed = source_entry.tenant_id != target_entry.tenant_id
            || source_entry.principal_id != target_entry.principal_id
            || source_entry.device_id != target_entry.device_id;
        let timestamp_changed = source_entry.updated_at != target_entry.updated_at;

        if !sequence_changed && !identity_changed && timestamp_changed {
            timestamp_only_changed_keys.push(key.clone());
        }
        if identity_changed {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "realtime_checkpoints".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys: Vec::new(),
        session_changed_keys: Vec::new(),
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: Some(latest_advanced_keys),
        latest_rewound_keys: Some(latest_rewound_keys),
        acked_advanced_keys: Some(acked_advanced_keys),
        acked_rewound_keys: Some(acked_rewound_keys),
        trimmed_advanced_keys: Some(trimmed_advanced_keys),
        trimmed_rewound_keys: Some(trimmed_rewound_keys),
        timestamp_only_changed_keys: Some(timestamp_only_changed_keys),
        added_scope_keys: None,
        removed_scope_keys: None,
        event_types_added_scope_keys: None,
        event_types_removed_scope_keys: None,
        subscribed_at_only_changed_scope_keys: None,
        unchanged_scope_count: None,
        stream_state_changed_keys: None,
        stream_last_frame_advanced_keys: None,
        stream_last_frame_rewound_keys: None,
        stream_checkpoint_advanced_keys: None,
        stream_checkpoint_rewound_keys: None,
        stream_result_message_changed_keys: None,
        added_frame_keys: None,
        removed_frame_keys: None,
        modified_frame_keys: None,
        unchanged_frame_count: None,
        rtc_state_changed_keys: None,
        rtc_signaling_stream_changed_keys: None,
        rtc_artifact_message_changed_keys: None,
        added_signal_keys: None,
        removed_signal_keys: None,
        modified_signal_keys: None,
        unchanged_signal_count: None,
    })
}

#[derive(Clone)]
struct RealtimeSubscriptionScopeSummary {
    exact_item: RealtimeSubscription,
    event_types: BTreeSet<String>,
}

fn realtime_subscription_scope_key(scope_type: &str, scope_id: &str) -> String {
    format!("{scope_type}:{scope_id}")
}

fn qualified_realtime_subscription_scope_key(record_key: &str, scope_key: &str) -> String {
    format!("{record_key}#{scope_key}")
}

fn summarize_realtime_subscription_items(
    items: &[RealtimeSubscription],
) -> Option<BTreeMap<String, RealtimeSubscriptionScopeSummary>> {
    let mut summary = BTreeMap::new();
    for item in items {
        let scope_key =
            realtime_subscription_scope_key(item.scope_type.as_str(), item.scope_id.as_str());
        let scope_summary = RealtimeSubscriptionScopeSummary {
            exact_item: item.clone(),
            event_types: item.event_types.iter().cloned().collect(),
        };
        if summary.insert(scope_key, scope_summary).is_some() {
            return None;
        }
    }
    Some(summary)
}

fn summarize_realtime_subscription_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "realtime-subscriptions.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeSubscriptionRecord>>(source_payload)
            .ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, RealtimeSubscriptionRecord>>(target_payload)
            .ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut added_scope_keys = Vec::new();
    let mut removed_scope_keys = Vec::new();
    let mut event_types_added_scope_keys = Vec::new();
    let mut event_types_removed_scope_keys = Vec::new();
    let mut subscribed_at_only_changed_scope_keys = Vec::new();
    let mut synced_timestamp_only_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;
    let mut unchanged_scope_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source realtime subscription entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target realtime subscription entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let identity_changed = source_entry.tenant_id != target_entry.tenant_id
            || source_entry.principal_id != target_entry.principal_id
            || source_entry.device_id != target_entry.device_id;
        if identity_changed {
            other_modified_keys.push(key.clone());
            continue;
        }

        let Some(source_scope_map) =
            summarize_realtime_subscription_items(source_entry.items.as_slice())
        else {
            other_modified_keys.push(key.clone());
            continue;
        };
        let Some(target_scope_map) =
            summarize_realtime_subscription_items(target_entry.items.as_slice())
        else {
            other_modified_keys.push(key.clone());
            continue;
        };

        let source_scope_keys: BTreeSet<String> = source_scope_map.keys().cloned().collect();
        let target_scope_keys: BTreeSet<String> = target_scope_map.keys().cloned().collect();
        let mut record_has_semantic_change = false;
        let mut record_has_other_change = false;

        for scope_key in source_scope_keys.difference(&target_scope_keys) {
            added_scope_keys.push(qualified_realtime_subscription_scope_key(
                key.as_str(),
                scope_key.as_str(),
            ));
            record_has_semantic_change = true;
        }
        for scope_key in target_scope_keys.difference(&source_scope_keys) {
            removed_scope_keys.push(qualified_realtime_subscription_scope_key(
                key.as_str(),
                scope_key.as_str(),
            ));
            record_has_semantic_change = true;
        }

        for scope_key in source_scope_keys.intersection(&target_scope_keys) {
            let source_scope = source_scope_map
                .get(scope_key)
                .expect("source realtime subscription scope should exist");
            let target_scope = target_scope_map
                .get(scope_key)
                .expect("target realtime subscription scope should exist");
            if source_scope.exact_item == target_scope.exact_item {
                unchanged_scope_count += 1;
                continue;
            }

            let qualified_scope_key =
                qualified_realtime_subscription_scope_key(key.as_str(), scope_key.as_str());
            let source_has_added_event_types = source_scope
                .event_types
                .difference(&target_scope.event_types)
                .next()
                .is_some();
            let source_has_removed_event_types = target_scope
                .event_types
                .difference(&source_scope.event_types)
                .next()
                .is_some();
            let subscribed_at_changed =
                source_scope.exact_item.subscribed_at != target_scope.exact_item.subscribed_at;

            if source_has_added_event_types {
                event_types_added_scope_keys.push(qualified_scope_key.clone());
                record_has_semantic_change = true;
            }
            if source_has_removed_event_types {
                event_types_removed_scope_keys.push(qualified_scope_key.clone());
                record_has_semantic_change = true;
            }
            if !source_has_added_event_types
                && !source_has_removed_event_types
                && subscribed_at_changed
            {
                subscribed_at_only_changed_scope_keys.push(qualified_scope_key);
                record_has_semantic_change = true;
            } else if !source_has_added_event_types
                && !source_has_removed_event_types
                && !subscribed_at_changed
            {
                record_has_other_change = true;
            }
        }

        let synced_at_changed = source_entry.synced_at != target_entry.synced_at;
        if !record_has_semantic_change && !record_has_other_change && synced_at_changed {
            synced_timestamp_only_changed_keys.push(key.clone());
        } else if record_has_other_change {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "realtime_subscriptions".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys: Vec::new(),
        session_changed_keys: Vec::new(),
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: None,
        latest_rewound_keys: None,
        acked_advanced_keys: None,
        acked_rewound_keys: None,
        trimmed_advanced_keys: None,
        trimmed_rewound_keys: None,
        timestamp_only_changed_keys: Some(synced_timestamp_only_changed_keys),
        added_scope_keys: Some(added_scope_keys),
        removed_scope_keys: Some(removed_scope_keys),
        event_types_added_scope_keys: Some(event_types_added_scope_keys),
        event_types_removed_scope_keys: Some(event_types_removed_scope_keys),
        subscribed_at_only_changed_scope_keys: Some(subscribed_at_only_changed_scope_keys),
        unchanged_scope_count: Some(unchanged_scope_count),
        stream_state_changed_keys: None,
        stream_last_frame_advanced_keys: None,
        stream_last_frame_rewound_keys: None,
        stream_checkpoint_advanced_keys: None,
        stream_checkpoint_rewound_keys: None,
        stream_result_message_changed_keys: None,
        added_frame_keys: None,
        removed_frame_keys: None,
        modified_frame_keys: None,
        unchanged_frame_count: None,
        rtc_state_changed_keys: None,
        rtc_signaling_stream_changed_keys: None,
        rtc_artifact_message_changed_keys: None,
        added_signal_keys: None,
        removed_signal_keys: None,
        modified_signal_keys: None,
        unchanged_signal_count: None,
    })
}

fn compare_optional_u64(source: Option<u64>, target: Option<u64>) -> std::cmp::Ordering {
    match (source, target) {
        (Some(source), Some(target)) => source.cmp(&target),
        (Some(_), None) => std::cmp::Ordering::Greater,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (None, None) => std::cmp::Ordering::Equal,
    }
}

fn qualified_stream_frame_key(record_key: &str, frame_seq: u64) -> String {
    format!("{record_key}#frame:{frame_seq}")
}

fn summarize_stream_frames(
    frames: &[im_domain_core::stream::StreamFrame],
) -> Option<BTreeMap<u64, im_domain_core::stream::StreamFrame>> {
    let mut summary = BTreeMap::new();
    for frame in frames {
        if summary.insert(frame.frame_seq, frame.clone()).is_some() {
            return None;
        }
    }
    Some(summary)
}

fn summarize_stream_state_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "stream-state.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, StreamStateRecord>>(source_payload).ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, StreamStateRecord>>(target_payload).ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut stream_state_changed_keys = Vec::new();
    let mut stream_last_frame_advanced_keys = Vec::new();
    let mut stream_last_frame_rewound_keys = Vec::new();
    let mut stream_checkpoint_advanced_keys = Vec::new();
    let mut stream_checkpoint_rewound_keys = Vec::new();
    let mut stream_result_message_changed_keys = Vec::new();
    let mut added_frame_keys = Vec::new();
    let mut removed_frame_keys = Vec::new();
    let mut modified_frame_keys = Vec::new();
    let mut timestamp_only_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;
    let mut unchanged_frame_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source stream state entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target stream state entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let identity_changed = source_entry.tenant_id != target_entry.tenant_id
            || source_entry.stream_id != target_entry.stream_id;
        if identity_changed {
            other_modified_keys.push(key.clone());
            continue;
        }

        let mut record_has_semantic_change = false;
        let mut record_has_other_change = false;

        let state_changed = source_entry.session.state != target_entry.session.state;
        let last_frame_cmp = source_entry
            .session
            .last_frame_seq
            .cmp(&target_entry.session.last_frame_seq);
        let checkpoint_cmp = compare_optional_u64(
            source_entry.session.last_checkpoint_seq,
            target_entry.session.last_checkpoint_seq,
        );
        let result_message_changed =
            source_entry.session.result_message_id != target_entry.session.result_message_id;
        let session_contract_changed = source_entry.session.stream_type
            != target_entry.session.stream_type
            || source_entry.session.scope_kind != target_entry.session.scope_kind
            || source_entry.session.scope_id != target_entry.session.scope_id
            || source_entry.session.durability_class != target_entry.session.durability_class
            || source_entry.session.ordering_scope != target_entry.session.ordering_scope
            || source_entry.session.schema_ref != target_entry.session.schema_ref;

        if state_changed {
            stream_state_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if last_frame_cmp == std::cmp::Ordering::Greater {
            stream_last_frame_advanced_keys.push(key.clone());
            record_has_semantic_change = true;
        } else if last_frame_cmp == std::cmp::Ordering::Less {
            stream_last_frame_rewound_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if checkpoint_cmp == std::cmp::Ordering::Greater {
            stream_checkpoint_advanced_keys.push(key.clone());
            record_has_semantic_change = true;
        } else if checkpoint_cmp == std::cmp::Ordering::Less {
            stream_checkpoint_rewound_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if result_message_changed {
            stream_result_message_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if session_contract_changed {
            record_has_other_change = true;
        }

        let Some(source_frames) = summarize_stream_frames(source_entry.frames.as_slice()) else {
            other_modified_keys.push(key.clone());
            continue;
        };
        let Some(target_frames) = summarize_stream_frames(target_entry.frames.as_slice()) else {
            other_modified_keys.push(key.clone());
            continue;
        };

        let source_frame_keys: BTreeSet<u64> = source_frames.keys().copied().collect();
        let target_frame_keys: BTreeSet<u64> = target_frames.keys().copied().collect();
        for frame_seq in source_frame_keys.difference(&target_frame_keys) {
            added_frame_keys.push(qualified_stream_frame_key(key.as_str(), *frame_seq));
            record_has_semantic_change = true;
        }
        for frame_seq in target_frame_keys.difference(&source_frame_keys) {
            removed_frame_keys.push(qualified_stream_frame_key(key.as_str(), *frame_seq));
            record_has_semantic_change = true;
        }
        for frame_seq in source_frame_keys.intersection(&target_frame_keys) {
            let source_frame = source_frames
                .get(frame_seq)
                .expect("source stream frame should exist");
            let target_frame = target_frames
                .get(frame_seq)
                .expect("target stream frame should exist");
            if source_frame == target_frame {
                unchanged_frame_count += 1;
            } else {
                modified_frame_keys.push(qualified_stream_frame_key(key.as_str(), *frame_seq));
                record_has_semantic_change = true;
            }
        }

        let updated_at_changed = source_entry.updated_at != target_entry.updated_at;
        if !record_has_semantic_change && !record_has_other_change && updated_at_changed {
            timestamp_only_changed_keys.push(key.clone());
        } else if record_has_other_change {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "stream_state".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys: Vec::new(),
        session_changed_keys: Vec::new(),
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: None,
        latest_rewound_keys: None,
        acked_advanced_keys: None,
        acked_rewound_keys: None,
        trimmed_advanced_keys: None,
        trimmed_rewound_keys: None,
        timestamp_only_changed_keys: Some(timestamp_only_changed_keys),
        added_scope_keys: None,
        removed_scope_keys: None,
        event_types_added_scope_keys: None,
        event_types_removed_scope_keys: None,
        subscribed_at_only_changed_scope_keys: None,
        unchanged_scope_count: None,
        stream_state_changed_keys: Some(stream_state_changed_keys),
        stream_last_frame_advanced_keys: Some(stream_last_frame_advanced_keys),
        stream_last_frame_rewound_keys: Some(stream_last_frame_rewound_keys),
        stream_checkpoint_advanced_keys: Some(stream_checkpoint_advanced_keys),
        stream_checkpoint_rewound_keys: Some(stream_checkpoint_rewound_keys),
        stream_result_message_changed_keys: Some(stream_result_message_changed_keys),
        added_frame_keys: Some(added_frame_keys),
        removed_frame_keys: Some(removed_frame_keys),
        modified_frame_keys: Some(modified_frame_keys),
        unchanged_frame_count: Some(unchanged_frame_count),
        rtc_state_changed_keys: None,
        rtc_signaling_stream_changed_keys: None,
        rtc_artifact_message_changed_keys: None,
        added_signal_keys: None,
        removed_signal_keys: None,
        modified_signal_keys: None,
        unchanged_signal_count: None,
    })
}

fn qualified_rtc_signal_key(record_key: &str, signal_index: usize) -> String {
    format!("{record_key}#signal:{signal_index}")
}

fn summarize_rtc_signals(
    signals: &[im_domain_core::rtc::RtcSignalEvent],
) -> BTreeMap<usize, im_domain_core::rtc::RtcSignalEvent> {
    signals
        .iter()
        .enumerate()
        .map(|(index, signal)| (index, signal.clone()))
        .collect()
}

fn summarize_rtc_state_restore_preview_change(
    file_name: &str,
    source_payload: &[u8],
    target_payload: &[u8],
) -> Option<RuntimeDirRestorePreviewDomainSummaryView> {
    if file_name != "rtc-state.json" {
        return None;
    }

    let source_map =
        serde_json::from_slice::<BTreeMap<String, RtcStateRecord>>(source_payload).ok()?;
    let target_map =
        serde_json::from_slice::<BTreeMap<String, RtcStateRecord>>(target_payload).ok()?;

    let source_keys: BTreeSet<String> = source_map.keys().cloned().collect();
    let target_keys: BTreeSet<String> = target_map.keys().cloned().collect();
    let added_keys = source_keys
        .difference(&target_keys)
        .cloned()
        .collect::<Vec<_>>();
    let removed_keys = target_keys
        .difference(&source_keys)
        .cloned()
        .collect::<Vec<_>>();

    let mut rtc_state_changed_keys = Vec::new();
    let mut rtc_signaling_stream_changed_keys = Vec::new();
    let mut rtc_artifact_message_changed_keys = Vec::new();
    let mut added_signal_keys = Vec::new();
    let mut removed_signal_keys = Vec::new();
    let mut modified_signal_keys = Vec::new();
    let mut timestamp_only_changed_keys = Vec::new();
    let mut other_modified_keys = Vec::new();
    let mut unchanged_key_count = 0usize;
    let mut unchanged_signal_count = 0usize;

    for key in source_keys.intersection(&target_keys) {
        let source_entry = source_map
            .get(key)
            .expect("source rtc state entry should exist");
        let target_entry = target_map
            .get(key)
            .expect("target rtc state entry should exist");
        if source_entry == target_entry {
            unchanged_key_count += 1;
            continue;
        }

        let identity_changed = source_entry.tenant_id != target_entry.tenant_id
            || source_entry.rtc_session_id != target_entry.rtc_session_id;
        if identity_changed {
            other_modified_keys.push(key.clone());
            continue;
        }

        let mut record_has_semantic_change = false;
        let mut record_has_other_change = false;

        let state_changed = source_entry.session.state != target_entry.session.state;
        let signaling_stream_changed =
            source_entry.session.signaling_stream_id != target_entry.session.signaling_stream_id;
        let artifact_message_changed =
            source_entry.session.artifact_message_id != target_entry.session.artifact_message_id;
        let session_contract_changed = source_entry.session.conversation_id
            != target_entry.session.conversation_id
            || source_entry.session.rtc_mode != target_entry.session.rtc_mode
            || source_entry.session.initiator_id != target_entry.session.initiator_id;

        if state_changed {
            rtc_state_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if signaling_stream_changed {
            rtc_signaling_stream_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if artifact_message_changed {
            rtc_artifact_message_changed_keys.push(key.clone());
            record_has_semantic_change = true;
        }
        if session_contract_changed {
            record_has_other_change = true;
        }

        let source_signals = summarize_rtc_signals(source_entry.signals.as_slice());
        let target_signals = summarize_rtc_signals(target_entry.signals.as_slice());
        let source_signal_keys: BTreeSet<usize> = source_signals.keys().copied().collect();
        let target_signal_keys: BTreeSet<usize> = target_signals.keys().copied().collect();

        for signal_index in source_signal_keys.difference(&target_signal_keys) {
            added_signal_keys.push(qualified_rtc_signal_key(key.as_str(), *signal_index));
            record_has_semantic_change = true;
        }
        for signal_index in target_signal_keys.difference(&source_signal_keys) {
            removed_signal_keys.push(qualified_rtc_signal_key(key.as_str(), *signal_index));
            record_has_semantic_change = true;
        }
        for signal_index in source_signal_keys.intersection(&target_signal_keys) {
            let source_signal = source_signals
                .get(signal_index)
                .expect("source rtc signal should exist");
            let target_signal = target_signals
                .get(signal_index)
                .expect("target rtc signal should exist");
            if source_signal == target_signal {
                unchanged_signal_count += 1;
            } else {
                modified_signal_keys.push(qualified_rtc_signal_key(key.as_str(), *signal_index));
                record_has_semantic_change = true;
            }
        }

        let updated_at_changed = source_entry.updated_at != target_entry.updated_at;
        if !record_has_semantic_change && !record_has_other_change && updated_at_changed {
            timestamp_only_changed_keys.push(key.clone());
        } else if record_has_other_change {
            other_modified_keys.push(key.clone());
        }
    }

    Some(RuntimeDirRestorePreviewDomainSummaryView {
        summary_kind: "rtc_state".into(),
        added_keys,
        removed_keys,
        owner_node_changed_keys: Vec::new(),
        session_changed_keys: Vec::new(),
        other_modified_keys,
        unchanged_key_count,
        latest_advanced_keys: None,
        latest_rewound_keys: None,
        acked_advanced_keys: None,
        acked_rewound_keys: None,
        trimmed_advanced_keys: None,
        trimmed_rewound_keys: None,
        timestamp_only_changed_keys: Some(timestamp_only_changed_keys),
        added_scope_keys: None,
        removed_scope_keys: None,
        event_types_added_scope_keys: None,
        event_types_removed_scope_keys: None,
        subscribed_at_only_changed_scope_keys: None,
        unchanged_scope_count: None,
        stream_state_changed_keys: None,
        stream_last_frame_advanced_keys: None,
        stream_last_frame_rewound_keys: None,
        stream_checkpoint_advanced_keys: None,
        stream_checkpoint_rewound_keys: None,
        stream_result_message_changed_keys: None,
        added_frame_keys: None,
        removed_frame_keys: None,
        modified_frame_keys: None,
        unchanged_frame_count: None,
        rtc_state_changed_keys: Some(rtc_state_changed_keys),
        rtc_signaling_stream_changed_keys: Some(rtc_signaling_stream_changed_keys),
        rtc_artifact_message_changed_keys: Some(rtc_artifact_message_changed_keys),
        added_signal_keys: Some(added_signal_keys),
        removed_signal_keys: Some(removed_signal_keys),
        modified_signal_keys: Some(modified_signal_keys),
        unchanged_signal_count: Some(unchanged_signal_count),
    })
}

fn describe_runtime_backup_snapshot(
    backup_dir: &StdPath,
    backup_name: impl Into<String>,
) -> RuntimeBackupSnapshotSummary {
    let backup_name = backup_name.into();
    let state_dir = backup_dir.join("state");
    let has_state_dir = state_dir.exists();
    let managed_file_count = EXPECTED_RUNTIME_STATE_FILES
        .iter()
        .filter(|file_name| state_dir.join(file_name).exists())
        .count();
    let missing_file_count = EXPECTED_RUNTIME_STATE_FILES.len() - managed_file_count;
    let (report_type, report_status) = runtime_backup_report_preview(backup_dir);

    RuntimeBackupSnapshotSummary {
        backup_name: backup_name.clone(),
        backup_dir: backup_dir.display().to_string(),
        operation: runtime_backup_operation(backup_name.as_str()).into(),
        has_state_dir,
        snapshot_quality: runtime_backup_snapshot_quality(managed_file_count).into(),
        managed_file_count,
        missing_file_count,
        report_type,
        report_status,
    }
}

fn validate_runtime_backup_source(
    backup_dir: &StdPath,
) -> Result<(PathBuf, RuntimeBackupSnapshotSummary), String> {
    if !backup_dir.exists() {
        return Err(format!(
            "backup dir does not exist: {}",
            backup_dir.display()
        ));
    }

    let source_state_dir = backup_dir.join("state");
    if !source_state_dir.exists() {
        return Err(format!(
            "backup state dir does not exist: {}",
            source_state_dir.display()
        ));
    }

    let backup_name = backup_dir
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| backup_dir.display().to_string());
    let summary = describe_runtime_backup_snapshot(backup_dir, backup_name);
    Ok((source_state_dir, summary))
}

fn snapshot_runtime_state_files(state_dir: &StdPath, backup_dir: &StdPath) {
    let backup_state_dir = backup_dir.join("state");
    fs::create_dir_all(&backup_state_dir).unwrap_or_else(|error| {
        panic!(
            "failed to create runtime-dir repair backup state dir {}: {error}",
            backup_state_dir.display()
        )
    });

    for file_name in EXPECTED_RUNTIME_STATE_FILES {
        let source = state_dir.join(file_name);
        if !source.exists() {
            continue;
        }

        let target = backup_state_dir.join(file_name);
        fs::copy(&source, &target).unwrap_or_else(|error| {
            panic!(
                "failed to snapshot runtime-dir state file {} to {}: {error}",
                source.display(),
                target.display()
            )
        });
    }
}

fn write_runtime_dir_repair_report(backup_dir: &StdPath, report: &RuntimeDirRepairView) {
    let report_path = backup_dir.join("repair-report.json");
    let payload = serde_json::to_vec_pretty(report)
        .expect("runtime-dir repair report should serialize to json");
    fs::write(&report_path, payload).unwrap_or_else(|error| {
        panic!(
            "failed to write runtime-dir repair report {}: {error}",
            report_path.display()
        )
    });
}

fn write_runtime_dir_restore_report(backup_dir: &StdPath, report: &RuntimeDirRestoreView) {
    let report_path = backup_dir.join("restore-report.json");
    let payload = serde_json::to_vec_pretty(report)
        .expect("runtime-dir restore report should serialize to json");
    fs::write(&report_path, payload).unwrap_or_else(|error| {
        panic!(
            "failed to write runtime-dir restore report {}: {error}",
            report_path.display()
        )
    });
}

pub fn repair_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> RuntimeDirRepairView {
    let runtime_dir = runtime_dir.as_ref();
    let state_dir = runtime_dir.join("state");
    let before = inspect_runtime_dir(runtime_dir);
    let backup_dir = runtime_dir_repair_backup_dir(runtime_dir);

    fs::create_dir_all(&state_dir).unwrap_or_else(|error| {
        panic!(
            "failed to create runtime-dir state dir {} before repair: {error}",
            state_dir.display()
        )
    });
    fs::create_dir_all(&backup_dir).unwrap_or_else(|error| {
        panic!(
            "failed to create runtime-dir repair backup dir {}: {error}",
            backup_dir.display()
        )
    });
    snapshot_runtime_state_files(state_dir.as_path(), backup_dir.as_path());

    let mut actions = Vec::new();
    let mut repaired_file_count = 0usize;
    let mut skipped_file_count = 0usize;

    for file in &before.files {
        let target_path = state_dir.join(file.file_name.as_str());
        match file.status.as_str() {
            "missing" => {
                fs::write(
                    &target_path,
                    empty_runtime_state_file_content(file.file_name.as_str()),
                )
                .unwrap_or_else(|error| {
                    panic!(
                        "failed to recreate missing runtime-dir file {}: {error}",
                        target_path.display()
                    )
                });
                actions.push(RuntimeDirRepairActionView {
                    file_name: file.file_name.clone(),
                    path: target_path.display().to_string(),
                    status: "repaired".into(),
                    detail: "recreated_missing_file".into(),
                });
                repaired_file_count += 1;
            }
            "corrupt" => {
                actions.push(RuntimeDirRepairActionView {
                    file_name: file.file_name.clone(),
                    path: target_path.display().to_string(),
                    status: "skipped".into(),
                    detail: "left_corrupt_file_untouched".into(),
                });
                skipped_file_count += 1;
            }
            _ => {}
        }
    }

    let after = inspect_runtime_dir(runtime_dir);
    let status = if repaired_file_count > 0 && after.status == "ok" {
        "repaired"
    } else if repaired_file_count > 0 || skipped_file_count > 0 {
        "partial"
    } else {
        "noop"
    };

    let report = RuntimeDirRepairView {
        status: status.into(),
        runtime_dir: runtime_dir.display().to_string(),
        backup_dir: Some(backup_dir.display().to_string()),
        repaired_file_count,
        skipped_file_count,
        before,
        after,
        actions,
    };
    write_runtime_dir_repair_report(backup_dir.as_path(), &report);
    report
}

pub fn restore_runtime_dir(
    runtime_dir: impl AsRef<StdPath>,
    backup_dir: impl AsRef<StdPath>,
) -> Result<RuntimeDirRestoreView, String> {
    restore_runtime_dir_with_expected_preview_fingerprint(runtime_dir, backup_dir, None)
}

pub fn restore_runtime_dir_with_expected_preview_fingerprint(
    runtime_dir: impl AsRef<StdPath>,
    backup_dir: impl AsRef<StdPath>,
    expected_preview_fingerprint: Option<&str>,
) -> Result<RuntimeDirRestoreView, String> {
    let runtime_dir = runtime_dir.as_ref();
    let backup_dir = backup_dir.as_ref();
    let confirmed_preview_fingerprint = if let Some(expected_preview_fingerprint) =
        expected_preview_fingerprint
    {
        let preview = preview_restore_runtime_dir(runtime_dir, backup_dir)?;
        if preview.preview_fingerprint != expected_preview_fingerprint {
            return Err(format!(
                "preview fingerprint mismatch: expected {expected_preview_fingerprint}, actual {}",
                preview.preview_fingerprint
            ));
        }
        Some(preview.preview_fingerprint)
    } else {
        None
    };
    let (source_state_dir, _) = validate_runtime_backup_source(backup_dir)?;

    let state_dir = runtime_dir.join("state");
    let before = inspect_runtime_dir(runtime_dir);
    let pre_restore_backup_dir = runtime_dir_restore_backup_dir(runtime_dir);

    fs::create_dir_all(&state_dir).map_err(|error| {
        format!(
            "failed to create runtime-dir state dir {} before restore: {error}",
            state_dir.display()
        )
    })?;
    fs::create_dir_all(&pre_restore_backup_dir).map_err(|error| {
        format!(
            "failed to create runtime-dir restore backup dir {}: {error}",
            pre_restore_backup_dir.display()
        )
    })?;
    snapshot_runtime_state_files(state_dir.as_path(), pre_restore_backup_dir.as_path());

    let mut actions = Vec::new();
    let mut restored_file_count = 0usize;
    let mut skipped_file_count = 0usize;

    for file_name in EXPECTED_RUNTIME_STATE_FILES {
        let source = source_state_dir.join(file_name);
        let target = state_dir.join(file_name);
        if source.exists() {
            fs::copy(&source, &target).map_err(|error| {
                format!(
                    "failed to restore runtime-dir state file {} to {}: {error}",
                    source.display(),
                    target.display()
                )
            })?;
            actions.push(RuntimeDirRepairActionView {
                file_name: file_name.into(),
                path: target.display().to_string(),
                status: "restored".into(),
                detail: "copied_from_backup_snapshot".into(),
            });
            restored_file_count += 1;
        } else {
            actions.push(RuntimeDirRepairActionView {
                file_name: file_name.into(),
                path: target.display().to_string(),
                status: "skipped".into(),
                detail: "missing_in_source_backup_snapshot".into(),
            });
            skipped_file_count += 1;
        }
    }

    let after = inspect_runtime_dir(runtime_dir);
    let status = if restored_file_count > 0 && after.status == "ok" {
        "restored"
    } else if restored_file_count > 0 || skipped_file_count > 0 {
        "partial"
    } else {
        "noop"
    };

    let report = RuntimeDirRestoreView {
        status: status.into(),
        runtime_dir: runtime_dir.display().to_string(),
        source_backup_dir: backup_dir.display().to_string(),
        confirmed_preview_fingerprint,
        pre_restore_backup_dir: Some(pre_restore_backup_dir.display().to_string()),
        restored_file_count,
        skipped_file_count,
        before,
        after,
        actions,
    };
    write_runtime_dir_restore_report(pre_restore_backup_dir.as_path(), &report);
    Ok(report)
}

pub fn preview_restore_runtime_dir(
    runtime_dir: impl AsRef<StdPath>,
    backup_dir: impl AsRef<StdPath>,
) -> Result<RuntimeDirRestorePreviewView, String> {
    let runtime_dir = runtime_dir.as_ref();
    let backup_dir = backup_dir.as_ref();
    let (source_state_dir, source_summary) = validate_runtime_backup_source(backup_dir)?;

    let state_dir = runtime_dir.join("state");
    let before = inspect_runtime_dir(runtime_dir);
    let mut actions = Vec::new();
    let mut would_restore_file_count = 0usize;
    let mut unchanged_file_count = 0usize;
    let mut skipped_file_count = 0usize;

    for file_name in EXPECTED_RUNTIME_STATE_FILES {
        let source_path = source_state_dir.join(file_name);
        let target_path = state_dir.join(file_name);
        let source_exists = source_path.exists();
        let target_exists = target_path.exists();
        let mut change_summary = None;
        let mut domain_summary = None;

        let (action, detail) = if !source_exists {
            skipped_file_count += 1;
            ("skip", "missing_in_source_backup_snapshot")
        } else if target_exists {
            let source_payload = fs::read(&source_path).map_err(|error| {
                format!(
                    "failed to read source backup file {} during restore preview: {error}",
                    source_path.display()
                )
            })?;
            let target_payload = fs::read(&target_path).map_err(|error| {
                format!(
                    "failed to read runtime state file {} during restore preview: {error}",
                    target_path.display()
                )
            })?;
            if source_payload == target_payload {
                unchanged_file_count += 1;
                ("noop", "source_matches_target")
            } else {
                would_restore_file_count += 1;
                change_summary =
                    summarize_runtime_restore_preview_change(&source_payload, &target_payload);
                domain_summary = summarize_disconnect_fence_restore_preview_change(
                    file_name,
                    &source_payload,
                    &target_payload,
                )
                .or_else(|| {
                    summarize_realtime_checkpoint_restore_preview_change(
                        file_name,
                        &source_payload,
                        &target_payload,
                    )
                })
                .or_else(|| {
                    summarize_realtime_subscription_restore_preview_change(
                        file_name,
                        &source_payload,
                        &target_payload,
                    )
                })
                .or_else(|| {
                    summarize_stream_state_restore_preview_change(
                        file_name,
                        &source_payload,
                        &target_payload,
                    )
                })
                .or_else(|| {
                    summarize_rtc_state_restore_preview_change(
                        file_name,
                        &source_payload,
                        &target_payload,
                    )
                });
                ("would_restore", "content_differs")
            }
        } else {
            would_restore_file_count += 1;
            ("would_restore", "target_missing")
        };

        actions.push(RuntimeDirRestorePreviewActionView {
            file_name: file_name.into(),
            source_path: source_path.display().to_string(),
            target_path: target_path.display().to_string(),
            source_exists,
            target_exists,
            action: action.into(),
            detail: detail.into(),
            change_summary,
            domain_summary,
        });
    }

    let status = if would_restore_file_count == 0 && skipped_file_count == 0 {
        "noop"
    } else if skipped_file_count == 0 {
        "ready"
    } else {
        "partial"
    };

    let runtime_dir_text = runtime_dir.display().to_string();
    let source_backup_dir_text = backup_dir.display().to_string();
    let fingerprint_material = RuntimeDirRestorePreviewFingerprintMaterial {
        status,
        runtime_dir: runtime_dir_text.as_str(),
        source_backup_dir: source_backup_dir_text.as_str(),
        source_snapshot_quality: source_summary.snapshot_quality.as_str(),
        source_managed_file_count: source_summary.managed_file_count,
        source_missing_file_count: source_summary.missing_file_count,
        source_report_type: source_summary.report_type.as_deref(),
        source_report_status: source_summary.report_status.as_deref(),
        would_restore_file_count,
        unchanged_file_count,
        skipped_file_count,
        before: &before,
        actions: actions.as_slice(),
    };
    let preview_fingerprint = stable_runtime_dir_restore_preview_fingerprint(&fingerprint_material);

    Ok(RuntimeDirRestorePreviewView {
        status: status.into(),
        runtime_dir: runtime_dir_text,
        source_backup_dir: source_backup_dir_text,
        preview_fingerprint,
        source_snapshot_quality: source_summary.snapshot_quality,
        source_managed_file_count: source_summary.managed_file_count,
        source_missing_file_count: source_summary.missing_file_count,
        source_report_type: source_summary.report_type,
        source_report_status: source_summary.report_status,
        would_restore_file_count,
        unchanged_file_count,
        skipped_file_count,
        before,
        actions,
    })
}

pub fn list_runtime_backups(runtime_dir: impl AsRef<StdPath>) -> RuntimeDirBackupCatalogView {
    let runtime_dir = runtime_dir.as_ref();
    let backups_dir = runtime_dir.join("backups");
    let mut items = Vec::new();

    if backups_dir.exists() {
        let entries = fs::read_dir(&backups_dir).unwrap_or_else(|error| {
            panic!(
                "failed to read runtime-dir backups dir {}: {error}",
                backups_dir.display()
            )
        });

        for entry in entries {
            let entry = entry.unwrap_or_else(|error| {
                panic!(
                    "failed to read runtime-dir backup directory entry under {}: {error}",
                    backups_dir.display()
                )
            });
            let backup_dir = entry.path();
            if !backup_dir.is_dir() {
                continue;
            }

            let summary = describe_runtime_backup_snapshot(
                backup_dir.as_path(),
                entry.file_name().to_string_lossy(),
            );

            items.push(RuntimeDirBackupCatalogItemView {
                backup_name: summary.backup_name,
                backup_dir: summary.backup_dir,
                operation: summary.operation,
                has_state_dir: summary.has_state_dir,
                snapshot_quality: summary.snapshot_quality,
                managed_file_count: summary.managed_file_count,
                missing_file_count: summary.missing_file_count,
                report_type: summary.report_type,
                report_status: summary.report_status,
            });
        }
    }

    items.sort_by(|left, right| right.backup_name.cmp(&left.backup_name));

    RuntimeDirBackupCatalogView {
        status: if items.is_empty() { "empty" } else { "ok" }.into(),
        runtime_dir: runtime_dir.display().to_string(),
        backups_dir: backups_dir.display().to_string(),
        backup_count: items.len(),
        items,
    }
}

pub fn inspect_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> RuntimeDirInspectionView {
    let runtime_dir = runtime_dir.as_ref();
    let state_dir = runtime_dir.join("state");
    let mut files = Vec::new();

    for file_name in EXPECTED_RUNTIME_STATE_FILES {
        let path = state_dir.join(file_name);
        if !path.exists() {
            files.push(RuntimeDirInspectionItem {
                file_name: file_name.into(),
                path: path.display().to_string(),
                required: true,
                exists: false,
                parseable: false,
                status: "missing".into(),
                size_bytes: None,
                parse_error: None,
                recommended_action: "recreate_on_next_managed_start_or_write".into(),
            });
            continue;
        }

        let size_bytes = fs::metadata(&path).ok().map(|metadata| metadata.len());
        match validate_runtime_state_file(file_name, path.as_path()) {
            Ok(()) => files.push(RuntimeDirInspectionItem {
                file_name: file_name.into(),
                path: path.display().to_string(),
                required: true,
                exists: true,
                parseable: true,
                status: "ok".into(),
                size_bytes,
                parse_error: None,
                recommended_action: "none".into(),
            }),
            Err(validation) => files.push(RuntimeDirInspectionItem {
                file_name: file_name.into(),
                path: path.display().to_string(),
                required: true,
                exists: true,
                parseable: validation.parseable,
                status: "corrupt".into(),
                size_bytes,
                parse_error: Some(validation.error),
                recommended_action: "manual_json_repair_or_restore".into(),
            }),
        }
    }

    let healthy_file_count = files.iter().filter(|file| file.status == "ok").count();
    let missing_file_count = files.iter().filter(|file| file.status == "missing").count();
    let corrupt_file_count = files.iter().filter(|file| file.status == "corrupt").count();
    let status = if missing_file_count == 0 && corrupt_file_count == 0 {
        "ok"
    } else {
        "degraded"
    };

    RuntimeDirInspectionView {
        status: status.into(),
        runtime_dir: Some(runtime_dir.display().to_string()),
        state_dir: Some(state_dir.display().to_string()),
        healthy_file_count,
        missing_file_count,
        corrupt_file_count,
        files,
    }
}

pub fn format_runtime_dir_repair(view: &RuntimeDirRepairView) -> String {
    let mut lines = vec![format!("runtime-dir repair status: {}", view.status)];
    lines.push(format!("runtime-dir: {}", view.runtime_dir));
    if let Some(backup_dir) = view.backup_dir.as_deref() {
        lines.push(format!("backup-dir: {backup_dir}"));
    }
    lines.push(format!("repaired files: {}", view.repaired_file_count));
    lines.push(format!("skipped files: {}", view.skipped_file_count));
    lines.push(format!("before status: {}", view.before.status));
    lines.push(format!("after status: {}", view.after.status));

    if view.actions.is_empty() {
        lines.push("actions: none".into());
    } else {
        lines.push("actions:".into());
        for action in &view.actions {
            lines.push(format!(
                "- {} {} ({})",
                action.status, action.file_name, action.detail
            ));
        }
    }

    lines.join("\n")
}

pub fn format_runtime_dir_restore(view: &RuntimeDirRestoreView) -> String {
    let mut lines = vec![format!("runtime-dir restore status: {}", view.status)];
    lines.push(format!("runtime-dir: {}", view.runtime_dir));
    lines.push(format!("source-backup-dir: {}", view.source_backup_dir));
    if let Some(confirmed_preview_fingerprint) = view.confirmed_preview_fingerprint.as_deref() {
        lines.push(format!(
            "confirmed-preview-fingerprint: {confirmed_preview_fingerprint}"
        ));
    }
    if let Some(pre_restore_backup_dir) = view.pre_restore_backup_dir.as_deref() {
        lines.push(format!("pre-restore-backup-dir: {pre_restore_backup_dir}"));
    }
    lines.push(format!("restored files: {}", view.restored_file_count));
    lines.push(format!("skipped files: {}", view.skipped_file_count));
    lines.push(format!("before status: {}", view.before.status));
    lines.push(format!("after status: {}", view.after.status));

    if view.actions.is_empty() {
        lines.push("actions: none".into());
    } else {
        lines.push("actions:".into());
        for action in &view.actions {
            lines.push(format!(
                "- {} {} ({})",
                action.status, action.file_name, action.detail
            ));
        }
    }

    lines.join("\n")
}

pub fn format_runtime_backup_catalog(view: &RuntimeDirBackupCatalogView) -> String {
    let mut lines = vec![format!(
        "runtime-dir backup catalog status: {}",
        view.status
    )];
    lines.push(format!("runtime-dir: {}", view.runtime_dir));
    lines.push(format!("backups-dir: {}", view.backups_dir));
    lines.push(format!("backup count: {}", view.backup_count));

    if view.items.is_empty() {
        lines.push("backups: none".into());
    } else {
        lines.push("backups:".into());
        for item in &view.items {
            let mut details = vec![
                item.operation.clone(),
                item.snapshot_quality.clone(),
                format!("managed={}", item.managed_file_count),
                format!("missing={}", item.missing_file_count),
            ];
            if let Some(report_type) = item.report_type.as_deref() {
                details.push(format!("report={report_type}"));
            }
            if let Some(report_status) = item.report_status.as_deref() {
                details.push(format!("status={report_status}"));
            }
            lines.push(format!("- {} ({})", item.backup_name, details.join(", ")));
        }
    }

    lines.join("\n")
}

pub fn format_runtime_dir_restore_preview(view: &RuntimeDirRestorePreviewView) -> String {
    fn format_change_keys(keys: &[String]) -> String {
        if keys.is_empty() {
            "[]".into()
        } else {
            format!("[{}]", keys.join(", "))
        }
    }

    fn format_optional_change_keys(keys: Option<&Vec<String>>) -> String {
        match keys {
            Some(keys) => format_change_keys(keys.as_slice()),
            None => "[]".into(),
        }
    }

    fn format_optional_count(value: Option<usize>) -> String {
        value
            .map(|value| value.to_string())
            .unwrap_or_else(|| "0".into())
    }

    let mut lines = vec![format!(
        "runtime-dir restore preview status: {}",
        view.status
    )];
    lines.push(format!("runtime-dir: {}", view.runtime_dir));
    lines.push(format!("source-backup-dir: {}", view.source_backup_dir));
    lines.push(format!("preview-fingerprint: {}", view.preview_fingerprint));
    lines.push(format!(
        "source-snapshot-quality: {}",
        view.source_snapshot_quality
    ));
    lines.push(format!(
        "source managed files: {}",
        view.source_managed_file_count
    ));
    lines.push(format!(
        "source missing files: {}",
        view.source_missing_file_count
    ));
    if let Some(report_type) = view.source_report_type.as_deref() {
        lines.push(format!("source report type: {report_type}"));
    }
    if let Some(report_status) = view.source_report_status.as_deref() {
        lines.push(format!("source report status: {report_status}"));
    }
    lines.push(format!(
        "would restore files: {}",
        view.would_restore_file_count
    ));
    lines.push(format!("unchanged files: {}", view.unchanged_file_count));
    lines.push(format!("skipped files: {}", view.skipped_file_count));
    lines.push(format!("before status: {}", view.before.status));

    if view.actions.is_empty() {
        lines.push("actions: none".into());
    } else {
        lines.push("actions:".into());
        for action in &view.actions {
            lines.push(format!(
                "- {} {} ({})",
                action.action, action.file_name, action.detail
            ));
            if let Some(change_summary) = action.change_summary.as_ref() {
                lines.push(format!(
                    "  json-object-diff: +{} -{} ~{} unchanged={} source_keys={} target_keys={}",
                    format_change_keys(change_summary.added_keys.as_slice()),
                    format_change_keys(change_summary.removed_keys.as_slice()),
                    format_change_keys(change_summary.modified_keys.as_slice()),
                    change_summary.unchanged_key_count,
                    change_summary.source_key_count,
                    change_summary.target_key_count
                ));
            }
            if let Some(domain_summary) = action.domain_summary.as_ref() {
                if domain_summary.summary_kind == "disconnect_fences" {
                    lines.push(format!(
                        "  disconnect-fence-diff: +{} -{} owner_changed={} session_changed={} other_modified={} unchanged={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_change_keys(domain_summary.owner_node_changed_keys.as_slice()),
                        format_change_keys(domain_summary.session_changed_keys.as_slice()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count
                    ));
                } else if domain_summary.summary_kind == "realtime_checkpoints" {
                    lines.push(format!(
                        "  checkpoint-diff: +{} -{} latest_advanced={} latest_rewound={} acked_advanced={} acked_rewound={} trimmed_advanced={} trimmed_rewound={} timestamp_only={} other_modified={} unchanged={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_optional_change_keys(domain_summary.latest_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.latest_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.acked_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.acked_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.trimmed_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.trimmed_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.timestamp_only_changed_keys.as_ref()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count
                    ));
                } else if domain_summary.summary_kind == "realtime_subscriptions" {
                    lines.push(format!(
                        "  subscription-diff: +{} -{} scope_added={} scope_removed={} event_types_added={} event_types_removed={} subscribed_at_only={} synced_timestamp_only={} other_modified={} unchanged={} unchanged_scopes={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_optional_change_keys(domain_summary.added_scope_keys.as_ref()),
                        format_optional_change_keys(domain_summary.removed_scope_keys.as_ref()),
                        format_optional_change_keys(domain_summary.event_types_added_scope_keys.as_ref()),
                        format_optional_change_keys(domain_summary.event_types_removed_scope_keys.as_ref()),
                        format_optional_change_keys(
                            domain_summary
                                .subscribed_at_only_changed_scope_keys
                                .as_ref()
                        ),
                        format_optional_change_keys(domain_summary.timestamp_only_changed_keys.as_ref()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count,
                        format_optional_count(domain_summary.unchanged_scope_count)
                    ));
                } else if domain_summary.summary_kind == "stream_state" {
                    lines.push(format!(
                        "  stream-diff: +{} -{} state_changed={} last_frame_advanced={} last_frame_rewound={} checkpoint_advanced={} checkpoint_rewound={} result_message_changed={} frame_added={} frame_removed={} frame_modified={} updated_at_only={} other_modified={} unchanged={} unchanged_frames={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_optional_change_keys(domain_summary.stream_state_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_last_frame_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_last_frame_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_checkpoint_advanced_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_checkpoint_rewound_keys.as_ref()),
                        format_optional_change_keys(domain_summary.stream_result_message_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.added_frame_keys.as_ref()),
                        format_optional_change_keys(domain_summary.removed_frame_keys.as_ref()),
                        format_optional_change_keys(domain_summary.modified_frame_keys.as_ref()),
                        format_optional_change_keys(domain_summary.timestamp_only_changed_keys.as_ref()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count,
                        format_optional_count(domain_summary.unchanged_frame_count)
                    ));
                } else if domain_summary.summary_kind == "rtc_state" {
                    lines.push(format!(
                        "  rtc-diff: +{} -{} state_changed={} signaling_stream_changed={} artifact_message_changed={} signal_added={} signal_removed={} signal_modified={} updated_at_only={} other_modified={} unchanged={} unchanged_signals={}",
                        format_change_keys(domain_summary.added_keys.as_slice()),
                        format_change_keys(domain_summary.removed_keys.as_slice()),
                        format_optional_change_keys(domain_summary.rtc_state_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.rtc_signaling_stream_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.rtc_artifact_message_changed_keys.as_ref()),
                        format_optional_change_keys(domain_summary.added_signal_keys.as_ref()),
                        format_optional_change_keys(domain_summary.removed_signal_keys.as_ref()),
                        format_optional_change_keys(domain_summary.modified_signal_keys.as_ref()),
                        format_optional_change_keys(domain_summary.timestamp_only_changed_keys.as_ref()),
                        format_change_keys(domain_summary.other_modified_keys.as_slice()),
                        domain_summary.unchanged_key_count,
                        format_optional_count(domain_summary.unchanged_signal_count)
                    ));
                }
            }
        }
    }

    lines.join("\n")
}

pub fn format_runtime_dir_inspection(view: &RuntimeDirInspectionView) -> String {
    let mut lines = vec![format!("runtime-dir status: {}", view.status)];

    if let Some(runtime_dir) = view.runtime_dir.as_deref() {
        lines.push(format!("runtime-dir: {runtime_dir}"));
    }
    if let Some(state_dir) = view.state_dir.as_deref() {
        lines.push(format!("state-dir: {state_dir}"));
    }

    lines.push(format!("healthy files: {}", view.healthy_file_count));
    lines.push(format!("missing files: {}", view.missing_file_count));
    lines.push(format!("corrupt files: {}", view.corrupt_file_count));

    if view.files.is_empty() {
        lines.push("files: none".into());
    } else {
        lines.push("files:".into());
        for file in &view.files {
            let mut line = format!(
                "- {} {} ({})",
                file.status, file.file_name, file.recommended_action
            );
            if let Some(size_bytes) = file.size_bytes {
                line.push_str(format!(", {} bytes", size_bytes).as_str());
            }
            if let Some(parse_error) = file.parse_error.as_deref() {
                line.push_str(format!(", parse error: {parse_error}").as_str());
            }
            lines.push(line);
        }
    }

    lines.join("\n")
}

pub fn build_default_app() -> Router {
    match configured_runtime_dir() {
        Some(runtime_dir) => build_default_app_with_runtime_dir(runtime_dir),
        None => build_default_app_with_bind_addr(resolve_bind_addr().as_str()),
    }
}

pub fn build_public_app() -> Router {
    match configured_runtime_dir() {
        Some(runtime_dir) => build_public_app_with_runtime_dir(runtime_dir),
        None => build_public_app_with_bind_addr(resolve_bind_addr().as_str()),
    }
}

pub fn build_default_app_with_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> Router {
    build_default_app_with_bind_addr_and_runtime_dir(resolve_bind_addr().as_str(), runtime_dir)
}

pub fn build_public_app_with_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> Router {
    build_public_app_with_bind_addr_and_runtime_dir(resolve_bind_addr().as_str(), runtime_dir)
}

fn configured_runtime_dir() -> Option<PathBuf> {
    std::env::var("CRAW_CHAT_RUNTIME_DIR")
        .ok()
        .map(PathBuf::from)
}

fn build_default_app_with_bind_addr(bind_addr: &str) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    build_app_with_dependencies(
        "local_minimal_node_1",
        bind_addr,
        projection_service,
        realtime_cluster,
    )
}

fn build_public_app_with_bind_addr(bind_addr: &str) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    build_app_with_dependencies(
        "local_minimal_node_1",
        bind_addr,
        projection_service,
        realtime_cluster,
    )
    .layer(middleware::from_fn(require_public_bearer_auth))
}

fn build_default_app_with_bind_addr_and_runtime_dir(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime_dir = runtime_dir.as_ref().to_path_buf();
    let realtime_cluster = build_local_minimal_realtime_cluster(runtime_dir.as_path());
    let journal = ProjectionJournal::new_file(
        projection_service.clone(),
        runtime_dir
            .as_path()
            .join("state")
            .join("commit-journal.json"),
    );
    build_app_with_dependencies_and_runtime_and_journal(
        "local_minimal_node_1",
        bind_addr,
        Some(runtime_dir.clone()),
        projection_service,
        realtime_cluster,
        journal.clone(),
        build_local_minimal_presence_runtime(runtime_dir.as_path()),
        build_local_minimal_realtime_runtime(runtime_dir.as_path()),
        build_local_minimal_streaming_runtime(runtime_dir.as_path()),
        build_local_minimal_rtc_runtime(runtime_dir.as_path()),
        build_local_minimal_notification_runtime(journal.clone(), runtime_dir.as_path()),
        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
    )
}

fn build_public_app_with_bind_addr_and_runtime_dir(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime_dir = runtime_dir.as_ref().to_path_buf();
    let realtime_cluster = build_local_minimal_realtime_cluster(runtime_dir.as_path());
    let journal = ProjectionJournal::new_file(
        projection_service.clone(),
        runtime_dir
            .as_path()
            .join("state")
            .join("commit-journal.json"),
    );
    build_app_with_dependencies_and_runtime_and_journal(
        "local_minimal_node_1",
        bind_addr,
        Some(runtime_dir.clone()),
        projection_service,
        realtime_cluster,
        journal.clone(),
        build_local_minimal_presence_runtime(runtime_dir.as_path()),
        build_local_minimal_realtime_runtime(runtime_dir.as_path()),
        build_local_minimal_streaming_runtime(runtime_dir.as_path()),
        build_local_minimal_rtc_runtime(runtime_dir.as_path()),
        build_local_minimal_notification_runtime(journal.clone(), runtime_dir.as_path()),
        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
    )
    .layer(middleware::from_fn(require_public_bearer_auth))
}

fn build_local_minimal_realtime_cluster(
    runtime_dir: impl AsRef<StdPath>,
) -> Arc<RealtimeClusterBridge> {
    Arc::new(RealtimeClusterBridge::with_disconnect_fence_store(
        Arc::new(FileRealtimeDisconnectFenceStore::new(
            runtime_dir
                .as_ref()
                .join("state")
                .join("realtime-disconnect-fences.json"),
        )),
    ))
}

fn build_local_minimal_realtime_runtime(
    runtime_dir: impl AsRef<StdPath>,
) -> Arc<RealtimeDeliveryRuntime> {
    Arc::new(RealtimeDeliveryRuntime::with_stores(
        Arc::new(FileRealtimeCheckpointStore::new(
            runtime_dir
                .as_ref()
                .join("state")
                .join("realtime-checkpoints.json"),
        )),
        Arc::new(FileRealtimeSubscriptionStore::new(
            runtime_dir
                .as_ref()
                .join("state")
                .join("realtime-subscriptions.json"),
        )),
    ))
}

fn build_local_minimal_presence_runtime(
    runtime_dir: impl AsRef<StdPath>,
) -> Arc<SessionPresenceRuntime> {
    Arc::new(SessionPresenceRuntime::with_store(Arc::new(
        FilePresenceStateStore::new(
            runtime_dir
                .as_ref()
                .join("state")
                .join("presence-state.json"),
        ),
    )))
}

fn build_local_minimal_streaming_runtime(
    runtime_dir: impl AsRef<StdPath>,
) -> Arc<StreamingRuntime> {
    Arc::new(StreamingRuntime::with_store(Arc::new(
        FileStreamStateStore::new(runtime_dir.as_ref().join("state").join("stream-state.json")),
    )))
}

fn build_local_minimal_rtc_runtime(runtime_dir: impl AsRef<StdPath>) -> Arc<RtcRuntime> {
    Arc::new(RtcRuntime::with_store(Arc::new(FileRtcStateStore::new(
        runtime_dir.as_ref().join("state").join("rtc-state.json"),
    ))))
}

fn build_local_minimal_notification_runtime(
    journal: ProjectionJournal,
    runtime_dir: impl AsRef<StdPath>,
) -> Arc<NotificationRuntime> {
    Arc::new(NotificationRuntime::with_journal_and_store(
        Arc::new(journal),
        Arc::new(FileNotificationTaskStore::new(
            runtime_dir
                .as_ref()
                .join("state")
                .join("notification-tasks.json"),
        )),
    ))
}

fn build_local_minimal_automation_runtime(
    journal: ProjectionJournal,
    runtime_dir: impl AsRef<StdPath>,
) -> Arc<AutomationRuntime> {
    Arc::new(AutomationRuntime::with_journal_and_store(
        Arc::new(journal),
        Arc::new(FileAutomationExecutionStore::new(
            runtime_dir
                .as_ref()
                .join("state")
                .join("automation-executions.json"),
        )),
    ))
}

pub fn build_app_with_dependencies(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
) -> Router {
    let journal = ProjectionJournal::new_memory(projection_service.clone());
    build_app_with_dependencies_and_runtime_and_journal(
        node_id,
        bind_addr,
        None,
        projection_service,
        realtime_cluster,
        journal.clone(),
        Arc::new(SessionPresenceRuntime::default()),
        Arc::new(RealtimeDeliveryRuntime::with_checkpoint_store(Arc::new(
            MemoryRealtimeCheckpointStore::default(),
        ))),
        Arc::new(StreamingRuntime::default()),
        Arc::new(RtcRuntime::default()),
        Arc::new(NotificationRuntime::with_journal(Arc::new(journal.clone()))),
        Arc::new(AutomationRuntime::with_journal(Arc::new(journal))),
    )
}

pub fn build_app_with_dependencies_and_runtime(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
) -> Router {
    build_app_with_dependencies_and_runtime_and_journal(
        node_id,
        bind_addr,
        None,
        projection_service.clone(),
        realtime_cluster,
        ProjectionJournal::new_memory(projection_service),
        Arc::new(SessionPresenceRuntime::default()),
        realtime_runtime,
        Arc::new(StreamingRuntime::default()),
        Arc::new(RtcRuntime::default()),
        Arc::new(NotificationRuntime::default()),
        Arc::new(AutomationRuntime::default()),
    )
}

fn build_app_with_dependencies_and_runtime_and_journal(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    runtime_dir: Option<PathBuf>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    journal: ProjectionJournal,
    session_presence_runtime: Arc<SessionPresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    streaming_runtime: Arc<StreamingRuntime>,
    rtc_runtime: Arc<RtcRuntime>,
    notification_runtime: Arc<NotificationRuntime>,
    automation_runtime: Arc<AutomationRuntime>,
) -> Router {
    let node_id = node_id.into();
    let bind_addr = bind_addr.into();
    realtime_cluster.bind_node_runtime(node_id.as_str(), realtime_runtime.clone());
    let conversation_runtime = Arc::new(ConversationRuntime::new(journal.clone()));
    replay_projection_journal(
        &journal,
        projection_service.as_ref(),
        conversation_runtime.as_ref(),
    );
    let ops_runtime = Arc::new(OpsRuntime::new(
        node_id.clone(),
        "local-minimal",
        bind_addr.clone(),
        vec![
            "conversation-runtime".into(),
            "projection-service".into(),
            "media-service".into(),
            "streaming-service".into(),
            "rtc-signaling-service".into(),
            "notification-service".into(),
            "automation-service".into(),
            "audit-service".into(),
            "ops-service".into(),
        ],
        vec![
            "conversation:*".into(),
            "stream:*".into(),
            "rtc:*".into(),
            "notification:*".into(),
            "automation:*".into(),
        ],
    ));
    let state = AppState {
        node_id: node_id.clone(),
        runtime_dir,
        realtime_cluster,
        conversation_runtime,
        projection_service,
        session_presence_runtime,
        realtime_runtime,
        media_runtime: Arc::new(MediaRuntime::with_journal(Arc::new(journal.clone()))),
        streaming_runtime,
        rtc_runtime,
        notification_runtime,
        automation_runtime,
        audit_runtime: Arc::new(AuditRuntime::default()),
        ops_runtime,
    };
    refresh_node_operational_view(&state);
    build_app(state)
}

fn replay_projection_journal(
    journal: &ProjectionJournal,
    projection_service: &TimelineProjectionService,
    conversation_runtime: &ConversationRuntime<ProjectionJournal>,
) {
    let recorded = journal.recorded().unwrap_or_else(|error| {
        panic!("failed to load local-minimal commit journal during startup replay: {error:?}")
    });
    apply_projection_journal_envelopes(
        recorded.as_slice(),
        projection_service,
        conversation_runtime,
        "local-minimal startup",
    )
    .unwrap_or_else(|error| panic!("{error}"));
}

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/sessions/resume", post(resume_session))
        .route("/api/v1/sessions/disconnect", post(disconnect_session))
        .route("/api/v1/presence/heartbeat", post(heartbeat_presence))
        .route("/api/v1/presence/me", get(get_presence_me))
        .route(
            "/api/v1/realtime/subscriptions/sync",
            post(sync_realtime_subscriptions),
        )
        .route("/api/v1/realtime/ws", get(realtime_websocket))
        .route("/api/v1/realtime/events/ack", post(ack_realtime_events))
        .route("/api/v1/realtime/events", get(list_realtime_events))
        .route("/api/v1/devices/register", post(register_device))
        .route(
            "/api/v1/devices/{device_id}/sync-feed",
            get(get_device_sync_feed),
        )
        .route("/api/v1/inbox", get(get_inbox))
        .route("/api/v1/conversations", post(create_conversation))
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
            "/api/v1/conversations/{conversation_id}",
            get(get_conversation_summary),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members",
            get(list_members),
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
        .route(
            "/api/v1/conversations/{conversation_id}/messages",
            post(post_message),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/system-channel/publish",
            post(publish_system_channel_message),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/messages",
            get(get_timeline),
        )
        .route("/api/v1/messages/{message_id}/edit", post(edit_message))
        .route("/api/v1/messages/{message_id}/recall", post(recall_message))
        .route("/api/v1/media/uploads", post(create_media_upload))
        .route(
            "/api/v1/media/uploads/{media_asset_id}/complete",
            post(complete_media_upload),
        )
        .route("/api/v1/media/{media_asset_id}", get(get_media))
        .route("/api/v1/media/{media_asset_id}/attach", post(attach_media))
        .route("/api/v1/streams", post(open_stream))
        .route(
            "/api/v1/streams/{stream_id}/frames",
            post(append_stream_frame).get(list_stream_frames),
        )
        .route(
            "/api/v1/streams/{stream_id}/checkpoint",
            post(checkpoint_stream),
        )
        .route(
            "/api/v1/streams/{stream_id}/complete",
            post(complete_stream),
        )
        .route("/api/v1/streams/{stream_id}/abort", post(abort_stream))
        .route("/api/v1/rtc/sessions", post(create_rtc_session))
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/invite",
            post(invite_rtc_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/accept",
            post(accept_rtc_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/reject",
            post(reject_rtc_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/end",
            post(end_rtc_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/signals",
            post(post_rtc_signal),
        )
        .route("/api/v1/notifications/requests", post(request_notification))
        .route("/api/v1/notifications", get(list_notifications))
        .route(
            "/api/v1/notifications/{notification_id}",
            get(get_notification),
        )
        .route(
            "/api/v1/automation/executions",
            post(request_automation_execution),
        )
        .route(
            "/api/v1/automation/executions/{execution_id}",
            get(get_automation_execution),
        )
        .route("/api/v1/audit/records", post(record_audit_anchor))
        .route("/api/v1/audit/records", get(list_audit_records))
        .route("/api/v1/audit/export", get(export_audit_bundle))
        .route("/api/v1/ops/health", get(get_ops_health))
        .route("/api/v1/ops/cluster", get(get_ops_cluster))
        .route("/api/v1/ops/lag", get(get_ops_lag))
        .route("/api/v1/ops/runtime-dir", get(get_ops_runtime_dir))
        .route("/api/v1/ops/diagnostics", get(get_ops_diagnostics))
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
        service: "local-minimal-node",
        profile: "local-minimal",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "local-minimal-node",
        profile: "local-minimal",
    })
}

async fn create_conversation(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .conversation_runtime
            .create_conversation_with_creator_kind(
                CreateConversationCommand {
                    tenant_id: auth.tenant_id,
                    conversation_id: request.conversation_id,
                    creator_id: auth.actor_id,
                    conversation_type: request.conversation_type,
                },
                auth.actor_kind.as_str(),
            )?,
    ))
}

async fn create_agent_dialog(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentDialogRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .conversation_runtime
            .create_agent_dialog_with_requester_kind(
                CreateAgentDialogCommand {
                    tenant_id: auth.tenant_id,
                    conversation_id: request.conversation_id,
                    requester_id: auth.actor_id,
                    agent_id: request.agent_id,
                },
                auth.actor_kind.as_str(),
            )?,
    ))
}

async fn create_agent_handoff(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentHandoffRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .conversation_runtime
            .create_agent_handoff_with_source_kind(
                CreateAgentHandoffCommand {
                    tenant_id: auth.tenant_id,
                    conversation_id: request.conversation_id,
                    source_id: auth.actor_id,
                    target_id: request.target_id,
                    target_kind: request.target_kind,
                    handoff_session_id: request.handoff_session_id,
                    handoff_reason: request.handoff_reason,
                },
                auth.actor_kind.as_str(),
            )?,
    ))
}

async fn create_system_channel(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateSystemChannelRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .conversation_runtime
            .create_system_channel_with_requester_kind(
                CreateSystemChannelCommand {
                    tenant_id: auth.tenant_id,
                    conversation_id: request.conversation_id,
                    requester_id: auth.actor_id,
                    subscriber_id: request.subscriber_id,
                },
                auth.actor_kind.as_str(),
            )?,
    ))
}

async fn get_agent_handoff_state(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.conversation_runtime.get_agent_handoff_state(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?))
}

async fn accept_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let previous_state = state.conversation_runtime.get_agent_handoff_state(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?;
    let result = state
        .conversation_runtime
        .accept_agent_handoff_with_actor_kind(
            AcceptAgentHandoffCommand {
                tenant_id: auth.tenant_id.clone(),
                conversation_id,
                accepted_by: auth.actor_id.clone(),
            },
            auth.actor_kind.as_str(),
        )?;
    if result != previous_state {
        publish_realtime_agent_handoff_status_changed_event(
            &state,
            &auth,
            &previous_state,
            &result,
        )?;
    }
    Ok(Json(result))
}

async fn resolve_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let previous_state = state.conversation_runtime.get_agent_handoff_state(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?;
    let result = state
        .conversation_runtime
        .resolve_agent_handoff_with_actor_kind(
            ResolveAgentHandoffCommand {
                tenant_id: auth.tenant_id.clone(),
                conversation_id,
                resolved_by: auth.actor_id.clone(),
            },
            auth.actor_kind.as_str(),
        )?;
    if result != previous_state {
        publish_realtime_agent_handoff_status_changed_event(
            &state,
            &auth,
            &previous_state,
            &result,
        )?;
    }
    Ok(Json(result))
}

async fn close_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let previous_state = state.conversation_runtime.get_agent_handoff_state(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_id.as_str(),
    )?;
    let result = state
        .conversation_runtime
        .close_agent_handoff_with_actor_kind(
            CloseAgentHandoffCommand {
                tenant_id: auth.tenant_id.clone(),
                conversation_id,
                closed_by: auth.actor_id.clone(),
            },
            auth.actor_kind.as_str(),
        )?;
    if result != previous_state {
        publish_realtime_agent_handoff_status_changed_event(
            &state,
            &auth,
            &previous_state,
            &result,
        )?;
    }
    Ok(Json(result))
}

async fn get_inbox(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<InboxResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(InboxResponse {
        items: state
            .projection_service
            .inbox(auth.tenant_id.as_str(), auth.actor_id.as_str()),
    }))
}

async fn resume_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ResumeSessionRequest>,
) -> Result<Json<SessionResumeView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    bind_registered_device(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        true,
    )?;
    let latest_sync_seq = state.projection_service.latest_device_sync_seq(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
    );
    let registered_devices = state
        .projection_service
        .registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str())
        .into_iter()
        .map(|item| item.device_id)
        .collect::<Vec<_>>();
    Ok(Json(state.session_presence_runtime.resume(
        &auth,
        device_id,
        request.last_seen_sync_seq.unwrap_or_default(),
        latest_sync_seq,
        registered_devices,
    )?))
}

async fn get_presence_me(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let registered_devices = state
        .projection_service
        .registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str())
        .into_iter()
        .map(|item| item.device_id)
        .collect::<Vec<_>>();
    Ok(Json(state.session_presence_runtime.presence_snapshot(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.device_id.clone(),
        registered_devices,
    )?))
}

async fn heartbeat_presence(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PresenceDeviceRequest>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    bind_registered_device(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        false,
    )?;
    let registered_devices = state
        .projection_service
        .registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str())
        .into_iter()
        .map(|item| item.device_id)
        .collect::<Vec<_>>();
    Ok(Json(state.session_presence_runtime.heartbeat(
        &auth,
        device_id.clone(),
        state.projection_service.latest_device_sync_seq(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        ),
        registered_devices,
    )?))
}

async fn disconnect_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PresenceDeviceRequest>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    if state.realtime_cluster.disconnect_fence_matches_session(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
    )? {
        state.realtime_runtime.signal_device_disconnect(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
        )?;
        let registered_devices = state
            .projection_service
            .registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str())
            .into_iter()
            .map(|item| item.device_id)
            .collect::<Vec<_>>();
        return Ok(Json(state.session_presence_runtime.presence_snapshot(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            Some(device_id),
            registered_devices,
        )?));
    }
    bind_registered_device(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        false,
    )?;
    state.realtime_runtime.clear_device_subscriptions(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
    )?;
    let _ = state.realtime_cluster.release_device_route(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        state.node_id.as_str(),
    );
    state.realtime_cluster.mark_device_disconnected(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        state.node_id.as_str(),
    )?;
    state.realtime_runtime.signal_device_disconnect(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
    )?;
    refresh_node_operational_view(&state);
    let registered_devices = state
        .projection_service
        .registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str())
        .into_iter()
        .map(|item| item.device_id)
        .collect::<Vec<_>>();
    Ok(Json(state.session_presence_runtime.disconnect(
        &auth,
        device_id,
        registered_devices,
    )?))
}

async fn register_device(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RegisterDeviceRequest>,
) -> Result<Json<projection_service::RegisteredDeviceView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    Ok(Json(bind_registered_device(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        false,
    )?))
}

async fn sync_realtime_subscriptions(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SyncRealtimeSubscriptionsRequest>,
) -> Result<Json<im_domain_core::realtime::RealtimeSubscriptionSnapshot>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    bind_registered_device(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        false,
    )?;
    Ok(Json(state.realtime_runtime.sync_subscriptions(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        request.items,
    )?))
}

async fn list_realtime_events(
    Query(query): Query<ListRealtimeEventsQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::realtime::RealtimeEventWindow>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    bind_registered_device(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http_poll",
        false,
    )?;
    let limit = query.limit.unwrap_or(100);
    if limit == 0 {
        return Err(ApiError::bad_request(
            "limit_invalid",
            "limit must be greater than 0",
        ));
    }
    Ok(Json(state.realtime_runtime.list_events(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        query.after_seq.unwrap_or_default(),
        limit,
    )?))
}

async fn ack_realtime_events(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AckRealtimeEventsRequest>,
) -> Result<Json<im_domain_core::realtime::RealtimeAckState>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    bind_registered_device(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "http",
        false,
    )?;
    Ok(Json(state.realtime_runtime.ack_events(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        request.acked_seq,
    )?))
}

async fn realtime_websocket(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<axum::response::Response, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    bind_registered_device(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "websocket",
        false,
    )?;
    let runtime = state.realtime_runtime.clone();
    Ok(ws
        .on_upgrade(move |socket| serve_realtime_websocket(socket, auth, device_id, runtime))
        .into_response())
}

async fn get_device_sync_feed(
    Path(device_id): Path<String>,
    Query(query): Query<SyncFeedQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<DeviceSyncFeedResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    validate_device_scope(&auth, device_id.as_str())?;
    Ok(Json(DeviceSyncFeedResponse {
        items: state.projection_service.device_sync_feed(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id.as_str(),
            query.after_seq,
        ),
    }))
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
    let result = post_message_with_side_effects(
        &state,
        &auth,
        conversation_id,
        request.client_msg_id,
        MessageType::Standard,
        body,
    )?;

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
    let result = publish_system_channel_message_with_side_effects(
        &state,
        &auth,
        conversation_id,
        request.client_msg_id,
        body,
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
    ensure_registered_device(&state, &auth)?;
    let summary = request.summary.clone();
    let body = build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;
    let result = state
        .conversation_runtime
        .edit_message(EditMessageCommand {
            tenant_id: auth.tenant_id.clone(),
            message_id: message_id.clone(),
            editor: build_sender(&auth),
            body,
        })?;

    state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: format!("audit_message_edited_{}", result.message_id),
            aggregate_type: "conversation".into(),
            aggregate_id: result.conversation_id.clone(),
            action: "message.edited".into(),
            payload: Some(
                serde_json::json!({
                    "messageId": result.message_id,
                    "messageSeq": result.message_seq,
                })
                .to_string(),
            ),
        },
    );

    publish_realtime_conversation_message_event(
        &state,
        auth.tenant_id.as_str(),
        result.conversation_id.as_str(),
        "message.edited",
        serde_json::json!({
            "conversationId": result.conversation_id,
            "messageId": result.message_id,
            "messageSeq": result.message_seq,
            "summary": summary,
        })
        .to_string(),
    )?;

    Ok(Json(result))
}

async fn recall_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MessageMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_registered_device(&state, &auth)?;
    let result = state
        .conversation_runtime
        .recall_message(RecallMessageCommand {
            tenant_id: auth.tenant_id.clone(),
            message_id,
            recalled_by: build_sender(&auth),
        })?;

    state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: format!("audit_message_recalled_{}", result.message_id),
            aggregate_type: "conversation".into(),
            aggregate_id: result.conversation_id.clone(),
            action: "message.recalled".into(),
            payload: Some(
                serde_json::json!({
                    "messageId": result.message_id,
                    "messageSeq": result.message_seq,
                })
                .to_string(),
            ),
        },
    );

    publish_realtime_conversation_message_event(
        &state,
        auth.tenant_id.as_str(),
        result.conversation_id.as_str(),
        "message.recalled",
        serde_json::json!({
            "conversationId": result.conversation_id,
            "messageId": result.message_id,
            "messageSeq": result.message_seq,
            "summary": "[recalled]",
        })
        .to_string(),
    )?;

    Ok(Json(result))
}

async fn list_members(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ListMembersResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    Ok(Json(ListMembersResponse {
        items: state
            .conversation_runtime
            .list_members(auth.tenant_id.as_str(), conversation_id.as_str())?,
    }))
}

async fn add_member(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AddConversationMemberRequest>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let actor_auth =
        resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let member = state.conversation_runtime.add_member_with_actor_kind(
        AddConversationMemberCommand {
            tenant_id: auth.tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            principal_id: request.principal_id,
            principal_kind: request.principal_kind,
            role: request.role,
            invited_by: auth.actor_id.clone(),
        },
        auth.actor_kind.as_str(),
    )?;

    record_membership_audit(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_joined",
        &member,
    );

    publish_realtime_membership_event(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_joined",
        serde_json::json!({
            "conversationId": conversation_id.as_str(),
            "member": &member,
            "actor": {
                "id": actor_auth.actor_id.as_str(),
                "kind": actor_auth.actor_kind.as_str(),
            }
        })
        .to_string(),
        BTreeSet::from([member.principal_id.clone()]),
    )?;

    Ok(Json(member))
}

async fn remove_member(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RemoveConversationMemberRequest>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let actor_auth =
        resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let member = state.conversation_runtime.remove_member_with_actor_kind(
        RemoveConversationMemberCommand {
            tenant_id: auth.tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            member_id: request.member_id,
            removed_by: auth.actor_id.clone(),
        },
        auth.actor_kind.as_str(),
    )?;

    record_membership_audit(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_removed",
        &member,
    );

    publish_realtime_membership_event(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_removed",
        serde_json::json!({
            "conversationId": conversation_id.as_str(),
            "member": &member,
            "actor": {
                "id": actor_auth.actor_id.as_str(),
                "kind": actor_auth.actor_kind.as_str(),
            }
        })
        .to_string(),
        BTreeSet::from([member.principal_id.clone()]),
    )?;

    Ok(Json(member))
}

async fn transfer_conversation_owner(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<TransferConversationOwnerRequest>,
) -> Result<Json<TransferConversationOwnerResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let actor_auth =
        resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let transfer = state
        .conversation_runtime
        .transfer_conversation_owner_with_actor_kind(
            TransferConversationOwnerCommand {
                tenant_id: auth.tenant_id.clone(),
                conversation_id: conversation_id.clone(),
                target_member_id: request.member_id,
                transferred_by: auth.actor_id.clone(),
            },
            auth.actor_kind.as_str(),
        )?;

    record_owner_transfer_audit(&state, &actor_auth, conversation_id.as_str(), &transfer);

    Ok(Json(transfer))
}

async fn change_conversation_member_role(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ChangeConversationMemberRoleRequest>,
) -> Result<Json<ChangeConversationMemberRoleResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let actor_auth =
        resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let change = state
        .conversation_runtime
        .change_conversation_member_role_with_actor_kind(
            ChangeConversationMemberRoleCommand {
                tenant_id: auth.tenant_id.clone(),
                conversation_id: conversation_id.clone(),
                target_member_id: request.member_id,
                new_role: request.role,
                changed_by: auth.actor_id.clone(),
            },
            auth.actor_kind.as_str(),
        )?;

    record_member_role_change_audit(&state, &actor_auth, conversation_id.as_str(), &change);

    publish_realtime_membership_event(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_role_changed",
        serde_json::json!({
            "conversationId": conversation_id.as_str(),
            "changedAt": change.changed_at.as_str(),
            "previousMember": &change.previous_member,
            "updatedMember": &change.updated_member,
            "actor": {
                "id": actor_auth.actor_id.as_str(),
                "kind": actor_auth.actor_kind.as_str(),
            }
        })
        .to_string(),
        BTreeSet::from([change.updated_member.principal_id.clone()]),
    )?;

    Ok(Json(change))
}

async fn leave_conversation(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let actor_auth =
        resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let member = state
        .conversation_runtime
        .leave_conversation_with_actor_kind(
            LeaveConversationCommand {
                tenant_id: auth.tenant_id.clone(),
                conversation_id: conversation_id.clone(),
                principal_id: auth.actor_id.clone(),
            },
            auth.actor_kind.as_str(),
        )?;

    record_membership_audit(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_left",
        &member,
    );

    publish_realtime_membership_event(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_left",
        serde_json::json!({
            "conversationId": conversation_id.as_str(),
            "member": &member,
            "actor": {
                "id": actor_auth.actor_id.as_str(),
                "kind": actor_auth.actor_kind.as_str(),
            }
        })
        .to_string(),
        BTreeSet::from([member.principal_id.clone()]),
    )?;

    Ok(Json(member))
}

async fn get_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let cursor = state
        .projection_service
        .read_cursor(
            auth.tenant_id.as_str(),
            conversation_id.as_str(),
            auth.actor_id.as_str(),
        )
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_read_cursor_not_found",
            message: format!("conversation read cursor not found: {conversation_id}"),
        })?;
    Ok(Json(cursor))
}

async fn update_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateReadCursorRequest>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_registered_device(&state, &auth)?;
    let cursor = state
        .conversation_runtime
        .update_read_cursor_with_actor_kind(
            UpdateReadCursorCommand {
                tenant_id: auth.tenant_id.clone(),
                conversation_id: conversation_id.clone(),
                principal_id: auth.actor_id.clone(),
                read_seq: request.read_seq,
                last_read_message_id: request.last_read_message_id,
            },
            auth.actor_kind.as_str(),
        )?;

    state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: format!("audit_read_cursor_{}", cursor.member_id),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id.clone(),
            action: "conversation.read_cursor_updated".into(),
            payload: Some(
                serde_json::json!({
                    "memberId": cursor.member_id,
                    "principalId": cursor.principal_id,
                    "readSeq": cursor.read_seq,
                    "lastReadMessageId": cursor.last_read_message_id,
                })
                .to_string(),
            ),
        },
    );

    let view = state
        .projection_service
        .read_cursor(
            auth.tenant_id.as_str(),
            conversation_id.as_str(),
            auth.actor_id.as_str(),
        )
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_read_cursor_not_found",
            message: format!("conversation read cursor not found: {conversation_id}"),
        })?;
    Ok(Json(view))
}

async fn get_timeline(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    Ok(Json(serde_json::json!({
        "items": state.projection_service.timeline(auth.tenant_id.as_str(), conversation_id.as_str())
    })))
}

async fn get_conversation_summary(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<projection_service::ConversationSummaryView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let summary = state
        .projection_service
        .conversation_summary(auth.tenant_id.as_str(), conversation_id.as_str())
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_summary_not_found",
            message: format!("conversation summary not found: {conversation_id}"),
        })?;
    Ok(Json(summary))
}

async fn create_media_upload(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateUploadRequest>,
) -> Result<Json<im_domain_core::media::MediaAsset>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.media_runtime.create_upload(&auth, request)?))
}

async fn complete_media_upload(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteUploadRequest>,
) -> Result<Json<im_domain_core::media::MediaAsset>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.media_runtime.complete_upload(
        &auth,
        media_asset_id.as_str(),
        request,
    )?))
}

async fn get_media(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::media::MediaAsset>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .media_runtime
            .get_asset(&auth, media_asset_id.as_str())?,
    ))
}

async fn attach_media(
    Path(media_asset_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AttachMediaRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let asset = state
        .media_runtime
        .get_asset(&auth, media_asset_id.as_str())?;
    if asset.processing_state != MediaProcessingState::Ready {
        return Err(ApiError::bad_request(
            "media_asset_not_ready",
            format!("media asset is not ready to attach: {media_asset_id}"),
        ));
    }

    let body = build_message_body(
        request.summary,
        request.text,
        vec![ContentPart::media(MediaPart {
            media_asset_id: asset.media_asset_id.clone(),
            resource: Some(asset.resource.clone()),
        })],
        request.render_hints,
    )?;

    let result = post_message_with_side_effects(
        &state,
        &auth,
        request.conversation_id,
        request.client_msg_id,
        MessageType::Standard,
        body,
    )?;

    Ok(Json(result))
}

async fn open_stream(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<OpenStreamRequest>,
) -> Result<Json<im_domain_core::stream::StreamSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_stream_open_access(&state, &auth, &request)?;
    Ok(Json(state.streaming_runtime.open_stream(&auth, request)?))
}

async fn checkpoint_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CheckpointStreamRequest>,
) -> Result<Json<im_domain_core::stream::StreamSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_stream_session_write_access(&state, &auth, stream_id.as_str(), "stream.checkpoint")?;
    Ok(Json(state.streaming_runtime.checkpoint_stream(
        &auth,
        stream_id.as_str(),
        request,
    )?))
}

async fn append_stream_frame(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AppendStreamFrameRequest>,
) -> Result<Json<im_domain_core::stream::StreamFrame>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_stream_session_write_access(&state, &auth, stream_id.as_str(), "stream.append")?;
    let frame = state
        .streaming_runtime
        .append_frame(&auth, stream_id.as_str(), request)?;
    publish_realtime_stream_frame_event(&state, &auth, &frame)?;
    Ok(Json(frame))
}

async fn list_stream_frames(
    Path(stream_id): Path<String>,
    Query(query): Query<ListStreamFramesQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<StreamFrameWindow>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_stream_session_conversation_member(&state, &auth, stream_id.as_str())?;
    Ok(Json(state.streaming_runtime.list_frames(
        &auth,
        stream_id.as_str(),
        query,
    )?))
}

async fn complete_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteStreamRequest>,
) -> Result<Json<im_domain_core::stream::StreamSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_stream_session_write_access(&state, &auth, stream_id.as_str(), "stream.complete")?;
    let session = state
        .streaming_runtime
        .complete_stream(&auth, stream_id.as_str(), request)?;
    publish_realtime_stream_lifecycle_event(&state, &auth, &session, "stream.completed", None)?;
    Ok(Json(session))
}

async fn abort_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AbortStreamRequest>,
) -> Result<Json<im_domain_core::stream::StreamSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_stream_session_write_access(&state, &auth, stream_id.as_str(), "stream.abort")?;
    let abort_reason = request.reason.clone();
    let session = state
        .streaming_runtime
        .abort_stream(&auth, stream_id.as_str(), request)?;
    publish_realtime_stream_lifecycle_event(
        &state,
        &auth,
        &session,
        "stream.aborted",
        abort_reason,
    )?;
    Ok(Json(session))
}

async fn create_rtc_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateRtcSessionRequest>,
) -> Result<Json<im_domain_core::rtc::RtcSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_rtc_create_access(&state, &auth, &request)?;
    Ok(Json(state.rtc_runtime.create_session(&auth, request)?))
}

async fn invite_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<InviteRtcSessionRequest>,
) -> Result<Json<im_domain_core::rtc::RtcSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.invite",
    )?;
    let outcome =
        state
            .rtc_runtime
            .invite_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    if outcome.applied {
        emit_rtc_signal_message(&state, &auth, &outcome.session, "rtc.invite")?;
    }
    Ok(Json(outcome.session))
}

async fn accept_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<im_domain_core::rtc::RtcSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.accept",
    )?;
    let outcome =
        state
            .rtc_runtime
            .accept_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    if outcome.applied {
        emit_rtc_signal_message(&state, &auth, &outcome.session, "rtc.accept")?;
    }
    Ok(Json(outcome.session))
}

async fn reject_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<im_domain_core::rtc::RtcSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.reject",
    )?;
    let outcome =
        state
            .rtc_runtime
            .reject_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    if outcome.applied {
        emit_rtc_signal_message(&state, &auth, &outcome.session, "rtc.reject")?;
    }
    Ok(Json(outcome.session))
}

async fn end_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<im_domain_core::rtc::RtcSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.end",
    )?;
    let outcome =
        state
            .rtc_runtime
            .end_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    if outcome.applied {
        emit_rtc_signal_message(&state, &auth, &outcome.session, "rtc.end")?;
    }
    Ok(Json(outcome.session))
}

async fn post_rtc_signal(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostRtcSignalRequest>,
) -> Result<Json<im_domain_core::rtc::RtcSignalEvent>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.signal",
    )?;
    let signal = state
        .rtc_runtime
        .post_signal(&auth, rtc_session_id.as_str(), request)?;
    emit_rtc_custom_signal_message(&state, &auth, &signal)?;
    Ok(Json(signal))
}

async fn request_notification(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestNotification>,
) -> Result<Json<NotificationTask>, axum::response::Response> {
    let is_bearer_request = headers.contains_key(axum::http::header::AUTHORIZATION);
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    ensure_notification_request_access(&auth, request.recipient_id.as_str(), is_bearer_request)
        .map_err(IntoResponse::into_response)?;
    let result = state
        .notification_runtime
        .request_notification_with_outcome(&auth, request)
        .map_err(IntoResponse::into_response)?;
    let is_new = result.is_new;
    let task = result.task;

    if is_new {
        state.audit_runtime.record_anchor(
            &auth,
            RecordAuditAnchor {
                record_id: format!("audit_{}", task.notification_id),
                aggregate_type: "notification".into(),
                aggregate_id: task.notification_id.clone(),
                action: "notification.requested".into(),
                payload: Some(
                    serde_json::json!({
                        "sourceEventType": task.source_event_type,
                        "recipientId": task.recipient_id,
                    })
                    .to_string(),
                ),
            },
        );
    }

    Ok(Json(task))
}

async fn list_notifications(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let items = state
        .notification_runtime
        .list_notifications(&auth)
        .map_err(IntoResponse::into_response)?;
    Ok(Json(serde_json::json!({
        "items": items
    })))
}

async fn get_notification(
    Path(notification_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<NotificationTask>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let task = state
        .notification_runtime
        .get_notification(&auth, notification_id.as_str())
        .map_err(IntoResponse::into_response)?;
    Ok(Json(task))
}

async fn request_automation_execution(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestAutomationExecution>,
) -> Result<Json<AutomationExecution>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let result = state
        .automation_runtime
        .request_execution_with_outcome(&auth, request)
        .map_err(IntoResponse::into_response)?;
    let is_new = result.is_new;
    let execution = result.execution;

    if is_new {
        state.audit_runtime.record_anchor(
            &auth,
            RecordAuditAnchor {
                record_id: format!("audit_{}", execution.execution_id),
                aggregate_type: "automation_execution".into(),
                aggregate_id: execution.execution_id.clone(),
                action: "automation.execution_requested".into(),
                payload: execution.input_payload.clone(),
            },
        );

        let _ = state.notification_runtime.request_notification(
            &auth,
            RequestNotification {
                notification_id: format!("ntf_automation_{}", execution.execution_id),
                source_event_id: format!(
                    "evt_{}_automation_execution_completed",
                    execution.execution_id
                ),
                source_event_type: "automation.execution_completed".into(),
                category: "automation.result".into(),
                channel: "inapp".into(),
                recipient_id: auth.actor_id.clone(),
                title: Some("Automation completed".into()),
                body: Some(execution.target_ref.clone()),
                payload: execution.output_payload.clone(),
            },
        );
    }

    Ok(Json(execution))
}

async fn get_automation_execution(
    Path(execution_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AutomationExecution>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let execution = state
        .automation_runtime
        .get_execution(&auth, execution_id.as_str())
        .map_err(IntoResponse::into_response)?;
    Ok(Json(execution))
}

async fn record_audit_anchor(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RecordAuditAnchor>,
) -> Result<Json<AuditRecord>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    ensure_audit_write_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(state.audit_runtime.record_anchor(&auth, request)))
}

async fn list_audit_records(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    ensure_audit_read_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(serde_json::json!({
        "items": state.audit_runtime.list_records(&auth)
    })))
}

async fn export_audit_bundle(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AuditExportBundle>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    ensure_audit_read_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(state.audit_runtime.export_bundle(&auth)))
}

async fn get_ops_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<OpsHealthResponse>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.health_view()))
}

async fn get_ops_cluster(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ClusterView>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.cluster_view()))
}

async fn get_ops_lag(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<LagView>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.lag_view()))
}

async fn get_ops_runtime_dir(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RuntimeDirInspectionView>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.runtime_dir_view()))
}

async fn get_ops_diagnostics(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<DiagnosticBundle>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.diagnostic_bundle()))
}

fn ensure_audit_read_access(auth: &AuthContext) -> Result<(), ApiError> {
    if auth.has_permission("audit.read") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission: audit.read",
    ))
}

fn ensure_audit_write_access(auth: &AuthContext) -> Result<(), ApiError> {
    if auth.has_permission("audit.write") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission: audit.write",
    ))
}

fn ensure_ops_read_access(auth: &AuthContext) -> Result<(), ApiError> {
    if auth.has_permission("ops.read") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission: ops.read",
    ))
}

fn ensure_notification_request_access(
    auth: &AuthContext,
    recipient_id: &str,
    is_bearer_request: bool,
) -> Result<(), ApiError> {
    if !is_bearer_request
        || recipient_id == auth.actor_id
        || auth.has_permission("notification.write")
    {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission to request notifications for other recipients: notification.write",
    ))
}

fn build_sender(auth: &im_auth_context::AuthContext) -> Sender {
    Sender {
        id: auth.actor_id.clone(),
        kind: auth.actor_kind.clone(),
        member_id: None,
        device_id: auth.device_id.clone(),
        session_id: auth.session_id.clone(),
        metadata: BTreeMap::new(),
    }
}

fn post_message_with_side_effects(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: String,
    client_msg_id: Option<String>,
    message_type: MessageType,
    body: MessageBody,
) -> Result<PostMessageResult, ApiError> {
    ensure_registered_device(state, auth)?;
    let summary = body.summary.clone();
    let message_type_name = match &message_type {
        MessageType::Standard => "standard",
        MessageType::Signal => "signal",
        MessageType::System => "system",
    };

    let result = state
        .conversation_runtime
        .post_message(PostMessageCommand {
            tenant_id: auth.tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            sender: build_sender(auth),
            client_msg_id,
            message_type,
            body,
        })?;

    finalize_post_message_with_side_effects(
        state,
        auth,
        conversation_id,
        message_type_name,
        summary,
        result,
    )
}

fn publish_system_channel_message_with_side_effects(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: String,
    client_msg_id: Option<String>,
    body: MessageBody,
) -> Result<PostMessageResult, ApiError> {
    ensure_registered_device(state, auth)?;
    let summary = body.summary.clone();
    let result = state.conversation_runtime.publish_system_channel_message(
        PublishSystemChannelMessageCommand {
            tenant_id: auth.tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            publisher: build_sender(auth),
            client_msg_id,
            body,
        },
    )?;

    finalize_post_message_with_side_effects(
        state,
        auth,
        conversation_id,
        "standard",
        summary,
        result,
    )
}

fn finalize_post_message_with_side_effects(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: String,
    message_type_name: &str,
    summary: Option<String>,
    result: PostMessageResult,
) -> Result<PostMessageResult, ApiError> {
    let conversation_scope_id = conversation_id.clone();

    fanout_message_notifications(
        state,
        auth,
        conversation_id.as_str(),
        result.message_id.as_str(),
        result.message_seq,
        result.event_id.as_str(),
        message_type_name,
        summary.clone(),
    );

    state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: format!("audit_{}", result.message_id),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id,
            action: "message.posted".into(),
            payload: Some(
                serde_json::json!({
                    "messageId": result.message_id,
                    "messageSeq": result.message_seq,
                    "messageType": message_type_name,
                })
                .to_string(),
            ),
        },
    );

    publish_realtime_conversation_message_event(
        state,
        auth.tenant_id.as_str(),
        conversation_scope_id.as_str(),
        "message.posted",
        serde_json::json!({
            "conversationId": conversation_scope_id,
            "messageId": result.message_id,
            "messageSeq": result.message_seq,
            "messageType": message_type_name,
            "summary": summary,
        })
        .to_string(),
    )?;

    Ok(result)
}

fn fanout_message_notifications(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    source_event_id: &str,
    message_type_name: &str,
    summary: Option<String>,
) {
    let category = if message_type_name == "signal" {
        "rtc.event"
    } else {
        "message.new"
    };
    let payload = serde_json::json!({
        "conversationId": conversation_id,
        "messageId": message_id,
        "messageSeq": message_seq,
        "messageType": message_type_name,
    })
    .to_string();

    for member in state
        .conversation_runtime
        .list_members(auth.tenant_id.as_str(), conversation_id)
        .unwrap_or_default()
        .into_iter()
        .filter(|member| member.principal_id != auth.actor_id)
    {
        let _ = state.notification_runtime.request_notification(
            auth,
            RequestNotification {
                notification_id: format!("ntf_{}_{}", message_id, member.principal_id),
                source_event_id: source_event_id.into(),
                source_event_type: "message.posted".into(),
                category: category.into(),
                channel: "inapp".into(),
                recipient_id: member.principal_id,
                title: summary.clone(),
                body: summary.clone(),
                payload: Some(payload.clone()),
            },
        );
    }
}

fn publish_realtime_conversation_message_event(
    state: &AppState,
    tenant_id: &str,
    conversation_id: &str,
    event_type: &str,
    payload: String,
) -> Result<(), ApiError> {
    let principals = conversation_member_principal_ids(state, tenant_id, conversation_id)?;
    publish_realtime_event_to_principals(
        state,
        tenant_id,
        principals,
        "conversation",
        conversation_id,
        event_type,
        payload,
    );

    Ok(())
}

fn publish_realtime_membership_event(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
    event_type: &str,
    payload: String,
    additional_principals: BTreeSet<String>,
) -> Result<(), ApiError> {
    let mut principals =
        conversation_member_principal_ids(state, auth.tenant_id.as_str(), conversation_id)?;
    principals.extend(additional_principals);
    publish_realtime_event_to_principals(
        state,
        auth.tenant_id.as_str(),
        principals,
        "conversation",
        conversation_id,
        event_type,
        payload,
    );

    Ok(())
}

fn publish_realtime_agent_handoff_status_changed_event(
    state: &AppState,
    auth: &AuthContext,
    previous_state: &AgentHandoffStateView,
    current_state: &AgentHandoffStateView,
) -> Result<(), ApiError> {
    let changed_at = handoff_lifecycle_changed_at(current_state)
        .expect("agent handoff lifecycle state should expose a changed timestamp");
    let principals = conversation_member_principal_ids(
        state,
        auth.tenant_id.as_str(),
        current_state.conversation_id.as_str(),
    )?;

    publish_realtime_event_to_principals(
        state,
        auth.tenant_id.as_str(),
        principals,
        "conversation",
        current_state.conversation_id.as_str(),
        "conversation.agent_handoff_status_changed",
        serde_json::json!({
            "tenantId": auth.tenant_id.as_str(),
            "conversationId": current_state.conversation_id.as_str(),
            "previousStatus": previous_state.status.as_str(),
            "currentStatus": current_state.status.as_str(),
            "changedBy": {
                "id": auth.actor_id.as_str(),
                "kind": auth.actor_kind.as_str(),
            },
            "changedAt": changed_at,
            "state": current_state,
        })
        .to_string(),
    );

    Ok(())
}

fn publish_realtime_stream_frame_event(
    state: &AppState,
    auth: &AuthContext,
    frame: &im_domain_core::stream::StreamFrame,
) -> Result<(), ApiError> {
    let principals = stream_target_principal_ids(
        state,
        auth,
        frame.scope_kind.as_str(),
        frame.scope_id.as_str(),
    )?;

    publish_realtime_event_to_principals(
        state,
        auth.tenant_id.as_str(),
        principals,
        "stream",
        frame.stream_id.as_str(),
        "stream.frame.appended",
        serde_json::json!({
            "streamId": frame.stream_id,
            "streamType": frame.stream_type,
            "scopeKind": frame.scope_kind,
            "scopeId": frame.scope_id,
            "frameSeq": frame.frame_seq,
            "frameType": frame.frame_type,
        })
        .to_string(),
    );

    Ok(())
}

fn publish_realtime_stream_lifecycle_event(
    state: &AppState,
    auth: &AuthContext,
    session: &im_domain_core::stream::StreamSession,
    event_type: &str,
    reason: Option<String>,
) -> Result<(), ApiError> {
    let principals = stream_target_principal_ids(
        state,
        auth,
        session.scope_kind.as_str(),
        session.scope_id.as_str(),
    )?;

    publish_realtime_event_to_principals(
        state,
        auth.tenant_id.as_str(),
        principals,
        "stream",
        session.stream_id.as_str(),
        event_type,
        serde_json::json!({
            "streamId": session.stream_id,
            "streamType": session.stream_type,
            "scopeKind": session.scope_kind,
            "scopeId": session.scope_id,
            "state": session.state.as_wire_value(),
            "lastFrameSeq": session.last_frame_seq,
            "lastCheckpointSeq": session.last_checkpoint_seq,
            "resultMessageId": session.result_message_id,
            "closedAt": session.closed_at,
            "reason": reason,
        })
        .to_string(),
    );

    Ok(())
}

fn stream_target_principal_ids(
    state: &AppState,
    auth: &AuthContext,
    scope_kind: &str,
    scope_id: &str,
) -> Result<BTreeSet<String>, ApiError> {
    if scope_kind == "conversation" {
        conversation_member_principal_ids(state, auth.tenant_id.as_str(), scope_id)
    } else {
        Ok(BTreeSet::from([auth.actor_id.clone()]))
    }
}

fn conversation_member_principal_ids(
    state: &AppState,
    tenant_id: &str,
    conversation_id: &str,
) -> Result<BTreeSet<String>, ApiError> {
    Ok(state
        .conversation_runtime
        .list_members(tenant_id, conversation_id)?
        .into_iter()
        .map(|member| member.principal_id)
        .collect::<BTreeSet<_>>())
}

fn publish_realtime_event_to_principals(
    state: &AppState,
    tenant_id: &str,
    principal_ids: BTreeSet<String>,
    scope_type: &str,
    scope_id: &str,
    event_type: &str,
    payload: String,
) {
    for principal_id in principal_ids {
        let registered_devices = state
            .projection_service
            .registered_devices(tenant_id, principal_id.as_str())
            .into_iter()
            .map(|item| item.device_id)
            .collect::<Vec<_>>();
        for device_id in registered_devices {
            let _ = state.realtime_cluster.publish_device_event(
                state.node_id.as_str(),
                tenant_id,
                principal_id.as_str(),
                device_id.as_str(),
                scope_type,
                scope_id,
                event_type,
                payload.clone(),
            );
        }
    }
}

fn handoff_lifecycle_changed_at(state: &AgentHandoffStateView) -> Option<String> {
    match state.status.as_str() {
        "accepted" => state.accepted_at.clone(),
        "resolved" => state.resolved_at.clone(),
        "closed" => state.closed_at.clone(),
        _ => None,
    }
}

fn build_message_body(
    summary: Option<String>,
    text: Option<String>,
    parts: Vec<ContentPart>,
    render_hints: BTreeMap<String, String>,
) -> Result<MessageBody, ApiError> {
    let mut resolved_parts = Vec::new();
    if let Some(text) = text {
        if !text.trim().is_empty() {
            resolved_parts.push(ContentPart::text(text));
        }
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

fn emit_rtc_signal_message(
    state: &AppState,
    auth: &AuthContext,
    session: &im_domain_core::rtc::RtcSession,
    signal_type: &'static str,
) -> Result<(), ApiError> {
    let Some(conversation_id) = session.conversation_id.clone() else {
        return Ok(());
    };

    let payload = serde_json::json!({
        "rtcSessionId": session.rtc_session_id,
        "conversationId": session.conversation_id,
        "rtcMode": session.rtc_mode,
        "state": session.state,
        "signalingStreamId": session.signaling_stream_id,
        "artifactMessageId": session.artifact_message_id,
    })
    .to_string();

    post_message_with_side_effects(
        state,
        auth,
        conversation_id,
        None,
        MessageType::Signal,
        MessageBody {
            summary: Some(signal_type.into()),
            parts: vec![ContentPart::Signal(SignalPart {
                signal_type: signal_type.into(),
                schema_ref: Some("rtc.signal.v1".into()),
                payload,
            })],
            render_hints: BTreeMap::from([("channel".into(), "rtc".into())]),
        },
    )
    .map(|_| ())
}

fn emit_rtc_custom_signal_message(
    state: &AppState,
    auth: &AuthContext,
    signal: &im_domain_core::rtc::RtcSignalEvent,
) -> Result<(), ApiError> {
    let Some(conversation_id) = signal.conversation_id.clone() else {
        return Ok(());
    };

    let signal_payload = serde_json::from_str::<serde_json::Value>(signal.payload.as_str())
        .unwrap_or_else(|_| serde_json::Value::String(signal.payload.clone()));
    let payload = serde_json::json!({
        "rtcSessionId": signal.rtc_session_id,
        "conversationId": signal.conversation_id,
        "rtcMode": signal.rtc_mode,
        "signalingStreamId": signal.signaling_stream_id,
        "signalType": signal.signal_type,
        "signalPayload": signal_payload,
    })
    .to_string();

    post_message_with_side_effects(
        state,
        auth,
        conversation_id,
        None,
        MessageType::Signal,
        MessageBody {
            summary: Some(signal.signal_type.clone()),
            parts: vec![ContentPart::Signal(SignalPart {
                signal_type: signal.signal_type.clone(),
                schema_ref: signal
                    .schema_ref
                    .clone()
                    .or_else(|| Some("rtc.signal.v1".into())),
                payload,
            })],
            render_hints: BTreeMap::from([("channel".into(), "rtc".into())]),
        },
    )
    .map(|_| ())
}

fn record_membership_audit(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
    action: &str,
    member: &ConversationMember,
) {
    state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: format!("audit_{}_{}", action.replace('.', "_"), member.member_id),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id.into(),
            action: action.into(),
            payload: Some(
                serde_json::json!({
                    "memberId": member.member_id,
                    "principalId": member.principal_id,
                    "principalKind": member.principal_kind,
                    "role": member.role,
                    "state": member.state,
                })
                .to_string(),
            ),
        },
    );
}

fn record_owner_transfer_audit(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
    transfer: &TransferConversationOwnerResult,
) {
    state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: format!(
                "audit_conversation_owner_transferred_{}",
                transfer.new_owner.member_id
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id.into(),
            action: "conversation.owner_transferred".into(),
            payload: Some(
                serde_json::json!({
                    "previousOwnerMemberId": transfer.previous_owner.member_id,
                    "previousOwnerPrincipalId": transfer.previous_owner.principal_id,
                    "previousOwnerRole": transfer.previous_owner.role,
                    "newOwnerMemberId": transfer.new_owner.member_id,
                    "newOwnerPrincipalId": transfer.new_owner.principal_id,
                    "newOwnerRole": transfer.new_owner.role,
                    "transferredAt": transfer.transferred_at,
                })
                .to_string(),
            ),
        },
    );
}

fn record_member_role_change_audit(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
    change: &ChangeConversationMemberRoleResult,
) {
    state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: format!("audit_{}", change.event_id),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id.into(),
            action: "conversation.member_role_changed".into(),
            payload: Some(
                serde_json::json!({
                    "previousMemberId": change.previous_member.member_id,
                    "previousPrincipalId": change.previous_member.principal_id,
                    "previousRole": change.previous_member.role,
                    "updatedMemberId": change.updated_member.member_id,
                    "updatedPrincipalId": change.updated_member.principal_id,
                    "updatedRole": change.updated_member.role,
                    "changedAt": change.changed_at,
                })
                .to_string(),
            ),
        },
    );
}

fn ensure_registered_device(state: &AppState, auth: &AuthContext) -> Result<(), ApiError> {
    if let Some(device_id) = auth.device_id.as_deref() {
        bind_registered_device(
            state,
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            device_id,
            auth.session_id.as_deref(),
            "command",
            false,
        )?;
    }
    Ok(())
}

fn ensure_route_session_current(
    state: &AppState,
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
    session_id: Option<&str>,
) -> Result<(), ApiError> {
    state.realtime_cluster.ensure_route_session_current(
        tenant_id,
        principal_id,
        device_id,
        session_id,
    )?;
    Ok(())
}

fn bind_registered_device(
    state: &AppState,
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
    session_id: Option<&str>,
    connection_kind: &str,
    allow_session_takeover: bool,
) -> Result<projection_service::RegisteredDeviceView, ApiError> {
    if !allow_session_takeover {
        state.realtime_cluster.ensure_device_resume_not_required(
            tenant_id,
            principal_id,
            device_id,
        )?;
        state
            .session_presence_runtime
            .ensure_device_resume_not_required(tenant_id, principal_id, device_id)?;
        ensure_route_session_current(state, tenant_id, principal_id, device_id, session_id)?;
    }
    state
        .session_presence_runtime
        .register_device(tenant_id, principal_id, device_id)?;
    state
        .realtime_runtime
        .ensure_device_state(tenant_id, principal_id, device_id)?;
    let device = state
        .projection_service
        .register_device(tenant_id, principal_id, device_id);
    state.realtime_cluster.bind_device_route(
        tenant_id,
        principal_id,
        device_id,
        state.node_id.as_str(),
        session_id,
        connection_kind,
    )?;
    if allow_session_takeover {
        state
            .realtime_cluster
            .clear_device_disconnect_fence(tenant_id, principal_id, device_id)?;
    }
    refresh_node_operational_view(state);
    Ok(device)
}

fn refresh_node_operational_view(state: &AppState) {
    let lifecycle = state
        .realtime_cluster
        .node_lifecycle(state.node_id.as_str())
        .unwrap_or_else(|| session_gateway::RealtimeNodeLifecycleView {
            node_id: state.node_id.clone(),
            drain_status: "active".into(),
            rebalance_state: "stable".into(),
            owned_route_count: 0,
        });
    state.ops_runtime.set_node_lifecycle(
        lifecycle.drain_status.as_str(),
        lifecycle.rebalance_state.as_str(),
    );
    let routes = state
        .realtime_cluster
        .routes_for_node(state.node_id.as_str())
        .into_iter()
        .map(|route| RouteOwnershipView {
            tenant_id: route.tenant_id,
            principal_id: route.principal_id,
            device_id: route.device_id,
            owner_node_id: route.owner_node_id,
            connection_kind: route.connection_kind,
            bound_at: route.bound_at,
        })
        .collect::<Vec<_>>();
    state.ops_runtime.update_route_ownership(routes);
    let inspection = match state.runtime_dir.as_deref() {
        Some(runtime_dir) => inspect_runtime_dir(runtime_dir),
        None => RuntimeDirInspectionView::unmanaged(),
    };
    state.ops_runtime.update_runtime_dir_inspection(inspection);
}

fn resolve_requested_device_id(
    auth: &AuthContext,
    requested_device_id: Option<String>,
) -> Result<String, ApiError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            if requested != bound {
                return Err(ApiError::bad_request(
                    "device_id_mismatch",
                    format!("device id does not match auth context: {requested}"),
                ));
            }
            Ok(requested)
        }
        (Some(requested), None) => Ok(requested),
        (None, Some(bound)) => Ok(bound),
        (None, None) => Err(ApiError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

fn validate_device_scope(auth: &AuthContext, device_id: &str) -> Result<(), ApiError> {
    if let Some(bound_device_id) = auth.device_id.as_deref() {
        if bound_device_id != device_id {
            return Err(ApiError::forbidden(
                "device_scope_forbidden",
                format!("device scope forbidden: {device_id}"),
            ));
        }
    }
    Ok(())
}

fn ensure_conversation_member(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
) -> Result<(), ApiError> {
    state.conversation_runtime.require_active_member(
        auth.tenant_id.as_str(),
        conversation_id,
        auth.actor_id.as_str(),
    )?;
    Ok(())
}

fn resolve_conversation_actor_auth_context(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
) -> Result<AuthContext, ApiError> {
    let actor_member = state.conversation_runtime.require_active_member(
        auth.tenant_id.as_str(),
        conversation_id,
        auth.actor_id.as_str(),
    )?;
    let mut actor_auth = auth.clone();
    actor_auth.actor_kind = actor_member.principal_kind;
    Ok(actor_auth)
}

fn ensure_conversation_bound_write_access(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
    capability: &str,
) -> Result<(), ApiError> {
    state
        .conversation_runtime
        .ensure_conversation_bound_write_allowed_with_actor_kind(
            auth.tenant_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            capability,
        )?;
    Ok(())
}

fn ensure_rtc_create_access(
    state: &AppState,
    auth: &AuthContext,
    request: &CreateRtcSessionRequest,
) -> Result<(), ApiError> {
    match state
        .rtc_runtime
        .session(auth, request.rtc_session_id.as_str())
    {
        Ok(session) => {
            if let Some(conversation_id) = session.conversation_id.as_deref() {
                ensure_conversation_bound_write_access(state, auth, conversation_id, "rtc.create")?;
            }
        }
        Err(error) if error.code() == "rtc_session_not_found" => {
            if let Some(conversation_id) = request.conversation_id.as_deref() {
                ensure_conversation_bound_write_access(state, auth, conversation_id, "rtc.create")?;
            }
        }
        Err(error) => return Err(error.into()),
    }

    Ok(())
}

fn ensure_rtc_session_conversation_write_access(
    state: &AppState,
    auth: &AuthContext,
    rtc_session_id: &str,
    capability: &str,
) -> Result<(), ApiError> {
    let session = state.rtc_runtime.session(auth, rtc_session_id)?;
    if let Some(conversation_id) = session.conversation_id.as_deref() {
        ensure_conversation_bound_write_access(state, auth, conversation_id, capability)?;
    }

    Ok(())
}

fn ensure_stream_open_access(
    state: &AppState,
    auth: &AuthContext,
    request: &OpenStreamRequest,
) -> Result<(), ApiError> {
    match state
        .streaming_runtime
        .session(auth, request.stream_id.as_str())
    {
        Ok(session) => {
            if session.scope_kind == "conversation" {
                ensure_conversation_bound_write_access(
                    state,
                    auth,
                    session.scope_id.as_str(),
                    "stream.open",
                )?;
            }
        }
        Err(error) if error.code() == "stream_not_found" => {
            if request.scope_kind == "conversation" {
                ensure_conversation_bound_write_access(
                    state,
                    auth,
                    request.scope_id.as_str(),
                    "stream.open",
                )?;
            }
        }
        Err(error) => return Err(error.into()),
    }

    Ok(())
}

fn ensure_stream_session_conversation_member(
    state: &AppState,
    auth: &AuthContext,
    stream_id: &str,
) -> Result<(), ApiError> {
    let session = state.streaming_runtime.session(auth, stream_id)?;
    if session.scope_kind == "conversation" {
        ensure_conversation_member(state, auth, session.scope_id.as_str())?;
    }

    Ok(())
}

fn ensure_stream_session_write_access(
    state: &AppState,
    auth: &AuthContext,
    stream_id: &str,
    capability: &str,
) -> Result<(), ApiError> {
    let session = state.streaming_runtime.session(auth, stream_id)?;
    if session.scope_kind == "conversation" {
        ensure_conversation_bound_write_access(state, auth, session.scope_id.as_str(), capability)?;
    }

    Ok(())
}
