use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{
    ContractError, ProviderDomain, ProviderHealthSnapshot, ProviderPluginDescriptor,
    UserModuleCreateOrBindRequest, UserModuleProvider, UserModuleUpdateProfileRequest,
    UserModuleUser,
};
use tower::ServiceExt;

static UNIQUE_RUNTIME_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let counter = UNIQUE_RUNTIME_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "craw_chat_user_module_provider_runtime_{unique}_{counter}"
    ))
}

fn state_file(runtime_dir: &Path, file_name: &str) -> PathBuf {
    runtime_dir.join("state").join(file_name)
}

fn journal_json(runtime_dir: &Path) -> serde_json::Value {
    let journal_content = fs::read_to_string(state_file(runtime_dir, "commit-journal.json"))
        .expect("commit journal should be readable");
    serde_json::from_str(&journal_content).expect("commit journal should be valid json")
}

fn event_payload<'a>(journal_json: &'a serde_json::Value, event_type: &str) -> serde_json::Value {
    let event = journal_json
        .as_array()
        .expect("journal should be an array")
        .iter()
        .find(|item| item["event_type"] == event_type)
        .unwrap_or_else(|| panic!("{event_type} event should exist"));
    serde_json::from_str(
        event["payload"]
            .as_str()
            .expect("payload should be serialized json"),
    )
    .expect("event payload should be valid json")
}

async fn list_members_json(
    app: axum::Router,
    tenant_id: &str,
    actor_id: &str,
    actor_kind: Option<&str>,
    device_id: &str,
    session_id: &str,
    conversation_id: &str,
) -> serde_json::Value {
    let mut request = Request::builder()
        .method("GET")
        .uri(format!("/api/v1/conversations/{conversation_id}/members"))
        .header("x-tenant-id", tenant_id)
        .header("x-user-id", actor_id)
        .header("x-device-id", device_id)
        .header("x-session-id", session_id);
    if let Some(actor_kind) = actor_kind {
        request = request.header("x-actor-kind", actor_kind);
    }
    let response = app
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .expect("list members request should succeed");
    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("list members body should collect")
        .to_bytes();
    serde_json::from_slice(&body).expect("list members response should be valid json")
}

#[derive(Clone)]
struct StubLocalUserModuleProvider;

impl UserModuleProvider for StubLocalUserModuleProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "user-module-local",
            ProviderDomain::UserModule,
            "local",
            "Local User Module",
        )
        .with_default_selected(true)
        .with_required_capabilities(["query", "profile", "bind"])
    }

    fn get_user(
        &self,
        tenant_id: &str,
        user_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        Ok(Some(UserModuleUser {
            tenant_id: tenant_id.into(),
            user_id: user_id.into(),
            display_name: format!("Local {user_id}"),
            external_system: None,
            external_principal_id: None,
            attributes: BTreeMap::from([
                ("department".into(), "platform".into()),
                ("source".into(), "local".into()),
            ]),
            disabled: false,
        }))
    }

    fn batch_get_users(
        &self,
        tenant_id: &str,
        user_ids: &[String],
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        user_ids
            .iter()
            .map(|user_id| self.get_user(tenant_id, user_id))
            .collect::<Result<Vec<_>, _>>()
            .map(|users| users.into_iter().flatten().collect())
    }

    fn search_users(
        &self,
        tenant_id: &str,
        keyword: &str,
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        Ok(self.get_user(tenant_id, keyword)?.into_iter().collect())
    }

    fn create_or_bind_user(
        &self,
        request: UserModuleCreateOrBindRequest,
    ) -> Result<UserModuleUser, ContractError> {
        Ok(UserModuleUser {
            tenant_id: request.tenant_id,
            user_id: request.user_id,
            display_name: request.display_name,
            external_system: request.external_system,
            external_principal_id: request.external_principal_id,
            attributes: BTreeMap::from([("bindingMode".into(), "local".into())]),
            disabled: false,
        })
    }

    fn update_user_profile(
        &self,
        request: UserModuleUpdateProfileRequest,
    ) -> Result<UserModuleUser, ContractError> {
        Ok(UserModuleUser {
            tenant_id: request.tenant_id,
            user_id: request.user_id,
            display_name: request
                .display_name
                .unwrap_or_else(|| "Updated Local".into()),
            external_system: None,
            external_principal_id: None,
            attributes: request.attributes,
            disabled: false,
        })
    }

    fn disable_user(&self, _tenant_id: &str, _user_id: &str) -> Result<bool, ContractError> {
        Ok(true)
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        Ok(None)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy("user-module-local", "2026-04-08T00:00:00Z")
    }
}

