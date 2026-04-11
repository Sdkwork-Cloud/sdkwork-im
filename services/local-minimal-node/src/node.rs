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
    AgentHandoffStateView, ChangeConversationMemberRoleResult, ConversationRuntime,
    CreateConversationResult, EditMessageCommand, MessageMutationResult, PostMessageCommand,
    PostMessageResult, PublishSystemChannelMessageCommand, RecallMessageCommand,
    TransferConversationOwnerResult,
};
use im_adapters_local_disk::{
    FileAutomationExecutionStore, FileCommitJournal, FileMetadataStore, FileNotificationTaskStore,
    FilePresenceStateStore, FileRealtimeCheckpointStore, FileRealtimeDisconnectFenceStore,
    FileRealtimeSubscriptionStore, FileRtcStateStore, FileStreamStateStore,
    FileTimelineProjectionStore, read_commit_journal_file,
    validate_automation_execution_store_file, validate_commit_journal_file,
    validate_device_twin_store_file, validate_notification_task_store_file,
    validate_presence_state_store_file, validate_realtime_checkpoint_store_file,
    validate_realtime_disconnect_fence_store_file, validate_realtime_subscription_store_file,
    validate_rtc_state_store_file, validate_stream_state_store_file,
};
use im_adapters_local_memory::{MemoryCommitJournal, MemoryRealtimeCheckpointStore};
use im_auth_context::AuthContext;
use im_auth_context::{AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context};
use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationMember, ConversationReadCursorView, DeviceSyncFeedEntry,
    MembershipRole,
};
use im_domain_core::media::MediaProcessingState;
use im_domain_core::message::{ContentPart, MediaPart, MessageBody, MessageType, SignalPart};
use im_domain_core::realtime::RealtimeSubscription;
use im_domain_core::session::{PresenceSnapshotView, SessionResumeView};
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{
    CommitJournal, CommitPosition, ContractError, DeviceAccessProvider, DeviceTwinStore,
    EffectiveProviderBinding, IotProtocolAdapter, MetadataStore,
    PROVIDER_REGISTRY_INTERFACE_VERSION, ProviderDomain, ProviderPluginDescriptor,
    ProviderRegistry, RealtimeCheckpointRecord, RealtimeDisconnectFenceRecord,
    RealtimeSubscriptionRecord, RtcStateRecord, StaticProviderRegistry, StreamStateRecord,
    UserModuleProvider,
};
use media_service::{CompleteUploadRequest, CreateUploadRequest, MediaRuntime};
use notification_service::{NotificationRuntime, NotificationTask, RequestNotification};
use ops_service::{
    ClusterView, DiagnosticBundle, LagItem, LagView, OpsHealthResponse, OpsRuntime,
    ProviderBindingDriftView, ProviderBindingItemView, ProviderBindingSnapshotView,
    ProviderBindingsView, RouteOwnershipView, RuntimeDirInspectionItem, RuntimeDirInspectionView,
};
use projection_service::{ProjectionAccessError, TimelineProjectionService};
use rtc_signaling_service::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest,
    PostRtcSignalRequest, RtcRuntime, UpdateRtcSessionRequest,
};
use serde::{Deserialize, Serialize};
use session_gateway::{
    AckRealtimeEventsRequest, ListRealtimeEventsQuery, PresenceRuntimeError, RealtimeClusterBridge,
    RealtimeClusterError, RealtimeDeliveryRuntime, RealtimePlaneAssembly, RealtimeRuntimeError,
    SessionPresenceRuntime, SyncRealtimeSubscriptionsRequest, serve_realtime_websocket,
};
use streaming_service::{
    AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest,
    ListStreamFramesQuery, OpenStreamRequest, StreamFrameWindow, StreamingRuntime,
};

mod access;
mod build;
mod conversation;
mod device_registration;
mod effects;
mod handoff;
mod iot;
mod media;
mod membership;
mod message;
mod platform;
mod projection;
mod rtc;
mod runtime_dir;
mod session;
mod stream;
mod twin;
mod user_module;

use self::device_registration::{DisconnectActiveDeviceRouteOutcome, LocalNodeDeviceRegistration};

