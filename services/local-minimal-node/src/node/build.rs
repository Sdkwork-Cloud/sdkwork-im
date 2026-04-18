use super::user_module::build_default_user_module_provider;
use super::*;
use control_plane_api::{
    SharedChannelLinkedMemberSyncRequest, SharedChannelLinkedMemberSyncTrigger,
};
use conversation_runtime::SyncSharedChannelLinkedMemberCommand;
use im_adapter_iot_access_local::LocalDeviceAccessProvider;
use im_adapter_iot_mqtt::MqttIotProtocolAdapter;
use im_adapters_local_disk::FileDeviceTwinStore;
use im_adapters_local_memory::MemoryDeviceTwinStore;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

#[derive(Clone)]
struct LocalMinimalSharedChannelLinkedMemberSyncTrigger {
    conversation_runtime: Arc<ConversationRuntime<ProjectionJournal>>,
}

impl SharedChannelLinkedMemberSyncTrigger for LocalMinimalSharedChannelLinkedMemberSyncTrigger {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String> {
        self.conversation_runtime
            .sync_shared_channel_linked_member(SyncSharedChannelLinkedMemberCommand {
                tenant_id: request.tenant_id,
                conversation_id: request.conversation_id,
                shared_channel_policy_id: request.shared_channel_policy_id,
                external_connection_id: request.external_connection_id,
                local_actor_id: request.local_actor_id,
                local_actor_kind: request.local_actor_kind,
                external_member_id: request.external_member_id,
                synced_by: "svc_control_plane".into(),
            })
            .map(|_| ())
            .map_err(|error| format!("{error:?}"))
    }
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

pub fn build_default_app_with_user_module_provider(
    user_module_provider: Arc<dyn UserModuleProvider>,
) -> Router {
    match configured_runtime_dir() {
        Some(runtime_dir) => build_default_app_with_runtime_dir_and_user_module_provider(
            runtime_dir,
            user_module_provider,
        ),
        None => build_default_app_with_bind_addr_and_user_module_provider(
            resolve_bind_addr().as_str(),
            user_module_provider,
        ),
    }
}

pub fn build_default_app_with_device_access_provider(
    device_access_provider: Arc<dyn DeviceAccessProvider>,
) -> Router {
    match configured_runtime_dir() {
        Some(runtime_dir) => build_default_app_with_runtime_dir_and_device_access_provider(
            runtime_dir,
            device_access_provider,
        ),
        None => build_default_app_with_bind_addr_and_device_access_provider(
            resolve_bind_addr().as_str(),
            device_access_provider,
        ),
    }
}

pub fn build_default_app_with_iot_protocol_adapter(
    iot_protocol_adapter: Arc<dyn IotProtocolAdapter>,
) -> Router {
    match configured_runtime_dir() {
        Some(runtime_dir) => build_default_app_with_runtime_dir_and_iot_protocol_adapter(
            runtime_dir,
            iot_protocol_adapter,
        ),
        None => build_default_app_with_bind_addr_and_iot_protocol_adapter(
            resolve_bind_addr().as_str(),
            iot_protocol_adapter,
        ),
    }
}

pub fn build_default_app_with_runtime_dir_and_user_module_provider(
    runtime_dir: impl AsRef<StdPath>,
    user_module_provider: Arc<dyn UserModuleProvider>,
) -> Router {
    build_default_app_with_bind_addr_and_runtime_dir_and_user_module_provider(
        resolve_bind_addr().as_str(),
        runtime_dir,
        user_module_provider,
    )
}

pub fn build_default_app_with_runtime_dir_and_device_access_provider(
    runtime_dir: impl AsRef<StdPath>,
    device_access_provider: Arc<dyn DeviceAccessProvider>,
) -> Router {
    build_default_app_with_bind_addr_and_runtime_dir_and_device_access_provider(
        resolve_bind_addr().as_str(),
        runtime_dir,
        device_access_provider,
    )
}

pub fn build_default_app_with_runtime_dir_and_iot_protocol_adapter(
    runtime_dir: impl AsRef<StdPath>,
    iot_protocol_adapter: Arc<dyn IotProtocolAdapter>,
) -> Router {
    build_default_app_with_bind_addr_and_runtime_dir_and_iot_protocol_adapter(
        resolve_bind_addr().as_str(),
        runtime_dir,
        iot_protocol_adapter,
    )
}

fn configured_runtime_dir() -> Option<PathBuf> {
    std::env::var("CRAW_CHAT_RUNTIME_DIR")
        .ok()
        .map(PathBuf::from)
}

fn build_default_device_access_provider() -> Arc<dyn DeviceAccessProvider> {
    Arc::new(LocalDeviceAccessProvider::default())
}

fn build_default_iot_protocol_adapter() -> Arc<dyn IotProtocolAdapter> {
    Arc::new(MqttIotProtocolAdapter::default())
}

fn build_default_app_with_bind_addr(bind_addr: &str) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    build_app_with_dependencies_and_provider_ports(
        "local_minimal_node_1",
        bind_addr,
        projection_service,
        realtime_cluster,
        build_default_user_module_provider(),
        build_default_device_access_provider(),
        build_default_iot_protocol_adapter(),
    )
}