#[derive(Clone)]
struct StubExternalUserModuleProvider;

impl UserModuleProvider for StubExternalUserModuleProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "user-module-external",
            ProviderDomain::UserModule,
            "external",
            "External User Module",
        )
        .with_required_capabilities(["query", "bind", "external-mapping"])
    }

    fn get_user(
        &self,
        tenant_id: &str,
        user_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        Ok(Some(UserModuleUser {
            tenant_id: tenant_id.into(),
            user_id: user_id.into(),
            display_name: format!("External {user_id}"),
            external_system: Some("corp-idp".into()),
            external_principal_id: Some(format!("ext::{user_id}")),
            attributes: BTreeMap::from([
                ("directory".into(), "corp-idp".into()),
                ("source".into(), "external".into()),
            ]),
            disabled: false,
        }))
    }

    fn batch_get_users(
        &self,
        tenant_id: &str,
        user_ids: &[String],
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        user_ids
            .iter()
            .map(|user_id| self.get_user(tenant_id, user_id))
            .collect::<Result<Vec<_>, _>>()
            .map(|users| users.into_iter().flatten().collect())
    }

    fn search_users(
        &self,
        tenant_id: &str,
        keyword: &str,
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        Ok(self.get_user(tenant_id, keyword)?.into_iter().collect())
    }

    fn create_or_bind_user(
        &self,
        request: UserModuleCreateOrBindRequest,
    ) -> Result<UserModuleUser, ContractError> {
        Ok(UserModuleUser {
            tenant_id: request.tenant_id,
            user_id: request.user_id.clone(),
            display_name: request.display_name,
            external_system: Some("corp-idp".into()),
            external_principal_id: Some(format!("ext::{}", request.user_id)),
            attributes: BTreeMap::from([("bindingMode".into(), "external".into())]),
            disabled: false,
        })
    }

    fn update_user_profile(
        &self,
        request: UserModuleUpdateProfileRequest,
    ) -> Result<UserModuleUser, ContractError> {
        Ok(UserModuleUser {
            tenant_id: request.tenant_id,
            user_id: request.user_id.clone(),
            display_name: request
                .display_name
                .unwrap_or_else(|| format!("External {}", request.user_id)),
            external_system: Some("corp-idp".into()),
            external_principal_id: Some(format!("ext::{}", request.user_id)),
            attributes: request.attributes,
            disabled: false,
        })
    }

    fn disable_user(&self, _tenant_id: &str, _user_id: &str) -> Result<bool, ContractError> {
        Ok(true)
    }

    fn map_external_principal(
        &self,
        tenant_id: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        Ok(Some(UserModuleUser {
            tenant_id: tenant_id.into(),
            user_id: external_principal_id.into(),
            display_name: format!("External {external_principal_id}"),
            external_system: Some(external_system.into()),
            external_principal_id: Some(external_principal_id.into()),
            attributes: BTreeMap::from([("source".into(), "external".into())]),
            disabled: false,
        }))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy("user-module-external", "2026-04-08T00:00:00Z")
    }
}

