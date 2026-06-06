use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use audit_service::AuditRuntime;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use base64::Engine as _;
use hmac::{Hmac, Mac};
use http_body_util::BodyExt;
use im_adapters_local_disk::FileCommitJournal;
use im_app_context::AppContext;
use im_domain_core::social::direct_chat_pair_hash;
use im_domain_events::social::{
    DirectChatBoundPayload, SocialCommitEnvelopeInput, SocialEventType, social_commit_envelope,
};
use im_domain_events::{AggregateType, EventActor};
use im_platform_contracts::CommitJournal;
use ops_service::OpsRuntime;
use session_gateway::RealtimeClusterBridge;
use sha2::Sha256;
use tower::ServiceExt;

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);
const TEST_FRIEND_REQUEST_CURSOR_SECRET: &str = "friend-request-cursor-test-secret";
const FRIEND_REQUEST_CURSOR_SECRET_ENV: &str = "CRAW_CHAT_FRIEND_REQUEST_CURSOR_HS256_SECRET";

struct ScopedEnvVar {
    name: &'static str,
    previous: Option<String>,
}

impl ScopedEnvVar {
    fn set(name: &'static str, value: &str) -> Self {
        let previous = std::env::var(name).ok();
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        if let Some(previous) = &self.previous {
            unsafe {
                std::env::set_var(self.name, previous);
            }
            return;
        }

        unsafe {
            std::env::remove_var(self.name);
        }
    }
}

async fn friend_request_cursor_env_guard() -> tokio::sync::MutexGuard<'static, ()> {
    static GUARD: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    GUARD
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await
}

fn decode_friend_request_cursor_payload(cursor: &str) -> serde_json::Value {
    let segments = cursor.split('.').collect::<Vec<_>>();
    assert_eq!(
        segments.len(),
        3,
        "friend request cursor should use signed compact token format"
    );
    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(segments[1])
        .expect("friend request cursor payload segment should be valid base64url");
    serde_json::from_slice(&payload).expect("friend request cursor payload should be valid json")
}

fn replace_friend_request_cursor_payload(cursor: &str, payload: &serde_json::Value) -> String {
    let segments = cursor.split('.').collect::<Vec<_>>();
    assert_eq!(
        segments.len(),
        3,
        "friend request cursor should use signed compact token format"
    );
    let next_payload = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(payload).expect("friend request cursor payload should serialize"),
    );
    format!("{}.{}.{}", segments[0], next_payload, segments[2])
}

fn sign_friend_request_cursor_for_test(payload: &serde_json::Value) -> String {
    let header_segment =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256","typ":"JWT"}"#);
    let payload_segment = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(payload).expect("friend request cursor payload should serialize"),
    );
    let signing_input = format!("{header_segment}.{payload_segment}");
    let mut mac = Hmac::<Sha256>::new_from_slice(TEST_FRIEND_REQUEST_CURSOR_SECRET.as_bytes())
        .expect("friend request cursor test secret should initialize HMAC");
    mac.update(signing_input.as_bytes());
    let signature_segment =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
    format!("{signing_input}.{signature_segment}")
}

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "craw_chat_control_plane_social_runtime_{unique}_{sequence}"
    ))
}

fn state_file(runtime_dir: &Path, file_name: &str) -> PathBuf {
    runtime_dir.join("state").join(file_name)
}

fn read_json_lines(path: &Path) -> Vec<serde_json::Value> {
    let body = fs::read_to_string(path).expect("JSON Lines file should be readable");
    body.lines()
        .map(|line| serde_json::from_str(line).expect("JSON Lines record should be valid json"))
        .collect()
}

fn social_failpoint_file(runtime_dir: &Path) -> PathBuf {
    state_file(runtime_dir, "social-failpoints.json")
}

#[cfg(unix)]
fn make_file_writable(path: &Path) {
    let mut permissions = fs::metadata(path)
        .expect("target file should exist")
        .permissions();
    use std::os::unix::fs::PermissionsExt;

    permissions.set_mode(permissions.mode() | 0o200);
    fs::set_permissions(path, permissions).expect("target file should become writable again");
}

#[cfg(not(unix))]
#[allow(clippy::permissions_set_readonly_false)]
fn make_file_writable(path: &Path) {
    let mut permissions = fs::metadata(path)
        .expect("target file should exist")
        .permissions();
    permissions.set_readonly(false);
    fs::set_permissions(path, permissions).expect("target file should become writable again");
}

fn social_tx_marker_file(runtime_dir: &Path) -> PathBuf {
    state_file(runtime_dir, "social-transaction-marker.json")
}

fn build_durable_social_control_app(
    runtime_dir: &Path,
    node_id: &str,
    bind_addr: &str,
) -> axum::Router {
    control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(OpsRuntime::new(
            node_id,
            "local-minimal",
            bind_addr,
            vec!["session-gateway".into(), "control-plane-api".into()],
            vec![],
        )),
        Arc::new(AuditRuntime::default()),
        runtime_dir,
    )
}

