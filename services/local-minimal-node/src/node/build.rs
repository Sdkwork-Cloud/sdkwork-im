use super::principal_profile::build_default_principal_profile_provider;
use super::*;
use conversation_runtime::SyncSharedChannelLinkedMemberCommand;
use im_adapters_postgres_realtime::{
    PostgresRealtimeCheckpointStore, PostgresRealtimeConfig, PostgresRealtimeDisconnectFenceStore,
    PostgresRealtimeEventWindowStore, PostgresRealtimePresenceStateStore,
    PostgresRealtimeSubscriptionStore,
};
use serde::Deserialize;
use social_service::SharedChannelLinkedMemberSyncRequest;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

#[allow(dead_code)]
trait SharedChannelLinkedMemberSyncTrigger: Send + Sync {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum LocalMinimalRealtimeStorageProvider {
    LocalDisk,
    Postgresql,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LocalMinimalRealtimeStorageConfig {
    provider: LocalMinimalRealtimeStorageProvider,
    postgres: Option<PostgresRealtimeConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalMinimalPostgresConfigFile {
    provider: String,
    connection: LocalMinimalPostgresConnectionConfig,
    #[serde(default)]
    pool: LocalMinimalPostgresPoolConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalMinimalPostgresConnectionConfig {
    host: String,
    #[serde(default = "default_postgres_port")]
    port: u16,
    database: String,
    username: String,
    password_file: String,
    #[serde(default = "default_postgres_sslmode")]
    sslmode: String,
    #[serde(default)]
    application_name: Option<String>,
    #[serde(default)]
    connect_timeout_seconds: Option<u64>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalMinimalPostgresPoolConfig {
    min_connections: Option<u32>,
    max_connections: Option<u32>,
}

#[derive(Clone)]
#[allow(dead_code)]
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

pub fn try_build_public_app() -> Result<Router, String> {
    match configured_runtime_dir() {
        Some(runtime_dir) => try_build_public_app_with_bind_addr_and_runtime_dir(
            resolve_bind_addr().as_str(),
            runtime_dir,
        ),
        None => Ok(build_public_app_with_bind_addr(
            resolve_bind_addr().as_str(),
        )),
    }
}

pub fn build_default_app_with_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> Router {
    build_default_app_with_bind_addr_and_runtime_dir(resolve_bind_addr().as_str(), runtime_dir)
}

pub fn build_public_app_with_runtime_dir(runtime_dir: impl AsRef<StdPath>) -> Router {
    build_public_app_with_bind_addr_and_runtime_dir(resolve_bind_addr().as_str(), runtime_dir)
}

pub fn build_default_app_with_principal_profile_provider(
    principal_profile_provider: Arc<dyn PrincipalProfileProvider>,
) -> Router {
    match configured_runtime_dir() {
        Some(runtime_dir) => build_default_app_with_runtime_dir_and_principal_profile_provider(
            runtime_dir,
            principal_profile_provider,
        ),
        None => build_default_app_with_bind_addr_and_principal_profile_provider(
            resolve_bind_addr().as_str(),
            principal_profile_provider,
        ),
    }
}

pub fn build_default_app_with_runtime_dir_and_principal_profile_provider(
    runtime_dir: impl AsRef<StdPath>,
    principal_profile_provider: Arc<dyn PrincipalProfileProvider>,
) -> Router {
    build_default_app_with_bind_addr_and_runtime_dir_and_principal_profile_provider(
        resolve_bind_addr().as_str(),
        runtime_dir,
        principal_profile_provider,
    )
}

fn configured_runtime_dir() -> Option<PathBuf> {
    std::env::var("SDKWORK_IM_RUNTIME_DIR")
        .ok()
        .map(PathBuf::from)
}

fn build_default_app_with_bind_addr(bind_addr: &str) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    build_app_with_dependencies_and_provider_ports(
        "local_minimal_node_1",
        bind_addr,
        projection_service,
        realtime_cluster,
        build_default_principal_profile_provider(),
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
        build_default_principal_profile_provider(),
    )
    .layer(axum::extract::DefaultBodyLimit::max(
        resolve_max_http_request_body_bytes(),
    ))
    .layer(build_public_browser_cors_layer())
    .layer(middleware::from_fn_with_state(
        build_public_app_guardrails(),
        require_app_context_with_guardrails,
    ))
}

fn build_default_app_with_bind_addr_and_runtime_dir(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
) -> Router {
    build_default_app_with_bind_addr_and_runtime_dir_and_principal_profile_provider(
        bind_addr,
        runtime_dir,
        build_default_principal_profile_provider(),
    )
}

fn build_default_app_with_bind_addr_and_principal_profile_provider(
    bind_addr: &str,
    principal_profile_provider: Arc<dyn PrincipalProfileProvider>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    build_app_with_dependencies_and_provider_ports(
        "local_minimal_node_1",
        bind_addr,
        projection_service,
        realtime_cluster,
        principal_profile_provider,
    )
}

fn build_default_app_with_bind_addr_and_runtime_dir_and_principal_profile_provider(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
    principal_profile_provider: Arc<dyn PrincipalProfileProvider>,
) -> Router {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime_dir = runtime_dir.as_ref().to_path_buf();
    ensure_local_minimal_task_runtime_state_files(runtime_dir.as_path())
        .unwrap_or_else(|error| panic!("failed to initialize local-minimal task state: {error}"));
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
        build_local_minimal_call_runtime(runtime_dir.as_path()),
        build_local_minimal_notification_runtime(
            journal.clone(),
            runtime_dir.as_path(),
            projection_service,
        ),
        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
        principal_profile_provider,
    )
}

fn build_public_app_with_bind_addr_and_runtime_dir(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
) -> Router {
    try_build_public_app_with_bind_addr_and_runtime_dir(bind_addr, runtime_dir).unwrap_or_else(
        |error| {
            panic!("failed to build local-minimal public app: {error}");
        },
    )
}

fn try_build_public_app_with_bind_addr_and_runtime_dir(
    bind_addr: &str,
    runtime_dir: impl AsRef<StdPath>,
) -> Result<Router, String> {
    let projection_service = Arc::new(TimelineProjectionService::default());
    let runtime_dir = runtime_dir.as_ref().to_path_buf();
    ensure_local_minimal_task_runtime_state_files(runtime_dir.as_path())
        .map_err(|error| format!("failed to initialize local-minimal task state: {error}"))?;
    let projection_snapshot_stores =
        build_local_minimal_projection_snapshot_stores(runtime_dir.as_path());
    let realtime_scope_policy =
        realtime_policy::direct_chat_realtime_policy(projection_service.clone());
    let realtime_plane = try_build_local_minimal_realtime_plane(
        runtime_dir.as_path(),
        realtime_scope_policy.clone(),
    )?;
    let journal = ProjectionJournal::new_file(
        projection_service.clone(),
        runtime_dir
            .as_path()
            .join("state")
            .join("commit-journal.json"),
        projection_snapshot_stores,
    );
    Ok(build_app_with_dependencies_and_runtime_and_journal(
        "local_minimal_node_1",
        bind_addr,
        Some(runtime_dir.clone()),
        projection_service.clone(),
        realtime_plane,
        journal.clone(),
        Some(realtime_scope_policy),
        build_local_minimal_streaming_runtime(runtime_dir.as_path()),
        build_local_minimal_call_runtime(runtime_dir.as_path()),
        build_local_minimal_notification_runtime(
            journal.clone(),
            runtime_dir.as_path(),
            projection_service,
        ),
        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
        build_default_principal_profile_provider(),
    )
    .layer(axum::extract::DefaultBodyLimit::max(
        resolve_max_http_request_body_bytes(),
    ))
    .layer(build_public_browser_cors_layer())
    .layer(middleware::from_fn_with_state(
        build_public_app_guardrails(),
        require_app_context_with_guardrails,
    )))
}

fn build_public_browser_cors_layer() -> CorsLayer {
    let mut allowed_headers = Vec::new();
    for header_name in [
        axum::http::header::AUTHORIZATION.as_str(),
        axum::http::header::CONTENT_TYPE.as_str(),
        "access-token",
    ] {
        if let Ok(parsed) = header_name.parse::<axum::http::header::HeaderName>()
            && !allowed_headers.contains(&parsed)
        {
            allowed_headers.push(parsed);
        }
    }
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
        .allow_headers(AllowHeaders::list(allowed_headers))
}

fn build_local_minimal_realtime_plane(
    runtime_dir: impl AsRef<StdPath>,
    scope_access_policy: Arc<dyn RealtimeScopeAccessPolicy>,
) -> RealtimePlaneAssembly {
    try_build_local_minimal_realtime_plane(runtime_dir, scope_access_policy).unwrap_or_else(
        |error| {
            panic!("failed to build local-minimal realtime plane: {error}");
        },
    )
}

fn try_build_local_minimal_realtime_plane(
    runtime_dir: impl AsRef<StdPath>,
    scope_access_policy: Arc<dyn RealtimeScopeAccessPolicy>,
) -> Result<RealtimePlaneAssembly, String> {
    let storage_config = resolve_local_minimal_realtime_storage_config_from_env()?;
    try_build_local_minimal_realtime_plane_with_storage_config(
        runtime_dir,
        scope_access_policy,
        storage_config,
    )
}

fn try_build_local_minimal_realtime_plane_with_storage_config(
    runtime_dir: impl AsRef<StdPath>,
    scope_access_policy: Arc<dyn RealtimeScopeAccessPolicy>,
    storage_config: LocalMinimalRealtimeStorageConfig,
) -> Result<RealtimePlaneAssembly, String> {
    match storage_config.provider {
        LocalMinimalRealtimeStorageProvider::LocalDisk => Ok(
            build_local_minimal_file_realtime_plane(runtime_dir, scope_access_policy),
        ),
        LocalMinimalRealtimeStorageProvider::Postgresql => {
            build_local_minimal_postgres_realtime_plane(
                runtime_dir,
                scope_access_policy,
                storage_config
                    .postgres
                    .ok_or_else(|| "postgresql realtime storage config is missing".to_owned())?,
            )
        }
    }
}

fn build_local_minimal_file_realtime_plane(
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
    let event_window_store = Arc::new(FileRealtimeEventWindowStore::new(
        runtime_dir
            .as_ref()
            .join("state")
            .join("realtime-event-windows.json"),
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
            RealtimeDeliveryRuntime::with_durable_stores_and_scope_access_policy(
                checkpoint_store,
                subscription_store,
                event_window_store,
                scope_access_policy,
            ),
        ),
        Arc::new(PresenceRuntime::with_store(presence_state_store)),
    )
}

fn build_local_minimal_postgres_realtime_plane(
    _runtime_dir: impl AsRef<StdPath>,
    scope_access_policy: Arc<dyn RealtimeScopeAccessPolicy>,
    postgres_config: PostgresRealtimeConfig,
) -> Result<RealtimePlaneAssembly, String> {
    let pool = postgres_config
        .connect_pool()
        .map_err(|error| format!("failed to create PostgreSQL realtime storage pool: {error:?}"))?;
    let disconnect_fence_store = Arc::new(PostgresRealtimeDisconnectFenceStore::from_pool(
        pool.clone(),
    ));
    let checkpoint_store = Arc::new(PostgresRealtimeCheckpointStore::from_pool(pool.clone()));
    let subscription_store = Arc::new(PostgresRealtimeSubscriptionStore::from_pool(pool.clone()));
    let event_window_store = Arc::new(PostgresRealtimeEventWindowStore::from_pool(pool.clone()));
    let presence_state_store = Arc::new(PostgresRealtimePresenceStateStore::from_pool(pool));

    Ok(RealtimePlaneAssembly::new(
        Arc::new(RealtimeClusterBridge::with_disconnect_fence_store(
            disconnect_fence_store,
        )),
        Arc::new(
            RealtimeDeliveryRuntime::with_durable_stores_and_scope_access_policy(
                checkpoint_store,
                subscription_store,
                event_window_store,
                scope_access_policy,
            ),
        ),
        Arc::new(PresenceRuntime::with_store(presence_state_store)),
    ))
}

fn resolve_local_minimal_realtime_storage_config_from_env()
-> Result<LocalMinimalRealtimeStorageConfig, String> {
    let storage_provider = std::env::var(SDKWORK_IM_STORAGE_PROVIDER_ENV).ok();
    let database_url = std::env::var(SDKWORK_IM_DATABASE_URL_ENV).ok();
    let postgres_config = std::env::var(SDKWORK_IM_POSTGRES_CONFIG_ENV).ok();
    resolve_local_minimal_realtime_storage_config(
        storage_provider.as_deref(),
        database_url.as_deref(),
        postgres_config.as_deref(),
    )
}

fn resolve_local_minimal_realtime_storage_config(
    storage_provider: Option<&str>,
    database_url: Option<&str>,
    postgres_config: Option<&str>,
) -> Result<LocalMinimalRealtimeStorageConfig, String> {
    let provider = normalize_realtime_storage_provider(storage_provider);
    match provider.as_deref() {
        None | Some("local-disk") | Some("localdisk") | Some("file") | Some("filesystem") => {
            Ok(LocalMinimalRealtimeStorageConfig {
                provider: LocalMinimalRealtimeStorageProvider::LocalDisk,
                postgres: None,
            })
        }
        Some("postgresql") | Some("postgres") => {
            let database_url = normalize_realtime_storage_value(database_url);
            let postgres = if let Some(database_url) = database_url {
                PostgresRealtimeConfig::new(database_url)
            } else if let Some(config_path) = normalize_realtime_storage_value(postgres_config) {
                load_local_minimal_postgres_realtime_config(config_path.as_str())?
            } else {
                return Err(format!(
                    "{SDKWORK_IM_STORAGE_PROVIDER_ENV}=postgresql requires {SDKWORK_IM_DATABASE_URL_ENV} or {SDKWORK_IM_POSTGRES_CONFIG_ENV}"
                ));
            };
            Ok(LocalMinimalRealtimeStorageConfig {
                provider: LocalMinimalRealtimeStorageProvider::Postgresql,
                postgres: Some(postgres),
            })
        }
        Some(provider) => Err(format!(
            "{SDKWORK_IM_STORAGE_PROVIDER_ENV} has unsupported realtime storage provider `{provider}`; supported values are local-disk and postgresql"
        )),
    }
}

fn load_local_minimal_postgres_realtime_config(
    config_path: &str,
) -> Result<PostgresRealtimeConfig, String> {
    let config_path = PathBuf::from(config_path);
    let body = std::fs::read_to_string(&config_path).map_err(|error| {
        format!(
            "failed to read {SDKWORK_IM_POSTGRES_CONFIG_ENV} at {}: {error}",
            config_path.display()
        )
    })?;
    let parsed: LocalMinimalPostgresConfigFile =
        serde_yaml::from_str(body.as_str()).map_err(|error| {
            format!(
                "failed to parse {SDKWORK_IM_POSTGRES_CONFIG_ENV} at {}: {error}",
                config_path.display()
            )
        })?;
    if parsed.provider.trim().eq_ignore_ascii_case("postgresql") {
        validate_local_minimal_postgres_config(&parsed)?;
        let connection = parsed.connection;
        let password_path = resolve_postgres_password_file_path(
            config_path.parent().unwrap_or_else(|| StdPath::new(".")),
            connection.password_file.as_str(),
        );
        let password = std::fs::read_to_string(&password_path).map_err(|error| {
            format!(
                "failed to read PostgreSQL passwordFile at {}: {error}",
                password_path.display()
            )
        })?;
        let database_url =
            render_postgres_key_value_connection_string(&connection, password.trim());
        let mut postgres = PostgresRealtimeConfig::new(database_url);
        if let Some(max_connections) = parsed.pool.max_connections {
            postgres = postgres.with_pool_max_size(max_connections);
        }
        if let Some(min_connections) = parsed.pool.min_connections {
            postgres = postgres.with_pool_min_idle(min_connections);
        }
        Ok(postgres)
    } else {
        Err(format!(
            "{SDKWORK_IM_POSTGRES_CONFIG_ENV} provider must be postgresql, got `{}`",
            parsed.provider
        ))
    }
}

fn validate_local_minimal_postgres_config(
    parsed: &LocalMinimalPostgresConfigFile,
) -> Result<(), String> {
    let mut errors = Vec::new();
    require_non_empty_postgres_field(
        "connection.host",
        parsed.connection.host.as_str(),
        &mut errors,
    );
    require_non_empty_postgres_field(
        "connection.database",
        parsed.connection.database.as_str(),
        &mut errors,
    );
    require_non_empty_postgres_field(
        "connection.username",
        parsed.connection.username.as_str(),
        &mut errors,
    );
    require_non_empty_postgres_field(
        "connection.passwordFile",
        parsed.connection.password_file.as_str(),
        &mut errors,
    );
    if parsed.connection.port == 0 {
        errors.push("connection.port must be greater than zero".to_owned());
    }
    if matches!(parsed.connection.connect_timeout_seconds, Some(0)) {
        errors.push("connection.connectTimeoutSeconds must be greater than zero".to_owned());
    }
    validate_postgres_sslmode(parsed.connection.sslmode.as_str(), &mut errors);
    validate_postgres_pool_config(&parsed.pool, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{SDKWORK_IM_POSTGRES_CONFIG_ENV} has invalid PostgreSQL settings: {}",
            errors.join("; ")
        ))
    }
}

fn require_non_empty_postgres_field(field: &'static str, value: &str, errors: &mut Vec<String>) {
    if value.trim().is_empty() {
        errors.push(format!("{field} must not be empty"));
    }
}

fn validate_postgres_sslmode(sslmode: &str, errors: &mut Vec<String>) {
    let normalized = sslmode.trim().to_ascii_lowercase();
    let supported = [
        "disable",
        "allow",
        "prefer",
        "require",
        "verify-ca",
        "verify-full",
    ];
    if !supported.contains(&normalized.as_str()) {
        errors.push(format!(
            "connection.sslmode must be one of {}; got `{}`",
            supported.join(", "),
            sslmode.trim()
        ));
    }
}

fn validate_postgres_pool_config(pool: &LocalMinimalPostgresPoolConfig, errors: &mut Vec<String>) {
    if matches!(pool.max_connections, Some(0)) {
        errors.push("pool.maxConnections must be greater than zero".to_owned());
    }
    if let (Some(min_connections), Some(max_connections)) =
        (pool.min_connections, pool.max_connections)
        && min_connections > max_connections
    {
        errors.push(format!(
            "pool.minConnections ({min_connections}) must be less than or equal to pool.maxConnections ({max_connections})"
        ));
    }
}

fn resolve_postgres_password_file_path(config_dir: &StdPath, password_file: &str) -> PathBuf {
    let password_path = PathBuf::from(password_file);
    if password_path.is_absolute() {
        return password_path;
    }
    let config_root = config_dir.parent().unwrap_or(config_dir);
    config_root.join(password_path)
}

fn render_postgres_key_value_connection_string(
    connection: &LocalMinimalPostgresConnectionConfig,
    password: &str,
) -> String {
    let mut parts = vec![
        render_postgres_kv("host", connection.host.trim()),
        format!("port={}", connection.port),
        render_postgres_kv("dbname", connection.database.trim()),
        render_postgres_kv("user", connection.username.trim()),
        render_postgres_kv("password", password),
        render_postgres_kv("sslmode", connection.sslmode.trim()),
    ];
    if let Some(application_name) = connection.application_name.as_deref()
        && !application_name.trim().is_empty()
    {
        parts.push(render_postgres_kv(
            "application_name",
            application_name.trim(),
        ));
    }
    if let Some(timeout_seconds) = connection.connect_timeout_seconds {
        parts.push(format!("connect_timeout={timeout_seconds}"));
    }
    parts.join(" ")
}

fn render_postgres_kv(key: &str, value: &str) -> String {
    let escaped = value
        .chars()
        .flat_map(|character| match character {
            '\\' => ['\\', '\\'].into_iter().collect::<Vec<_>>(),
            '\'' => ['\\', '\''].into_iter().collect::<Vec<_>>(),
            character => [character].into_iter().collect::<Vec<_>>(),
        })
        .collect::<String>();
    format!("{key}='{escaped}'")
}

fn default_postgres_port() -> u16 {
    5432
}

fn default_postgres_sslmode() -> String {
    "prefer".to_owned()
}

fn normalize_realtime_storage_provider(value: Option<&str>) -> Option<String> {
    normalize_realtime_storage_value(value)
        .map(|value| value.to_ascii_lowercase().replace('_', "-"))
}

fn normalize_realtime_storage_value(value: Option<&str>) -> Option<String> {
    let normalized = value?.trim();
    if normalized.is_empty() {
        None
    } else {
        Some(normalized.to_owned())
    }
}

fn build_local_minimal_streaming_runtime(
    runtime_dir: impl AsRef<StdPath>,
) -> Arc<StreamingRuntime> {
    Arc::new(StreamingRuntime::with_store(Arc::new(
        FileStreamStateStore::new(runtime_dir.as_ref().join("state").join("stream-state.json")),
    )))
}

fn build_local_minimal_call_runtime(runtime_dir: impl AsRef<StdPath>) -> Arc<ImCallRuntime> {
    Arc::new(ImCallRuntime::with_store(Arc::new(
        FileImCallStateStore::new(runtime_dir.as_ref().join("state").join("rtc-state.json")),
    )))
}

fn ensure_local_minimal_task_runtime_state_files(runtime_dir: &StdPath) -> Result<(), String> {
    let state_dir = runtime_dir.join("state");
    fs::create_dir_all(state_dir.as_path()).map_err(|error| {
        format!(
            "failed to create runtime state dir {}: {error}",
            state_dir.display()
        )
    })?;
    for (file_name, content) in [
        (
            "notification-tasks.json",
            "{\"by_notification\":{},\"tasks_by_recipient\":{}}\n",
        ),
        ("automation-executions.json", "{}\n"),
    ] {
        let file_path = state_dir.join(file_name);
        if file_path.exists() {
            continue;
        }
        fs::write(file_path.as_path(), content).map_err(|error| {
            format!(
                "failed to initialize runtime state file {}: {error}",
                file_path.display()
            )
        })?;
    }
    Ok(())
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
    _conversation_runtime: Arc<ConversationRuntime<ProjectionJournal>>,
    ops_runtime: Arc<OpsRuntime>,
    audit_runtime: Arc<AuditRuntime>,
    runtime_dir: Option<&StdPath>,
) -> (Router, Arc<SocialRuntime>) {
    let social_runtime = match runtime_dir {
        Some(runtime_dir) => Arc::new(SocialRuntime::from_runtime_dir(runtime_dir)),
        None => Arc::new(SocialRuntime::default()),
    };
    let social_router = social_service::build_app(social_runtime.clone());
    let governance_router =
        governance_service::build_control_surface_with_cluster_and_governance_sinks(
            realtime_cluster,
            ops_runtime,
            audit_runtime,
        );
    (governance_router.merge(social_router), social_runtime)
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
        build_default_principal_profile_provider(),
    )
}