pub use build::{
    build_app_with_dependencies, build_app_with_dependencies_and_runtime, build_default_app,
    build_default_app_with_device_access_provider, build_default_app_with_iot_protocol_adapter,
    build_default_app_with_runtime_dir,
    build_default_app_with_runtime_dir_and_device_access_provider,
    build_default_app_with_runtime_dir_and_iot_protocol_adapter,
    build_default_app_with_runtime_dir_and_user_module_provider,
    build_default_app_with_user_module_provider, build_public_app,
    build_public_app_with_runtime_dir,
};
pub use runtime_dir::{
    RuntimeDirArchivePruneActionView, RuntimeDirArchivePruneView, RuntimeDirArchiveView,
    RuntimeDirBackupCatalogItemView, RuntimeDirBackupCatalogView, RuntimeDirRepairActionView,
    RuntimeDirRepairView, RuntimeDirRestorePreviewActionView,
    RuntimeDirRestorePreviewChangeSummaryView, RuntimeDirRestorePreviewDomainSummaryView,
    RuntimeDirRestorePreviewView, RuntimeDirRestoreView, archive_runtime_backup,
    archive_runtime_backup_with_policy, format_runtime_backup_catalog, format_runtime_dir_archive,
    format_runtime_dir_archive_prune, format_runtime_dir_inspection, format_runtime_dir_repair,
    format_runtime_dir_restore, format_runtime_dir_restore_preview, inspect_runtime_dir,
    list_runtime_backups, preview_restore_runtime_dir, prune_archived_runtime_backups,
    repair_runtime_dir, restore_runtime_dir, restore_runtime_dir_with_expected_preview_fingerprint,
};

#[derive(Clone)]
struct AppState {
    node_id: String,
    runtime_dir: Option<PathBuf>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    conversation_runtime: Arc<ConversationRuntime<ProjectionJournal>>,
    user_module_provider: Arc<dyn UserModuleProvider>,
    projection_service: Arc<TimelineProjectionService>,
    session_presence_runtime: Arc<SessionPresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    device_registration: LocalNodeDeviceRegistration,
    device_twin_store: Arc<dyn DeviceTwinStore>,
    iot_protocol_adapter: Arc<dyn IotProtocolAdapter>,
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
    snapshot_stores: Option<ProjectionSnapshotStores>,
}

#[derive(Clone)]
enum ProjectionJournalInner {
    Memory(MemoryCommitJournal),
    File(FileCommitJournal),
}

#[derive(Clone)]
struct ProjectionSnapshotStores {
    metadata: FileMetadataStore,
    timeline: FileTimelineProjectionStore,
}

#[derive(Default)]
struct ProjectionSnapshotRestoreSummary {
    restored_checkpoints: BTreeMap<String, u64>,
    restored_device_sync: bool,
}

const PROJECTION_METADATA_FILE_NAME: &str = "projection-metadata.json";
const PROJECTION_TIMELINE_FILE_NAME: &str = "projection-timeline.json";
const PROJECTION_SNAPSHOT_CHECKPOINT_KEY: &str = "conversation-snapshot-checkpoint";

impl AppState {
    #[rustfmt::skip]
    fn require_registered_device_binding(&self, auth: &AuthContext) -> Result<(), ApiError> {
        self.device_registration.ensure_registered_device(self, auth)
    }