#[tokio::test]
async fn test_control_plane_social_friend_request_write_persists_snapshot_commit_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_001",
                        "eventId":"evt_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestMessage":"hello",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let submit_body = submit_response
        .into_body()
        .collect()
        .await
        .expect("submit body should collect")
        .to_bytes();
    let submit_json: serde_json::Value =
        serde_json::from_slice(&submit_body).expect("submit body should be valid json");

    assert_eq!(submit_json["status"], "submitted");
    assert_eq!(submit_json["friendRequest"]["tenantId"], "t_demo");
    assert_eq!(submit_json["friendRequest"]["requestId"], "fr_001");
    assert_eq!(submit_json["friendRequest"]["requesterUserId"], "u_alice");
    assert_eq!(submit_json["friendRequest"]["targetUserId"], "u_bob");
    assert_eq!(submit_json["friendRequest"]["status"], "pending");
    assert_eq!(
        submit_json["latestCommit"]["eventType"],
        "friend_request.submitted"
    );
    assert_eq!(submit_json["latestCommit"]["scopeType"], "friend_request");
    assert_eq!(
        submit_json["latestCommit"]["payloadSchema"],
        "social.friend_request.submitted.v1"
    );
    assert_eq!(submit_json["latestCommit"]["orderingSeq"], 1);

    let snapshot_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot body should be valid json");

    assert_eq!(snapshot_json["status"], "snapshot");
    assert_eq!(snapshot_json["friendRequest"]["requestId"], "fr_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "friend_request.submitted"
    );

    let audit_auth = AppContext {
        tenant_id: "t_demo".into(),
        organization_id: None,
        user_id: "u_admin".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        app_id: Some("craw-chat".into()),
        environment: None,
        deployment_mode: None,
        device_id: None,
        permission_scope: BTreeSet::new(),
        data_scope: BTreeSet::new(),
        auth_level: None,
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 1);
    assert_eq!(
        audit_export.items[0].action,
        "control.friend_request_submitted"
    );
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("friend request audit should include payload")
            .contains("\"requestId\":\"fr_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_second_instance_accepts_request_committed_by_first_instance()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = build_durable_social_control_app(
        runtime_dir.as_path(),
        "node_cross_instance_a",
        "127.0.0.1:18190",
    );
    let app_b = build_durable_social_control_app(
        runtime_dir.as_path(),
        "node_cross_instance_b",
        "127.0.0.1:18191",
    );

    let submit_response = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_cross_instance_accept_001",
                        "eventId":"evt_cross_instance_accept_submit_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-16T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first instance friend request submit should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let accept_response = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_cross_instance_accept_001/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_cross_instance_accept_accept_001",
                        "acceptedByUserId":"u_bob",
                        "acceptedAt":"2026-04-16T10:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second instance friend request accept should return response");
    assert_eq!(accept_response.status(), StatusCode::OK);
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("second instance friend request accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value = serde_json::from_slice(&accept_body)
        .expect("second instance friend request accept body should be valid json");
    assert_eq!(accept_json["friendRequest"]["status"], "accepted");

    let snapshot_response = app_a
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_cross_instance_accept_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first instance snapshot after second instance accept should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);
    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("first instance snapshot after second instance accept body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("first instance snapshot after second instance accept body should be valid json");
    assert_eq!(snapshot_json["friendRequest"]["status"], "accepted");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_concurrent_submit_same_pair_across_instances_converges_to_single_pending_request()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = build_durable_social_control_app(
        runtime_dir.as_path(),
        "node_cross_submit_a",
        "127.0.0.1:18192",
    );
    let app_b = build_durable_social_control_app(
        runtime_dir.as_path(),
        "node_cross_submit_b",
        "127.0.0.1:18193",
    );

    let submit_from_alice = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friend_requests")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "requestId":"fr_cross_pair_race_submit_alice",
                "eventId":"evt_cross_pair_race_submit_alice",
                "requesterUserId":"u_alice",
                "targetUserId":"u_bob",
                "requestedAt":"2026-04-16T11:00:00Z"
            }"#,
        ))
        .unwrap();
    let submit_from_bob = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friend_requests")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "requestId":"fr_cross_pair_race_submit_bob",
                "eventId":"evt_cross_pair_race_submit_bob",
                "requesterUserId":"u_bob",
                "targetUserId":"u_alice",
                "requestedAt":"2026-04-16T11:00:01Z"
            }"#,
        ))
        .unwrap();

    let (alice_response, bob_response) = tokio::join!(
        app_a.clone().oneshot(submit_from_alice),
        app_b.clone().oneshot(submit_from_bob),
    );
    let alice_response =
        alice_response.expect("cross-instance concurrent submit from alice should return response");
    let bob_response =
        bob_response.expect("cross-instance concurrent submit from bob should return response");

    let alice_status = alice_response.status();
    let alice_body = alice_response
        .into_body()
        .collect()
        .await
        .expect("cross-instance concurrent submit from alice body should collect")
        .to_bytes();
    let alice_json: serde_json::Value = serde_json::from_slice(&alice_body)
        .expect("cross-instance concurrent submit from alice body should be valid json");

    let bob_status = bob_response.status();
    let bob_body = bob_response
        .into_body()
        .collect()
        .await
        .expect("cross-instance concurrent submit from bob body should collect")
        .to_bytes();
    let bob_json: serde_json::Value = serde_json::from_slice(&bob_body)
        .expect("cross-instance concurrent submit from bob body should be valid json");

    let alice_succeeded = alice_status == StatusCode::OK;
    let bob_succeeded = bob_status == StatusCode::OK;
    assert_ne!(
        alice_succeeded, bob_succeeded,
        "exactly one cross-instance submit for the same pair should commit"
    );

    let winning_request_id = if alice_succeeded {
        assert_eq!(bob_status, StatusCode::CONFLICT);
        assert_eq!(bob_json["code"], "friend_request_pair_conflict");
        alice_json["friendRequest"]["requestId"]
            .as_str()
            .expect("cross-instance winning submit should expose request id")
            .to_owned()
    } else {
        assert_eq!(alice_status, StatusCode::CONFLICT);
        assert_eq!(alice_json["code"], "friend_request_pair_conflict");
        bob_json["friendRequest"]["requestId"]
            .as_str()
            .expect("cross-instance winning submit should expose request id")
            .to_owned()
    };

    let winning_snapshot = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{winning_request_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cross-instance winning request snapshot should return response");
    assert_eq!(winning_snapshot.status(), StatusCode::OK);

    let rejected_request_id = if winning_request_id == "fr_cross_pair_race_submit_alice" {
        "fr_cross_pair_race_submit_bob"
    } else {
        "fr_cross_pair_race_submit_alice"
    };
    let rejected_snapshot = app_b
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{rejected_request_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cross-instance rejected request snapshot should return response");
    assert_eq!(rejected_snapshot.status(), StatusCode::NOT_FOUND);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_concurrent_accept_and_cancel_across_instances_converge_to_single_terminal_state()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = build_durable_social_control_app(
        runtime_dir.as_path(),
        "node_cross_accept_cancel_a",
        "127.0.0.1:18194",
    );
    let app_b = build_durable_social_control_app(
        runtime_dir.as_path(),
        "node_cross_accept_cancel_b",
        "127.0.0.1:18195",
    );

    let submit_response = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_cross_accept_cancel_race_001",
                        "eventId":"evt_cross_accept_cancel_race_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-16T11:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("cross-instance submit before accept/cancel race should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let accept_request = Request::builder()
        .method("POST")
        .uri(
            "/backend/v3/api/control/social/friend_requests/fr_cross_accept_cancel_race_001/accept",
        )
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "eventId":"evt_cross_accept_cancel_race_accept",
                "acceptedByUserId":"u_bob",
                "acceptedAt":"2026-04-16T11:06:00Z"
            }"#,
        ))
        .unwrap();
    let cancel_request = Request::builder()
        .method("POST")
        .uri(
            "/backend/v3/api/control/social/friend_requests/fr_cross_accept_cancel_race_001/cancel",
        )
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "eventId":"evt_cross_accept_cancel_race_cancel",
                "canceledByUserId":"u_alice",
                "canceledAt":"2026-04-16T11:06:00Z"
            }"#,
        ))
        .unwrap();

    let (accept_response, cancel_response) = tokio::join!(
        app_a.clone().oneshot(accept_request),
        app_b.clone().oneshot(cancel_request),
    );
    let accept_response =
        accept_response.expect("cross-instance concurrent accept should return response");
    let cancel_response =
        cancel_response.expect("cross-instance concurrent cancel should return response");

    let accept_status = accept_response.status();
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("cross-instance concurrent accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value = serde_json::from_slice(&accept_body)
        .expect("cross-instance concurrent accept body should be valid json");

    let cancel_status = cancel_response.status();
    let cancel_body = cancel_response
        .into_body()
        .collect()
        .await
        .expect("cross-instance concurrent cancel body should collect")
        .to_bytes();
    let cancel_json: serde_json::Value = serde_json::from_slice(&cancel_body)
        .expect("cross-instance concurrent cancel body should be valid json");

    let success_count = [accept_status, cancel_status]
        .into_iter()
        .filter(|status| *status == StatusCode::OK)
        .count();
    let conflict_count = [accept_status, cancel_status]
        .into_iter()
        .filter(|status| *status == StatusCode::CONFLICT)
        .count();
    assert_eq!(
        success_count, 1,
        "exactly one cross-instance terminal operation should win the accept/cancel race"
    );
    assert_eq!(
        conflict_count, 1,
        "the losing cross-instance terminal operation should be rejected after the winner commits"
    );

    let expected_final_status = if accept_status == StatusCode::OK {
        assert_eq!(accept_json["friendRequest"]["status"], "accepted");
        assert_eq!(cancel_json["code"], "friend_request_not_pending");
        "accepted"
    } else {
        assert_eq!(cancel_status, StatusCode::OK);
        assert_eq!(cancel_json["friendRequest"]["status"], "canceled");
        assert_eq!(accept_json["code"], "friend_request_not_pending");
        "canceled"
    };

    let snapshot_response = app_a
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_cross_accept_cancel_race_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cross-instance friend request snapshot after accept/cancel race should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);
    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect(
            "cross-instance friend request snapshot after accept/cancel race body should collect",
        )
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body).expect(
        "cross-instance friend request snapshot after accept/cancel race body should be valid json",
    );
    assert_eq!(
        snapshot_json["friendRequest"]["status"],
        expected_final_status
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_concurrent_remove_and_submit_across_instances_never_leave_active_pair_with_pending_request()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = build_durable_social_control_app(
        runtime_dir.as_path(),
        "node_cross_remove_submit_a",
        "127.0.0.1:18196",
    );
    let app_b = build_durable_social_control_app(
        runtime_dir.as_path(),
        "node_cross_remove_submit_b",
        "127.0.0.1:18197",
    );

    let activate_friendship = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_cross_remove_submit_race_001",
                        "eventId":"evt_cross_remove_submit_race_activate",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "establishedAt":"2026-04-16T11:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect(
            "cross-instance friendship activation before remove/submit race should return response",
        );
    assert_eq!(activate_friendship.status(), StatusCode::OK);

    let remove_request = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friendships/fs_cross_remove_submit_race_001/remove")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "eventId":"evt_cross_remove_submit_race_remove",
                "removedByUserId":"u_alice",
                "removedAt":"2026-04-16T11:11:00Z"
            }"#,
        ))
        .unwrap();
    let submit_request = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friend_requests")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "requestId":"fr_cross_remove_submit_race_001",
                "eventId":"evt_cross_remove_submit_race_submit",
                "requesterUserId":"u_alice",
                "targetUserId":"u_bob",
                "requestedAt":"2026-04-16T11:11:00Z"
            }"#,
        ))
        .unwrap();

    let (remove_response, submit_response) = tokio::join!(
        app_a.clone().oneshot(remove_request),
        app_b.clone().oneshot(submit_request),
    );
    let remove_response = remove_response
        .expect("cross-instance concurrent friendship remove should return response");
    let submit_response = submit_response
        .expect("cross-instance concurrent friend request submit should return response");

    assert_eq!(remove_response.status(), StatusCode::OK);
    let remove_body = remove_response
        .into_body()
        .collect()
        .await
        .expect("cross-instance concurrent friendship remove body should collect")
        .to_bytes();
    let remove_json: serde_json::Value = serde_json::from_slice(&remove_body)
        .expect("cross-instance concurrent friendship remove body should be valid json");
    assert_eq!(remove_json["friendship"]["status"], "removed");

    let submit_status = submit_response.status();
    let submit_body = submit_response
        .into_body()
        .collect()
        .await
        .expect("cross-instance concurrent friend request submit body should collect")
        .to_bytes();
    let submit_json: serde_json::Value = serde_json::from_slice(&submit_body)
        .expect("cross-instance concurrent friend request submit body should be valid json");
    match submit_status {
        StatusCode::OK => assert_eq!(submit_json["friendRequest"]["status"], "pending"),
        StatusCode::CONFLICT => assert_eq!(submit_json["code"], "friendship_pair_conflict"),
        other => {
            panic!("unexpected cross-instance submit status during remove/submit race: {other}")
        }
    }

    let friendship_snapshot = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friendships/fs_cross_remove_submit_race_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect(
            "cross-instance friendship snapshot after remove/submit race should return response",
        );
    assert_eq!(friendship_snapshot.status(), StatusCode::OK);
    let friendship_snapshot_body = friendship_snapshot
        .into_body()
        .collect()
        .await
        .expect("cross-instance friendship snapshot after remove/submit race body should collect")
        .to_bytes();
    let friendship_snapshot_json: serde_json::Value =
        serde_json::from_slice(&friendship_snapshot_body).expect(
            "cross-instance friendship snapshot after remove/submit race body should be valid json",
        );
    assert_eq!(friendship_snapshot_json["friendship"]["status"], "removed");

    let friend_request_snapshot = app_b
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_cross_remove_submit_race_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cross-instance friend request snapshot after remove/submit race should return response");
    if submit_status == StatusCode::OK {
        assert_eq!(friend_request_snapshot.status(), StatusCode::OK);
        let friend_request_snapshot_body = friend_request_snapshot
            .into_body()
            .collect()
            .await
            .expect(
                "cross-instance successful remove/submit friend request snapshot should collect",
            )
            .to_bytes();
        let friend_request_snapshot_json: serde_json::Value = serde_json::from_slice(
            &friend_request_snapshot_body,
        )
        .expect(
            "cross-instance successful remove/submit friend request snapshot should be valid json",
        );
        assert_eq!(
            friend_request_snapshot_json["friendRequest"]["status"],
            "pending"
        );
    } else {
        assert_eq!(friend_request_snapshot.status(), StatusCode::NOT_FOUND);
    }

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_friend_request_rejects_identical_user_pair() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_invalid",
                        "eventId":"evt_invalid",
                        "requesterUserId":"u_same",
                        "targetUserId":"u_same",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invalid friend request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("invalid body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("invalid body should be valid json");
    assert_eq!(json["status"], 400);
    assert_eq!(json["errorStatus"], "invalid");
    assert_eq!(json["code"], "invalid_friend_request");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_rejects_duplicate_open_pair_and_returns_existing_request_details()
 {
    let app = control_plane_api::build_app();

    let first_submit = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_pair_guard_001",
                        "eventId":"evt_pair_guard_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first pair-guard friend request should return response");
    assert_eq!(first_submit.status(), StatusCode::OK);

    let duplicate_submit = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_pair_guard_002",
                        "eventId":"evt_pair_guard_002",
                        "requesterUserId":"u_bob",
                        "targetUserId":"u_alice",
                        "requestedAt":"2026-04-10T10:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate pair-guard friend request should return response");

    assert_eq!(duplicate_submit.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_submit
        .into_body()
        .collect()
        .await
        .expect("duplicate pair-guard body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value = serde_json::from_slice(&duplicate_body)
        .expect("duplicate pair-guard body should be valid json");
    assert_eq!(duplicate_json["status"], 409);
    assert_eq!(duplicate_json["errorStatus"], "conflict");
    assert_eq!(duplicate_json["code"], "friend_request_pair_conflict");
    assert_eq!(
        duplicate_json["details"]["existingRequestId"],
        "fr_pair_guard_001"
    );
    assert_eq!(duplicate_json["details"]["existingStatus"], "pending");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_concurrent_submit_same_pair_converges_to_single_pending_request()
 {
    let app = control_plane_api::build_app();

    let submit_from_alice = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friend_requests")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "requestId":"fr_pair_race_submit_alice",
                "eventId":"evt_pair_race_submit_alice",
                "requesterUserId":"u_alice",
                "targetUserId":"u_bob",
                "requestedAt":"2026-04-10T10:00:00Z"
            }"#,
        ))
        .unwrap();
    let submit_from_bob = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friend_requests")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "requestId":"fr_pair_race_submit_bob",
                "eventId":"evt_pair_race_submit_bob",
                "requesterUserId":"u_bob",
                "targetUserId":"u_alice",
                "requestedAt":"2026-04-10T10:00:01Z"
            }"#,
        ))
        .unwrap();

    let (alice_response, bob_response) = tokio::join!(
        app.clone().oneshot(submit_from_alice),
        app.clone().oneshot(submit_from_bob),
    );
    let alice_response =
        alice_response.expect("concurrent submit from alice should return response");
    let bob_response = bob_response.expect("concurrent submit from bob should return response");

    let alice_status = alice_response.status();
    let alice_body = alice_response
        .into_body()
        .collect()
        .await
        .expect("concurrent submit from alice body should collect")
        .to_bytes();
    let alice_json: serde_json::Value = serde_json::from_slice(&alice_body)
        .expect("concurrent submit from alice body should be valid json");

    let bob_status = bob_response.status();
    let bob_body = bob_response
        .into_body()
        .collect()
        .await
        .expect("concurrent submit from bob body should collect")
        .to_bytes();
    let bob_json: serde_json::Value = serde_json::from_slice(&bob_body)
        .expect("concurrent submit from bob body should be valid json");

    let alice_succeeded = alice_status == StatusCode::OK;
    let bob_succeeded = bob_status == StatusCode::OK;
    assert_ne!(
        alice_succeeded, bob_succeeded,
        "exactly one concurrent submit for the same pair should commit"
    );

    let winning_request_id = if alice_succeeded {
        assert_eq!(bob_status, StatusCode::CONFLICT);
        assert_eq!(bob_json["code"], "friend_request_pair_conflict");
        alice_json["friendRequest"]["requestId"]
            .as_str()
            .expect("winning submit should expose request id")
            .to_owned()
    } else {
        assert_eq!(alice_status, StatusCode::CONFLICT);
        assert_eq!(alice_json["code"], "friend_request_pair_conflict");
        bob_json["friendRequest"]["requestId"]
            .as_str()
            .expect("winning submit should expose request id")
            .to_owned()
    };
    let conflict_json = if alice_succeeded {
        &bob_json
    } else {
        &alice_json
    };
    assert_eq!(
        conflict_json["details"]["existingRequestId"],
        winning_request_id.as_str()
    );
    assert_eq!(conflict_json["details"]["existingStatus"], "pending");

    let winning_snapshot = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{winning_request_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("winning request snapshot should return response");
    assert_eq!(winning_snapshot.status(), StatusCode::OK);
    let winning_snapshot_body = winning_snapshot
        .into_body()
        .collect()
        .await
        .expect("winning request snapshot body should collect")
        .to_bytes();
    let winning_snapshot_json: serde_json::Value = serde_json::from_slice(&winning_snapshot_body)
        .expect("winning request snapshot body should be valid json");
    assert_eq!(winning_snapshot_json["friendRequest"]["status"], "pending");

    let rejected_request_id = if winning_request_id == "fr_pair_race_submit_alice" {
        "fr_pair_race_submit_bob"
    } else {
        "fr_pair_race_submit_alice"
    };
    let rejected_snapshot = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{rejected_request_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("rejected request snapshot should return response");
    assert_eq!(rejected_snapshot.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_control_plane_social_friend_request_concurrent_accept_and_cancel_converge_to_single_terminal_state()
 {
    let app = control_plane_api::build_app();

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_accept_cancel_race_001",
                        "eventId":"evt_accept_cancel_race_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit before accept/cancel race should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let accept_request = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friend_requests/fr_accept_cancel_race_001/accept")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "eventId":"evt_accept_cancel_race_accept",
                "acceptedByUserId":"u_bob",
                "acceptedAt":"2026-04-10T10:05:00Z"
            }"#,
        ))
        .unwrap();
    let cancel_request = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friend_requests/fr_accept_cancel_race_001/cancel")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "eventId":"evt_accept_cancel_race_cancel",
                "canceledByUserId":"u_alice",
                "canceledAt":"2026-04-10T10:05:00Z"
            }"#,
        ))
        .unwrap();

    let (accept_response, cancel_response) = tokio::join!(
        app.clone().oneshot(accept_request),
        app.clone().oneshot(cancel_request),
    );
    let accept_response = accept_response.expect("concurrent accept should return response");
    let cancel_response = cancel_response.expect("concurrent cancel should return response");

    let accept_status = accept_response.status();
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("concurrent accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value =
        serde_json::from_slice(&accept_body).expect("concurrent accept body should be valid json");

    let cancel_status = cancel_response.status();
    let cancel_body = cancel_response
        .into_body()
        .collect()
        .await
        .expect("concurrent cancel body should collect")
        .to_bytes();
    let cancel_json: serde_json::Value =
        serde_json::from_slice(&cancel_body).expect("concurrent cancel body should be valid json");

    let success_count = [accept_status, cancel_status]
        .into_iter()
        .filter(|status| *status == StatusCode::OK)
        .count();
    let conflict_count = [accept_status, cancel_status]
        .into_iter()
        .filter(|status| *status == StatusCode::CONFLICT)
        .count();
    assert_eq!(
        success_count, 1,
        "exactly one terminal operation should win the accept/cancel race"
    );
    assert_eq!(
        conflict_count, 1,
        "the losing terminal operation should be rejected after the winner commits"
    );

    let expected_final_status = if accept_status == StatusCode::OK {
        assert_eq!(accept_json["friendRequest"]["status"], "accepted");
        assert_eq!(cancel_json["code"], "friend_request_not_pending");
        "accepted"
    } else {
        assert_eq!(cancel_status, StatusCode::OK);
        assert_eq!(cancel_json["friendRequest"]["status"], "canceled");
        assert_eq!(accept_json["code"], "friend_request_not_pending");
        "canceled"
    };

    let snapshot_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_cancel_race_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after accept/cancel race should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);
    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request snapshot after accept/cancel race body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request snapshot after accept/cancel race body should be valid json");
    assert_eq!(
        snapshot_json["friendRequest"]["status"],
        expected_final_status
    );
    assert_eq!(
        snapshot_json["commits"]
            .as_array()
            .expect("friend request snapshot should include commits")
            .len(),
        2
    );
}

