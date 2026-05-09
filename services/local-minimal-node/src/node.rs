use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path as StdPath, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use audit_service::{
    AuditExportBundle, AuditRecordMutationResponse, AuditRuntime, RecordAuditAnchor,
    audit_record_request_key,
};
use automation_service::{AutomationExecution, AutomationRuntime, RequestAutomationExecution};
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::http::header::CONTENT_TYPE;
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{
    Json, Router,
    routing::{get, post},
};
use control_plane_api::SocialControlQuery;
use conversation_runtime::{
    AgentHandoffStateView, BindDirectChatConversationCommand, ChangeConversationMemberRoleResult,
    ConversationRuntime, CreateConversationResult, EditMessageCommand, MessageMutationResult,
    PostMessageCommand, PostMessageResult, PublishSystemChannelMessageCommand,
    RecallMessageCommand, TransferConversationOwnerResult,
};
use im_adapters_local_disk::{
    FileAutomationExecutionStore, FileCommitJournal, FileMetadataStore, FileNotificationTaskStore,
    FilePresenceStateStore, FileRealtimeCheckpointStore, FileRealtimeDisconnectFenceStore,
    FileRealtimeEventWindowStore, FileRealtimeSubscriptionStore, FileRtcStateStore,
    FileStreamStateStore, FileTimelineProjectionStore, read_commit_journal_file,
    validate_automation_execution_store_file, validate_commit_journal_file,
    validate_device_twin_store_file, validate_notification_task_store_file,
    validate_presence_state_store_file, validate_realtime_checkpoint_store_file,
    validate_realtime_disconnect_fence_store_file, validate_realtime_event_window_store_file,
    validate_realtime_subscription_store_file, validate_rtc_state_store_file,
    validate_stream_state_store_file,
};
use im_adapters_local_memory::{MemoryCommitJournal, MemoryRealtimeCheckpointStore};
use im_auth_context::AuthContext;
use im_auth_context::{AuthContextError, resolve_public_bearer_auth_context};
use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationMember, ConversationReadCursorView, MembershipRole,
};
use im_domain_core::media::MediaProcessingState;
use im_domain_core::message::{ContentPart, MediaPart, MessageBody, MessageType, SignalPart};
use im_domain_core::realtime::RealtimeSubscription;
use im_domain_core::session::{PresenceSnapshotView, SessionResumeView};
use im_domain_core::social::{DirectChat, FriendRequest, Friendship};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    CommitJournal, CommitPosition, ContractError, DeviceAccessProvider, DeviceTwinStore,
    EffectiveProviderBinding, IotProtocolAdapter, MetadataStore,
    PROVIDER_REGISTRY_INTERFACE_VERSION, ProviderDomain, ProviderPluginDescriptor,
    ProviderRegistry, RealtimeCheckpointRecord, RealtimeDisconnectFenceRecord,
    RealtimeEventWindowRecord, RealtimeSubscriptionRecord, RtcStateRecord, StaticProviderRegistry,
    StreamStateRecord, UserModuleProvider,
};
use media_service::{
    CompleteUploadRequest, CreateUploadRequest, MediaRuntime, MediaUploadMutationResponse,
    media_complete_upload_request_key, media_create_upload_request_key,
};
use notification_service::{
    NotificationRequestResponse, NotificationRuntime, NotificationTask, RequestNotification,
};
use ops_service::{
    ClusterView, DiagnosticBundle, LagItem, LagView, OpsHealthResponse, OpsRuntime,
    ProviderBindingDriftView, ProviderBindingItemView, ProviderBindingSnapshotView,
    ProviderBindingsView, RouteOwnershipView, RuntimeDirInspectionItem, RuntimeDirInspectionView,
};
use projection_service::{
    ContactView, ConversationMemberDirectoryEntry, MessageInteractionSummaryView,
    NotificationRecipientView, ProjectionAccessError, TimelineProjectionService,
};
use rtc_signaling_service::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest,
    PostRtcSignalRequest, RtcRuntime, RtcSessionMutationResponse, UpdateRtcSessionRequest,
    rtc_create_request_key, rtc_session_action_request_key,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use session_gateway::{
    AckRealtimeEventsRequest, ListRealtimeEventsQuery, PresenceRuntimeError, RealtimeClusterBridge,
    RealtimeClusterError, RealtimeDeliveryRuntime, RealtimePlaneAssembly, RealtimeRuntimeError,
    RealtimeScopeAccessPolicy, SessionPresenceRuntime, SyncRealtimeSubscriptionsRequest,
    serve_realtime_websocket,
};
use sha2::{Digest, Sha256};
use streaming_service::{
    AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest,
    ListStreamFramesQuery, OpenStreamRequest, StreamFrameMutationResponse, StreamFrameWindow,
    StreamSessionMutationResponse, StreamingRuntime, stream_abort_request_key,
    stream_append_request_key, stream_checkpoint_request_key, stream_complete_request_key,
    stream_open_request_key,
};

