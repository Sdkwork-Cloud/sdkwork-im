use std::collections::BTreeSet;
use std::sync::Arc;

use im_adapters_local_memory::MemoryPresenceStateStore;
use im_auth_context::AuthContext;

fn demo_auth(session_id: &str, device_id: &str) -> AuthContext {
    AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some(session_id.into()),
        device_id: Some(device_id.into()),
        permissions: BTreeSet::new(),
    }
}

#[test]
fn test_runtime_restores_presence_as_offline_and_requires_fresh_resume_after_rebuild() {
    let presence_store = Arc::new(MemoryPresenceStateStore::default());
    let runtime_before =
        session_gateway::SessionPresenceRuntime::with_store(presence_store.clone());
    runtime_before
        .register_device("t_demo", "u_demo", "d_phone")
        .expect("phone registration should persist presence inventory");
    runtime_before
        .register_device("t_demo", "u_demo", "d_pad")
        .expect("pad registration should persist presence inventory");

    let resumed = runtime_before
        .resume(
            &demo_auth("s_before", "d_pad"),
            "d_pad".into(),
            0,
            7,
            vec!["d_pad".into(), "d_phone".into()],
        )
        .expect("initial resume should succeed");
    assert_eq!(resumed.presence.devices[0].status.as_str(), "online");

    let runtime_after = session_gateway::SessionPresenceRuntime::with_store(presence_store);

    let restored = runtime_after
        .presence_snapshot("t_demo", "u_demo", Some("d_pad".into()), Vec::new())
        .expect("presence snapshot should restore after rebuild");
    assert_eq!(restored.devices.len(), 2);
    assert_eq!(restored.devices[0].device_id, "d_pad");
    assert_eq!(restored.devices[0].status.as_str(), "offline");
    assert!(restored.devices[0].last_resume_at.is_some());
    assert!(restored.devices[0].last_seen_at.is_some());
    assert_eq!(restored.devices[1].device_id, "d_phone");
    assert_eq!(restored.devices[1].status.as_str(), "offline");

    let stale_heartbeat = runtime_after.heartbeat(
        &demo_auth("s_before", "d_pad"),
        "d_pad".into(),
        7,
        vec!["d_pad".into(), "d_phone".into()],
    );
    let stale_error = stale_heartbeat.expect_err("stale pre-restart heartbeat should be rejected");
    assert_eq!(stale_error.code(), "reconnect_required");

    let resumed_after = runtime_after
        .resume(
            &demo_auth("s_after", "d_pad"),
            "d_pad".into(),
            7,
            7,
            vec!["d_pad".into(), "d_phone".into()],
        )
        .expect("fresh resume after rebuild should succeed");
    assert_eq!(resumed_after.presence.devices[0].status.as_str(), "online");
    assert_eq!(
        resumed_after.presence.devices[0].session_id.as_deref(),
        Some("s_after")
    );
}

#[test]
fn test_presence_runtime_resume_returns_incremental_sync_window_from_runtime_link_owner() {
    let runtime = session_gateway::SessionPresenceRuntime::default();
    runtime
        .register_device("t_demo", "u_demo", "d_pad")
        .expect("device registration should seed presence state");

    let resumed = runtime
        .resume(
            &demo_auth("s_demo", "d_pad"),
            "d_pad".into(),
            4,
            9,
            vec!["d_pad".into()],
        )
        .expect("resume should succeed");

    assert!(resumed.resume_required);
    assert_eq!(resumed.resume_from_sync_seq, 5);
    assert_eq!(resumed.latest_sync_seq, 9);
}