#[tokio::test]
async fn test_control_plane_social_friendship_concurrent_remove_and_submit_never_leaves_active_pair_with_pending_request()
 {
    let app = control_plane_api::build_app();

    let activate_friendship = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_remove_submit_race_001",
                        "eventId":"evt_remove_submit_race_activate",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friendship activation before remove/submit race should return response");
    assert_eq!(activate_friendship.status(), StatusCode::OK);

    let remove_request = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friendships/fs_remove_submit_race_001/remove")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "eventId":"evt_remove_submit_race_remove",
                "removedByUserId":"u_alice",
                "removedAt":"2026-04-10T11:05:00Z"
            }"#,
        ))
        .unwrap();
    let submit_request = Request::builder()
        .method("POST")
        .uri("/backend/v3/api/control/social/friend_requests")
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_admin")
        .header("x-sdkwork-actor-kind", "admin")
        .header("x-sdkwork-permission-scope", "control.write")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{
                "requestId":"fr_remove_submit_race_001",
                "eventId":"evt_remove_submit_race_submit",
                "requesterUserId":"u_alice",
                "targetUserId":"u_bob",
                "requestedAt":"2026-04-10T11:05:00Z"
            }"#,
        ))
        .unwrap();

    let (remove_response, submit_response) = tokio::join!(
        app.clone().oneshot(remove_request),
        app.clone().oneshot(submit_request),
    );
    let remove_response =
        remove_response.expect("concurrent friendship remove should return response");
    let submit_response =
        submit_response.expect("concurrent friend request submit should return response");

    assert_eq!(remove_response.status(), StatusCode::OK);
    let remove_body = remove_response
        .into_body()
        .collect()
        .await
        .expect("concurrent friendship remove body should collect")
        .to_bytes();
    let remove_json: serde_json::Value = serde_json::from_slice(&remove_body)
        .expect("concurrent friendship remove body should be valid json");
    assert_eq!(remove_json["friendship"]["status"], "removed");

    let submit_status = submit_response.status();
    let submit_body = submit_response
        .into_body()
        .collect()
        .await
        .expect("concurrent friend request submit body should collect")
        .to_bytes();
    let submit_json: serde_json::Value = serde_json::from_slice(&submit_body)
        .expect("concurrent friend request submit body should be valid json");
    match submit_status {
        StatusCode::OK => assert_eq!(submit_json["friendRequest"]["status"], "pending"),
        StatusCode::CONFLICT => assert_eq!(submit_json["code"], "friendship_pair_conflict"),
        other => panic!("unexpected submit status during remove/submit race: {other}"),
    }

    let friendship_snapshot = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friendships/fs_remove_submit_race_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot after remove/submit race should return response");
    assert_eq!(friendship_snapshot.status(), StatusCode::OK);
    let friendship_snapshot_body = friendship_snapshot
        .into_body()
        .collect()
        .await
        .expect("friendship snapshot after remove/submit race body should collect")
        .to_bytes();
    let friendship_snapshot_json: serde_json::Value =
        serde_json::from_slice(&friendship_snapshot_body)
            .expect("friendship snapshot after remove/submit race body should be valid json");
    assert_eq!(friendship_snapshot_json["friendship"]["status"], "removed");

    let friend_request_snapshot = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_remove_submit_race_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after remove/submit race should return response");
    if submit_status == StatusCode::OK {
        assert_eq!(friend_request_snapshot.status(), StatusCode::OK);
        let friend_request_snapshot_body = friend_request_snapshot
            .into_body()
            .collect()
            .await
            .expect("friend request snapshot after successful remove/submit race should collect")
            .to_bytes();
        let friend_request_snapshot_json: serde_json::Value = serde_json::from_slice(
            &friend_request_snapshot_body,
        )
        .expect("friend request snapshot after successful remove/submit race should be valid json");
        assert_eq!(
            friend_request_snapshot_json["friendRequest"]["status"],
            "pending"
        );
    } else {
        assert_eq!(friend_request_snapshot.status(), StatusCode::NOT_FOUND);
    }
}

#[tokio::test]
async fn test_control_plane_social_friend_request_rejects_submit_for_active_friendship_pair() {
    let app = control_plane_api::build_app();

    let activate_friendship = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_submit_guard_001",
                        "eventId":"evt_submit_guard_001_activate",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friendship activation before submit guard should return response");
    assert_eq!(activate_friendship.status(), StatusCode::OK);

    let submit_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_submit_guard_001",
                        "eventId":"evt_submit_guard_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T11:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit for active friendship should return response");

    assert_eq!(submit_response.status(), StatusCode::CONFLICT);
    let submit_body = submit_response
        .into_body()
        .collect()
        .await
        .expect("submit guard body should collect")
        .to_bytes();
    let submit_json: serde_json::Value =
        serde_json::from_slice(&submit_body).expect("submit guard body should be valid json");
    assert_eq!(submit_json["status"], 409);
    assert_eq!(submit_json["errorStatus"], "conflict");
    assert_eq!(submit_json["code"], "friendship_pair_conflict");
    assert_eq!(
        submit_json["details"]["existingFriendshipId"],
        "fs_submit_guard_001"
    );
}

#[tokio::test]
async fn test_control_plane_social_friend_request_rejects_submit_for_pair_with_accepted_request_before_friendship_materializes()
 {
    let app = control_plane_api::build_app();

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_submit_guard_accepted_existing",
                        "eventId":"evt_submit_guard_accepted_existing_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T11:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before accepted guard should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_submit_guard_accepted_existing/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_submit_guard_accepted_existing_accept",
                        "acceptedByUserId":"u_bob",
                        "acceptedAt":"2026-04-10T11:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request accept before accepted guard should return response");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let duplicate_submit = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_submit_guard_accepted_duplicate",
                        "eventId":"evt_submit_guard_accepted_duplicate_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T11:12:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate submit against accepted pair should return response");

    assert_eq!(duplicate_submit.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_submit
        .into_body()
        .collect()
        .await
        .expect("duplicate submit against accepted pair body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value = serde_json::from_slice(&duplicate_body)
        .expect("duplicate submit against accepted pair body should be valid json");
    assert_eq!(duplicate_json["status"], 409);
    assert_eq!(duplicate_json["errorStatus"], "conflict");
    assert_eq!(duplicate_json["code"], "friendship_pair_conflict");
    assert!(
        duplicate_json["details"]["existingFriendshipId"]
            .as_str()
            .is_some_and(|friendship_id| friendship_id.starts_with("fs_")),
        "accepted pair conflict should surface the materialized friendship id: {duplicate_json}"
    );
    assert_eq!(duplicate_json["details"]["existingStatus"], "active");

    let duplicate_snapshot = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_submit_guard_accepted_duplicate")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("duplicate accepted pair snapshot should return response");
    assert_eq!(duplicate_snapshot.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_control_plane_social_friend_request_list_filters_by_direction_and_status() {
    let app = control_plane_api::build_app();

    let incoming_submit = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_list_pending_001",
                        "eventId":"evt_list_pending_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("incoming friend request submit should return response");
    assert_eq!(incoming_submit.status(), StatusCode::OK);

    let outgoing_submit = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_list_canceled_001",
                        "eventId":"evt_list_canceled_001_submit",
                        "requesterUserId":"u_bob",
                        "targetUserId":"u_carol",
                        "requestedAt":"2026-04-10T10:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("outgoing friend request submit should return response");
    assert_eq!(outgoing_submit.status(), StatusCode::OK);

    let cancel_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_list_canceled_001/cancel")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_list_canceled_001_cancel",
                        "canceledByUserId":"u_bob",
                        "canceledAt":"2026-04-10T10:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("cancel outgoing friend request should return response");
    assert_eq!(cancel_response.status(), StatusCode::OK);

    let incoming_list = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests?userId=u_bob&direction=incoming")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("incoming friend request inventory should return response");
    assert_eq!(incoming_list.status(), StatusCode::OK);
    let incoming_list_body = incoming_list
        .into_body()
        .collect()
        .await
        .expect("incoming friend request inventory body should collect")
        .to_bytes();
    let incoming_list_json: serde_json::Value = serde_json::from_slice(&incoming_list_body)
        .expect("incoming friend request inventory body should be valid json");
    let incoming_items = incoming_list_json["items"]
        .as_array()
        .expect("incoming friend request inventory should include items");
    assert_eq!(incoming_list_json["status"], "inventory");
    assert_eq!(incoming_items.len(), 1);
    assert_eq!(incoming_items[0]["requestId"], "fr_list_pending_001");
    assert_eq!(incoming_items[0]["status"], "pending");

    let outgoing_pending_list = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests?userId=u_bob&direction=outgoing")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("outgoing pending friend request inventory should return response");
    assert_eq!(outgoing_pending_list.status(), StatusCode::OK);
    let outgoing_pending_body = outgoing_pending_list
        .into_body()
        .collect()
        .await
        .expect("outgoing pending inventory body should collect")
        .to_bytes();
    let outgoing_pending_json: serde_json::Value = serde_json::from_slice(&outgoing_pending_body)
        .expect("outgoing pending inventory body should be valid json");
    let outgoing_pending_items = outgoing_pending_json["items"]
        .as_array()
        .expect("outgoing pending inventory should include items");
    assert!(outgoing_pending_items.is_empty());

    let outgoing_canceled_list = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(
                    "/backend/v3/api/control/social/friend_requests?userId=u_bob&direction=outgoing&status=canceled",
                )
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("outgoing canceled friend request inventory should return response");
    assert_eq!(outgoing_canceled_list.status(), StatusCode::OK);
    let outgoing_canceled_body = outgoing_canceled_list
        .into_body()
        .collect()
        .await
        .expect("outgoing canceled inventory body should collect")
        .to_bytes();
    let outgoing_canceled_json: serde_json::Value = serde_json::from_slice(&outgoing_canceled_body)
        .expect("outgoing canceled inventory body should be valid json");
    let outgoing_canceled_items = outgoing_canceled_json["items"]
        .as_array()
        .expect("outgoing canceled inventory should include items");
    assert_eq!(outgoing_canceled_items.len(), 1);
    assert_eq!(
        outgoing_canceled_items[0]["requestId"],
        "fr_list_canceled_001"
    );
    assert_eq!(outgoing_canceled_items[0]["status"], "canceled");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_list_applies_limit_after_sorting() {
    let app = control_plane_api::build_app();

    let first_submit = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_list_limit_001",
                        "eventId":"evt_list_limit_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first limit friend request submit should return response");
    assert_eq!(first_submit.status(), StatusCode::OK);

    let second_submit = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_list_limit_002",
                        "eventId":"evt_list_limit_002_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_carol",
                        "requestedAt":"2026-04-10T10:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second limit friend request submit should return response");
    assert_eq!(second_submit.status(), StatusCode::OK);

    let list_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests?userId=u_alice&direction=outgoing&limit=1")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("limited friend request inventory should return response");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("limited friend request inventory body should collect")
        .to_bytes();
    let list_json: serde_json::Value = serde_json::from_slice(&list_body)
        .expect("limited friend request inventory body should be valid json");
    let items = list_json["items"]
        .as_array()
        .expect("limited friend request inventory should include items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["requestId"], "fr_list_limit_002");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_list_uses_cursor_for_next_page() {
    let _guard = friend_request_cursor_env_guard().await;
    let _secret = ScopedEnvVar::set(
        FRIEND_REQUEST_CURSOR_SECRET_ENV,
        TEST_FRIEND_REQUEST_CURSOR_SECRET,
    );
    let app = control_plane_api::build_app();

    for (request_id, event_id, target_user_id, requested_at) in [
        (
            "fr_list_cursor_001",
            "evt_list_cursor_001_submit",
            "u_bob",
            "2026-04-10T10:00:00Z",
        ),
        (
            "fr_list_cursor_002",
            "evt_list_cursor_002_submit",
            "u_carol",
            "2026-04-10T10:05:00Z",
        ),
    ] {
        let submit_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/backend/v3/api/control/social/friend_requests")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "admin")
                    .header("x-sdkwork-permission-scope", "control.write")
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "requestId":"{request_id}",
                            "eventId":"{event_id}",
                            "requesterUserId":"u_alice",
                            "targetUserId":"{target_user_id}",
                            "requestedAt":"{requested_at}"
                        }}"#,
                    )))
                    .unwrap(),
            )
            .await
            .expect("cursor friend request submit should return response");
        assert_eq!(submit_response.status(), StatusCode::OK);
    }

    let first_page = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(
                    "/backend/v3/api/control/social/friend_requests?userId=u_alice&direction=outgoing&limit=1",
                )
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first friend request cursor page should return response");
    assert_eq!(first_page.status(), StatusCode::OK);
    let first_page_body = first_page
        .into_body()
        .collect()
        .await
        .expect("first friend request cursor page body should collect")
        .to_bytes();
    let first_page_json: serde_json::Value = serde_json::from_slice(&first_page_body)
        .expect("first friend request cursor page body should be valid json");
    let first_items = first_page_json["items"]
        .as_array()
        .expect("first friend request cursor page should include items");
    assert_eq!(first_items.len(), 1);
    assert_eq!(first_items[0]["requestId"], "fr_list_cursor_002");
    let next_cursor = first_page_json["nextCursor"]
        .as_str()
        .expect("first friend request cursor page should include nextCursor");

    let second_page = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests?userId=u_alice&direction=outgoing&limit=1&cursor={next_cursor}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second friend request cursor page should return response");
    assert_eq!(second_page.status(), StatusCode::OK);
    let second_page_body = second_page
        .into_body()
        .collect()
        .await
        .expect("second friend request cursor page body should collect")
        .to_bytes();
    let second_page_json: serde_json::Value = serde_json::from_slice(&second_page_body)
        .expect("second friend request cursor page body should be valid json");
    let second_items = second_page_json["items"]
        .as_array()
        .expect("second friend request cursor page should include items");
    assert_eq!(second_items.len(), 1);
    assert_eq!(second_items[0]["requestId"], "fr_list_cursor_001");
    assert!(second_page_json["nextCursor"].is_null());
}

