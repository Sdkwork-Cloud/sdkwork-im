use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use craw_chat_ccp_control::{HelloAckFrame, HelloFrame};
use craw_chat_ccp_core::{CapabilitySet, ProtocolVersion, TransportBinding};
use craw_chat_ccp_registry::{
    CcpRegistry, ClientCompatibilityDescriptor, EffectiveProtocolSnapshot, ReleaseChannel,
};
use craw_chat_runtime_link::{LinkHelloError, LinkSession, OutboundQueuePolicy};
use http_body_util::BodyExt;
use serde::Deserialize;
use serde_json::{Value, json};
use session_gateway::RealtimeClusterBridge;
use tower::ServiceExt;

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Step11LocalDrillBaseline {
    profile: String,
    tier: String,
    drain_rebalance: DrainRebalanceBaseline,
    restore_recovery: RestoreRecoveryBaseline,
    failover: FailoverBaseline,
    upgrade_rollback: UpgradeRollbackBaseline,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DrainRebalanceBaseline {
    expected_route_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RestoreRecoveryBaseline {
    expected_restored_file_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FailoverBaseline {
    expected_owner_node_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpgradeRollbackBaseline {
    expected_safe_client_count: usize,
    expected_blocked_binding: String,
    expected_disabled_capability: String,
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn local_drill_baseline_path() -> PathBuf {
    workspace_root()
        .join("tools")
        .join("perf")
        .join("step-11-cp11-3-local-drill-baseline.json")
}

fn load_local_drill_baseline() -> Step11LocalDrillBaseline {
    let path = local_drill_baseline_path();
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing Step 11 local drill baseline: {}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|_| panic!("invalid Step 11 local drill baseline: {}", path.display()))
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn transport_binding_from_protocol_id(protocol_id: &str) -> TransportBinding {
    match protocol_id {
        "ccp/http/1" => TransportBinding::Http1,
        "ccp/ws/1" => TransportBinding::Ws1,
        "ccp/sse/1" => TransportBinding::Sse1,
        "ccp/mqtt/1" => TransportBinding::Mqtt1,
        other => panic!("unsupported transport binding protocol id: {other}"),
    }
}

fn select_safe_binding(
    descriptor: &ClientCompatibilityDescriptor,
    snapshot: &EffectiveProtocolSnapshot,
) -> TransportBinding {
    let binding = descriptor
        .supported_bindings
        .iter()
        .find(|binding| snapshot.allowed_bindings.contains(*binding))
        .unwrap_or_else(|| {
            panic!(
                "client {} must keep at least one safe binding after rollback",
                descriptor.client_type
            )
        });
    transport_binding_from_protocol_id(binding.as_str())
}

fn build_canary_upgrade_snapshot(
    rollback_snapshot: &EffectiveProtocolSnapshot,
    disabled_capability: &str,
    blocked_binding: &str,
) -> EffectiveProtocolSnapshot {
    let mut canary = rollback_snapshot.clone();
    canary.release_channel = ReleaseChannel::Canary;
    canary.kill_switch_active = false;
    canary
        .enabled_capabilities
        .insert(disabled_capability.to_owned());
    canary.allowed_bindings.insert(blocked_binding.to_owned());
    canary.allowed_codecs.insert("cbor".to_owned());
    canary
}

fn negotiate_protocol_hello(
    snapshot: &EffectiveProtocolSnapshot,
    binding: TransportBinding,
    requested_capabilities: &[&str],
) -> Result<HelloAckFrame, LinkHelloError> {
    let mut session = LinkSession::new_with_effective_snapshot(
        "t_demo",
        "u_step11_upgrade",
        "user",
        "d_step11_upgrade",
        Some("s_step11_upgrade"),
        OutboundQueuePolicy::realtime_default(),
        snapshot.clone(),
    );
    let hello = HelloFrame {
        protocol: ProtocolVersion::new("ccp", 1, 0),
        binding,
        capabilities: CapabilitySet::from_iter(requested_capabilities.iter().copied()),
        trace_id: Some("trace-step11-upgrade-rollback".into()),
    };
    session.negotiate_hello(&hello)
}

fn unique_path(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}_{sequence}"))
}

fn write_state_file(root: &Path, file_name: &str, content: &str) {
    let state_dir = root.join("state");
    fs::create_dir_all(&state_dir).expect("state dir should be created");
    fs::write(state_dir.join(file_name), content).expect("state file should be written");
}

fn write_valid_backup_snapshot(root: &Path, owner_node_id: &str) {
    write_state_file(root, "commit-journal.json", "");
    for file_name in [
        "realtime-checkpoints.json",
        "realtime-subscriptions.json",
        "stream-state.json",
        "rtc-state.json",
        "automation-executions.json",
        "projection-metadata.json",
        "projection-timeline.json",
    ] {
        write_state_file(root, file_name, "{}");
    }
    write_state_file(
        root,
        "presence-state.json",
        "{\"by_device\":{},\"presence_by_principal\":{},\"online_by_seen_at\":{}}",
    );
    write_state_file(
        root,
        "notification-tasks.json",
        "{\"by_notification\":{},\"tasks_by_recipient\":{}}",
    );
    write_state_file(
        root,
        "realtime-disconnect-fences.json",
        serde_json::to_string_pretty(&json!({
            "t_demo:u_demo:d_demo": {
                "tenant_id": "t_demo",
                "principal_kind": "user",
                "principal_id": "u_demo",
                "device_id": "d_demo",
                "session_id": "s_demo",
                "owner_node_id": owner_node_id,
                "disconnected_at": "2026-04-06T00:00:00.000Z",
                "fence_token": format!("fence:t_demo:user:u_demo:d_demo:s_demo:{owner_node_id}:2026-04-06T00:00:00.000Z")
            }
        }))
        .expect("disconnect fence snapshot should serialize")
        .as_str(),
    );
}

fn print_metric(metric: Value) {
    println!("STEP11_DRILL {}", metric);
}

#[test]
fn test_step11_local_drill_baseline_config_and_operator_doc_are_frozen() {
    let baseline = load_local_drill_baseline();
    assert_eq!(baseline.profile, "local-minimal");
    assert_eq!(baseline.tier, "CI Smoke Tier");
    assert!(baseline.drain_rebalance.expected_route_count > 0);
    assert!(baseline.restore_recovery.expected_restored_file_count > 0);
    assert!(
        !baseline.failover.expected_owner_node_id.is_empty(),
        "failover expected owner node id must not be empty"
    );
    assert!(
        baseline.upgrade_rollback.expected_safe_client_count > 0,
        "upgrade rollback expected safe client count must be greater than zero"
    );
    assert!(
        !baseline
            .upgrade_rollback
            .expected_blocked_binding
            .is_empty(),
        "upgrade rollback blocked binding must not be empty"
    );
    assert!(
        !baseline
            .upgrade_rollback
            .expected_disabled_capability
            .is_empty(),
        "upgrade rollback disabled capability must not be empty"
    );

    let doc_path = workspace_root()
        .join("docs")
        .join("部署")
        .join("性能与灾备演练场景.md");
    let doc = fs::read_to_string(&doc_path)
        .unwrap_or_else(|_| panic!("missing Step 11 operator doc: {}", doc_path.display()));
    assert!(doc.contains("tools/perf/step-11-cp11-3-local-drill-baseline.json"));
    assert!(doc.contains("services/local-minimal-node/tests/performance_ha_dr_drill_test.rs"));
}

#[tokio::test]
async fn test_step11_local_drain_rebalance_drill_emits_metrics() {
    let baseline = load_local_drill_baseline();
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18201",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies(
        "node_b",
        "127.0.0.1:18202",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let control_app = control_plane_api::build_app_with_cluster(realtime_cluster.clone());

    let create_conversation = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_step11_drain_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_step11_drain_demo/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_remote",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let register_remote_device = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_remote")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_remote")
                .header("x-sdkwork-session-id", "s_remote")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register remote device should succeed");
    assert_eq!(register_remote_device.status(), StatusCode::OK);

    let sync_remote_subscriptions = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_remote")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_remote")
                .header("x-sdkwork-session-id", "s_remote")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_step11_drain_demo",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("sync remote subscriptions should succeed");
    assert_eq!(sync_remote_subscriptions.status(), StatusCode::OK);

    let drill_start = Instant::now();
    let drain_response = control_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/nodes/node_a/drain")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("drain request should succeed");
    assert_eq!(drain_response.status(), StatusCode::OK);

    let migrate_response = control_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/nodes/node_a/routes/migrate")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"targetNodeId":"node_b"}"#))
                .unwrap(),
        )
        .await
        .expect("migrate request should succeed");
    assert_eq!(migrate_response.status(), StatusCode::OK);
    let migrate_body = migrate_response
        .into_body()
        .collect()
        .await
        .expect("migrate body should collect")
        .to_bytes();
    let migrate_json: Value =
        serde_json::from_slice(&migrate_body).expect("migrate body should be valid json");

    let drained_pull = app_a
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_remote")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_remote")
                .header("x-sdkwork-session-id", "s_remote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("drained pull should return response");
    assert_eq!(drained_pull.status(), StatusCode::CONFLICT);

    let post_message = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_step11_drain_demo/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_step11_drain_route_1",
                        "summary":"drain hello",
                        "text":"drain hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let remote_events = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_remote")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_remote")
                .header("x-sdkwork-session-id", "s_remote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote events should succeed");
    assert_eq!(remote_events.status(), StatusCode::OK);
    let remote_events_body = remote_events
        .into_body()
        .collect()
        .await
        .expect("remote events body should collect")
        .to_bytes();
    let remote_events_json: Value =
        serde_json::from_slice(&remote_events_body).expect("remote events should be valid json");
    let items = remote_events_json["items"]
        .as_array()
        .expect("remote events items should be array");
    let drill_duration_ms = drill_start.elapsed().as_secs_f64() * 1000.0;

    assert_eq!(items.len(), 1);
    assert_eq!(migrate_json["migratedRouteCount"], 1);
    assert_eq!(baseline.drain_rebalance.expected_route_count, 1);

    print_metric(json!({
        "scenario": "drain-rebalance",
        "profile": baseline.profile,
        "tier": baseline.tier,
        "expectedRouteCount": baseline.drain_rebalance.expected_route_count,
        "migratedRouteCount": migrate_json["migratedRouteCount"],
        "deliveredEventCount": items.len(),
        "drillDurationMs": round3(drill_duration_ms),
        "deliveryPreserved": true
    }));
}

