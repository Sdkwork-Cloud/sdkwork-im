use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{
    ContractError, PrincipalProfile, PrincipalProfileProvider, ProviderDomain,
    ProviderHealthSnapshot, ProviderPluginDescriptor,
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
        "craw_chat_principal_profile_provider_runtime_{unique}_{counter}"
    ))
}

fn state_file(runtime_dir: &Path, file_name: &str) -> PathBuf {
    runtime_dir.join("state").join(file_name)
}

fn journal_events(runtime_dir: &Path) -> Vec<serde_json::Value> {
    let journal_content = fs::read_to_string(state_file(runtime_dir, "commit-journal.json"))
        .expect("commit journal should be readable");
    journal_content
        .lines()
        .map(|line| serde_json::from_str(line).expect("commit journal line should be valid json"))
        .collect()
}

fn event_payload(journal_events: &[serde_json::Value], event_type: &str) -> serde_json::Value {
    let event = journal_events
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
    actor_kind: &str,
    device_id: &str,
    session_id: &str,
    conversation_id: &str,
) -> serde_json::Value {
    let request = Request::builder()
        .method("GET")
        .uri(format!(
            "/im/v3/api/chat/conversations/{conversation_id}/members"
        ))
        .header("x-sdkwork-tenant-id", tenant_id)
        .header("x-sdkwork-user-id", actor_id)
        .header("x-sdkwork-actor-kind", actor_kind)
        .header("x-sdkwork-device-id", device_id)
        .header("x-sdkwork-session-id", session_id);
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
struct StubLocalPrincipalProfileProvider;

impl PrincipalProfileProvider for StubLocalPrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-upstream-context",
            ProviderDomain::PrincipalProfile,
            "upstream-context",
            "Upstream Context Principal Profile",
        )
        .with_default_selected(true)
        .with_required_capabilities(["read", "profile"])
    }

    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        _principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(Some(PrincipalProfile {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            display_name: format!("Local {principal_id}"),
            external_system: None,
            external_principal_id: None,
            attributes: BTreeMap::from([
                ("department".into(), "platform".into()),
                ("source".into(), "local".into()),
            ]),
            inactive: false,
        }))
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        principal_ids
            .iter()
            .map(|principal_id| self.get_profile(tenant_id, principal_id, principal_kind))
            .collect::<Result<Vec<_>, _>>()
            .map(|users| users.into_iter().flatten().collect())
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(self
            .get_profile(tenant_id, keyword, principal_kind)?
            .into_iter()
            .collect())
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(None)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(
            "principal-profile-upstream-context",
            "2026-04-08T00:00:00Z",
        )
    }
}

#[derive(Clone)]
struct StubExternalPrincipalProfileProvider;

impl PrincipalProfileProvider for StubExternalPrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-external-catalog",
            ProviderDomain::PrincipalProfile,
            "external-catalog",
            "External Catalog Principal Profile",
        )
        .with_required_capabilities(["read", "profile", "external-mapping"])
    }

    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        _principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(Some(PrincipalProfile {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            display_name: format!("External {principal_id}"),
            external_system: Some("corp-idp".into()),
            external_principal_id: Some(format!("ext::{principal_id}")),
            attributes: BTreeMap::from([
                ("directory".into(), "corp-idp".into()),
                ("source".into(), "external".into()),
            ]),
            inactive: false,
        }))
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        principal_ids
            .iter()
            .map(|principal_id| self.get_profile(tenant_id, principal_id, principal_kind))
            .collect::<Result<Vec<_>, _>>()
            .map(|users| users.into_iter().flatten().collect())
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(self
            .get_profile(tenant_id, keyword, principal_kind)?
            .into_iter()
            .collect())
    }

    fn map_external_principal(
        &self,
        tenant_id: &str,
        _principal_kind: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(Some(PrincipalProfile {
            tenant_id: tenant_id.into(),
            principal_id: external_principal_id.into(),
            display_name: format!("External {external_principal_id}"),
            external_system: Some(external_system.into()),
            external_principal_id: Some(external_principal_id.into()),
            attributes: BTreeMap::from([("source".into(), "external".into())]),
            inactive: false,
        }))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(
            "principal-profile-external-catalog",
            "2026-04-08T00:00:00Z",
        )
    }
}

