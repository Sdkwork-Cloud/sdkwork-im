use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_build_diagnostic_views_from_runtime() {
    let runtime = ops_service::OpsRuntime::new(
        "node_local_1",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["conversation-runtime".into(), "notification-service".into()],
        vec!["conversation:c_demo".into()],
    );

    let cluster = runtime.cluster_view();
    assert_eq!(cluster.nodes.len(), 1);
    assert_eq!(cluster.nodes[0].node_id, "node_local_1");
    assert_eq!(cluster.nodes[0].profile, "local-minimal");
    assert_eq!(cluster.nodes[0].device_route_count, 0);

    let lag = runtime.lag_view();
    assert_eq!(lag.items[0].lag, 0);

    let diagnostics = runtime.diagnostic_bundle();
    assert_eq!(diagnostics.profile, "local-minimal");
    assert_eq!(diagnostics.owned_scopes[0], "conversation:c_demo");
    assert_eq!(diagnostics.device_routes.len(), 0);
}

#[test]
fn test_runtime_exposes_route_ownership_and_drain_state() {
    let runtime = ops_service::OpsRuntime::new(
        "node_local_1",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["conversation-runtime".into()],
        vec!["conversation:*".into()],
    );
    runtime.set_node_lifecycle("draining", "moving_routes");
    runtime.update_route_ownership(vec![ops_service::RouteOwnershipView {
        tenant_id: "t_demo".into(),
        principal_id: "u_demo".into(),
        device_id: "d_pad".into(),
        owner_node_id: "node_local_1".into(),
        connection_kind: "websocket".into(),
        bound_at: "2026-04-05T11:20:00Z".into(),
    }]);

    let cluster = runtime.cluster_view();
    assert_eq!(cluster.nodes[0].drain_status, "draining");
    assert_eq!(cluster.nodes[0].rebalance_state, "moving_routes");
    assert_eq!(cluster.nodes[0].device_route_count, 1);

    let diagnostics = runtime.diagnostic_bundle();
    assert_eq!(diagnostics.drain_status, "draining");
    assert_eq!(diagnostics.rebalance_state, "moving_routes");
    assert_eq!(diagnostics.device_routes.len(), 1);
    assert_eq!(diagnostics.device_routes[0].device_id, "d_pad");
}

#[test]
fn test_diagnostic_bundle_generated_at_advances_between_calls() {
    let runtime = ops_service::OpsRuntime::new(
        "node_local_1",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["conversation-runtime".into()],
        vec!["conversation:*".into()],
    );

    let first = runtime.diagnostic_bundle();
    sleep(Duration::from_millis(20));
    let second = runtime.diagnostic_bundle();

    assert!(first.generated_at < second.generated_at);
}