#[tokio::test]
async fn test_control_plane_social_friend_request_list_rejects_invalid_cursor() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(
                    "/backend/v3/api/control/social/friend_requests?userId=u_alice&direction=outgoing&cursor=not-valid",
                )
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid cursor inventory request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("invalid cursor inventory body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("invalid cursor inventory body should be valid json");
    assert_eq!(json["code"], "cursor_invalid");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_list_emits_signed_versioned_cursor() {
    let _guard = friend_request_cursor_env_guard().await;
    let _cursor_secret = ScopedEnvVar::set(
        FRIEND_REQUEST_CURSOR_SECRET_ENV,
        TEST_FRIEND_REQUEST_CURSOR_SECRET,
    );
    let app = control_plane_api::build_app();

    for (request_id, target_user_id, requested_at) in [
        ("fr_list_signed_cursor_001", "u_bob", "2026-04-12T10:00:00Z"),
        (
            "fr_list_signed_cursor_002",
            "u_carol",
            "2026-04-12T10:01:00Z",
        ),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/backend/v3/api/control/social/friend_requests")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "admin")
                    .header("x-sdkwork-permission-scope", "control.write")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "requestId": request_id,
                            "eventId": format!("evt_{request_id}"),
                            "requesterUserId": "u_alice",
                            "targetUserId": target_user_id,
                            "requestMessage": format!("hello {target_user_id}"),
                            "requestedAt": requested_at,
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("friend request seed before signed cursor list should return response");
        assert_eq!(response.status(), StatusCode::OK);
    }

    let first_page = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests?userId=u_alice&direction=outgoing&limit=1")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("signed cursor inventory request should return response");
    assert_eq!(first_page.status(), StatusCode::OK);
    let first_page_body = first_page
        .into_body()
        .collect()
        .await
        .expect("signed cursor inventory body should collect")
        .to_bytes();
    let first_page_json: serde_json::Value = serde_json::from_slice(&first_page_body)
        .expect("signed cursor inventory body should be valid json");
    let next_cursor = first_page_json["nextCursor"]
        .as_str()
        .expect("signed cursor inventory should include nextCursor");
    let payload = decode_friend_request_cursor_payload(next_cursor);

    assert_eq!(payload["v"], 1);
    assert_eq!(payload["requestId"], "fr_list_signed_cursor_002");
    assert_eq!(payload["createdAt"], "2026-04-12T10:01:00Z");
    assert_eq!(payload["updatedAt"], "2026-04-12T10:01:00Z");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_list_rejects_tampered_cursor_signature() {
    let _guard = friend_request_cursor_env_guard().await;
    let _cursor_secret = ScopedEnvVar::set(
        FRIEND_REQUEST_CURSOR_SECRET_ENV,
        TEST_FRIEND_REQUEST_CURSOR_SECRET,
    );
    let app = control_plane_api::build_app();

    for (request_id, target_user_id, requested_at) in [
        ("fr_list_tamper_cursor_001", "u_bob", "2026-04-12T11:00:00Z"),
        (
            "fr_list_tamper_cursor_002",
            "u_carol",
            "2026-04-12T11:01:00Z",
        ),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/backend/v3/api/control/social/friend_requests")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "admin")
                    .header("x-sdkwork-permission-scope", "control.write")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "requestId": request_id,
                            "eventId": format!("evt_{request_id}"),
                            "requesterUserId": "u_alice",
                            "targetUserId": target_user_id,
                            "requestMessage": format!("hello {target_user_id}"),
                            "requestedAt": requested_at,
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("friend request seed before tampered cursor list should return response");
        assert_eq!(response.status(), StatusCode::OK);
    }

    let first_page = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests?userId=u_alice&direction=outgoing&limit=1")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("tampered cursor first page should return response");
    assert_eq!(first_page.status(), StatusCode::OK);
    let first_page_body = first_page
        .into_body()
        .collect()
        .await
        .expect("tampered cursor first page body should collect")
        .to_bytes();
    let first_page_json: serde_json::Value = serde_json::from_slice(&first_page_body)
        .expect("tampered cursor first page body should be valid json");
    let next_cursor = first_page_json["nextCursor"]
        .as_str()
        .expect("tampered cursor first page should include nextCursor");
    let mut payload = decode_friend_request_cursor_payload(next_cursor);
    payload["requestId"] = serde_json::Value::String("fr_list_tamper_cursor_001".into());
    let tampered_cursor = replace_friend_request_cursor_payload(next_cursor, &payload);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests?userId=u_alice&direction=outgoing&limit=1&cursor={tampered_cursor}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("tampered cursor inventory request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("tampered cursor inventory body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("tampered cursor inventory body should be valid json");
    assert_eq!(json["code"], "cursor_invalid");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_list_rejects_unsupported_cursor_version() {
    let _guard = friend_request_cursor_env_guard().await;
    let _cursor_secret = ScopedEnvVar::set(
        FRIEND_REQUEST_CURSOR_SECRET_ENV,
        TEST_FRIEND_REQUEST_CURSOR_SECRET,
    );
    let app = control_plane_api::build_app();

    let cursor = sign_friend_request_cursor_for_test(&serde_json::json!({
        "v": 999,
        "updatedAt": "2026-04-12T12:00:00Z",
        "createdAt": "2026-04-12T12:00:00Z",
        "requestId": "fr_cursor_unknown_version_001"
    }));

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests?userId=u_alice&direction=outgoing&cursor={cursor}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")

                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("unsupported cursor version inventory request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("unsupported cursor version inventory body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body)
        .expect("unsupported cursor version inventory body should be valid json");
    assert_eq!(json["code"], "cursor_invalid");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_accept_updates_snapshot_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_accept_001",
                        "eventId":"evt_accept_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestMessage":"hello",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before accept should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_001/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_accept_001_accept",
                        "acceptedByUserId":"u_bob",
                        "acceptedAt":"2026-04-10T10:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request accept should return response");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("friend request accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value = serde_json::from_slice(&accept_body)
        .expect("friend request accept body should be valid json");

    assert_eq!(accept_json["status"], "accepted");
    assert_eq!(accept_json["friendRequest"]["requestId"], "fr_accept_001");
    assert_eq!(accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(accept_json["friendship"]["status"], "active");
    assert_eq!(accept_json["directChat"]["status"], "active");
    let friendship_id = accept_json["friendship"]["friendshipId"]
        .as_str()
        .expect("accepted response should include friendship id")
        .to_owned();
    let direct_chat_id = accept_json["directChat"]["directChatId"]
        .as_str()
        .expect("accepted response should include direct chat id")
        .to_owned();
    let conversation_id = accept_json["directChat"]["conversationId"]
        .as_str()
        .expect("accepted response should include bound conversation id")
        .to_owned();
    assert_eq!(
        accept_json["latestCommit"]["eventType"],
        "friend_request.accepted"
    );
    assert_eq!(
        accept_json["latestCommit"]["payloadSchema"],
        "social.friend_request.accepted.v1"
    );
    assert_eq!(accept_json["latestCommit"]["orderingSeq"], 2);

    let snapshot_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after accept should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request snapshot after accept should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request snapshot after accept should be valid json");

    assert_eq!(snapshot_json["friendRequest"]["status"], "accepted");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 2);
    assert_eq!(
        snapshot_json["commits"][1]["eventType"],
        "friend_request.accepted"
    );

    let friendship_snapshot_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friendships/{friendship_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot after accept should return response");
    assert_eq!(friendship_snapshot_response.status(), StatusCode::OK);
    let friendship_snapshot_body = friendship_snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friendship snapshot after accept should collect")
        .to_bytes();
    let friendship_snapshot_json: serde_json::Value =
        serde_json::from_slice(&friendship_snapshot_body)
            .expect("friendship snapshot after accept should be valid json");
    assert_eq!(friendship_snapshot_json["friendship"]["status"], "active");

    let direct_chat_snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/direct_chats/{direct_chat_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("direct chat snapshot after accept should return response");
    assert_eq!(direct_chat_snapshot_response.status(), StatusCode::OK);
    let direct_chat_snapshot_body = direct_chat_snapshot_response
        .into_body()
        .collect()
        .await
        .expect("direct chat snapshot after accept should collect")
        .to_bytes();
    let direct_chat_snapshot_json: serde_json::Value =
        serde_json::from_slice(&direct_chat_snapshot_body)
            .expect("direct chat snapshot after accept should be valid json");
    assert_eq!(direct_chat_snapshot_json["directChat"]["status"], "active");
    assert_eq!(
        direct_chat_snapshot_json["directChat"]["conversationId"],
        conversation_id
    );

    let audit_auth = AppContext {
        tenant_id: "t_demo".into(),
        organization_id: None,
        user_id: "u_admin".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        app_id: Some("craw-chat".into()),
        environment: None,
        deployment_mode: None,
        device_id: None,
        permission_scope: BTreeSet::new(),
        data_scope: BTreeSet::new(),
        auth_level: None,
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 4);
    assert!(
        audit_export
            .items
            .iter()
            .any(|item| item.action == "control.friend_request_submitted"
                && item
                    .payload
                    .as_deref()
                    .is_some_and(|payload| payload.contains("\"requestId\":\"fr_accept_001\""))),
        "friend request submit audit record should be persisted"
    );
    assert!(
        audit_export
            .items
            .iter()
            .any(|item| item.action == "control.friend_request_accepted"
                && item
                    .payload
                    .as_deref()
                    .is_some_and(|payload| payload.contains("\"requestId\":\"fr_accept_001\""))),
        "friend request accept audit record should be persisted"
    );
    assert!(
        audit_export
            .items
            .iter()
            .any(|item| item.action == "control.friendship_activated"
                && item
                    .payload
                    .as_deref()
                    .is_some_and(|payload| payload.contains(friendship_id.as_str()))),
        "friendship activation audit record should be persisted"
    );
    assert!(
        audit_export
            .items
            .iter()
            .any(|item| item.action == "control.direct_chat_bound"
                && item
                    .payload
                    .as_deref()
                    .is_some_and(|payload| payload.contains(direct_chat_id.as_str()))),
        "direct chat bind audit record should be persisted"
    );
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_restart_repairs_atomic_friend_request_accept_materialization()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(state_file(runtime_dir.as_path(), "").as_path())
        .expect("state dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_accept_failpoint_001",
                        "eventId":"evt_accept_failpoint_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T16:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before failpoint accept should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    fs::write(
        social_failpoint_file(runtime_dir.as_path()),
        r#"{"failNextSnapshotSave":true}"#,
    )
    .expect("social failpoint file should be written");

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/backend/v3/api/control/social/friend_requests/fr_accept_failpoint_001/accept",
                )
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_accept_failpoint_001_accept",
                        "acceptedByUserId":"u_bob",
                        "acceptedAt":"2026-04-10T16:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request accept with failpoint should return response");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("friend request accept with failpoint body should collect")
        .to_bytes();
    let accept_json: serde_json::Value = serde_json::from_slice(&accept_body)
        .expect("accept failpoint response should be valid json");
    assert_eq!(accept_json["status"], "accepted");
    assert_eq!(accept_json["persistence"]["journalAuthority"], true);
    assert_eq!(
        accept_json["persistence"]["snapshotStatus"],
        "repair_required"
    );
    let friendship_id = accept_json["friendship"]["friendshipId"]
        .as_str()
        .expect("accept failpoint response should include friendship")
        .to_owned();
    let direct_chat_id = accept_json["directChat"]["directChatId"]
        .as_str()
        .expect("accept failpoint response should include direct chat")
        .to_owned();
    let conversation_id = accept_json["directChat"]["conversationId"]
        .as_str()
        .expect("accept failpoint response should include conversation id")
        .to_owned();

    let marker_path = social_tx_marker_file(runtime_dir.as_path());
    assert!(
        marker_path.exists(),
        "pending social tx marker should exist after accept snapshot save failure"
    );

    let cluster_after = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime_after = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster_after,
        ops_runtime_after,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    assert!(
        !marker_path.exists(),
        "pending social tx marker should be cleared after restart replay repairs accepted materialization"
    );

    let friendship_snapshot_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friendships/{friendship_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot after accept restart repair should return response");
    assert_eq!(friendship_snapshot_response.status(), StatusCode::OK);

    let direct_chat_snapshot_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/direct_chats/{direct_chat_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("direct chat snapshot after accept restart repair should return response");
    assert_eq!(direct_chat_snapshot_response.status(), StatusCode::OK);
    let direct_chat_snapshot_body = direct_chat_snapshot_response
        .into_body()
        .collect()
        .await
        .expect("direct chat snapshot after accept restart repair should collect")
        .to_bytes();
    let direct_chat_snapshot_json: serde_json::Value =
        serde_json::from_slice(&direct_chat_snapshot_body)
            .expect("direct chat snapshot after accept restart repair should be valid json");
    assert_eq!(
        direct_chat_snapshot_json["directChat"]["conversationId"],
        conversation_id
    );

    let friend_request_snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_failpoint_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after accept restart repair should return response");
    assert_eq!(friend_request_snapshot_response.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_friend_request_submit_rejects_active_friendship_block() {
    let app = control_plane_api::build_app();

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_friend_request_submit_blocked",
                        "eventId":"evt_ub_friend_request_submit_blocked",
                        "blockerUserId":"u_bob",
                        "blockedUserId":"u_alice",
                        "scope":"friendship",
                        "effectiveAt":"2026-04-10T09:59:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user block before friend request submit should return response");
    assert_eq!(block_response.status(), StatusCode::OK);

    let submit_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_submit_blocked_001",
                        "eventId":"evt_submit_blocked_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("blocked friend request submit should return response");

    assert_eq!(submit_response.status(), StatusCode::CONFLICT);
    let submit_body = submit_response
        .into_body()
        .collect()
        .await
        .expect("blocked friend request submit body should collect")
        .to_bytes();
    let submit_json: serde_json::Value = serde_json::from_slice(&submit_body)
        .expect("blocked friend request submit body should be valid json");
    assert_eq!(submit_json["status"], 409);
    assert_eq!(submit_json["errorStatus"], "conflict");
    assert_eq!(submit_json["code"], "friend_request_blocked");
    assert_eq!(
        submit_json["details"]["blockId"],
        "ub_friend_request_submit_blocked"
    );
    assert_eq!(submit_json["details"]["scope"], "friendship");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_accept_rejects_active_friendship_block() {
    let app = control_plane_api::build_app();

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_accept_blocked_001",
                        "eventId":"evt_accept_blocked_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before blocked accept should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_friend_request_accept_blocked",
                        "eventId":"evt_ub_friend_request_accept_blocked",
                        "blockerUserId":"u_bob",
                        "blockedUserId":"u_alice",
                        "scope":"friendship",
                        "effectiveAt":"2026-04-10T10:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user block before friend request accept should return response");
    assert_eq!(block_response.status(), StatusCode::OK);

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_blocked_001/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_accept_blocked_001_accept",
                        "acceptedByUserId":"u_bob",
                        "acceptedAt":"2026-04-10T10:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("blocked friend request accept should return response");

    assert_eq!(accept_response.status(), StatusCode::CONFLICT);
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("blocked friend request accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value = serde_json::from_slice(&accept_body)
        .expect("blocked friend request accept body should be valid json");
    assert_eq!(accept_json["status"], 409);
    assert_eq!(accept_json["errorStatus"], "conflict");
    assert_eq!(accept_json["code"], "friend_request_blocked");
    assert_eq!(
        accept_json["details"]["blockId"],
        "ub_friend_request_accept_blocked"
    );
    assert_eq!(accept_json["details"]["scope"], "friendship");

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_blocked_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after blocked accept should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);
    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request snapshot after blocked accept should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request snapshot after blocked accept should be valid json");
    assert_eq!(snapshot_json["friendRequest"]["status"], "pending");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_decline_updates_snapshot_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_decline_001",
                        "eventId":"evt_decline_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestMessage":"hello",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before decline should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let decline_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_decline_001/decline")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_decline_001_decline",
                        "declinedByUserId":"u_bob",
                        "declinedAt":"2026-04-10T10:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request decline should return response");
    assert_eq!(decline_response.status(), StatusCode::OK);

    let decline_body = decline_response
        .into_body()
        .collect()
        .await
        .expect("friend request decline body should collect")
        .to_bytes();
    let decline_json: serde_json::Value = serde_json::from_slice(&decline_body)
        .expect("friend request decline body should be valid json");

    assert_eq!(decline_json["status"], "declined");
    assert_eq!(decline_json["friendRequest"]["requestId"], "fr_decline_001");
    assert_eq!(decline_json["friendRequest"]["status"], "declined");
    assert_eq!(
        decline_json["latestCommit"]["eventType"],
        "friend_request.declined"
    );
    assert_eq!(
        decline_json["latestCommit"]["payloadSchema"],
        "social.friend_request.declined.v1"
    );
    assert_eq!(decline_json["latestCommit"]["orderingSeq"], 2);

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_decline_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after decline should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request snapshot after decline should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request snapshot after decline should be valid json");

    assert_eq!(snapshot_json["friendRequest"]["status"], "declined");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 2);
    assert_eq!(
        snapshot_json["commits"][1]["eventType"],
        "friend_request.declined"
    );

    let audit_auth = AppContext {
        tenant_id: "t_demo".into(),
        organization_id: None,
        user_id: "u_admin".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        app_id: Some("craw-chat".into()),
        environment: None,
        deployment_mode: None,
        device_id: None,
        permission_scope: BTreeSet::new(),
        data_scope: BTreeSet::new(),
        auth_level: None,
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 2);
    assert!(
        audit_export
            .items
            .iter()
            .any(|item| item.action == "control.friend_request_declined"
                && item
                    .payload
                    .as_deref()
                    .is_some_and(|payload| payload.contains("\"requestId\":\"fr_decline_001\""))),
        "friend request decline audit record should be persisted"
    );
}

#[tokio::test]
async fn test_control_plane_social_friend_request_cancel_updates_snapshot_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_cancel_001",
                        "eventId":"evt_cancel_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestMessage":"hello",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before cancel should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let cancel_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_cancel_001/cancel")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_cancel_001_cancel",
                        "canceledByUserId":"u_alice",
                        "canceledAt":"2026-04-10T10:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request cancel should return response");
    assert_eq!(cancel_response.status(), StatusCode::OK);

    let cancel_body = cancel_response
        .into_body()
        .collect()
        .await
        .expect("friend request cancel body should collect")
        .to_bytes();
    let cancel_json: serde_json::Value = serde_json::from_slice(&cancel_body)
        .expect("friend request cancel body should be valid json");

    assert_eq!(cancel_json["status"], "canceled");
    assert_eq!(cancel_json["friendRequest"]["requestId"], "fr_cancel_001");
    assert_eq!(cancel_json["friendRequest"]["status"], "canceled");
    assert_eq!(
        cancel_json["latestCommit"]["eventType"],
        "friend_request.canceled"
    );
    assert_eq!(
        cancel_json["latestCommit"]["payloadSchema"],
        "social.friend_request.canceled.v1"
    );
    assert_eq!(cancel_json["latestCommit"]["orderingSeq"], 2);

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_cancel_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after cancel should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request snapshot after cancel should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request snapshot after cancel should be valid json");

    assert_eq!(snapshot_json["friendRequest"]["status"], "canceled");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 2);
    assert_eq!(
        snapshot_json["commits"][1]["eventType"],
        "friend_request.canceled"
    );

    let audit_auth = AppContext {
        tenant_id: "t_demo".into(),
        organization_id: None,
        user_id: "u_admin".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        app_id: Some("craw-chat".into()),
        environment: None,
        deployment_mode: None,
        device_id: None,
        permission_scope: BTreeSet::new(),
        data_scope: BTreeSet::new(),
        auth_level: None,
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 2);
    assert!(
        audit_export
            .items
            .iter()
            .any(|item| item.action == "control.friend_request_canceled"
                && item
                    .payload
                    .as_deref()
                    .is_some_and(|payload| payload.contains("\"requestId\":\"fr_cancel_001\""))),
        "friend request cancel audit record should be persisted"
    );
}

#[tokio::test]
async fn test_control_plane_social_friendship_activation_persists_snapshot_commit_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let activate_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_001",
                        "eventId":"evt_fs_001",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "directChatId":"dc_001",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friendship activation should return response");
    assert_eq!(activate_response.status(), StatusCode::OK);

    let activate_body = activate_response
        .into_body()
        .collect()
        .await
        .expect("activation body should collect")
        .to_bytes();
    let activate_json: serde_json::Value =
        serde_json::from_slice(&activate_body).expect("activation body should be valid json");

    assert_eq!(activate_json["status"], "activated");
    assert_eq!(activate_json["friendship"]["tenantId"], "t_demo");
    assert_eq!(activate_json["friendship"]["friendshipId"], "fs_001");
    assert_eq!(activate_json["friendship"]["userLowId"], "u_alice");
    assert_eq!(activate_json["friendship"]["userHighId"], "u_bob");
    assert_eq!(activate_json["friendship"]["initiatorUserId"], "u_alice");
    assert_eq!(activate_json["friendship"]["status"], "active");
    assert_eq!(
        activate_json["latestCommit"]["eventType"],
        "friendship.activated"
    );
    assert_eq!(activate_json["latestCommit"]["scopeType"], "friendship");
    assert_eq!(
        activate_json["latestCommit"]["payloadSchema"],
        "social.friendship.activated.v1"
    );
    assert_eq!(activate_json["latestCommit"]["orderingSeq"], 1);
    assert!(
        activate_json["latestCommit"]["payload"]
            .as_str()
            .expect("payload should be string")
            .contains("\"directChatId\":\"dc_001\"")
    );

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friendships/fs_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot body should be valid json");

    assert_eq!(snapshot_json["status"], "snapshot");
    assert_eq!(snapshot_json["friendship"]["friendshipId"], "fs_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "friendship.activated"
    );

    let audit_auth = AppContext {
        tenant_id: "t_demo".into(),
        organization_id: None,
        user_id: "u_admin".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        app_id: Some("craw-chat".into()),
        environment: None,
        deployment_mode: None,
        device_id: None,
        permission_scope: BTreeSet::new(),
        data_scope: BTreeSet::new(),
        auth_level: None,
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 1);
    assert_eq!(audit_export.items[0].action, "control.friendship_activated");
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("friendship audit should include payload")
            .contains("\"friendshipId\":\"fs_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_friendship_activation_rejects_active_friendship_block() {
    let app = control_plane_api::build_app();

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_friendship_activate_blocked",
                        "eventId":"evt_ub_friendship_activate_blocked",
                        "blockerUserId":"u_bob",
                        "blockedUserId":"u_alice",
                        "scope":"friendship",
                        "effectiveAt":"2026-04-10T10:59:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user block before friendship activation should return response");
    assert_eq!(block_response.status(), StatusCode::OK);

    let activate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_blocked_001",
                        "eventId":"evt_fs_blocked_001",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "directChatId":"dc_blocked_001",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("blocked friendship activation should return response");

    assert_eq!(activate_response.status(), StatusCode::CONFLICT);
    let activate_body = activate_response
        .into_body()
        .collect()
        .await
        .expect("blocked friendship activation body should collect")
        .to_bytes();
    let activate_json: serde_json::Value = serde_json::from_slice(&activate_body)
        .expect("blocked friendship activation body should be valid json");
    assert_eq!(activate_json["status"], 409);
    assert_eq!(activate_json["errorStatus"], "conflict");
    assert_eq!(activate_json["code"], "friendship_blocked");
    assert_eq!(
        activate_json["details"]["blockId"],
        "ub_friendship_activate_blocked"
    );
    assert_eq!(activate_json["details"]["scope"], "friendship");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_accept_replays_duplicate_event_idempotently() {
    let app = control_plane_api::build_app();

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_accept_replay_001",
                        "eventId":"evt_accept_replay_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before replayed accept should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let accept_body = r#"{
        "eventId":"evt_accept_replay_001_accept",
        "acceptedByUserId":"u_bob",
        "acceptedAt":"2026-04-10T10:05:00Z"
    }"#;

    let first_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_replay_001/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(accept_body))
                .unwrap(),
        )
        .await
        .expect("first replayable accept should return response");
    assert_eq!(first_accept.status(), StatusCode::OK);
    let first_accept_body = first_accept
        .into_body()
        .collect()
        .await
        .expect("first replayable accept body should collect")
        .to_bytes();
    let first_accept_json: serde_json::Value = serde_json::from_slice(&first_accept_body)
        .expect("first replayable accept body should be valid json");

    let second_accept = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_replay_001/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(accept_body))
                .unwrap(),
        )
        .await
        .expect("duplicate accept replay should return response");
    assert_eq!(second_accept.status(), StatusCode::OK);
    let second_accept_body = second_accept
        .into_body()
        .collect()
        .await
        .expect("duplicate accept replay body should collect")
        .to_bytes();
    let second_accept_json: serde_json::Value = serde_json::from_slice(&second_accept_body)
        .expect("duplicate accept replay body should be valid json");

    assert_eq!(second_accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(
        second_accept_json["latestCommit"]["eventId"],
        first_accept_json["latestCommit"]["eventId"]
    );
    assert_eq!(
        second_accept_json["latestCommit"]["orderingSeq"],
        first_accept_json["latestCommit"]["orderingSeq"]
    );
}