    fn bind_device_registration(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<projection_service::RegisteredDeviceView, ApiError> {
        self.device_registration.bind_registered_device(
            self,
            tenant_id,
            principal_id,
            device_id,
            session_id,
            connection_kind,
            allow_session_takeover,
        )
    }

    fn prepare_active_device_route(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
    ) -> Result<projection_service::RegisteredDeviceView, ApiError> {
        self.device_registration.prepare_active_device_route(
            self,
            tenant_id,
            principal_id,
            device_id,
            session_id,
            connection_kind,
        )
    }

    fn disconnect_active_device_route(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
    ) -> Result<DisconnectActiveDeviceRouteOutcome, ApiError> {
        self.device_registration.disconnect_active_device_route(
            self,
            tenant_id,
            principal_id,
            device_id,
            session_id,
            connection_kind,
        )
    }

    fn iot_access_provider_health(&self) -> im_platform_contracts::ProviderHealthSnapshot {
        self.device_registration.provider_health_snapshot()
    }

    fn iot_protocol_provider_health(&self) -> im_platform_contracts::ProviderHealthSnapshot {
        self.iot_protocol_adapter.provider_health_snapshot()
    }

    fn user_module_provider_health(&self) -> im_platform_contracts::ProviderHealthSnapshot {
        self.user_module_provider.provider_health_snapshot()
    }

    fn provider_binding_snapshots(&self) -> Vec<ProviderBindingSnapshotView> {
        let registry = StaticProviderRegistry::platform_default();
        let mut bindings = BTreeMap::new();

        for domain in ProviderDomain::ALL {
            if let Some(binding) = registry.effective_binding(domain, None) {
                bindings.insert(domain, binding);
            }
        }

        if let Ok(binding) = self.rtc_runtime.provider_binding(None) {
            bindings.insert(ProviderDomain::Rtc, binding);
        }
        if let Ok(binding) = self.media_runtime.provider_binding(None) {
            bindings.insert(ProviderDomain::ObjectStorage, binding);
        }

        bindings.insert(
            ProviderDomain::UserModule,
            binding_from_descriptor(&registry, self.user_module_provider.descriptor()),
        );
        bindings.insert(
            ProviderDomain::IotAccess,
            binding_from_descriptor(&registry, self.device_registration.provider_descriptor()),
        );
        bindings.insert(
            ProviderDomain::IotProtocol,
            binding_from_descriptor(&registry, self.iot_protocol_adapter.descriptor()),
        );

        vec![ProviderBindingSnapshotView {
            interface_version: PROVIDER_REGISTRY_INTERFACE_VERSION.into(),
            tenant_id: None,
            effective_bindings: ProviderDomain::ALL
                .into_iter()
                .filter_map(|domain| bindings.get(&domain))
                .map(provider_binding_item_view)
                .collect(),
            precedence: vec![
                "tenant_override".into(),
                "deployment_profile".into(),
                "global_default".into(),
            ],
        }]
    }
}

fn binding_from_descriptor(
    registry: &StaticProviderRegistry,
    descriptor: ProviderPluginDescriptor,
) -> EffectiveProviderBinding {
    let mut binding = registry
        .effective_binding(descriptor.domain, None)
        .unwrap_or(EffectiveProviderBinding {
            domain: descriptor.domain,
            default_plugin_id: None,
            selected_plugin_id: None,
            selection_source: "deployment_required".into(),
            tenant_override_allowed: descriptor.tenant_override_allowed,
        });

    if descriptor.default_selected {
        binding.default_plugin_id = Some(descriptor.plugin_id.clone());
    }
    binding.selected_plugin_id = Some(descriptor.plugin_id);
    binding.selection_source = if descriptor.default_selected {
        "global_default".into()
    } else {
        "deployment_profile".into()
    };
    binding.tenant_override_allowed = descriptor.tenant_override_allowed;
    binding
}

fn provider_binding_item_view(binding: &EffectiveProviderBinding) -> ProviderBindingItemView {
    ProviderBindingItemView {
        domain: binding.domain.as_str().into(),
        default_plugin_id: binding.default_plugin_id.clone(),
        selected_plugin_id: binding.selected_plugin_id.clone(),
        selection_source: binding.selection_source.clone(),
        tenant_override_allowed: binding.tenant_override_allowed,
    }
}

impl ProjectionJournal {
    fn new_memory(projection_service: Arc<TimelineProjectionService>) -> Self {
        Self {
            inner: ProjectionJournalInner::Memory(MemoryCommitJournal::with_partition(
                "local-minimal",
            )),
            projection_service,
            snapshot_stores: None,
        }
    }

    fn new_file(
        projection_service: Arc<TimelineProjectionService>,
        file_path: impl Into<PathBuf>,
        snapshot_stores: ProjectionSnapshotStores,
    ) -> Self {
        Self {
            inner: ProjectionJournalInner::File(FileCommitJournal::new("local-minimal", file_path)),
            projection_service,
            snapshot_stores: Some(snapshot_stores),
        }
    }

    fn recorded(&self) -> Result<Vec<CommitEnvelope>, ContractError> {
        match &self.inner {
            ProjectionJournalInner::Memory(inner) => Ok(inner.recorded()),
            ProjectionJournalInner::File(inner) => inner.recorded(),
        }
    }

    fn restore_projection_snapshots(
        &self,
        recorded: &[CommitEnvelope],
        projection_service: &TimelineProjectionService,
    ) -> ProjectionSnapshotRestoreSummary {
        let Some(snapshot_stores) = self.snapshot_stores.as_ref() else {
            return ProjectionSnapshotRestoreSummary::default();
        };
        let restored_device_sync = snapshot_stores.restore_device_sync_snapshot(projection_service);

        let mut scopes = BTreeMap::new();
        for envelope in recorded
            .iter()
            .filter(|envelope| envelope.scope_type == "conversation")
        {
            scopes
                .entry(projection_snapshot_scope(
                    envelope.tenant_id.as_str(),
                    envelope.scope_id.as_str(),
                ))
                .or_insert_with(|| (envelope.tenant_id.clone(), envelope.scope_id.clone()));
        }
        for scope in snapshot_stores
            .metadata
            .scopes_for_key(PROJECTION_SNAPSHOT_CHECKPOINT_KEY)
        {
            let Some((tenant_id, conversation_id)) =
                parse_projection_snapshot_scope(scope.as_str())
            else {
                continue;
            };
            let tenant_id = tenant_id.to_owned();
            let conversation_id = conversation_id.to_owned();
            scopes.entry(scope).or_insert((tenant_id, conversation_id));
        }

        let mut restored_checkpoints = BTreeMap::new();
        for (scope, (tenant_id, conversation_id)) in scopes {
            if let Some(checkpoint) = snapshot_stores.restore_scope_checkpoint(
                projection_service,
                tenant_id.as_str(),
                conversation_id.as_str(),
            ) {
                restored_checkpoints.insert(scope, checkpoint);
            }
        }

        ProjectionSnapshotRestoreSummary {
            restored_checkpoints,
            restored_device_sync,
        }
    }

