use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use craw_chat_runtime_route::{RouteBinding, RouteMigrationResult, RouteNodeLifecycle};
use im_adapters_local_memory::MemoryRealtimeCheckpointStore;
use im_platform_contracts::{ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore};
use session_gateway::{
    RealtimeClusterBridge, RealtimeDeliveryRuntime, RealtimeRuntimeError,
    RealtimeSubscriptionItemInput,
};

fn expect_ok<T>(result: Result<T, RealtimeRuntimeError>) -> T {
    result.expect("realtime runtime operation should succeed")
}

#[test]
fn test_cluster_bridge_routes_device_event_to_owner_node_runtime() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    expect_ok(runtime_b.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));
    cluster
        .bind_device_route("t_demo", "u_demo", "d_pad", "node_b", None, "websocket")
        .expect("route bind should succeed");

    let result = cluster.publish_device_event(
        "node_a",
        "t_demo",
        "u_demo",
        "d_pad",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
    );

    assert_eq!(result.target_node_id, "node_b");
    assert_eq!(result.route_state, "resolved");
    assert_eq!(result.delivered, 1);

    let owner_window = expect_ok(runtime_b.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(owner_window.items.len(), 1);
    assert_eq!(owner_window.items[0].event_type, "message.posted");

    let origin_window = expect_ok(runtime_a.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(origin_window.items.len(), 0);
}

#[test]
fn test_cluster_bridge_falls_back_to_origin_node_when_route_is_missing() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a.clone());

    expect_ok(runtime_a.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let result = cluster.publish_device_event(
        "node_a",
        "t_demo",
        "u_demo",
        "d_pad",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
    );

    assert_eq!(result.target_node_id, "node_a");
    assert_eq!(result.route_state, "local_fallback");
    assert_eq!(result.delivered, 1);

    let origin_window = expect_ok(runtime_a.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(origin_window.items.len(), 1);
}

#[test]
fn test_cluster_bridge_rejects_new_route_binds_when_node_is_draining() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a);

    cluster
        .bind_device_route(
            "t_demo",
            "u_demo",
            "d_existing",
            "node_a",
            None,
            "websocket",
        )
        .expect("existing route bind should succeed");

    let drain = cluster
        .mark_node_draining("node_a")
        .expect("drain should succeed");
    assert_eq!(drain.node_id, "node_a");
    assert_eq!(drain.drain_status, "draining");
    assert_eq!(drain.rebalance_state, "moving_routes");
    assert_eq!(drain.owned_route_count, 1);

    let error = cluster
        .bind_device_route("t_demo", "u_demo", "d_new", "node_a", None, "http")
        .expect_err("draining node should reject new route bind");
    assert_eq!(error.code, "node_draining");
    assert_eq!(error.node_id, "node_a");
    assert_eq!(error.drain_status, "draining");

    let preserved = cluster
        .resolve_device_route("t_demo", "u_demo", "d_existing")
        .expect("existing route should remain");
    assert_eq!(preserved.owner_node_id, "node_a");
}

#[test]
fn test_cluster_bridge_release_route_reconciles_draining_node_to_drained() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a);

    cluster
        .bind_device_route(
            "t_demo",
            "u_demo",
            "d_existing",
            "node_a",
            Some("s_demo"),
            "websocket",
        )
        .expect("existing route bind should succeed");

    let draining = cluster
        .mark_node_draining("node_a")
        .expect("drain should succeed");
    assert_eq!(draining.drain_status, "draining");
    assert_eq!(draining.rebalance_state, "moving_routes");
    assert_eq!(draining.owned_route_count, 1);

    let released = cluster
        .release_device_route("t_demo", "u_demo", "d_existing", "node_a")
        .expect("route should be released");
    assert_eq!(released.owner_node_id, "node_a");
    assert_eq!(released.session_id.as_deref(), Some("s_demo"));

    assert!(
        cluster
            .resolve_device_route("t_demo", "u_demo", "d_existing")
            .is_none(),
        "released route should be removed from the directory"
    );

    let lifecycle = cluster
        .node_lifecycle("node_a")
        .expect("node lifecycle should remain visible");
    assert_eq!(lifecycle.drain_status, "drained");
    assert_eq!(lifecycle.rebalance_state, "stable");
    assert_eq!(lifecycle.owned_route_count, 0);
}