fn build_public_app_with_bind_addr(bind_addr: &str) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    build_app_with_dependencies_and_provider_ports(
        "local_minimal_node_1",
        bind_addr,
        projection_service,
        realtime_cluster,
        build_default_user_module_provider(),
        build_default_device_access_provider(),
        build_default_iot_protocol_adapter(),
    )
    .layer(build_public_browser_cors_layer())
    .layer(middleware::from_fn(require_public_bearer_auth))
}

fn build_default_app_with_bind_addr_and_runtime_dir(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
) -> Router {
    build_default_app_with_bind_addr_and_runtime_dir_and_user_module_provider(
        bind_addr,
        runtime_dir,
        build_default_user_module_provider(),
    )
}

fn build_default_app_with_bind_addr_and_user_module_provider(
    bind_addr: &str,
    user_module_provider: Arc<dyn UserModuleProvider>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    build_app_with_dependencies_and_provider_ports(
        "local_minimal_node_1",
        bind_addr,
        projection_service,
        realtime_cluster,
        user_module_provider,
        build_default_device_access_provider(),
        build_default_iot_protocol_adapter(),
    )
}

fn build_default_app_with_bind_addr_and_device_access_provider(
    bind_addr: &str,
    device_access_provider: Arc<dyn DeviceAccessProvider>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    build_app_with_dependencies_and_provider_ports(
        "local_minimal_node_1",
        bind_addr,
        projection_service,
        realtime_cluster,
        build_default_user_module_provider(),
        device_access_provider,
        build_default_iot_protocol_adapter(),
    )
}

fn build_default_app_with_bind_addr_and_iot_protocol_adapter(
    bind_addr: &str,
    iot_protocol_adapter: Arc<dyn IotProtocolAdapter>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    build_app_with_dependencies_and_provider_ports(
        "local_minimal_node_1",
        bind_addr,
        projection_service,
        realtime_cluster,
        build_default_user_module_provider(),
        build_default_device_access_provider(),
        iot_protocol_adapter,
    )
}

fn build_default_app_with_bind_addr_and_runtime_dir_and_user_module_provider(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
    user_module_provider: Arc<dyn UserModuleProvider>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime_dir = runtime_dir.as_ref().to_path_buf();
    let projection_snapshot_stores =
        build_local_minimal_projection_snapshot_stores(runtime_dir.as_path());
    let realtime_scope_policy =
        realtime_policy::direct_chat_realtime_policy(projection_service.clone());
    let realtime_plane =
        build_local_minimal_realtime_plane(runtime_dir.as_path(), realtime_scope_policy.clone());
    let journal = ProjectionJournal::new_file(
        projection_service.clone(),
        runtime_dir
            .as_path()
            .join("state")
            .join("commit-journal.json"),
        projection_snapshot_stores,
    );
    build_app_with_dependencies_and_runtime_and_journal(
        "local_minimal_node_1",
        bind_addr,
        Some(runtime_dir.clone()),
        projection_service.clone(),
        realtime_plane,
        journal.clone(),
        Some(realtime_scope_policy),
        build_local_minimal_streaming_runtime(runtime_dir.as_path()),
        build_local_minimal_rtc_runtime(runtime_dir.as_path()),
        build_local_minimal_notification_runtime(
            journal.clone(),
            runtime_dir.as_path(),
            projection_service,
        ),
        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
        user_module_provider,
        build_default_device_access_provider(),
        build_default_iot_protocol_adapter(),
    )
}