#[tokio::test]
async fn test_local_user_module_provider_enriches_user_message_sender_and_member_attributes() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_user_module_provider(
        runtime_dir.as_path(),
        Arc::new(StubLocalUserModuleProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_user_module_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_user_module_local/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_other_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);
    let add_member_body = add_member
        .into_body()
        .collect()
        .await
        .expect("add member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value =
        serde_json::from_slice(&add_member_body).expect("member response should be valid json");
    assert_eq!(add_member_json["principalKind"], "user");
    assert_eq!(
        add_member_json["attributes"]["displayName"],
        "Local u_other_demo"
    );
    assert_eq!(add_member_json["attributes"]["department"], "platform");
    assert_eq!(
        add_member_json["attributes"]["userModulePluginId"],
        "user-module-local"
    );

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_user_module_local/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_local_provider",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let journal_content =
        fs::read_to_string(state_file(runtime_dir.as_path(), "commit-journal.json"))
            .expect("commit journal should be readable");
    let journal_json: serde_json::Value =
        serde_json::from_str(&journal_content).expect("commit journal should be valid json");
    let message_posted = journal_json
        .as_array()
        .expect("journal should be an array")
        .iter()
        .find(|item| item["event_type"] == "message.posted")
        .expect("message.posted event should exist");
    let payload: serde_json::Value = serde_json::from_str(
        message_posted["payload"]
            .as_str()
            .expect("payload should be serialized json"),
    )
    .expect("message payload should be valid json");
    assert_eq!(payload["sender"]["id"], "u_demo");
    assert_eq!(payload["sender"]["metadata"]["displayName"], "Local u_demo");
    assert_eq!(payload["sender"]["metadata"]["department"], "platform");
    assert_eq!(
        payload["sender"]["metadata"]["userModulePluginId"],
        "user-module-local"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_external_user_module_provider_enriches_user_message_sender_and_member_attributes() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_user_module_provider(
        runtime_dir.as_path(),
        Arc::new(StubExternalUserModuleProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_ext_owner")
                .header("x-device-id", "d_ext_owner")
                .header("x-session-id", "s_ext_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_user_module_external",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_user_module_external/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_ext_owner")
                .header("x-device-id", "d_ext_owner")
                .header("x-session-id", "s_ext_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_ext_member",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);
    let add_member_body = add_member
        .into_body()
        .collect()
        .await
        .expect("add member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value =
        serde_json::from_slice(&add_member_body).expect("member response should be valid json");
    assert_eq!(
        add_member_json["attributes"]["displayName"],
        "External u_ext_member"
    );
    assert_eq!(add_member_json["attributes"]["externalSystem"], "corp-idp");
    assert_eq!(
        add_member_json["attributes"]["externalPrincipalId"],
        "ext::u_ext_member"
    );
    assert_eq!(
        add_member_json["attributes"]["userModulePluginId"],
        "user-module-external"
    );

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_user_module_external/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_ext_owner")
                .header("x-device-id", "d_ext_owner")
                .header("x-session-id", "s_ext_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_external_provider",
                        "summary":"external",
                        "text":"external"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let journal_content =
        fs::read_to_string(state_file(runtime_dir.as_path(), "commit-journal.json"))
            .expect("commit journal should be readable");
    let journal_json: serde_json::Value =
        serde_json::from_str(&journal_content).expect("commit journal should be valid json");
    let message_posted = journal_json
        .as_array()
        .expect("journal should be an array")
        .iter()
        .find(|item| item["event_type"] == "message.posted")
        .expect("message.posted event should exist");
    let payload: serde_json::Value = serde_json::from_str(
        message_posted["payload"]
            .as_str()
            .expect("payload should be serialized json"),
    )
    .expect("message payload should be valid json");
    assert_eq!(
        payload["sender"]["metadata"]["displayName"],
        "External u_ext_owner"
    );
    assert_eq!(payload["sender"]["metadata"]["externalSystem"], "corp-idp");
    assert_eq!(
        payload["sender"]["metadata"]["externalPrincipalId"],
        "ext::u_ext_owner"
    );
    assert_eq!(
        payload["sender"]["metadata"]["userModulePluginId"],
        "user-module-external"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_user_module_provider_enriches_bootstrap_user_members_across_creation_flows() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_user_module_provider(
        runtime_dir.as_path(),
        Arc::new(StubLocalUserModuleProvider),
    );

    let create_group = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_group_owner")
                .header("x-device-id", "d_group_owner")
                .header("x-session-id", "s_group_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_bootstrap_group_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("group create should succeed");
    assert_eq!(create_group.status(), StatusCode::OK);

    let group_members = list_members_json(
        app.clone(),
        "t_demo",
        "u_group_owner",
        None,
        "d_group_owner",
        "s_group_owner",
        "c_bootstrap_group_local",
    )
    .await;
    let group_owner = group_members["items"]
        .as_array()
        .expect("group members should be an array")
        .iter()
        .find(|member| member["principalId"] == "u_group_owner")
        .expect("group owner should exist");
    assert_eq!(
        group_owner["attributes"]["displayName"],
        "Local u_group_owner"
    );
    assert_eq!(group_owner["attributes"]["department"], "platform");
    assert_eq!(
        group_owner["attributes"]["userModulePluginId"],
        "user-module-local"
    );

    let create_agent_dialog = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-dialogs")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dialog_owner")
                .header("x-device-id", "d_dialog_owner")
                .header("x-session-id", "s_dialog_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_bootstrap_agent_dialog_local",
                        "agentId":"ag_assistant"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent dialog create should succeed");
    assert_eq!(create_agent_dialog.status(), StatusCode::OK);

    let agent_dialog_members = list_members_json(
        app.clone(),
        "t_demo",
        "u_dialog_owner",
        None,
        "d_dialog_owner",
        "s_dialog_owner",
        "c_bootstrap_agent_dialog_local",
    )
    .await;
    let requester = agent_dialog_members["items"]
        .as_array()
        .expect("agent dialog members should be an array")
        .iter()
        .find(|member| member["principalId"] == "u_dialog_owner")
        .expect("agent dialog requester should exist");
    assert_eq!(requester["attributes"]["dialogRole"], "requester");
    assert_eq!(
        requester["attributes"]["displayName"],
        "Local u_dialog_owner"
    );
    assert_eq!(
        requester["attributes"]["userModulePluginId"],
        "user-module-local"
    );

    let create_system_channel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/system-channels")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "svc_system")
                .header("x-actor-kind", "system")
                .header("x-device-id", "d_system")
                .header("x-session-id", "s_system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_bootstrap_system_channel_local",
                        "subscriberId":"u_system_subscriber"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should succeed");
    assert_eq!(create_system_channel.status(), StatusCode::OK);

    let system_channel_members = list_members_json(
        app.clone(),
        "t_demo",
        "svc_system",
        Some("system"),
        "d_system",
        "s_system",
        "c_bootstrap_system_channel_local",
    )
    .await;
    let subscriber = system_channel_members["items"]
        .as_array()
        .expect("system channel members should be an array")
        .iter()
        .find(|member| member["principalId"] == "u_system_subscriber")
        .expect("system channel subscriber should exist");
    assert_eq!(subscriber["attributes"]["channelRole"], "subscriber");
    assert_eq!(
        subscriber["attributes"]["displayName"],
        "Local u_system_subscriber"
    );
    assert_eq!(subscriber["attributes"]["department"], "platform");
    assert_eq!(
        subscriber["attributes"]["userModulePluginId"],
        "user-module-local"
    );

    let create_agent_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-handoffs")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "ag_handoff_source")
                .header("x-actor-kind", "agent")
                .header("x-device-id", "d_handoff_source")
                .header("x-session-id", "s_handoff_source")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_bootstrap_handoff_local",
                        "targetId":"u_handoff_target",
                        "targetKind":"user",
                        "handoffSessionId":"hs_local_bootstrap",
                        "handoffReason":"needs_human"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent handoff create should succeed");
    assert_eq!(create_agent_handoff.status(), StatusCode::OK);

    let handoff_members = list_members_json(
        app.clone(),
        "t_demo",
        "ag_handoff_source",
        Some("agent"),
        "d_handoff_source",
        "s_handoff_source",
        "c_bootstrap_handoff_local",
    )
    .await;
    let handoff_target = handoff_members["items"]
        .as_array()
        .expect("handoff members should be an array")
        .iter()
        .find(|member| member["principalId"] == "u_handoff_target")
        .expect("handoff target should exist");
    assert_eq!(handoff_target["attributes"]["handoffRole"], "target");
    assert_eq!(
        handoff_target["attributes"]["displayName"],
        "Local u_handoff_target"
    );
    assert_eq!(handoff_target["attributes"]["department"], "platform");
    assert_eq!(
        handoff_target["attributes"]["userModulePluginId"],
        "user-module-local"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_external_user_module_provider_enriches_bootstrap_user_members_across_creation_flows()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_user_module_provider(
        runtime_dir.as_path(),
        Arc::new(StubExternalUserModuleProvider),
    );

    let create_group = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_group_owner_external")
                .header("x-device-id", "d_group_owner_external")
                .header("x-session-id", "s_group_owner_external")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_bootstrap_group_external",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("group create should succeed");
    assert_eq!(create_group.status(), StatusCode::OK);

    let group_members = list_members_json(
        app.clone(),
        "t_demo",
        "u_group_owner_external",
        None,
        "d_group_owner_external",
        "s_group_owner_external",
        "c_bootstrap_group_external",
    )
    .await;
    let group_owner = group_members["items"]
        .as_array()
        .expect("group members should be an array")
        .iter()
        .find(|member| member["principalId"] == "u_group_owner_external")
        .expect("group owner should exist");
    assert_eq!(
        group_owner["attributes"]["displayName"],
        "External u_group_owner_external"
    );
    assert_eq!(group_owner["attributes"]["externalSystem"], "corp-idp");
    assert_eq!(
        group_owner["attributes"]["externalPrincipalId"],
        "ext::u_group_owner_external"
    );
    assert_eq!(
        group_owner["attributes"]["userModulePluginId"],
        "user-module-external"
    );

    let create_system_channel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/system-channels")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "svc_system_external")
                .header("x-actor-kind", "system")
                .header("x-device-id", "d_system_external")
                .header("x-session-id", "s_system_external")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_bootstrap_system_channel_external",
                        "subscriberId":"u_system_subscriber_external"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should succeed");
    assert_eq!(create_system_channel.status(), StatusCode::OK);

    let system_channel_members = list_members_json(
        app.clone(),
        "t_demo",
        "svc_system_external",
        Some("system"),
        "d_system_external",
        "s_system_external",
        "c_bootstrap_system_channel_external",
    )
    .await;
    let subscriber = system_channel_members["items"]
        .as_array()
        .expect("system channel members should be an array")
        .iter()
        .find(|member| member["principalId"] == "u_system_subscriber_external")
        .expect("system channel subscriber should exist");
    assert_eq!(
        subscriber["attributes"]["displayName"],
        "External u_system_subscriber_external"
    );
    assert_eq!(subscriber["attributes"]["externalSystem"], "corp-idp");
    assert_eq!(
        subscriber["attributes"]["externalPrincipalId"],
        "ext::u_system_subscriber_external"
    );
    assert_eq!(
        subscriber["attributes"]["userModulePluginId"],
        "user-module-external"
    );

    let create_agent_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/agent-handoffs")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "ag_handoff_source_external")
                .header("x-actor-kind", "agent")
                .header("x-device-id", "d_handoff_source_external")
                .header("x-session-id", "s_handoff_source_external")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_bootstrap_handoff_external",
                        "targetId":"u_handoff_target_external",
                        "targetKind":"user",
                        "handoffSessionId":"hs_external_bootstrap",
                        "handoffReason":"needs_human"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent handoff create should succeed");
    assert_eq!(create_agent_handoff.status(), StatusCode::OK);

    let handoff_members = list_members_json(
        app.clone(),
        "t_demo",
        "ag_handoff_source_external",
        Some("agent"),
        "d_handoff_source_external",
        "s_handoff_source_external",
        "c_bootstrap_handoff_external",
    )
    .await;
    let handoff_target = handoff_members["items"]
        .as_array()
        .expect("handoff members should be an array")
        .iter()
        .find(|member| member["principalId"] == "u_handoff_target_external")
        .expect("handoff target should exist");
    assert_eq!(
        handoff_target["attributes"]["displayName"],
        "External u_handoff_target_external"
    );
    assert_eq!(handoff_target["attributes"]["externalSystem"], "corp-idp");
    assert_eq!(
        handoff_target["attributes"]["externalPrincipalId"],
        "ext::u_handoff_target_external"
    );
    assert_eq!(
        handoff_target["attributes"]["userModulePluginId"],
        "user-module-external"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_user_module_provider_enriches_message_edit_and_recall_actor_metadata() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_user_module_provider(
        runtime_dir.as_path(),
        Arc::new(StubLocalUserModuleProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_editor_local")
                .header("x-device-id", "d_editor_local")
                .header("x-session-id", "s_editor_local")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_user_module_edit_recall_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_user_module_edit_recall_local/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_editor_local")
                .header("x-device-id", "d_editor_local")
                .header("x-session-id", "s_editor_local")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_edit_recall_local",
                        "summary":"before",
                        "text":"before"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_body = post_message
        .into_body()
        .collect()
        .await
        .expect("post body should collect")
        .to_bytes();
    let post_json: serde_json::Value =
        serde_json::from_slice(&post_body).expect("post response should be valid json");
    let message_id = post_json["messageId"]
        .as_str()
        .expect("message id should exist");

    let edit_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/messages/{message_id}/edit"))
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_editor_local")
                .header("x-device-id", "d_editor_local")
                .header("x-session-id", "s_editor_local")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "summary":"after edit",
                        "text":"after edit"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("edit message should succeed");
    assert_eq!(edit_message.status(), StatusCode::OK);

    let recall_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/messages/{message_id}/recall"))
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_editor_local")
                .header("x-device-id", "d_editor_local")
                .header("x-session-id", "s_editor_local")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("recall message should succeed");
    assert_eq!(recall_message.status(), StatusCode::OK);

    let journal = journal_json(runtime_dir.as_path());
    let edited_payload = event_payload(&journal, "message.edited");
    assert_eq!(edited_payload["editor"]["id"], "u_editor_local");
    assert_eq!(
        edited_payload["editor"]["metadata"]["displayName"],
        "Local u_editor_local"
    );
    assert_eq!(
        edited_payload["editor"]["metadata"]["department"],
        "platform"
    );
    assert_eq!(
        edited_payload["editor"]["metadata"]["userModulePluginId"],
        "user-module-local"
    );

    let recalled_payload = event_payload(&journal, "message.recalled");
    assert_eq!(recalled_payload["recalledBy"]["id"], "u_editor_local");
    assert_eq!(
        recalled_payload["recalledBy"]["metadata"]["displayName"],
        "Local u_editor_local"
    );
    assert_eq!(
        recalled_payload["recalledBy"]["metadata"]["department"],
        "platform"
    );
    assert_eq!(
        recalled_payload["recalledBy"]["metadata"]["userModulePluginId"],
        "user-module-local"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_external_user_module_provider_enriches_message_edit_and_recall_actor_metadata() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_user_module_provider(
        runtime_dir.as_path(),
        Arc::new(StubExternalUserModuleProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_editor_external")
                .header("x-device-id", "d_editor_external")
                .header("x-session-id", "s_editor_external")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_user_module_edit_recall_external",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_user_module_edit_recall_external/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_editor_external")
                .header("x-device-id", "d_editor_external")
                .header("x-session-id", "s_editor_external")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_edit_recall_external",
                        "summary":"before external",
                        "text":"before external"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_body = post_message
        .into_body()
        .collect()
        .await
        .expect("post body should collect")
        .to_bytes();
    let post_json: serde_json::Value =
        serde_json::from_slice(&post_body).expect("post response should be valid json");
    let message_id = post_json["messageId"]
        .as_str()
        .expect("message id should exist");

    let edit_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/messages/{message_id}/edit"))
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_editor_external")
                .header("x-device-id", "d_editor_external")
                .header("x-session-id", "s_editor_external")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "summary":"after external edit",
                        "text":"after external edit"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("edit message should succeed");
    assert_eq!(edit_message.status(), StatusCode::OK);

    let recall_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/messages/{message_id}/recall"))
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_editor_external")
                .header("x-device-id", "d_editor_external")
                .header("x-session-id", "s_editor_external")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("recall message should succeed");
    assert_eq!(recall_message.status(), StatusCode::OK);

    let journal = journal_json(runtime_dir.as_path());
    let edited_payload = event_payload(&journal, "message.edited");
    assert_eq!(
        edited_payload["editor"]["metadata"]["displayName"],
        "External u_editor_external"
    );
    assert_eq!(
        edited_payload["editor"]["metadata"]["externalSystem"],
        "corp-idp"
    );
    assert_eq!(
        edited_payload["editor"]["metadata"]["externalPrincipalId"],
        "ext::u_editor_external"
    );
    assert_eq!(
        edited_payload["editor"]["metadata"]["userModulePluginId"],
        "user-module-external"
    );

    let recalled_payload = event_payload(&journal, "message.recalled");
    assert_eq!(
        recalled_payload["recalledBy"]["metadata"]["displayName"],
        "External u_editor_external"
    );
    assert_eq!(
        recalled_payload["recalledBy"]["metadata"]["externalSystem"],
        "corp-idp"
    );
    assert_eq!(
        recalled_payload["recalledBy"]["metadata"]["externalPrincipalId"],
        "ext::u_editor_external"
    );
    assert_eq!(
        recalled_payload["recalledBy"]["metadata"]["userModulePluginId"],
        "user-module-external"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