mod access;
mod auth;
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
mod portal;
mod projection;
mod realtime_policy;
mod rtc;
mod runtime_dir;
mod session;
mod side_effect_outbox;
mod social;
mod stream;
mod twin;
mod user_center;
mod user_module;

use self::device_registration::{DisconnectActiveDeviceRouteOutcome, LocalNodeDeviceRegistration};

pub use build::{
    build_app_with_dependencies, build_app_with_dependencies_and_runtime,
    build_app_with_dependencies_and_runtime_dir,
    build_app_with_dependencies_realtime_and_notification_runtime, build_default_app,
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
pub use user_center::{
    UserCenterProviderKind, UserCenterRuntimeConfig, UserCenterRuntimeMode,
    resolve_user_center_runtime_config,
};

#[derive(Clone)]
struct AppState {
    node_id: String,
    runtime_dir: Option<PathBuf>,
    auth_runtime: Arc<auth::AuthRuntime>,
    control_plane_app: Router,
    social_query: Arc<SocialControlQuery>,
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
    message_side_effect_outbox: Arc<dyn side_effect_outbox::MessageSideEffectOutboxStore>,
    projection_replay_state: Arc<std::sync::Mutex<ProjectionReplayAuthorityState>>,
    pending_friend_request_accept_repairs:
        Arc<std::sync::Mutex<PendingFriendRequestAcceptanceRepairStore>>,
    friend_request_accept_repair_gate: Arc<tokio::sync::Mutex<()>>,
}

type PendingFriendRequestAcceptanceRepairStore =
    BTreeMap<String, PendingFriendRequestAcceptanceRepair>;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PendingFriendRequestAcceptanceRepair {
    tenant_id: String,
    request_id: String,
    requester_user_id: String,
    target_user_id: String,
    accepted_at: String,
}

#[derive(Clone)]
struct ProjectionJournal {
    inner: ProjectionJournalInner,
    projection_service: Arc<TimelineProjectionService>,
    snapshot_stores: Option<ProjectionSnapshotStores>,
    replay_state: Arc<std::sync::Mutex<ProjectionReplayAuthorityState>>,
    applied_social_projection_event_ids: Arc<std::sync::Mutex<BTreeSet<String>>>,
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

#[derive(Default)]
struct ProjectionReplayAuthorityState {
    applied_event_count: usize,
}

const PROJECTION_METADATA_FILE_NAME: &str = "projection-metadata.json";
const PROJECTION_TIMELINE_FILE_NAME: &str = "projection-timeline.json";
const PROJECTION_SNAPSHOT_CHECKPOINT_KEY: &str = "conversation-snapshot-checkpoint";
const LOCAL_NODE_AUDIT_AGGREGATE_ID_MAX_BYTES: usize = 256;
const LOCAL_NODE_AUDIT_RECORD_ID_MAX_BYTES: usize = 256;
const APP_OPENAPI_SCHEMA_PATH_ENV: &str = "CRAW_CHAT_APP_OPENAPI_SCHEMA_PATH";
const APP_OPENAPI_SCHEMA_PATH: &str = "/openapi/craw-chat-app.openapi.yaml";
const APP_OPENAPI_SCHEMA_EMBEDDED_YAML: &str =
    include_str!("../../../sdks/sdkwork-im-sdk/openapi/craw-chat-app.openapi.yaml");
const PUBLIC_BROWSER_ORIGINS_ENV: &str = "CRAW_CHAT_BROWSER_ORIGINS";
const DEFAULT_PUBLIC_BROWSER_ORIGINS: &[&str] = &["http://127.0.0.1:4176", "http://localhost:4176"];

fn stable_local_audit_aggregate_id(namespace: &str, business_id: &str) -> String {
    if business_id.len() <= LOCAL_NODE_AUDIT_AGGREGATE_ID_MAX_BYTES {
        return business_id.into();
    }

    let digest = Sha256::digest(business_id.as_bytes());
    let bounded_aggregate_id = format!("{namespace}:{digest:x}");
    debug_assert!(bounded_aggregate_id.len() <= LOCAL_NODE_AUDIT_AGGREGATE_ID_MAX_BYTES);
    bounded_aggregate_id
}

fn stable_local_audit_record_id(prefix: &str, business_id: &str) -> String {
    let raw_record_id = format!("{prefix}{business_id}");
    if raw_record_id.len() <= LOCAL_NODE_AUDIT_RECORD_ID_MAX_BYTES {
        return raw_record_id;
    }

    let digest = Sha256::digest(business_id.as_bytes());
    let digest_component = format!("sha256_{digest:x}");
    let max_prefix_bytes =
        LOCAL_NODE_AUDIT_RECORD_ID_MAX_BYTES.saturating_sub(digest_component.len());
    let prefix = if prefix.len() > max_prefix_bytes {
        &prefix[..max_prefix_bytes]
    } else {
        prefix
    };
    let bounded_record_id = format!("{prefix}{digest_component}");
    debug_assert!(bounded_record_id.len() <= LOCAL_NODE_AUDIT_RECORD_ID_MAX_BYTES);
    bounded_record_id
}

impl AppState {
    #[rustfmt::skip]
    fn require_registered_device_binding(&self, auth: &AuthContext) -> Result<(), ApiError> {
        self.device_registration.ensure_registered_device(self, auth)
    }

    fn bind_device_registration(
        &self,
        auth: &AuthContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<projection_service::RegisteredDeviceView, ApiError> {
        self.device_registration.bind_registered_device(
            self,
            auth,
            device_id,
            connection_kind,
            allow_session_takeover,
        )
    }

    #[rustfmt::skip]
    fn prepare_active_device_route(
        &self,
        auth: &AuthContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<projection_service::RegisteredDeviceView, ApiError> {
        self.device_registration.prepare_active_device_route(self, auth, device_id, connection_kind)
    }

    fn disconnect_active_device_route(
        &self,
        auth: &AuthContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<DisconnectActiveDeviceRouteOutcome, ApiError> {
        self.device_registration.disconnect_active_device_route(
            self,
            auth,
            device_id,
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
            replay_state: Arc::new(std::sync::Mutex::new(
                ProjectionReplayAuthorityState::default(),
            )),
            applied_social_projection_event_ids: Arc::new(std::sync::Mutex::new(BTreeSet::new())),
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
            replay_state: Arc::new(std::sync::Mutex::new(
                ProjectionReplayAuthorityState::default(),
            )),
            applied_social_projection_event_ids: Arc::new(std::sync::Mutex::new(BTreeSet::new())),
        }
    }

    fn replay_state(&self) -> Arc<std::sync::Mutex<ProjectionReplayAuthorityState>> {
        self.replay_state.clone()
    }

    fn set_applied_event_count(&self, applied_event_count: usize) {
        let mut state = self
            .replay_state
            .lock()
            .expect("projection replay authority state should not be poisoned");
        state.applied_event_count = applied_event_count;
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

    fn mark_social_projection_events<'a>(
        &self,
        envelopes: impl IntoIterator<Item = &'a CommitEnvelope>,
    ) {
        let mut applied = self
            .applied_social_projection_event_ids
            .lock()
            .expect("social projection event id set should not be poisoned");
        for envelope in envelopes {
            if !tracks_social_projection_event(envelope) {
                continue;
            }
            applied.insert(envelope.event_id.clone());
        }
    }

    fn apply_committed_projection_envelope(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), ContractError> {
        self.projection_service.apply(envelope).map_err(|error| {
            ContractError::Unavailable(format!(
                "projection apply failed for committed event {} ({}): {error:?}",
                envelope.event_id, envelope.event_type
            ))
        })?;
        if let Some(snapshot_stores) = self.snapshot_stores.as_ref() {
            snapshot_stores.persist_for_envelope(self.projection_service.as_ref(), envelope);
        }
        self.mark_social_projection_events(std::iter::once(envelope));
        Ok(())
    }
}

impl CommitJournal for ProjectionJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut replay_state = self
            .replay_state
            .lock()
            .expect("projection replay authority state should not be poisoned");
        let position = match &self.inner {
            ProjectionJournalInner::Memory(inner) => inner.append(envelope.clone())?,
            ProjectionJournalInner::File(inner) => inner.append(envelope.clone())?,
        };

        self.apply_committed_projection_envelope(&envelope)?;
        replay_state.applied_event_count = replay_state.applied_event_count.saturating_add(1);

        Ok(position)
    }

    fn append_batch(
        &self,
        envelopes: Vec<CommitEnvelope>,
    ) -> Result<Vec<CommitPosition>, ContractError> {
        let mut replay_state = self
            .replay_state
            .lock()
            .expect("projection replay authority state should not be poisoned");
        let positions = match &self.inner {
            ProjectionJournalInner::Memory(inner) => inner.append_batch(envelopes.clone())?,
            ProjectionJournalInner::File(inner) => inner.append_batch(envelopes.clone())?,
        };

        for envelope in &envelopes {
            self.apply_committed_projection_envelope(envelope)?;
            replay_state.applied_event_count = replay_state.applied_event_count.saturating_add(1);
        }

        Ok(positions)
    }
}

fn tracks_social_projection_event(envelope: &CommitEnvelope) -> bool {
    matches!(
        envelope.event_type.as_str(),
        "friendship.activated" | "friendship.removed" | "direct_chat.bound"
    )
}

impl AppState {
    fn refresh_projection_state_from_runtime_dir(&self) -> Result<(), ApiError> {
        let Some(runtime_dir) = self.runtime_dir.as_ref() else {
            return Ok(());
        };
        let journal_path = runtime_dir
            .as_path()
            .join("state")
            .join("commit-journal.json");
        if !journal_path.exists() {
            return Ok(());
        }

        let recorded =
            read_commit_journal_file(journal_path.as_path()).map_err(|error| ApiError {
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "projection_authority_refresh_unavailable",
                message: format!(
                    "failed to read shared projection commit journal {}: {error:?}",
                    journal_path.display()
                ),
            })?;
        let mut replay_state = self
            .projection_replay_state
            .lock()
            .expect("projection replay authority state should not be poisoned");
        if recorded.len() == replay_state.applied_event_count {
            return Ok(());
        }

        if recorded.len() < replay_state.applied_event_count {
            self.projection_service.reset_for_recovery();
            self.conversation_runtime.reset_for_recovery();
            runtime_dir::apply_projection_journal_envelopes(
                recorded.as_slice(),
                self.projection_service.as_ref(),
                self.conversation_runtime.as_ref(),
                "local-minimal authority rebuild",
            )
            .map_err(|error| ApiError {
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "projection_authority_refresh_failed",
                message: error,
            })?;
            replay_state.applied_event_count = recorded.len();
            return Ok(());
        }

        runtime_dir::apply_projection_journal_envelopes(
            &recorded[replay_state.applied_event_count..],
            self.projection_service.as_ref(),
            self.conversation_runtime.as_ref(),
            "local-minimal authority tail replay",
        )
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "projection_authority_refresh_failed",
            message: error,
        })?;
        replay_state.applied_event_count = recorded.len();
        Ok(())
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
    let mut parts = Vec::new();
    let bytes = scope.as_bytes();
    let mut offset = 0;

    while offset < scope.len() {
        let len_start = offset;
        while offset < scope.len() && bytes[offset] != b'#' {
            if !bytes[offset].is_ascii_digit() {
                return None;
            }
            offset += 1;
        }
        if offset == len_start || offset >= scope.len() {
            return None;
        }
        let segment_len = scope[len_start..offset].parse::<usize>().ok()?;
        offset += 1;
        let segment_end = offset.checked_add(segment_len)?;
        if segment_end > scope.len() || !scope.is_char_boundary(segment_end) {
            return None;
        }
        parts.push(&scope[offset..segment_end]);
        offset = segment_end;
    }

    match parts.as_slice() {
        [tenant_id, conversation_id] => Some((*tenant_id, *conversation_id)),
        _ => None,
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
struct InboxResponse {
    items: Vec<ConversationInboxEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactsResponse {
    items: Vec<ContactView>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubmitFriendRequestAppRequest {
    target_user_id: String,
    request_message: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SocialFriendRequestListDirectionQuery {
    Incoming,
    Outgoing,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SocialFriendRequestListStatusQuery {
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
struct ListFriendRequestsAppQuery {
    direction: SocialFriendRequestListDirectionQuery,
    #[serde(default)]
    status: SocialFriendRequestListStatusQuery,
    limit: Option<usize>,
    cursor: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialFriendRequestMutationResponse {
    friend_request: FriendRequest,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialFriendRequestListResponse {
    items: Vec<FriendRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialFriendRequestAcceptanceResponse {
    friend_request: FriendRequest,
    friendship: Friendship,
    direct_chat: DirectChat,
    conversation: CreateConversationResult,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialFriendshipMutationResponse {
    friendship: Friendship,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MemberDirectoryResponse {
    items: Vec<ConversationMemberDirectoryEntry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PinnedMessagesResponse {
    items: Vec<MessageInteractionSummaryView>,
}

type DeviceSyncFeedResponse = projection_service::DeviceSyncFeedWindowView;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RegisterDeviceRequest {
    device_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SyncFeedQuery {
    after_seq: Option<u64>,
    limit: Option<usize>,
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

    fn unauthorized(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
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

    fn service_unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
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

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
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
            conversation_runtime::RuntimeError::PayloadTooLarge(message) => Self {
                status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                code: "payload_too_large",
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
            "payload_too_large" => axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            "limit_invalid" => axum::http::StatusCode::BAD_REQUEST,
            "conversation_archived" | "conversation_blocked" => axum::http::StatusCode::FORBIDDEN,
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

pub fn resolve_app_openapi_schema_source_path() -> PathBuf {
    env::var(APP_OPENAPI_SCHEMA_PATH_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../sdks/sdkwork-im-sdk/openapi/craw-chat-app.openapi.yaml")
        })
}

pub fn load_app_openapi_schema_yaml() -> String {
    let source_path = resolve_app_openapi_schema_source_path();
    match fs::read_to_string(&source_path) {
        Ok(contents) if !contents.trim().is_empty() => contents,
        Ok(_) => APP_OPENAPI_SCHEMA_EMBEDDED_YAML.to_string(),
        Err(_) => APP_OPENAPI_SCHEMA_EMBEDDED_YAML.to_string(),
    }
}

pub fn resolve_public_browser_origins() -> Vec<String> {
    std::env::var(PUBLIC_BROWSER_ORIGINS_ENV)
        .map(|value| parse_public_browser_origins(value.as_str()))
        .unwrap_or_else(|_| {
            Ok(DEFAULT_PUBLIC_BROWSER_ORIGINS
                .iter()
                .map(|origin| origin.to_string())
                .collect())
        })
        .unwrap_or_else(|error| panic!("{error}"))
}

fn parse_public_browser_origins(value: &str) -> Result<Vec<String>, String> {
    let mut normalized = BTreeSet::new();
    let mut invalid = Vec::new();

    for candidate in value.split([',', ';', '\n', '\r']) {
        let candidate = candidate.trim();
        if candidate.is_empty() {
            continue;
        }

        match normalize_public_browser_origin(candidate) {
            Ok(origin) => {
                normalized.insert(origin);
            }
            Err(error) => invalid.push(error),
        }
    }

    if !invalid.is_empty() {
        return Err(format!(
            "{PUBLIC_BROWSER_ORIGINS_ENV} contains invalid origins: {}",
            invalid.join("; ")
        ));
    }

    if normalized.is_empty() {
        return Err(format!(
            "{PUBLIC_BROWSER_ORIGINS_ENV} must define at least one origin when set"
        ));
    }

    Ok(normalized.into_iter().collect())
}

fn normalize_public_browser_origin(value: &str) -> Result<String, String> {
    let uri = value
        .parse::<axum::http::Uri>()
        .map_err(|_| format!("`{value}` is not a valid origin URI"))?;
    let scheme = uri
        .scheme_str()
        .ok_or_else(|| format!("`{value}` is missing a URI scheme"))?;
    let authority = uri
        .authority()
        .ok_or_else(|| format!("`{value}` is missing a URI authority"))?;
    let path = uri.path();

    if path != "/" && !path.is_empty() {
        return Err(format!("`{value}` must not include a path"));
    }

    if uri.query().is_some() {
        return Err(format!("`{value}` must not include a query string"));
    }

    Ok(format!(
        "{}://{}",
        scheme.to_ascii_lowercase(),
        authority.as_str().to_ascii_lowercase()
    ))
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    let user_center_config = user_center::resolve_effective_user_center_runtime_config();
    let user_center_login_path = user_center::login_path(&user_center_config);
    let user_center_refresh_path = user_center::refresh_path(&user_center_config);
    let user_center_health_path = user_center::health_path(&user_center_config);
    let path = request.uri().path();
    let method = request.method();
    let allows_public_route = matches!(
        path,
        "/healthz"
            | "/readyz"
            | APP_OPENAPI_SCHEMA_PATH
            | "/api/v1/auth/login"
            | "/api/v1/auth/refresh"
            | "/api/v1/portal/home"
            | "/api/v1/portal/auth"
    ) || path == user_center_login_path
        || path == user_center_refresh_path
        || path == user_center_health_path;

    if method == axum::http::Method::OPTIONS {
        return next.run(request).await;
    }

    if allows_public_route
        && (method == axum::http::Method::GET || method == axum::http::Method::POST)
    {
        return next.run(request).await;
    }

    match resolve_public_bearer_auth_context(request.headers()) {
        Ok(_) => next.run(request).await,
        Err(error) => ApiError::from(error).into_response(),
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

async fn export_app_openapi_schema() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "application/yaml; charset=utf-8")],
        load_app_openapi_schema_yaml(),
    )
}

fn resolve_auth_context(headers: &HeaderMap) -> Result<AuthContext, ApiError> {
    user_center::resolve_auth_context(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn invalid_message_posted_envelope(event_id: &str) -> CommitEnvelope {
        CommitEnvelope::minimal(
            event_id,
            "t_demo",
            "message.posted",
            "conversation",
            "c_projection_failure",
            1,
        )
        .with_payload("message.posted.v1", "{ invalid json")
    }

    fn valid_conversation_created_envelope(event_id: &str) -> CommitEnvelope {
        CommitEnvelope::minimal(
            event_id,
            "t_demo",
            "conversation.created",
            "conversation",
            "c_projection_failure",
            1,
        )
        .with_payload(
            "conversation.created.v1",
            r#"{
                "conversationId":"c_projection_failure",
                "conversationType":"group",
                "creatorId":"u_demo",
                "source":null,
                "target":null,
                "handoff":null
            }"#,
        )
    }

    #[test]
    fn test_projection_journal_append_returns_error_and_keeps_replay_cursor_when_projection_fails()
    {
        let projection_service = Arc::new(TimelineProjectionService::default());
        let journal = ProjectionJournal::new_memory(projection_service);

        let result = journal.append(invalid_message_posted_envelope("evt_projection_append_bad"));

        assert!(
            matches!(result, Err(ContractError::Unavailable(ref message)) if message.contains("projection apply failed")),
            "projection apply failure should be surfaced as journal unavailable: {result:?}"
        );
        assert_eq!(
            journal
                .replay_state()
                .lock()
                .expect("projection replay state should lock")
                .applied_event_count,
            0,
            "failed projection must not advance the applied projection cursor"
        );
    }

    #[test]
    fn test_projection_journal_append_batch_returns_error_and_keeps_replay_cursor_when_projection_fails()
     {
        let projection_service = Arc::new(TimelineProjectionService::default());
        let journal = ProjectionJournal::new_memory(projection_service);

        let result = journal.append_batch(vec![invalid_message_posted_envelope(
            "evt_projection_batch_bad",
        )]);

        assert!(
            matches!(result, Err(ContractError::Unavailable(ref message)) if message.contains("projection apply failed")),
            "batch projection apply failure should be surfaced as journal unavailable: {result:?}"
        );
        assert_eq!(
            journal
                .replay_state()
                .lock()
                .expect("projection replay state should lock")
                .applied_event_count,
            0,
            "failed batch projection must not advance the applied projection cursor"
        );
    }

    #[test]
    fn test_projection_journal_append_batch_advances_replay_cursor_for_successful_prefix_only() {
        let projection_service = Arc::new(TimelineProjectionService::default());
        let journal = ProjectionJournal::new_memory(projection_service);

        let result = journal.append_batch(vec![
            valid_conversation_created_envelope("evt_projection_batch_good_prefix"),
            invalid_message_posted_envelope("evt_projection_batch_bad_suffix"),
        ]);

        assert!(
            matches!(result, Err(ContractError::Unavailable(ref message)) if message.contains("projection apply failed")),
            "batch projection suffix failure should be surfaced as journal unavailable: {result:?}"
        );
        assert_eq!(
            journal
                .replay_state()
                .lock()
                .expect("projection replay state should lock")
                .applied_event_count,
            1,
            "batch projection cursor should record the committed prefix that was already projected"
        );
    }

    #[test]
    fn test_parse_projection_snapshot_scope_accepts_segment_safe_ordering_key() {
        assert_eq!(
            parse_projection_snapshot_scope(CommitEnvelope::ordering_key("tenant:a", "b").as_str()),
            Some(("tenant:a", "b"))
        );
        assert_eq!(
            parse_projection_snapshot_scope(CommitEnvelope::ordering_key("tenant", "a:b").as_str()),
            Some(("tenant", "a:b"))
        );
    }

    #[test]
    fn test_parse_projection_snapshot_scope_rejects_ambiguous_legacy_scope_key() {
        assert_eq!(parse_projection_snapshot_scope("tenant:a:b"), None);
    }
}
