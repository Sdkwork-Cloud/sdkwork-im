use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use audit_service::AuditRuntime;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use bytes::Bytes;
use control_plane_api::{
    SharedChannelLinkedMemberSyncRequest, SharedChannelLinkedMemberSyncTrigger,
};
use http_body_util::{BodyExt, Full};
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::{Method, Request as HyperRequest};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use im_auth_context::{AuthContext, PUBLIC_BEARER_HS256_SECRET_ENV, encode_hs256_bearer_token};
use ops_service::OpsRuntime;
use session_gateway::RealtimeClusterBridge;
use tokio::net::TcpListener;
use tokio::sync::{Mutex as AsyncMutex, MutexGuard};
use tower::ServiceExt;

const TEST_PUBLIC_SECRET: &str = "public-test-secret";
static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

struct TestRuntimeDir {
    path: PathBuf,
}

impl TestRuntimeDir {
    fn new(prefix: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}_{sequence}"));
        fs::create_dir_all(path.join("state")).expect("test runtime dir state should be created");
        Self { path }
    }

    fn path(&self) -> &Path {
        self.path.as_path()
    }
}

impl Drop for TestRuntimeDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn state_file(runtime_dir: &Path, file_name: &str) -> PathBuf {
    runtime_dir.join("state").join(file_name)
}

fn shared_channel_sync_request_key(request: &SharedChannelLinkedMemberSyncRequest) -> String {
    format!(
        "{}|{}|{}|{}|{}|{}|{}",
        request.tenant_id,
        request.conversation_id,
        request.shared_channel_policy_id,
        request.external_connection_id,
        request.local_actor_id,
        request.local_actor_kind,
        request.external_member_id
    )
}

fn read_social_state_json(runtime_dir: &Path) -> serde_json::Value {
    let body = fs::read_to_string(state_file(runtime_dir, "social-state.json"))
        .expect("social state file should be readable");
    serde_json::from_str(&body).expect("social state file should be valid json")
}

fn write_social_state_json(runtime_dir: &Path, state: &serde_json::Value) {
    let body = serde_json::to_string_pretty(state)
        .expect("social state json should serialize back to disk");
    fs::write(state_file(runtime_dir, "social-state.json"), body)
        .expect("social state file should be writable");
}

async fn wait_for_pending_shared_channel_sync_reclaim(
    runtime_dir: &Path,
    request_key: &str,
    timeout: Duration,
) -> serde_json::Value {
    let deadline = Instant::now() + timeout;
    loop {
        let state = read_social_state_json(runtime_dir);
        let Some(pending) = state["pending_shared_channel_sync_requests"]
            .as_object()
            .and_then(|items| items.get(request_key))
        else {
            panic!("pending shared-channel sync request {request_key} should exist");
        };
        if pending["ownerActorId"].is_null()
            && pending["ownerActorKind"].is_null()
            && pending["claimedAt"].is_null()
            && pending["leaseExpiresAt"].is_null()
        {
            return state;
        }
        if Instant::now() >= deadline {
            panic!(
                "pending shared-channel sync request {request_key} was not reclaimed before timeout"
            );
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

async fn public_auth_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<AsyncMutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| AsyncMutex::new(())).lock().await
}

async fn configure_public_bearer_secret() -> MutexGuard<'static, ()> {
    let guard = public_auth_guard().await;
    unsafe {
        std::env::set_var(PUBLIC_BEARER_HS256_SECRET_ENV, TEST_PUBLIC_SECRET);
    }
    guard
}

fn bearer_token(tenant_id: &str, actor_id: &str, actor_kind: &str, permissions: &[&str]) -> String {
    let mut claims = serde_json::json!({
        "tenant_id": tenant_id,
        "sub": actor_id,
        "actor_kind": actor_kind,
    });
    if !permissions.is_empty() {
        claims["permissions"] = serde_json::json!(permissions);
    }

    format!(
        "Bearer {}",
        encode_hs256_bearer_token(&claims, TEST_PUBLIC_SECRET)
            .expect("signed bearer token should encode")
    )
}

fn http_client() -> Client<HttpConnector, Full<Bytes>> {
    let connector = HttpConnector::new();
    Client::builder(TokioExecutor::new()).build(connector)
}

async fn http_json_request(
    base_url: &str,
    method: Method,
    path: &str,
    authorization: &str,
    body: Option<serde_json::Value>,
) -> (StatusCode, serde_json::Value) {
    let payload = body
        .map(|value| serde_json::to_vec(&value).expect("json request body should encode"))
        .map(Bytes::from)
        .unwrap_or_default();
    let request = HyperRequest::builder()
        .method(method)
        .uri(format!("{}{}", base_url.trim_end_matches('/'), path))
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, authorization)
        .body(Full::new(payload))
        .expect("http request should build");

    let response = http_client()
        .request(request)
        .await
        .expect("http request should succeed");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("http response body should collect")
        .to_bytes();
    let json = if bytes.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_slice(&bytes).expect("http response body should be valid json")
    };
    (status, json)
}

async fn spawn_public_runtime_server() -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("public runtime listener should bind");
    let addr = listener
        .local_addr()
        .expect("public runtime listener should expose local addr");
    let app = conversation_runtime::build_public_app();
    let handle = tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("public runtime server should run");
    });
    (format!("http://{addr}"), handle)
}

#[derive(Default)]
struct RecordingSharedChannelSyncTrigger {
    requests: Mutex<Vec<SharedChannelLinkedMemberSyncRequest>>,
}

impl RecordingSharedChannelSyncTrigger {
    fn snapshot(&self) -> Vec<SharedChannelLinkedMemberSyncRequest> {
        self.requests
            .lock()
            .expect("recording trigger lock should not be poisoned")
            .clone()
    }
}

impl SharedChannelLinkedMemberSyncTrigger for RecordingSharedChannelSyncTrigger {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String> {
        self.requests
            .lock()
            .expect("recording trigger lock should not be poisoned")
            .push(request);
        Ok(())
    }
}

enum SwitchableSharedChannelSyncTriggerMode {
    Fail(String),
    Delegate(Arc<dyn SharedChannelLinkedMemberSyncTrigger>),
}

struct SwitchableSharedChannelSyncTrigger {
    mode: Mutex<SwitchableSharedChannelSyncTriggerMode>,
}

impl SwitchableSharedChannelSyncTrigger {
    fn failing(message: impl Into<String>) -> Self {
        Self {
            mode: Mutex::new(SwitchableSharedChannelSyncTriggerMode::Fail(message.into())),
        }
    }

    fn set_delegate(&self, trigger: Arc<dyn SharedChannelLinkedMemberSyncTrigger>) {
        *self
            .mode
            .lock()
            .expect("switchable trigger lock should not be poisoned") =
            SwitchableSharedChannelSyncTriggerMode::Delegate(trigger);
    }
}

impl SharedChannelLinkedMemberSyncTrigger for SwitchableSharedChannelSyncTrigger {
    fn trigger(&self, request: SharedChannelLinkedMemberSyncRequest) -> Result<(), String> {
        match &*self
            .mode
            .lock()
            .expect("switchable trigger lock should not be poisoned")
        {
            SwitchableSharedChannelSyncTriggerMode::Fail(message) => Err(message.clone()),
            SwitchableSharedChannelSyncTriggerMode::Delegate(trigger) => trigger.trigger(request),
        }
    }
}

#[tokio::test]
async fn test_control_plane_social_external_connection_write_persists_snapshot_commit_and_audit() {
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

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_001",
                        "eventId":"evt_ec_001",
                        "externalTenantId":"t_partner",
                        "externalOrgName":"Partner Org",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-10T13:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let establish_body = establish_response
        .into_body()
        .collect()
        .await
        .expect("establish body should collect")
        .to_bytes();
    let establish_json: serde_json::Value =
        serde_json::from_slice(&establish_body).expect("establish body should be valid json");

    assert_eq!(establish_json["status"], "established");
    assert_eq!(establish_json["externalConnection"]["tenantId"], "t_demo");
    assert_eq!(
        establish_json["externalConnection"]["connectionId"],
        "ec_001"
    );
    assert_eq!(
        establish_json["externalConnection"]["externalTenantId"],
        "t_partner"
    );
    assert_eq!(
        establish_json["externalConnection"]["externalOrgName"],
        "Partner Org"
    );
    assert_eq!(
        establish_json["externalConnection"]["connectionKind"],
        "shared_channel"
    );
    assert_eq!(establish_json["externalConnection"]["status"], "active");
    assert_eq!(
        establish_json["latestCommit"]["eventType"],
        "external_connection.established"
    );
    assert_eq!(
        establish_json["latestCommit"]["scopeType"],
        "external_connection"
    );
    assert_eq!(
        establish_json["latestCommit"]["payloadSchema"],
        "social.external_connection.established.v1"
    );
    assert_eq!(establish_json["latestCommit"]["orderingSeq"], 1);

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/external-connections/ec_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("external connection snapshot should return response");
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
    assert_eq!(
        snapshot_json["externalConnection"]["connectionId"],
        "ec_001"
    );
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "external_connection.established"
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
        "control.external_connection_established"
    );
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("external connection audit should include payload")
            .contains("\"connectionId\":\"ec_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_external_connection_rejects_same_tenant_target() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_invalid",
                        "eventId":"evt_ec_invalid",
                        "externalTenantId":"t_demo",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-10T13:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invalid external connection should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("invalid body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("invalid body should be valid json");
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["code"], "invalid_external_connection");
}

#[tokio::test]
async fn test_control_plane_social_external_member_link_write_persists_snapshot_commit_and_audit() {
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

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_002",
                        "eventId":"evt_ec_002",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-10T13:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");

    let bind_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_001",
                        "eventId":"evt_eml_001",
                        "connectionId":"ec_002",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "externalDisplayName":"Bob Partner",
                        "linkedAt":"2026-04-10T13:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
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
    assert_eq!(bind_json["externalMemberLink"]["tenantId"], "t_demo");
    assert_eq!(bind_json["externalMemberLink"]["linkId"], "eml_001");
    assert_eq!(bind_json["externalMemberLink"]["connectionId"], "ec_002");
    assert_eq!(
        bind_json["externalMemberLink"]["localActorId"],
        "actor_alice"
    );
    assert_eq!(bind_json["externalMemberLink"]["localActorKind"], "user");
    assert_eq!(
        bind_json["externalMemberLink"]["externalMemberId"],
        "partner::bob"
    );
    assert_eq!(
        bind_json["externalMemberLink"]["externalDisplayName"],
        "Bob Partner"
    );
    assert_eq!(bind_json["externalMemberLink"]["status"], "active");
    assert_eq!(
        bind_json["latestCommit"]["eventType"],
        "external_member_link.bound"
    );
    assert_eq!(
        bind_json["latestCommit"]["scopeType"],
        "external_member_link"
    );
    assert_eq!(
        bind_json["latestCommit"]["payloadSchema"],
        "social.external_member_link.bound.v1"
    );
    assert_eq!(bind_json["latestCommit"]["orderingSeq"], 1);

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/external-member-links/eml_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("external member link snapshot should return response");
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
    assert_eq!(snapshot_json["externalMemberLink"]["linkId"], "eml_001");
    assert_eq!(
        snapshot_json["externalMemberLink"]["localActorKind"],
        "user"
    );
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "external_member_link.bound"
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
        audit_export.items[1].action,
        "control.external_member_link_bound"
    );
    assert!(
        audit_export.items[1]
            .payload
            .as_deref()
            .expect("external member link audit should include payload")
            .contains("\"linkId\":\"eml_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_external_member_link_requires_active_external_connection() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_missing",
                        "eventId":"evt_eml_missing",
                        "connectionId":"ec_missing",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-10T13:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("missing connection should return response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("missing connection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("missing connection body should be valid json");
    assert_eq!(json["status"], "not_found");
    assert_eq!(json["code"], "external_connection_not_found");
}

#[tokio::test]
async fn test_control_plane_social_shared_channel_policy_write_persists_snapshot_commit_and_audit()
{
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

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_003",
                        "eventId":"evt_ec_003",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-10T13:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");

    let apply_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_001",
                        "eventId":"evt_scp_001",
                        "connectionId":"ec_003",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-10T13:21:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(apply_response.status(), StatusCode::OK);

    let apply_body = apply_response
        .into_body()
        .collect()
        .await
        .expect("apply body should collect")
        .to_bytes();
    let apply_json: serde_json::Value =
        serde_json::from_slice(&apply_body).expect("apply body should be valid json");

    assert_eq!(apply_json["status"], "applied");
    assert_eq!(apply_json["sharedChannelPolicy"]["tenantId"], "t_demo");
    assert_eq!(apply_json["sharedChannelPolicy"]["policyId"], "scp_001");
    assert_eq!(apply_json["sharedChannelPolicy"]["connectionId"], "ec_003");
    assert_eq!(
        apply_json["sharedChannelPolicy"]["channelId"],
        "ch_partner_ops"
    );
    assert_eq!(
        apply_json["sharedChannelPolicy"]["conversationId"],
        "c_partner_ops"
    );
    assert_eq!(apply_json["sharedChannelPolicy"]["policyVersion"], 1);
    assert_eq!(
        apply_json["sharedChannelPolicy"]["historyVisibility"],
        "shared"
    );
    assert_eq!(apply_json["sharedChannelPolicy"]["status"], "active");
    assert_eq!(
        apply_json["latestCommit"]["eventType"],
        "shared_channel_policy.applied"
    );
    assert_eq!(
        apply_json["latestCommit"]["scopeType"],
        "shared_channel_policy"
    );
    assert_eq!(
        apply_json["latestCommit"]["payloadSchema"],
        "social.shared_channel_policy.applied.v1"
    );
    assert_eq!(apply_json["latestCommit"]["orderingSeq"], 1);

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/shared-channel-policies/scp_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("shared channel policy snapshot should return response");
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
    assert_eq!(snapshot_json["sharedChannelPolicy"]["policyId"], "scp_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "shared_channel_policy.applied"
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
        audit_export.items[1].action,
        "control.shared_channel_policy_applied"
    );
    assert!(
        audit_export.items[1]
            .payload
            .as_deref()
            .expect("shared channel policy audit should include payload")
            .contains("\"policyId\":\"scp_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_shared_channel_policy_rejects_non_shared_history_visibility() {
    let app = control_plane_api::build_app();

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_004",
                        "eventId":"evt_ec_004",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-10T13:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_invalid",
                        "eventId":"evt_scp_invalid",
                        "connectionId":"ec_004",
                        "channelId":"ch_partner_ops",
                        "policyVersion":1,
                        "historyVisibility":"joined",
                        "appliedAt":"2026-04-10T13:21:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invalid shared channel policy should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("invalid shared channel policy body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body)
        .expect("invalid shared channel policy body should be valid json");
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["code"], "invalid_shared_channel_policy");
}

#[tokio::test]
async fn test_control_plane_social_external_member_link_auto_triggers_shared_channel_sync_when_policy_already_exists()
 {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(RecordingSharedChannelSyncTrigger::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        trigger.clone(),
    );

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_005",
                        "eventId":"evt_ec_005",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-10T13:30:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_005",
                        "eventId":"evt_scp_005",
                        "connectionId":"ec_005",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops",
                        "policyVersion":2,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-10T13:31:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");

    let bind_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_005",
                        "eventId":"evt_eml_005",
                        "connectionId":"ec_005",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-10T13:32:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(bind_response.status(), StatusCode::OK);

    let requests = trigger.snapshot();
    assert_eq!(
        requests,
        vec![SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".into(),
            conversation_id: "c_partner_ops".into(),
            shared_channel_policy_id: "scp_005".into(),
            external_connection_id: "ec_005".into(),
            local_actor_id: "actor_alice".into(),
            local_actor_kind: "user".into(),
            external_member_id: "partner::bob".into(),
        }]
    );
}

#[tokio::test]
async fn test_control_plane_social_shared_channel_policy_auto_triggers_shared_channel_sync_for_existing_links()
 {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(RecordingSharedChannelSyncTrigger::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        trigger.clone(),
    );

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_006",
                        "eventId":"evt_ec_006",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-10T13:40:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");

    for (link_id, local_actor_id, external_member_id) in [
        ("eml_006_a", "actor_alice", "partner::alice"),
        ("eml_006_b", "actor_bob", "partner::bob"),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/control/social/external-member-links")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", "u_admin")
                    .header("x-permissions", "control.write")
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "linkId":"{link_id}",
                            "eventId":"evt_{link_id}",
                            "connectionId":"ec_006",
                            "localActorId":"{local_actor_id}",
                            "localActorKind":"user",
                            "externalMemberId":"{external_member_id}",
                            "linkedAt":"2026-04-10T13:41:00Z"
                        }}"#,
                    )))
                    .unwrap(),
            )
            .await
            .expect("external member link write should return response");
        assert_eq!(response.status(), StatusCode::OK);
    }

    assert!(
        trigger.snapshot().is_empty(),
        "links alone should not trigger sync before policy exists"
    );

    let apply_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_006",
                        "eventId":"evt_scp_006",
                        "connectionId":"ec_006",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops",
                        "policyVersion":3,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-10T13:42:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(apply_response.status(), StatusCode::OK);

    let requests = trigger.snapshot();
    assert_eq!(
        requests,
        vec![
            SharedChannelLinkedMemberSyncRequest {
                tenant_id: "t_demo".into(),
                conversation_id: "c_partner_ops".into(),
                shared_channel_policy_id: "scp_006".into(),
                external_connection_id: "ec_006".into(),
                local_actor_id: "actor_alice".into(),
                local_actor_kind: "user".into(),
                external_member_id: "partner::alice".into(),
            },
            SharedChannelLinkedMemberSyncRequest {
                tenant_id: "t_demo".into(),
                conversation_id: "c_partner_ops".into(),
                shared_channel_policy_id: "scp_006".into(),
                external_connection_id: "ec_006".into(),
                local_actor_id: "actor_bob".into(),
                local_actor_kind: "user".into(),
                external_member_id: "partner::bob".into(),
            },
        ]
    );
}