#[derive(Clone)]
struct OversizedLocalPrincipalProfileProvider;

#[derive(Clone)]
struct SelectivelyDisabledLocalPrincipalProfileProvider {
    inactive_user_ids: Arc<Vec<&'static str>>,
}

impl SelectivelyDisabledLocalPrincipalProfileProvider {
    fn new(inactive_user_ids: &[&'static str]) -> Self {
        Self {
            inactive_user_ids: Arc::new(inactive_user_ids.to_vec()),
        }
    }

    fn is_disabled(&self, principal_id: &str) -> bool {
        self.inactive_user_ids.contains(&principal_id)
    }
}

impl PrincipalProfileProvider for SelectivelyDisabledLocalPrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-upstream-context",
            ProviderDomain::PrincipalProfile,
            "upstream-context",
            "Upstream Context Principal Profile",
        )
        .with_default_selected(true)
        .with_required_capabilities(["read", "profile"])
    }

    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        _principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(Some(PrincipalProfile {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            display_name: format!("Local {principal_id}"),
            external_system: None,
            external_principal_id: None,
            attributes: BTreeMap::from([("source".into(), "local".into())]),
            inactive: self.is_disabled(principal_id),
        }))
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        principal_ids
            .iter()
            .map(|principal_id| self.get_profile(tenant_id, principal_id, principal_kind))
            .collect::<Result<Vec<_>, _>>()
            .map(|users| users.into_iter().flatten().collect())
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(self
            .get_profile(tenant_id, keyword, principal_kind)?
            .into_iter()
            .collect())
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(None)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(
            "principal-profile-upstream-context",
            "2026-04-08T00:00:00Z",
        )
    }
}

impl PrincipalProfileProvider for OversizedLocalPrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-upstream-context",
            ProviderDomain::PrincipalProfile,
            "upstream-context",
            "Upstream Context Principal Profile",
        )
        .with_default_selected(true)
        .with_required_capabilities(["read", "profile"])
    }

    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        _principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(Some(PrincipalProfile {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            display_name: format!("Local {principal_id}"),
            external_system: None,
            external_principal_id: None,
            attributes: BTreeMap::from([("profile".into(), "x".repeat(70 * 1024))]),
            inactive: false,
        }))
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        principal_ids
            .iter()
            .map(|principal_id| self.get_profile(tenant_id, principal_id, principal_kind))
            .collect::<Result<Vec<_>, _>>()
            .map(|users| users.into_iter().flatten().collect())
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(self
            .get_profile(tenant_id, keyword, principal_kind)?
            .into_iter()
            .collect())
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(None)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(
            "principal-profile-upstream-context",
            "2026-04-08T00:00:00Z",
        )
    }
}

#[derive(Clone, Default)]
struct EscalatingLocalPrincipalProfileProvider {
    get_user_calls: Arc<AtomicUsize>,
}

impl PrincipalProfileProvider for EscalatingLocalPrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-upstream-context",
            ProviderDomain::PrincipalProfile,
            "upstream-context",
            "Upstream Context Principal Profile",
        )
        .with_default_selected(true)
        .with_required_capabilities(["read", "profile"])
    }

    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        _principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        let call_index = self.get_user_calls.fetch_add(1, Ordering::Relaxed);
        let attributes = if call_index == 0 {
            BTreeMap::from([("department".into(), "platform".into())])
        } else {
            BTreeMap::from([("profile".into(), "x".repeat(70 * 1024))])
        };
        Ok(Some(PrincipalProfile {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            display_name: format!("Local {principal_id}"),
            external_system: None,
            external_principal_id: None,
            attributes,
            inactive: false,
        }))
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        principal_ids
            .iter()
            .map(|principal_id| self.get_profile(tenant_id, principal_id, principal_kind))
            .collect::<Result<Vec<_>, _>>()
            .map(|users| users.into_iter().flatten().collect())
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(self
            .get_profile(tenant_id, keyword, principal_kind)?
            .into_iter()
            .collect())
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(None)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(
            "principal-profile-upstream-context",
            "2026-04-08T00:00:00Z",
        )
    }
}

