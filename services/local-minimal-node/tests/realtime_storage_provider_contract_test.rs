use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn test_local_minimal_realtime_plane_selects_postgres_adapter_for_postgresql_provider() {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path).unwrap_or_else(|_| {
        panic!(
            "local-minimal-node manifest should exist at {}",
            manifest_path.display()
        )
    });
    let build_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/node/build.rs");
    let build_source = fs::read_to_string(&build_path).unwrap_or_else(|_| {
        panic!(
            "local-minimal-node build module should exist at {}",
            build_path.display()
        )
    });

    assert!(
        manifest.contains("im-adapters-postgres-realtime"),
        "local-minimal-node must depend on the PostgreSQL realtime adapter before a postgresql provider can be selected"
    );
    assert!(
        build_source.contains("LocalMinimalRealtimeStorageProvider"),
        "local-minimal realtime storage must use an explicit provider enum instead of ad hoc env checks"
    );
    assert!(
        build_source.contains("resolve_local_minimal_realtime_storage_config_from_env"),
        "local-minimal realtime storage must have a single env-to-config resolver"
    );
    assert!(
        build_source.contains("CRAW_CHAT_STORAGE_PROVIDER_ENV"),
        "runtime storage selection must use the standardized storage provider env name"
    );
    assert!(
        build_source.contains("CRAW_CHAT_DATABASE_URL_ENV"),
        "PostgreSQL runtime storage must use the standardized database URL env name"
    );
    assert!(
        build_source.contains("PostgresRealtimeConfig"),
        "PostgreSQL provider selection must construct the driver-backed realtime adapter config"
    );
    assert!(
        build_source.contains("PostgresRealtimeCheckpointStore::from_pool")
            && build_source.contains("PostgresRealtimeSubscriptionStore::from_pool")
            && build_source.contains("PostgresRealtimeEventWindowStore::from_pool")
            && build_source.contains("PostgresRealtimeDisconnectFenceStore::from_pool")
            && build_source.contains("PostgresRealtimePresenceStateStore::from_pool"),
        "all realtime and presence stores must be backed by the same PostgreSQL realtime pool for the postgresql provider"
    );
    let postgres_builder = build_source
        .split("fn build_local_minimal_postgres_realtime_plane")
        .nth(1)
        .and_then(|tail| {
            tail.split("fn resolve_local_minimal_realtime_storage_config_from_env")
                .next()
        })
        .expect("postgres realtime plane builder should exist");
    assert!(
        !postgres_builder.contains("FilePresenceStateStore"),
        "postgresql provider must not keep presence on local files because production reconnect state must survive node restart and multi-node routing"
    );
}

#[test]
fn test_commercial_gate_runs_local_minimal_realtime_storage_provider_contract() {
    let workflow_path = repo_root().join(".github/workflows/im-commercial-gates.yml");
    let workflow = fs::read_to_string(&workflow_path).unwrap_or_else(|_| {
        panic!(
            "commercial gate workflow should exist at {}",
            workflow_path.display()
        )
    });

    assert!(
        workflow.contains(
            "cargo test -p local-minimal-node --test realtime_storage_provider_contract_test"
        ),
        "commercial gate workflow must include local-minimal realtime storage provider contract tests"
    );
    assert!(
        workflow.contains(
            "cargo clippy -p im-postgres-realtime-contracts -p im-adapters-postgres-realtime -p local-minimal-node --tests -- -D warnings"
        ),
        "commercial gate workflow must lint PostgreSQL realtime wiring together with local-minimal-node"
    );
}