#[test]
fn test_cluster_bridge_migrates_route_and_realtime_state_to_target_node() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    expect_ok(runtime_a.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into(), "message.edited".into()],
        }],
    ));
    cluster
        .bind_device_route("t_demo", "u_demo", "d_pad", "node_a", None, "websocket")
        .expect("initial route bind should succeed");

    let delivered = expect_ok(runtime_a.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_before_migrate"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(delivered, 1);

    cluster
        .mark_node_draining("node_a")
        .expect("drain should succeed");
    let migration = cluster
        .migrate_node_routes("node_a", "node_b")
        .expect("route migration should succeed");
    assert_eq!(migration.source_node_id, "node_a");
    assert_eq!(migration.target_node_id, "node_b");
    assert_eq!(migration.migrated_route_count, 1);
    assert_eq!(migration.source_drain_status, "drained");
    assert_eq!(migration.source_rebalance_state, "stable");
    assert_eq!(migration.target_drain_status, "active");
    assert_eq!(migration.target_rebalance_state, "stable");

    let migrated_route = cluster
        .resolve_device_route("t_demo", "u_demo", "d_pad")
        .expect("route should exist after migration");
    assert_eq!(migrated_route.owner_node_id, "node_b");

    let source_window = expect_ok(runtime_a.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(source_window.items.len(), 0);

    let target_window = expect_ok(runtime_b.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(target_window.items.len(), 1);
    assert_eq!(target_window.items[0].event_type, "message.posted");

    let target_checkpoint = expect_ok(runtime_b.window_checkpoint("t_demo", "u_demo", "d_pad"));
    assert_eq!(target_checkpoint.latest_realtime_seq, 1);
    assert_eq!(target_checkpoint.acked_through_seq, 0);
    assert_eq!(target_checkpoint.trimmed_through_seq, 0);

    let publish_result = cluster.publish_device_event(
        "node_a",
        "t_demo",
        "u_demo",
        "d_pad",
        "conversation",
        "c_demo",
        "message.edited",
        r#"{"messageId":"msg_after_migrate"}"#.into(),
    );
    assert_eq!(publish_result.target_node_id, "node_b");
    assert_eq!(publish_result.route_state, "resolved");
    assert_eq!(publish_result.delivered, 1);

    let target_window_after_publish =
        expect_ok(runtime_b.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(target_window_after_publish.items.len(), 2);
    assert_eq!(
        target_window_after_publish.items[1].event_type,
        "message.edited"
    );
}

#[test]
fn test_cluster_bridge_isolates_same_actor_id_across_principal_kinds_for_routes_and_publish() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    expect_ok(runtime_a.sync_subscriptions_for_principal_kind(
        "t_demo",
        "u_demo",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_user".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));
    expect_ok(runtime_b.sync_subscriptions_for_principal_kind(
        "t_demo",
        "u_demo",
        "agent",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_agent".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));
    cluster
        .bind_device_route_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "d_pad",
            "node_a",
            Some("s_user"),
            "websocket",
        )
        .expect("user route bind should succeed");
    cluster
        .bind_device_route_for_principal_kind(
            "t_demo",
            "u_demo",
            "agent",
            "d_pad",
            "node_b",
            Some("s_agent"),
            "websocket",
        )
        .expect("agent route bind should succeed");

    let user_route = cluster
        .resolve_device_route_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
        .expect("user route should exist");
    let agent_route = cluster
        .resolve_device_route_for_principal_kind("t_demo", "u_demo", "agent", "d_pad")
        .expect("agent route should exist");
    assert_eq!(user_route.owner_node_id, "node_a");
    assert_eq!(agent_route.owner_node_id, "node_b");

    let user_publish = cluster.publish_device_event_for_principal_kind(
        "node_b",
        "t_demo",
        "u_demo",
        "user",
        "d_pad",
        "conversation",
        "c_user",
        "message.posted",
        r#"{"messageId":"msg_user"}"#.into(),
    );
    let agent_publish = cluster.publish_device_event_for_principal_kind(
        "node_a",
        "t_demo",
        "u_demo",
        "agent",
        "d_pad",
        "conversation",
        "c_agent",
        "message.posted",
        r#"{"messageId":"msg_agent"}"#.into(),
    );
    assert_eq!(user_publish.target_node_id, "node_a");
    assert_eq!(user_publish.route_state, "resolved");
    assert_eq!(user_publish.delivered, 1);
    assert_eq!(agent_publish.target_node_id, "node_b");
    assert_eq!(agent_publish.route_state, "resolved");
    assert_eq!(agent_publish.delivered, 1);

    let user_window = expect_ok(
        runtime_a.list_events_for_principal_kind("t_demo", "u_demo", "user", "d_pad", 0, 10),
    );
    let agent_window = expect_ok(
        runtime_b.list_events_for_principal_kind("t_demo", "u_demo", "agent", "d_pad", 0, 10),
    );
    assert_eq!(user_window.items.len(), 1);
    assert_eq!(user_window.items[0].scope_id, "c_user");
    assert_eq!(agent_window.items.len(), 1);
    assert_eq!(agent_window.items[0].scope_id, "c_agent");
}

#[test]
fn test_cluster_disconnect_fence_isolated_by_principal_kind() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime);

    cluster
        .mark_device_disconnected_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "d_pad",
            Some("s_user"),
            "node_a",
        )
        .expect("user disconnect fence should persist");

    let user_error = cluster
        .ensure_device_resume_not_required_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
        .expect_err("user principal kind should require reconnect");
    assert_eq!(user_error.code, "reconnect_required");

    cluster
        .ensure_device_resume_not_required_for_principal_kind("t_demo", "u_demo", "agent", "d_pad")
        .expect("agent principal kind should remain isolated from user disconnect fence");
}

#[test]
fn test_cluster_bridge_rebind_latest_owner_transfers_realtime_state() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    expect_ok(runtime_a.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into(), "message.edited".into()],
        }],
    ));
    cluster
        .bind_device_route(
            "t_demo",
            "u_demo",
            "d_pad",
            "node_a",
            Some("s_old"),
            "websocket",
        )
        .expect("initial route bind should succeed");

    let delivered_before_rebind = expect_ok(runtime_a.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_before_rebind_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(delivered_before_rebind, 1);

    let ack = expect_ok(runtime_a.ack_events("t_demo", "u_demo", "d_pad", 1));
    assert_eq!(ack.acked_through_seq, 1);
    assert_eq!(ack.trimmed_through_seq, 1);

    let delivered_pending = expect_ok(runtime_a.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_before_rebind_2"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(delivered_pending, 1);

    let rebound_route = cluster
        .bind_device_route("t_demo", "u_demo", "d_pad", "node_b", Some("s_new"), "http")
        .expect("latest owner bind should succeed");
    assert_eq!(rebound_route.owner_node_id, "node_b");
    assert_eq!(rebound_route.connection_kind, "http");

    let source_window_after_rebind =
        expect_ok(runtime_a.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(
        source_window_after_rebind.items.len(),
        0,
        "source runtime must hand off pending window state on direct rebind"
    );

    let target_checkpoint = expect_ok(runtime_b.window_checkpoint("t_demo", "u_demo", "d_pad"));
    assert_eq!(target_checkpoint.latest_realtime_seq, 2);
    assert_eq!(target_checkpoint.acked_through_seq, 1);
    assert_eq!(target_checkpoint.trimmed_through_seq, 1);

    let target_window_after_rebind =
        expect_ok(runtime_b.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(target_window_after_rebind.items.len(), 1);
    assert_eq!(
        target_window_after_rebind.items[0].event_type,
        "message.posted"
    );
    assert_eq!(target_window_after_rebind.items[0].realtime_seq, 2);

    let publish_result = cluster.publish_device_event(
        "node_a",
        "t_demo",
        "u_demo",
        "d_pad",
        "conversation",
        "c_demo",
        "message.edited",
        r#"{"messageId":"msg_after_rebind"}"#.into(),
    );
    assert_eq!(publish_result.target_node_id, "node_b");
    assert_eq!(publish_result.route_state, "resolved");
    assert_eq!(publish_result.delivered, 1);

    let target_window_after_publish =
        expect_ok(runtime_b.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(target_window_after_publish.items.len(), 2);
    assert_eq!(
        target_window_after_publish.items[1].event_type,
        "message.edited"
    );
    assert_eq!(target_window_after_publish.items[1].realtime_seq, 3);
}

#[test]
fn test_cluster_bridge_rejects_route_migration_when_source_node_is_not_draining() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b);

    let error = cluster
        .migrate_node_routes("node_a", "node_b")
        .expect_err("active source node must not migrate before draining");
    assert_eq!(error.code, "node_not_draining");
    assert_eq!(error.node_id, "node_a");

    let source = cluster
        .node_lifecycle("node_a")
        .expect("source node lifecycle should remain");
    assert_eq!(source.drain_status, "active");
    assert_eq!(source.rebalance_state, "stable");

    let target = cluster
        .node_lifecycle("node_b")
        .expect("target node lifecycle should remain");
    assert_eq!(target.drain_status, "active");
    assert_eq!(target.rebalance_state, "stable");
}

#[test]
fn test_cluster_bridge_migration_restores_lazy_checkpoint_state_from_source_runtime() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let source_checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    source_checkpoint_store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 7,
            acked_through_seq: 5,
            trimmed_through_seq: 5,
            updated_at: "2026-04-05T12:30:00Z".into(),
        })
        .expect("checkpoint fixture should save");

    let runtime_a = Arc::new(RealtimeDeliveryRuntime::with_checkpoint_store(
        source_checkpoint_store,
    ));
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b.clone());
    cluster
        .bind_device_route("t_demo", "u_demo", "d_pad", "node_a", None, "websocket")
        .expect("route bind should succeed");

    cluster
        .mark_node_draining("node_a")
        .expect("drain should succeed");
    cluster
        .migrate_node_routes("node_a", "node_b")
        .expect("route migration should succeed");

    let target_checkpoint = expect_ok(runtime_b.window_checkpoint("t_demo", "u_demo", "d_pad"));
    assert_eq!(target_checkpoint.latest_realtime_seq, 7);
    assert_eq!(target_checkpoint.acked_through_seq, 5);
    assert_eq!(target_checkpoint.trimmed_through_seq, 5);
}