#[tokio::test]
async fn test_control_plane_social_friend_request_accept_rejects_new_event_after_request_already_accepted()
 {
    let app = control_plane_api::build_app();

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_accept_reject_new_event_001",
                        "eventId":"evt_accept_reject_new_event_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before duplicate accept guard should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let first_accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_reject_new_event_001/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_accept_reject_new_event_001_accept",
                        "acceptedByUserId":"u_bob",
                        "acceptedAt":"2026-04-10T10:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("initial friend request accept should return response");
    assert_eq!(first_accept_response.status(), StatusCode::OK);

    let second_accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_accept_reject_new_event_001/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_accept_reject_new_event_001_accept_duplicate",
                        "acceptedByUserId":"u_bob",
                        "acceptedAt":"2026-04-10T10:06:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate accept with new event id should return response");

    assert_eq!(second_accept_response.status(), StatusCode::CONFLICT);
    let second_accept_body = second_accept_response
        .into_body()
        .collect()
        .await
        .expect("duplicate accept with new event id body should collect")
        .to_bytes();
    let second_accept_json: serde_json::Value = serde_json::from_slice(&second_accept_body)
        .expect("duplicate accept with new event id body should be valid json");
    assert_eq!(second_accept_json["code"], "friend_request_not_pending");

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(
                    "/backend/v3/api/control/social/friend_requests/fr_accept_reject_new_event_001",
                )
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after duplicate accept guard should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);
    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request snapshot after duplicate accept guard should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request snapshot after duplicate accept guard should be valid json");
    assert_eq!(snapshot_json["friendRequest"]["status"], "accepted");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn test_control_plane_social_friend_request_decline_replays_duplicate_event_idempotently() {
    let app = control_plane_api::build_app();

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_decline_replay_001",
                        "eventId":"evt_decline_replay_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before replayed decline should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let decline_body = r#"{
        "eventId":"evt_decline_replay_001_decline",
        "declinedByUserId":"u_bob",
        "declinedAt":"2026-04-10T10:05:00Z"
    }"#;

    let first_decline = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_decline_replay_001/decline")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(decline_body))
                .unwrap(),
        )
        .await
        .expect("first replayable decline should return response");
    assert_eq!(first_decline.status(), StatusCode::OK);
    let first_decline_body = first_decline
        .into_body()
        .collect()
        .await
        .expect("first replayable decline body should collect")
        .to_bytes();
    let first_decline_json: serde_json::Value = serde_json::from_slice(&first_decline_body)
        .expect("first replayable decline body should be valid json");

    let second_decline = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_decline_replay_001/decline")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(decline_body))
                .unwrap(),
        )
        .await
        .expect("duplicate decline replay should return response");
    assert_eq!(second_decline.status(), StatusCode::OK);
    let second_decline_body = second_decline
        .into_body()
        .collect()
        .await
        .expect("duplicate decline replay body should collect")
        .to_bytes();
    let second_decline_json: serde_json::Value = serde_json::from_slice(&second_decline_body)
        .expect("duplicate decline replay body should be valid json");

    assert_eq!(second_decline_json["friendRequest"]["status"], "declined");
    assert_eq!(
        second_decline_json["latestCommit"]["eventId"],
        first_decline_json["latestCommit"]["eventId"]
    );
    assert_eq!(
        second_decline_json["latestCommit"]["orderingSeq"],
        first_decline_json["latestCommit"]["orderingSeq"]
    );
}

