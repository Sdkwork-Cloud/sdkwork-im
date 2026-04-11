use std::collections::BTreeSet;
use std::sync::Arc;

use audit_service::AuditRuntime;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_auth_context::AuthContext;
use im_platform_contracts::{ProviderDomain, RuntimeProviderRegistry, StaticProviderRegistry};
use ops_service::OpsRuntime;
use session_gateway::{
    RealtimeClusterBridge, RealtimeDeliveryRuntime, RealtimeSubscriptionItemInput,
};
use tower::ServiceExt;

#[tokio::test]
async fn test_control_plane_governance_writes_feed_ops_and_audit_runtimes() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
    let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
    cluster.bind_node_runtime("node_a", runtime_a.clone());
    cluster.bind_node_runtime("node_b", runtime_b.clone());

    let _ = runtime_a.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    );
    cluster
        .bind_device_route("t_demo", "u_demo", "d_pad", "node_a", None, "websocket")
        .expect("route bind should succeed");

    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec!["conversation:c_demo".into()],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());

    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
    );

    let drain_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/nodes/node_a/drain")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("drain request should return response");
    assert_eq!(drain_response.status(), StatusCode::OK);

    let drain_cluster = ops_runtime.cluster_view();
    assert_eq!(drain_cluster.nodes[0].node_id, "node_a");
    assert_eq!(drain_cluster.nodes[0].drain_status, "draining");
    assert_eq!(drain_cluster.nodes[0].rebalance_state, "moving_routes");
    assert_eq!(drain_cluster.nodes[0].device_route_count, 1);

    let migrate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/nodes/node_a/routes/migrate")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"targetNodeId":"node_b"}"#))
                .unwrap(),
        )
        .await
        .expect("migrate request should return response");
    assert_eq!(migrate_response.status(), StatusCode::OK);

    let migrate_body = migrate_response
        .into_body()
        .collect()
        .await
        .expect("migrate body should collect")
        .to_bytes();
    let migrate_json: serde_json::Value =
        serde_json::from_slice(&migrate_body).expect("migrate body should be valid json");
    assert_eq!(migrate_json["migratedRouteCount"], 1);

    let migrated_cluster = ops_runtime.cluster_view();
    assert_eq!(migrated_cluster.nodes[0].drain_status, "drained");
    assert_eq!(migrated_cluster.nodes[0].device_route_count, 0);

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 2);
    assert_eq!(audit_export.items[0].action, "control.node_draining_marked");
    assert_eq!(audit_export.items[1].action, "control.node_routes_migrated");
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("drain audit record should include payload")
            .contains("\"nodeId\":\"node_a\"")
    );
    assert!(
        audit_export.items[1]
            .payload
            .as_deref()
            .expect("migrate audit record should include payload")
            .contains("\"targetNodeId\":\"node_b\"")
    );
}

#[tokio::test]
async fn test_control_plane_provider_bindings_feed_ops_runtime() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec!["conversation:c_demo".into()],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let provider_registry = Arc::new(
        StaticProviderRegistry::platform_default()
            .with_deployment_profile(ProviderDomain::ObjectStorage, "object-storage-volcengine")
            .with_tenant_override("t_provider_combo", ProviderDomain::Rtc, "rtc-aliyun")
            .with_tenant_override(
                "t_provider_combo",
                ProviderDomain::ObjectStorage,
                "object-storage-aws",
            ),
    );

    let app = control_plane_api::build_app_with_cluster_provider_registry_and_governance_sinks(
        cluster,
        provider_registry,
        ops_runtime.clone(),
        audit_runtime,
    );

    let global_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("global provider bindings request should return response");
    assert_eq!(global_response.status(), StatusCode::OK);

    let tenant_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/provider-bindings?tenantId=t_provider_combo")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("tenant provider bindings request should return response");
    assert_eq!(tenant_response.status(), StatusCode::OK);

    let provider_bindings = ops_runtime.provider_bindings_view();
    assert_eq!(provider_bindings.items.len(), 2);
    assert_eq!(provider_bindings.items[0].tenant_id, None);
    assert_eq!(
        provider_bindings.items[1].tenant_id.as_deref(),
        Some("t_provider_combo")
    );
    assert!(
        provider_bindings.items[0]
            .effective_bindings
            .iter()
            .any(|binding| binding.domain == "object-storage"
                && binding.selected_plugin_id.as_deref() == Some("object-storage-volcengine")
                && binding.selection_source == "deployment_profile")
    );
    assert!(
        provider_bindings.items[1]
            .effective_bindings
            .iter()
            .any(|binding| binding.domain == "rtc"
                && binding.selected_plugin_id.as_deref() == Some("rtc-aliyun")
                && binding.selection_source == "tenant_override")
    );

    let drift = ops_runtime.provider_binding_drift_view();
    assert_eq!(drift.items.len(), 2);
    assert!(
        drift
            .items
            .iter()
            .any(|item| item.domain == "object-storage"
                && item.tenant_id == "t_provider_combo"
                && item.selected_plugin_id.as_deref() == Some("object-storage-aws")
                && item.baseline_selected_plugin_id.as_deref()
                    == Some("object-storage-volcengine"))
    );
}