    fn snapshot_stores(&self) -> Option<ProjectionSnapshotStores> {
        self.snapshot_stores.clone()
    }
}

impl CommitJournal for ProjectionJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let position = match &self.inner {
            ProjectionJournalInner::Memory(inner) => inner.append(envelope.clone())?,
            ProjectionJournalInner::File(inner) => inner.append(envelope.clone())?,
        };

        if self.projection_service.apply(&envelope).is_ok()
            && let Some(snapshot_stores) = self.snapshot_stores.as_ref()
        {
            snapshot_stores.persist_for_envelope(self.projection_service.as_ref(), &envelope);
        }

        Ok(position)
    }
}

impl ProjectionSnapshotStores {
    fn new(metadata: FileMetadataStore, timeline: FileTimelineProjectionStore) -> Self {
        Self { metadata, timeline }
    }

    fn persist_for_envelope(
        &self,
        projection_service: &TimelineProjectionService,
        envelope: &CommitEnvelope,
    ) {
        if envelope.scope_type != "conversation" {
            return;
        }

        let scope =
            projection_snapshot_scope(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        if !matches!(
            projection_service.persist_conversation_snapshot(
                envelope.tenant_id.as_str(),
                envelope.scope_id.as_str(),
                &self.metadata,
                &self.timeline,
            ),
            Ok(true)
        ) {
            return;
        }

        let checkpoint = envelope.ordering_seq.to_string();
        let _ = self.metadata.put_snapshot(
            scope.as_str(),
            PROJECTION_SNAPSHOT_CHECKPOINT_KEY,
            checkpoint.as_str(),
        );
    }

    fn restore_scope_checkpoint(
        &self,
        projection_service: &TimelineProjectionService,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Option<u64> {
        let scope = projection_snapshot_scope(tenant_id, conversation_id);
        let checkpoint = self
            .metadata
            .load_snapshot(scope.as_str(), PROJECTION_SNAPSHOT_CHECKPOINT_KEY)
            .ok()
            .flatten()?
            .parse::<u64>()
            .ok()?;
        if matches!(
            projection_service.restore_conversation_snapshot(
                tenant_id,
                conversation_id,
                &self.metadata,
                &self.timeline,
            ),
            Ok(true)
        ) {
            Some(checkpoint)
        } else {
            None
        }
    }

    fn persist_device_sync_snapshot(&self, projection_service: &TimelineProjectionService) {
        let _ = projection_service.persist_device_sync_snapshot(&self.metadata, &self.timeline);
    }

    fn restore_device_sync_snapshot(&self, projection_service: &TimelineProjectionService) -> bool {
        projection_service
            .restore_device_sync_snapshot(&self.metadata, &self.timeline)
            .unwrap_or(false)
    }
}

fn projection_snapshot_scope(tenant_id: &str, conversation_id: &str) -> String {
    CommitEnvelope::ordering_key(tenant_id, conversation_id)
}

fn parse_projection_snapshot_scope(scope: &str) -> Option<(&str, &str)> {
    scope.split_once(':')
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

    fn not_found(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::NOT_FOUND,
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
            conversation_runtime::RuntimeError::InvalidInput(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "conversation_request_invalid",
                message,
            },
            conversation_runtime::RuntimeError::ConversationNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_not_found",
                message,
            },
            conversation_runtime::RuntimeError::ConversationBindingNotFound(message) => Self {
                status: axum::http::StatusCode::NOT_FOUND,
                code: "conversation_binding_not_found",
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

impl From<ContractError> for ApiError {
    fn from(value: ContractError) -> Self {
        match value {
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "provider_capability_unsupported",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "provider_conflict",
                message,
            },
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "provider_unavailable",
                message,
            },
        }
    }
}

impl From<ProjectionAccessError> for ApiError {
    fn from(value: ProjectionAccessError) -> Self {
        Self {
            status: value.status(),
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