#[derive(Clone)]
struct StrictKnownPrincipalProfileProvider {
    known_user_ids: Arc<Vec<&'static str>>,
}

impl StrictKnownPrincipalProfileProvider {
    fn new(known_user_ids: &[&'static str]) -> Self {
        Self {
            known_user_ids: Arc::new(known_user_ids.to_vec()),
        }
    }

    fn known_user(&self, tenant_id: &str, principal_id: &str) -> Option<PrincipalProfile> {
        self.known_user_ids
            .iter()
            .copied()
            .find(|known_user_id| *known_user_id == principal_id)
            .map(|known_user_id| PrincipalProfile {
                tenant_id: tenant_id.into(),
                principal_id: known_user_id.into(),
                display_name: format!("Known {known_user_id}"),
                external_system: None,
                external_principal_id: None,
                attributes: BTreeMap::from([("source".into(), "strict-known".into())]),
                inactive: false,
            })
    }
}

impl PrincipalProfileProvider for StrictKnownPrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-upstream-context",
            ProviderDomain::PrincipalProfile,
            "upstream-context",
            "Upstream Context Principal Profile",
        )
        .with_default_selected(true)
        .with_required_capabilities(["read", "profile"])
    }

    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        _principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(self.known_user(tenant_id, principal_id))
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        _principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(principal_ids
            .iter()
            .filter_map(|principal_id| self.known_user(tenant_id, principal_id.as_str()))
            .collect())
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        _principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(self
            .known_user_ids
            .iter()
            .copied()
            .filter(|principal_id| principal_id.contains(keyword))
            .filter_map(|principal_id| self.known_user(tenant_id, principal_id))
            .collect())
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(None)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot::healthy(
            "principal-profile-upstream-context",
            "2026-04-08T00:00:00Z",
        )
    }
}