#[tokio::test]
async fn test_control_plane_social_friend_request_cancel_replays_duplicate_event_idempotently() {
    let app = control_plane_api::build_app();

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_cancel_replay_001",
                        "eventId":"evt_cancel_replay_001_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit before replayed cancel should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let cancel_body = r#"{
        "eventId":"evt_cancel_replay_001_cancel",
        "canceledByUserId":"u_alice",
        "canceledAt":"2026-04-10T10:05:00Z"
    }"#;

    let first_cancel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_cancel_replay_001/cancel")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(cancel_body))
                .unwrap(),
        )
        .await
        .expect("first replayable cancel should return response");
    assert_eq!(first_cancel.status(), StatusCode::OK);
    let first_cancel_body = first_cancel
        .into_body()
        .collect()
        .await
        .expect("first replayable cancel body should collect")
        .to_bytes();
    let first_cancel_json: serde_json::Value = serde_json::from_slice(&first_cancel_body)
        .expect("first replayable cancel body should be valid json");

    let second_cancel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests/fr_cancel_replay_001/cancel")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(cancel_body))
                .unwrap(),
        )
        .await
        .expect("duplicate cancel replay should return response");
    assert_eq!(second_cancel.status(), StatusCode::OK);
    let second_cancel_body = second_cancel
        .into_body()
        .collect()
        .await
        .expect("duplicate cancel replay body should collect")
        .to_bytes();
    let second_cancel_json: serde_json::Value = serde_json::from_slice(&second_cancel_body)
        .expect("duplicate cancel replay body should be valid json");

    assert_eq!(second_cancel_json["friendRequest"]["status"], "canceled");
    assert_eq!(
        second_cancel_json["latestCommit"]["eventId"],
        first_cancel_json["latestCommit"]["eventId"]
    );
    assert_eq!(
        second_cancel_json["latestCommit"]["orderingSeq"],
        first_cancel_json["latestCommit"]["orderingSeq"]
    );
}

#[tokio::test]
async fn test_control_plane_social_friendship_removal_updates_snapshot_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let activate_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_remove_001",
                        "eventId":"evt_fs_remove_001_activate",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "directChatId":"dc_remove_001",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friendship activation before removal should return response");
    assert_eq!(activate_response.status(), StatusCode::OK);

    let remove_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships/fs_remove_001/remove")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_fs_remove_001_remove",
                        "removedByUserId":"u_alice",
                        "removedAt":"2026-04-10T11:30:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friendship removal should return response");
    assert_eq!(remove_response.status(), StatusCode::OK);

    let remove_body = remove_response
        .into_body()
        .collect()
        .await
        .expect("friendship removal body should collect")
        .to_bytes();
    let remove_json: serde_json::Value =
        serde_json::from_slice(&remove_body).expect("friendship removal body should be valid json");

    assert_eq!(remove_json["status"], "removed");
    assert_eq!(remove_json["friendship"]["friendshipId"], "fs_remove_001");
    assert_eq!(remove_json["friendship"]["status"], "removed");
    assert_eq!(
        remove_json["latestCommit"]["eventType"],
        "friendship.removed"
    );
    assert_eq!(
        remove_json["latestCommit"]["payloadSchema"],
        "social.friendship.removed.v1"
    );
    assert_eq!(remove_json["latestCommit"]["orderingSeq"], 2);

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friendships/fs_remove_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot after removal should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friendship snapshot after removal should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friendship snapshot after removal should be valid json");

    assert_eq!(snapshot_json["friendship"]["status"], "removed");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 2);
    assert_eq!(
        snapshot_json["commits"][1]["eventType"],
        "friendship.removed"
    );

    let audit_auth = AppContext {
        tenant_id: "t_demo".into(),
        organization_id: None,
        user_id: "u_admin".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        app_id: Some("craw-chat".into()),
        environment: None,
        deployment_mode: None,
        device_id: None,
        permission_scope: BTreeSet::new(),
        data_scope: BTreeSet::new(),
        auth_level: None,
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 2);
    assert!(
        audit_export
            .items
            .iter()
            .any(|item| item.action == "control.friendship_removed"
                && item
                    .payload
                    .as_deref()
                    .is_some_and(|payload| payload.contains("\"friendshipId\":\"fs_remove_001\""))),
        "friendship removal audit record should be persisted"
    );
}

#[tokio::test]
async fn test_control_plane_social_friendship_removal_archives_direct_chat_pair_and_allows_rebind()
{
    let app = control_plane_api::build_app();

    let activate_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_remove_archives_dc_001",
                        "eventId":"evt_fs_remove_archives_dc_activate",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "directChatId":"dc_remove_archives_dc_001",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friendship activation before direct chat archive test should return response");
    assert_eq!(activate_response.status(), StatusCode::OK);

    let bind_direct_chat = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_remove_archives_dc_001",
                        "eventId":"evt_dc_remove_archives_dc_bind_001",
                        "leftActorId":"u_alice",
                        "rightActorId":"u_bob",
                        "conversationId":"c_remove_archives_dc_001",
                        "boundAt":"2026-04-10T11:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind before direct chat archive test should return response");
    assert_eq!(bind_direct_chat.status(), StatusCode::OK);

    let remove_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships/fs_remove_archives_dc_001/remove")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "eventId":"evt_fs_remove_archives_dc_remove",
                        "removedByUserId":"u_alice",
                        "removedAt":"2026-04-10T11:30:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friendship removal before direct chat archive test should return response");
    assert_eq!(remove_response.status(), StatusCode::OK);

    let direct_chat_snapshot = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/direct_chats/dc_remove_archives_dc_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("direct chat snapshot after friendship removal should return response");
    assert_eq!(direct_chat_snapshot.status(), StatusCode::OK);
    let direct_chat_snapshot_body = direct_chat_snapshot
        .into_body()
        .collect()
        .await
        .expect("direct chat snapshot after friendship removal body should collect")
        .to_bytes();
    let direct_chat_snapshot_json: serde_json::Value =
        serde_json::from_slice(&direct_chat_snapshot_body)
            .expect("direct chat snapshot after friendship removal body should be valid json");
    assert_eq!(
        direct_chat_snapshot_json["directChat"]["status"],
        "archived"
    );

    let rebind_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_remove_archives_dc_002",
                        "eventId":"evt_dc_remove_archives_dc_bind_002",
                        "leftActorId":"u_bob",
                        "rightActorId":"u_alice",
                        "conversationId":"c_remove_archives_dc_002",
                        "boundAt":"2026-04-10T11:31:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat rebind after friendship removal should return response");
    assert_eq!(rebind_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_control_plane_social_friendship_remove_replays_duplicate_event_idempotently() {
    let app = control_plane_api::build_app();

    let activate_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_remove_replay_001",
                        "eventId":"evt_fs_remove_replay_001_activate",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friendship activation before replayed remove should return response");
    assert_eq!(activate_response.status(), StatusCode::OK);

    let remove_body = r#"{
        "eventId":"evt_fs_remove_replay_001_remove",
        "removedByUserId":"u_alice",
        "removedAt":"2026-04-10T11:30:00Z"
    }"#;

    let first_remove = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships/fs_remove_replay_001/remove")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(remove_body))
                .unwrap(),
        )
        .await
        .expect("first replayable remove should return response");
    assert_eq!(first_remove.status(), StatusCode::OK);
    let first_remove_body = first_remove
        .into_body()
        .collect()
        .await
        .expect("first replayable remove body should collect")
        .to_bytes();
    let first_remove_json: serde_json::Value = serde_json::from_slice(&first_remove_body)
        .expect("first replayable remove body should be valid json");

    let second_remove = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships/fs_remove_replay_001/remove")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(remove_body))
                .unwrap(),
        )
        .await
        .expect("duplicate remove replay should return response");
    assert_eq!(second_remove.status(), StatusCode::OK);
    let second_remove_body = second_remove
        .into_body()
        .collect()
        .await
        .expect("duplicate remove replay body should collect")
        .to_bytes();
    let second_remove_json: serde_json::Value = serde_json::from_slice(&second_remove_body)
        .expect("duplicate remove replay body should be valid json");

    assert_eq!(second_remove_json["friendship"]["status"], "removed");
    assert_eq!(
        second_remove_json["latestCommit"]["eventId"],
        first_remove_json["latestCommit"]["eventId"]
    );
    assert_eq!(
        second_remove_json["latestCommit"]["orderingSeq"],
        first_remove_json["latestCommit"]["orderingSeq"]
    );
}

#[tokio::test]
async fn test_control_plane_social_friendship_rejects_duplicate_active_pair() {
    let app = control_plane_api::build_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_001",
                        "eventId":"evt_fs_001",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first friendship activation should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    let duplicate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_002",
                        "eventId":"evt_fs_002",
                        "initiatorUserId":"u_bob",
                        "peerUserId":"u_alice",
                        "establishedAt":"2026-04-10T11:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate friendship activation should return response");

    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value =
        serde_json::from_slice(&duplicate_body).expect("duplicate body should be valid json");
    assert_eq!(duplicate_json["status"], 409);
    assert_eq!(duplicate_json["errorStatus"], "conflict");
    assert_eq!(duplicate_json["code"], "friendship_pair_conflict");
}

