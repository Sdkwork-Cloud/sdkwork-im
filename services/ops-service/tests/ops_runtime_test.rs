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

#[test]
fn test_runtime_exposes_projection_replay_status_with_derived_throughput() {
    let runtime = ops_service::OpsRuntime::new(
        "node_local_1",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["conversation-runtime".into()],
        vec!["conversation:*".into()],
    );
    runtime.update_projection_plane(ops_service::ProjectionPlaneDiagnosticsView {
        status: "ok".into(),
        replay: ops_service::ProjectionReplayMetricsView {
            backlog_size: 4,
            replayed_event_count: 20,
            duration_ms: 5,
        },
        ..Default::default()
    });
    runtime.update_projection_replay_lag(vec![ops_service::LagItem {
        component: "projection_replay".into(),
        scope_id: "t_demo:c_demo".into(),
        current_offset: 10,
        committed_offset: 6,
        lag: 4,
    }]);

    let replay_status = runtime.replay_status_view();
    assert_eq!(replay_status.status, "replayed");
    assert_eq!(replay_status.replay.backlog_size, 4);
    assert_eq!(replay_status.replay.replayed_event_count, 20);
    assert_eq!(replay_status.replay.duration_ms, 5);
    assert_eq!(replay_status.replay_throughput_per_second, 4000);
    assert_eq!(replay_status.lag.len(), 1);
    assert_eq!(replay_status.lag[0].scope_id, "t_demo:c_demo");
}

#[test]
fn test_runtime_exposes_provider_binding_snapshots_and_diagnostics() {
    let runtime = ops_service::OpsRuntime::new(
        "node_local_1",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["conversation-runtime".into()],
        vec!["conversation:*".into()],
    );
    runtime.update_provider_binding_snapshot(ops_service::ProviderBindingSnapshotView {
        interface_version: "provider-registry/v1".into(),
        tenant_id: None,
        effective_bindings: vec![ops_service::ProviderBindingItemView {
            domain: "rtc".into(),
            default_plugin_id: Some("rtc-volcengine".into()),
            selected_plugin_id: Some("rtc-volcengine".into()),
            selection_source: "global_default".into(),
            tenant_override_allowed: true,
        }],
        precedence: vec![
            "tenant_override".into(),
            "deployment_profile".into(),
            "global_default".into(),
        ],
    });
    runtime.update_provider_binding_snapshot(ops_service::ProviderBindingSnapshotView {
        interface_version: "provider-registry/v1".into(),
        tenant_id: Some("t_provider_combo".into()),
        effective_bindings: vec![ops_service::ProviderBindingItemView {
            domain: "object-storage".into(),
            default_plugin_id: None,
            selected_plugin_id: Some("object-storage-aws".into()),
            selection_source: "tenant_override".into(),
            tenant_override_allowed: true,
        }],
        precedence: vec![
            "tenant_override".into(),
            "deployment_profile".into(),
            "global_default".into(),
        ],
    });

    let provider_bindings = runtime.provider_bindings_view();
    assert_eq!(provider_bindings.items.len(), 2);
    assert_eq!(provider_bindings.items[0].tenant_id, None);
    assert_eq!(
        provider_bindings.items[1].tenant_id.as_deref(),
        Some("t_provider_combo")
    );
    assert_eq!(
        provider_bindings.items[1].effective_bindings[0]
            .selected_plugin_id
            .as_deref(),
        Some("object-storage-aws")
    );

    let diagnostics = runtime.diagnostic_bundle();
    assert_eq!(diagnostics.provider_bindings.len(), 2);
    assert_eq!(
        diagnostics.provider_bindings[0].effective_bindings[0]
            .selected_plugin_id
            .as_deref(),
        Some("rtc-volcengine")
    );
}

#[test]
fn test_runtime_exposes_provider_binding_drift_against_global_snapshot() {
    let runtime = ops_service::OpsRuntime::new(
        "node_local_1",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["conversation-runtime".into()],
        vec!["conversation:*".into()],
    );
    runtime.update_provider_binding_snapshot(ops_service::ProviderBindingSnapshotView {
        interface_version: "provider-registry/v1".into(),
        tenant_id: None,
        effective_bindings: vec![
            ops_service::ProviderBindingItemView {
                domain: "object-storage".into(),
                default_plugin_id: None,
                selected_plugin_id: Some("object-storage-volcengine".into()),
                selection_source: "deployment_profile".into(),
                tenant_override_allowed: true,
            },
            ops_service::ProviderBindingItemView {
                domain: "rtc".into(),
                default_plugin_id: Some("rtc-volcengine".into()),
                selected_plugin_id: Some("rtc-volcengine".into()),
                selection_source: "global_default".into(),
                tenant_override_allowed: true,
            },
        ],
        precedence: vec![
            "tenant_override".into(),
            "deployment_profile".into(),
            "global_default".into(),
        ],
    });
    runtime.update_provider_binding_snapshot(ops_service::ProviderBindingSnapshotView {
        interface_version: "provider-registry/v1".into(),
        tenant_id: Some("t_provider_combo".into()),
        effective_bindings: vec![
            ops_service::ProviderBindingItemView {
                domain: "object-storage".into(),
                default_plugin_id: None,
                selected_plugin_id: Some("object-storage-aws".into()),
                selection_source: "tenant_override".into(),
                tenant_override_allowed: true,
            },
            ops_service::ProviderBindingItemView {
                domain: "rtc".into(),
                default_plugin_id: Some("rtc-volcengine".into()),
                selected_plugin_id: Some("rtc-aliyun".into()),
                selection_source: "tenant_override".into(),
                tenant_override_allowed: true,
            },
        ],
        precedence: vec![
            "tenant_override".into(),
            "deployment_profile".into(),
            "global_default".into(),
        ],
    });

    let drift = runtime.provider_binding_drift_view();
    assert_eq!(drift.baseline_tenant_id, None);
    assert_eq!(drift.items.len(), 2);
    assert_eq!(drift.items[0].tenant_id, "t_provider_combo");
    assert_eq!(drift.items[0].domain, "object-storage");
    assert_eq!(
        drift.items[0].baseline_selected_plugin_id.as_deref(),
        Some("object-storage-volcengine")
    );
    assert_eq!(
        drift.items[0].selected_plugin_id.as_deref(),
        Some("object-storage-aws")
    );
    assert_eq!(
        drift.items[0].drift_kind,
        "plugin_and_selection_source_changed"
    );

    let diagnostics = runtime.diagnostic_bundle();
    assert_eq!(diagnostics.provider_binding_drift.items.len(), 2);
    assert_eq!(
        diagnostics.provider_binding_drift.items[1]
            .selected_plugin_id
            .as_deref(),
        Some("rtc-aliyun")
    );
}