fn build_default_app_with_bind_addr_and_runtime_dir_and_device_access_provider(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
    device_access_provider: Arc<dyn DeviceAccessProvider>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime_dir = runtime_dir.as_ref().to_path_buf();
    let projection_snapshot_stores =
        build_local_minimal_projection_snapshot_stores(runtime_dir.as_path());
    let realtime_scope_policy =
        realtime_policy::direct_chat_realtime_policy(projection_service.clone());
    let realtime_plane =
        build_local_minimal_realtime_plane(runtime_dir.as_path(), realtime_scope_policy.clone());
    let journal = ProjectionJournal::new_file(
        projection_service.clone(),
        runtime_dir
            .as_path()
            .join("state")
            .join("commit-journal.json"),
        projection_snapshot_stores,
    );
    build_app_with_dependencies_and_runtime_and_journal(
        "local_minimal_node_1",
        bind_addr,
        Some(runtime_dir.clone()),
        projection_service.clone(),
        realtime_plane,
        journal.clone(),
        Some(realtime_scope_policy),
        build_local_minimal_streaming_runtime(runtime_dir.as_path()),
        build_local_minimal_rtc_runtime(runtime_dir.as_path()),
        build_local_minimal_notification_runtime(
            journal.clone(),
            runtime_dir.as_path(),
            projection_service,
        ),
        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
        build_default_user_module_provider(),
        device_access_provider,
        build_default_iot_protocol_adapter(),
    )
}

fn build_default_app_with_bind_addr_and_runtime_dir_and_iot_protocol_adapter(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
    iot_protocol_adapter: Arc<dyn IotProtocolAdapter>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime_dir = runtime_dir.as_ref().to_path_buf();
    let projection_snapshot_stores =
        build_local_minimal_projection_snapshot_stores(runtime_dir.as_path());
    let realtime_scope_policy =
        realtime_policy::direct_chat_realtime_policy(projection_service.clone());
    let realtime_plane =
        build_local_minimal_realtime_plane(runtime_dir.as_path(), realtime_scope_policy.clone());
    let journal = ProjectionJournal::new_file(
        projection_service.clone(),
        runtime_dir
            .as_path()
            .join("state")
            .join("commit-journal.json"),
        projection_snapshot_stores,
    );
    build_app_with_dependencies_and_runtime_and_journal(
        "local_minimal_node_1",
        bind_addr,
        Some(runtime_dir.clone()),
        projection_service.clone(),
        realtime_plane,
        journal.clone(),
        Some(realtime_scope_policy),
        build_local_minimal_streaming_runtime(runtime_dir.as_path()),
        build_local_minimal_rtc_runtime(runtime_dir.as_path()),
        build_local_minimal_notification_runtime(
            journal.clone(),
            runtime_dir.as_path(),
            projection_service,
        ),
        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
        build_default_user_module_provider(),
        build_default_device_access_provider(),
        iot_protocol_adapter,
    )
}

fn build_public_app_with_bind_addr_and_runtime_dir(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime_dir = runtime_dir.as_ref().to_path_buf();
    let projection_snapshot_stores =
        build_local_minimal_projection_snapshot_stores(runtime_dir.as_path());
    let realtime_scope_policy =
        realtime_policy::direct_chat_realtime_policy(projection_service.clone());
    let realtime_plane =
        build_local_minimal_realtime_plane(runtime_dir.as_path(), realtime_scope_policy.clone());
    let journal = ProjectionJournal::new_file(
        projection_service.clone(),
        runtime_dir
            .as_path()
            .join("state")
            .join("commit-journal.json"),
        projection_snapshot_stores,
    );
    build_app_with_dependencies_and_runtime_and_journal(
        "local_minimal_node_1",
        bind_addr,
        Some(runtime_dir.clone()),
        projection_service.clone(),
        realtime_plane,
        journal.clone(),
        Some(realtime_scope_policy),
        build_local_minimal_streaming_runtime(runtime_dir.as_path()),
        build_local_minimal_rtc_runtime(runtime_dir.as_path()),
        build_local_minimal_notification_runtime(
            journal.clone(),
            runtime_dir.as_path(),
            projection_service,
        ),
        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
        build_default_user_module_provider(),
        build_default_device_access_provider(),
        build_default_iot_protocol_adapter(),
    )
    .layer(build_public_browser_cors_layer())
    .layer(middleware::from_fn(require_public_bearer_auth))
}