#[tokio::test]
async fn test_control_plane_social_direct_chat_binding_persists_snapshot_commit_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let bind_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_001",
                        "eventId":"evt_dc_001",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_direct_001",
                        "boundAt":"2026-04-10T12:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind should return response");
    assert_eq!(bind_response.status(), StatusCode::OK);

    let bind_body = bind_response
        .into_body()
        .collect()
        .await
        .expect("bind body should collect")
        .to_bytes();
    let bind_json: serde_json::Value =
        serde_json::from_slice(&bind_body).expect("bind body should be valid json");

    assert_eq!(bind_json["status"], "bound");
    assert_eq!(bind_json["directChat"]["tenantId"], "t_demo");
    assert_eq!(bind_json["directChat"]["directChatId"], "dc_001");
    assert_eq!(bind_json["directChat"]["leftActorId"], "actor_alice");
    assert_eq!(bind_json["directChat"]["rightActorId"], "actor_bob");
    assert_eq!(bind_json["directChat"]["pairHash"], "actor_alice:actor_bob");
    assert_eq!(bind_json["directChat"]["status"], "active");
    assert_eq!(bind_json["directChat"]["conversationId"], "c_direct_001");
    assert_eq!(bind_json["latestCommit"]["eventType"], "direct_chat.bound");
    assert_eq!(bind_json["latestCommit"]["scopeType"], "direct_chat");
    assert_eq!(
        bind_json["latestCommit"]["payloadSchema"],
        "social.direct_chat.bound.v1"
    );
    assert_eq!(bind_json["latestCommit"]["orderingSeq"], 1);
    assert!(
        bind_json["latestCommit"]["payload"]
            .as_str()
            .expect("payload should be string")
            .contains("\"conversationId\":\"c_direct_001\"")
    );

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/direct_chats/dc_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("direct chat snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot body should be valid json");

    assert_eq!(snapshot_json["status"], "snapshot");
    assert_eq!(snapshot_json["directChat"]["directChatId"], "dc_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "direct_chat.bound"
    );

    let audit_auth = AppContext {
        tenant_id: "t_demo".into(),
        organization_id: None,
        user_id: "u_admin".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        app_id: Some("craw-chat".into()),
        environment: None,
        deployment_mode: None,
        device_id: None,
        permission_scope: BTreeSet::new(),
        data_scope: BTreeSet::new(),
        auth_level: None,
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 1);
    assert_eq!(audit_export.items[0].action, "control.direct_chat_bound");
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("direct chat audit should include payload")
            .contains("\"directChatId\":\"dc_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_direct_chat_rejects_duplicate_active_pair() {
    let app = control_plane_api::build_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_001",
                        "eventId":"evt_dc_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_direct_001",
                        "boundAt":"2026-04-10T12:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first direct chat bind should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    let duplicate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_002",
                        "eventId":"evt_dc_002",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_direct_002",
                        "boundAt":"2026-04-10T12:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate direct chat bind should return response");

    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value =
        serde_json::from_slice(&duplicate_body).expect("duplicate body should be valid json");
    assert_eq!(duplicate_json["status"], 409);
    assert_eq!(duplicate_json["errorStatus"], "conflict");
    assert_eq!(duplicate_json["code"], "direct_chat_pair_conflict");
}

#[tokio::test]
async fn test_control_plane_social_user_block_persists_snapshot_commit_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let direct_chat_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_001",
                        "eventId":"evt_dc_before_ub_001",
                        "leftActorId":"u_alice",
                        "rightActorId":"u_bob",
                        "conversationId":"c_direct_before_ub_001",
                        "boundAt":"2026-04-10T12:09:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat before scoped user block should return response");
    assert_eq!(direct_chat_response.status(), StatusCode::OK);

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_001",
                        "eventId":"evt_ub_001",
                        "blockerUserId":"u_alice",
                        "blockedUserId":"u_bob",
                        "scope":"direct_chat",
                        "directChatId":"dc_001",
                        "effectiveAt":"2026-04-10T12:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user block should return response");
    assert_eq!(block_response.status(), StatusCode::OK);

    let block_body = block_response
        .into_body()
        .collect()
        .await
        .expect("user block body should collect")
        .to_bytes();
    let block_json: serde_json::Value =
        serde_json::from_slice(&block_body).expect("user block body should be valid json");

    assert_eq!(block_json["status"], "blocked");
    assert_eq!(block_json["userBlock"]["tenantId"], "t_demo");
    assert_eq!(block_json["userBlock"]["blockId"], "ub_001");
    assert_eq!(block_json["userBlock"]["blockerUserId"], "u_alice");
    assert_eq!(block_json["userBlock"]["blockedUserId"], "u_bob");
    assert_eq!(block_json["userBlock"]["scope"], "direct_chat");
    assert_eq!(block_json["userBlock"]["directChatId"], "dc_001");
    assert_eq!(block_json["userBlock"]["status"], "active");
    assert_eq!(
        block_json["latestCommit"]["eventType"],
        "user_block.blocked"
    );
    assert_eq!(block_json["latestCommit"]["scopeType"], "user_block");
    assert_eq!(
        block_json["latestCommit"]["payloadSchema"],
        "social.user_block.blocked.v1"
    );
    assert_eq!(block_json["latestCommit"]["orderingSeq"], 1);
    assert!(
        block_json["latestCommit"]["payload"]
            .as_str()
            .expect("payload should be string")
            .contains("\"directChatId\":\"dc_001\"")
    );

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/user_blocks/ub_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("user block snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("user block snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("user block snapshot should be valid json");

    assert_eq!(snapshot_json["status"], "snapshot");
    assert_eq!(snapshot_json["userBlock"]["blockId"], "ub_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "user_block.blocked"
    );

    let audit_auth = AppContext {
        tenant_id: "t_demo".into(),
        organization_id: None,
        user_id: "u_admin".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        app_id: Some("craw-chat".into()),
        environment: None,
        deployment_mode: None,
        device_id: None,
        permission_scope: BTreeSet::new(),
        data_scope: BTreeSet::new(),
        auth_level: None,
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 2);
    assert!(
        audit_export
            .items
            .iter()
            .any(|item| item.action == "control.user_block_blocked"
                && item
                    .payload
                    .as_deref()
                    .is_some_and(|payload| payload.contains("\"blockId\":\"ub_001\""))),
        "user block audit record should be persisted"
    );
}

#[tokio::test]
async fn test_control_plane_social_user_block_rejects_duplicate_active_scope() {
    let app = control_plane_api::build_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_001",
                        "eventId":"evt_ub_001",
                        "blockerUserId":"u_alice",
                        "blockedUserId":"u_bob",
                        "scope":"all",
                        "effectiveAt":"2026-04-10T12:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first user block should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    let duplicate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_002",
                        "eventId":"evt_ub_002",
                        "blockerUserId":"u_alice",
                        "blockedUserId":"u_bob",
                        "scope":"all",
                        "effectiveAt":"2026-04-10T12:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate user block should return response");

    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate user block body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value = serde_json::from_slice(&duplicate_body)
        .expect("duplicate user block body should be valid json");
    assert_eq!(duplicate_json["status"], 409);
    assert_eq!(duplicate_json["errorStatus"], "conflict");
    assert_eq!(duplicate_json["code"], "user_block_scope_conflict");
}