#[tokio::test]
async fn test_control_plane_provider_policy_writes_feed_ops_and_audit_runtimes() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec!["conversation:c_demo".into()],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());

    let app =
        control_plane_api::build_app_with_cluster_runtime_provider_registry_and_governance_sinks(
            cluster,
            provider_registry,
            ops_runtime.clone(),
            audit_runtime.clone(),
        );

    let deployment_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("deployment write should return response");
    assert_eq!(deployment_write.status(), StatusCode::OK);

    let tenant_write = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tenant write should return response");
    assert_eq!(tenant_write.status(), StatusCode::OK);

    let provider_bindings = ops_runtime.provider_bindings_view();
    assert_eq!(provider_bindings.items.len(), 2);
    assert!(
        provider_bindings.items[0]
            .effective_bindings
            .iter()
            .any(|binding| binding.domain == "object-storage"
                && binding.selected_plugin_id.as_deref() == Some("object-storage-volcengine")
                && binding.selection_source == "deployment_profile")
    );
    assert!(
        provider_bindings.items[1]
            .effective_bindings
            .iter()
            .any(|binding| binding.domain == "rtc"
                && binding.selected_plugin_id.as_deref() == Some("rtc-aliyun")
                && binding.selection_source == "tenant_override")
    );

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 2);
    assert_eq!(
        audit_export.items[0].action,
        "control.provider_deployment_profile_updated"
    );
    assert_eq!(
        audit_export.items[1].action,
        "control.provider_tenant_override_updated"
    );
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("deployment policy audit should include payload")
            .contains("\"pluginId\":\"object-storage-volcengine\"")
    );
    assert!(
        audit_export.items[1]
            .payload
            .as_deref()
            .expect("tenant override audit should include payload")
            .contains("\"tenantId\":\"t_provider_combo\"")
    );
}

#[tokio::test]
async fn test_control_plane_provider_policy_rollback_refreshes_ops_runtime_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec!["conversation:c_demo".into()],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());

    let app =
        control_plane_api::build_app_with_cluster_runtime_provider_registry_and_governance_sinks(
            cluster,
            provider_registry,
            ops_runtime.clone(),
            audit_runtime.clone(),
        );

    let deployment_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("deployment write should return response");
    assert_eq!(deployment_write.status(), StatusCode::OK);

    let tenant_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tenant write should return response");
    assert_eq!(tenant_write.status(), StatusCode::OK);

    let rollback_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-policies/rollback")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"targetVersion":1}"#))
                .unwrap(),
        )
        .await
        .expect("rollback should return response");
    assert_eq!(rollback_response.status(), StatusCode::OK);
    let rollback_body = rollback_response
        .into_body()
        .collect()
        .await
        .expect("rollback body should collect")
        .to_bytes();
    let rollback_json: serde_json::Value =
        serde_json::from_slice(&rollback_body).expect("rollback body should be valid json");
    assert_eq!(rollback_json["status"], "rolled_back");

    let provider_bindings = ops_runtime.provider_bindings_view();
    assert_eq!(provider_bindings.items.len(), 1);
    assert_eq!(provider_bindings.items[0].tenant_id, None);
    assert!(
        provider_bindings.items[0]
            .effective_bindings
            .iter()
            .any(|binding| binding.domain == "object-storage"
                && binding.selected_plugin_id.is_none()
                && binding.selection_source == "deployment_required")
    );
    assert!(
        ops_runtime.provider_binding_drift_view().items.is_empty(),
        "rollback should clear tenant drift when all tenant overrides are removed"
    );

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 3);
    assert_eq!(
        audit_export.items[2].action,
        "control.provider_policy_rolled_back"
    );
    assert!(
        audit_export.items[2]
            .payload
            .as_deref()
            .expect("rollback audit should include payload")
            .contains("\"targetVersion\":1")
    );
}