#[tokio::test]
async fn test_control_plane_social_shared_channel_auto_trigger_preserves_audit_when_composite_ids_are_long_but_valid()
 {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(RecordingSharedChannelSyncTrigger::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
        trigger.clone(),
    );

    let policy_id = "p".repeat(120);
    let conversation_id = "c".repeat(120);
    let local_actor_id = "a".repeat(120);

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_long_audit",
                        "eventId":"evt_ec_long_audit",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-12T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "policyId":"{policy_id}",
                        "eventId":"evt_scp_long_audit",
                        "connectionId":"ec_long_audit",
                        "channelId":"ch_partner_ops",
                        "conversationId":"{conversation_id}",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-12T10:01:00Z"
                    }}"#,
                )))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "linkId":"eml_long_audit",
                        "eventId":"evt_eml_long_audit",
                        "connectionId":"ec_long_audit",
                        "localActorId":"{local_actor_id}",
                        "localActorKind":"user",
                        "externalMemberId":"partner::long-audit",
                        "linkedAt":"2026-04-12T10:02:00Z"
                    }}"#,
                )))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(link_response.status(), StatusCode::OK);

    assert_eq!(
        trigger.snapshot(),
        vec![SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".into(),
            conversation_id: conversation_id.clone(),
            shared_channel_policy_id: policy_id.clone(),
            external_connection_id: "ec_long_audit".into(),
            local_actor_id: local_actor_id.clone(),
            local_actor_kind: "user".into(),
            external_member_id: "partner::long-audit".into(),
        }]
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
    assert_eq!(audit_export.total, 4);
    assert!(
        audit_export
            .items
            .iter()
            .any(|item| item.action == "control.shared_channel_linked_member_sync_triggered"),
        "shared-channel sync dispatch should remain auditable when component ids are long but valid"
    );
}