#[tokio::test]
async fn test_control_plane_social_user_block_rejects_direct_chat_scope_for_unknown_chat() {
    let app = control_plane_api::build_app();

    let block_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_unknown_direct_chat_001",
                        "eventId":"evt_ub_unknown_direct_chat_001",
                        "blockerUserId":"u_alice",
                        "blockedUserId":"u_bob",
                        "scope":"direct_chat",
                        "directChatId":"dc_missing_001",
                        "effectiveAt":"2026-04-10T12:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct-chat scoped user block for unknown chat should return response");

    assert_eq!(block_response.status(), StatusCode::BAD_REQUEST);
    let block_body = block_response
        .into_body()
        .collect()
        .await
        .expect("unknown direct-chat scoped block body should collect")
        .to_bytes();
    let block_json: serde_json::Value = serde_json::from_slice(&block_body)
        .expect("unknown direct-chat scoped block body should be valid json");
    assert_eq!(block_json["status"], 400);
    assert_eq!(block_json["errorStatus"], "invalid");
    assert_eq!(block_json["code"], "invalid_user_block");
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_restores_friend_request_snapshot_and_outbox() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime,
        runtime_dir.as_path(),
    );

    let submit_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_persist_001",
                        "eventId":"evt_persist_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T13:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("durable friend request should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    assert!(
        state_file(runtime_dir.as_path(), "social-state.json").exists(),
        "durable social state file should be materialized"
    );
    assert!(
        state_file(runtime_dir.as_path(), "social-commit-journal.json").exists(),
        "durable social commit journal should be materialized"
    );

    let journal_path = state_file(runtime_dir.as_path(), "social-commit-journal.json");
    let journal_items = read_json_lines(journal_path.as_path());
    assert_eq!(journal_items.len(), 1);
    assert_eq!(journal_items[0]["event_type"], "friend_request.submitted");
    assert_eq!(journal_items[0]["scope_type"], "friend_request");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let snapshot_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_persist_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after rebuild should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);
    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot body after rebuild should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot body after rebuild should be json");
    assert_eq!(
        snapshot_json["friendRequest"]["requestId"],
        "fr_persist_001"
    );
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_restores_direct_chat_pair_uniqueness_after_rebuild()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let first_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_persist_001",
                        "eventId":"evt_dc_persist_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_persist_001",
                        "boundAt":"2026-04-10T13:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("initial durable direct chat bind should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let duplicate_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_persist_002",
                        "eventId":"evt_dc_persist_002",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_persist_002",
                        "boundAt":"2026-04-10T13:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate durable direct chat bind should return response");
    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate response body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value =
        serde_json::from_slice(&duplicate_body).expect("duplicate response should be valid json");
    assert_eq!(duplicate_json["code"], "direct_chat_pair_conflict");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_replays_friend_request_when_snapshot_is_missing() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let submit_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_replay_001",
                        "eventId":"evt_replay_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T14:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request write should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    fs::remove_file(state_file(runtime_dir.as_path(), "social-state.json"))
        .expect("social state snapshot should be removed");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let snapshot_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_replay_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after journal replay should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request replay snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request replay snapshot should be json");
    assert_eq!(snapshot_json["friendRequest"]["requestId"], "fr_replay_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_fails_closed_when_journal_replay_fails() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let submit_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_corrupt_journal_001",
                        "eventId":"evt_corrupt_journal_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T14:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request write before journal corruption should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    fs::write(
        state_file(runtime_dir.as_path(), "social-commit-journal.json"),
        "{invalid-journal",
    )
    .expect("corrupted social journal should be writable");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let snapshot_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_corrupt_journal_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect(
            "friend request snapshot after journal corruption should return unavailable response",
        );
    assert_eq!(snapshot_response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request unavailable body after journal corruption should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request unavailable body after journal corruption should be json");
    assert_eq!(snapshot_json["code"], "social_commit_journal_unavailable");

    let write_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_corrupt_journal_002",
                        "eventId":"evt_corrupt_journal_002",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_charlie",
                        "requestedAt":"2026-04-10T14:06:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request write after journal corruption should return unavailable response");
    assert_eq!(write_response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let write_body = write_response
        .into_body()
        .collect()
        .await
        .expect("friend request write unavailable body after journal corruption should collect")
        .to_bytes();
    let write_json: serde_json::Value = serde_json::from_slice(&write_body)
        .expect("friend request write unavailable body after journal corruption should be json");
    assert_eq!(write_json["code"], "social_commit_journal_unavailable");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_fails_closed_when_snapshot_is_invalid_and_journal_missing()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(runtime_dir.join("state")).expect("runtime state dir should be created");
    fs::write(
        state_file(runtime_dir.as_path(), "social-state.json"),
        "{invalid-snapshot",
    )
    .expect("invalid social snapshot fixture should be writable");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_missing_invalid_snapshot")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("snapshot read on invalid startup snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot unavailable body on invalid startup snapshot should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("snapshot unavailable body on invalid startup snapshot should be json");
    assert_eq!(snapshot_json["code"], "social_state_unavailable");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_replaces_existing_snapshot_atomically() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    for (request_id, event_id, target_user_id) in [
        ("fr_atomic_snapshot_001", "evt_atomic_snapshot_001", "u_bob"),
        (
            "fr_atomic_snapshot_002",
            "evt_atomic_snapshot_002",
            "u_charlie",
        ),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/backend/v3/api/control/social/friend_requests")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "admin")
                    .header("x-sdkwork-permission-scope", "control.write")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "requestId": request_id,
                            "eventId": event_id,
                            "requesterUserId": "u_alice",
                            "targetUserId": target_user_id,
                            "requestedAt": "2026-04-10T14:07:00Z"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("friend request write should return response");
        assert_eq!(response.status(), StatusCode::OK);
    }

    let snapshot_path = state_file(runtime_dir.as_path(), "social-state.json");
    let snapshot_body =
        fs::read_to_string(snapshot_path.as_path()).expect("social snapshot should be readable");
    let snapshot_json: serde_json::Value =
        serde_json::from_str(&snapshot_body).expect("social snapshot should be valid json");
    assert!(
        snapshot_json["friend_requests"]["fr_atomic_snapshot_001"].is_object(),
        "first request should remain after atomic replacement"
    );
    assert!(
        snapshot_json["friend_requests"]["fr_atomic_snapshot_002"].is_object(),
        "second request should be present after atomic replacement"
    );

    let temp_files = fs::read_dir(snapshot_path.parent().expect("snapshot should have parent"))
        .expect("social state dir should be readable")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_name().to_str().is_some_and(|name| {
                name.starts_with(".social-state.json.") && name.ends_with(".tmp")
            })
        })
        .collect::<Vec<_>>();
    assert!(
        temp_files.is_empty(),
        "successful atomic snapshot replacement must not leave temp files"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_replays_direct_chat_pair_guard_when_snapshot_is_missing()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let first_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_replay_001",
                        "eventId":"evt_dc_replay_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_replay_001",
                        "boundAt":"2026-04-10T14:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("initial direct chat bind should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    fs::remove_file(state_file(runtime_dir.as_path(), "social-state.json"))
        .expect("social state snapshot should be removed");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let duplicate_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_replay_002",
                        "eventId":"evt_dc_replay_002",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_replay_002",
                        "boundAt":"2026-04-10T14:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate direct chat bind after journal replay should return response");
    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate direct chat replay response should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value = serde_json::from_slice(&duplicate_body)
        .expect("duplicate direct chat replay response should be json");
    assert_eq!(duplicate_json["code"], "direct_chat_pair_conflict");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_discards_friend_request_snapshot_ahead_of_journal()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let original_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_stable_001",
                        "eventId":"evt_stable_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T14:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("original friend request write should return response");
    assert_eq!(original_response.status(), StatusCode::OK);

    let original_journal = fs::read_to_string(state_file(
        runtime_dir.as_path(),
        "social-commit-journal.json",
    ))
    .expect("original journal should be readable");

    let phantom_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_phantom_001",
                        "eventId":"evt_phantom_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_cindy",
                        "requestedAt":"2026-04-10T14:21:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("phantom friend request write should return response");
    assert_eq!(phantom_response.status(), StatusCode::OK);

    fs::write(
        state_file(runtime_dir.as_path(), "social-commit-journal.json"),
        original_journal,
    )
    .expect("journal should be rolled back to original version");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let phantom_snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/friend_requests/fr_phantom_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("phantom snapshot lookup should return response");
    assert_eq!(phantom_snapshot_response.status(), StatusCode::NOT_FOUND);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_discards_direct_chat_snapshot_ahead_of_journal() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let original_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_stable_001",
                        "eventId":"evt_dc_stable_001",
                        "leftActorId":"actor_carla",
                        "rightActorId":"actor_david",
                        "conversationId":"c_stable_001",
                        "boundAt":"2026-04-10T14:30:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("original direct chat bind should return response");
    assert_eq!(original_response.status(), StatusCode::OK);

    let original_journal = fs::read_to_string(state_file(
        runtime_dir.as_path(),
        "social-commit-journal.json",
    ))
    .expect("original direct chat journal should be readable");

    let phantom_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_phantom_001",
                        "eventId":"evt_dc_phantom_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_phantom_001",
                        "boundAt":"2026-04-10T14:31:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("phantom direct chat bind should return response");
    assert_eq!(phantom_response.status(), StatusCode::OK);

    fs::write(
        state_file(runtime_dir.as_path(), "social-commit-journal.json"),
        original_journal,
    )
    .expect("direct chat journal should be rolled back to original version");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let replacement_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_replacement_001",
                        "eventId":"evt_dc_replacement_001",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_replacement_001",
                        "boundAt":"2026-04-10T14:32:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("replacement direct chat bind should return response");
    assert_eq!(replacement_response.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_keeps_direct_chat_pair_guard_after_snapshot_save_failure()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let seed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_seed_001",
                        "eventId":"evt_seed_001",
                        "requesterUserId":"u_seed_alice",
                        "targetUserId":"u_seed_bob",
                        "requestedAt":"2026-04-10T14:40:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed friend request should return response");
    assert_eq!(seed_response.status(), StatusCode::OK);

    let snapshot_path = state_file(runtime_dir.as_path(), "social-state.json");
    let mut readonly_permissions = fs::metadata(&snapshot_path)
        .expect("snapshot should exist")
        .permissions();
    readonly_permissions.set_readonly(true);
    fs::set_permissions(&snapshot_path, readonly_permissions)
        .expect("snapshot should become readonly");

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_fail_save_001",
                        "eventId":"evt_dc_fail_save_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_fail_save_001",
                        "boundAt":"2026-04-10T14:41:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with readonly snapshot should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["directChat"]["directChatId"],
        "dc_fail_save_001"
    );
    assert_eq!(
        committed_json["latestCommit"]["eventId"],
        "evt_dc_fail_save_001"
    );
    assert_eq!(committed_json["persistence"]["journalAuthority"], true);
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    make_file_writable(&snapshot_path);

    let duplicate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_fail_save_002",
                        "eventId":"evt_dc_fail_save_002",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_fail_save_002",
                        "boundAt":"2026-04-10T14:42:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate direct chat bind after snapshot save failure should return response");
    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);

    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate response body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value =
        serde_json::from_slice(&duplicate_body).expect("duplicate response should be valid json");
    assert_eq!(duplicate_json["code"], "direct_chat_pair_conflict");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_replays_same_event_id_after_snapshot_save_failure()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let seed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_seed_retry_001",
                        "eventId":"evt_seed_retry_001",
                        "requesterUserId":"u_seed_alice",
                        "targetUserId":"u_seed_bob",
                        "requestedAt":"2026-04-10T14:50:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed friend request should return response");
    assert_eq!(seed_response.status(), StatusCode::OK);

    let snapshot_path = state_file(runtime_dir.as_path(), "social-state.json");
    let mut readonly_permissions = fs::metadata(&snapshot_path)
        .expect("snapshot should exist")
        .permissions();
    readonly_permissions.set_readonly(true);
    fs::set_permissions(&snapshot_path, readonly_permissions)
        .expect("snapshot should become readonly");

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_retry_same_event_001",
                        "eventId":"evt_dc_retry_same_event_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_retry_same_event_001",
                        "boundAt":"2026-04-10T14:51:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with readonly snapshot should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    make_file_writable(&snapshot_path);

    let retry_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_retry_same_event_001",
                        "eventId":"evt_dc_retry_same_event_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_retry_same_event_001",
                        "boundAt":"2026-04-10T14:51:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("same event retry should return response");
    assert_eq!(retry_response.status(), StatusCode::OK);

    let retry_body = retry_response
        .into_body()
        .collect()
        .await
        .expect("retry response body should collect")
        .to_bytes();
    let retry_json: serde_json::Value =
        serde_json::from_slice(&retry_body).expect("retry response should be valid json");
    assert_eq!(retry_json["status"], "bound");
    assert_eq!(
        retry_json["directChat"]["directChatId"],
        "dc_retry_same_event_001"
    );
    assert_eq!(
        retry_json["latestCommit"]["eventId"],
        "evt_dc_retry_same_event_001"
    );
    assert_eq!(retry_json["persistence"]["journalAuthority"], true);
    assert_eq!(retry_json["persistence"]["snapshotStatus"], "current");

    let journal_path = state_file(runtime_dir.as_path(), "social-commit-journal.json");
    let journal_items = read_json_lines(journal_path.as_path());
    assert_eq!(journal_items.len(), 2);
    assert_eq!(
        journal_items
            .iter()
            .filter(|item| item["event_id"] == "evt_dc_retry_same_event_001")
            .count(),
        1
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_failpoint_forces_next_snapshot_save_failure_once() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(state_file(runtime_dir.as_path(), "").as_path())
        .expect("state dir should be created");
    fs::write(
        social_failpoint_file(runtime_dir.as_path()),
        r#"{"failNextSnapshotSave":true}"#,
    )
    .expect("social failpoint file should be written");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_failpoint_once_001",
                        "eventId":"evt_failpoint_once_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_failpoint_once_001",
                        "boundAt":"2026-04-10T15:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with failpoint should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["directChat"]["directChatId"],
        "dc_failpoint_once_001"
    );
    assert_eq!(
        committed_json["latestCommit"]["eventId"],
        "evt_failpoint_once_001"
    );
    assert_eq!(committed_json["persistence"]["journalAuthority"], true);
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    let failpoint_file = social_failpoint_file(runtime_dir.as_path());
    if failpoint_file.exists() {
        let failpoint_json: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(&failpoint_file).expect("failpoint file should be readable"),
        )
        .expect("failpoint file should be valid json");
        assert_eq!(failpoint_json["failNextSnapshotSave"], false);
    }

    let retry_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_failpoint_once_001",
                        "eventId":"evt_failpoint_once_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_failpoint_once_001",
                        "boundAt":"2026-04-10T15:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("same-event retry after failpoint should return response");
    assert_eq!(retry_response.status(), StatusCode::OK);

    let retry_body = retry_response
        .into_body()
        .collect()
        .await
        .expect("retry response body should collect")
        .to_bytes();
    let retry_json: serde_json::Value =
        serde_json::from_slice(&retry_body).expect("retry response should be valid json");
    assert_eq!(retry_json["status"], "bound");
    assert_eq!(
        retry_json["directChat"]["directChatId"],
        "dc_failpoint_once_001"
    );
    assert_eq!(retry_json["persistence"]["journalAuthority"], true);
    assert_eq!(retry_json["persistence"]["snapshotStatus"], "current");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_operator_repair_rebuilds_snapshot_after_failpoint()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(state_file(runtime_dir.as_path(), "").as_path())
        .expect("state dir should be created");
    fs::write(
        social_failpoint_file(runtime_dir.as_path()),
        r#"{"failNextSnapshotSave":true}"#,
    )
    .expect("social failpoint file should be written");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_operator_repair_001",
                        "eventId":"evt_operator_repair_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_operator_repair_001",
                        "boundAt":"2026-04-10T15:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with failpoint should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    let repair_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/runtime/repair_derived_snapshot")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("social operator repair should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);

    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("repair response body should collect")
        .to_bytes();
    let repair_json: serde_json::Value =
        serde_json::from_slice(&repair_body).expect("repair response should be valid json");
    assert_eq!(repair_json["status"], "repaired");
    assert_eq!(repair_json["journalAuthority"], true);
    assert_eq!(repair_json["snapshotUpdated"], true);
    assert_eq!(repair_json["transactionMarkerCleared"], true);
    assert_eq!(repair_json["aggregateCounts"]["directChats"], 1);

    let snapshot_body = fs::read_to_string(state_file(runtime_dir.as_path(), "social-state.json"))
        .expect("social state snapshot should be readable after repair");
    let snapshot_json: serde_json::Value =
        serde_json::from_str(&snapshot_body).expect("social state snapshot should be valid json");
    assert_eq!(
        snapshot_json["direct_chats"]["dc_operator_repair_001"]["direct_chat"]["conversationId"],
        "c_operator_repair_001"
    );

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );
    let snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/direct_chats/dc_operator_repair_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("snapshot after repair should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_leaves_pending_tx_marker_after_snapshot_failure_and_clears_it_after_restart_replay()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(state_file(runtime_dir.as_path(), "").as_path())
        .expect("state dir should be created");
    fs::write(
        social_failpoint_file(runtime_dir.as_path()),
        r#"{"failNextSnapshotSave":true}"#,
    )
    .expect("social failpoint file should be written");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_tx_marker_001",
                        "eventId":"evt_tx_marker_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_tx_marker_001",
                        "boundAt":"2026-04-11T09:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with failpoint should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    let marker_path = social_tx_marker_file(runtime_dir.as_path());
    assert!(
        marker_path.exists(),
        "pending social tx marker should exist after snapshot save failure"
    );
    let marker_body =
        fs::read_to_string(&marker_path).expect("pending social tx marker should be readable");
    let marker_json: serde_json::Value =
        serde_json::from_str(&marker_body).expect("pending social tx marker should be valid json");
    assert_eq!(marker_json["status"], "pending_snapshot_repair");
    assert_eq!(marker_json["eventId"], "evt_tx_marker_001");

    let cluster_after = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime_after = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster_after,
        ops_runtime_after,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    assert!(
        !marker_path.exists(),
        "pending social tx marker should be cleared after restart replay repairs snapshot"
    );

    let snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/direct_chats/dc_tx_marker_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("snapshot after restart replay should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_operator_repair_replays_external_journal_append_into_live_state()
 {
    let runtime_dir = unique_runtime_dir();
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let journal_path = state_file(runtime_dir.as_path(), "social-commit-journal.json");
    let journal = FileCommitJournal::new("social", journal_path);
    let payload = DirectChatBoundPayload {
        direct_chat_id: "dc_operator_repair_journal_001".into(),
        conversation_id: "c_operator_repair_journal_001".into(),
        left_actor_id: "actor_alice".into(),
        right_actor_id: "actor_carol".into(),
        pair_hash: direct_chat_pair_hash("actor_alice", "actor_carol")
            .expect("direct chat pair hash should normalize"),
        bound_at: "2026-04-11T01:00:00Z".into(),
    };
    let payload_json =
        serde_json::to_string(&payload).expect("external journal payload should serialize");
    journal
        .append(social_commit_envelope(SocialCommitEnvelopeInput {
            event_id: "evt_operator_repair_journal_001",
            tenant_id: "t_demo",
            aggregate_type: AggregateType::DirectChat,
            aggregate_id: payload.direct_chat_id.as_str(),
            event_type: SocialEventType::DirectChatBound,
            ordering_seq: 1,
            actor: EventActor {
                actor_id: "operator_repair".into(),
                actor_kind: "operator".into(),
                actor_session_id: None,
            },
            occurred_at: payload.bound_at.as_str(),
            committed_at: payload.bound_at.as_str(),
            payload: payload_json.as_str(),
        }))
        .expect("external journal append should succeed");

    let repair_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/runtime/repair_derived_snapshot")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("social operator repair should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);

    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("repair response body should collect")
        .to_bytes();
    let repair_json: serde_json::Value =
        serde_json::from_slice(&repair_body).expect("repair response should be valid json");
    assert_eq!(repair_json["status"], "repaired");
    assert_eq!(repair_json["journalAuthority"], true);
    assert_eq!(repair_json["snapshotUpdated"], true);
    assert_eq!(repair_json["transactionMarkerCleared"], false);
    assert_eq!(repair_json["aggregateCounts"]["directChats"], 1);

    let read_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/direct_chats/dc_operator_repair_journal_001")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "admin")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("direct chat read after repair should return response");
    assert_eq!(read_response.status(), StatusCode::OK);

    let snapshot_body = fs::read_to_string(state_file(runtime_dir.as_path(), "social-state.json"))
        .expect("social state snapshot should be readable after repair");
    let snapshot_json: serde_json::Value =
        serde_json::from_str(&snapshot_body).expect("social state snapshot should be valid json");
    assert_eq!(
        snapshot_json["direct_chats"]["dc_operator_repair_journal_001"]["direct_chat"]["conversationId"],
        "c_operator_repair_journal_001"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