#[test]
fn test_step11_local_restore_recovery_drill_emits_metrics() {
    let baseline = load_local_drill_baseline();
    let runtime_dir = unique_path("step11_restore_runtime");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path())
        .expect("repair should succeed");

    write_state_file(
        runtime_dir.as_path(),
        "realtime-disconnect-fences.json",
        serde_json::to_string_pretty(&json!({
            "t_demo:u_demo:d_demo": {
                "tenant_id": "t_demo",
                "principal_kind": "user",
                "principal_id": "u_demo",
                "device_id": "d_demo",
                "session_id": "s_demo",
                "owner_node_id": "node_current",
                "disconnected_at": "2026-04-06T00:00:00.000Z",
                "fence_token": "fence:t_demo:user:u_demo:d_demo:s_demo:node_current:2026-04-06T00:00:00.000Z"
            }
        }))
        .expect("current fence snapshot should serialize")
        .as_str(),
    );
    write_state_file(
        runtime_dir.as_path(),
        "projection-metadata.json",
        "{\"t_demo:c_demo:conversation-summary\":\"current\"}",
    );
    write_state_file(
        runtime_dir.as_path(),
        "projection-timeline.json",
        "{\"t_demo:c_demo\":{\"1\":\"current\"}}",
    );

    let backup_dir = unique_path("step11_restore_backup");
    write_valid_backup_snapshot(backup_dir.as_path(), "node_backup");
    write_state_file(
        backup_dir.as_path(),
        "projection-metadata.json",
        "{\"t_demo:c_demo:conversation-summary\":\"backup\"}",
    );
    write_state_file(
        backup_dir.as_path(),
        "projection-timeline.json",
        "{\"t_demo:c_demo\":{\"1\":\"backup\"}}",
    );

    let preview_started = Instant::now();
    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");
    let preview_duration_ms = preview_started.elapsed().as_secs_f64() * 1000.0;

    let restore_started = Instant::now();
    let report = local_minimal_node::restore_runtime_dir_with_expected_preview_fingerprint(
        runtime_dir.as_path(),
        backup_dir.as_path(),
        Some(preview.preview_fingerprint.as_str()),
    )
    .expect("restore should succeed");
    let restore_duration_ms = restore_started.elapsed().as_secs_f64() * 1000.0;

    assert_eq!(report.status, "restored");
    assert_eq!(
        report.restored_file_count,
        baseline.restore_recovery.expected_restored_file_count
    );

    print_metric(json!({
        "scenario": "restore-recovery",
        "profile": baseline.profile,
        "tier": baseline.tier,
        "expectedRestoredFileCount": baseline.restore_recovery.expected_restored_file_count,
        "restoredFileCount": report.restored_file_count,
        "previewDurationMs": round3(preview_duration_ms),
        "restoreDurationMs": round3(restore_duration_ms),
        "restoreStatus": report.status
    }));

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_step11_local_upgrade_rollback_drill_emits_metrics() {
    let baseline = load_local_drill_baseline();
    let registry = CcpRegistry::control_plane_v1();
    let rollback_snapshot = registry
        .governance_snapshot()
        .expect("control plane registry should expose governance snapshot")
        .effective_snapshot
        .clone();
    let canary_snapshot = build_canary_upgrade_snapshot(
        &rollback_snapshot,
        baseline
            .upgrade_rollback
            .expected_disabled_capability
            .as_str(),
        baseline.upgrade_rollback.expected_blocked_binding.as_str(),
    );

    let canary_ack = negotiate_protocol_hello(
        &canary_snapshot,
        TransportBinding::Mqtt1,
        &[
            "payload.json",
            baseline
                .upgrade_rollback
                .expected_disabled_capability
                .as_str(),
        ],
    )
    .expect("risky binding should stay available before rollback");
    assert!(
        canary_ack.capabilities.supports(
            baseline
                .upgrade_rollback
                .expected_disabled_capability
                .as_str()
        ),
        "pre-rollback canary path should still advertise the risky capability"
    );

    let compatible_client_count = registry
        .compatibility_matrix()
        .values()
        .filter(|descriptor| {
            descriptor
                .supported_bindings
                .iter()
                .any(|binding| rollback_snapshot.allowed_bindings.contains(binding))
                && descriptor
                    .supported_codecs
                    .iter()
                    .any(|codec| rollback_snapshot.allowed_codecs.contains(codec))
                && descriptor
                    .supported_capabilities
                    .iter()
                    .any(|capability| rollback_snapshot.enabled_capabilities.contains(capability))
        })
        .count();
    let total_client_count = registry.compatibility_matrix().len();
    assert_eq!(
        compatible_client_count,
        baseline.upgrade_rollback.expected_safe_client_count
    );

    let rollback_started = Instant::now();
    let rollback_error = negotiate_protocol_hello(
        &rollback_snapshot,
        TransportBinding::Mqtt1,
        &[
            "payload.json",
            baseline
                .upgrade_rollback
                .expected_disabled_capability
                .as_str(),
        ],
    )
    .expect_err("rollback must reject the risky binding");
    let rollback_activation_ms = rollback_started.elapsed().as_secs_f64() * 1000.0;
    let blocked_binding_rejected = matches!(
        rollback_error,
        LinkHelloError::UnsupportedBinding { ref protocol_id }
            if protocol_id == &baseline.upgrade_rollback.expected_blocked_binding
    );
    assert!(
        blocked_binding_rejected,
        "rollback should reject the blocked binding {}",
        baseline.upgrade_rollback.expected_blocked_binding
    );
    assert!(
        rollback_snapshot.kill_switch_active,
        "rollback snapshot must carry kill switch activation"
    );

    let downgraded_desktop_ack = negotiate_protocol_hello(
        &rollback_snapshot,
        TransportBinding::Ws1,
        &[
            "payload.json",
            baseline
                .upgrade_rollback
                .expected_disabled_capability
                .as_str(),
        ],
    )
    .expect("safe websocket path should stay available after rollback");
    assert!(
        downgraded_desktop_ack.capabilities.supports("payload.json"),
        "rollback should preserve json payload capability on safe bindings"
    );
    assert!(
        !downgraded_desktop_ack.capabilities.supports(
            baseline
                .upgrade_rollback
                .expected_disabled_capability
                .as_str()
        ),
        "rollback should strip the risky capability from negotiated hello capabilities"
    );

    let mut safe_protocol_error_count = 0usize;
    for descriptor in registry.compatibility_matrix().values() {
        let safe_binding = select_safe_binding(descriptor, &rollback_snapshot);
        if negotiate_protocol_hello(&rollback_snapshot, safe_binding, &[]).is_err() {
            safe_protocol_error_count += 1;
        }
    }
    assert_eq!(safe_protocol_error_count, 0);

    let kill_switch_success_count = [
        blocked_binding_rejected,
        !rollback_snapshot.allowed_codecs.contains("cbor"),
        !rollback_snapshot.enabled_capabilities.contains(
            baseline
                .upgrade_rollback
                .expected_disabled_capability
                .as_str(),
        ),
        !downgraded_desktop_ack.capabilities.supports(
            baseline
                .upgrade_rollback
                .expected_disabled_capability
                .as_str(),
        ),
    ]
    .into_iter()
    .filter(|result| *result)
    .count();

    print_metric(json!({
        "scenario": "upgrade-rollback",
        "profile": baseline.profile,
        "tier": baseline.tier,
        "safeClientCount": total_client_count,
        "compatibleClientCount": compatible_client_count,
        "compatibilityMatrixPassRate": round3(compatible_client_count as f64 / total_client_count.max(1) as f64),
        "rollbackActivationMs": round3(rollback_activation_ms),
        "killSwitchPropagationSuccessRate": round3(kill_switch_success_count as f64 / 4.0),
        "postRollbackProtocolErrorRate": round3(safe_protocol_error_count as f64 / total_client_count.max(1) as f64),
        "blockedBinding": baseline.upgrade_rollback.expected_blocked_binding,
        "disabledCapability": baseline.upgrade_rollback.expected_disabled_capability,
        "preRollbackRiskyHandshakeAccepted": true
    }));
}
