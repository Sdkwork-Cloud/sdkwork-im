use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use conversation_runtime::{
    ApplyConversationPolicyCommand, ConversationRuntime, CreateConversationCommand,
    PostMessageCommand,
};
use http_body_util::BodyExt;
use im_adapters_local_disk::FileCommitJournal;
use im_domain_core::conversation::ConversationPolicy;
use im_domain_core::message::{ContentPart, MessageBody, MessageType, Sender};
use tower::ServiceExt;

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

fn seed_shared_history_conversation(runtime_dir: &Path) {
    let journal = FileCommitJournal::new(
        "local-minimal",
        runtime_dir.join("state").join("commit-journal.json"),
    );
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_history_shared_local".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("seed create conversation should succeed");

    runtime
        .apply_conversation_policy(ApplyConversationPolicyCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_history_shared_local".into(),
            applied_by: "u_owner".into(),
            policy: ConversationPolicy {
                policy_version: "group.policy.v1".into(),
                capability_flags: None,
                history_visibility: "shared".into(),
                retention_policy_ref: "tenant.standard".into(),
            },
        })
        .expect("seed apply shared-history policy should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_history_shared_local".into(),
            sender: Sender {
                id: "u_owner".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_history_shared_local".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello local shared".into()),
                parts: vec![ContentPart::text("hello local shared")],
                render_hints: Default::default(),
            },
        })
        .expect("seed post shared-history message should succeed");
}

#[tokio::test]
async fn test_local_minimal_profile_control_plane_shared_channel_auto_sync_materializes_runtime_linked_member()
 {
    let runtime_dir = TestRuntimeDir::new("local_minimal_control_plane_social_sync");
    seed_shared_history_conversation(runtime_dir.path());

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.path());

    let establish_connection = app
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
                        "connectionId":"ec_local_001",
                        "eventId":"evt_ec_local_001",
                        "externalTenantId":"t_partner",
                        "connectionKind":"shared_channel",
                        "establishedAt":"2026-04-11T09:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external connection write should return response");
    assert_eq!(establish_connection.status(), StatusCode::OK);

    let apply_policy = app
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
                        "policyId":"scp_local_001",
                        "eventId":"evt_scp_local_001",
                        "connectionId":"ec_local_001",
                        "channelId":"ch_partner_ops",
                        "conversationId":"c_history_shared_local",
                        "policyVersion":1,
                        "historyVisibility":"shared",
                        "appliedAt":"2026-04-11T09:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("shared channel policy write should return response");
    assert_eq!(apply_policy.status(), StatusCode::OK);

    let bind_link = app
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
                        "linkId":"eml_local_001",
                        "eventId":"evt_eml_local_001",
                        "connectionId":"ec_local_001",
                        "localActorId":"u_partner_local",
                        "localActorKind":"user",
                        "externalMemberId":"partner::alice",
                        "linkedAt":"2026-04-11T09:02:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("external member link write should return response");
    assert_eq!(bind_link.status(), StatusCode::OK);

    let linked_history = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/conversations/c_history_shared_local/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_partner_local")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("linked actor history request should return response");
    assert_eq!(linked_history.status(), StatusCode::OK);

    let linked_history_body = linked_history
        .into_body()
        .collect()
        .await
        .expect("linked actor history body should collect")
        .to_bytes();
    let linked_history_json: serde_json::Value = serde_json::from_slice(&linked_history_body)
        .expect("linked actor history body should be valid json");
    assert_eq!(
        linked_history_json["items"][0]["messageId"],
        "msg_c_history_shared_local_1"
    );
    assert_eq!(
        linked_history_json["items"][0]["summary"],
        "hello local shared"
    );
}