fn build_public_browser_cors_layer() -> CorsLayer {
    let allowed_origins = resolve_public_browser_origins()
        .into_iter()
        .map(|origin| {
            origin
                .parse::<axum::http::HeaderValue>()
                .expect("configured browser origin should be a valid header value")
        })
        .collect::<Vec<_>>();
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods(AllowMethods::list([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::list([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
        ]))
}

fn build_local_minimal_realtime_plane(
    runtime_dir: impl AsRef<StdPath>,
    scope_access_policy: Arc<dyn RealtimeScopeAccessPolicy>,
) -> RealtimePlaneAssembly {
    let disconnect_fence_store = Arc::new(FileRealtimeDisconnectFenceStore::new(
        runtime_dir
            .as_ref()
            .join("state")
            .join("realtime-disconnect-fences.json"),
    ));
    let checkpoint_store = Arc::new(FileRealtimeCheckpointStore::new(
        runtime_dir
            .as_ref()
            .join("state")
            .join("realtime-checkpoints.json"),
    ));
    let subscription_store = Arc::new(FileRealtimeSubscriptionStore::new(
        runtime_dir
            .as_ref()
            .join("state")
            .join("realtime-subscriptions.json"),
    ));
    let presence_state_store = Arc::new(FilePresenceStateStore::new(
        runtime_dir
            .as_ref()
            .join("state")
            .join("presence-state.json"),
    ));

    RealtimePlaneAssembly::new(
        Arc::new(RealtimeClusterBridge::with_disconnect_fence_store(
            disconnect_fence_store,
        )),
        Arc::new(
            RealtimeDeliveryRuntime::with_stores_and_scope_access_policy(
                checkpoint_store,
                subscription_store,
                scope_access_policy,
            ),
        ),
        Arc::new(SessionPresenceRuntime::with_store(presence_state_store)),
    )
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
    projection_service: Arc<TimelineProjectionService>,
) -> Arc<NotificationRuntime> {
    Arc::new(NotificationRuntime::with_journal_and_store_and_projection(
        Arc::new(journal),
        Arc::new(FileNotificationTaskStore::new(
            runtime_dir
                .as_ref()
                .join("state")
                .join("notification-tasks.json"),
        )),
        projection_service,
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

fn build_local_minimal_projection_snapshot_stores(
    runtime_dir: impl AsRef<StdPath>,
) -> ProjectionSnapshotStores {
    ProjectionSnapshotStores::new(
        FileMetadataStore::new(
            runtime_dir
                .as_ref()
                .join("state")
                .join(PROJECTION_METADATA_FILE_NAME),
        ),
        FileTimelineProjectionStore::new(
            runtime_dir
                .as_ref()
                .join("state")
                .join(PROJECTION_TIMELINE_FILE_NAME),
        ),
    )
}

fn build_local_minimal_control_plane_app(
    realtime_cluster: Arc<RealtimeClusterBridge>,
    conversation_runtime: Arc<ConversationRuntime<ProjectionJournal>>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    runtime_dir: Option<&StdPath>,
) -> (Router, Arc<control_plane_api::SocialControlQuery>) {
    let shared_channel_sync_trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger> =
        Arc::new(LocalMinimalSharedChannelLinkedMemberSyncTrigger {
            conversation_runtime,
        });

    match runtime_dir {
        Some(runtime_dir) => control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger_with_social_query(
            realtime_cluster,
            ops_runtime,
            audit_runtime,
            runtime_dir,
            shared_channel_sync_trigger,
        ),
        None => control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_shared_channel_sync_trigger_with_social_query(
            realtime_cluster,
            ops_runtime,
            audit_runtime,
            shared_channel_sync_trigger,
        ),
    }
}

pub fn build_app_with_dependencies(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
) -> Router {
    build_app_with_dependencies_and_provider_ports(
        node_id,
        bind_addr,
        projection_service,
        realtime_cluster,
        build_default_user_module_provider(),
        build_default_device_access_provider(),
        build_default_iot_protocol_adapter(),
    )
}

fn build_app_with_dependencies_and_provider_ports(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    user_module_provider: Arc<dyn UserModuleProvider>,
    device_access_provider: Arc<dyn DeviceAccessProvider>,
    iot_protocol_adapter: Arc<dyn IotProtocolAdapter>,
) -> Router {
    let realtime_scope_policy =
        realtime_policy::direct_chat_realtime_policy(projection_service.clone());
    let journal = ProjectionJournal::new_memory(projection_service.clone());
    let realtime_runtime = Arc::new(
        RealtimeDeliveryRuntime::with_checkpoint_store_and_scope_access_policy(
            Arc::new(MemoryRealtimeCheckpointStore::default()),
            realtime_scope_policy.clone(),
        ),
    );
    build_app_with_dependencies_and_runtime_and_journal(
        node_id,
        bind_addr,
        None,
        projection_service.clone(),
        RealtimePlaneAssembly::with_cluster_and_runtime(realtime_cluster, realtime_runtime),
        journal.clone(),
        Some(realtime_scope_policy),
        Arc::new(StreamingRuntime::default()),
        Arc::new(RtcRuntime::default()),
        Arc::new(NotificationRuntime::with_journal_and_projection(
            Arc::new(journal.clone()),
            projection_service,
        )),
        Arc::new(AutomationRuntime::with_journal(Arc::new(journal))),
        user_module_provider,
        device_access_provider,
        iot_protocol_adapter,
    )
}

pub fn build_app_with_dependencies_and_runtime(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
) -> Router {
    let journal = ProjectionJournal::new_memory(projection_service.clone());
    build_app_with_dependencies_and_runtime_and_journal(
        node_id,
        bind_addr,
        None,
        projection_service.clone(),
        RealtimePlaneAssembly::with_cluster_and_runtime(realtime_cluster, realtime_runtime),
        journal.clone(),
        None,
        Arc::new(StreamingRuntime::default()),
        Arc::new(RtcRuntime::default()),
        Arc::new(NotificationRuntime::with_journal_and_projection(
            Arc::new(journal),
            projection_service,
        )),
        Arc::new(AutomationRuntime::default()),
        build_default_user_module_provider(),
        build_default_device_access_provider(),
        build_default_iot_protocol_adapter(),
    )
}

// This assembly helper keeps the concrete runtime components explicit so local
// node bootstrap tests can swap any subsystem without hidden defaults.
#[allow(clippy::too_many_arguments)]
fn build_app_with_dependencies_and_runtime_and_journal(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    runtime_dir: Option<PathBuf>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_plane: RealtimePlaneAssembly,
    journal: ProjectionJournal,
    realtime_scope_policy: Option<Arc<realtime_policy::DirectChatRealtimePolicy>>,
    streaming_runtime: Arc<StreamingRuntime>,
    rtc_runtime: Arc<RtcRuntime>,
    notification_runtime: Arc<NotificationRuntime>,
    automation_runtime: Arc<AutomationRuntime>,
    user_module_provider: Arc<dyn UserModuleProvider>,
    device_access_provider: Arc<dyn DeviceAccessProvider>,
    iot_protocol_adapter: Arc<dyn IotProtocolAdapter>,
) -> Router {
    let node_id = node_id.into();
    let bind_addr = bind_addr.into();
    realtime_plane.bind_node_runtime(node_id.as_str());
    let realtime_cluster = realtime_plane.realtime_cluster();
    let session_presence_runtime = realtime_plane.presence_runtime();
    let realtime_runtime = realtime_plane.realtime_runtime();
    let conversation_runtime = Arc::new(ConversationRuntime::new(journal.clone()));
    let replay_summary = replay_projection_journal(
        &journal,
        projection_service.as_ref(),
        conversation_runtime.as_ref(),
    );
    journal.set_applied_event_count(replay_summary.recorded_event_count);
    projection_service.record_projection_replay_metrics(
        replay_summary.backlog_size,
        replay_summary.replayed_event_count,
        replay_summary.duration_ms,
    );
    projection_service.record_projection_rebuild_duration(replay_summary.rebuild_duration_ms);
    let ops_runtime = Arc::new(OpsRuntime::new(
        node_id.clone(),
        "local-minimal",
        bind_addr.clone(),
        vec![
            "conversation-runtime".into(),
            "control-plane-api".into(),
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
    ops_runtime.update_projection_replay_lag(replay_summary.lag_items);
    let audit_runtime = Arc::new(AuditRuntime::default());
    let (control_plane_app, social_query) = build_local_minimal_control_plane_app(
        realtime_cluster.clone(),
        conversation_runtime.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.as_deref(),
    );
    if let Some(realtime_scope_policy) = realtime_scope_policy.as_ref() {
        realtime_scope_policy.bind_social_query(social_query.clone());
    }
    let device_registration = LocalNodeDeviceRegistration::new(
        node_id.clone(),
        realtime_cluster.clone(),
        session_presence_runtime.clone(),
        realtime_runtime.clone(),
        projection_service.clone(),
        journal.snapshot_stores(),
        device_access_provider,
    );
    let device_twin_store: Arc<dyn DeviceTwinStore> = match runtime_dir.as_ref() {
        Some(runtime_dir) => Arc::new(FileDeviceTwinStore::new(
            runtime_dir
                .as_path()
                .join("state")
                .join("device-twin-state.json"),
        )),
        None => Arc::new(MemoryDeviceTwinStore::default()),
    };
    let auth_runtime = Arc::new(auth::AuthRuntime::new(runtime_dir.clone()));
    let pending_friend_request_accept_repairs =
        social::load_pending_friend_request_accept_repairs(runtime_dir.as_deref());
    let state = AppState {
        node_id: node_id.clone(),
        runtime_dir,
        auth_runtime,
        control_plane_app: control_plane_app.clone(),
        social_query,
        realtime_cluster,
        conversation_runtime,
        user_module_provider,
        projection_service,
        session_presence_runtime,
        realtime_runtime,
        device_registration,
        device_twin_store,
        iot_protocol_adapter,
        media_runtime: Arc::new(MediaRuntime::with_journal(Arc::new(journal.clone()))),
        streaming_runtime,
        rtc_runtime,
        notification_runtime,
        automation_runtime,
        audit_runtime,
        ops_runtime,
        projection_replay_state: journal.replay_state(),
        pending_friend_request_accept_repairs: Arc::new(std::sync::Mutex::new(
            pending_friend_request_accept_repairs,
        )),
        friend_request_accept_repair_gate: Arc::new(tokio::sync::Mutex::new(())),
    };
    social::spawn_pending_friend_request_accept_repair(state.clone());
    platform::refresh_node_operational_view(&state);
    build_app(state).merge(control_plane_app)
}

struct ProjectionReplaySummary {
    backlog_size: u64,
    replayed_event_count: u64,
    recorded_event_count: usize,
    duration_ms: u64,
    rebuild_duration_ms: u64,
    lag_items: Vec<LagItem>,
}

fn replay_projection_journal(
    journal: &ProjectionJournal,
    projection_service: &TimelineProjectionService,
    conversation_runtime: &ConversationRuntime<ProjectionJournal>,
) -> ProjectionReplaySummary {
    let replay_started_at = std::time::Instant::now();
    let recorded = match journal.recorded() {
        Ok(recorded) => recorded,
        Err(error) => {
            eprintln!(
                "failed to load local-minimal commit journal during startup replay: {error:?}. starting with empty replay backlog"
            );
            Vec::new()
        }
    };
    let restore_summary =
        journal.restore_projection_snapshots(recorded.as_slice(), projection_service);
    let restored_checkpoints = restore_summary.restored_checkpoints;
    let mut scope_lag = BTreeMap::new();
    for (scope_id, checkpoint) in &restored_checkpoints {
        scope_lag.insert(
            scope_id.clone(),
            LagItem {
                component: "projection_replay".into(),
                scope_id: scope_id.clone(),
                current_offset: *checkpoint,
                committed_offset: *checkpoint,
                lag: 0,
            },
        );
    }
    let mut backlog_size = 0;
    let mut replayed_event_count = 0;
    let rebuild_happened = restore_summary.restored_device_sync
        || !restored_checkpoints.is_empty()
        || recorded
            .iter()
            .any(|envelope| envelope.scope_type == "conversation");

    for envelope in &recorded {
        if envelope.scope_type == "conversation" {
            let scope_id =
                projection_snapshot_scope(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
            let checkpoint = restored_checkpoints
                .get(scope_id.as_str())
                .copied()
                .unwrap_or(0);
            let lag = scope_lag
                .entry(scope_id.clone())
                .or_insert_with(|| LagItem {
                    component: "projection_replay".into(),
                    scope_id: scope_id.clone(),
                    current_offset: checkpoint,
                    committed_offset: checkpoint,
                    lag: 0,
                });
            lag.current_offset = lag.current_offset.max(envelope.ordering_seq);
            lag.committed_offset = checkpoint;
            lag.lag = lag.current_offset.saturating_sub(lag.committed_offset);
        }
        let replay_projection = !matches!(
            restored_checkpoints.get(
                projection_snapshot_scope(envelope.tenant_id.as_str(), envelope.scope_id.as_str())
                    .as_str(),
            ),
            Some(checkpoint)
                if envelope.scope_type == "conversation"
                    && envelope.ordering_seq <= *checkpoint
        );

        if replay_projection {
            backlog_size += 1;
            if let Err(error) = projection_service.apply(envelope) {
                eprintln!(
                    "failed to replay projection event {} during local-minimal startup: {error:?}. continuing bootstrap in degraded replay mode",
                    envelope.event_id
                );
            } else {
                replayed_event_count += 1;
            }
        }

        if let Err(error) = conversation_runtime.apply_recovered_envelope(envelope) {
            eprintln!(
                "failed to replay conversation event {} during local-minimal startup: {error:?}. continuing bootstrap with partial domain recovery",
                envelope.event_id
            );
        }
    }

    journal.mark_social_projection_events(recorded.iter());

    ProjectionReplaySummary {
        backlog_size,
        replayed_event_count,
        recorded_event_count: recorded.len(),
        duration_ms: if replayed_event_count == 0 {
            0
        } else {
            std::cmp::max(1, replay_started_at.elapsed().as_millis() as u64)
        },
        rebuild_duration_ms: if rebuild_happened {
            std::cmp::max(1, replay_started_at.elapsed().as_millis() as u64)
        } else {
            0
        },
        lag_items: scope_lag.into_values().collect(),
    }
}

fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route(APP_OPENAPI_SCHEMA_PATH, get(export_app_openapi_schema))
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/refresh", post(auth::refresh))
        .route("/api/v1/auth/me", get(auth::me))
        .route("/api/v1/portal/home", get(portal::get_home))
        .route("/api/v1/portal/auth", get(portal::get_auth))
        .route("/api/v1/portal/workspace", get(portal::get_workspace))
        .route("/api/v1/portal/dashboard", get(portal::get_dashboard))
        .route(
            "/api/v1/portal/conversations",
            get(portal::get_conversations),
        )
        .route("/api/v1/portal/realtime", get(portal::get_realtime))
        .route("/api/v1/portal/media", get(portal::get_media))
        .route("/api/v1/portal/automation", get(portal::get_automation))
        .route("/api/v1/portal/governance", get(portal::get_governance))
        .route("/api/v1/sessions/resume", post(session::resume_session))
        .route(
            "/api/v1/sessions/disconnect",
            post(session::disconnect_session),
        )
        .route(
            "/api/v1/presence/heartbeat",
            post(session::heartbeat_presence),
        )
        .route("/api/v1/presence/me", get(session::get_presence_me))
        .route(
            "/api/v1/realtime/subscriptions/sync",
            post(session::sync_realtime_subscriptions),
        )
        .route("/api/v1/realtime/ws", get(session::realtime_websocket))
        .route(
            "/api/v1/realtime/events/ack",
            post(session::ack_realtime_events),
        )
        .route(
            "/api/v1/realtime/events",
            get(session::list_realtime_events),
        )
        .route("/api/v1/devices/register", post(session::register_device))
        .route(
            "/api/v1/devices/{device_id}/sync-feed",
            get(session::get_device_sync_feed),
        )
        .route(
            "/api/v1/devices/{device_id}/twin",
            get(twin::get_device_twin),
        )
        .route(
            "/api/v1/devices/{device_id}/twin/desired",
            post(twin::update_device_twin_desired),
        )
        .route(
            "/api/v1/devices/{device_id}/twin/reported",
            post(twin::update_device_twin_reported),
        )
        .route(
            "/api/v1/social/friend-requests",
            get(social::list_friend_requests).post(social::submit_friend_request),
        )
        .route(
            "/api/v1/social/friend-requests/{request_id}/accept",
            post(social::accept_friend_request),
        )
        .route(
            "/api/v1/social/friend-requests/{request_id}/decline",
            post(social::decline_friend_request),
        )
        .route(
            "/api/v1/social/friend-requests/{request_id}/cancel",
            post(social::cancel_friend_request),
        )
        .route(
            "/api/v1/social/friendships/{friendship_id}/remove",
            post(social::remove_friendship),
        )
        .route("/api/v1/contacts", get(projection::get_contacts))
        .route("/api/v1/inbox", get(projection::get_inbox))
        .route(
            "/api/v1/conversations",
            post(conversation::create_conversation),
        )
        .route(
            "/api/v1/conversations/agent-dialogs",
            post(conversation::create_agent_dialog),
        )
        .route(
            "/api/v1/conversations/agent-handoffs",
            post(conversation::create_agent_handoff),
        )
        .route(
            "/api/v1/conversations/system-channels",
            post(conversation::create_system_channel),
        )
        .route(
            "/api/v1/conversations/threads",
            post(conversation::create_thread_conversation),
        )
        .route(
            "/api/v1/conversations/direct-chats/bindings",
            post(conversation::bind_direct_chat_conversation),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff",
            get(handoff::get_agent_handoff_state),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff/accept",
            post(handoff::accept_agent_handoff),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff/resolve",
            post(handoff::resolve_agent_handoff),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/agent-handoff/close",
            post(handoff::close_agent_handoff),
        )
        .route(
            "/api/v1/conversations/{conversation_id}",
            get(projection::get_conversation_summary),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members",
            get(membership::list_members),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/add",
            post(membership::add_member),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/remove",
            post(membership::remove_member),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/transfer-owner",
            post(membership::transfer_conversation_owner),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/change-role",
            post(membership::change_conversation_member_role),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/members/leave",
            post(membership::leave_conversation),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/read-cursor",
            get(projection::get_read_cursor).post(projection::update_read_cursor),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/member-directory",
            get(projection::get_member_directory),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/messages",
            post(message::post_message),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/system-channel/publish",
            post(message::publish_system_channel_message),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/messages",
            get(projection::get_timeline),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/pins",
            get(projection::get_pinned_messages),
        )
        .route(
            "/api/v1/conversations/{conversation_id}/messages/{message_id}/interaction-summary",
            get(projection::get_message_interaction_summary),
        )
        .route(
            "/api/v1/messages/{message_id}/edit",
            post(message::edit_message),
        )
        .route(
            "/api/v1/messages/{message_id}/recall",
            post(message::recall_message),
        )
        .route("/api/v1/media/uploads", post(media::create_media_upload))
        .route(
            "/api/v1/media/uploads/{media_asset_id}/complete",
            post(media::complete_media_upload),
        )
        .route(
            "/api/v1/media/provider-health",
            get(media::get_media_provider_health),
        )
        .route(
            "/api/v1/user-module/provider-health",
            get(user_module::get_user_module_provider_health),
        )
        .route(
            "/api/v1/iot/access/provider-health",
            get(iot::get_iot_access_provider_health),
        )
        .route(
            "/api/v1/iot/protocol/provider-health",
            get(iot::get_iot_protocol_provider_health),
        )
        .route(
            "/api/v1/iot/protocol/uplink",
            post(iot::ingest_iot_protocol_uplink),
        )
        .route(
            "/api/v1/iot/protocol/downlink",
            post(iot::ingest_iot_protocol_downlink),
        )
        .route(
            "/api/v1/media/{media_asset_id}/download-url",
            get(media::get_media_download_url),
        )
        .route("/api/v1/media/{media_asset_id}", get(media::get_media))
        .route(
            "/api/v1/media/{media_asset_id}/attach",
            post(media::attach_media),
        )
        .route("/api/v1/streams", post(stream::open_stream))
        .route(
            "/api/v1/streams/{stream_id}/frames",
            post(stream::append_stream_frame).get(stream::list_stream_frames),
        )
        .route(
            "/api/v1/streams/{stream_id}/checkpoint",
            post(stream::checkpoint_stream),
        )
        .route(
            "/api/v1/streams/{stream_id}/complete",
            post(stream::complete_stream),
        )
        .route(
            "/api/v1/streams/{stream_id}/abort",
            post(stream::abort_stream),
        )
        .route("/api/v1/rtc/sessions", post(rtc::create_rtc_session))
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/invite",
            post(rtc::invite_rtc_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/accept",
            post(rtc::accept_rtc_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/reject",
            post(rtc::reject_rtc_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/end",
            post(rtc::end_rtc_session),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/signals",
            post(rtc::post_rtc_signal),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/credentials",
            post(rtc::issue_rtc_participant_credential),
        )
        .route(
            "/api/v1/rtc/sessions/{rtc_session_id}/artifacts/recording",
            get(rtc::get_rtc_recording_artifact),
        )
        .route(
            "/api/v1/rtc/provider-callbacks",
            post(rtc::map_rtc_provider_callback),
        )
        .route(
            "/api/v1/rtc/provider-health",
            get(rtc::get_rtc_provider_health),
        )
        .route(
            "/api/v1/notifications/requests",
            post(platform::request_notification),
        )
        .route("/api/v1/notifications", get(platform::list_notifications))
        .route(
            "/api/v1/notifications/{notification_id}",
            get(platform::get_notification),
        )
        .route(
            "/api/v1/automation/executions",
            post(platform::request_automation_execution),
        )
        .route(
            "/api/v1/automation/governance",
            get(platform::get_automation_governance),
        )
        .route(
            "/api/v1/automation/agent-responses",
            post(platform::start_agent_response),
        )
        .route(
            "/api/v1/automation/agent-responses/{stream_id}/frames",
            post(platform::append_agent_response_delta),
        )
        .route(
            "/api/v1/automation/agent-responses/{stream_id}/complete",
            post(platform::complete_agent_response),
        )
        .route(
            "/api/v1/automation/agent-tool-calls",
            post(platform::request_agent_tool_call),
        )
        .route(
            "/api/v1/automation/executions/{execution_id}/agent-tool-calls/{tool_call_id}/complete",
            post(platform::complete_agent_tool_call),
        )
        .route(
            "/api/v1/automation/executions/{execution_id}",
            get(platform::get_automation_execution),
        )
        .route("/api/v1/audit/records", post(platform::record_audit_anchor))
        .route("/api/v1/audit/records", get(platform::list_audit_records))
        .route("/api/v1/audit/export", get(platform::export_audit_bundle))
        .route("/api/v1/ops/health", get(platform::get_ops_health))
        .route("/api/v1/ops/cluster", get(platform::get_ops_cluster))
        .route("/api/v1/ops/lag", get(platform::get_ops_lag))
        .route(
            "/api/v1/ops/replay-status",
            get(platform::get_ops_replay_status),
        )
        .route(
            "/api/v1/ops/runtime-dir",
            get(platform::get_ops_runtime_dir),
        )
        .route(
            "/api/v1/ops/provider-bindings",
            get(platform::get_ops_provider_bindings),
        )
        .route(
            "/api/v1/ops/provider-bindings/drift",
            get(platform::get_ops_provider_binding_drift),
        )
        .route(
            "/api/v1/ops/diagnostics",
            get(platform::get_ops_diagnostics),
        )
        .with_state(state)
}
