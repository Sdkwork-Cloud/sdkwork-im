"""Fix build.rs: ensure all PR1-14 parameters are wired correctly."""
with open('services/local-minimal-node/src/node/build.rs', 'r') as f:
    c = f.read()

# Track what's been added
changes = []

# 1. Add imports (check if already present)
if 'use im_adapters_postgres_journal::{' not in c:
    c = c.replace(
        'use im_adapters_postgres_realtime::{',
        'use im_adapters_postgres_journal::{\n    PostgresAggregateStore, PostgresJournalConfig, PostgresMessageStore,\n    PostgresOutboxStore, PostgresSearchProvider,\n};\nuse im_adapters_postgres_realtime::{'
    )
    changes.append('postgres_journal imports')

if 'use im_adapters_redis_cache::{' not in c:
    c = c.replace(
        'use im_platform_contracts::{',
        'use im_adapters_redis_cache::{RedisRealtimeCheckpointStore, RedisRealtimeEventWindowStore, RedisSeqAllocator};\nuse im_platform_contracts::{'
    )
    changes.append('redis_cache imports')

if 'ConversationAggregateStore' not in c:
    c = c.replace(
        'use im_platform_contracts::{IdGenerator, MessageStore, OutboxStore};',
        'use im_platform_contracts::{ConversationAggregateStore, ConversationSeqAllocator, IdGenerator, MessageStore, OutboxStore, SearchProvider};'
    )
    changes.append('contract imports')

if 'use sdkwork_im_runtime_id::' not in c:
    c = c.replace(
        'use serde::Deserialize;',
        'use sdkwork_im_runtime_id::RuntimeSnowflakeIdGenerator;\nuse serde::Deserialize;'
    )
    changes.append('runtime_id import')

# 2. Add type alias after the Postgres plane function
if 'type PostgresConversationStores' not in c:
    c = c.replace(
        '    ))\n}\n\nfn resolve_local_minimal_realtime_storage_config_from_env()',
        '    ))\n}\n\n/// Bundle of optional PostgreSQL conversation stores.\ntype PostgresConversationStores = (\n    Arc<dyn MessageStore>,\n    Arc<dyn OutboxStore>,\n    Arc<dyn IdGenerator>,\n    Arc<dyn ConversationAggregateStore>,\n    Option<Arc<dyn ConversationSeqAllocator>>,\n    Arc<dyn SearchProvider>,\n);\n\nfn resolve_local_minimal_realtime_storage_config_from_env()'
    )
    changes.append('type alias')

# 3. Add helper functions
if 'fn try_build_postgres_conversation_stores' not in c:
    helper = '''
#[allow(clippy::type_complexity)]
fn try_build_postgres_conversation_stores(
) -> Result<Option<PostgresConversationStores>, String> {
    let storage_provider = std::env::var(SDKWORK_IM_STORAGE_PROVIDER_ENV).ok();
    let is_postgres = storage_provider
        .as_deref()
        .map(|s| s.trim().eq_ignore_ascii_case("postgresql"))
        .unwrap_or(false);
    if !is_postgres {
        return Ok(None);
    }
    let database_url = std::env::var(SDKWORK_IM_DATABASE_URL_ENV)
        .ok()
        .map(|url| url.trim().to_owned())
        .filter(|url| !url.is_empty());
    let config = if let Some(url) = database_url {
        PostgresJournalConfig::new(url)
    } else {
        let path = std::env::var(SDKWORK_IM_POSTGRES_CONFIG_ENV)
            .map_err(|_| format!("{SDKWORK_IM_STORAGE_PROVIDER_ENV}=postgresql requires {SDKWORK_IM_DATABASE_URL_ENV} or {SDKWORK_IM_POSTGRES_CONFIG_ENV}"))?
            .trim().to_owned();
        let config_bytes = std::fs::read(&path).map_err(|e| format!("failed to read config at {path}: {e}"))?;
        let parsed: LocalMinimalPostgresConfigFile = serde_yaml::from_slice(&config_bytes).map_err(|e| format!("parse error: {e}"))?;
        if !parsed.provider.trim().eq_ignore_ascii_case("postgresql") {
            return Err(format!("provider must be postgresql"));
        }
        let password = std::fs::read_to_string(&parsed.connection.password_file)
            .map_err(|e| format!("read password error: {e}"))?;
        let database_url = format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode={}",
            parsed.connection.username, password.trim(), parsed.connection.host,
            parsed.connection.port, parsed.connection.database, parsed.connection.sslmode,
        );
        let mut config = PostgresJournalConfig::new(database_url);
        if let Some(mx) = parsed.pool.max_connections { config = config.with_pool_max_size(mx); }
        if let Some(mn) = parsed.pool.min_connections { config = config.with_pool_min_idle(mn); }
        config
    };
    let pool = config.connect_pool().map_err(|e| format!("pool: {e:?}"))?;
    let message_store: Arc<dyn MessageStore> = Arc::new(PostgresMessageStore::from_pool(pool.clone()));
    let outbox_store: Arc<dyn OutboxStore> = Arc::new(PostgresOutboxStore::from_pool(pool.clone()));
    let id_generator: Arc<dyn IdGenerator> = Arc::new(
        RuntimeSnowflakeIdGenerator::from_env().map_err(|e| format!("snowflake: {e}"))?,
    );
    let aggregate_store: Arc<dyn ConversationAggregateStore> = Arc::new(PostgresAggregateStore::from_pool(pool.clone()));
    let seq_allocator: Option<Arc<dyn ConversationSeqAllocator>> = None;
    let search_provider: Arc<dyn SearchProvider> = Arc::new(PostgresSearchProvider::from_pool(pool.clone()));
    Ok(Some((message_store, outbox_store, id_generator, aggregate_store, seq_allocator, search_provider)))
}
'''
    c = c.replace(
        'fn resolve_local_minimal_realtime_storage_config_from_env()',
        helper + '\nfn resolve_local_minimal_realtime_storage_config_from_env()'
    )
    changes.append('PG stores helper')

