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
use automation_service::AutomationRuntime;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Extension, Path, Query, State};
use axum::http::HeaderMap;
use axum::http::Request;
use axum::http::header::CONTENT_TYPE;
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::response::Response;
use axum::{
    Json, Router,
    routing::{any, delete, get, patch, post},
};
use control_plane_api::SocialControlQuery;
use conversation_runtime::{
    AddMessageReactionCommand, AgentHandoffStateView, BindDirectChatConversationCommand,
    ChangeConversationMemberRoleResult, ConversationRuntime, CreateConversationResult,
    EditMessageCommand, MessageMutationResult, MessagePinMutationResult,
    MessageReactionMutationResult, PinMessageCommand, PostMessageCommand, PostMessageResult,
    PublishSystemChannelMessageCommand, RecallMessageCommand, RemoveMessageReactionCommand,
    TransferConversationOwnerResult, UnpinMessageCommand,
};
use im_adapters_local_disk::{
    FileAutomationExecutionStore, FileCommitJournal, FileMetadataStore, FileNotificationTaskStore,
    FilePresenceStateStore, FileRealtimeCheckpointStore, FileRealtimeDisconnectFenceStore,
    FileRealtimeEventWindowStore, FileRealtimeSubscriptionStore, FileStreamStateStore,
    FileTimelineProjectionStore, read_commit_journal_file,
    validate_automation_execution_store_file, validate_commit_journal_file,
    validate_notification_task_store_file, validate_presence_state_store_file,
    validate_realtime_checkpoint_store_file, validate_realtime_disconnect_fence_store_file,
    validate_realtime_event_window_store_file, validate_realtime_subscription_store_file,
    validate_stream_state_store_file,
};
use im_adapters_local_memory::{MemoryCommitJournal, MemoryRealtimeCheckpointStore};
use im_app_context::{AppContext, AppContextError};
use im_domain_core::conversation::{
    ConversationInboxEntry, ConversationMember, ConversationReadCursorView, MembershipRole,
};
use im_domain_core::message::{ContentPart, MessageBody, MessageType, SignalPart};
use im_domain_core::presence::PresenceSnapshotView;
use im_domain_core::realtime::RealtimeSubscription;
use im_domain_core::social::{DirectChat, FriendRequest, Friendship};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    CommitJournal, CommitPosition, ContractError, EffectiveProviderBinding, MetadataStore,
    PROVIDER_REGISTRY_INTERFACE_VERSION, PrincipalProfileProvider, ProviderDomain,
    ProviderPluginDescriptor, ProviderRegistry, RealtimeCheckpointRecord,
    RealtimeDisconnectFenceRecord, RealtimeEventWindowRecord, RealtimeSubscriptionRecord,
    StaticProviderRegistry, StreamStateRecord,
};
use notification_service::NotificationRuntime;
use ops_service::{
    ClusterView, DiagnosticBundle, LagItem, LagView, OpsHealthResponse, OpsRuntime,
    ProviderBindingDriftView, ProviderBindingItemView, ProviderBindingSnapshotView,
    ProviderBindingsView, RouteOwnershipView, RuntimeDirInspectionItem, RuntimeDirInspectionView,
};
use projection_service::{
    ContactView, ConversationMemberDirectoryEntry, MessageInteractionSummaryView,
    NotificationRecipientView, ProjectionAccessError, TimelineProjectionService,
};
use sdkwork_rtc_app_context::AppContext as RtcAppContext;
use sdkwork_rtc_core::{
    ProviderHealthSnapshot, RtcCallbackEvent, RtcCallbackRequest, RtcStateRecord,
};
use sdkwork_rtc_signaling_service::{
    CreateRtcSessionRequest, InviteRtcSessionRequest, IssueRtcParticipantCredentialRequest,
    PostRtcSignalRequest, RtcRuntime, RtcSessionMutationResponse, UpdateRtcSessionRequest,
    rtc_create_request_key, rtc_session_action_request_key,
};
use sdkwork_rtc_state_store::{FileRtcStateStore, validate_rtc_state_store_file};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use session_gateway::{
    AckRealtimeEventsRequest, ListRealtimeEventsQuery, PresenceRuntime, PresenceRuntimeError,
    RealtimeClusterBridge, RealtimeClusterError, RealtimeDeliveryRuntime, RealtimePlaneAssembly,
    RealtimeRuntimeError, RealtimeScopeAccessPolicy, SyncRealtimeSubscriptionsRequest,
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
use tokio::sync::Semaphore;

mod access;
mod aiot_bridge;
mod build;
mod client_route_registration;
mod commercial_readiness;
mod conversation;
mod effects;
mod handoff;
mod membership;
mod message;
mod platform;
mod presence_routes;
mod principal_profile;
mod projection;
mod realtime_policy;
mod rtc;
mod runtime_dir;
mod side_effect_outbox;
mod social;
mod stream;

use self::client_route_registration::LocalNodeClientRouteRegistration;

fn rtc_app_context_from_auth(auth: &AppContext) -> RtcAppContext {
    RtcAppContext {
        tenant_id: auth.tenant_id.clone(),
        organization_id: auth.organization_id.clone(),
        user_id: auth.user_id.clone(),
        session_id: auth.session_id.clone(),
        app_id: auth.app_id.clone(),
        environment: auth.environment.clone(),
        deployment_mode: auth.deployment_mode.clone(),
        auth_level: auth.auth_level.clone(),
        data_scope: auth.data_scope.clone(),
        permission_scope: auth.permission_scope.clone(),
        actor_id: auth.actor_id.clone(),
        actor_kind: auth.actor_kind.clone(),
        device_id: auth.device_id.clone(),
    }
}

pub use build::{
    build_app_with_dependencies, build_app_with_dependencies_and_runtime,
    build_app_with_dependencies_and_runtime_dir,
    build_app_with_dependencies_realtime_and_notification_runtime, build_default_app,
    build_default_app_with_principal_profile_provider, build_default_app_with_runtime_dir,
    build_default_app_with_runtime_dir_and_principal_profile_provider, build_public_app,
    build_public_app_with_runtime_dir, try_build_public_app,
};
pub use commercial_readiness::{
    CRAW_CHAT_COMMERCIAL_EVIDENCE_ROOT_ENV, CRAW_CHAT_DATABASE_URL_ENV,
    CRAW_CHAT_POSTGRES_CONFIG_ENV, CRAW_CHAT_RUNTIME_PROFILE_ENV, CRAW_CHAT_STORAGE_PROVIDER_ENV,
    CommercialReadinessBlocker, CommercialReadinessInputs, CommercialReadinessReport,
    CommercialReadinessStatus, CommercialStep11Evidence, commercial_readiness_required_for_profile,
    commercial_readiness_required_from_env, evaluate_commercial_readiness,
    evaluate_commercial_readiness_from_env, evaluate_commercial_readiness_from_workspace,
    format_commercial_readiness_blocked_error, format_commercial_readiness_report,
    load_commercial_step11_evidence,
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
    control_plane_app: Router,
    social_query: Arc<SocialControlQuery>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    conversation_runtime: Arc<ConversationRuntime<ProjectionJournal>>,
    principal_profile_provider: Arc<dyn PrincipalProfileProvider>,
    projection_service: Arc<TimelineProjectionService>,
    presence_runtime: Arc<PresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    client_route_registration: LocalNodeClientRouteRegistration,
    aiot_app_api_server: Arc<sdkwork_aiot_http_api::AiotApiServer>,
    aiot_backend_api_server: Arc<sdkwork_aiot_http_api::AiotApiServer>,
    streaming_runtime: Arc<StreamingRuntime>,
    rtc_runtime: Arc<RtcRuntime>,
    notification_runtime: Arc<NotificationRuntime>,
    automation_runtime: Arc<AutomationRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    ops_runtime: Arc<OpsRuntime>,
    message_side_effect_outbox: Arc<dyn side_effect_outbox::MessageSideEffectOutboxStore>,
    conversation_preferences: Arc<std::sync::Mutex<BTreeMap<String, ConversationPreferencesView>>>,
    conversation_profiles: Arc<std::sync::Mutex<BTreeMap<String, ConversationProfileView>>>,
    contact_preferences: Arc<std::sync::Mutex<BTreeMap<String, ContactPreferencesView>>>,
    contact_tags: Arc<std::sync::Mutex<BTreeMap<String, ContactTagView>>>,
    contact_recommendations: Arc<std::sync::Mutex<BTreeMap<String, ContactRecommendationView>>>,
    message_visibility: Arc<std::sync::Mutex<BTreeMap<String, MessageVisibilityMutationResult>>>,
    message_favorites: Arc<std::sync::Mutex<BTreeMap<String, MessageFavoriteView>>>,
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
    restored_client_route_sync: bool,
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
const LOCAL_MINIMAL_NODE_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "CRAW_CHAT_LOCAL_MINIMAL_NODE_MAX_IN_FLIGHT_REQUESTS";
const LOCAL_MINIMAL_NODE_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const LOCAL_MINIMAL_NODE_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 50_000;
const LOCAL_MINIMAL_NODE_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "CRAW_CHAT_LOCAL_MINIMAL_NODE_MAX_REQUEST_BODY_BYTES";
const LOCAL_MINIMAL_NODE_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const LOCAL_MINIMAL_NODE_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const LOCAL_MINIMAL_NODE_REQUIRE_DUAL_TOKEN_HEADERS_ENV: &str =
    "CRAW_CHAT_LOCAL_MINIMAL_NODE_REQUIRE_DUAL_TOKEN_HEADERS";
const IM_OPENAPI_SCHEMA_PATH_ENV: &str = "CRAW_CHAT_IM_OPENAPI_SCHEMA_PATH";
const APP_API_OPENAPI_SCHEMA_PATH_ENV: &str = "CRAW_CHAT_APP_API_OPENAPI_SCHEMA_PATH";
const BACKEND_API_OPENAPI_SCHEMA_PATH_ENV: &str = "CRAW_CHAT_BACKEND_API_OPENAPI_SCHEMA_PATH";
const IM_OPENAPI_SCHEMA_PATH: &str = "/im/v3/openapi.json";
const APP_API_OPENAPI_SCHEMA_PATH: &str = "/app/v3/openapi.json";
const BACKEND_API_OPENAPI_SCHEMA_PATH: &str = "/backend/v3/openapi.json";

#[derive(Clone)]
pub(crate) struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
    require_dual_token_headers: bool,
}
const IM_OPENAPI_SCHEMA_EMBEDDED_YAML: &str =
    include_str!("../../../sdks/sdkwork-im-sdk/openapi/craw-chat-im.openapi.yaml");
const APP_API_OPENAPI_SCHEMA_EMBEDDED_YAML: &str =
    include_str!("../../../sdks/sdkwork-im-app-sdk/openapi/craw-chat-app-api.openapi.yaml");
const BACKEND_API_OPENAPI_SCHEMA_EMBEDDED_YAML: &str =
    include_str!("../../../sdks/sdkwork-im-backend-sdk/openapi/craw-chat-backend-api.openapi.yaml");
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
    fn require_client_route_key_binding(&self, auth: &AppContext) -> Result<(), ApiError> {
        self.client_route_registration
            .ensure_client_route_key(self, auth)
    }

    #[rustfmt::skip]
    fn prepare_active_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<projection_service::RegisteredClientRouteView, ApiError> {
        self.client_route_registration
            .prepare_active_client_route(self, auth, device_id, connection_kind)
    }

    #[rustfmt::skip]
    fn prepare_resumed_client_route(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<projection_service::RegisteredClientRouteView, ApiError> {
        self.client_route_registration
            .bind_client_route_key(self, auth, device_id, connection_kind, true)
    }

    fn provider_binding_snapshots(&self) -> Vec<ProviderBindingSnapshotView> {
        let registry = StaticProviderRegistry::platform_default()
            .with_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine");
        let mut bindings = BTreeMap::new();

        for domain in ProviderDomain::ALL {
            if let Some(binding) = registry.effective_binding(domain, None) {
                bindings.insert(domain, binding);
            }
        }

        if let Ok(binding) = self.rtc_runtime.provider_binding(None) {
            bindings.insert(ProviderDomain::Rtc, provider_binding_from_rtc(binding));
        }
        bindings.insert(
            ProviderDomain::PrincipalProfile,
            binding_from_descriptor(&registry, self.principal_profile_provider.descriptor()),
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

fn provider_binding_from_rtc(
    binding: sdkwork_rtc_core::EffectiveProviderBinding,
) -> EffectiveProviderBinding {
    EffectiveProviderBinding {
        domain: ProviderDomain::Rtc,
        default_plugin_id: binding.default_plugin_id,
        selected_plugin_id: binding.selected_plugin_id,
        selection_source: binding.selection_source,
        tenant_override_allowed: binding.tenant_override_allowed,
    }
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
        let restored_client_route_sync =
            snapshot_stores.restore_client_route_sync_snapshot(projection_service);

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
            restored_client_route_sync,
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

    fn persist_client_route_sync_snapshot(&self, projection_service: &TimelineProjectionService) {
        let _ =
            projection_service.persist_client_route_sync_snapshot(&self.metadata, &self.timeline);
    }

    fn restore_client_route_sync_snapshot(
        &self,
        projection_service: &TimelineProjectionService,
    ) -> bool {
        projection_service
            .restore_client_route_sync_snapshot(&self.metadata, &self.timeline)
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

fn encode_local_key_segments<'a>(segments: impl IntoIterator<Item = &'a str>) -> String {
    let mut encoded = String::new();
    for segment in segments {
        encoded.push_str(segment.len().to_string().as_str());
        encoded.push('#');
        encoded.push_str(segment);
    }
    encoded
}

fn conversation_preferences_key(
    tenant_id: &str,
    conversation_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    encode_local_key_segments([tenant_id, conversation_id, principal_kind, principal_id])
}

fn conversation_profile_key(tenant_id: &str, conversation_id: &str) -> String {
    encode_local_key_segments([tenant_id, conversation_id])
}

fn contact_preferences_key(tenant_id: &str, owner_user_id: &str, target_user_id: &str) -> String {
    encode_local_key_segments([tenant_id, owner_user_id, target_user_id])
}

fn contact_tag_key(tenant_id: &str, owner_user_id: &str, tag_id: &str) -> String {
    encode_local_key_segments([tenant_id, owner_user_id, tag_id])
}

fn contact_recommendation_key(
    tenant_id: &str,
    owner_user_id: &str,
    target_user_id: &str,
    recommendation_id: &str,
) -> String {
    encode_local_key_segments([tenant_id, owner_user_id, target_user_id, recommendation_id])
}

fn message_visibility_key(
    tenant_id: &str,
    message_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    encode_local_key_segments([tenant_id, message_id, principal_kind, principal_id])
}

fn message_favorite_key(
    tenant_id: &str,
    favorite_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    encode_local_key_segments([tenant_id, principal_kind, principal_id, favorite_id])
}

fn message_favorite_id(
    tenant_id: &str,
    message_id: &str,
    principal_kind: &str,
    principal_id: &str,
) -> String {
    let digest = Sha256::digest(
        encode_local_key_segments([tenant_id, message_id, principal_kind, principal_id]).as_bytes(),
    );
    format!("fav_{digest:x}").chars().take(40).collect()
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
    reply_to: Option<im_domain_core::message::MessageReplyReference>,
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
    reply_to: Option<im_domain_core::message::MessageReplyReference>,
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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum MessageFavoriteType {
    Link,
    Image,
    File,
    Chat,
}

impl MessageFavoriteType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Link => "link",
            Self::Image => "image",
            Self::File => "file",
            Self::Chat => "chat",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FavoriteMessageRequest {
    conversation_id: String,
    favorite_type: MessageFavoriteType,
    title: String,
    content_preview: String,
    source_display_name: String,
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
    next_cursor: Option<String>,
    has_more: bool,
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
    next_cursor: Option<String>,
    has_more: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactPreferencesView {
    tenant_id: String,
    owner_user_id: String,
    target_user_id: String,
    is_starred: bool,
    remark: String,
    is_blocked: bool,
    updated_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactTagView {
    tenant_id: String,
    owner_user_id: String,
    tag_id: String,
    name: String,
    color: String,
    count: u32,
    bg: String,
    border: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactTagsResponse {
    items: Vec<ContactTagView>,
    next_cursor: Option<String>,
    has_more: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateContactTagRequest {
    name: String,
    color: String,
    count: Option<u32>,
    bg: Option<String>,
    border: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateContactTagRequest {
    name: Option<String>,
    color: Option<String>,
    count: Option<u32>,
    bg: Option<String>,
    border: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteContactTagResponse {
    tag_id: String,
    deleted: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactRecommendationView {
    tenant_id: String,
    owner_user_id: String,
    target_user_id: String,
    recommendation_id: String,
    target_conversation_id: Option<String>,
    created_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateContactRecommendationRequest {
    target_conversation_id: Option<String>,
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

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialUserSearchResult {
    tenant_id: String,
    user_id: String,
    chat_id: String,
    display_name: String,
    relationship_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    metadata: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SocialUserSearchQuery {
    q: Option<String>,
    limit: Option<usize>,
    cursor: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SocialUserSearchResponse {
    items: Vec<SocialUserSearchResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    next_cursor: Option<String>,
    has_more: bool,
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

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct MessageFavoritesQuery {
    limit: Option<usize>,
    cursor: Option<String>,
    favorite_type: Option<MessageFavoriteType>,
    q: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageFavoriteView {
    tenant_id: String,
    principal_kind: String,
    principal_id: String,
    favorite_id: String,
    favorite_type: String,
    conversation_id: String,
    message_id: String,
    message_seq: u64,
    title: String,
    content_preview: String,
    source_display_name: String,
    favorited_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FavoriteMessagesResponse {
    items: Vec<MessageFavoriteView>,
    next_cursor: Option<String>,
    has_more: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteMessageFavoriteResponse {
    favorite_id: String,
    deleted: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageVisibilityMutationResult {
    tenant_id: String,
    conversation_id: String,
    message_id: String,
    message_seq: u64,
    principal_kind: String,
    principal_id: String,
    is_deleted: bool,
    updated_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationPreferencesView {
    tenant_id: String,
    conversation_id: String,
    principal_kind: String,
    principal_id: String,
    is_pinned: bool,
    is_muted: bool,
    is_marked_unread: bool,
    is_hidden: bool,
    updated_at: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConversationProfileView {
    tenant_id: String,
    conversation_id: String,
    display_name: String,
    avatar_url: String,
    notice: String,
    updated_at: String,
    updated_by_principal_kind: Option<String>,
    updated_by_principal_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PresenceHeartbeatRequest {
    device_id: Option<String>,
    last_seen_sync_seq: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateReadCursorRequest {
    read_seq: u64,
    last_read_message_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateConversationPreferencesRequest {
    is_pinned: Option<bool>,
    is_muted: Option<bool>,
    is_marked_unread: Option<bool>,
    is_hidden: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateContactPreferencesRequest {
    is_starred: Option<bool>,
    remark: Option<String>,
    is_blocked: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateConversationProfileRequest {
    display_name: Option<String>,
    avatar_url: Option<String>,
    notice: Option<String>,
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

    fn service_unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code,
            message: message.into(),
        }
    }

    fn unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
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
            conversation_runtime::RuntimeError::AgentIdInvalid(message) => Self {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "agent_id_invalid",
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

impl From<AppContextError> for ApiError {
    fn from(value: AppContextError) -> Self {
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

impl From<sdkwork_rtc_signaling_service::RtcError> for ApiError {
    fn from(value: sdkwork_rtc_signaling_service::RtcError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let detail = self.message;
        let message = detail.clone();
        let title = status.canonical_reason().unwrap_or("Unknown Error");
        (
            status,
            [(CONTENT_TYPE, "application/problem+json; charset=utf-8")],
            Json(serde_json::json!({
                "type": "about:blank",
                "title": title,
                "status": status.as_u16(),
                "detail": detail,
                "code": self.code,
                "message": message
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

pub fn resolve_commercial_evidence_root() -> PathBuf {
    env::var(CRAW_CHAT_COMMERCIAL_EVIDENCE_ROOT_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

fn resolve_openapi_schema_source_path(env_name: &str, default_relative_path: &str) -> PathBuf {
    env::var(env_name)
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(default_relative_path))
}

fn load_openapi_schema_yaml(
    env_name: &str,
    default_relative_path: &str,
    embedded_yaml: &str,
) -> String {
    let source_path = resolve_openapi_schema_source_path(env_name, default_relative_path);
    match fs::read_to_string(&source_path) {
        Ok(contents) if !contents.trim().is_empty() => contents,
        Ok(_) => embedded_yaml.to_string(),
        Err(_) => embedded_yaml.to_string(),
    }
}

fn load_openapi_schema_json(
    label: &str,
    env_name: &str,
    default_relative_path: &str,
    embedded_yaml: &str,
) -> Result<Value, ApiError> {
    serde_yaml::from_str(
        load_openapi_schema_yaml(env_name, default_relative_path, embedded_yaml).as_str(),
    )
    .map_err(|error| {
        ApiError::unavailable(
            "openapi_schema_invalid",
            format!("{label} OpenAPI schema source is not a valid OpenAPI document: {error}"),
        )
    })
}

pub fn resolve_im_openapi_schema_source_path() -> PathBuf {
    resolve_openapi_schema_source_path(
        IM_OPENAPI_SCHEMA_PATH_ENV,
        "../../sdks/sdkwork-im-sdk/openapi/craw-chat-im.openapi.yaml",
    )
}

pub fn resolve_app_api_openapi_schema_source_path() -> PathBuf {
    resolve_openapi_schema_source_path(
        APP_API_OPENAPI_SCHEMA_PATH_ENV,
        "../../sdks/sdkwork-im-app-sdk/openapi/craw-chat-app-api.openapi.yaml",
    )
}

pub fn resolve_backend_api_openapi_schema_source_path() -> PathBuf {
    resolve_openapi_schema_source_path(
        BACKEND_API_OPENAPI_SCHEMA_PATH_ENV,
        "../../sdks/sdkwork-im-backend-sdk/openapi/craw-chat-backend-api.openapi.yaml",
    )
}

fn load_im_openapi_schema_json() -> Result<Value, ApiError> {
    load_openapi_schema_json(
        "im-open-api",
        IM_OPENAPI_SCHEMA_PATH_ENV,
        "../../sdks/sdkwork-im-sdk/openapi/craw-chat-im.openapi.yaml",
        IM_OPENAPI_SCHEMA_EMBEDDED_YAML,
    )
}

fn load_app_api_openapi_schema_json() -> Result<Value, ApiError> {
    let mut schema = load_openapi_schema_json(
        "im-app-api",
        APP_API_OPENAPI_SCHEMA_PATH_ENV,
        "../../sdks/sdkwork-im-app-sdk/openapi/craw-chat-app-api.openapi.yaml",
        APP_API_OPENAPI_SCHEMA_EMBEDDED_YAML,
    )?;
    if env::var(APP_API_OPENAPI_SCHEMA_PATH_ENV).is_err() {
        compose_app_api_dependency_openapi_paths(&mut schema);
    }
    Ok(schema)
}

fn load_backend_api_openapi_schema_json() -> Result<Value, ApiError> {
    load_openapi_schema_json(
        "im-backend-api",
        BACKEND_API_OPENAPI_SCHEMA_PATH_ENV,
        "../../sdks/sdkwork-im-backend-sdk/openapi/craw-chat-backend-api.openapi.yaml",
        BACKEND_API_OPENAPI_SCHEMA_EMBEDDED_YAML,
    )
}

fn compose_app_api_dependency_openapi_paths(schema: &mut Value) {
    ensure_openapi_schema_component(schema, "AppbaseApiResult");
    let Some(paths) = schema.get_mut("paths").and_then(Value::as_object_mut) else {
        return;
    };

    for (path, method, operation_id, summary) in [
        (
            "/app/v3/api/auth/registrations",
            "post",
            "appbase.auth.registrations.create",
            "Create an appbase registration",
        ),
        (
            "/app/v3/api/auth/sessions",
            "post",
            "appbase.auth.sessions.create",
            "Create an appbase session",
        ),
        (
            "/app/v3/api/auth/sessions/current",
            "get",
            "appbase.auth.sessions.current.retrieve",
            "Get the current appbase session",
        ),
        (
            "/app/v3/api/auth/sessions/current",
            "patch",
            "appbase.auth.sessions.current.update",
            "Update the current appbase session",
        ),
        (
            "/app/v3/api/auth/sessions/current",
            "delete",
            "appbase.auth.sessions.current.delete",
            "Delete the current appbase session",
        ),
        (
            "/app/v3/api/auth/sessions/refresh",
            "post",
            "appbase.auth.sessions.refresh",
            "Refresh an appbase session",
        ),
        (
            "/app/v3/api/auth/verification_codes",
            "post",
            "appbase.auth.verificationCodes.create",
            "Create an appbase verification code",
        ),
        (
            "/app/v3/api/auth/verification_codes/verify",
            "post",
            "appbase.auth.verificationCodes.verify",
            "Verify an appbase verification code",
        ),
        (
            "/app/v3/api/iam/users/current",
            "get",
            "appbase.iam.users.current.retrieve",
            "Get the current IAM user",
        ),
        (
            "/app/v3/api/system/iam/runtime",
            "get",
            "appbase.system.iam.runtime.retrieve",
            "Get appbase IAM runtime metadata",
        ),
        (
            "/app/v3/api/system/iam/verification_policy",
            "get",
            "appbase.system.iam.verificationPolicy.retrieve",
            "Get appbase verification policy",
        ),
        (
            "/app/v3/api/open_platform/qr_auth/sessions",
            "post",
            "appbase.openPlatform.qrAuth.sessions.create",
            "Create a QR auth session",
        ),
        (
            "/app/v3/api/open_platform/qr_auth/sessions/{sessionKey}",
            "get",
            "appbase.openPlatform.qrAuth.sessions.retrieve",
            "Get a QR auth session",
        ),
        (
            "/app/v3/api/open_platform/qr_auth/sessions/{sessionKey}/scans",
            "post",
            "appbase.openPlatform.qrAuth.sessions.scans.create",
            "Scan a QR auth session",
        ),
        (
            "/app/v3/api/open_platform/qr_auth/sessions/{sessionKey}/passwords",
            "post",
            "appbase.openPlatform.qrAuth.sessions.passwords.create",
            "Approve a QR auth session with a password",
        ),
    ] {
        insert_app_api_dependency_operation(paths, path, method, operation_id, summary);
    }
}

fn ensure_openapi_schema_component(schema: &mut Value, name: &str) {
    if !schema.get("components").is_some_and(Value::is_object) {
        schema["components"] = serde_json::json!({});
    }
    let components = schema
        .get_mut("components")
        .and_then(Value::as_object_mut)
        .expect("components should be an object after initialization");
    let schemas = components
        .entry("schemas".to_owned())
        .or_insert_with(|| serde_json::json!({}));
    if !schemas.is_object() {
        *schemas = serde_json::json!({});
    }
    schemas
        .as_object_mut()
        .expect("schemas should be an object after initialization")
        .entry(name.to_owned())
        .or_insert_with(|| {
            serde_json::json!({
                "type": "object",
                "additionalProperties": true
            })
        });
}

fn insert_app_api_dependency_operation(
    paths: &mut serde_json::Map<String, Value>,
    path: &str,
    method: &str,
    operation_id: &str,
    summary: &str,
) {
    let path_item = paths
        .entry(path.to_owned())
        .or_insert_with(|| serde_json::json!({}));
    if !path_item.is_object() {
        *path_item = serde_json::json!({});
    }
    let operation = if path.contains("{sessionKey}") {
        serde_json::json!({
            "tags": ["appbase"],
            "operationId": operation_id,
            "summary": summary,
            "parameters": [
                {
                    "name": "sessionKey",
                    "in": "path",
                    "required": true,
                    "schema": {
                        "type": "string"
                    }
                }
            ],
            "responses": appbase_dependency_openapi_responses()
        })
    } else {
        serde_json::json!({
            "tags": ["appbase"],
            "operationId": operation_id,
            "summary": summary,
            "responses": appbase_dependency_openapi_responses()
        })
    };
    path_item
        .as_object_mut()
        .expect("path item should be an object after initialization")
        .entry(method.to_owned())
        .or_insert(operation);
}

fn appbase_dependency_openapi_responses() -> Value {
    serde_json::json!({
        "200": {
            "description": "Appbase dependency SDK response",
            "content": {
                "application/json": {
                    "schema": {
                        "$ref": "#/components/schemas/AppbaseApiResult"
                    }
                }
            }
        }
    })
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

pub(crate) async fn require_app_context_with_guardrails(
    State(guardrails): State<PublicAppGuardrails>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let path = request.uri().path();
    let method = request.method();
    let allows_public_route = matches!(
        path,
        "/healthz"
            | "/readyz"
            | IM_OPENAPI_SCHEMA_PATH
            | APP_API_OPENAPI_SCHEMA_PATH
            | BACKEND_API_OPENAPI_SCHEMA_PATH
            | "/im/v3/api/portal/home"
            | "/im/v3/api/portal/access"
    );

    if method == axum::http::Method::OPTIONS {
        return next.run(request).await;
    }

    if allows_public_route
        && (method == axum::http::Method::GET || method == axum::http::Method::POST)
    {
        return next.run(request).await;
    }

    let permit = match guardrails.request_gate.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            return ApiError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "http_overloaded",
                message: "server is at maximum in-flight request capacity, please retry later"
                    .to_owned(),
            }
            .into_response();
        }
    };

    if guardrails.require_dual_token_headers
        && let Err(error) = require_dual_token_headers(request.headers())
    {
        return error.into_response();
    }

    let auth = match resolve_request_app_context(None, request.headers()) {
        Ok(auth) => auth,
        Err(error) => return error.into_response(),
    };
    request.extensions_mut().insert(auth);
    let response = next.run(request).await;
    drop(permit);
    response
}

fn require_dual_token_headers(headers: &HeaderMap) -> Result<(), ApiError> {
    if !has_bearer_auth_token(headers) {
        return Err(ApiError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "auth_token_missing",
            message: "authorization header must provide a bearer token".to_owned(),
        });
    }
    if !has_access_token_header(headers) {
        return Err(ApiError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "access_token_missing",
            message: "access-token header is required".to_owned(),
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
    std::env::var(LOCAL_MINIMAL_NODE_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(LOCAL_MINIMAL_NODE_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(LOCAL_MINIMAL_NODE_MAX_IN_FLIGHT_REQUESTS_MAX)
}

pub(crate) fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(LOCAL_MINIMAL_NODE_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(LOCAL_MINIMAL_NODE_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(LOCAL_MINIMAL_NODE_MAX_REQUEST_BODY_BYTES_MAX)
}

fn resolve_require_dual_token_headers() -> bool {
    std::env::var(LOCAL_MINIMAL_NODE_REQUIRE_DUAL_TOKEN_HEADERS_ENV)
        .ok()
        .map(|value| parse_truthy_env_flag(Some(value)))
        .unwrap_or(false)
}

fn parse_truthy_env_flag(raw: Option<String>) -> bool {
    raw.as_deref().map(str::trim).is_some_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

pub(crate) fn build_public_app_guardrails() -> PublicAppGuardrails {
    PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
        require_dual_token_headers: resolve_require_dual_token_headers(),
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

async fn export_im_openapi_schema() -> Result<impl IntoResponse, ApiError> {
    let schema = load_im_openapi_schema_json()?;
    Ok((
        [(CONTENT_TYPE, "application/json; charset=utf-8")],
        Json(schema),
    ))
}

async fn export_app_api_openapi_schema() -> Result<impl IntoResponse, ApiError> {
    let schema = load_app_api_openapi_schema_json()?;
    Ok((
        [(CONTENT_TYPE, "application/json; charset=utf-8")],
        Json(schema),
    ))
}

async fn export_backend_api_openapi_schema() -> Result<impl IntoResponse, ApiError> {
    let schema = load_backend_api_openapi_schema_json()?;
    Ok((
        [(CONTENT_TYPE, "application/json; charset=utf-8")],
        Json(schema),
    ))
}

fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, ApiError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => im_app_context::resolve_app_context(headers).map_err(ApiError::from),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderMap, HeaderValue};
    use std::sync::{Mutex, OnceLock};

    struct ScopedEnvVar {
        name: &'static str,
        previous: Option<String>,
    }

    impl ScopedEnvVar {
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
            unsafe {
                match &self.previous {
                    Some(value) => std::env::set_var(self.name, value),
                    None => std::env::remove_var(self.name),
                }
            }
        }
    }

    fn env_test_lock<'a>() -> std::sync::MutexGuard<'a, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("env lock")
    }

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
        assert_eq!(error.status, axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(error.code, "access_token_missing");

        headers.insert("access-token", HeaderValue::from_static("access"));
        assert!(has_access_token_header(&headers));
        require_dual_token_headers(&headers).expect("dual token headers should pass");
    }

    #[test]
    fn dual_token_guardrail_defaults_to_app_context_projection() {
        let _guard = env_test_lock();
        let _env = ScopedEnvVar::remove(LOCAL_MINIMAL_NODE_REQUIRE_DUAL_TOKEN_HEADERS_ENV);

        assert!(
            !resolve_require_dual_token_headers(),
            "local-minimal public app should default to SDKWork AppContext projection without legacy bearer/access-token headers"
        );
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