#[derive(Clone, Default)]
struct FailingCheckpointStore;

impl RealtimeCheckpointStore for FailingCheckpointStore {
    fn load_checkpoint(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Err(ContractError::Unavailable(
            "synthetic checkpoint load failure".into(),
        ))
    }

    fn save_checkpoint(&self, _record: RealtimeCheckpointRecord) -> Result<(), ContractError> {
        Err(ContractError::Unavailable(
            "synthetic checkpoint save failure".into(),
        ))
    }
}

#[test]
fn test_cluster_bridge_rebind_surfaces_checkpoint_store_failures_as_controlled_errors() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::with_checkpoint_store(Arc::new(
        FailingCheckpointStore,
    )));
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b);

    cluster
        .bind_device_route(
            "t_demo",
            "u_demo",
            "d_pad",
            "node_a",
            Some("s_old"),
            "websocket",
        )
        .expect("initial route bind should succeed");

    let error = cluster
        .bind_device_route("t_demo", "u_demo", "d_pad", "node_b", Some("s_new"), "http")
        .expect_err("rebind should surface a controlled error when checkpoint restore fails");
    assert_eq!(error.code, "checkpoint_store_unavailable");
    assert_eq!(error.node_id, "node_a");
}

#[test]
fn test_cluster_bridge_rejects_control_writes_for_unknown_node() {
    let cluster = Arc::new(RealtimeClusterBridge::default());

    let drain_error = cluster
        .mark_node_draining("node_missing")
        .expect_err("unknown node should not enter draining state");
    assert_eq!(drain_error.code, "node_not_found");
    assert_eq!(drain_error.node_id, "node_missing");

    let activate_error = cluster
        .activate_node("node_missing")
        .expect_err("unknown node should not be activated");
    assert_eq!(activate_error.code, "node_not_found");
    assert_eq!(activate_error.node_id, "node_missing");
}