# 4. Modify function signature
old_sig = '''fn build_app_with_dependencies_and_runtime_and_journal(
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
    message_store: Option<Arc<dyn MessageStore>>,
    outbox_store: Option<Arc<dyn OutboxStore>>,
    id_generator: Option<Arc<dyn IdGenerator>>,
) -> Router {'''
new_sig = '''fn build_app_with_dependencies_and_runtime_and_journal(
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
    message_store: Option<Arc<dyn MessageStore>>,
    outbox_store: Option<Arc<dyn OutboxStore>>,
    id_generator: Option<Arc<dyn IdGenerator>>,
    aggregate_store: Option<Arc<dyn ConversationAggregateStore>>,
    seq_allocator: Option<Arc<dyn ConversationSeqAllocator>>,
    search_provider: Option<Arc<dyn SearchProvider>>,
) -> Router {'''
if old_sig in c:
    c = c.replace(old_sig, new_sig)
    changes.append('function signature')
else:
    changes.append('WARNING: signature NOT found')

# 5. Update ConversationRuntime construction
old_conv = '    let conversation_runtime = Arc::new(ConversationRuntime::new(journal.clone()));'
new_conv = '''    let conversation_runtime = {
        let mut builder = ConversationRuntime::new(journal.clone());
        if let Some(store) = message_store { builder = builder.with_message_store(store); }
        if let Some(store) = outbox_store { builder = builder.with_outbox_store(store); }
        if let Some(id_gen) = id_generator { builder = builder.with_id_generator(id_gen); }
        if let Some(ag) = aggregate_store { builder = builder.with_aggregate_store(ag); }
        if let Some(sq) = seq_allocator { builder = builder.with_seq_allocator(sq); }
        Arc::new(builder)
    };'''
if old_conv in c:
    c = c.replace(old_conv, new_conv)
    changes.append('ConversationRuntime')
else:
    changes.append('WARNING: ConversationRuntime NOT found')

# 6. Modify try_build_public_app_with_bind_addr_and_runtime_dir
old_try = '''    let realtime_plane = try_build_local_minimal_realtime_plane(
        runtime_dir.as_path(),
        realtime_scope_policy.clone(),
    )?;
    let journal = ProjectionJournal::new_file('''
new_try = '''    let realtime_plane = try_build_local_minimal_realtime_plane(
        runtime_dir.as_path(),
        realtime_scope_policy.clone(),
    )?;
    let conversation_stores = try_build_postgres_conversation_stores()?;
    let (message_store, outbox_store, id_generator, aggregate_store, seq_allocator, search_provider) = match conversation_stores {
        Some((ms, os, ig, ag, sq, sp)) => (Some(ms), Some(os), Some(ig), Some(ag), sq, Some(sp)),
        None => (None, None, None, None, None, None),
    };
    let journal = ProjectionJournal::new_file('''
if old_try in c:
    c = c.replace(old_try, new_try)
    changes.append('try_build stores')
else:
    changes.append('WARNING: try_build NOT found')

# 7. Update the function call in try_build to pass new params
old_call = '''        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
        build_default_principal_profile_provider(),
    )
    .layer(axum::extract::DefaultBodyLimit::max('''
new_call = '''        build_local_minimal_automation_runtime(journal, runtime_dir.as_path()),
        build_default_principal_profile_provider(),
        message_store,
        outbox_store,
        id_generator,
        aggregate_store,
        seq_allocator,
        search_provider,
    )
    .layer(axum::extract::DefaultBodyLimit::max('''
if old_call in c:
    c = c.replace(old_call, new_call)
    changes.append('try_build call')
else:
    changes.append('WARNING: try_build call NOT found')

# 8. Update AppState construction
c = c.replace(
    'friend_request_accept_repair_gate: Arc::new(tokio::sync::Mutex::new(())),\n    };',
    'friend_request_accept_repair_gate: Arc::new(tokio::sync::Mutex::new(())),\n        search_provider,\n    };'
)

# 9. Fix all callers - add 6 None args
# Find all call patterns ending with principal_profile_provider, followed by )
# and add 6 Nones

# Pattern for callers that use memory journal (no stores)
old_ending = '        build_default_principal_profile_provider(),\n    )\n}'
new_ending = '        build_default_principal_profile_provider(),\n        None,\n        None,\n        None,\n        None,\n        None,\n        None,\n    )\n}'
c = c.replace(old_ending, new_ending)

# Callers with principal_profile_provider (not build_default)
old_ending2 = '        principal_profile_provider,\n    )\n}'
new_ending2 = '        principal_profile_provider,\n        None,\n        None,\n        None,\n        None,\n        None,\n        None,\n    )\n}'
c = c.replace(old_ending2, new_ending2)

with open('services/local-minimal-node/src/node/build.rs', 'w') as f:
    f.write(c)

for ch in changes:
    print(f'  {ch}')
print('done')