pub fn build_app_with_dependencies_and_runtime_dir(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    runtime_dir: impl AsRef<StdPath>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
) -> Router {
    let runtime_dir = runtime_dir.as_ref().to_path_buf();
    ensure_local_minimal_task_runtime_state_files(runtime_dir.as_path())
        .unwrap_or_else(|error| panic!("failed to initialize local-minimal task state: {error}"));
    let projection_snapshot_stores =
        build_local_minimal_projection_snapshot_stores(runtime_dir.as_path());
    let realtime_scope_policy =
        realtime_policy::direct_chat_realtime_policy(projection_service.clone());
    let checkpoint_store = Arc::new(FileRealtimeCheckpointStore::new(
        runtime_dir
            .as_path()
            .join("state")
            .join("realtime-checkpoints.json"),
    ));
    let subscription_store = Arc::new(FileRealtimeSubscriptionStore::new(
        runtime_dir
            .as_path()
            .join("state")
            .join("realtime-subscriptions.json"),
    ));
    let event_window_store = Arc::new(FileRealtimeEventWindowStore::new(
        runtime_dir
            .as_path()
            .join("state")
            .join("realtime-event-windows.json"),
    ));
    let presence_state_store = Arc::new(FilePresenceStateStore::new(
        runtime_dir
            .as_path()
            .join("state")
            .join("presence-state.json"),
    ));
    let realtime_plane = RealtimePlaneAssembly::new(
        realtime_cluster,
        Arc::new(
            RealtimeDeliveryRuntime::with_durable_stores_and_scope_access_policy(
                checkpoint_store,
                subscription_store,
                event_window_store,
                realtime_scope_policy.clone(),
            ),
        ),
        Arc::new(PresenceRuntime::with_store(presence_state_store)),
    );
    let journal = ProjectionJournal::new_file(
        projection_service.clone(),
        runtime_dir
            .as_path()
            .join("state")
            .join("commit-journal.json"),
        projection_snapshot_stores,
    );
    build_app_with_dependencies_and_runtime_and_journal(
        node_id,
        bind_addr,
        Some(runtime_dir.clone()),
        projection_service.clone(),
        realtime_plane,
        journal.clone(),
        Some(realtime_scope_policy),
        build_local_minimal_streaming_runtime(runtime_dir.as_path()),
        build_local_minimal_call_runtime(runtime_dir.as_path()),
        build_local_minimal_notification_runtime(
            journal.clone(),
            runtime_dir.as_path(),
            projection_service,
        ),
        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
        build_default_principal_profile_provider(),
    )
}