#[test]
fn test_cluster_bridge_rejects_route_bind_for_unknown_node() {
    let cluster = Arc::new(RealtimeClusterBridge::default());

    let error = cluster
        .bind_device_route(
            "t_demo",
            "u_demo",
            "d_missing",
            "node_missing",
            None,
            "websocket",
        )
        .expect_err("unknown node should reject route bind");
    assert_eq!(error.code, "node_not_found");
    assert_eq!(error.node_id, "node_missing");

    let route = cluster.resolve_device_route("t_demo", "u_demo", "d_missing");
    assert!(
        route.is_none(),
        "failed bind must not create route ownership"
    );

    let lifecycle = cluster.node_lifecycle("node_missing");
    assert!(
        lifecycle.is_none(),
        "failed bind must not create node lifecycle"
    );
}

#[test]
fn test_cluster_route_bound_at_advances_between_distinct_bind_and_migration_operations() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b);

    let first_route = cluster
        .bind_device_route("t_demo", "u_demo", "d_one", "node_a", None, "websocket")
        .expect("first route bind should succeed");

    sleep(Duration::from_millis(20));

    let second_route = cluster
        .bind_device_route("t_demo", "u_demo", "d_two", "node_a", None, "http")
        .expect("second route bind should succeed");

    assert!(first_route.bound_at < second_route.bound_at);

    cluster
        .mark_node_draining("node_a")
        .expect("drain should succeed");
    sleep(Duration::from_millis(20));
    cluster
        .migrate_node_routes("node_a", "node_b")
        .expect("route migration should succeed");

    let migrated_route = cluster
        .resolve_device_route("t_demo", "u_demo", "d_one")
        .expect("migrated route should exist");
    assert_eq!(migrated_route.owner_node_id, "node_b");
    assert!(second_route.bound_at < migrated_route.bound_at);
}