#[tokio::test]
async fn test_local_principal_profile_provider_enriches_user_message_sender_and_member_attributes()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StubLocalPrincipalProfileProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_principal_profile_local",
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
                .uri("/im/v3/api/chat/conversations/c_principal_profile_local/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
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
        add_member_json["attributes"]["principalProfilePluginId"],
        "principal-profile-upstream-context"
    );

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_principal_profile_local/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
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

    let journal_events = journal_events(runtime_dir.as_path());
    let message_posted = journal_events
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
        payload["sender"]["metadata"]["principalProfilePluginId"],
        "principal-profile-upstream-context"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_social_friend_request_submit_rejects_unknown_target_user() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StrictKnownPrincipalProfileProvider::new(&["u_alice"])),
    );

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "targetUserId":"u_missing",
                        "requestMessage":"hello missing"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("submit friend request body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("submit friend request body should be valid json");
    assert_eq!(json["code"], "principal_profile_not_found");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_social_friend_request_accept_rejects_unknown_requester_user() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StrictKnownPrincipalProfileProvider::new(&["u_bob"])),
    );

    let seed_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_unknown_requester",
                        "eventId":"evt_fr_unknown_requester_submit",
                        "requesterUserId":"u_missing",
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob",
                        "requestedAt":"2026-04-15T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("control-plane friend request seed should return response");
    assert_eq!(seed_request.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/friend_requests/fr_unknown_requester/accept")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("accept friend request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("accept friend request body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("accept friend request body should be valid json");
    assert_eq!(json["code"], "principal_profile_not_found");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_direct_chat_binding_rejects_unknown_user_participant() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StrictKnownPrincipalProfileProvider::new(&["actor_a"])),
    );

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "svc_control")
                .header("x-sdkwork-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_direct_strict_unknown",
                        "directChatId":"dc_direct_strict_unknown",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_missing",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat binding should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("direct chat binding body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("direct chat binding body should be valid json");
    assert_eq!(json["code"], "principal_profile_not_found");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_timeline_query_rejects_unknown_user_member_after_restart_with_strict_provider() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let seed_app =
        local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
            runtime_dir.as_path(),
            Arc::new(StubLocalPrincipalProfileProvider),
        );

    let create_conversation = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_strict_restart_timeline",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_strict_restart_timeline/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_missing",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    let post_message = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_strict_restart_timeline/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_strict_restart_timeline",
                        "summary":"hello ghost",
                        "text":"hello ghost"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed post message should return response");
    assert_eq!(post_message.status(), StatusCode::OK);

    drop(seed_app);

    let strict_app =
        local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
            runtime_dir.as_path(),
            Arc::new(StrictKnownPrincipalProfileProvider::new(&["u_owner"])),
        );

    let response = strict_app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_strict_restart_timeline/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_missing")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("timeline body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("timeline body should be valid json");
    assert_eq!(json["code"], "principal_profile_not_found");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_list_members_rejects_unknown_user_member_after_restart_with_strict_provider() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let seed_app =
        local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
            runtime_dir.as_path(),
            Arc::new(StubLocalPrincipalProfileProvider),
        );

    let create_conversation = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_strict_restart_members",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_strict_restart_members/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_missing",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    drop(seed_app);

    let strict_app =
        local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
            runtime_dir.as_path(),
            Arc::new(StrictKnownPrincipalProfileProvider::new(&["u_owner"])),
        );

    let response = strict_app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_strict_restart_members/members")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_missing")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("list members body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("list members body should be valid json");
    assert_eq!(json["code"], "principal_profile_not_found");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_thread_create_rejects_unknown_user_member_after_restart_with_strict_provider() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let seed_app =
        local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
            runtime_dir.as_path(),
            Arc::new(StubLocalPrincipalProfileProvider),
        );

    let create_conversation = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_strict_restart_thread_parent",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed create parent conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_strict_restart_thread_parent/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_missing",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    let post_message = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_strict_restart_thread_parent/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_strict_restart_thread_parent",
                        "summary":"root",
                        "text":"root"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed post parent message should return response");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_message_body = post_message
        .into_body()
        .collect()
        .await
        .expect("seed post parent message body should collect")
        .to_bytes();
    let post_message_json: serde_json::Value = serde_json::from_slice(&post_message_body)
        .expect("seed post parent message body should be valid json");
    let root_message_id = post_message_json["messageId"]
        .as_str()
        .expect("root message id should exist")
        .to_owned();

    drop(seed_app);

    let strict_app =
        local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
            runtime_dir.as_path(),
            Arc::new(StrictKnownPrincipalProfileProvider::new(&["u_owner"])),
        );

    let response = strict_app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/threads")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_missing")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_strict_restart_thread_child",
                        "parentConversationId":"c_strict_restart_thread_parent",
                        "rootMessageId":"{root_message_id}"
                    }}"#
                )))
                .unwrap(),
        )
        .await
        .expect("thread create request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("thread create body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("thread create body should be valid json");
    assert_eq!(json["code"], "principal_profile_not_found");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_friend_request_list_rejects_unknown_user_after_restart_with_strict_provider() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let seed_app =
        local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
            runtime_dir.as_path(),
            Arc::new(StubLocalPrincipalProfileProvider),
        );

    let submit_response = seed_app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_missing")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "targetUserId":"u_owner",
                        "requestMessage":"ghost request"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    drop(seed_app);

    let strict_app =
        local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
            runtime_dir.as_path(),
            Arc::new(StrictKnownPrincipalProfileProvider::new(&["u_owner"])),
        );

    let response = strict_app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/friend_requests?direction=outgoing")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_missing")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request list should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("friend request list body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("friend request list body should be valid json");
    assert_eq!(json["code"], "principal_profile_not_found");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_principal_profile_provider_rejects_oversized_creator_attributes_on_create_conversation()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(OversizedLocalPrincipalProfileProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_principal_profile_local_oversized_creator_attributes",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");

    assert_eq!(create_conversation.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = create_conversation
        .into_body()
        .collect()
        .await
        .expect("create conversation body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("creatorAttributes")
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_principal_profile_provider_rejects_oversized_sender_metadata_on_post_message() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(EscalatingLocalPrincipalProfileProvider::default()),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_principal_profile_local_oversized_sender_metadata",
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
                .uri("/im/v3/api/chat/conversations/c_principal_profile_local_oversized_sender_metadata/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_local_oversized_sender_metadata",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should return response");

    assert_eq!(post_message.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = post_message
        .into_body()
        .collect()
        .await
        .expect("post message body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("senderMetadata")
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_principal_profile_provider_merges_add_member_request_attributes() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StubLocalPrincipalProfileProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_principal_profile_local_member_request_attributes",
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
                .uri("/im/v3/api/chat/conversations/c_principal_profile_local_member_request_attributes/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_other_demo",
                        "principalKind":"user",
                        "role":"member",
                        "attributes":{
                            "project":"apollo"
                        }
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
        "Local u_other_demo"
    );
    assert_eq!(add_member_json["attributes"]["department"], "platform");
    assert_eq!(add_member_json["attributes"]["project"], "apollo");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_external_principal_profile_provider_enriches_user_message_sender_and_member_attributes()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StubExternalPrincipalProfileProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_ext_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_ext_owner")
                .header("x-sdkwork-session-id", "s_ext_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_principal_profile_external",
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
                .uri("/im/v3/api/chat/conversations/c_principal_profile_external/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_ext_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_ext_owner")
                .header("x-sdkwork-session-id", "s_ext_owner")
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
        add_member_json["attributes"]["principalProfilePluginId"],
        "principal-profile-external-catalog"
    );

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_principal_profile_external/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_ext_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_ext_owner")
                .header("x-sdkwork-session-id", "s_ext_owner")
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

    let journal_events = journal_events(runtime_dir.as_path());
    let message_posted = journal_events
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
        payload["sender"]["metadata"]["principalProfilePluginId"],
        "principal-profile-external-catalog"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_principal_profile_provider_enriches_bootstrap_user_members_across_creation_flows()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StubLocalPrincipalProfileProvider),
    );

    let create_group = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_group_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_group_owner")
                .header("x-sdkwork-session-id", "s_group_owner")
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
        "user",
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
        group_owner["attributes"]["principalProfilePluginId"],
        "principal-profile-upstream-context"
    );

    let create_agent_dialog = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_dialog_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_dialog_owner")
                .header("x-sdkwork-session-id", "s_dialog_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_bootstrap_agent_dialog_local",
                        "agentId":"agent.assistant"
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
        "user",
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
        requester["attributes"]["principalProfilePluginId"],
        "principal-profile-upstream-context"
    );

    let create_system_channel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "svc_system")
                .header("x-sdkwork-actor-kind", "system")
                .header("x-sdkwork-device-id", "d_system")
                .header("x-sdkwork-session-id", "s_system")
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
        "system",
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
        subscriber["attributes"]["principalProfilePluginId"],
        "principal-profile-upstream-context"
    );

    let create_agent_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "ag_handoff_source")
                .header("x-sdkwork-actor-kind", "agent")
                .header("x-sdkwork-device-id", "d_handoff_source")
                .header("x-sdkwork-session-id", "s_handoff_source")
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
        "agent",
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
        handoff_target["attributes"]["principalProfilePluginId"],
        "principal-profile-upstream-context"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_principal_profile_provider_rejects_disabled_user_group_create() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(SelectivelyDisabledLocalPrincipalProfileProvider::new(&[
            "u_disabled_creator",
        ])),
    );

    let create_group = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_disabled_creator")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_disabled_creator")
                .header("x-sdkwork-session-id", "s_disabled_creator")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_disabled_creator_group",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("disabled user group create should return response");
    assert_eq!(create_group.status(), StatusCode::FORBIDDEN);
    let create_group_body = create_group
        .into_body()
        .collect()
        .await
        .expect("disabled user group create body should collect")
        .to_bytes();
    let create_group_json: serde_json::Value = serde_json::from_slice(&create_group_body)
        .expect("disabled user group create response should be valid json");
    assert_eq!(create_group_json["code"], "principal_profile_inactive");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_principal_profile_provider_rejects_disabled_user_agent_dialog_create() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(SelectivelyDisabledLocalPrincipalProfileProvider::new(&[
            "u_disabled_dialog_owner",
        ])),
    );

    let create_agent_dialog = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_disabled_dialog_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_disabled_dialog_owner")
                .header("x-sdkwork-session-id", "s_disabled_dialog_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_disabled_creator_agent_dialog",
                        "agentId":"agent.disabled.target"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("disabled user agent dialog create should return response");
    assert_eq!(create_agent_dialog.status(), StatusCode::FORBIDDEN);
    let create_agent_dialog_body = create_agent_dialog
        .into_body()
        .collect()
        .await
        .expect("disabled user agent dialog create body should collect")
        .to_bytes();
    let create_agent_dialog_json: serde_json::Value =
        serde_json::from_slice(&create_agent_dialog_body)
            .expect("disabled user agent dialog create response should be valid json");
    assert_eq!(
        create_agent_dialog_json["code"],
        "principal_profile_inactive"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_external_principal_profile_provider_enriches_bootstrap_user_members_across_creation_flows()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StubExternalPrincipalProfileProvider),
    );

    let create_group = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_group_owner_external")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_group_owner_external")
                .header("x-sdkwork-session-id", "s_group_owner_external")
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
        "user",
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
        group_owner["attributes"]["principalProfilePluginId"],
        "principal-profile-external-catalog"
    );

    let create_system_channel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "svc_system_external")
                .header("x-sdkwork-actor-kind", "system")
                .header("x-sdkwork-device-id", "d_system_external")
                .header("x-sdkwork-session-id", "s_system_external")
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
        "system",
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
        subscriber["attributes"]["principalProfilePluginId"],
        "principal-profile-external-catalog"
    );

    let create_agent_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "ag_handoff_source_external")
                .header("x-sdkwork-actor-kind", "agent")
                .header("x-sdkwork-device-id", "d_handoff_source_external")
                .header("x-sdkwork-session-id", "s_handoff_source_external")
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
        "agent",
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
        handoff_target["attributes"]["principalProfilePluginId"],
        "principal-profile-external-catalog"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_principal_profile_provider_enriches_message_edit_and_recall_actor_metadata() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StubLocalPrincipalProfileProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_editor_local")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_editor_local")
                .header("x-sdkwork-session-id", "s_editor_local")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_principal_profile_edit_recall_local",
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
                .uri("/im/v3/api/chat/conversations/c_principal_profile_edit_recall_local/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_editor_local")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_editor_local")
                .header("x-sdkwork-session-id", "s_editor_local")
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
                .uri(format!("/im/v3/api/chat/messages/{message_id}/edit"))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_editor_local")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_editor_local")
                .header("x-sdkwork-session-id", "s_editor_local")
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
                .uri(format!("/im/v3/api/chat/messages/{message_id}/recall"))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_editor_local")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_editor_local")
                .header("x-sdkwork-session-id", "s_editor_local")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("recall message should succeed");
    assert_eq!(recall_message.status(), StatusCode::OK);

    let journal = journal_events(runtime_dir.as_path());
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
        edited_payload["editor"]["metadata"]["principalProfilePluginId"],
        "principal-profile-upstream-context"
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
        recalled_payload["recalledBy"]["metadata"]["principalProfilePluginId"],
        "principal-profile-upstream-context"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_external_principal_profile_provider_enriches_message_edit_and_recall_actor_metadata()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(StubExternalPrincipalProfileProvider),
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_editor_external")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_editor_external")
                .header("x-sdkwork-session-id", "s_editor_external")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_principal_profile_edit_recall_external",
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
                .uri("/im/v3/api/chat/conversations/c_principal_profile_edit_recall_external/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_editor_external")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_editor_external")
                .header("x-sdkwork-session-id", "s_editor_external")
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
                .uri(format!("/im/v3/api/chat/messages/{message_id}/edit"))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_editor_external")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_editor_external")
                .header("x-sdkwork-session-id", "s_editor_external")
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
                .uri(format!("/im/v3/api/chat/messages/{message_id}/recall"))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_editor_external")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_editor_external")
                .header("x-sdkwork-session-id", "s_editor_external")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("recall message should succeed");
    assert_eq!(recall_message.status(), StatusCode::OK);

    let journal = journal_events(runtime_dir.as_path());
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
        edited_payload["editor"]["metadata"]["principalProfilePluginId"],
        "principal-profile-external-catalog"
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
        recalled_payload["recalledBy"]["metadata"]["principalProfilePluginId"],
        "principal-profile-external-catalog"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