#[tokio::test]
async fn test_control_plane_noop_provider_policy_write_does_not_append_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec!["conversation:c_demo".into()],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());

    let app =
        control_plane_api::build_app_with_cluster_runtime_provider_registry_and_governance_sinks(
            cluster,
            provider_registry,
            ops_runtime.clone(),
            audit_runtime.clone(),
        );

    let first_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first write should return response");
    assert_eq!(first_write.status(), StatusCode::OK);

    let noop_write = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine","expectedBaseVersion":2}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("noop write should return response");
    assert_eq!(noop_write.status(), StatusCode::OK);
    let noop_body = noop_write
        .into_body()
        .collect()
        .await
        .expect("noop body should collect")
        .to_bytes();
    let noop_json: serde_json::Value =
        serde_json::from_slice(&noop_body).expect("noop body should be valid json");
    assert_eq!(noop_json["status"], "noop");
    assert_eq!(noop_json["applied"], false);

    let provider_bindings = ops_runtime.provider_bindings_view();
    assert_eq!(provider_bindings.items.len(), 1);
    assert!(
        provider_bindings.items[0]
            .effective_bindings
            .iter()
            .any(|binding| binding.domain == "object-storage"
                && binding.selected_plugin_id.as_deref() == Some("object-storage-volcengine")
                && binding.selection_source == "deployment_profile")
    );

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 1);
    assert_eq!(
        audit_export.items[0].action,
        "control.provider_deployment_profile_updated"
    );
}

#[tokio::test]
async fn test_control_plane_provider_policy_preview_does_not_touch_ops_or_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec!["conversation:c_demo".into()],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());

    let app =
        control_plane_api::build_app_with_cluster_runtime_provider_registry_and_governance_sinks(
            cluster,
            provider_registry,
            ops_runtime.clone(),
            audit_runtime.clone(),
        );

    let preview_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-policies/preview")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("preview should return response");
    assert_eq!(preview_response.status(), StatusCode::OK);
    let preview_body = preview_response
        .into_body()
        .collect()
        .await
        .expect("preview body should collect")
        .to_bytes();
    let preview_json: serde_json::Value =
        serde_json::from_slice(&preview_body).expect("preview body should be valid json");
    assert_eq!(preview_json["status"], "preview");

    assert!(
        ops_runtime.provider_bindings_view().items.is_empty(),
        "preview must not refresh ops provider bindings"
    );
    assert!(
        ops_runtime.provider_binding_drift_view().items.is_empty(),
        "preview must not create provider drift"
    );

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 0);
}

#[tokio::test]
async fn test_control_plane_stale_provider_policy_confirm_write_does_not_touch_ops_or_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec!["conversation:c_demo".into()],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let provider_registry = Arc::new(RuntimeProviderRegistry::platform_default());

    let app =
        control_plane_api::build_app_with_cluster_runtime_provider_registry_and_governance_sinks(
            cluster,
            provider_registry,
            ops_runtime.clone(),
            audit_runtime.clone(),
        );

    let preview_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-policies/preview")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("preview should return response");
    assert_eq!(preview_response.status(), StatusCode::OK);
    let preview_body = preview_response
        .into_body()
        .collect()
        .await
        .expect("preview body should collect")
        .to_bytes();
    let preview_json: serde_json::Value =
        serde_json::from_slice(&preview_body).expect("preview body should be valid json");
    assert_eq!(preview_json["status"], "preview");

    let concurrent_write = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("concurrent write should return response");
    assert_eq!(concurrent_write.status(), StatusCode::OK);

    let stale_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/provider-bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"tenantId":"t_provider_combo","domain":"rtc","pluginId":"rtc-aliyun","expectedBaseVersion":1}"#,
                ))
                .unwrap(),
        )
        .await
        .expect("stale write should return response");
    assert_eq!(stale_response.status(), StatusCode::CONFLICT);

    let provider_bindings = ops_runtime.provider_bindings_view();
    assert_eq!(provider_bindings.items.len(), 1);
    assert_eq!(provider_bindings.items[0].tenant_id, None);
    assert!(
        provider_bindings.items[0]
            .effective_bindings
            .iter()
            .any(|binding| binding.domain == "object-storage"
                && binding.selected_plugin_id.as_deref() == Some("object-storage-volcengine")
                && binding.selection_source == "deployment_profile")
    );
    assert!(
        ops_runtime.provider_binding_drift_view().items.is_empty(),
        "stale confirm write must not create tenant drift"
    );

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 1);
    assert_eq!(
        audit_export.items[0].action,
        "control.provider_deployment_profile_updated"
    );
}