fn build_app_with_dependencies_and_provider_ports(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    principal_profile_provider: Arc<dyn PrincipalProfileProvider>,
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
        Arc::new(ImCallRuntime::default()),
        Arc::new(NotificationRuntime::with_journal_and_projection(
            Arc::new(journal.clone()),
            projection_service,
        )),
        Arc::new(AutomationRuntime::with_journal(Arc::new(journal))),
        principal_profile_provider,
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
        Arc::new(ImCallRuntime::default()),
        Arc::new(NotificationRuntime::with_journal_and_projection(
            Arc::new(journal),
            projection_service,
        )),
        Arc::new(AutomationRuntime::default()),
        build_default_principal_profile_provider(),
    )
}

pub fn build_app_with_dependencies_realtime_and_notification_runtime(
    node_id: impl Into<String>,
    bind_addr: impl Into<String>,
    projection_service: Arc<TimelineProjectionService>,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    notification_runtime: Arc<NotificationRuntime>,
) -> Router {
    let journal = ProjectionJournal::new_memory(projection_service.clone());
    build_app_with_dependencies_and_runtime_and_journal(
        node_id,
        bind_addr,
        None,
        projection_service,
        RealtimePlaneAssembly::with_cluster_and_runtime(realtime_cluster, realtime_runtime),
        journal.clone(),
        None,
        Arc::new(StreamingRuntime::default()),
        Arc::new(ImCallRuntime::default()),
        notification_runtime,
        Arc::new(AutomationRuntime::default()),
        build_default_principal_profile_provider(),
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
    call_runtime: Arc<ImCallRuntime>,
    notification_runtime: Arc<NotificationRuntime>,
    automation_runtime: Arc<AutomationRuntime>,
    principal_profile_provider: Arc<dyn PrincipalProfileProvider>,
) -> Router {
    let node_id = node_id.into();
    let bind_addr = bind_addr.into();
    realtime_plane.bind_node_runtime(node_id.as_str());
    let realtime_cluster = realtime_plane.realtime_cluster();
    let presence_runtime = realtime_plane.presence_runtime();
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
            "governance-service".into(),
            "projection-service".into(),
            "media-service".into(),
            "streaming-service".into(),
            "im-calls-service".into(),
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
    let message_side_effect_outbox: Arc<dyn side_effect_outbox::MessageSideEffectOutboxStore> =
        match runtime_dir.as_ref() {
            Some(runtime_dir) => {
                Arc::new(side_effect_outbox::FileMessageSideEffectOutboxStore::new(
                    runtime_dir
                        .as_path()
                        .join("state")
                        .join("message-side-effect-outbox.json"),
                ))
            }
            None => Arc::new(side_effect_outbox::MemoryMessageSideEffectOutboxStore::default()),
        };
    let (control_plane_app, social_runtime) = build_local_minimal_control_plane_app(
        realtime_cluster.clone(),
        conversation_runtime.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.as_deref(),
    );
    if let Some(realtime_scope_policy) = realtime_scope_policy.as_ref() {
        realtime_scope_policy.bind_social_runtime(social_runtime.clone());
    }
    let client_route_registration = LocalNodeClientRouteRegistration::new(
        node_id.clone(),
        realtime_cluster.clone(),
        presence_runtime.clone(),
        realtime_runtime.clone(),
        projection_service.clone(),
        journal.snapshot_stores(),
    );
    let pending_friend_request_accept_repairs =
        social::load_pending_friend_request_accept_repairs(runtime_dir.as_deref());
    let state = AppState {
        node_id: node_id.clone(),
        runtime_dir,
        control_plane_app: control_plane_app.clone(),
        social_runtime,
        realtime_cluster,
        conversation_runtime,
        principal_profile_provider,
        projection_service,
        presence_runtime,
        realtime_runtime,
        client_route_registration,
        streaming_runtime,
        call_runtime,
        notification_runtime,
        automation_runtime,
        audit_runtime,
        ops_runtime,
        message_side_effect_outbox,
        conversation_preferences: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
        conversation_profiles: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
        contact_preferences: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
        contact_tags: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
        contact_recommendations: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
        message_visibility: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
        message_favorites: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
        projection_replay_state: journal.replay_state(),
        pending_friend_request_accept_repairs: Arc::new(std::sync::Mutex::new(
            pending_friend_request_accept_repairs,
        )),
        friend_request_accept_repair_gate: Arc::new(tokio::sync::Mutex::new(())),
        search_provider: None,
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
            tracing::warn!(
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
    let rebuild_happened = restore_summary.restored_client_route_sync
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
                tracing::warn!(
                    "failed to replay projection event {} during local-minimal startup: {error:?}. continuing bootstrap in degraded replay mode",
                    envelope.event_id
                );
            } else {
                replayed_event_count += 1;
            }
        }

        if let Err(error) = conversation_runtime.apply_recovered_envelope(envelope) {
            tracing::warn!(
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
        .route(IM_OPENAPI_SCHEMA_PATH, get(export_im_openapi_schema))
        .route(
            APP_API_OPENAPI_SCHEMA_PATH,
            get(export_app_api_openapi_schema),
        )
        .route(
            BACKEND_API_OPENAPI_SCHEMA_PATH,
            get(export_backend_api_openapi_schema),
        )
        .nest("/im/v3/api", im_standard_api_routes())
        .route_service(
            "/app/v3/api/media/provider_health",
            media_service::build_default_app(),
        )
        .route(
            "/app/v3/api/principal/profiles/provider_health",
            get(retrieve_principal_profile_health),
        )
        .nest("/backend/v3/api", backend_api_routes())
        .with_state(state)
}

async fn retrieve_principal_profile_health(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<im_platform_contracts::ProviderHealthSnapshot>, ApiError> {
    let _auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(
        state.principal_profile_provider.provider_health_snapshot(),
    ))
}

fn im_standard_api_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/presence/heartbeat",
            post(presence_routes::heartbeat_presence),
        )
        .route("/presence/me", get(presence_routes::get_presence_me))
        .route(
            "/realtime/subscriptions/sync",
            post(presence_routes::sync_realtime_subscriptions),
        )
        .route("/realtime/ws", get(presence_routes::realtime_websocket))
        .route(
            "/realtime/events/ack",
            post(presence_routes::ack_realtime_events),
        )
        .route(
            "/realtime/events",
            get(presence_routes::list_realtime_events),
        )
        .route("/social/users", get(social::list_social_users))
        .route(
            "/social/friend_requests",
            get(social::list_friend_requests).post(social::submit_friend_request),
        )
        .route(
            "/social/friend_requests/{request_id}/accept",
            post(social::accept_friend_request),
        )
        .route(
            "/social/friend_requests/{request_id}/decline",
            post(social::decline_friend_request),
        )
        .route(
            "/social/friend_requests/{request_id}/cancel",
            post(social::cancel_friend_request),
        )
        .route(
            "/social/friendships/{friendship_id}/remove",
            post(social::remove_friendship),
        )
        .route(
            "/social/contacts/tags",
            get(projection::list_contact_tags).post(projection::create_contact_tag),
        )
        .route(
            "/social/contacts/tags/{tag_id}",
            patch(projection::update_contact_tag).delete(projection::delete_contact_tag),
        )
        .route(
            "/social/contacts/{target_user_id}/recommendations",
            post(projection::create_contact_recommendation),
        )
        .route(
            "/social/contacts/{target_user_id}/preferences",
            get(projection::get_contact_preferences).patch(projection::update_contact_preferences),
        )
        .route("/chat/contacts", get(projection::get_contacts))
        .route("/chat/inbox", get(projection::get_inbox))
        .route(
            "/chat/conversations",
            post(conversation::create_conversation),
        )
        .route(
            "/chat/conversations/agent_dialogs",
            post(conversation::create_agent_dialog),
        )
        .route(
            "/chat/conversations/agent_handoffs",
            post(conversation::create_agent_handoff),
        )
        .route(
            "/chat/conversations/system_channels",
            post(conversation::create_system_channel),
        )
        .route(
            "/chat/conversations/threads",
            post(conversation::create_thread_conversation),
        )
        .route(
            "/chat/conversations/direct_chats/bindings",
            post(conversation::bind_direct_chat_conversation),
        )
        .route(
            "/chat/conversations/{conversation_id}/agent_handoff",
            get(handoff::get_agent_handoff_state),
        )
        .route(
            "/chat/conversations/{conversation_id}/agent_handoff/accept",
            post(handoff::accept_agent_handoff),
        )
        .route(
            "/chat/conversations/{conversation_id}/agent_handoff/resolve",
            post(handoff::resolve_agent_handoff),
        )
        .route(
            "/chat/conversations/{conversation_id}/agent_handoff/close",
            post(handoff::close_agent_handoff),
        )
        .route(
            "/chat/conversations/{conversation_id}",
            get(projection::get_conversation_summary),
        )
        .route(
            "/chat/conversations/{conversation_id}/members",
            get(membership::list_members),
        )
        .route(
            "/chat/conversations/{conversation_id}/members/add",
            post(membership::add_member),
        )
        .route(
            "/chat/conversations/{conversation_id}/members/remove",
            post(membership::remove_member),
        )
        .route(
            "/chat/conversations/{conversation_id}/members/transfer_owner",
            post(membership::transfer_conversation_owner),
        )
        .route(
            "/chat/conversations/{conversation_id}/members/change_role",
            post(membership::change_conversation_member_role),
        )
        .route(
            "/chat/conversations/{conversation_id}/members/leave",
            post(membership::leave_conversation),
        )
        .route(
            "/chat/conversations/{conversation_id}/preferences",
            get(projection::get_conversation_preferences)
                .patch(projection::update_conversation_preferences),
        )
        .route(
            "/chat/conversations/{conversation_id}/profile",
            get(projection::get_conversation_profile)
                .patch(projection::update_conversation_profile),
        )
        .route(
            "/chat/conversations/{conversation_id}/read_cursor",
            get(projection::get_read_cursor).post(projection::update_read_cursor),
        )
        .route(
            "/chat/conversations/{conversation_id}/member_directory",
            get(projection::get_member_directory),
        )
        .route(
            "/chat/conversations/{conversation_id}/messages",
            get(projection::get_timeline).post(message::post_message),
        )
        .route("/chat/messages/search", get(message::search_messages))
        .route(
            "/chat/conversations/{conversation_id}/system_channel/publish",
            post(message::publish_system_channel_message),
        )
        .route(
            "/chat/conversations/{conversation_id}/pins",
            get(projection::get_pinned_messages),
        )
        .route(
            "/chat/conversations/{conversation_id}/messages/{message_id}/interaction_summary",
            get(projection::get_message_interaction_summary),
        )
        .route(
            "/chat/messages/{message_id}/edit",
            post(message::edit_message),
        )
        .route(
            "/chat/messages/{message_id}/recall",
            post(message::recall_message),
        )
        .route(
            "/chat/messages/favorites",
            get(message::list_message_favorites),
        )
        .route(
            "/chat/messages/{message_id}/favorites",
            post(message::create_message_favorite),
        )
        .route(
            "/chat/messages/favorites/{favorite_id}",
            delete(message::delete_message_favorite),
        )
        .route(
            "/chat/messages/{message_id}/visibility",
            delete(message::delete_message_visibility),
        )
        .route(
            "/chat/messages/{message_id}/reactions",
            post(message::add_message_reaction),
        )
        .route(
            "/chat/messages/{message_id}/reactions/remove",
            post(message::remove_message_reaction),
        )
        .route(
            "/chat/messages/{message_id}/pin",
            post(message::pin_message),
        )
        .route(
            "/chat/messages/{message_id}/unpin",
            post(message::unpin_message),
        )
        .route("/streams", post(stream::open_stream))
        .route(
            "/streams/{stream_id}/frames",
            get(stream::list_stream_frames).post(stream::append_stream_frame),
        )
        .route(
            "/streams/{stream_id}/checkpoint",
            post(stream::checkpoint_stream),
        )
        .route(
            "/streams/{stream_id}/complete",
            post(stream::complete_stream),
        )
        .route("/streams/{stream_id}/abort", post(stream::abort_stream))
        .nest("/calls", im_calls_routes())
}

fn im_calls_routes() -> Router<AppState> {
    Router::new()
        .route("/sessions", post(calls::create_rtc_session))
        .route("/sessions/{rtc_session_id}", get(calls::get_rtc_session))
        .route(
            "/sessions/{rtc_session_id}/invite",
            post(calls::invite_rtc_session),
        )
        .route(
            "/sessions/{rtc_session_id}/accept",
            post(calls::accept_rtc_session),
        )
        .route(
            "/sessions/{rtc_session_id}/reject",
            post(calls::reject_rtc_session),
        )
        .route(
            "/sessions/{rtc_session_id}/end",
            post(calls::end_rtc_session),
        )
        .route(
            "/sessions/{rtc_session_id}/signals",
            post(calls::post_rtc_signal),
        )
        .route(
            "/sessions/{rtc_session_id}/credentials",
            post(calls::issue_rtc_participant_credential),
        )
}

fn backend_api_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/automation/governance",
            get(platform::get_automation_governance),
        )
        .route(
            "/audit/records",
            get(platform::list_audit_records).post(platform::record_audit_anchor),
        )
        .route("/audit/export", get(platform::export_audit_bundle))
        .route("/ops/health", get(platform::get_ops_health))
        .route("/ops/cluster", get(platform::get_ops_cluster))
        .route("/ops/lag", get(platform::get_ops_lag))
        .route("/ops/replay_status", get(platform::get_ops_replay_status))
        .route(
            "/ops/commercial_readiness",
            get(platform::get_ops_commercial_readiness),
        )
        .route("/ops/runtime_dir", get(platform::get_ops_runtime_dir))
        .route(
            "/ops/provider_bindings",
            get(platform::get_ops_provider_bindings),
        )
        .route(
            "/ops/provider_bindings/drift",
            get(platform::get_ops_provider_binding_drift),
        )
        .route("/ops/diagnostics", get(platform::get_ops_diagnostics))
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_gateway::StandaloneRealtimeScopeAccessPolicy;

    #[test]
    fn test_resolve_local_minimal_realtime_storage_config_defaults_to_local_disk() {
        let config = resolve_local_minimal_realtime_storage_config(None, None, None)
            .expect("missing storage provider should keep the local development default");

        assert_eq!(
            config.provider,
            LocalMinimalRealtimeStorageProvider::LocalDisk
        );
        assert!(config.postgres.is_none());
    }

    #[test]
    fn test_resolve_local_minimal_realtime_storage_config_rejects_postgres_without_database_url() {
        let error = resolve_local_minimal_realtime_storage_config(Some("postgresql"), None, None)
            .expect_err("postgresql realtime storage must not silently fall back to local disk");

        assert!(
            error.contains(SDKWORK_IM_DATABASE_URL_ENV),
            "missing PostgreSQL database URL should name the executable runtime env var: {error}"
        );
        assert!(
            error.contains(SDKWORK_IM_POSTGRES_CONFIG_ENV),
            "missing PostgreSQL database URL should explain why config-only input is insufficient: {error}"
        );
    }

    #[test]
    fn test_resolve_local_minimal_realtime_storage_config_builds_postgres_config_from_database_url()
    {
        let config = resolve_local_minimal_realtime_storage_config(
            Some(" PostgreSQL "),
            Some(" postgres://chat_user:chat_pass@localhost:5432/chat "),
            None,
        )
        .expect("postgresql storage provider should accept an explicit database URL");

        assert_eq!(
            config.provider,
            LocalMinimalRealtimeStorageProvider::Postgresql
        );
        let postgres = config
            .postgres
            .expect("postgresql storage config should include adapter config");
        assert_eq!(
            postgres.database_url(),
            "postgres://chat_user:chat_pass@localhost:5432/chat"
        );
    }

    #[test]
    fn test_resolve_local_minimal_realtime_storage_config_builds_postgres_config_from_yaml_file() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        let config_root = std::env::temp_dir().join(format!("sdkwork_im_postgres_config_{unique}"));
        let storage_dir = config_root.join("storage");
        let secrets_dir = config_root.join("secrets");
        std::fs::create_dir_all(&storage_dir).expect("storage dir should be created");
        std::fs::create_dir_all(&secrets_dir).expect("secrets dir should be created");
        std::fs::write(secrets_dir.join("postgresql.password"), "demo-secret\n")
            .expect("password file should be written");
        let postgres_config_path = storage_dir.join("postgresql.yaml");
        std::fs::write(
            &postgres_config_path,
            r#"provider: postgresql
connection:
  host: 127.0.0.1
  port: 15432
  database: sdkwork_im
  username: sdkwork_im_app
  passwordFile: ./secrets/postgresql.password
  sslmode: require
  applicationName: sdkwork-im-server
  connectTimeoutSeconds: 10
pool:
  minConnections: 5
  maxConnections: 30
"#,
        )
        .expect("postgresql config should be written");

        let config = resolve_local_minimal_realtime_storage_config(
            Some("postgresql"),
            None,
            Some(
                postgres_config_path
                    .to_str()
                    .expect("postgres config path should be utf-8"),
            ),
        )
        .expect("postgresql storage provider should accept the packaged postgresql.yaml contract");

        assert_eq!(
            config.provider,
            LocalMinimalRealtimeStorageProvider::Postgresql
        );
        let postgres = config
            .postgres
            .expect("postgresql storage config should include adapter config");
        assert_eq!(postgres.pool_max_size(), 30);
        assert_eq!(postgres.pool_min_idle(), Some(5));
        let connection = postgres.database_url();
        for expected in [
            "host='127.0.0.1'",
            "port=15432",
            "dbname='sdkwork_im'",
            "user='sdkwork_im_app'",
            "password='demo-secret'",
            "sslmode='require'",
            "application_name='sdkwork-im-server'",
            "connect_timeout=10",
        ] {
            assert!(
                connection.contains(expected),
                "generated PostgreSQL connection string should contain `{expected}`, got: {connection}"
            );
        }

        let _ = std::fs::remove_dir_all(config_root);
    }

    #[test]
    fn test_resolve_local_minimal_realtime_storage_config_rejects_unknown_provider() {
        let error = resolve_local_minimal_realtime_storage_config(Some("memory"), None, None)
            .expect_err("unknown realtime storage providers should fail closed");

        assert!(
            error.contains(SDKWORK_IM_STORAGE_PROVIDER_ENV) && error.contains("memory"),
            "unknown provider error should be actionable: {error}"
        );
    }

    #[test]
    fn test_postgres_key_value_connection_string_escapes_password_quotes_and_backslashes() {
        let connection = LocalMinimalPostgresConnectionConfig {
            host: "127.0.0.1".into(),
            port: 5432,
            database: "sdkwork_im".into(),
            username: "sdkwork_im_app".into(),
            password_file: "unused".into(),
            sslmode: "require".into(),
            application_name: Some("sdkwork-im-server".into()),
            connect_timeout_seconds: Some(10),
        };

        let rendered = render_postgres_key_value_connection_string(&connection, r#"pa'ss\word"#);

        assert!(
            rendered.contains(r#"password='pa\'ss\\word'"#),
            "PostgreSQL key/value strings must escape password quotes and backslashes, got: {rendered}"
        );
    }

    #[test]
    fn test_resolve_local_minimal_realtime_storage_config_rejects_unsafe_postgres_yaml_values() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        let config_root =
            std::env::temp_dir().join(format!("sdkwork_im_postgres_invalid_config_{unique}"));
        let storage_dir = config_root.join("storage");
        let secrets_dir = config_root.join("secrets");
        std::fs::create_dir_all(&storage_dir).expect("storage dir should be created");
        std::fs::create_dir_all(&secrets_dir).expect("secrets dir should be created");
        std::fs::write(secrets_dir.join("postgresql.password"), "demo-secret\n")
            .expect("password file should be written");
        let postgres_config_path = storage_dir.join("postgresql.yaml");
        std::fs::write(
            &postgres_config_path,
            r#"provider: postgresql
connection:
  host: "   "
  database: sdkwork_im
  username: sdkwork_im_app
  passwordFile: ./secrets/postgresql.password
  sslmode: trust-me
pool:
  minConnections: 31
  maxConnections: 30
"#,
        )
        .expect("postgresql config should be written");

        let error = resolve_local_minimal_realtime_storage_config(
            Some("postgresql"),
            None,
            Some(
                postgres_config_path
                    .to_str()
                    .expect("postgres config path should be utf-8"),
            ),
        )
        .expect_err("unsafe PostgreSQL yaml values must fail before pool construction");

        assert!(
            error.contains("connection.host"),
            "empty host should be rejected with a field-specific error: {error}"
        );
        assert!(
            error.contains("connection.sslmode"),
            "unsupported sslmode should be rejected with a field-specific error: {error}"
        );
        assert!(
            error.contains("pool.minConnections") && error.contains("pool.maxConnections"),
            "pool min/max inversion should be rejected before r2d2 clamps values: {error}"
        );

        let _ = std::fs::remove_dir_all(config_root);
    }

    #[test]
    fn test_resolve_local_minimal_realtime_storage_config_rejects_zero_postgres_port_and_timeout() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        let config_root =
            std::env::temp_dir().join(format!("sdkwork_im_postgres_zero_config_{unique}"));
        let storage_dir = config_root.join("storage");
        let secrets_dir = config_root.join("secrets");
        std::fs::create_dir_all(&storage_dir).expect("storage dir should be created");
        std::fs::create_dir_all(&secrets_dir).expect("secrets dir should be created");
        std::fs::write(secrets_dir.join("postgresql.password"), "demo-secret\n")
            .expect("password file should be written");
        let postgres_config_path = storage_dir.join("postgresql.yaml");
        std::fs::write(
            &postgres_config_path,
            r#"provider: postgresql
connection:
  host: 127.0.0.1
  port: 0
  database: sdkwork_im
  username: sdkwork_im_app
  passwordFile: ./secrets/postgresql.password
  sslmode: prefer
  connectTimeoutSeconds: 0
pool:
  minConnections: 0
  maxConnections: 30
"#,
        )
        .expect("postgresql config should be written");

        let error = resolve_local_minimal_realtime_storage_config(
            Some("postgresql"),
            None,
            Some(
                postgres_config_path
                    .to_str()
                    .expect("postgres config path should be utf-8"),
            ),
        )
        .expect_err("zero PostgreSQL port and timeout must fail before pool construction");

        assert!(
            error.contains("connection.port"),
            "zero PostgreSQL port should be rejected with a field-specific error: {error}"
        );
        assert!(
            error.contains("connection.connectTimeoutSeconds"),
            "zero PostgreSQL connect timeout should be rejected with a field-specific error: {error}"
        );
        assert!(
            !error.contains("demo-secret"),
            "configuration validation errors must not leak secrets: {error}"
        );

        let _ = std::fs::remove_dir_all(config_root);
    }

    #[test]
    fn test_try_build_local_minimal_realtime_plane_rejects_postgres_without_database_url() {
        let policy = Arc::new(StandaloneRealtimeScopeAccessPolicy);
        let result = try_build_local_minimal_realtime_plane_with_storage_config(
            std::env::temp_dir(),
            policy,
            LocalMinimalRealtimeStorageConfig {
                provider: LocalMinimalRealtimeStorageProvider::Postgresql,
                postgres: None,
            },
        );
        let error = match result {
            Ok(_) => panic!("postgresql realtime storage must not build without executable config"),
            Err(error) => error,
        };

        assert!(
            error.contains("postgresql realtime storage config is missing"),
            "missing runtime adapter config should fail before any local-disk fallback: {error}"
        );
    }
}