#[test]
fn test_cluster_bridge_public_route_models_use_runtime_route_owner_types() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a);
    cluster.bind_node_runtime("node_b", runtime_b);

    let first_bind: RouteBinding = cluster
        .bind_device_route(
            "t_demo",
            "u_demo",
            "d_runtime_owner",
            "node_a",
            Some("s_owner_1"),
            "websocket",
        )
        .expect("initial route bind should succeed");
    assert_eq!(first_bind.route_epoch, 1);
    assert_eq!(first_bind.owner_node_id, "node_a");
    assert_eq!(first_bind.session_id.as_deref(), Some("s_owner_1"));
    assert_eq!(first_bind.connection_kind, "websocket");

    let draining: RouteNodeLifecycle = cluster
        .mark_node_draining("node_a")
        .expect("source node should enter draining");
    assert_eq!(draining.drain_status, "draining");
    assert_eq!(draining.rebalance_state, "moving_routes");
    assert_eq!(draining.owned_route_count, 1);

    let migration: RouteMigrationResult = cluster
        .migrate_node_routes("node_a", "node_b")
        .expect("route migration should succeed");
    assert_eq!(migration.migrated_route_count, 1);
    assert_eq!(migration.source_drain_status, "drained");
    assert_eq!(migration.target_drain_status, "active");

    let migrated: RouteBinding = cluster
        .resolve_device_route("t_demo", "u_demo", "d_runtime_owner")
        .expect("migrated route should remain present");
    assert_eq!(migrated.owner_node_id, "node_b");
    assert_eq!(migrated.route_epoch, 2);
    assert_eq!(migrated.session_id.as_deref(), Some("s_owner_1"));
    assert_eq!(migrated.connection_kind, "websocket");
}