#[tokio::test]
async fn test_control_plane_social_external_member_link_idempotent_replay_does_not_duplicate_shared_channel_sync_dispatch()
 {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(RecordingSharedChannelSyncTrigger::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        trigger.clone(),
    );

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_005_replay",
                        "eventId":"evt_ec_005_replay",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-10T13:30:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_005_replay",
                        "eventId":"evt_scp_005_replay",
                        "connectionId":"ec_005_replay",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops",
                        "policyVersion":2,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-10T13:31:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");

    let request_body = r#"{
        "linkId":"eml_005_replay",
        "eventId":"evt_eml_005_replay",
        "connectionId":"ec_005_replay",
        "localActorId":"actor_alice",
        "localActorKind":"user",
        "externalMemberId":"partner::bob",
        "linkedAt":"2026-04-10T13:32:00Z"
    }"#;

    let first_bind = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("first external member link write should return response");
    assert_eq!(first_bind.status(), StatusCode::OK);

    let second_bind = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("idempotent replay external member link write should return response");
    assert_eq!(second_bind.status(), StatusCode::OK);

    let requests = trigger.snapshot();
    assert_eq!(
        requests,
        vec![SharedChannelLinkedMemberSyncRequest {
            tenant_id: "t_demo".into(),
            conversation_id: "c_partner_ops".into(),
            shared_channel_policy_id: "scp_005_replay".into(),
            external_connection_id: "ec_005_replay".into(),
            local_actor_id: "actor_alice".into(),
            local_actor_kind: "user".into(),
            external_member_id: "partner::bob".into(),
        }],
        "idempotent replay of the same external member link event should not enqueue duplicate shared-channel sync dispatches"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_delivered_ledger_skips_replayed_pending_after_restart()
 {
    let runtime_dir = TestRuntimeDir::new("control_plane_shared_channel_delivered_ledger_restart");

    let cluster_seed = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime_seed = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime_seed = Arc::new(AuditRuntime::default());
    let trigger_seed = Arc::new(RecordingSharedChannelSyncTrigger::default());
    let seed_app =
        control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
            cluster_seed,
            ops_runtime_seed,
            audit_runtime_seed,
            runtime_dir.path(),
            trigger_seed.clone(),
        );

    let _ = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_ledger_restart",
                        "eventId":"evt_ec_ledger_restart",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-12T12:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");

    let _ = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_ledger_restart",
                        "eventId":"evt_scp_ledger_restart",
                        "connectionId":"ec_ledger_restart",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops",
                        "policyVersion":2,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-12T12:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");

    let first_bind = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_ledger_restart_alice",
                        "eventId":"evt_eml_ledger_restart_alice",
                        "connectionId":"ec_ledger_restart",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-12T12:12:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first external member link write should return response");
    assert_eq!(first_bind.status(), StatusCode::OK);

    let first_requests = trigger_seed.snapshot();
    assert_eq!(first_requests.len(), 1);
    let first_request = first_requests[0].clone();
    assert_eq!(first_request.local_actor_id, "actor_alice");

    let request_key = shared_channel_sync_request_key(&first_request);
    let mut seeded_state = read_social_state_json(runtime_dir.path());
    seeded_state["pending_shared_channel_sync_requests"][request_key.as_str()] = serde_json::json!({
        "request": first_request,
        "failureCount": 1,
        "lastError": "manually requeued delivered request for regression coverage",
        "ownerActorId": null,
        "ownerActorKind": null,
        "claimedAt": null,
        "leaseExpiresAt": null
    });
    write_social_state_json(runtime_dir.path(), &seeded_state);

    let cluster_restart = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime_restart = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime_restart = Arc::new(AuditRuntime::default());
    let trigger_restart = Arc::new(RecordingSharedChannelSyncTrigger::default());
    let restart_app =
        control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
            cluster_restart,
            ops_runtime_restart,
            audit_runtime_restart,
            runtime_dir.path(),
            trigger_restart.clone(),
        );

    let second_bind = restart_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_ledger_restart_bob",
                        "eventId":"evt_eml_ledger_restart_bob",
                        "connectionId":"ec_ledger_restart",
                        "localActorId":"actor_bob",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-12T12:13:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second external member link write should return response");
    assert_eq!(second_bind.status(), StatusCode::OK);

    let restarted_requests = trigger_restart.snapshot();
    assert_eq!(
        restarted_requests.len(),
        1,
        "restart dispatch should only emit the new actor request; delivered replay must be skipped"
    );
    assert_eq!(restarted_requests[0].local_actor_id, "actor_bob");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_http_trigger_materializes_remote_runtime_linked_member_over_public_runtime()
 {
    let _guard = configure_public_bearer_secret().await;
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_remote",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(create_json["conversationId"], "c_partner_ops_remote");

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_remote/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_remote_001",
            "summary":"hello public shared",
            "text":"hello public shared"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(post_json["messageId"], "msg_c_partner_ops_remote_1");

    let trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("http shared-channel sync trigger should build");
    let app = control_plane_api::build_public_app_with_shared_channel_sync_trigger(trigger);
    let control_bearer = bearer_token("t_demo", "u_admin", "admin", &["control.write"]);

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("authorization", control_bearer.as_str())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_remote_001",
                        "eventId":"evt_ec_remote_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T01:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("authorization", control_bearer.as_str())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_remote_001",
                        "eventId":"evt_scp_remote_001",
                        "connectionId":"ec_remote_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_remote",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T01:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("authorization", control_bearer.as_str())
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_remote_001",
                        "eventId":"evt_eml_remote_001",
                        "connectionId":"ec_remote_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T01:02:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(link_response.status(), StatusCode::OK);

    let linked_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (history_status, history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_remote/messages",
        linked_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(history_status, StatusCode::OK);
    assert_eq!(
        history_json["items"][0]["message"]["body"]["summary"],
        "hello public shared"
    );
    assert_eq!(
        history_json["items"][0]["message"]["messageId"],
        "msg_c_partner_ops_remote_1"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_next_write_reclaims_stale_claim_metadata_when_trigger_is_unconfigured()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_stale_next_write_reclaim_unconfigured");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_stale_next_write_reclaim_unconfigured",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_stale_next_write_reclaim_unconfigured"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_stale_next_write_reclaim_unconfigured/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_stale_next_write_reclaim_unconfigured_001",
            "summary":"hello stale next write reclaim unconfigured",
            "text":"hello stale next write reclaim unconfigured"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_stale_next_write_reclaim_unconfigured_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_stale_next_write_reclaim_unconfigured_001",
                        "eventId":"evt_ec_stale_next_write_reclaim_unconfigured_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-12T01:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_stale_next_write_reclaim_unconfigured_001",
                        "eventId":"evt_scp_stale_next_write_reclaim_unconfigured_001",
                        "connectionId":"ec_stale_next_write_reclaim_unconfigured_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_stale_next_write_reclaim_unconfigured",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-12T01:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let first_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_stale_next_write_reclaim_unconfigured_001",
                        "eventId":"evt_eml_stale_next_write_reclaim_unconfigured_001",
                        "connectionId":"ec_stale_next_write_reclaim_unconfigured_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-12T01:02:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first external member link write should return response");
    assert_eq!(first_link_response.status(), StatusCode::OK);

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let alice_request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("stale next write reclaim targeted claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("stale next write reclaim targeted claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let mut stale_state = read_social_state_json(runtime_dir.path());
    stale_state["pending_shared_channel_sync_requests"][alice_request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &stale_state);

    let stale_app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
    );

    let stale_inventory_response = stale_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale next write reclaim inventory should return response");
    assert_eq!(stale_inventory_response.status(), StatusCode::OK);
    let stale_inventory_body = stale_inventory_response
        .into_body()
        .collect()
        .await
        .expect("stale next write reclaim inventory body should collect")
        .to_bytes();
    let stale_inventory_json: serde_json::Value = serde_json::from_slice(&stale_inventory_body)
        .expect("stale next write reclaim inventory body should be valid json");
    let stale_inventory_item = stale_inventory_json["items"]
        .as_array()
        .expect("stale next write reclaim inventory should serialize items as an array")
        .iter()
        .find(|item| item["requestKey"] == alice_request_key)
        .expect("stale next write reclaim inventory item should exist");
    assert_eq!(stale_inventory_item["leaseStatus"], "stale");

    let second_link_response = stale_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_stale_next_write_reclaim_unconfigured_002",
                        "eventId":"evt_eml_stale_next_write_reclaim_unconfigured_002",
                        "connectionId":"ec_stale_next_write_reclaim_unconfigured_001",
                        "localActorId":"actor_bob",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-12T01:03:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second external member link write should return response");
    assert_eq!(second_link_response.status(), StatusCode::OK);

    let retried_state = read_social_state_json(runtime_dir.path());
    let retried_pending_items = retried_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should remain serialized as an object");
    assert_eq!(retried_pending_items.len(), 2);
    let alice_pending = retried_pending_items
        .get(alice_request_key.as_str())
        .expect("original stale pending request should stay in backlog");
    assert!(alice_pending["ownerActorId"].is_null());
    assert!(alice_pending["ownerActorKind"].is_null());
    assert!(alice_pending["claimedAt"].is_null());
    assert!(
        alice_pending["leaseExpiresAt"].is_null(),
        "next write should reclaim stale owner metadata even when dispatch is unconfigured"
    );

    let reclaimed_inventory_response = stale_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("reclaimed next write inventory should return response");
    assert_eq!(reclaimed_inventory_response.status(), StatusCode::OK);
    let reclaimed_inventory_body = reclaimed_inventory_response
        .into_body()
        .collect()
        .await
        .expect("reclaimed next write inventory body should collect")
        .to_bytes();
    let reclaimed_inventory_json: serde_json::Value =
        serde_json::from_slice(&reclaimed_inventory_body)
            .expect("reclaimed next write inventory body should be valid json");
    let reclaimed_inventory_item = reclaimed_inventory_json["items"]
        .as_array()
        .expect("reclaimed next write inventory should serialize items as an array")
        .iter()
        .find(|item| item["requestKey"] == alice_request_key)
        .expect("reclaimed next write inventory item should exist");
    assert_eq!(reclaimed_inventory_item["leaseStatus"], "unclaimed");
    assert_eq!(reclaimed_inventory_item["takeoverEligible"], false);
    assert_eq!(reclaimed_inventory_item["legacyTakeoverRequired"], false);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_repeated_failure_moves_request_to_dead_letter_and_stops_repair_retry()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir = TestRuntimeDir::new("control_plane_shared_channel_dead_letter");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_dead_letter",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(create_json["conversationId"], "c_partner_ops_dead_letter");

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_dead_letter/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_dead_letter_001",
            "summary":"hello dead letter",
            "text":"hello dead letter"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(post_json["messageId"], "msg_c_partner_ops_dead_letter_1");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during dead-letter retries",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
        trigger.clone(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_dead_letter_001",
                        "eventId":"evt_ec_dead_letter_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T02:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_dead_letter_001",
                        "eventId":"evt_scp_dead_letter_001",
                        "connectionId":"ec_dead_letter_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_dead_letter",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T02:21:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let dead_letter_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_dead_letter_001",
                        "eventId":"evt_eml_dead_letter_001",
                        "connectionId":"ec_dead_letter_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T02:22:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("dead-letter external member link write should return response");
    assert_eq!(
        dead_letter_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    for _ in 0..2 {
        let repair_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", "u_admin")
                    .header("x-permissions", "control.write")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("failing shared-channel sync repair should return response");
        assert_eq!(repair_response.status(), StatusCode::OK);
    }

    let dead_letter_state = read_social_state_json(runtime_dir.path());
    assert!(
        dead_letter_state["pending_shared_channel_sync_requests"]
            .as_object()
            .expect("pending shared-channel sync backlog should stay serialized as an object")
            .is_empty(),
        "repeated failure should remove the request from pending backlog"
    );
    let dead_letter_items = dead_letter_state["dead_letter_shared_channel_sync_requests"]
        .as_object()
        .expect("dead-letter shared-channel sync requests should be serialized as an object");
    assert_eq!(dead_letter_items.len(), 1);
    let dead_letter_item = dead_letter_items
        .values()
        .next()
        .expect("dead-letter shared-channel sync request should exist");
    assert_eq!(dead_letter_item["failureCount"], 3);

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    trigger.set_delegate(public_trigger);

    let repair_app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        trigger,
    );

    let repair_response = repair_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("healthy repair should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);
    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("healthy repair body should collect")
        .to_bytes();
    let repair_json: serde_json::Value =
        serde_json::from_slice(&repair_body).expect("healthy repair body should be valid json");
    assert_eq!(repair_json["status"], "noop");
    assert_eq!(repair_json["pendingBefore"], 0);
    assert_eq!(repair_json["attempted"], 0);
    assert_eq!(repair_json["deadLetterAfter"], 1);

    let healthy_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_dead_letter_002",
                        "eventId":"evt_eml_dead_letter_002",
                        "connectionId":"ec_dead_letter_001",
                        "localActorId":"actor_bob",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-11T02:23:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("healthy external member link write should return response");
    assert_eq!(healthy_link_response.status(), StatusCode::OK);

    let final_state = read_social_state_json(runtime_dir.path());
    assert_eq!(
        final_state["dead_letter_shared_channel_sync_requests"]
            .as_object()
            .expect("dead-letter shared-channel sync requests should stay serialized as an object")
            .len(),
        1,
        "next healthy ready-pair write should not auto-retry dead-lettered requests"
    );

    let alice_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (alice_history_status, _) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_dead_letter/messages",
        alice_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(alice_history_status, StatusCode::FORBIDDEN);

    let bob_bearer = bearer_token("t_demo", "actor_bob", "user", &[]);
    let (bob_history_status, bob_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_dead_letter/messages",
        bob_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(bob_history_status, StatusCode::OK);
    assert_eq!(
        bob_history_json["items"][0]["message"]["body"]["summary"],
        "hello dead letter"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_dead_letter_requeue_restores_pending_backlog_and_repair_materializes_remote_runtime()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir = TestRuntimeDir::new("control_plane_shared_channel_dead_letter_requeue");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_dead_letter_requeue",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_dead_letter_requeue"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_dead_letter_requeue/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_dead_letter_requeue_001",
            "summary":"hello dead letter requeue",
            "text":"hello dead letter requeue"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_dead_letter_requeue_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during dead-letter requeue test",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
        trigger.clone(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_dead_letter_requeue_001",
                        "eventId":"evt_ec_dead_letter_requeue_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T02:24:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_dead_letter_requeue_001",
                        "eventId":"evt_scp_dead_letter_requeue_001",
                        "connectionId":"ec_dead_letter_requeue_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_dead_letter_requeue",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T02:25:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let dead_letter_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_dead_letter_requeue_001",
                        "eventId":"evt_eml_dead_letter_requeue_001",
                        "connectionId":"ec_dead_letter_requeue_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T02:26:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("dead-letter external member link write should return response");
    assert_eq!(
        dead_letter_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    for _ in 0..2 {
        let repair_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", "u_admin")
                    .header("x-permissions", "control.write")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("failing shared-channel sync repair should return response");
        assert_eq!(repair_response.status(), StatusCode::OK);
    }

    let dead_letter_state = read_social_state_json(runtime_dir.path());
    assert!(
        dead_letter_state["pending_shared_channel_sync_requests"]
            .as_object()
            .expect("pending shared-channel sync backlog should stay serialized as an object")
            .is_empty()
    );
    assert_eq!(
        dead_letter_state["dead_letter_shared_channel_sync_requests"]
            .as_object()
            .expect("dead-letter shared-channel sync requests should be serialized as an object")
            .len(),
        1
    );

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    trigger.set_delegate(public_trigger);

    let requeue_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("dead-letter requeue should return response");
    assert_eq!(requeue_response.status(), StatusCode::OK);
    let requeue_body = requeue_response
        .into_body()
        .collect()
        .await
        .expect("dead-letter requeue body should collect")
        .to_bytes();
    let requeue_json: serde_json::Value = serde_json::from_slice(&requeue_body)
        .expect("dead-letter requeue body should be valid json");
    assert_eq!(requeue_json["status"], "requeued");
    assert_eq!(requeue_json["deadLetterBefore"], 1);
    assert_eq!(requeue_json["requeued"], 1);
    assert_eq!(requeue_json["pendingAfter"], 1);
    assert_eq!(requeue_json["deadLetterAfter"], 0);

    let repair_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("healthy repair after requeue should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);
    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("healthy repair after requeue body should collect")
        .to_bytes();
    let repair_json: serde_json::Value = serde_json::from_slice(&repair_body)
        .expect("healthy repair after requeue body should be valid json");
    assert_eq!(repair_json["status"], "repaired");
    assert_eq!(repair_json["pendingBefore"], 1);
    assert_eq!(repair_json["attempted"], 1);
    assert_eq!(repair_json["dispatched"], 1);
    assert_eq!(repair_json["failed"], 0);
    assert_eq!(repair_json["pendingAfter"], 0);
    assert_eq!(repair_json["deadLetterAfter"], 0);

    let final_state = read_social_state_json(runtime_dir.path());
    assert!(
        final_state["pending_shared_channel_sync_requests"]
            .as_object()
            .expect("pending shared-channel sync requests should stay serialized as an object")
            .is_empty()
    );
    assert!(
        final_state["dead_letter_shared_channel_sync_requests"]
            .as_object()
            .expect("dead-letter shared-channel sync requests should stay serialized as an object")
            .is_empty()
    );

    let alice_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (alice_history_status, alice_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_dead_letter_requeue/messages",
        alice_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(alice_history_status, StatusCode::OK);
    assert_eq!(
        alice_history_json["items"][0]["message"]["body"]["summary"],
        "hello dead letter requeue"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_dead_letter_requeue_rearms_failure_budget_before_next_repair_attempt()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir = TestRuntimeDir::new("control_plane_shared_channel_dead_letter_requeue_rearm");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_dead_letter_requeue_rearm",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_dead_letter_requeue_rearm"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_dead_letter_requeue_rearm/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_dead_letter_requeue_rearm_001",
            "summary":"hello dead letter requeue rearm",
            "text":"hello dead letter requeue rearm"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_dead_letter_requeue_rearm_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during dead-letter requeue rearm test",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        trigger,
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_dead_letter_requeue_rearm_001",
                        "eventId":"evt_ec_dead_letter_requeue_rearm_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T02:27:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_dead_letter_requeue_rearm_001",
                        "eventId":"evt_scp_dead_letter_requeue_rearm_001",
                        "connectionId":"ec_dead_letter_requeue_rearm_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_dead_letter_requeue_rearm",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T02:28:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let dead_letter_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_dead_letter_requeue_rearm_001",
                        "eventId":"evt_eml_dead_letter_requeue_rearm_001",
                        "connectionId":"ec_dead_letter_requeue_rearm_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T02:29:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("dead-letter external member link write should return response");
    assert_eq!(
        dead_letter_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    for _ in 0..2 {
        let repair_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", "u_admin")
                    .header("x-permissions", "control.write")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("failing shared-channel sync repair should return response");
        assert_eq!(repair_response.status(), StatusCode::OK);
    }

    let dead_letter_state = read_social_state_json(runtime_dir.path());
    assert_eq!(
        dead_letter_state["dead_letter_shared_channel_sync_requests"]
            .as_object()
            .expect("dead-letter shared-channel sync requests should be serialized as an object")
            .len(),
        1
    );

    let requeue_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("dead-letter requeue should return response");
    assert_eq!(requeue_response.status(), StatusCode::OK);

    let requeued_state = read_social_state_json(runtime_dir.path());
    let requeued_pending_items = requeued_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync requests should be serialized as an object");
    assert_eq!(requeued_pending_items.len(), 1);
    let requeued_pending_item = requeued_pending_items
        .values()
        .next()
        .expect("requeued pending item should exist");
    assert_eq!(
        requeued_pending_item["failureCount"], 0,
        "operator requeue should reset failure budget before the next repair attempt"
    );
    assert!(
        requeued_state["dead_letter_shared_channel_sync_requests"]
            .as_object()
            .expect("dead-letter shared-channel sync requests should be serialized as an object")
            .is_empty()
    );

    let failed_repair_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("failed repair after requeue should return response");
    assert_eq!(failed_repair_response.status(), StatusCode::OK);
    let failed_repair_body = failed_repair_response
        .into_body()
        .collect()
        .await
        .expect("failed repair after requeue body should collect")
        .to_bytes();
    let failed_repair_json: serde_json::Value = serde_json::from_slice(&failed_repair_body)
        .expect("failed repair after requeue body should be valid json");
    assert_eq!(failed_repair_json["status"], "pending");
    assert_eq!(failed_repair_json["failed"], 1);
    assert_eq!(failed_repair_json["pendingAfter"], 1);
    assert_eq!(failed_repair_json["deadLetterAfter"], 0);

    let final_state = read_social_state_json(runtime_dir.path());
    let final_pending_items = final_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync requests should stay serialized as an object");
    assert_eq!(final_pending_items.len(), 1);
    let final_pending_item = final_pending_items
        .values()
        .next()
        .expect("pending shared-channel sync request should still exist after one failed repair");
    assert_eq!(final_pending_item["failureCount"], 1);
    assert!(
        final_state["dead_letter_shared_channel_sync_requests"]
            .as_object()
            .expect("dead-letter shared-channel sync requests should stay serialized as an object")
            .is_empty(),
        "first failed repair after requeue should not immediately move the request back to dead-letter"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_dead_letter_requeue_reclaims_stale_claim_metadata()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_dead_letter_requeue_stale_reclaim");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_dead_letter_requeue_stale_reclaim",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_dead_letter_requeue_stale_reclaim"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_dead_letter_requeue_stale_reclaim/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_dead_letter_requeue_stale_reclaim_001",
            "summary":"hello dead letter stale reclaim",
            "text":"hello dead letter stale reclaim"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_dead_letter_requeue_stale_reclaim_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during dead-letter stale reclaim test",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
        trigger,
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_dead_letter_requeue_stale_reclaim_001",
                        "eventId":"evt_ec_dead_letter_requeue_stale_reclaim_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T07:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_dead_letter_requeue_stale_reclaim_001",
                        "eventId":"evt_scp_dead_letter_requeue_stale_reclaim_001",
                        "connectionId":"ec_dead_letter_requeue_stale_reclaim_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_dead_letter_requeue_stale_reclaim",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T07:21:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let dead_letter_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_dead_letter_requeue_stale_reclaim_001",
                        "eventId":"evt_eml_dead_letter_requeue_stale_reclaim_001",
                        "connectionId":"ec_dead_letter_requeue_stale_reclaim_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T07:22:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("dead-letter external member link write should return response");
    assert_eq!(
        dead_letter_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    for _ in 0..2 {
        let repair_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", "u_admin")
                    .header("x-permissions", "control.write")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("failing shared-channel sync repair should return response");
        assert_eq!(repair_response.status(), StatusCode::OK);
    }

    let mut dead_letter_state = read_social_state_json(runtime_dir.path());
    let dead_letter_items = dead_letter_state["dead_letter_shared_channel_sync_requests"]
        .as_object()
        .expect("dead-letter shared-channel sync requests should be serialized as an object");
    assert_eq!(dead_letter_items.len(), 1);
    let request_key = dead_letter_items
        .keys()
        .next()
        .expect("dead-letter request key should exist")
        .to_owned();
    dead_letter_state["dead_letter_shared_channel_sync_requests"][request_key.as_str()]["ownerActorId"] =
        serde_json::Value::String("u_operator_a".into());
    dead_letter_state["dead_letter_shared_channel_sync_requests"][request_key.as_str()]["ownerActorKind"] =
        serde_json::Value::String("user".into());
    dead_letter_state["dead_letter_shared_channel_sync_requests"][request_key.as_str()]["claimedAt"] =
        serde_json::Value::String("2026-04-11T07:23:00.000Z".into());
    dead_letter_state["dead_letter_shared_channel_sync_requests"][request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &dead_letter_state);

    let requeue_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster,
            ops_runtime,
            audit_runtime,
            runtime_dir.path(),
        );

    let requeue_response = requeue_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("dead-letter requeue should return response");
    assert_eq!(requeue_response.status(), StatusCode::OK);

    let requeued_state = read_social_state_json(runtime_dir.path());
    let requeued_pending_items = requeued_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync requests should be serialized as an object");
    assert_eq!(requeued_pending_items.len(), 1);
    let requeued_pending_item = requeued_pending_items
        .get(request_key.as_str())
        .expect("requeued pending item should exist");
    assert!(requeued_pending_item["ownerActorId"].is_null());
    assert!(requeued_pending_item["ownerActorKind"].is_null());
    assert!(requeued_pending_item["claimedAt"].is_null());
    assert!(
        requeued_pending_item["leaseExpiresAt"].is_null(),
        "dead-letter requeue should reclaim stale owner metadata before restoring pending backlog"
    );
    assert_eq!(requeued_pending_item["failureCount"], 0);

    let inventory_response = requeue_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("requeued pending inventory should return response");
    assert_eq!(inventory_response.status(), StatusCode::OK);
    let inventory_body = inventory_response
        .into_body()
        .collect()
        .await
        .expect("requeued pending inventory body should collect")
        .to_bytes();
    let inventory_json: serde_json::Value = serde_json::from_slice(&inventory_body)
        .expect("requeued pending inventory body should be valid json");
    let inventory_item = inventory_json["items"]
        .as_array()
        .expect("requeued pending inventory should serialize items as an array")
        .first()
        .expect("requeued pending inventory item should exist");
    assert_eq!(inventory_item["requestKey"], request_key);
    assert_eq!(inventory_item["leaseStatus"], "unclaimed");
    assert_eq!(inventory_item["takeoverEligible"], false);
    assert_eq!(inventory_item["legacyTakeoverRequired"], false);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_dead_letter_inventory_and_targeted_requeue_only_restores_selected_actor()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_dead_letter_targeted_requeue");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_dead_letter_targeted_requeue",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_dead_letter_targeted_requeue"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_dead_letter_targeted_requeue/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_dead_letter_targeted_requeue_001",
            "summary":"hello dead letter targeted requeue",
            "text":"hello dead letter targeted requeue"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_dead_letter_targeted_requeue_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during dead-letter targeted requeue test",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
        trigger.clone(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_dead_letter_targeted_requeue_001",
                        "eventId":"evt_ec_dead_letter_targeted_requeue_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T02:30:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_dead_letter_targeted_requeue_001",
                        "eventId":"evt_scp_dead_letter_targeted_requeue_001",
                        "connectionId":"ec_dead_letter_targeted_requeue_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_dead_letter_targeted_requeue",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T02:31:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let alice_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_dead_letter_targeted_requeue_001",
                        "eventId":"evt_eml_dead_letter_targeted_requeue_001",
                        "connectionId":"ec_dead_letter_targeted_requeue_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T02:32:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("alice dead-letter external member link write should return response");
    assert_eq!(
        alice_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    let bob_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_dead_letter_targeted_requeue_002",
                        "eventId":"evt_eml_dead_letter_targeted_requeue_002",
                        "connectionId":"ec_dead_letter_targeted_requeue_001",
                        "localActorId":"actor_bob",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-11T02:33:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("bob dead-letter external member link write should return response");
    assert_eq!(bob_link_response.status(), StatusCode::SERVICE_UNAVAILABLE);

    for _ in 0..2 {
        let repair_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                    .header("x-tenant-id", "t_demo")
                    .header("x-user-id", "u_admin")
                    .header("x-permissions", "control.write")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("failing shared-channel sync repair should return response");
        assert_eq!(repair_response.status(), StatusCode::OK);
    }

    let dead_letter_state = read_social_state_json(runtime_dir.path());
    assert_eq!(
        dead_letter_state["dead_letter_shared_channel_sync_requests"]
            .as_object()
            .expect("dead-letter shared-channel sync requests should be serialized as an object")
            .len(),
        2
    );

    let inventory_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/dead-letter-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("dead-letter inventory should return response");
    assert_eq!(inventory_response.status(), StatusCode::OK);
    let inventory_body = inventory_response
        .into_body()
        .collect()
        .await
        .expect("dead-letter inventory body should collect")
        .to_bytes();
    let inventory_json: serde_json::Value = serde_json::from_slice(&inventory_body)
        .expect("dead-letter inventory body should be valid json");
    assert_eq!(inventory_json["deadLetterCount"], 2);
    let inventory_items = inventory_json["items"]
        .as_array()
        .expect("dead-letter inventory should serialize items as an array");
    assert_eq!(inventory_items.len(), 2);

    let alice_request_key = inventory_items
        .iter()
        .find(|item| item["request"]["localActorId"] == "actor_alice")
        .expect("alice dead-letter inventory item should exist");
    assert_eq!(alice_request_key["leaseStatus"], "unclaimed");
    assert_eq!(alice_request_key["takeoverEligible"], false);
    assert_eq!(alice_request_key["legacyTakeoverRequired"], false);
    assert!(alice_request_key["ownerActorId"].is_null());
    assert!(alice_request_key["ownerActorKind"].is_null());
    assert!(alice_request_key["claimedAt"].is_null());
    assert!(alice_request_key["leaseExpiresAt"].is_null());
    let alice_request_key = alice_request_key["requestKey"]
        .as_str()
        .expect("alice dead-letter inventory item should expose a stable request key")
        .to_owned();
    let bob_request_key = inventory_items
        .iter()
        .find(|item| item["request"]["localActorId"] == "actor_bob")
        .expect("bob dead-letter inventory item should exist");
    assert_eq!(bob_request_key["leaseStatus"], "unclaimed");
    assert_eq!(bob_request_key["takeoverEligible"], false);
    assert_eq!(bob_request_key["legacyTakeoverRequired"], false);
    assert!(bob_request_key["ownerActorId"].is_null());
    assert!(bob_request_key["ownerActorKind"].is_null());
    assert!(bob_request_key["claimedAt"].is_null());
    assert!(bob_request_key["leaseExpiresAt"].is_null());
    let bob_request_key = bob_request_key["requestKey"]
        .as_str()
        .expect("bob dead-letter inventory item should expose a stable request key")
        .to_owned();
    assert_ne!(alice_request_key, bob_request_key);

    let targeted_requeue_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("targeted dead-letter requeue request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("targeted dead-letter requeue should return response");
    assert_eq!(targeted_requeue_response.status(), StatusCode::OK);
    let targeted_requeue_body = targeted_requeue_response
        .into_body()
        .collect()
        .await
        .expect("targeted dead-letter requeue body should collect")
        .to_bytes();
    let targeted_requeue_json: serde_json::Value = serde_json::from_slice(&targeted_requeue_body)
        .expect("targeted dead-letter requeue body should be valid json");
    assert_eq!(targeted_requeue_json["status"], "requeued");
    assert_eq!(targeted_requeue_json["requested"], 1);
    assert_eq!(targeted_requeue_json["deadLetterBefore"], 2);
    assert_eq!(targeted_requeue_json["requeued"], 1);
    assert_eq!(targeted_requeue_json["pendingAfter"], 1);
    assert_eq!(targeted_requeue_json["deadLetterAfter"], 1);

    let requeued_state = read_social_state_json(runtime_dir.path());
    assert_eq!(
        requeued_state["pending_shared_channel_sync_requests"]
            .as_object()
            .expect("pending shared-channel sync requests should be serialized as an object")
            .len(),
        1
    );
    let remaining_dead_letters = requeued_state["dead_letter_shared_channel_sync_requests"]
        .as_object()
        .expect("dead-letter shared-channel sync requests should be serialized as an object");
    assert_eq!(remaining_dead_letters.len(), 1);
    let remaining_dead_letter = remaining_dead_letters
        .values()
        .next()
        .expect("one dead-letter request should remain after targeted requeue");
    assert_eq!(
        remaining_dead_letter["request"]["localActorId"],
        "actor_bob"
    );

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    trigger.set_delegate(public_trigger);

    let repair_app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        trigger,
    );

    let repair_response = repair_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("healthy repair after targeted requeue should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);
    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("healthy repair after targeted requeue body should collect")
        .to_bytes();
    let repair_json: serde_json::Value = serde_json::from_slice(&repair_body)
        .expect("healthy repair after targeted requeue body should be valid json");
    assert_eq!(repair_json["status"], "repaired");
    assert_eq!(repair_json["pendingBefore"], 1);
    assert_eq!(repair_json["attempted"], 1);
    assert_eq!(repair_json["dispatched"], 1);
    assert_eq!(repair_json["failed"], 0);
    assert_eq!(repair_json["pendingAfter"], 0);
    assert_eq!(repair_json["deadLetterAfter"], 1);

    let final_state = read_social_state_json(runtime_dir.path());
    assert!(
        final_state["pending_shared_channel_sync_requests"]
            .as_object()
            .expect("pending shared-channel sync requests should stay serialized as an object")
            .is_empty()
    );
    let final_dead_letters = final_state["dead_letter_shared_channel_sync_requests"]
        .as_object()
        .expect("dead-letter shared-channel sync requests should stay serialized as an object");
    assert_eq!(final_dead_letters.len(), 1);
    let final_dead_letter = final_dead_letters
        .values()
        .next()
        .expect("unselected dead-letter request should remain isolated");
    assert_eq!(final_dead_letter["request"]["localActorId"], "actor_bob");

    let alice_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (alice_history_status, alice_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_dead_letter_targeted_requeue/messages",
        alice_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(alice_history_status, StatusCode::OK);
    assert_eq!(
        alice_history_json["items"][0]["message"]["body"]["summary"],
        "hello dead letter targeted requeue"
    );

    let bob_bearer = bearer_token("t_demo", "actor_bob", "user", &[]);
    let (bob_history_status, _) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_dead_letter_targeted_requeue/messages",
        bob_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(bob_history_status, StatusCode::FORBIDDEN);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_pending_inventory_and_targeted_republish_only_materializes_selected_actor()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_pending_targeted_republish");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_pending_targeted_republish",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_pending_targeted_republish"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_pending_targeted_republish/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_pending_targeted_republish_001",
            "summary":"hello pending targeted republish",
            "text":"hello pending targeted republish"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_pending_targeted_republish_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let pending_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster.clone(),
            ops_runtime.clone(),
            audit_runtime.clone(),
            runtime_dir.path(),
        );

    let establish_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_pending_targeted_republish_001",
                        "eventId":"evt_ec_pending_targeted_republish_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T02:34:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_pending_targeted_republish_001",
                        "eventId":"evt_scp_pending_targeted_republish_001",
                        "connectionId":"ec_pending_targeted_republish_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_pending_targeted_republish",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T02:35:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let alice_link_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_pending_targeted_republish_001",
                        "eventId":"evt_eml_pending_targeted_republish_001",
                        "connectionId":"ec_pending_targeted_republish_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T02:36:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("alice pending external member link write should return response");
    assert_eq!(alice_link_response.status(), StatusCode::OK);

    let bob_link_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_pending_targeted_republish_002",
                        "eventId":"evt_eml_pending_targeted_republish_002",
                        "connectionId":"ec_pending_targeted_republish_001",
                        "localActorId":"actor_bob",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-11T02:37:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("bob pending external member link write should return response");
    assert_eq!(bob_link_response.status(), StatusCode::OK);

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 2);
    assert!(
        pending_state["dead_letter_shared_channel_sync_requests"]
            .as_object()
            .expect("dead-letter shared-channel sync requests should be serialized as an object")
            .is_empty()
    );

    let pending_inventory_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pending shared-channel sync inventory should return response");
    assert_eq!(pending_inventory_response.status(), StatusCode::OK);
    let pending_inventory_body = pending_inventory_response
        .into_body()
        .collect()
        .await
        .expect("pending shared-channel sync inventory body should collect")
        .to_bytes();
    let pending_inventory_json: serde_json::Value = serde_json::from_slice(&pending_inventory_body)
        .expect("pending shared-channel sync inventory body should be valid json");
    assert_eq!(pending_inventory_json["pendingCount"], 2);
    let pending_inventory_items = pending_inventory_json["items"]
        .as_array()
        .expect("pending shared-channel sync inventory should serialize items as an array");
    assert_eq!(pending_inventory_items.len(), 2);

    let alice_request_key = pending_inventory_items
        .iter()
        .find(|item| item["request"]["localActorId"] == "actor_alice")
        .and_then(|item| item["requestKey"].as_str())
        .expect("alice pending inventory item should expose a stable request key")
        .to_owned();
    let bob_request_key = pending_inventory_items
        .iter()
        .find(|item| item["request"]["localActorId"] == "actor_bob")
        .and_then(|item| item["requestKey"].as_str())
        .expect("bob pending inventory item should expose a stable request key")
        .to_owned();
    assert_ne!(alice_request_key, bob_request_key);

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    let republish_app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        public_trigger,
    );

    let claim_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("targeted pending claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("targeted pending claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let targeted_republish_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("targeted pending republish request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("targeted pending republish should return response");
    assert_eq!(targeted_republish_response.status(), StatusCode::OK);
    let targeted_republish_body = targeted_republish_response
        .into_body()
        .collect()
        .await
        .expect("targeted pending republish body should collect")
        .to_bytes();
    let targeted_republish_json: serde_json::Value =
        serde_json::from_slice(&targeted_republish_body)
            .expect("targeted pending republish body should be valid json");
    assert_eq!(targeted_republish_json["status"], "republished");
    assert_eq!(targeted_republish_json["pendingBefore"], 2);
    assert_eq!(targeted_republish_json["requested"], 1);
    assert_eq!(targeted_republish_json["attempted"], 1);
    assert_eq!(targeted_republish_json["dispatched"], 1);
    assert_eq!(targeted_republish_json["failed"], 0);
    assert_eq!(targeted_republish_json["pendingAfter"], 1);
    assert_eq!(targeted_republish_json["deadLetterAfter"], 0);

    let final_state = read_social_state_json(runtime_dir.path());
    let final_pending_items = final_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync requests should stay serialized as an object");
    assert_eq!(final_pending_items.len(), 1);
    let final_pending_item = final_pending_items
        .values()
        .next()
        .expect("unselected pending request should remain in backlog");
    assert_eq!(final_pending_item["request"]["localActorId"], "actor_bob");
    assert!(
        final_state["dead_letter_shared_channel_sync_requests"]
            .as_object()
            .expect("dead-letter shared-channel sync requests should stay serialized as an object")
            .is_empty()
    );

    let alice_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (alice_history_status, alice_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_pending_targeted_republish/messages",
        alice_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(alice_history_status, StatusCode::OK);
    assert_eq!(
        alice_history_json["items"][0]["message"]["body"]["summary"],
        "hello pending targeted republish"
    );

    let bob_bearer = bearer_token("t_demo", "actor_bob", "user", &[]);
    let (bob_history_status, _) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_pending_targeted_republish/messages",
        bob_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(bob_history_status, StatusCode::FORBIDDEN);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_pending_claim_enforces_targeted_republish_ownership()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_pending_claim_targeted_republish");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_pending_claim_targeted_republish",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_pending_claim_targeted_republish"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_pending_claim_targeted_republish/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_pending_claim_targeted_republish_001",
            "summary":"hello pending claim targeted republish",
            "text":"hello pending claim targeted republish"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_pending_claim_targeted_republish_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let pending_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster.clone(),
            ops_runtime.clone(),
            audit_runtime.clone(),
            runtime_dir.path(),
        );

    let establish_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_pending_claim_targeted_republish_001",
                        "eventId":"evt_ec_pending_claim_targeted_republish_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T03:04:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_pending_claim_targeted_republish_001",
                        "eventId":"evt_scp_pending_claim_targeted_republish_001",
                        "connectionId":"ec_pending_claim_targeted_republish_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_pending_claim_targeted_republish",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T03:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let alice_link_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_pending_claim_targeted_republish_001",
                        "eventId":"evt_eml_pending_claim_targeted_republish_001",
                        "connectionId":"ec_pending_claim_targeted_republish_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T03:06:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("alice pending external member link write should return response");
    assert_eq!(alice_link_response.status(), StatusCode::OK);

    let bob_link_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_pending_claim_targeted_republish_002",
                        "eventId":"evt_eml_pending_claim_targeted_republish_002",
                        "connectionId":"ec_pending_claim_targeted_republish_001",
                        "localActorId":"actor_bob",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-11T03:07:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("bob pending external member link write should return response");
    assert_eq!(bob_link_response.status(), StatusCode::OK);

    let pending_inventory_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pending shared-channel sync inventory should return response");
    assert_eq!(pending_inventory_response.status(), StatusCode::OK);
    let pending_inventory_body = pending_inventory_response
        .into_body()
        .collect()
        .await
        .expect("pending shared-channel sync inventory body should collect")
        .to_bytes();
    let pending_inventory_json: serde_json::Value = serde_json::from_slice(&pending_inventory_body)
        .expect("pending shared-channel sync inventory body should be valid json");
    assert_eq!(pending_inventory_json["pendingCount"], 2);
    let pending_inventory_items = pending_inventory_json["items"]
        .as_array()
        .expect("pending shared-channel sync inventory should serialize items as an array");
    assert_eq!(pending_inventory_items.len(), 2);

    let alice_pending_item = pending_inventory_items
        .iter()
        .find(|item| item["request"]["localActorId"] == "actor_alice")
        .expect("alice pending inventory item should exist");
    let alice_request_key = alice_pending_item["requestKey"]
        .as_str()
        .expect("alice pending inventory item should expose a stable request key")
        .to_owned();
    assert!(alice_pending_item["ownerActorId"].is_null());
    assert!(alice_pending_item["ownerActorKind"].is_null());

    let bob_pending_item = pending_inventory_items
        .iter()
        .find(|item| item["request"]["localActorId"] == "actor_bob")
        .expect("bob pending inventory item should exist");
    let bob_request_key = bob_pending_item["requestKey"]
        .as_str()
        .expect("bob pending inventory item should expose a stable request key")
        .to_owned();
    assert_ne!(alice_request_key, bob_request_key);
    assert!(bob_pending_item["ownerActorId"].is_null());
    assert!(bob_pending_item["ownerActorKind"].is_null());

    let claim_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("targeted pending claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("targeted pending claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);
    let claim_body = claim_response
        .into_body()
        .collect()
        .await
        .expect("targeted pending claim body should collect")
        .to_bytes();
    let claim_json: serde_json::Value = serde_json::from_slice(&claim_body)
        .expect("targeted pending claim body should be valid json");
    assert_eq!(claim_json["status"], "claimed");
    assert_eq!(claim_json["pendingBefore"], 2);
    assert_eq!(claim_json["requested"], 1);
    assert_eq!(claim_json["claimed"], 1);
    assert_eq!(claim_json["conflicted"], 0);
    assert_eq!(claim_json["pendingAfter"], 2);

    let claimed_inventory_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("claimed pending shared-channel sync inventory should return response");
    assert_eq!(claimed_inventory_response.status(), StatusCode::OK);
    let claimed_inventory_body = claimed_inventory_response
        .into_body()
        .collect()
        .await
        .expect("claimed pending shared-channel sync inventory body should collect")
        .to_bytes();
    let claimed_inventory_json: serde_json::Value = serde_json::from_slice(&claimed_inventory_body)
        .expect("claimed pending shared-channel sync inventory body should be valid json");
    let claimed_inventory_items = claimed_inventory_json["items"]
        .as_array()
        .expect("claimed pending shared-channel sync inventory should serialize items as an array");
    let claimed_alice_item = claimed_inventory_items
        .iter()
        .find(|item| item["requestKey"] == alice_request_key)
        .expect("claimed alice pending item should still exist");
    assert_eq!(claimed_alice_item["ownerActorId"], "u_operator_a");
    assert_eq!(claimed_alice_item["ownerActorKind"], "user");
    assert_eq!(claimed_alice_item["leaseStatus"], "active");
    assert_eq!(claimed_alice_item["takeoverEligible"], false);
    assert_eq!(claimed_alice_item["legacyTakeoverRequired"], false);
    let claimed_alice_lease_expires_at = claimed_alice_item["leaseExpiresAt"]
        .as_str()
        .expect("claimed alice pending item should expose leaseExpiresAt after claim")
        .to_owned();
    let claimed_bob_item = claimed_inventory_items
        .iter()
        .find(|item| item["requestKey"] == bob_request_key)
        .expect("bob pending item should still exist");
    assert!(claimed_bob_item["ownerActorId"].is_null());
    assert!(claimed_bob_item["ownerActorKind"].is_null());

    let foreign_claim_conflict_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("foreign targeted pending claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("foreign targeted pending claim should return response");
    assert_eq!(foreign_claim_conflict_response.status(), StatusCode::OK);
    let foreign_claim_conflict_body = foreign_claim_conflict_response
        .into_body()
        .collect()
        .await
        .expect("foreign targeted pending claim conflict body should collect")
        .to_bytes();
    let foreign_claim_conflict_json: serde_json::Value =
        serde_json::from_slice(&foreign_claim_conflict_body)
            .expect("foreign targeted pending claim conflict body should be valid json");
    assert_eq!(foreign_claim_conflict_json["status"], "conflict");
    assert_eq!(foreign_claim_conflict_json["pendingBefore"], 2);
    assert_eq!(foreign_claim_conflict_json["requested"], 1);
    assert_eq!(foreign_claim_conflict_json["claimed"], 0);
    assert_eq!(foreign_claim_conflict_json["conflicted"], 1);
    assert_eq!(foreign_claim_conflict_json["pendingAfter"], 2);
    let conflict_items = foreign_claim_conflict_json["conflictItems"]
        .as_array()
        .expect("foreign targeted pending claim should expose conflictItems as an array");
    assert_eq!(conflict_items.len(), 1);
    assert_eq!(conflict_items[0]["requestKey"], alice_request_key);
    assert_eq!(conflict_items[0]["ownerActorId"], "u_operator_a");
    assert_eq!(
        conflict_items[0]["leaseExpiresAt"],
        claimed_alice_lease_expires_at
    );
    assert_eq!(conflict_items[0]["leaseStatus"], "active");
    assert_eq!(conflict_items[0]["takeoverEligible"], false);
    assert_eq!(conflict_items[0]["legacyTakeoverRequired"], false);
    assert_eq!(
        conflict_items[0]["suggestedAction"],
        "wait_for_owner_release_or_expiry"
    );

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    let republish_app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        public_trigger,
    );

    let foreign_republish_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("foreign targeted pending republish request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("foreign targeted pending republish should return response");
    assert_eq!(foreign_republish_response.status(), StatusCode::CONFLICT);
    let foreign_republish_body = foreign_republish_response
        .into_body()
        .collect()
        .await
        .expect("foreign targeted pending republish body should collect")
        .to_bytes();
    let foreign_republish_json: serde_json::Value = serde_json::from_slice(&foreign_republish_body)
        .expect("foreign targeted pending republish body should be valid json");
    assert_eq!(
        foreign_republish_json["code"],
        "shared_channel_sync_owner_conflict"
    );
    assert_eq!(
        foreign_republish_json["details"]["requestKey"],
        alice_request_key
    );
    assert_eq!(
        foreign_republish_json["details"]["ownerActorId"],
        "u_operator_a"
    );
    assert_eq!(
        foreign_republish_json["details"]["leaseExpiresAt"],
        claimed_alice_lease_expires_at
    );
    assert_eq!(foreign_republish_json["details"]["leaseStatus"], "active");
    assert_eq!(foreign_republish_json["details"]["takeoverEligible"], false);
    assert_eq!(
        foreign_republish_json["details"]["legacyTakeoverRequired"],
        false
    );
    assert_eq!(
        foreign_republish_json["details"]["suggestedAction"],
        "wait_for_owner_release_or_expiry"
    );

    let owner_republish_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("owner targeted pending republish request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("owner targeted pending republish should return response");
    assert_eq!(owner_republish_response.status(), StatusCode::OK);
    let owner_republish_body = owner_republish_response
        .into_body()
        .collect()
        .await
        .expect("owner targeted pending republish body should collect")
        .to_bytes();
    let owner_republish_json: serde_json::Value = serde_json::from_slice(&owner_republish_body)
        .expect("owner targeted pending republish body should be valid json");
    assert_eq!(owner_republish_json["status"], "republished");
    assert_eq!(owner_republish_json["requested"], 1);
    assert_eq!(owner_republish_json["attempted"], 1);
    assert_eq!(owner_republish_json["dispatched"], 1);
    assert_eq!(owner_republish_json["failed"], 0);
    assert_eq!(owner_republish_json["pendingAfter"], 1);
    assert_eq!(owner_republish_json["deadLetterAfter"], 0);

    let final_state = read_social_state_json(runtime_dir.path());
    let final_pending_items = final_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync requests should stay serialized as an object");
    assert_eq!(final_pending_items.len(), 1);
    let final_pending_item = final_pending_items
        .values()
        .next()
        .expect("only the unclaimed bob request should remain pending");
    assert_eq!(final_pending_item["request"]["localActorId"], "actor_bob");
    assert!(final_pending_item["ownerActorId"].is_null());
    assert!(final_pending_item["ownerActorKind"].is_null());

    let alice_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (alice_history_status, alice_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_pending_claim_targeted_republish/messages",
        alice_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(alice_history_status, StatusCode::OK);
    assert_eq!(
        alice_history_json["items"][0]["message"]["body"]["summary"],
        "hello pending claim targeted republish"
    );

    let bob_bearer = bearer_token("t_demo", "actor_bob", "user", &[]);
    let (bob_history_status, _) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_pending_claim_targeted_republish/messages",
        bob_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(bob_history_status, StatusCode::FORBIDDEN);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_targeted_republish_dead_letter_reclaims_claim_metadata()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir = TestRuntimeDir::new(
        "control_plane_shared_channel_pending_claim_dead_letter_targeted_republish",
    );
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_pending_claim_dead_letter_targeted_republish",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_pending_claim_dead_letter_targeted_republish"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_pending_claim_dead_letter_targeted_republish/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_pending_claim_dead_letter_targeted_republish_001",
            "summary":"hello pending claim dead letter targeted republish",
            "text":"hello pending claim dead letter targeted republish"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_pending_claim_dead_letter_targeted_republish_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during targeted dead-letter republish test",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        trigger,
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_pending_claim_dead_letter_targeted_republish_001",
                        "eventId":"evt_ec_pending_claim_dead_letter_targeted_republish_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T08:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_pending_claim_dead_letter_targeted_republish_001",
                        "eventId":"evt_scp_pending_claim_dead_letter_targeted_republish_001",
                        "connectionId":"ec_pending_claim_dead_letter_targeted_republish_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_pending_claim_dead_letter_targeted_republish",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T08:02:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let alice_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_pending_claim_dead_letter_targeted_republish_001",
                        "eventId":"evt_eml_pending_claim_dead_letter_targeted_republish_001",
                        "connectionId":"ec_pending_claim_dead_letter_targeted_republish_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T08:03:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("alice external member link write should return response");
    assert_eq!(
        alice_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    let pending_inventory_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pending shared-channel sync inventory should return response");
    assert_eq!(pending_inventory_response.status(), StatusCode::OK);
    let pending_inventory_body = pending_inventory_response
        .into_body()
        .collect()
        .await
        .expect("pending shared-channel sync inventory body should collect")
        .to_bytes();
    let pending_inventory_json: serde_json::Value = serde_json::from_slice(&pending_inventory_body)
        .expect("pending shared-channel sync inventory body should be valid json");
    let pending_item = pending_inventory_json["items"]
        .as_array()
        .expect("pending shared-channel sync inventory should serialize items as an array")
        .first()
        .expect("pending inventory item should exist");
    let alice_request_key = pending_item["requestKey"]
        .as_str()
        .expect("pending inventory item should expose a stable request key")
        .to_owned();
    assert!(pending_item["ownerActorId"].is_null());
    assert!(pending_item["ownerActorKind"].is_null());

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("targeted pending claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("targeted pending claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let first_republish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("first targeted pending republish request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("first targeted pending republish should return response");
    assert_eq!(first_republish_response.status(), StatusCode::OK);
    let first_republish_body = first_republish_response
        .into_body()
        .collect()
        .await
        .expect("first targeted pending republish body should collect")
        .to_bytes();
    let first_republish_json: serde_json::Value = serde_json::from_slice(&first_republish_body)
        .expect("first targeted pending republish body should be valid json");
    assert_eq!(first_republish_json["status"], "pending");
    assert_eq!(first_republish_json["requested"], 1);
    assert_eq!(first_republish_json["attempted"], 1);
    assert_eq!(first_republish_json["failed"], 1);
    assert_eq!(first_republish_json["pendingAfter"], 1);
    assert_eq!(first_republish_json["deadLettered"], 0);
    assert_eq!(first_republish_json["deadLetterAfter"], 0);

    let second_republish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("second targeted pending republish request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("second targeted pending republish should return response");
    assert_eq!(second_republish_response.status(), StatusCode::OK);
    let second_republish_body = second_republish_response
        .into_body()
        .collect()
        .await
        .expect("second targeted pending republish body should collect")
        .to_bytes();
    let second_republish_json: serde_json::Value = serde_json::from_slice(&second_republish_body)
        .expect("second targeted pending republish body should be valid json");
    assert_eq!(second_republish_json["status"], "dead_lettered");
    assert_eq!(second_republish_json["requested"], 1);
    assert_eq!(second_republish_json["attempted"], 1);
    assert_eq!(second_republish_json["failed"], 1);
    assert_eq!(second_republish_json["pendingAfter"], 0);
    assert_eq!(second_republish_json["deadLettered"], 1);
    assert_eq!(second_republish_json["deadLetterAfter"], 1);

    let final_state = read_social_state_json(runtime_dir.path());
    assert!(
        final_state["pending_shared_channel_sync_requests"]
            .as_object()
            .expect("pending shared-channel sync requests should stay serialized as an object")
            .is_empty()
    );
    let dead_letter_items = final_state["dead_letter_shared_channel_sync_requests"]
        .as_object()
        .expect("dead-letter shared-channel sync requests should stay serialized as an object");
    assert_eq!(dead_letter_items.len(), 1);
    let dead_letter_item = dead_letter_items
        .get(alice_request_key.as_str())
        .expect("dead-letter shared-channel sync request should preserve the request key");
    assert_eq!(dead_letter_item["failureCount"], 3);
    assert!(
        dead_letter_item["ownerActorId"].is_null(),
        "dead-lettered requests should drop ownerActorId when they leave the pending claim pool"
    );
    assert!(dead_letter_item["ownerActorKind"].is_null());
    assert!(dead_letter_item["claimedAt"].is_null());
    assert!(dead_letter_item["leaseExpiresAt"].is_null());

    let dead_letter_inventory_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/dead-letter-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("dead-letter inventory should return response");
    assert_eq!(dead_letter_inventory_response.status(), StatusCode::OK);
    let dead_letter_inventory_body = dead_letter_inventory_response
        .into_body()
        .collect()
        .await
        .expect("dead-letter inventory body should collect")
        .to_bytes();
    let dead_letter_inventory_json: serde_json::Value =
        serde_json::from_slice(&dead_letter_inventory_body)
            .expect("dead-letter inventory body should be valid json");
    let dead_letter_inventory_item = dead_letter_inventory_json["items"]
        .as_array()
        .expect("dead-letter inventory should serialize items as an array")
        .first()
        .expect("dead-letter inventory item should exist");
    assert_eq!(dead_letter_inventory_item["requestKey"], alice_request_key);
    assert_eq!(dead_letter_inventory_item["leaseStatus"], "unclaimed");
    assert_eq!(dead_letter_inventory_item["takeoverEligible"], false);
    assert_eq!(dead_letter_inventory_item["legacyTakeoverRequired"], false);
    assert!(dead_letter_inventory_item["ownerActorId"].is_null());
    assert!(dead_letter_inventory_item["ownerActorKind"].is_null());
    assert!(dead_letter_inventory_item["claimedAt"].is_null());
    assert!(dead_letter_inventory_item["leaseExpiresAt"].is_null());

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_pending_release_returns_claim_to_unowned_pool() {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_pending_release_targeted_republish");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_pending_release_targeted_republish",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_pending_release_targeted_republish"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_pending_release_targeted_republish/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_pending_release_targeted_republish_001",
            "summary":"hello pending release targeted republish",
            "text":"hello pending release targeted republish"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_pending_release_targeted_republish_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let pending_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster.clone(),
            ops_runtime.clone(),
            audit_runtime.clone(),
            runtime_dir.path(),
        );

    let establish_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_pending_release_targeted_republish_001",
                        "eventId":"evt_ec_pending_release_targeted_republish_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T03:34:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_pending_release_targeted_republish_001",
                        "eventId":"evt_scp_pending_release_targeted_republish_001",
                        "connectionId":"ec_pending_release_targeted_republish_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_pending_release_targeted_republish",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T03:35:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let alice_link_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_pending_release_targeted_republish_001",
                        "eventId":"evt_eml_pending_release_targeted_republish_001",
                        "connectionId":"ec_pending_release_targeted_republish_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T03:36:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("alice pending external member link write should return response");
    assert_eq!(alice_link_response.status(), StatusCode::OK);

    let pending_inventory_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pending shared-channel sync inventory should return response");
    assert_eq!(pending_inventory_response.status(), StatusCode::OK);
    let pending_inventory_body = pending_inventory_response
        .into_body()
        .collect()
        .await
        .expect("pending shared-channel sync inventory body should collect")
        .to_bytes();
    let pending_inventory_json: serde_json::Value = serde_json::from_slice(&pending_inventory_body)
        .expect("pending shared-channel sync inventory body should be valid json");
    assert_eq!(pending_inventory_json["pendingCount"], 1);
    let pending_inventory_items = pending_inventory_json["items"]
        .as_array()
        .expect("pending shared-channel sync inventory should serialize items as an array");
    assert_eq!(pending_inventory_items.len(), 1);
    assert_eq!(pending_inventory_items[0]["leaseStatus"], "unclaimed");
    assert_eq!(pending_inventory_items[0]["takeoverEligible"], false);
    assert!(pending_inventory_items[0]["claimedAt"].is_null());
    assert!(pending_inventory_items[0]["leaseExpiresAt"].is_null());
    let alice_request_key = pending_inventory_items[0]["requestKey"]
        .as_str()
        .expect("alice pending inventory item should expose a stable request key")
        .to_owned();

    let claim_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("targeted pending claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("targeted pending claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let claimed_inventory_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("claimed pending shared-channel sync inventory should return response");
    assert_eq!(claimed_inventory_response.status(), StatusCode::OK);
    let claimed_inventory_body = claimed_inventory_response
        .into_body()
        .collect()
        .await
        .expect("claimed pending shared-channel sync inventory body should collect")
        .to_bytes();
    let claimed_inventory_json: serde_json::Value = serde_json::from_slice(&claimed_inventory_body)
        .expect("claimed pending shared-channel sync inventory body should be valid json");
    let claimed_item = claimed_inventory_json["items"]
        .as_array()
        .expect("claimed pending shared-channel sync inventory should serialize items as an array")
        .first()
        .expect("claimed pending inventory item should still exist");
    assert_eq!(claimed_item["ownerActorId"], "u_operator_a");
    assert_eq!(claimed_item["ownerActorKind"], "user");
    assert_eq!(claimed_item["leaseStatus"], "active");
    assert_eq!(claimed_item["takeoverEligible"], false);
    let claimed_at = claimed_item["claimedAt"]
        .as_str()
        .expect("claimed pending inventory item should expose claimedAt after claim");
    assert!(!claimed_at.is_empty());
    let lease_expires_at = claimed_item["leaseExpiresAt"]
        .as_str()
        .expect("claimed pending inventory item should expose leaseExpiresAt after claim");
    assert!(lease_expires_at > claimed_at);

    let release_app = pending_app.clone();

    let foreign_release_response = release_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("foreign targeted pending release request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("foreign targeted pending release should return response");
    assert_eq!(foreign_release_response.status(), StatusCode::CONFLICT);
    let foreign_release_body = foreign_release_response
        .into_body()
        .collect()
        .await
        .expect("foreign targeted pending release body should collect")
        .to_bytes();
    let foreign_release_json: serde_json::Value = serde_json::from_slice(&foreign_release_body)
        .expect("foreign targeted pending release body should be valid json");
    assert_eq!(
        foreign_release_json["code"],
        "shared_channel_sync_owner_conflict"
    );
    assert_eq!(
        foreign_release_json["details"]["requestKey"],
        alice_request_key
    );
    assert_eq!(
        foreign_release_json["details"]["ownerActorId"],
        "u_operator_a"
    );
    assert_eq!(
        foreign_release_json["details"]["leaseExpiresAt"],
        lease_expires_at
    );
    assert_eq!(foreign_release_json["details"]["leaseStatus"], "active");
    assert_eq!(foreign_release_json["details"]["takeoverEligible"], false);
    assert_eq!(
        foreign_release_json["details"]["legacyTakeoverRequired"],
        false
    );
    assert_eq!(
        foreign_release_json["details"]["suggestedAction"],
        "wait_for_owner_release_or_expiry"
    );

    let owner_release_response = release_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("owner targeted pending release request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("owner targeted pending release should return response");
    assert_eq!(owner_release_response.status(), StatusCode::OK);
    let owner_release_body = owner_release_response
        .into_body()
        .collect()
        .await
        .expect("owner targeted pending release body should collect")
        .to_bytes();
    let owner_release_json: serde_json::Value = serde_json::from_slice(&owner_release_body)
        .expect("owner targeted pending release body should be valid json");
    assert_eq!(owner_release_json["status"], "released");
    assert_eq!(owner_release_json["pendingBefore"], 1);
    assert_eq!(owner_release_json["requested"], 1);
    assert_eq!(owner_release_json["released"], 1);
    assert_eq!(owner_release_json["conflicted"], 0);
    assert_eq!(owner_release_json["pendingAfter"], 1);

    let released_inventory_response = release_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("released pending shared-channel sync inventory should return response");
    assert_eq!(released_inventory_response.status(), StatusCode::OK);
    let released_inventory_body = released_inventory_response
        .into_body()
        .collect()
        .await
        .expect("released pending shared-channel sync inventory body should collect")
        .to_bytes();
    let released_inventory_json: serde_json::Value =
        serde_json::from_slice(&released_inventory_body)
            .expect("released pending shared-channel sync inventory body should be valid json");
    let released_item = released_inventory_json["items"]
        .as_array()
        .expect("released pending shared-channel sync inventory should serialize items as an array")
        .first()
        .expect("released pending inventory item should still exist");
    assert_eq!(released_item["leaseStatus"], "unclaimed");
    assert_eq!(released_item["takeoverEligible"], false);
    assert!(released_item["ownerActorId"].is_null());
    assert!(released_item["ownerActorKind"].is_null());
    assert!(released_item["claimedAt"].is_null());
    assert!(released_item["leaseExpiresAt"].is_null());

    let reclaim_response = release_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("reclaim pending claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("reclaim pending claim should return response");
    assert_eq!(reclaim_response.status(), StatusCode::OK);

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    let republish_app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        public_trigger,
    );

    let republish_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("owner targeted pending republish request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("reclaimed targeted pending republish should return response");
    assert_eq!(republish_response.status(), StatusCode::OK);

    let alice_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (alice_history_status, alice_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_pending_release_targeted_republish/messages",
        alice_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(alice_history_status, StatusCode::OK);
    assert_eq!(
        alice_history_json["items"][0]["message"]["body"]["summary"],
        "hello pending release targeted republish"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_pending_takeover_transfers_claim_to_foreign_operator()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_pending_takeover_targeted_republish");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_pending_takeover_targeted_republish",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_pending_takeover_targeted_republish"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_pending_takeover_targeted_republish/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_pending_takeover_targeted_republish_001",
            "summary":"hello pending takeover targeted republish",
            "text":"hello pending takeover targeted republish"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_pending_takeover_targeted_republish_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let pending_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster.clone(),
            ops_runtime.clone(),
            audit_runtime.clone(),
            runtime_dir.path(),
        );

    let establish_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_pending_takeover_targeted_republish_001",
                        "eventId":"evt_ec_pending_takeover_targeted_republish_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T03:44:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_pending_takeover_targeted_republish_001",
                        "eventId":"evt_scp_pending_takeover_targeted_republish_001",
                        "connectionId":"ec_pending_takeover_targeted_republish_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_pending_takeover_targeted_republish",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T03:45:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let alice_link_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_pending_takeover_targeted_republish_001",
                        "eventId":"evt_eml_pending_takeover_targeted_republish_001",
                        "connectionId":"ec_pending_takeover_targeted_republish_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T03:46:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("alice pending external member link write should return response");
    assert_eq!(alice_link_response.status(), StatusCode::OK);

    let pending_inventory_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pending shared-channel sync inventory should return response");
    assert_eq!(pending_inventory_response.status(), StatusCode::OK);
    let pending_inventory_body = pending_inventory_response
        .into_body()
        .collect()
        .await
        .expect("pending shared-channel sync inventory body should collect")
        .to_bytes();
    let pending_inventory_json: serde_json::Value = serde_json::from_slice(&pending_inventory_body)
        .expect("pending shared-channel sync inventory body should be valid json");
    let pending_item = pending_inventory_json["items"]
        .as_array()
        .expect("pending shared-channel sync inventory should serialize items as an array")
        .first()
        .expect("pending inventory item should exist");
    assert_eq!(pending_item["leaseStatus"], "unclaimed");
    assert_eq!(pending_item["legacyTakeoverRequired"], false);
    assert!(pending_item["claimedAt"].is_null());
    assert!(pending_item["leaseExpiresAt"].is_null());
    let alice_request_key = pending_item["requestKey"]
        .as_str()
        .expect("pending inventory item should expose a stable request key")
        .to_owned();

    let claim_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("targeted pending claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("targeted pending claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let claimed_inventory_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("claimed pending shared-channel sync inventory should return response");
    assert_eq!(claimed_inventory_response.status(), StatusCode::OK);
    let claimed_inventory_body = claimed_inventory_response
        .into_body()
        .collect()
        .await
        .expect("claimed pending shared-channel sync inventory body should collect")
        .to_bytes();
    let claimed_inventory_json: serde_json::Value = serde_json::from_slice(&claimed_inventory_body)
        .expect("claimed pending shared-channel sync inventory body should be valid json");
    let claimed_item = claimed_inventory_json["items"]
        .as_array()
        .expect("claimed pending shared-channel sync inventory should serialize items as an array")
        .first()
        .expect("claimed pending inventory item should still exist");
    assert_eq!(claimed_item["ownerActorId"], "u_operator_a");
    assert_eq!(claimed_item["leaseStatus"], "active");
    assert_eq!(claimed_item["takeoverEligible"], false);
    assert_eq!(claimed_item["legacyTakeoverRequired"], false);
    let first_claimed_at = claimed_item["claimedAt"]
        .as_str()
        .expect("claimed pending inventory item should expose claimedAt after claim")
        .to_owned();
    assert!(!first_claimed_at.is_empty());
    let first_lease_expires_at = claimed_item["leaseExpiresAt"]
        .as_str()
        .expect("claimed pending inventory item should expose leaseExpiresAt after claim")
        .to_owned();
    assert!(first_lease_expires_at > first_claimed_at);

    let active_takeover_response = pending_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("targeted pending takeover request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("targeted pending takeover should return response");
    assert_eq!(active_takeover_response.status(), StatusCode::CONFLICT);
    let active_takeover_body = active_takeover_response
        .into_body()
        .collect()
        .await
        .expect("active targeted pending takeover body should collect")
        .to_bytes();
    let active_takeover_json: serde_json::Value = serde_json::from_slice(&active_takeover_body)
        .expect("active targeted pending takeover body should be valid json");
    assert_eq!(
        active_takeover_json["code"],
        "shared_channel_sync_owner_conflict"
    );
    assert_eq!(
        active_takeover_json["details"]["requestKey"],
        alice_request_key
    );
    assert_eq!(active_takeover_json["details"]["leaseStatus"], "active");
    assert_eq!(active_takeover_json["details"]["takeoverEligible"], false);
    assert_eq!(
        active_takeover_json["details"]["legacyTakeoverRequired"],
        false
    );
    assert_eq!(
        active_takeover_json["details"]["suggestedAction"],
        "wait_for_owner_release_or_expiry"
    );

    let mut legacy_state = read_social_state_json(runtime_dir.path());
    legacy_state["pending_shared_channel_sync_requests"][alice_request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::Null;
    write_social_state_json(runtime_dir.path(), &legacy_state);

    let legacy_inventory_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster.clone(),
            ops_runtime.clone(),
            audit_runtime.clone(),
            runtime_dir.path(),
        );

    let legacy_inventory_response = legacy_inventory_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("legacy pending shared-channel sync inventory should return response");
    assert_eq!(legacy_inventory_response.status(), StatusCode::OK);
    let legacy_inventory_body = legacy_inventory_response
        .into_body()
        .collect()
        .await
        .expect("legacy pending shared-channel sync inventory body should collect")
        .to_bytes();
    let legacy_inventory_json: serde_json::Value = serde_json::from_slice(&legacy_inventory_body)
        .expect("legacy pending shared-channel sync inventory body should be valid json");
    let legacy_item = legacy_inventory_json["items"]
        .as_array()
        .expect("legacy pending shared-channel sync inventory should serialize items as an array")
        .first()
        .expect("legacy pending inventory item should still exist");
    assert_eq!(legacy_item["leaseStatus"], "untracked");
    assert_eq!(legacy_item["takeoverEligible"], false);
    assert_eq!(legacy_item["legacyTakeoverRequired"], true);

    let legacy_takeover_response = legacy_inventory_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("legacy targeted pending takeover request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("legacy targeted pending takeover should return response");
    assert_eq!(legacy_takeover_response.status(), StatusCode::CONFLICT);
    let legacy_takeover_body = legacy_takeover_response
        .into_body()
        .collect()
        .await
        .expect("legacy targeted pending takeover body should collect")
        .to_bytes();
    let legacy_takeover_json: serde_json::Value = serde_json::from_slice(&legacy_takeover_body)
        .expect("legacy targeted pending takeover body should be valid json");
    assert_eq!(
        legacy_takeover_json["code"],
        "shared_channel_sync_legacy_takeover_override_required"
    );
    assert_eq!(
        legacy_takeover_json["details"]["requestKey"],
        alice_request_key
    );
    assert_eq!(legacy_takeover_json["details"]["leaseStatus"], "untracked");
    assert_eq!(legacy_takeover_json["details"]["takeoverEligible"], false);
    assert_eq!(
        legacy_takeover_json["details"]["legacyTakeoverRequired"],
        true
    );
    assert_eq!(
        legacy_takeover_json["details"]["suggestedAction"],
        "takeover_with_legacy_override"
    );

    let mut expired_state = read_social_state_json(runtime_dir.path());
    expired_state["pending_shared_channel_sync_requests"][alice_request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &expired_state);

    tokio::time::sleep(std::time::Duration::from_millis(2)).await;

    let takeover_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster.clone(),
            ops_runtime.clone(),
            audit_runtime.clone(),
            runtime_dir.path(),
        );

    let stale_inventory_response = takeover_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale pending shared-channel sync inventory should return response");
    assert_eq!(stale_inventory_response.status(), StatusCode::OK);
    let stale_inventory_body = stale_inventory_response
        .into_body()
        .collect()
        .await
        .expect("stale pending shared-channel sync inventory body should collect")
        .to_bytes();
    let stale_inventory_json: serde_json::Value = serde_json::from_slice(&stale_inventory_body)
        .expect("stale pending shared-channel sync inventory body should be valid json");
    let stale_item = stale_inventory_json["items"]
        .as_array()
        .expect("stale pending shared-channel sync inventory should serialize items as an array")
        .first()
        .expect("stale pending inventory item should still exist");
    assert_eq!(stale_item["leaseStatus"], "stale");
    assert_eq!(stale_item["takeoverEligible"], true);
    assert_eq!(stale_item["legacyTakeoverRequired"], false);

    let stale_claim_conflict_response = takeover_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("stale targeted pending claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("stale targeted pending claim should return response");
    assert_eq!(stale_claim_conflict_response.status(), StatusCode::OK);
    let stale_claim_conflict_body = stale_claim_conflict_response
        .into_body()
        .collect()
        .await
        .expect("stale targeted pending claim body should collect")
        .to_bytes();
    let stale_claim_conflict_json: serde_json::Value =
        serde_json::from_slice(&stale_claim_conflict_body)
            .expect("stale targeted pending claim body should be valid json");
    assert_eq!(stale_claim_conflict_json["status"], "conflict");
    assert_eq!(stale_claim_conflict_json["claimed"], 0);
    assert_eq!(stale_claim_conflict_json["conflicted"], 1);
    let stale_conflict_items = stale_claim_conflict_json["conflictItems"]
        .as_array()
        .expect("stale targeted pending claim should expose conflictItems as an array");
    assert_eq!(stale_conflict_items.len(), 1);
    assert_eq!(stale_conflict_items[0]["requestKey"], alice_request_key);
    assert_eq!(stale_conflict_items[0]["leaseStatus"], "stale");
    assert_eq!(stale_conflict_items[0]["takeoverEligible"], true);
    assert_eq!(stale_conflict_items[0]["legacyTakeoverRequired"], false);
    assert_eq!(
        stale_conflict_items[0]["suggestedAction"],
        "takeover_pending_request"
    );

    let takeover_response = takeover_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("expired targeted pending takeover request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("expired targeted pending takeover should return response");
    assert_eq!(takeover_response.status(), StatusCode::OK);
    let takeover_body = takeover_response
        .into_body()
        .collect()
        .await
        .expect("expired targeted pending takeover body should collect")
        .to_bytes();
    let takeover_json: serde_json::Value = serde_json::from_slice(&takeover_body)
        .expect("expired targeted pending takeover body should be valid json");
    assert_eq!(takeover_json["status"], "taken_over");
    assert_eq!(takeover_json["pendingBefore"], 1);
    assert_eq!(takeover_json["requested"], 1);
    assert_eq!(takeover_json["takenOver"], 1);
    assert_eq!(takeover_json["pendingAfter"], 1);

    let taken_over_inventory_response = takeover_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("taken over pending shared-channel sync inventory should return response");
    assert_eq!(taken_over_inventory_response.status(), StatusCode::OK);
    let taken_over_inventory_body = taken_over_inventory_response
        .into_body()
        .collect()
        .await
        .expect("taken over pending shared-channel sync inventory body should collect")
        .to_bytes();
    let taken_over_inventory_json: serde_json::Value =
        serde_json::from_slice(&taken_over_inventory_body)
            .expect("taken over pending shared-channel sync inventory body should be valid json");
    let taken_over_item = taken_over_inventory_json["items"]
        .as_array()
        .expect(
            "taken over pending shared-channel sync inventory should serialize items as an array",
        )
        .first()
        .expect("taken over pending inventory item should still exist");
    assert_eq!(taken_over_item["ownerActorId"], "u_operator_b");
    assert_eq!(taken_over_item["ownerActorKind"], "user");
    assert_eq!(taken_over_item["leaseStatus"], "active");
    assert_eq!(taken_over_item["takeoverEligible"], false);
    assert_eq!(taken_over_item["legacyTakeoverRequired"], false);
    let taken_over_claimed_at = taken_over_item["claimedAt"]
        .as_str()
        .expect("taken over pending inventory item should expose claimedAt");
    assert_ne!(taken_over_claimed_at, first_claimed_at);
    let taken_over_lease_expires_at = taken_over_item["leaseExpiresAt"]
        .as_str()
        .expect("taken over pending inventory item should expose leaseExpiresAt");
    assert!(taken_over_lease_expires_at > taken_over_claimed_at);
    assert_ne!(taken_over_lease_expires_at, first_lease_expires_at);

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    let republish_app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        public_trigger,
    );

    let stale_owner_republish_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("stale owner targeted pending republish request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("stale owner targeted pending republish should return response");
    assert_eq!(
        stale_owner_republish_response.status(),
        StatusCode::CONFLICT
    );

    let republish_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("taken over targeted pending republish request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("taken over targeted pending republish should return response");
    assert_eq!(republish_response.status(), StatusCode::OK);

    let alice_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (alice_history_status, alice_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_pending_takeover_targeted_republish/messages",
        alice_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(alice_history_status, StatusCode::OK);
    assert_eq!(
        alice_history_json["items"][0]["message"]["body"]["summary"],
        "hello pending takeover targeted republish"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_pending_takeover_legacy_untracked_requires_explicit_override()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_pending_takeover_legacy_override");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_pending_takeover_legacy_override",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_pending_takeover_legacy_override"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_pending_takeover_legacy_override/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_pending_takeover_legacy_override_001",
            "summary":"hello pending takeover legacy override",
            "text":"hello pending takeover legacy override"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_pending_takeover_legacy_override_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_pending_takeover_legacy_override_001",
                        "eventId":"evt_ec_pending_takeover_legacy_override_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T04:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_pending_takeover_legacy_override_001",
                        "eventId":"evt_scp_pending_takeover_legacy_override_001",
                        "connectionId":"ec_pending_takeover_legacy_override_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_pending_takeover_legacy_override",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T04:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let alice_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_pending_takeover_legacy_override_001",
                        "eventId":"evt_eml_pending_takeover_legacy_override_001",
                        "connectionId":"ec_pending_takeover_legacy_override_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T04:12:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("alice pending external member link write should return response");
    assert_eq!(alice_link_response.status(), StatusCode::OK);

    let pending_inventory_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_reader")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pending shared-channel sync inventory should return response");
    assert_eq!(pending_inventory_response.status(), StatusCode::OK);
    let pending_inventory_body = pending_inventory_response
        .into_body()
        .collect()
        .await
        .expect("pending shared-channel sync inventory body should collect")
        .to_bytes();
    let pending_inventory_json: serde_json::Value = serde_json::from_slice(&pending_inventory_body)
        .expect("pending shared-channel sync inventory body should be valid json");
    let pending_item = pending_inventory_json["items"]
        .as_array()
        .expect("pending shared-channel sync inventory should serialize items as an array")
        .first()
        .expect("pending inventory item should exist");
    let alice_request_key = pending_item["requestKey"]
        .as_str()
        .expect("pending inventory item should expose a stable request key")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("targeted pending claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("targeted pending claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let mut legacy_state = read_social_state_json(runtime_dir.path());
    legacy_state["pending_shared_channel_sync_requests"][alice_request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::Null;
    write_social_state_json(runtime_dir.path(), &legacy_state);

    let legacy_app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
    );

    let legacy_inventory_response = legacy_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("legacy pending shared-channel sync inventory should return response");
    assert_eq!(legacy_inventory_response.status(), StatusCode::OK);
    let legacy_inventory_body = legacy_inventory_response
        .into_body()
        .collect()
        .await
        .expect("legacy pending shared-channel sync inventory body should collect")
        .to_bytes();
    let legacy_inventory_json: serde_json::Value = serde_json::from_slice(&legacy_inventory_body)
        .expect("legacy pending shared-channel sync inventory body should be valid json");
    let legacy_item = legacy_inventory_json["items"]
        .as_array()
        .expect("legacy pending shared-channel sync inventory should serialize items as an array")
        .first()
        .expect("legacy pending inventory item should still exist");
    assert_eq!(legacy_item["leaseStatus"], "untracked");
    assert_eq!(legacy_item["takeoverEligible"], false);
    assert_eq!(legacy_item["legacyTakeoverRequired"], true);

    let override_takeover_response = legacy_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key],
                        "allowLegacyUntracked": true
                    }))
                    .expect("legacy override targeted pending takeover request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("legacy override targeted pending takeover should return response");
    assert_eq!(override_takeover_response.status(), StatusCode::OK);
    let override_takeover_body = override_takeover_response
        .into_body()
        .collect()
        .await
        .expect("legacy override targeted pending takeover body should collect")
        .to_bytes();
    let override_takeover_json: serde_json::Value = serde_json::from_slice(&override_takeover_body)
        .expect("legacy override targeted pending takeover body should be valid json");
    assert_eq!(override_takeover_json["status"], "taken_over");
    assert_eq!(override_takeover_json["legacyOverrideUsed"], true);

    let taken_over_inventory_response = legacy_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_b")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("taken over pending shared-channel sync inventory should return response");
    assert_eq!(taken_over_inventory_response.status(), StatusCode::OK);
    let taken_over_inventory_body = taken_over_inventory_response
        .into_body()
        .collect()
        .await
        .expect("taken over pending shared-channel sync inventory body should collect")
        .to_bytes();
    let taken_over_inventory_json: serde_json::Value =
        serde_json::from_slice(&taken_over_inventory_body)
            .expect("taken over pending shared-channel sync inventory body should be valid json");
    let taken_over_item = taken_over_inventory_json["items"]
        .as_array()
        .expect(
            "taken over pending shared-channel sync inventory should serialize items as an array",
        )
        .first()
        .expect("taken over pending inventory item should still exist");
    assert_eq!(taken_over_item["ownerActorId"], "u_operator_b");
    assert_eq!(taken_over_item["leaseStatus"], "active");
    assert_eq!(taken_over_item["takeoverEligible"], false);
    assert_eq!(taken_over_item["legacyTakeoverRequired"], false);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_sync_failure_persists_pending_work_and_repair_replays_remote_runtime_materialization()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir = TestRuntimeDir::new("control_plane_shared_channel_sync_repair");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_repair",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(create_json["conversationId"], "c_partner_ops_repair");

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_repair/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_repair_001",
            "summary":"hello pending repair",
            "text":"hello pending repair"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(post_json["messageId"], "msg_c_partner_ops_repair_1");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during initial dispatch",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
        trigger.clone(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_repair_001",
                        "eventId":"evt_ec_repair_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T02:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_repair_001",
                        "eventId":"evt_scp_repair_001",
                        "connectionId":"ec_repair_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_repair",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T02:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_repair_001",
                        "eventId":"evt_eml_repair_001",
                        "connectionId":"ec_repair_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T02:02:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(link_response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let pending_state = read_social_state_json(runtime_dir.path());
    assert_eq!(
        pending_state["external_member_links"]["eml_repair_001"]["external_member_link"]["localActorId"],
        "actor_alice"
    );
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    trigger.set_delegate(public_trigger);

    let repair_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("shared-channel sync repair should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);
    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("shared-channel sync repair body should collect")
        .to_bytes();
    let repair_json: serde_json::Value =
        serde_json::from_slice(&repair_body).expect("repair body should be valid json");
    assert_eq!(repair_json["status"], "repaired");
    assert_eq!(repair_json["pendingBefore"], 1);
    assert_eq!(repair_json["attempted"], 1);
    assert_eq!(repair_json["dispatched"], 1);
    assert_eq!(repair_json["failed"], 0);
    assert_eq!(repair_json["pendingAfter"], 0);

    let repaired_state = read_social_state_json(runtime_dir.path());
    assert!(
        repaired_state["pending_shared_channel_sync_requests"]
            .as_object()
            .expect("pending shared-channel sync backlog should stay serialized as an object")
            .is_empty(),
        "repair should clear the pending shared-channel sync backlog"
    );

    let linked_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (history_status, history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_repair/messages",
        linked_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(history_status, StatusCode::OK);
    assert_eq!(
        history_json["items"][0]["message"]["body"]["summary"],
        "hello pending repair"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_repair_reclaims_stale_claim_before_dispatch() {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir = TestRuntimeDir::new("control_plane_shared_channel_stale_claim_repair");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_stale_claim_repair",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_stale_claim_repair"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_stale_claim_repair/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_stale_claim_repair_001",
            "summary":"hello stale claim repair",
            "text":"hello stale claim repair"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_stale_claim_repair_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during stale claim repair test",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
        trigger.clone(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_stale_claim_repair_001",
                        "eventId":"evt_ec_stale_claim_repair_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T04:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_stale_claim_repair_001",
                        "eventId":"evt_scp_stale_claim_repair_001",
                        "connectionId":"ec_stale_claim_repair_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_stale_claim_repair",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T04:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_stale_claim_repair_001",
                        "eventId":"evt_eml_stale_claim_repair_001",
                        "connectionId":"ec_stale_claim_repair_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T04:02:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(link_response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect("stale claim repair targeted claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("stale claim repair targeted claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let mut stale_state = read_social_state_json(runtime_dir.path());
    stale_state["pending_shared_channel_sync_requests"][request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &stale_state);

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    trigger.set_delegate(public_trigger);

    let repair_app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        trigger,
    );

    let stale_inventory_response = repair_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale claim repair inventory should return response");
    assert_eq!(stale_inventory_response.status(), StatusCode::OK);
    let stale_inventory_body = stale_inventory_response
        .into_body()
        .collect()
        .await
        .expect("stale claim repair inventory body should collect")
        .to_bytes();
    let stale_inventory_json: serde_json::Value = serde_json::from_slice(&stale_inventory_body)
        .expect("stale claim repair inventory body should be valid json");
    let stale_inventory_item = stale_inventory_json["items"]
        .as_array()
        .expect("stale claim repair inventory should serialize items as an array")
        .first()
        .expect("stale claim repair inventory item should exist");
    assert_eq!(stale_inventory_item["requestKey"], request_key);
    assert_eq!(stale_inventory_item["leaseStatus"], "stale");

    let repair_response = repair_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale claim repair should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);
    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("stale claim repair body should collect")
        .to_bytes();
    let repair_json: serde_json::Value =
        serde_json::from_slice(&repair_body).expect("stale claim repair body should be valid json");
    assert_eq!(repair_json["status"], "repaired");
    assert_eq!(repair_json["pendingBefore"], 1);
    assert_eq!(repair_json["attempted"], 1);
    assert_eq!(repair_json["dispatched"], 1);
    assert_eq!(repair_json["failed"], 0);
    assert_eq!(repair_json["reclaimed"], 1);
    assert_eq!(repair_json["pendingAfter"], 0);

    let repaired_state = read_social_state_json(runtime_dir.path());
    assert!(
        repaired_state["pending_shared_channel_sync_requests"]
            .as_object()
            .expect("pending shared-channel sync backlog should stay serialized as an object")
            .is_empty(),
        "stale-aware repair should clear the pending shared-channel sync backlog"
    );

    let linked_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (history_status, history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_stale_claim_repair/messages",
        linked_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(history_status, StatusCode::OK);
    assert_eq!(
        history_json["items"][0]["message"]["body"]["summary"],
        "hello stale claim repair"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_repair_reclaims_stale_claim_when_trigger_is_unconfigured()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_stale_claim_repair_unconfigured");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_stale_claim_repair_unconfigured",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_stale_claim_repair_unconfigured"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_stale_claim_repair_unconfigured/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_stale_claim_repair_unconfigured_001",
            "summary":"hello stale claim repair unconfigured",
            "text":"hello stale claim repair unconfigured"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_stale_claim_repair_unconfigured_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_stale_claim_repair_unconfigured_001",
                        "eventId":"evt_ec_stale_claim_repair_unconfigured_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-12T00:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_stale_claim_repair_unconfigured_001",
                        "eventId":"evt_scp_stale_claim_repair_unconfigured_001",
                        "connectionId":"ec_stale_claim_repair_unconfigured_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_stale_claim_repair_unconfigured",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-12T00:02:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_stale_claim_repair_unconfigured_001",
                        "eventId":"evt_eml_stale_claim_repair_unconfigured_001",
                        "connectionId":"ec_stale_claim_repair_unconfigured_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-12T00:03:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(link_response.status(), StatusCode::OK);

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect("stale claim repair unconfigured targeted claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("stale claim repair unconfigured targeted claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let mut stale_state = read_social_state_json(runtime_dir.path());
    stale_state["pending_shared_channel_sync_requests"][request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &stale_state);

    let repair_app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
    );

    let stale_inventory_response = repair_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale claim repair unconfigured inventory should return response");
    assert_eq!(stale_inventory_response.status(), StatusCode::OK);
    let stale_inventory_body = stale_inventory_response
        .into_body()
        .collect()
        .await
        .expect("stale claim repair unconfigured inventory body should collect")
        .to_bytes();
    let stale_inventory_json: serde_json::Value = serde_json::from_slice(&stale_inventory_body)
        .expect("stale claim repair unconfigured inventory body should be valid json");
    let stale_inventory_item = stale_inventory_json["items"]
        .as_array()
        .expect("stale claim repair unconfigured inventory should serialize items as an array")
        .first()
        .expect("stale claim repair unconfigured inventory item should exist");
    assert_eq!(stale_inventory_item["requestKey"], request_key);
    assert_eq!(stale_inventory_item["leaseStatus"], "stale");

    let repair_response = repair_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/repair-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale claim repair unconfigured should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);
    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("stale claim repair unconfigured body should collect")
        .to_bytes();
    let repair_json: serde_json::Value = serde_json::from_slice(&repair_body)
        .expect("stale claim repair unconfigured body should be valid json");
    assert_eq!(repair_json["status"], "trigger_unconfigured");
    assert_eq!(repair_json["pendingBefore"], 1);
    assert_eq!(repair_json["attempted"], 0);
    assert_eq!(repair_json["dispatched"], 0);
    assert_eq!(repair_json["failed"], 0);
    assert_eq!(repair_json["reclaimed"], 1);
    assert_eq!(repair_json["pendingAfter"], 1);
    assert_eq!(repair_json["deadLetterAfter"], 0);

    let reclaimed_state = read_social_state_json(runtime_dir.path());
    let reclaimed_pending = reclaimed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should stay serialized as an object")
        .get(request_key.as_str())
        .expect("reclaimed pending shared-channel sync request should still exist");
    assert!(reclaimed_pending["ownerActorId"].is_null());
    assert!(reclaimed_pending["ownerActorKind"].is_null());
    assert!(reclaimed_pending["claimedAt"].is_null());
    assert!(
        reclaimed_pending["leaseExpiresAt"].is_null(),
        "repair should reclaim stale owner metadata even when dispatch is unconfigured"
    );

    let reclaimed_inventory_response = repair_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("reclaimed pending inventory should return response");
    assert_eq!(reclaimed_inventory_response.status(), StatusCode::OK);
    let reclaimed_inventory_body = reclaimed_inventory_response
        .into_body()
        .collect()
        .await
        .expect("reclaimed pending inventory body should collect")
        .to_bytes();
    let reclaimed_inventory_json: serde_json::Value =
        serde_json::from_slice(&reclaimed_inventory_body)
            .expect("reclaimed pending inventory body should be valid json");
    let reclaimed_inventory_item = reclaimed_inventory_json["items"]
        .as_array()
        .expect("reclaimed pending inventory should serialize items as an array")
        .first()
        .expect("reclaimed pending inventory item should exist");
    assert_eq!(reclaimed_inventory_item["requestKey"], request_key);
    assert_eq!(reclaimed_inventory_item["leaseStatus"], "unclaimed");
    assert_eq!(reclaimed_inventory_item["takeoverEligible"], false);
    assert_eq!(reclaimed_inventory_item["legacyTakeoverRequired"], false);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_stale_claim_scheduler_reclaims_without_manual_repair()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_stale_claim_scheduler_reclaim");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_stale_claim_scheduler_reclaim",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_stale_claim_scheduler_reclaim"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_stale_claim_scheduler_reclaim/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_stale_claim_scheduler_reclaim_001",
            "summary":"hello stale claim scheduler reclaim",
            "text":"hello stale claim scheduler reclaim"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_stale_claim_scheduler_reclaim_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let seed_app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
    );

    let establish_response = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_stale_claim_scheduler_reclaim_001",
                        "eventId":"evt_ec_stale_claim_scheduler_reclaim_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-12T00:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_stale_claim_scheduler_reclaim_001",
                        "eventId":"evt_scp_stale_claim_scheduler_reclaim_001",
                        "connectionId":"ec_stale_claim_scheduler_reclaim_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_stale_claim_scheduler_reclaim",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-12T00:02:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let link_response = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_stale_claim_scheduler_reclaim_001",
                        "eventId":"evt_eml_stale_claim_scheduler_reclaim_001",
                        "connectionId":"ec_stale_claim_scheduler_reclaim_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-12T00:03:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(link_response.status(), StatusCode::OK);

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect("stale claim scheduler reclaim targeted claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("stale claim scheduler reclaim targeted claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let mut stale_state = read_social_state_json(runtime_dir.path());
    stale_state["pending_shared_channel_sync_requests"][request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &stale_state);

    let scheduler_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir_with_shared_channel_sync_stale_reclaim_scheduler_config(
            cluster,
            ops_runtime,
            audit_runtime,
            runtime_dir.path(),
            control_plane_api::SharedChannelSyncStaleReclaimSchedulerConfig {
                enabled: true,
                interval_millis: 20,
                jitter_millis: 0,
            },
        );

    let reclaimed_state = wait_for_pending_shared_channel_sync_reclaim(
        runtime_dir.path(),
        request_key.as_str(),
        Duration::from_secs(2),
    )
    .await;
    let reclaimed_pending = reclaimed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should stay serialized as an object")
        .get(request_key.as_str())
        .expect("reclaimed pending shared-channel sync request should still exist");
    assert!(reclaimed_pending["ownerActorId"].is_null());
    assert!(reclaimed_pending["ownerActorKind"].is_null());
    assert!(reclaimed_pending["claimedAt"].is_null());
    assert!(reclaimed_pending["leaseExpiresAt"].is_null());

    let inventory_response = scheduler_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("scheduler reclaim inventory should return response");
    assert_eq!(inventory_response.status(), StatusCode::OK);
    let inventory_body = inventory_response
        .into_body()
        .collect()
        .await
        .expect("scheduler reclaim inventory body should collect")
        .to_bytes();
    let inventory_json: serde_json::Value = serde_json::from_slice(&inventory_body)
        .expect("scheduler reclaim inventory body should be valid json");
    let inventory_item = inventory_json["items"]
        .as_array()
        .expect("scheduler reclaim inventory should serialize items as an array")
        .first()
        .expect("scheduler reclaim inventory item should exist");
    assert_eq!(inventory_item["requestKey"], request_key);
    assert_eq!(inventory_item["leaseStatus"], "unclaimed");

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_pending_claim_renews_stale_lease_for_same_operator()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_same_owner_stale_claim_renewal");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_same_owner_stale_claim_renewal",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_same_owner_stale_claim_renewal"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_same_owner_stale_claim_renewal/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_same_owner_stale_claim_renewal_001",
            "summary":"hello same owner stale claim renewal",
            "text":"hello same owner stale claim renewal"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_same_owner_stale_claim_renewal_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_same_owner_stale_claim_renewal_001",
                        "eventId":"evt_ec_same_owner_stale_claim_renewal_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-12T00:21:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_same_owner_stale_claim_renewal_001",
                        "eventId":"evt_scp_same_owner_stale_claim_renewal_001",
                        "connectionId":"ec_same_owner_stale_claim_renewal_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_same_owner_stale_claim_renewal",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-12T00:22:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_same_owner_stale_claim_renewal_001",
                        "eventId":"evt_eml_same_owner_stale_claim_renewal_001",
                        "connectionId":"ec_same_owner_stale_claim_renewal_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-12T00:23:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(link_response.status(), StatusCode::OK);

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect("same-owner stale lease targeted claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("initial targeted pending claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);
    let claim_body = claim_response
        .into_body()
        .collect()
        .await
        .expect("initial targeted pending claim body should collect")
        .to_bytes();
    let claim_json: serde_json::Value = serde_json::from_slice(&claim_body)
        .expect("initial targeted pending claim body should be valid json");
    assert_eq!(claim_json["status"], "claimed");
    assert_eq!(claim_json["claimed"], 1);
    assert_eq!(claim_json["conflicted"], 0);

    let claimed_state = read_social_state_json(runtime_dir.path());
    let claimed_request = claimed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("claimed shared-channel sync backlog should stay serialized as an object")
        .get(request_key.as_str())
        .expect("claimed shared-channel sync request should exist");
    let first_claimed_at = claimed_request["claimedAt"]
        .as_str()
        .expect("claimed shared-channel sync request should expose claimedAt")
        .to_owned();
    let first_lease_expires_at = claimed_request["leaseExpiresAt"]
        .as_str()
        .expect("claimed shared-channel sync request should expose leaseExpiresAt")
        .to_owned();
    assert!(first_lease_expires_at > first_claimed_at);

    let mut stale_state = claimed_state;
    stale_state["pending_shared_channel_sync_requests"][request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &stale_state);

    tokio::time::sleep(std::time::Duration::from_millis(5)).await;

    let renewed_claim_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster,
            ops_runtime,
            audit_runtime,
            runtime_dir.path(),
        );

    let stale_inventory_response = renewed_claim_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("same-owner stale lease inventory should return response");
    assert_eq!(stale_inventory_response.status(), StatusCode::OK);
    let stale_inventory_body = stale_inventory_response
        .into_body()
        .collect()
        .await
        .expect("same-owner stale lease inventory body should collect")
        .to_bytes();
    let stale_inventory_json: serde_json::Value = serde_json::from_slice(&stale_inventory_body)
        .expect("same-owner stale lease inventory body should be valid json");
    let stale_inventory_item = stale_inventory_json["items"]
        .as_array()
        .expect("same-owner stale lease inventory should serialize items as an array")
        .first()
        .expect("same-owner stale lease inventory item should exist");
    assert_eq!(stale_inventory_item["requestKey"], request_key);
    assert_eq!(stale_inventory_item["leaseStatus"], "stale");

    let renewed_claim_response = renewed_claim_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect("same-owner stale lease renewal request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("same-owner stale lease renewal should return response");
    assert_eq!(renewed_claim_response.status(), StatusCode::OK);
    let renewed_claim_body = renewed_claim_response
        .into_body()
        .collect()
        .await
        .expect("same-owner stale lease renewal body should collect")
        .to_bytes();
    let renewed_claim_json: serde_json::Value = serde_json::from_slice(&renewed_claim_body)
        .expect("same-owner stale lease renewal body should be valid json");
    assert_eq!(renewed_claim_json["status"], "claimed");
    assert_eq!(renewed_claim_json["claimed"], 1);
    assert_eq!(renewed_claim_json["conflicted"], 0);

    let renewed_state = read_social_state_json(runtime_dir.path());
    let renewed_request = renewed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("renewed shared-channel sync backlog should stay serialized as an object")
        .get(request_key.as_str())
        .expect("renewed shared-channel sync request should exist");
    let renewed_claimed_at = renewed_request["claimedAt"]
        .as_str()
        .expect("renewed shared-channel sync request should expose claimedAt");
    let renewed_lease_expires_at = renewed_request["leaseExpiresAt"]
        .as_str()
        .expect("renewed shared-channel sync request should expose leaseExpiresAt");
    assert_eq!(renewed_request["ownerActorId"], "u_operator_a");
    assert_eq!(renewed_request["ownerActorKind"], "user");
    assert_ne!(
        renewed_claimed_at, first_claimed_at,
        "same-owner stale claim should refresh claimedAt"
    );
    assert_ne!(
        renewed_lease_expires_at, first_lease_expires_at,
        "same-owner stale claim should refresh leaseExpiresAt"
    );
    assert_ne!(
        renewed_lease_expires_at, "1970-01-01T00:00:00.000Z",
        "same-owner stale claim should not preserve expired lease metadata"
    );
    assert!(renewed_lease_expires_at > renewed_claimed_at);

    let renewed_inventory_response = renewed_claim_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("same-owner renewed inventory should return response");
    assert_eq!(renewed_inventory_response.status(), StatusCode::OK);
    let renewed_inventory_body = renewed_inventory_response
        .into_body()
        .collect()
        .await
        .expect("same-owner renewed inventory body should collect")
        .to_bytes();
    let renewed_inventory_json: serde_json::Value = serde_json::from_slice(&renewed_inventory_body)
        .expect("same-owner renewed inventory body should be valid json");
    let renewed_inventory_item = renewed_inventory_json["items"]
        .as_array()
        .expect("same-owner renewed inventory should serialize items as an array")
        .first()
        .expect("same-owner renewed inventory item should exist");
    assert_eq!(renewed_inventory_item["requestKey"], request_key);
    assert_eq!(renewed_inventory_item["ownerActorId"], "u_operator_a");
    assert_eq!(renewed_inventory_item["ownerActorKind"], "user");
    assert_eq!(renewed_inventory_item["leaseStatus"], "active");
    assert_eq!(renewed_inventory_item["takeoverEligible"], false);
    assert_eq!(renewed_inventory_item["legacyTakeoverRequired"], false);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_pending_republish_renews_stale_lease_for_same_operator_on_failure()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_same_owner_stale_republish_renewal");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_same_owner_stale_republish_renewal",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_same_owner_stale_republish_renewal"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_same_owner_stale_republish_renewal/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_same_owner_stale_republish_renewal_001",
            "summary":"hello same owner stale republish renewal",
            "text":"hello same owner stale republish renewal"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_same_owner_stale_republish_renewal_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during same-owner stale republish renewal test",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
        trigger.clone(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_same_owner_stale_republish_renewal_001",
                        "eventId":"evt_ec_same_owner_stale_republish_renewal_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-12T00:31:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_same_owner_stale_republish_renewal_001",
                        "eventId":"evt_scp_same_owner_stale_republish_renewal_001",
                        "connectionId":"ec_same_owner_stale_republish_renewal_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_same_owner_stale_republish_renewal",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-12T00:32:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_same_owner_stale_republish_renewal_001",
                        "eventId":"evt_eml_same_owner_stale_republish_renewal_001",
                        "connectionId":"ec_same_owner_stale_republish_renewal_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-12T00:33:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(link_response.status(), StatusCode::SERVICE_UNAVAILABLE);

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect("same-owner stale republish targeted claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("same-owner stale republish targeted claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let claimed_state = read_social_state_json(runtime_dir.path());
    let claimed_request = claimed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("claimed shared-channel sync backlog should stay serialized as an object")
        .get(request_key.as_str())
        .expect("claimed shared-channel sync request should exist");
    let first_claimed_at = claimed_request["claimedAt"]
        .as_str()
        .expect("claimed shared-channel sync request should expose claimedAt")
        .to_owned();
    let first_lease_expires_at = claimed_request["leaseExpiresAt"]
        .as_str()
        .expect("claimed shared-channel sync request should expose leaseExpiresAt")
        .to_owned();
    assert!(first_lease_expires_at > first_claimed_at);

    let mut stale_state = claimed_state;
    stale_state["pending_shared_channel_sync_requests"][request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &stale_state);

    tokio::time::sleep(std::time::Duration::from_millis(5)).await;

    let republish_app =
        control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
            cluster,
            ops_runtime,
            audit_runtime,
            runtime_dir.path(),
            trigger,
        );

    let stale_inventory_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("same-owner stale republish inventory should return response");
    assert_eq!(stale_inventory_response.status(), StatusCode::OK);
    let stale_inventory_body = stale_inventory_response
        .into_body()
        .collect()
        .await
        .expect("same-owner stale republish inventory body should collect")
        .to_bytes();
    let stale_inventory_json: serde_json::Value = serde_json::from_slice(&stale_inventory_body)
        .expect("same-owner stale republish inventory body should be valid json");
    let stale_inventory_item = stale_inventory_json["items"]
        .as_array()
        .expect("same-owner stale republish inventory should serialize items as an array")
        .first()
        .expect("same-owner stale republish inventory item should exist");
    assert_eq!(stale_inventory_item["requestKey"], request_key);
    assert_eq!(stale_inventory_item["leaseStatus"], "stale");

    let republish_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect("same-owner stale republish renewal request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("same-owner stale republish should return response");
    assert_eq!(republish_response.status(), StatusCode::OK);
    let republish_body = republish_response
        .into_body()
        .collect()
        .await
        .expect("same-owner stale republish body should collect")
        .to_bytes();
    let republish_json: serde_json::Value = serde_json::from_slice(&republish_body)
        .expect("same-owner stale republish body should be valid json");
    assert_eq!(republish_json["status"], "pending");
    assert_eq!(republish_json["requested"], 1);
    assert_eq!(republish_json["attempted"], 1);
    assert_eq!(republish_json["dispatched"], 0);
    assert_eq!(republish_json["failed"], 1);
    assert_eq!(republish_json["pendingAfter"], 1);
    assert_eq!(republish_json["deadLettered"], 0);
    assert_eq!(republish_json["deadLetterAfter"], 0);

    let renewed_state = read_social_state_json(runtime_dir.path());
    let renewed_request = renewed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("renewed shared-channel sync backlog should stay serialized as an object")
        .get(request_key.as_str())
        .expect("renewed shared-channel sync request should exist");
    let renewed_claimed_at = renewed_request["claimedAt"]
        .as_str()
        .expect("renewed shared-channel sync request should expose claimedAt");
    let renewed_lease_expires_at = renewed_request["leaseExpiresAt"]
        .as_str()
        .expect("renewed shared-channel sync request should expose leaseExpiresAt");
    assert_eq!(renewed_request["ownerActorId"], "u_operator_a");
    assert_eq!(renewed_request["ownerActorKind"], "user");
    assert_eq!(renewed_request["failureCount"], 2);
    assert_ne!(
        renewed_claimed_at, first_claimed_at,
        "same-owner stale republish should refresh claimedAt before a failed dispatch is persisted"
    );
    assert_ne!(
        renewed_lease_expires_at, first_lease_expires_at,
        "same-owner stale republish should refresh leaseExpiresAt before a failed dispatch is persisted"
    );
    assert_ne!(
        renewed_lease_expires_at, "1970-01-01T00:00:00.000Z",
        "same-owner stale republish should not preserve expired lease metadata after failure"
    );
    assert!(renewed_lease_expires_at > renewed_claimed_at);

    let renewed_inventory_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("same-owner renewed republish inventory should return response");
    assert_eq!(renewed_inventory_response.status(), StatusCode::OK);
    let renewed_inventory_body = renewed_inventory_response
        .into_body()
        .collect()
        .await
        .expect("same-owner renewed republish inventory body should collect")
        .to_bytes();
    let renewed_inventory_json: serde_json::Value = serde_json::from_slice(&renewed_inventory_body)
        .expect("same-owner renewed republish inventory body should be valid json");
    let renewed_inventory_item = renewed_inventory_json["items"]
        .as_array()
        .expect("same-owner renewed republish inventory should serialize items as an array")
        .first()
        .expect("same-owner renewed republish inventory item should exist");
    assert_eq!(renewed_inventory_item["requestKey"], request_key);
    assert_eq!(renewed_inventory_item["ownerActorId"], "u_operator_a");
    assert_eq!(renewed_inventory_item["ownerActorKind"], "user");
    assert_eq!(renewed_inventory_item["leaseStatus"], "active");
    assert_eq!(renewed_inventory_item["takeoverEligible"], false);
    assert_eq!(renewed_inventory_item["legacyTakeoverRequired"], false);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_pending_republish_renews_stale_lease_for_same_operator_when_trigger_unconfigured()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir = TestRuntimeDir::new(
        "control_plane_shared_channel_same_owner_stale_republish_unconfigured_renewal",
    );
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_same_owner_stale_republish_unconfigured_renewal",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_same_owner_stale_republish_unconfigured_renewal"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_same_owner_stale_republish_unconfigured_renewal/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_same_owner_stale_republish_unconfigured_renewal_001",
            "summary":"hello same owner stale republish unconfigured renewal",
            "text":"hello same owner stale republish unconfigured renewal"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_same_owner_stale_republish_unconfigured_renewal_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_same_owner_stale_republish_unconfigured_renewal_001",
                        "eventId":"evt_ec_same_owner_stale_republish_unconfigured_renewal_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-12T00:34:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_same_owner_stale_republish_unconfigured_renewal_001",
                        "eventId":"evt_scp_same_owner_stale_republish_unconfigured_renewal_001",
                        "connectionId":"ec_same_owner_stale_republish_unconfigured_renewal_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_same_owner_stale_republish_unconfigured_renewal",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-12T00:35:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_same_owner_stale_republish_unconfigured_renewal_001",
                        "eventId":"evt_eml_same_owner_stale_republish_unconfigured_renewal_001",
                        "connectionId":"ec_same_owner_stale_republish_unconfigured_renewal_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-12T00:36:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(link_response.status(), StatusCode::OK);

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect(
                        "same-owner stale republish unconfigured targeted claim request should encode",
                    ),
                ))
                .unwrap(),
        )
        .await
        .expect("same-owner stale republish unconfigured targeted claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let claimed_state = read_social_state_json(runtime_dir.path());
    let claimed_request = claimed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("claimed shared-channel sync backlog should stay serialized as an object")
        .get(request_key.as_str())
        .expect("claimed shared-channel sync request should exist");
    let first_claimed_at = claimed_request["claimedAt"]
        .as_str()
        .expect("claimed shared-channel sync request should expose claimedAt")
        .to_owned();
    let first_lease_expires_at = claimed_request["leaseExpiresAt"]
        .as_str()
        .expect("claimed shared-channel sync request should expose leaseExpiresAt")
        .to_owned();
    assert!(first_lease_expires_at > first_claimed_at);

    let mut stale_state = claimed_state;
    stale_state["pending_shared_channel_sync_requests"][request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &stale_state);

    tokio::time::sleep(std::time::Duration::from_millis(5)).await;

    let republish_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster,
            ops_runtime,
            audit_runtime,
            runtime_dir.path(),
        );

    let stale_inventory_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("same-owner stale republish unconfigured inventory should return response");
    assert_eq!(stale_inventory_response.status(), StatusCode::OK);
    let stale_inventory_body = stale_inventory_response
        .into_body()
        .collect()
        .await
        .expect("same-owner stale republish unconfigured inventory body should collect")
        .to_bytes();
    let stale_inventory_json: serde_json::Value = serde_json::from_slice(&stale_inventory_body)
        .expect("same-owner stale republish unconfigured inventory body should be valid json");
    let stale_inventory_item = stale_inventory_json["items"]
        .as_array()
        .expect(
            "same-owner stale republish unconfigured inventory should serialize items as an array",
        )
        .first()
        .expect("same-owner stale republish unconfigured inventory item should exist");
    assert_eq!(stale_inventory_item["requestKey"], request_key);
    assert_eq!(stale_inventory_item["leaseStatus"], "stale");

    let republish_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect(
                        "same-owner stale republish unconfigured renewal request should encode",
                    ),
                ))
                .unwrap(),
        )
        .await
        .expect("same-owner stale republish unconfigured should return response");
    assert_eq!(republish_response.status(), StatusCode::OK);
    let republish_body = republish_response
        .into_body()
        .collect()
        .await
        .expect("same-owner stale republish unconfigured body should collect")
        .to_bytes();
    let republish_json: serde_json::Value = serde_json::from_slice(&republish_body)
        .expect("same-owner stale republish unconfigured body should be valid json");
    assert_eq!(republish_json["status"], "trigger_unconfigured");
    assert_eq!(republish_json["requested"], 1);
    assert_eq!(republish_json["attempted"], 1);
    assert_eq!(republish_json["dispatched"], 0);
    assert_eq!(republish_json["failed"], 0);
    assert_eq!(republish_json["pendingAfter"], 1);
    assert_eq!(republish_json["deadLettered"], 0);
    assert_eq!(republish_json["deadLetterAfter"], 0);

    let renewed_state = read_social_state_json(runtime_dir.path());
    let renewed_request = renewed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("renewed shared-channel sync backlog should stay serialized as an object")
        .get(request_key.as_str())
        .expect("renewed shared-channel sync request should exist");
    let renewed_claimed_at = renewed_request["claimedAt"]
        .as_str()
        .expect("renewed shared-channel sync request should expose claimedAt");
    let renewed_lease_expires_at = renewed_request["leaseExpiresAt"]
        .as_str()
        .expect("renewed shared-channel sync request should expose leaseExpiresAt");
    assert_eq!(renewed_request["ownerActorId"], "u_operator_a");
    assert_eq!(renewed_request["ownerActorKind"], "user");
    assert_ne!(
        renewed_claimed_at, first_claimed_at,
        "same-owner stale republish should refresh claimedAt before returning trigger_unconfigured"
    );
    assert_ne!(
        renewed_lease_expires_at, first_lease_expires_at,
        "same-owner stale republish should refresh leaseExpiresAt before returning trigger_unconfigured"
    );
    assert_ne!(
        renewed_lease_expires_at, "1970-01-01T00:00:00.000Z",
        "same-owner stale republish should not preserve expired lease metadata when trigger is unconfigured"
    );
    assert!(renewed_lease_expires_at > renewed_claimed_at);

    let renewed_inventory_response = republish_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("same-owner renewed unconfigured republish inventory should return response");
    assert_eq!(renewed_inventory_response.status(), StatusCode::OK);
    let renewed_inventory_body = renewed_inventory_response
        .into_body()
        .collect()
        .await
        .expect("same-owner renewed unconfigured republish inventory body should collect")
        .to_bytes();
    let renewed_inventory_json: serde_json::Value = serde_json::from_slice(&renewed_inventory_body)
        .expect("same-owner renewed unconfigured republish inventory body should be valid json");
    let renewed_inventory_item = renewed_inventory_json["items"]
        .as_array()
        .expect(
            "same-owner renewed unconfigured republish inventory should serialize items as an array",
        )
        .first()
        .expect("same-owner renewed unconfigured republish inventory item should exist");
    assert_eq!(renewed_inventory_item["requestKey"], request_key);
    assert_eq!(renewed_inventory_item["ownerActorId"], "u_operator_a");
    assert_eq!(renewed_inventory_item["ownerActorKind"], "user");
    assert_eq!(renewed_inventory_item["leaseStatus"], "active");
    assert_eq!(renewed_inventory_item["takeoverEligible"], false);
    assert_eq!(renewed_inventory_item["legacyTakeoverRequired"], false);

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_stale_claim_reclaim_surface_clears_owner_metadata()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_stale_claim_reclaim_surface");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_stale_claim_reclaim_surface",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_stale_claim_reclaim_surface"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_stale_claim_reclaim_surface/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_stale_claim_reclaim_surface_001",
            "summary":"hello stale claim reclaim surface",
            "text":"hello stale claim reclaim surface"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_stale_claim_reclaim_surface_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during stale reclaim surface test",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
        trigger,
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_stale_claim_reclaim_surface_001",
                        "eventId":"evt_ec_stale_claim_reclaim_surface_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T06:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_stale_claim_reclaim_surface_001",
                        "eventId":"evt_scp_stale_claim_reclaim_surface_001",
                        "connectionId":"ec_stale_claim_reclaim_surface_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_stale_claim_reclaim_surface",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T06:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let initial_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_stale_claim_reclaim_surface_001",
                        "eventId":"evt_eml_stale_claim_reclaim_surface_001",
                        "connectionId":"ec_stale_claim_reclaim_surface_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T06:12:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("initial external member link write should return response");
    assert_eq!(
        initial_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[request_key]
                    }))
                    .expect("stale reclaim surface targeted claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("stale reclaim surface targeted claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let mut stale_state = read_social_state_json(runtime_dir.path());
    stale_state["pending_shared_channel_sync_requests"][request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &stale_state);

    let reclaim_app =
        control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
            cluster,
            ops_runtime,
            audit_runtime,
            runtime_dir.path(),
        );

    let reclaim_response = reclaim_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale reclaim surface should return response");
    assert_eq!(reclaim_response.status(), StatusCode::OK);
    let reclaim_body = reclaim_response
        .into_body()
        .collect()
        .await
        .expect("stale reclaim surface body should collect")
        .to_bytes();
    let reclaim_json: serde_json::Value = serde_json::from_slice(&reclaim_body)
        .expect("stale reclaim surface body should be valid json");
    assert_eq!(reclaim_json["status"], "reclaimed");
    assert_eq!(reclaim_json["pendingBefore"], 1);
    assert_eq!(reclaim_json["reclaimed"], 1);
    assert_eq!(reclaim_json["pendingAfter"], 1);

    let inventory_response = reclaim_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale reclaim surface inventory should return response");
    assert_eq!(inventory_response.status(), StatusCode::OK);
    let inventory_body = inventory_response
        .into_body()
        .collect()
        .await
        .expect("stale reclaim surface inventory body should collect")
        .to_bytes();
    let inventory_json: serde_json::Value = serde_json::from_slice(&inventory_body)
        .expect("stale reclaim surface inventory body should be valid json");
    let inventory_item = inventory_json["items"]
        .as_array()
        .expect("stale reclaim surface inventory should serialize items as an array")
        .first()
        .expect("stale reclaim surface inventory item should exist");
    assert_eq!(inventory_item["requestKey"], request_key);
    assert!(inventory_item["ownerActorId"].is_null());
    assert!(inventory_item["ownerActorKind"].is_null());
    assert!(inventory_item["claimedAt"].is_null());
    assert!(inventory_item["leaseExpiresAt"].is_null());
    assert_eq!(inventory_item["leaseStatus"], "unclaimed");
    assert_eq!(inventory_item["takeoverEligible"], false);
    assert_eq!(inventory_item["legacyTakeoverRequired"], false);

    let reclaimed_state = read_social_state_json(runtime_dir.path());
    let reclaimed_pending = reclaimed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should remain serialized as an object")
        .get(request_key.as_str())
        .expect("reclaimed pending shared-channel sync request should still exist");
    assert!(reclaimed_pending["ownerActorId"].is_null());
    assert!(reclaimed_pending["ownerActorKind"].is_null());
    assert!(reclaimed_pending["claimedAt"].is_null());
    assert!(reclaimed_pending["leaseExpiresAt"].is_null());

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_pending_backlog_retries_on_next_healthy_ready_pair_write()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir = TestRuntimeDir::new("control_plane_shared_channel_auto_retry");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_auto_retry",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(create_json["conversationId"], "c_partner_ops_auto_retry");

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_auto_retry/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_auto_retry_001",
            "summary":"hello pending auto retry",
            "text":"hello pending auto retry"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(post_json["messageId"], "msg_c_partner_ops_auto_retry_1");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during initial dispatch",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        trigger.clone(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_auto_retry_001",
                        "eventId":"evt_ec_auto_retry_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T02:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_auto_retry_001",
                        "eventId":"evt_scp_auto_retry_001",
                        "connectionId":"ec_auto_retry_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_auto_retry",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T02:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let pending_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_auto_retry_001",
                        "eventId":"evt_eml_auto_retry_001",
                        "connectionId":"ec_auto_retry_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T02:12:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("initial external member link write should return response");
    assert_eq!(
        pending_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    trigger.set_delegate(public_trigger);

    let healthy_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_auto_retry_002",
                        "eventId":"evt_eml_auto_retry_002",
                        "connectionId":"ec_auto_retry_001",
                        "localActorId":"actor_bob",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-11T02:13:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("healthy external member link write should return response");
    assert_eq!(healthy_link_response.status(), StatusCode::OK);

    let flushed_state = read_social_state_json(runtime_dir.path());
    assert!(
        flushed_state["pending_shared_channel_sync_requests"]
            .as_object()
            .expect("pending shared-channel sync backlog should stay serialized as an object")
            .is_empty(),
        "next healthy ready-pair write should flush the pending shared-channel sync backlog"
    );

    let alice_bearer = bearer_token("t_demo", "actor_alice", "user", &[]);
    let (alice_history_status, alice_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_auto_retry/messages",
        alice_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(alice_history_status, StatusCode::OK);
    assert_eq!(
        alice_history_json["items"][0]["message"]["body"]["summary"],
        "hello pending auto retry"
    );

    let bob_bearer = bearer_token("t_demo", "actor_bob", "user", &[]);
    let (bob_history_status, bob_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_auto_retry/messages",
        bob_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(bob_history_status, StatusCode::OK);
    assert_eq!(
        bob_history_json["items"][0]["message"]["body"]["summary"],
        "hello pending auto retry"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_next_healthy_ready_pair_write_respects_active_pending_claim_ownership()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir = TestRuntimeDir::new("control_plane_shared_channel_claim_blocked_auto_retry");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_claim_blocked_auto_retry",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_claim_blocked_auto_retry"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_claim_blocked_auto_retry/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_claim_blocked_auto_retry_001",
            "summary":"hello claim blocked auto retry",
            "text":"hello claim blocked auto retry"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_claim_blocked_auto_retry_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during initial dispatch",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        trigger.clone(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_claim_blocked_auto_retry_001",
                        "eventId":"evt_ec_claim_blocked_auto_retry_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T02:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_claim_blocked_auto_retry_001",
                        "eventId":"evt_scp_claim_blocked_auto_retry_001",
                        "connectionId":"ec_claim_blocked_auto_retry_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_claim_blocked_auto_retry",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T02:21:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let pending_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_claim_blocked_auto_retry_001",
                        "eventId":"evt_eml_claim_blocked_auto_retry_001",
                        "connectionId":"ec_claim_blocked_auto_retry_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T02:22:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("initial external member link write should return response");
    assert_eq!(
        pending_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let alice_request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("claim blocked auto retry targeted claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("claim blocked auto retry targeted claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let public_trigger = control_plane_api::build_public_shared_channel_sync_trigger(
        runtime_base_url.as_str(),
        TEST_PUBLIC_SECRET,
    )
    .expect("public shared-channel trigger should build");
    trigger.set_delegate(public_trigger);

    let healthy_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_claim_blocked_auto_retry_002",
                        "eventId":"evt_eml_claim_blocked_auto_retry_002",
                        "connectionId":"ec_claim_blocked_auto_retry_001",
                        "localActorId":"actor_bob",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-11T02:23:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("healthy external member link write should return response");
    assert_eq!(healthy_link_response.status(), StatusCode::OK);

    let flushed_state = read_social_state_json(runtime_dir.path());
    let remaining_pending_items = flushed_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should stay serialized as an object");
    assert_eq!(
        remaining_pending_items.len(),
        1,
        "healthy ready-pair writes must not flush actively claimed pending backlog"
    );
    let remaining_pending = remaining_pending_items
        .get(alice_request_key.as_str())
        .expect("claimed pending request should stay in backlog");
    assert_eq!(remaining_pending["ownerActorId"], "u_operator_a");
    assert_eq!(remaining_pending["ownerActorKind"], "user");
    assert!(
        remaining_pending["claimedAt"].as_str().is_some(),
        "claimed backlog item should keep claimedAt after unrelated healthy write"
    );
    assert!(
        remaining_pending["leaseExpiresAt"].as_str().is_some(),
        "claimed backlog item should keep leaseExpiresAt after unrelated healthy write"
    );

    let bob_bearer = bearer_token("t_demo", "actor_bob", "user", &[]);
    let (bob_history_status, bob_history_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::GET,
        "/api/v1/conversations/c_partner_ops_claim_blocked_auto_retry/messages",
        bob_bearer.as_str(),
        None,
    )
    .await;
    assert_eq!(bob_history_status, StatusCode::OK);
    assert_eq!(
        bob_history_json["items"][0]["message"]["body"]["summary"],
        "hello claim blocked auto retry"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_control_plane_social_shared_channel_next_ready_pair_retry_failure_reclaims_stale_claim_metadata()
 {
    let _guard = configure_public_bearer_secret().await;
    let runtime_dir =
        TestRuntimeDir::new("control_plane_shared_channel_stale_retry_failure_reclaim");
    let (runtime_base_url, runtime_handle) = spawn_public_runtime_server().await;

    let owner_bearer = bearer_token("t_demo", "owner_alice", "user", &[]);
    let (create_status, create_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "conversationId":"c_partner_ops_stale_retry_failure_reclaim",
            "conversationType":"group",
            "historyVisibility":"shared"
        })),
    )
    .await;
    assert_eq!(create_status, StatusCode::OK);
    assert_eq!(
        create_json["conversationId"],
        "c_partner_ops_stale_retry_failure_reclaim"
    );

    let (post_status, post_json) = http_json_request(
        runtime_base_url.as_str(),
        Method::POST,
        "/api/v1/conversations/c_partner_ops_stale_retry_failure_reclaim/messages",
        owner_bearer.as_str(),
        Some(serde_json::json!({
            "clientMsgId":"client_shared_stale_retry_failure_reclaim_001",
            "summary":"hello stale retry failure reclaim",
            "text":"hello stale retry failure reclaim"
        })),
    )
    .await;
    assert_eq!(post_status, StatusCode::OK);
    assert_eq!(
        post_json["messageId"],
        "msg_c_partner_ops_stale_retry_failure_reclaim_1"
    );

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let trigger = Arc::new(SwitchableSharedChannelSyncTrigger::failing(
        "remote runtime unavailable during stale retry failure reclaim test",
    ));
    let app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime.clone(),
        runtime_dir.path(),
        trigger.clone(),
    );

    let establish_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-connections")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "connectionId":"ec_stale_retry_failure_reclaim_001",
                        "eventId":"evt_ec_stale_retry_failure_reclaim_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T02:30:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_response.status(), StatusCode::OK);

    let policy_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/shared-channel-policies")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "policyId":"scp_stale_retry_failure_reclaim_001",
                        "eventId":"evt_scp_stale_retry_failure_reclaim_001",
                        "connectionId":"ec_stale_retry_failure_reclaim_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_partner_ops_stale_retry_failure_reclaim",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T02:31:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(policy_response.status(), StatusCode::OK);

    let initial_link_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_stale_retry_failure_reclaim_001",
                        "eventId":"evt_eml_stale_retry_failure_reclaim_001",
                        "connectionId":"ec_stale_retry_failure_reclaim_001",
                        "localActorId":"actor_alice",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T02:32:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("initial external member link write should return response");
    assert_eq!(
        initial_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    let pending_state = read_social_state_json(runtime_dir.path());
    let pending_items = pending_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should be serialized as an object");
    assert_eq!(pending_items.len(), 1);
    let alice_request_key = pending_items
        .keys()
        .next()
        .expect("pending shared-channel sync request key should exist")
        .to_owned();

    let claim_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_operator_a")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "requestKeys":[alice_request_key]
                    }))
                    .expect("stale retry failure reclaim targeted claim request should encode"),
                ))
                .unwrap(),
        )
        .await
        .expect("stale retry failure reclaim targeted claim should return response");
    assert_eq!(claim_response.status(), StatusCode::OK);

    let mut stale_state = read_social_state_json(runtime_dir.path());
    stale_state["pending_shared_channel_sync_requests"][alice_request_key.as_str()]["leaseExpiresAt"] =
        serde_json::Value::String("1970-01-01T00:00:00.000Z".into());
    write_social_state_json(runtime_dir.path(), &stale_state);

    let stale_app = control_plane_api::build_control_surface_with_cluster_and_governance_sinks_and_runtime_dir_and_shared_channel_sync_trigger(
        cluster,
        ops_runtime,
        audit_runtime,
        runtime_dir.path(),
        trigger,
    );

    let stale_inventory_response = stale_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/runtime/pending-shared-channel-sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stale retry failure reclaim inventory should return response");
    assert_eq!(stale_inventory_response.status(), StatusCode::OK);
    let stale_inventory_body = stale_inventory_response
        .into_body()
        .collect()
        .await
        .expect("stale retry failure reclaim inventory body should collect")
        .to_bytes();
    let stale_inventory_json: serde_json::Value = serde_json::from_slice(&stale_inventory_body)
        .expect("stale retry failure reclaim inventory body should be valid json");
    let stale_inventory_item = stale_inventory_json["items"]
        .as_array()
        .expect("stale retry failure reclaim inventory should serialize items as an array")
        .first()
        .expect("stale retry failure reclaim inventory item should exist");
    assert_eq!(stale_inventory_item["requestKey"], alice_request_key);
    assert_eq!(stale_inventory_item["leaseStatus"], "stale");

    let retry_link_response = stale_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/external-member-links")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "linkId":"eml_stale_retry_failure_reclaim_002",
                        "eventId":"evt_eml_stale_retry_failure_reclaim_002",
                        "connectionId":"ec_stale_retry_failure_reclaim_001",
                        "localActorId":"actor_bob",
                        "localActorKind":"user",
                        "externalMemberId":"partner::bob",
                        "linkedAt":"2026-04-11T02:33:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("retry external member link write should return response");
    assert_eq!(
        retry_link_response.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    let retried_state = read_social_state_json(runtime_dir.path());
    let retried_pending_items = retried_state["pending_shared_channel_sync_requests"]
        .as_object()
        .expect("pending shared-channel sync backlog should remain serialized as an object");
    assert_eq!(retried_pending_items.len(), 2);
    let alice_pending = retried_pending_items
        .get(alice_request_key.as_str())
        .expect("original stale pending request should stay in backlog after failed retry");
    assert!(alice_pending["ownerActorId"].is_null());
    assert!(alice_pending["ownerActorKind"].is_null());
    assert!(alice_pending["claimedAt"].is_null());
    assert!(
        alice_pending["leaseExpiresAt"].is_null(),
        "failed retry should reclaim stale owner metadata before re-persisting backlog"
    );

    runtime_handle.abort();
    let _ = runtime_handle.await;
}
