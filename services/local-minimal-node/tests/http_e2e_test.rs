use axum::body::Body;
use axum::http::{Request, StatusCode};
use fs4::fs_std::FileExt;
use http_body_util::BodyExt;
use im_platform_contracts::{
    ContractError, PrincipalProfile, PrincipalProfileProvider, ProviderDomain,
    ProviderHealthSnapshot, ProviderPluginDescriptor,
};
use projection_service::TimelineProjectionService;
use session_gateway::RealtimeClusterBridge;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, timeout};
use tower::ServiceExt;

static NEXT_TEST_RUNTIME_DIR_SEQUENCE: AtomicU64 = AtomicU64::new(0);

trait AppContextRequestBuilderExt {
    fn demo_app_context(self) -> Self;
    fn other_app_context(self) -> Self;
}

impl AppContextRequestBuilderExt for axum::http::request::Builder {
    fn demo_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_demo")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "sdkwork_iam_session_demo")
    }

    fn other_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_other")
            .header("x-sdkwork-user-id", "u_other")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "sdkwork_iam_session_other")
    }
}

fn deterministic_social_id_for_test(prefix: &str, seed: &str) -> String {
    let digest = Sha256::digest(seed.as_bytes());
    let digest = format!("{digest:x}");
    format!("{prefix}{}", &digest[..24])
}

fn assert_professional_chat_id(chat_id: &str) {
    assert!(
        chat_id.starts_with("cc"),
        "public chat id should use the Craw Chat product id prefix"
    );
    assert!(
        (10..=18).contains(&chat_id.len()),
        "public chat id should be short enough for profile display"
    );
    assert!(
        chat_id
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit()),
        "public chat id should only contain lowercase letters and digits"
    );
    assert!(
        !chat_id.contains('_'),
        "public chat id should avoid technical separators in profile display"
    );
}

fn unique_test_runtime_dir(prefix: &str) -> std::path::PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let sequence = NEXT_TEST_RUNTIME_DIR_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}_{sequence}"))
}

#[derive(Clone)]
struct TestExternalCatalogPrincipalProfileProvider {
    catalog_path: PathBuf,
    default_external_system: String,
}

impl TestExternalCatalogPrincipalProfileProvider {
    fn new(catalog_path: PathBuf, default_external_system: impl Into<String>) -> Self {
        Self {
            catalog_path,
            default_external_system: default_external_system.into(),
        }
    }

    fn load_profiles(&self) -> Result<Vec<PrincipalProfile>, ContractError> {
        let content = fs::read_to_string(&self.catalog_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "test principal-profile catalog unreadable: {} ({error})",
                self.catalog_path.display()
            ))
        })?;
        let catalog: serde_json::Value = serde_json::from_str(&content).map_err(|error| {
            ContractError::Unavailable(format!(
                "test principal-profile catalog invalid json: {} ({error})",
                self.catalog_path.display()
            ))
        })?;
        let catalog_external_system = catalog
            .get("externalSystem")
            .and_then(serde_json::Value::as_str)
            .unwrap_or(self.default_external_system.as_str());
        let profiles = catalog
            .get("profiles")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| {
                ContractError::Unavailable("test principal-profile catalog profiles missing".into())
            })?;

        profiles
            .iter()
            .map(|entry| {
                let attributes = entry
                    .get("attributes")
                    .and_then(serde_json::Value::as_object)
                    .map(|attributes| {
                        attributes
                            .iter()
                            .filter_map(|(key, value)| {
                                value.as_str().map(|text| (key.clone(), text.to_owned()))
                            })
                            .collect::<BTreeMap<_, _>>()
                    })
                    .unwrap_or_default();
                Ok(PrincipalProfile {
                    tenant_id: required_catalog_field(entry, "tenantId")?,
                    principal_id: required_catalog_field(entry, "principalId")?,
                    display_name: required_catalog_field(entry, "displayName")?,
                    external_system: Some(
                        entry
                            .get("externalSystem")
                            .and_then(serde_json::Value::as_str)
                            .unwrap_or(catalog_external_system)
                            .to_owned(),
                    ),
                    external_principal_id: entry
                        .get("externalPrincipalId")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned),
                    attributes,
                    inactive: entry
                        .get("inactive")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false),
                })
            })
            .collect()
    }
}

fn required_catalog_field(entry: &serde_json::Value, field: &str) -> Result<String, ContractError> {
    entry
        .get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| ContractError::Unavailable(format!("catalog field missing: {field}")))
}

fn test_profile_matches_search_keyword(profile: &PrincipalProfile, keyword: &str) -> bool {
    let keyword = keyword.trim();
    if keyword.is_empty() {
        return true;
    }
    let keyword_lower = keyword.to_ascii_lowercase();
    contains_case_insensitive(profile.principal_id.as_str(), keyword_lower.as_str())
        || contains_case_insensitive(profile.display_name.as_str(), keyword_lower.as_str())
        || profile
            .external_principal_id
            .as_deref()
            .is_some_and(|principal_id| {
                contains_case_insensitive(principal_id, keyword_lower.as_str())
            })
        || ["email", "phone", "phoneNumber", "mobile"]
            .into_iter()
            .any(|key| {
                profile
                    .attributes
                    .get(key)
                    .is_some_and(|value| contains_case_insensitive(value, keyword_lower.as_str()))
            })
}

fn contains_case_insensitive(candidate: &str, keyword_lower: &str) -> bool {
    candidate.to_ascii_lowercase().contains(keyword_lower)
}

impl PrincipalProfileProvider for TestExternalCatalogPrincipalProfileProvider {
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
        principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(self.load_profiles()?.into_iter().find(|profile| {
            profile.tenant_id == tenant_id
                && profile.principal_id == principal_id
                && principal_kind == "user"
        }))
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(self
            .load_profiles()?
            .into_iter()
            .filter(|profile| {
                profile.tenant_id == tenant_id
                    && principal_kind == "user"
                    && principal_ids.contains(&profile.principal_id)
            })
            .collect())
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(self
            .load_profiles()?
            .into_iter()
            .filter(|profile| profile.tenant_id == tenant_id && principal_kind == "user")
            .filter(|profile| test_profile_matches_search_keyword(profile, keyword))
            .collect())
    }

    fn map_external_principal(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(self.load_profiles()?.into_iter().find(|profile| {
            profile.tenant_id == tenant_id
                && principal_kind == "user"
                && profile.external_system.as_deref() == Some(external_system)
                && profile.external_principal_id.as_deref() == Some(external_principal_id)
        }))
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot {
            plugin_id: "principal-profile-external-catalog".into(),
            status: "healthy".into(),
            checked_at: "2026-04-09T00:00:00Z".into(),
            details: BTreeMap::from([
                ("providerKind".into(), "external-catalog".into()),
                (
                    "catalogPath".into(),
                    self.catalog_path.display().to_string(),
                ),
            ]),
        }
    }
}

struct ScopedEnvVar {
    key: &'static str,
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        unsafe {
            std::env::remove_var(self.key);
        }
    }
}

fn set_scoped_env_var(key: &'static str, value: &str) -> ScopedEnvVar {
    unsafe {
        std::env::set_var(key, value);
    }
    ScopedEnvVar { key }
}

fn social_accept_delay_env_guard() -> &'static tokio::sync::Mutex<()> {
    static SOCIAL_ACCEPT_DELAY_ENV_GUARD: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    SOCIAL_ACCEPT_DELAY_ENV_GUARD.get_or_init(|| tokio::sync::Mutex::new(()))
}

async fn lock_social_accept_delay_env_guard() -> tokio::sync::MutexGuard<'static, ()> {
    social_accept_delay_env_guard().lock().await
}

fn friendship_activated_event(
    tenant_id: &str,
    friendship_id: &str,
    user_low_id: &str,
    user_high_id: &str,
    direct_chat_id: Option<&str>,
    established_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{friendship_id}_friendship"),
        tenant_id,
        "friendship.activated",
        "friendship",
        friendship_id,
        1,
    )
    .with_payload(
        "social.friendship.activated.v1",
        &serde_json::json!({
            "friendshipId": friendship_id,
            "userLowId": user_low_id,
            "userHighId": user_high_id,
            "initiatorUserId": user_low_id,
            "directChatId": direct_chat_id,
            "establishedAt": established_at,
        })
        .to_string(),
    )
}

fn direct_chat_bound_event(
    tenant_id: &str,
    direct_chat_id: &str,
    conversation_id: &str,
    bound_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{direct_chat_id}_bound"),
        tenant_id,
        "direct_chat.bound",
        "direct_chat",
        direct_chat_id,
        1,
    )
    .with_payload(
        "social.direct_chat.bound.v1",
        &serde_json::json!({
            "directChatId": direct_chat_id,
            "conversationId": conversation_id,
            "leftActorId": "actor_alice",
            "rightActorId": "actor_bob",
            "pairHash": "actor_alice:actor_bob",
            "boundAt": bound_at,
        })
        .to_string(),
    )
}

struct ActiveFriendshipDirectChatFixture {
    friendship_id: String,
    direct_chat_id: String,
    conversation_id: String,
}

async fn create_active_friendship_direct_chat_fixture(
    app: &axum::Router,
) -> ActiveFriendshipDirectChatFixture {
    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("friend request submit body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("friend request submit body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("friend request submit should return request id")
        .to_owned();

    let accept_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request accept should return response");
    assert_eq!(accept_request.status(), StatusCode::OK);
    let accept_request_body = accept_request
        .into_body()
        .collect()
        .await
        .expect("friend request accept body should collect")
        .to_bytes();
    let accept_request_json: serde_json::Value = serde_json::from_slice(&accept_request_body)
        .expect("friend request accept body should be valid json");

    ActiveFriendshipDirectChatFixture {
        friendship_id: accept_request_json["friendship"]["friendshipId"]
            .as_str()
            .expect("friend request accept should return friendship id")
            .to_owned(),
        direct_chat_id: accept_request_json["directChat"]["directChatId"]
            .as_str()
            .expect("friend request accept should return direct chat id")
            .to_owned(),
        conversation_id: accept_request_json["directChat"]["conversationId"]
            .as_str()
            .expect("friend request accept should return direct chat conversation id")
            .to_owned(),
    }
}

async fn create_active_friendship_direct_chat(app: &axum::Router) -> (String, String) {
    let fixture = create_active_friendship_direct_chat_fixture(app).await;
    (fixture.friendship_id, fixture.conversation_id)
}

#[tokio::test]
async fn test_local_minimal_profile_social_user_search_uses_real_catalog_without_mocking_input() {
    let default_app = local_minimal_node::build_default_app();
    let default_search = default_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/users?q=alice&limit=20")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("default social user search should return response");
    assert_eq!(default_search.status(), StatusCode::OK);
    let default_search_body = default_search
        .into_body()
        .collect()
        .await
        .expect("default social user search body should collect")
        .to_bytes();
    let default_search_json: serde_json::Value = serde_json::from_slice(&default_search_body)
        .expect("default social user search body should be valid json");
    assert_eq!(default_search_json["items"], serde_json::json!([]));
    assert_eq!(default_search_json["hasMore"], false);

    let default_current_search = default_app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/users?q=user_test005_a_com&limit=20")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "user_test005_a_com")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-session-id", "sdkwork_iam_session_test005")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("default current social user search should return response");
    assert_eq!(default_current_search.status(), StatusCode::OK);
    let default_current_search_body = default_current_search
        .into_body()
        .collect()
        .await
        .expect("default current social user search body should collect")
        .to_bytes();
    let default_current_search_json: serde_json::Value =
        serde_json::from_slice(&default_current_search_body)
            .expect("default current social user search body should be valid json");
    let default_current_items = default_current_search_json["items"]
        .as_array()
        .expect("default current social user search should return items");
    assert_eq!(default_current_items.len(), 1);
    assert_eq!(default_current_items[0]["userId"], "user_test005_a_com");
    assert_eq!(default_current_items[0]["relationshipState"], "self");
    let default_current_chat_id = default_current_items[0]["chatId"]
        .as_str()
        .expect("default current user search must expose a public chat id");
    assert_professional_chat_id(default_current_chat_id);
    assert_ne!(default_current_chat_id, "user_test005_a_com");

    let runtime_dir = unique_test_runtime_dir("social_user_search_external_catalog");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let catalog_path = runtime_dir.join("principal-profile-catalog.json");
    fs::write(
        &catalog_path,
        serde_json::json!({
            "externalSystem": "corp-idp",
            "profiles": [
                {
                    "tenantId": "t_demo",
                    "principalId": "u_demo",
                    "principalKind": "user",
                    "displayName": "Demo User",
                    "externalPrincipalId": "demo",
                    "attributes": {
                        "email": "demo@example.com"
                    }
                },
                {
                    "tenantId": "t_demo",
                    "principalId": "u_alice",
                    "principalKind": "user",
                    "displayName": "Alice Chen",
                    "externalPrincipalId": "alice",
                    "attributes": {
                        "avatarUrl": "https://example.com/alice.png",
                        "email": "alice@example.com",
                        "phone": "+12025550100"
                    }
                },
                {
                    "tenantId": "t_demo",
                    "principalId": "user_test005_a_com",
                    "principalKind": "user",
                    "displayName": "Test 005",
                    "externalPrincipalId": "user_test005_a_com",
                    "attributes": {
                        "email": "test005@a.com"
                    }
                }
            ]
        })
        .to_string(),
    )
    .expect("principal profile catalog should be written");

    let app = local_minimal_node::build_default_app_with_runtime_dir_and_principal_profile_provider(
        runtime_dir.as_path(),
        Arc::new(TestExternalCatalogPrincipalProfileProvider::new(
            catalog_path.clone(),
            "corp-idp",
        )),
    );

    let current_user_search = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/users?q=u_demo&limit=20")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("current social user search should return response");
    assert_eq!(current_user_search.status(), StatusCode::OK);
    let current_user_search_body = current_user_search
        .into_body()
        .collect()
        .await
        .expect("current social user search body should collect")
        .to_bytes();
    let current_user_search_json: serde_json::Value =
        serde_json::from_slice(&current_user_search_body)
            .expect("current social user search body should be valid json");
    let current_user_items = current_user_search_json["items"]
        .as_array()
        .expect("current social user search should return items");
    assert_eq!(current_user_items.len(), 1);
    assert_eq!(current_user_items[0]["userId"], "u_demo");
    assert_eq!(current_user_items[0]["relationshipState"], "self");
    let current_user_chat_id = current_user_items[0]["chatId"]
        .as_str()
        .expect("current user search must expose a public chat id");
    assert_professional_chat_id(current_user_chat_id);

    let search = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/users?q=alice&limit=20")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("external-catalog social user search should return response");
    assert_eq!(search.status(), StatusCode::OK);
    let search_body = search
        .into_body()
        .collect()
        .await
        .expect("external-catalog social user search body should collect")
        .to_bytes();
    let search_json: serde_json::Value = serde_json::from_slice(&search_body)
        .expect("external-catalog social user search body should be valid json");
    let items = search_json["items"]
        .as_array()
        .expect("social user search should return items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["tenantId"], "t_demo");
    assert_eq!(items[0]["userId"], "u_alice");
    let alice_chat_id = items[0]["chatId"]
        .as_str()
        .expect("social user search must expose a public chat id");
    assert_professional_chat_id(alice_chat_id);
    assert_ne!(
        alice_chat_id, "u_alice",
        "public chat id must not expose the internal user id"
    );
    assert_eq!(items[0]["displayName"], "Alice Chen");
    assert_eq!(items[0]["relationshipState"], "none");
    assert_eq!(items[0]["avatarUrl"], "https://example.com/alice.png");
    assert_eq!(items[0]["email"], "alice@example.com");
    assert_eq!(items[0]["phone"], "+12025550100");
    assert_eq!(search_json["hasMore"], false);

    let chat_id_search = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/social/users?q={alice_chat_id}&limit=20"
                ))
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("external-catalog social user public id search should return response");
    assert_eq!(chat_id_search.status(), StatusCode::OK);
    let chat_id_search_body = chat_id_search
        .into_body()
        .collect()
        .await
        .expect("external-catalog public id search body should collect")
        .to_bytes();
    let chat_id_search_json: serde_json::Value = serde_json::from_slice(&chat_id_search_body)
        .expect("external-catalog public id search body should be valid json");
    let chat_id_items = chat_id_search_json["items"]
        .as_array()
        .expect("public id search should return items");
    assert_eq!(chat_id_items.len(), 1);
    assert_eq!(chat_id_items[0]["userId"], "u_alice");
    assert_eq!(chat_id_items[0]["chatId"], alice_chat_id);

    for (query, label) in [("alice@example.com", "email"), ("%2B12025550100", "phone")] {
        let search_by_alias = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(format!("/im/v3/api/social/users?q={query}&limit=20"))
                    .demo_app_context()
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap_or_else(|_| panic!("{label} social user search should return response"));
        assert_eq!(search_by_alias.status(), StatusCode::OK);
        let search_by_alias_body = search_by_alias
            .into_body()
            .collect()
            .await
            .unwrap_or_else(|_| panic!("{label} social user search body should collect"))
            .to_bytes();
        let search_by_alias_json: serde_json::Value = serde_json::from_slice(&search_by_alias_body)
            .unwrap_or_else(|_| panic!("{label} social user search body should be valid json"));
        let alias_items = search_by_alias_json["items"]
            .as_array()
            .unwrap_or_else(|| panic!("{label} social user search should return items"));
        assert_eq!(alias_items.len(), 1);
        assert_eq!(alias_items[0]["userId"], "u_alice");
        assert_eq!(alias_items[0]["chatId"], alice_chat_id);
    }

    let email_like_id_search = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/users?q=user_test005_a_com&limit=20")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("email-like internal user id search should return response");
    assert_eq!(email_like_id_search.status(), StatusCode::OK);
    let email_like_id_search_body = email_like_id_search
        .into_body()
        .collect()
        .await
        .expect("email-like internal user id search body should collect")
        .to_bytes();
    let email_like_id_search_json: serde_json::Value =
        serde_json::from_slice(&email_like_id_search_body)
            .expect("email-like internal user id search body should be valid json");
    let email_like_items = email_like_id_search_json["items"]
        .as_array()
        .expect("email-like internal user id search should return items");
    assert_eq!(email_like_items.len(), 1);
    assert_eq!(email_like_items[0]["userId"], "user_test005_a_com");
    let email_like_chat_id = email_like_items[0]["chatId"]
        .as_str()
        .expect("email-like internal user must receive a public chat id");
    assert_professional_chat_id(email_like_chat_id);
    assert_ne!(
        email_like_chat_id, "user_test005_a_com",
        "public chat id must not equal an email-derived internal user id"
    );
    assert!(
        !email_like_chat_id.contains("test005") && !email_like_chat_id.contains("_com"),
        "public chat id must not visibly leak email-derived account fragments"
    );

    let email_like_public_id_search = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/social/users?q={email_like_chat_id}&limit=20"
                ))
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("email-like user's public chat id search should return response");
    assert_eq!(email_like_public_id_search.status(), StatusCode::OK);
    let email_like_public_id_search_body = email_like_public_id_search
        .into_body()
        .collect()
        .await
        .expect("email-like public id search body should collect")
        .to_bytes();
    let email_like_public_id_search_json: serde_json::Value =
        serde_json::from_slice(&email_like_public_id_search_body)
            .expect("email-like public id search body should be valid json");
    let email_like_public_items = email_like_public_id_search_json["items"]
        .as_array()
        .expect("email-like public id search should return items");
    assert_eq!(email_like_public_items.len(), 1);
    assert_eq!(email_like_public_items[0]["userId"], "user_test005_a_com");
    assert_eq!(email_like_public_items[0]["chatId"], email_like_chat_id);

    let submit_request = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/friend_requests")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "targetUserId": "u_alice",
                        "requestMessage": "hello alice"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("friend request to searched catalog user should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("friend request to searched catalog user body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("friend request to searched catalog user body should be valid json");
    assert_eq!(
        submit_request_json["friendRequest"]["targetUserId"],
        "u_alice"
    );
    assert_eq!(submit_request_json["friendRequest"]["status"], "pending");
}

async fn remove_friendship_for_test(
    app: &axum::Router,
    friendship_id: &str,
    remover_user_id: &str,
) {
    let remove_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friendships/{friendship_id}/remove"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", remover_user_id)
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship removal should return response");
    assert_eq!(remove_request.status(), StatusCode::OK);
}

async fn block_direct_chat_for_test(
    app: &axum::Router,
    block_id: &str,
    blocker_user_id: &str,
    blocked_user_id: &str,
    direct_chat_id: &str,
) {
    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "blockId": block_id,
                        "eventId": format!("evt_{block_id}"),
                        "blockerUserId": blocker_user_id,
                        "blockedUserId": blocked_user_id,
                        "scope": "direct_chat",
                        "directChatId": direct_chat_id,
                        "effectiveAt": "2026-04-15T10:20:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat block should return response");
    assert_eq!(block_response.status(), StatusCode::OK);
}

async fn post_standard_message_for_test(
    app: &axum::Router,
    conversation_id: &str,
    sender_user_id: &str,
    client_msg_id: &str,
    summary: &str,
) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", sender_user_id)
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "clientMsgId": client_msg_id,
                        "summary": summary,
                        "text": summary,
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("post message should return response")
}

async fn list_message_summaries_for_test(
    app: &axum::Router,
    conversation_id: &str,
    reader_user_id: &str,
) -> Vec<String> {
    let timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", reader_user_id)
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline request should return response");
    assert_eq!(timeline.status(), StatusCode::OK);
    let timeline_body = timeline
        .into_body()
        .collect()
        .await
        .expect("timeline body should collect")
        .to_bytes();
    let timeline_json: serde_json::Value =
        serde_json::from_slice(&timeline_body).expect("timeline body should be valid json");
    timeline_json["items"]
        .as_array()
        .expect("timeline items should be an array")
        .iter()
        .filter_map(|item| item["summary"].as_str().map(ToOwned::to_owned))
        .collect()
}

async fn register_client_route_for_test(app: &axum::Router, user_id: &str, device_id: &str) {
    let register = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", user_id)
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", device_id)
                .header("content-type", "application/json")
                .body(Body::from(format!(r#"{{"deviceId":"{device_id}"}}"#)))
                .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register.status(), StatusCode::OK);
}

async fn sync_conversation_realtime_subscription_for_test(
    app: &axum::Router,
    user_id: &str,
    device_id: &str,
    conversation_id: &str,
) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", user_id)
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", device_id)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "items": [
                            {
                                "scopeType": "conversation",
                                "scopeId": conversation_id,
                                "eventTypes": ["message.posted"],
                            }
                        ]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("realtime subscription sync should return response")
}

async fn sync_user_realtime_subscription_for_test(
    app: &axum::Router,
    user_id: &str,
    device_id: &str,
    event_types: &[&str],
) -> axum::response::Response {
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", user_id)
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", device_id)
                .header("x-sdkwork-session-id", format!("s_{device_id}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "items": [
                            {
                                "scopeType": "user",
                                "scopeId": user_id,
                                "eventTypes": event_types,
                            }
                        ]
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("user realtime subscription sync should return response")
}

async fn list_realtime_events_for_test(
    app: &axum::Router,
    user_id: &str,
    device_id: &str,
    after_seq: u64,
) -> axum::response::Response {
    list_realtime_events_with_session_for_test(app, user_id, device_id, None, after_seq).await
}

async fn list_realtime_events_with_session_for_test(
    app: &axum::Router,
    user_id: &str,
    device_id: &str,
    session_id: Option<&str>,
    after_seq: u64,
) -> axum::response::Response {
    let mut request = Request::builder()
        .uri(format!(
            "/im/v3/api/realtime/events?afterSeq={after_seq}&limit=10"
        ))
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", user_id)
        .header("x-sdkwork-actor-kind", "user")
        .header("x-sdkwork-device-id", device_id);

    if let Some(session_id) = session_id {
        request = request.header("x-sdkwork-session-id", session_id);
    }

    app.clone()
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .expect("realtime events request should return response")
}

fn message_reaction_added_event(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    reaction_key: &str,
    actor_id: &str,
    reacted_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{message_id}_{reaction_key}_{actor_id}_reaction_added"),
        tenant_id,
        "message.reaction_added",
        "conversation",
        conversation_id,
        message_seq + 1,
    )
    .with_payload(
        "message.reaction_added.v1",
        &serde_json::json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "reactionKey": reaction_key,
            "reactedBy": {
                "id": actor_id,
                "kind": "user",
                "memberId": format!("cm_{actor_id}"),
                "deviceId": format!("d_{actor_id}"),
                "sessionId": format!("s_{actor_id}"),
                "metadata": {}
            },
            "reactedAt": reacted_at
        })
        .to_string(),
    )
}

fn message_pinned_event(
    tenant_id: &str,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    actor_id: &str,
    pinned_at: &str,
) -> im_domain_events::CommitEnvelope {
    im_domain_events::CommitEnvelope::minimal(
        &format!("evt_{tenant_id}_{message_id}_{actor_id}_pin_added"),
        tenant_id,
        "message.pin_added",
        "conversation",
        conversation_id,
        message_seq + 2,
    )
    .with_payload(
        "message.pin_added.v1",
        &serde_json::json!({
            "tenantId": tenant_id,
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "pinnedBy": {
                "id": actor_id,
                "kind": "user",
                "memberId": format!("cm_{actor_id}"),
                "deviceId": format!("d_{actor_id}"),
                "sessionId": format!("s_{actor_id}"),
                "metadata": {}
            },
            "pinnedAt": pinned_at
        })
        .to_string(),
    )
}

#[tokio::test]
async fn test_local_minimal_profile_runs_end_to_end_flow() {
    let app = local_minimal_node::build_default_app();

    let health = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("healthz should succeed");
    assert_eq!(health.status(), StatusCode::OK);

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_demo",
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
                .uri("/im/v3/api/chat/conversations/c_demo/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_demo",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_demo/messages")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should succeed");
    assert_eq!(timeline.status(), StatusCode::OK);
    let timeline_body = timeline
        .into_body()
        .collect()
        .await
        .expect("timeline body should collect")
        .to_bytes();
    let timeline_json: serde_json::Value =
        serde_json::from_slice(&timeline_body).expect("timeline should be valid json");
    assert_eq!(timeline_json["items"][0]["messageId"], "msg_c_demo_1");
    assert_eq!(timeline_json["items"][0]["summary"], "hello");

    let conversation_summary = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_demo")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary should succeed");
    assert_eq!(conversation_summary.status(), StatusCode::OK);
    let conversation_summary_body = conversation_summary
        .into_body()
        .collect()
        .await
        .expect("conversation summary body should collect")
        .to_bytes();
    let conversation_summary_json: serde_json::Value =
        serde_json::from_slice(&conversation_summary_body)
            .expect("conversation summary should be valid json");
    assert_eq!(conversation_summary_json["lastMessageId"], "msg_c_demo_1");
    assert_eq!(conversation_summary_json["messageCount"], 1);

    let create_conversation_other_tenant = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .other_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation for other tenant should succeed");
    assert_eq!(create_conversation_other_tenant.status(), StatusCode::OK);

    let post_message_other_tenant = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_demo/messages")
                .other_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_other",
                        "summary":"other",
                        "text":"other"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message for other tenant should succeed");
    assert_eq!(post_message_other_tenant.status(), StatusCode::OK);

    let other_timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_demo/messages")
                .other_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("other tenant timeline should succeed");
    assert_eq!(other_timeline.status(), StatusCode::OK);
    let other_timeline_body = other_timeline
        .into_body()
        .collect()
        .await
        .expect("other tenant timeline body should collect")
        .to_bytes();
    let other_timeline_json: serde_json::Value =
        serde_json::from_slice(&other_timeline_body).expect("other timeline should be valid json");
    assert_eq!(other_timeline_json["items"][0]["summary"], "other");
    assert_eq!(other_timeline_json["items"].as_array().unwrap().len(), 1);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_demo",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);
    let open_stream_body = open_stream
        .into_body()
        .collect()
        .await
        .expect("open stream body should collect")
        .to_bytes();
    let open_stream_json: serde_json::Value =
        serde_json::from_slice(&open_stream_body).expect("open stream should be valid json");
    assert_eq!(open_stream_json["state"], "opened");

    let checkpoint_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_demo/checkpoint")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("checkpoint stream should succeed");
    assert_eq!(checkpoint_stream.status(), StatusCode::OK);

    let complete_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_demo/complete")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 5,
                        "resultMessageId": "msg_c_demo_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream should succeed");
    assert_eq!(complete_stream.status(), StatusCode::OK);

    let open_abort_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_abort",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open abort stream should succeed");
    assert_eq!(open_abort_stream.status(), StatusCode::OK);

    let abort_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort/abort")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "reason": "client_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("abort stream should succeed");
    assert_eq!(abort_stream.status(), StatusCode::OK);
    let abort_stream_body = abort_stream
        .into_body()
        .collect()
        .await
        .expect("abort stream body should collect")
        .to_bytes();
    let abort_stream_json: serde_json::Value =
        serde_json::from_slice(&abort_stream_body).expect("abort stream should be valid json");
    assert_eq!(abort_stream_json["state"], "aborted");
    assert_eq!(abort_stream_json["lastFrameSeq"], 2);
    assert_eq!(
        abort_stream_json["resultMessageId"],
        serde_json::Value::Null
    );

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_demo",
                        "conversationId":"c_demo",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let invite_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_demo/invite")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId": "st_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invite rtc should succeed");
    assert_eq!(invite_rtc.status(), StatusCode::OK);

    let custom_signal = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_demo/signals")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("custom signal should succeed");
    assert_eq!(custom_signal.status(), StatusCode::OK);

    let accept_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_demo/accept")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId": "msg_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept rtc should succeed");
    assert_eq!(accept_rtc.status(), StatusCode::OK);

    let end_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_demo/end")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId": "msg_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("end rtc should succeed");
    assert_eq!(end_rtc.status(), StatusCode::OK);

    let post_media_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_demo/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_attach",
                        "summary":"poster asset",
                        "parts":[
                            {"kind":"text","text":"see attachment"},
                            {
                                "kind":"media",
                                "mediaRole":"attachment",
                                "drive":{
                                    "driveUri":"drive://spaces/space_app_upload_demo/nodes/node_ma_demo",
                                    "spaceId":"space_app_upload_demo",
                                    "nodeId":"node_ma_demo"
                                },
                                "resource":{
                                    "id":"node_ma_demo",
                                    "kind":"image",
                                    "source":"provider_asset",
                                    "uri":"drive://spaces/space_app_upload_demo/nodes/node_ma_demo",
                                    "mimeType":"image/png",
                                    "sizeBytes":"42",
                                    "fileName":"demo.png",
                                    "metadata":{"origin":"e2e"},
                                    "title":"poster"
                                }
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("drive-backed media message should succeed");
    assert_eq!(post_media_message.status(), StatusCode::OK);

    let conversation_summary_after_attach = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_demo")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary after attach should succeed");
    assert_eq!(conversation_summary_after_attach.status(), StatusCode::OK);
    let conversation_summary_after_attach_body = conversation_summary_after_attach
        .into_body()
        .collect()
        .await
        .expect("conversation summary after attach body should collect")
        .to_bytes();
    let conversation_summary_after_attach_json: serde_json::Value =
        serde_json::from_slice(&conversation_summary_after_attach_body)
            .expect("conversation summary after attach should be valid json");
    assert_eq!(
        conversation_summary_after_attach_json["lastMessageId"],
        "msg_c_demo_6"
    );
    assert_eq!(conversation_summary_after_attach_json["messageCount"], 6);
    assert_eq!(
        conversation_summary_after_attach_json["lastSummary"],
        "poster asset"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_returns_bounded_timeline_cursor_window() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_timeline_page_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for seq in 1..=2 {
        let post_message = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations/c_timeline_page_local/messages")
                    .demo_app_context()
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "clientMsgId":"client_timeline_page_local_{seq}",
                            "summary":"message {seq}",
                            "text":"message {seq}"
                        }}"#
                    )))
                    .unwrap(),
            )
            .await
            .expect("post message should succeed");
        assert_eq!(post_message.status(), StatusCode::OK);
    }

    let first_page = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_timeline_page_local/messages?afterSeq=0&limit=1")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first timeline page should return response");
    assert_eq!(first_page.status(), StatusCode::OK);
    let first_page_body = first_page
        .into_body()
        .collect()
        .await
        .expect("first timeline page body should collect")
        .to_bytes();
    let first_page_json: serde_json::Value =
        serde_json::from_slice(&first_page_body).expect("first page should be valid json");
    assert_eq!(first_page_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(first_page_json["items"][0]["messageSeq"], 1);
    assert_eq!(first_page_json["nextAfterSeq"], 1);
    assert_eq!(first_page_json["hasMore"], true);

    let second_page = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_timeline_page_local/messages?afterSeq=1&limit=1")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second timeline page should return response");
    assert_eq!(second_page.status(), StatusCode::OK);
    let second_page_body = second_page
        .into_body()
        .collect()
        .await
        .expect("second timeline page body should collect")
        .to_bytes();
    let second_page_json: serde_json::Value =
        serde_json::from_slice(&second_page_body).expect("second page should be valid json");
    assert_eq!(second_page_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(second_page_json["items"][0]["messageSeq"], 2);
    assert_eq!(second_page_json["nextAfterSeq"], 2);
    assert_eq!(second_page_json["hasMore"], false);

    let invalid_limit = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_timeline_page_local/messages?afterSeq=0&limit=0")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid limit request should return response");
    assert_eq!(invalid_limit.status(), StatusCode::BAD_REQUEST);
    let invalid_limit_body = invalid_limit
        .into_body()
        .collect()
        .await
        .expect("invalid limit body should collect")
        .to_bytes();
    let invalid_limit_json: serde_json::Value =
        serde_json::from_slice(&invalid_limit_body).expect("invalid limit should be valid json");
    assert_eq!(invalid_limit_json["code"], "limit_invalid");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_create_conversation_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_create_retry_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_create_retry_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_create_retry_local/members")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    assert_eq!(members_json["items"].as_array().unwrap().len(), 1);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_create_retry_local",
                        "conversationType":"direct"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_non_standard_agent_dialog_agent_id() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_invalid_agent_id",
                        "agentId":"ag_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invalid agent dialog create should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("invalid agent dialog create body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("invalid agent dialog create should be valid json");
    assert_eq!(value["code"], "agent_id_invalid");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_agent_dialog_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_retry_local",
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first agent dialog create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first agent dialog create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first agent dialog create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "6#t_demo4#user6#u_demo19#create-agent-dialog26#c_agent_dialog_retry_local"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_retry_local",
                        "agentId":"agent.demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate agent dialog create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate agent dialog create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate agent dialog create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_agent_dialog_retry_local/members")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    assert_eq!(members_json["items"].as_array().unwrap().len(), 2);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_dialogs")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_dialog_retry_local",
                        "agentId":"agent.other"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting agent dialog create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting agent dialog create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting agent dialog create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_system_channel_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "svc_ops")
                .header("x-sdkwork-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_retry_local",
                        "subscriberId":"u_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first system channel create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first system channel create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first system channel create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "6#t_demo6#system7#svc_ops21#create-system_channel28#c_system_channel_retry_local"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "svc_ops")
                .header("x-sdkwork-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_retry_local",
                        "subscriberId":"u_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate system channel create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate system channel create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate system channel create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_system_channel_retry_local/members")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "svc_ops")
                .header("x-sdkwork-actor-kind", "system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    assert_eq!(members_json["items"].as_array().unwrap().len(), 2);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "svc_ops")
                .header("x-sdkwork-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_retry_local",
                        "subscriberId":"u_other"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting system channel create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting system channel create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting system channel create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_system_channel_publish_from_subscriber_user() {
    let app = local_minimal_node::build_default_app();

    let create_channel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/system_channels")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "svc_ops")
                .header("x-sdkwork-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_system_channel_publish_guard_local",
                        "subscriberId":"u_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel create should return response");
    assert_eq!(create_channel.status(), StatusCode::OK);

    register_client_route_for_test(&app, "u_demo", "d_demo_phone").await;

    let publish_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_system_channel_publish_guard_local/system_channel/publish")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_demo_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_system_channel_publish_guard_1",
                        "summary":"should fail",
                        "text":"should fail"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system channel publish as subscriber should return response");
    assert_eq!(publish_response.status(), StatusCode::FORBIDDEN);
    let publish_body = publish_response
        .into_body()
        .collect()
        .await
        .expect("system channel publish forbidden body should collect")
        .to_bytes();
    let publish_json: serde_json::Value = serde_json::from_slice(&publish_body)
        .expect("system channel publish forbidden body should be valid json");
    assert_eq!(publish_json["code"], "conversation_permission_denied");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_agent_handoff_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "ag_source")
                .header("x-sdkwork-actor-kind", "agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_retry_local",
                        "targetId":"u_demo",
                        "targetKind":"user",
                        "handoffSessionId":"hs_retry_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first agent handoff create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first agent handoff create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first agent handoff create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "6#t_demo5#agent9#ag_source20#create-agent_handoff27#c_agent_handoff_retry_local"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "ag_source")
                .header("x-sdkwork-actor-kind", "agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_retry_local",
                        "targetId":"u_demo",
                        "targetKind":"user",
                        "handoffSessionId":"hs_retry_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate agent handoff create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate agent handoff create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate agent handoff create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_agent_handoff_retry_local/members")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "ag_source")
                .header("x-sdkwork-actor-kind", "agent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    assert_eq!(members_json["items"].as_array().unwrap().len(), 2);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "ag_source")
                .header("x-sdkwork-actor-kind", "agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_agent_handoff_retry_local",
                        "targetId":"u_other",
                        "targetKind":"user",
                        "handoffSessionId":"hs_retry_local",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting agent handoff create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting agent handoff create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting agent handoff create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_thread_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let create_parent = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_parent_thread_retry_local",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create parent conversation should return response");
    assert_eq!(create_parent.status(), StatusCode::OK);

    let first_root = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_parent_thread_retry_local/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_thread_retry_local_root_1",
                        "summary":"root-1",
                        "text":"root-1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first root message should return response");
    assert_eq!(first_root.status(), StatusCode::OK);
    let first_root_body = first_root
        .into_body()
        .collect()
        .await
        .expect("first root body should collect")
        .to_bytes();
    let first_root_json: serde_json::Value =
        serde_json::from_slice(&first_root_body).expect("first root should be valid json");

    let second_root = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_parent_thread_retry_local/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_thread_retry_local_root_2",
                        "summary":"root-2",
                        "text":"root-2"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second root message should return response");
    assert_eq!(second_root.status(), StatusCode::OK);
    let second_root_body = second_root
        .into_body()
        .collect()
        .await
        .expect("second root body should collect")
        .to_bytes();
    let second_root_json: serde_json::Value =
        serde_json::from_slice(&second_root_body).expect("second root should be valid json");

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/threads")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_retry_local",
                        "parentConversationId":"c_parent_thread_retry_local",
                        "rootMessageId":"{}"
                    }}"#,
                    first_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("first thread create should return response");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first thread create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value = serde_json::from_slice(&first_create_body)
        .expect("first thread create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_create_json["requestKey"],
        "6#t_demo4#user6#u_demo13#create-thread20#c_thread_retry_local"
    );

    let duplicate_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/threads")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_retry_local",
                        "parentConversationId":"c_parent_thread_retry_local",
                        "rootMessageId":"{}"
                    }}"#,
                    first_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("duplicate thread create should return response");
    assert_eq!(duplicate_create.status(), StatusCode::OK);
    let duplicate_create_body = duplicate_create
        .into_body()
        .collect()
        .await
        .expect("duplicate thread create body should collect")
        .to_bytes();
    let duplicate_create_json: serde_json::Value = serde_json::from_slice(&duplicate_create_body)
        .expect("duplicate thread create should be valid json");
    assert_eq!(duplicate_create_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_create_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        duplicate_create_json["eventId"],
        first_create_json["eventId"]
    );

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_thread_retry_local/members")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    assert_eq!(members_json["items"].as_array().unwrap().len(), 1);

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/threads")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{
                        "conversationId":"c_thread_retry_local",
                        "parentConversationId":"c_parent_thread_retry_local",
                        "rootMessageId":"{}"
                    }}"#,
                    second_root_json["messageId"].as_str().unwrap()
                )))
                .unwrap(),
        )
        .await
        .expect("conflicting thread create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_create_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting thread create body should collect")
        .to_bytes();
    let conflicting_create_json: serde_json::Value =
        serde_json::from_slice(&conflicting_create_body)
            .expect("conflicting thread create should be valid json");
    assert_eq!(conflicting_create_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_direct_chat_binding_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_bind = app
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
                        "conversationId":"c_direct_retry_local",
                        "directChatId":"dc_retry_local",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first direct chat binding should return response");
    assert_eq!(first_bind.status(), StatusCode::OK);
    let first_bind_body = first_bind
        .into_body()
        .collect()
        .await
        .expect("first direct chat bind body should collect")
        .to_bytes();
    let first_bind_json: serde_json::Value = serde_json::from_slice(&first_bind_body)
        .expect("first direct chat bind should be valid json");
    assert_eq!(first_bind_json["deliveryStatus"], "applied");
    assert_eq!(
        first_bind_json["proofVersion"],
        "conversation.create.delivery-proof.v1"
    );
    assert_eq!(
        first_bind_json["requestKey"],
        "6#t_demo6#system11#svc_control16#bind-direct-chat20#c_direct_retry_local"
    );

    let duplicate_bind = app
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
                        "conversationId":"c_direct_retry_local",
                        "directChatId":"dc_retry_local",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate direct chat binding should return response");
    assert_eq!(duplicate_bind.status(), StatusCode::OK);
    let duplicate_bind_body = duplicate_bind
        .into_body()
        .collect()
        .await
        .expect("duplicate direct chat bind body should collect")
        .to_bytes();
    let duplicate_bind_json: serde_json::Value = serde_json::from_slice(&duplicate_bind_body)
        .expect("duplicate direct chat bind should be valid json");
    assert_eq!(duplicate_bind_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_bind_json["requestKey"],
        first_bind_json["requestKey"]
    );
    assert_eq!(duplicate_bind_json["eventId"], first_bind_json["eventId"]);

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_direct_retry_local/members")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "actor_a")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    assert_eq!(members_json["items"].as_array().unwrap().len(), 2);

    let conflicting_bind = app
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
                        "conversationId":"c_direct_retry_local",
                        "directChatId":"dc_other_local",
                        "leftActorId":"actor_a",
                        "leftActorKind":"user",
                        "rightActorId":"actor_b",
                        "rightActorKind":"user"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting direct chat binding should return response");
    assert_eq!(conflicting_bind.status(), StatusCode::CONFLICT);
    let conflicting_bind_body = conflicting_bind
        .into_body()
        .collect()
        .await
        .expect("conflicting direct chat bind body should collect")
        .to_bytes();
    let conflicting_bind_json: serde_json::Value = serde_json::from_slice(&conflicting_bind_body)
        .expect("conflicting direct chat bind should be valid json");
    assert_eq!(conflicting_bind_json["code"], "conversation_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_expose_removed_media_resource_route() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/media/media_reference_missing")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("removed media resource route should return response");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_accept_legacy_media_upload_lifecycle_requests() {
    let app = local_minimal_node::build_default_app();

    for path in [
        "/im/v3/api/media/uploads",
        "/im/v3/api/media/uploads/media_reference_idempotent/complete",
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(path)
                    .demo_app_context()
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .expect("removed media lifecycle request should return response");
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{path} must stay removed because sdkwork-drive owns upload lifecycle"
        );
    }
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_rtc_session_create_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_idempotent",
                        "conversationId":"c_demo",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first rtc create should succeed");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first rtc create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first rtc create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert!(
        !first_create_json["requestKey"]
            .as_str()
            .expect("first rtc create requestKey should be present")
            .is_empty()
    );
    assert_eq!(
        first_create_json["proofVersion"],
        "rtc.session.delivery-proof.v1"
    );

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_local_idempotent/accept")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_local_rtc_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept rtc should succeed");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let idempotent_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_idempotent",
                        "conversationId":"c_demo",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent rtc create should return response");
    assert_eq!(idempotent_create.status(), StatusCode::OK);
    let idempotent_body = idempotent_create
        .into_body()
        .collect()
        .await
        .expect("idempotent rtc create body should collect")
        .to_bytes();
    let idempotent_json: serde_json::Value = serde_json::from_slice(&idempotent_body)
        .expect("idempotent rtc create should be valid json");
    assert_eq!(idempotent_json["state"], "accepted");
    assert_eq!(idempotent_json["artifactMessageId"], "msg_local_rtc_accept");
    assert_eq!(idempotent_json["deliveryStatus"], "replayed");
    assert_eq!(
        idempotent_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        idempotent_json["proofVersion"],
        first_create_json["proofVersion"]
    );

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_idempotent",
                        "conversationId":"c_other",
                        "rtcMode":"video"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting rtc create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting rtc body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value = serde_json::from_slice(&conflicting_body)
        .expect("conflicting rtc create should be valid json");
    assert_eq!(conflicting_json["code"], "rtc_session_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_retrieves_rtc_session_for_state_backfill() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_rtc_retrieve",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc retrieve conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_retrieve",
                        "conversationId":"c_rtc_retrieve",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc retrieve session should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let accept_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_retrieve/accept")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_rtc_retrieve_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept rtc retrieve session should succeed");
    assert_eq!(accept_rtc.status(), StatusCode::OK);

    let retrieve_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/calls/sessions/rtc_retrieve")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("retrieve rtc session should return response");
    assert_eq!(retrieve_rtc.status(), StatusCode::OK);
    let retrieve_body = retrieve_rtc
        .into_body()
        .collect()
        .await
        .expect("retrieve rtc session body should collect")
        .to_bytes();
    let retrieve_json: serde_json::Value =
        serde_json::from_slice(&retrieve_body).expect("retrieve rtc session should be valid json");
    assert_eq!(retrieve_json["rtcSessionId"], "rtc_retrieve");
    assert_eq!(retrieve_json["conversationId"], "c_rtc_retrieve");
    assert_eq!(retrieve_json["rtcMode"], "voice");
    assert_eq!(retrieve_json["state"], "accepted");
    assert_eq!(
        retrieve_json["artifactMessageId"],
        "msg_rtc_retrieve_accept"
    );

    let missing_rtc = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/calls/sessions/rtc_retrieve_missing")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("missing rtc session retrieve should return response");
    assert_eq!(missing_rtc.status(), StatusCode::NOT_FOUND);
    let missing_body = missing_rtc
        .into_body()
        .collect()
        .await
        .expect("missing rtc session body should collect")
        .to_bytes();
    let missing_json: serde_json::Value =
        serde_json::from_slice(&missing_body).expect("missing rtc session should be valid json");
    assert_eq!(missing_json["code"], "rtc_session_not_found");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_duplicate_rtc_create_from_different_actor_kind() {
    let app = local_minimal_node::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "shared_actor")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_kind_scope",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first rtc create should succeed");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first rtc create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first rtc create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert!(
        first_create_json["requestKey"]
            .as_str()
            .expect("first rtc create requestKey should be present")
            .contains("4#user12#shared_actor6#create20#rtc_local_kind_scope")
    );

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "shared_actor")
                .header("x-sdkwork-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_kind_scope",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting rtc create should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting rtc body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value = serde_json::from_slice(&conflicting_body)
        .expect("conflicting rtc create should be valid json");
    assert_eq!(conflicting_json["code"], "rtc_session_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_suppresses_duplicate_rtc_state_side_effects() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_rtc_state_side_effects",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_state_side_effects",
                        "conversationId":"c_rtc_state_side_effects",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let first_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_state_side_effects/accept")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first accept should succeed");
    assert_eq!(first_accept.status(), StatusCode::OK);

    let duplicate_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_state_side_effects/accept")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate accept should return response");
    assert_eq!(duplicate_accept.status(), StatusCode::OK);
    let duplicate_accept_body = duplicate_accept
        .into_body()
        .collect()
        .await
        .expect("duplicate accept body should collect")
        .to_bytes();
    let duplicate_accept_json: serde_json::Value = serde_json::from_slice(&duplicate_accept_body)
        .expect("duplicate accept should be valid json");
    assert_eq!(duplicate_accept_json["deliveryStatus"], "replayed");
    assert!(
        !duplicate_accept_json["requestKey"]
            .as_str()
            .expect("duplicate accept requestKey should be present")
            .is_empty()
    );
    assert_eq!(
        duplicate_accept_json["proofVersion"],
        "rtc.session.delivery-proof.v1"
    );

    let first_end = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_state_side_effects/end")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first end should succeed");
    assert_eq!(first_end.status(), StatusCode::OK);

    let duplicate_end = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_state_side_effects/end")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate end should return response");
    assert_eq!(duplicate_end.status(), StatusCode::OK);

    let conflicting_reject = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_state_side_effects/reject")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_state_reject_conflict"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting reject should return response");
    assert_eq!(conflicting_reject.status(), StatusCode::CONFLICT);
    let conflicting_reject_body = conflicting_reject
        .into_body()
        .collect()
        .await
        .expect("conflicting reject body should collect")
        .to_bytes();
    let conflicting_reject_json: serde_json::Value =
        serde_json::from_slice(&conflicting_reject_body)
            .expect("conflicting reject should be valid json");
    assert_eq!(
        conflicting_reject_json["code"],
        "rtc_session_state_conflict"
    );

    let timeline = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_rtc_state_side_effects/messages")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should succeed");
    assert_eq!(timeline.status(), StatusCode::OK);
    let timeline_body = timeline
        .into_body()
        .collect()
        .await
        .expect("timeline body should collect")
        .to_bytes();
    let timeline_json: serde_json::Value =
        serde_json::from_slice(&timeline_body).expect("timeline should be valid json");
    let items = timeline_json["items"]
        .as_array()
        .expect("timeline items should be array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["summary"], "rtc.accept");
    assert_eq!(items[1]["summary"], "rtc.end");
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_conversation_member_management() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_members",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let initial_members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members/members")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list initial members should succeed");
    assert_eq!(initial_members.status(), StatusCode::OK);
    let initial_members_body = initial_members
        .into_body()
        .collect()
        .await
        .expect("initial members body should collect")
        .to_bytes();
    let initial_members_json: serde_json::Value = serde_json::from_slice(&initial_members_body)
        .expect("initial members response should be valid json");
    assert_eq!(initial_members_json["items"][0]["principalId"], "u_demo");
    assert_eq!(initial_members_json["items"][0]["role"], "owner");

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members/members/add")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"ag_demo",
                        "principalKind":"agent",
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
        serde_json::from_slice(&add_member_body).expect("add member response should be valid json");
    assert_eq!(add_member_json["memberId"], "cm_c_members_agent_ag_demo");
    assert_eq!(add_member_json["principalKind"], "agent");
    assert_eq!(add_member_json["state"], "joined");

    let members_after_add = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members/members")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members after add should succeed");
    assert_eq!(members_after_add.status(), StatusCode::OK);
    let members_after_add_body = members_after_add
        .into_body()
        .collect()
        .await
        .expect("members after add body should collect")
        .to_bytes();
    let members_after_add_json: serde_json::Value = serde_json::from_slice(&members_after_add_body)
        .expect("members after add response should be valid json");
    assert_eq!(members_after_add_json["items"].as_array().unwrap().len(), 2);

    let remove_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_members/members/remove")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_members_agent_ag_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove member should succeed");
    assert_eq!(remove_member.status(), StatusCode::OK);
    let remove_member_body = remove_member
        .into_body()
        .collect()
        .await
        .expect("remove member body should collect")
        .to_bytes();
    let remove_member_json: serde_json::Value = serde_json::from_slice(&remove_member_body)
        .expect("remove member response should be valid json");
    assert_eq!(remove_member_json["state"], "removed");

    let members_after_remove = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_members/members")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list members after remove should succeed");
    assert_eq!(members_after_remove.status(), StatusCode::OK);
    let members_after_remove_body = members_after_remove
        .into_body()
        .collect()
        .await
        .expect("members after remove body should collect")
        .to_bytes();
    let members_after_remove_json: serde_json::Value =
        serde_json::from_slice(&members_after_remove_body)
            .expect("members after remove response should be valid json");
    assert_eq!(
        members_after_remove_json["items"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        members_after_remove_json["items"][0]["principalId"],
        "u_demo"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_preserves_add_member_request_attributes_for_non_user_principal()
{
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_member_request_attributes",
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
                .uri("/im/v3/api/chat/conversations/c_member_request_attributes/members/add")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"ag_attr_demo",
                        "principalKind":"agent",
                        "role":"member",
                        "attributes":{
                            "serviceTier":"gold",
                            "region":"cn-sh"
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
        serde_json::from_slice(&add_member_body).expect("add member response should be valid json");
    assert_eq!(add_member_json["attributes"]["serviceTier"], "gold");
    assert_eq!(add_member_json["attributes"]["region"], "cn-sh");
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_read_cursor_and_unread_view() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cursor",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for (client_msg_id, summary) in [("client_1", "one"), ("client_2", "two")] {
        let post_message = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations/c_cursor/messages")
                    .demo_app_context()
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "clientMsgId":"{client_msg_id}",
                            "summary":"{summary}",
                            "text":"{summary}"
                        }}"#,
                    )))
                    .unwrap(),
            )
            .await
            .expect("post message should succeed");
        assert_eq!(post_message.status(), StatusCode::OK);
    }

    let initial_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_cursor/read_cursor")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("initial read cursor should succeed");
    assert_eq!(initial_cursor.status(), StatusCode::OK);
    let initial_cursor_body = initial_cursor
        .into_body()
        .collect()
        .await
        .expect("initial cursor body should collect")
        .to_bytes();
    let initial_cursor_json: serde_json::Value =
        serde_json::from_slice(&initial_cursor_body).expect("initial cursor should be valid json");
    assert_eq!(initial_cursor_json["readSeq"], 0);
    assert_eq!(initial_cursor_json["unreadCount"], 2);

    let update_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cursor/read_cursor")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":1,
                        "lastReadMessageId":"msg_c_cursor_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("update read cursor should succeed");
    assert_eq!(update_cursor.status(), StatusCode::OK);
    let update_cursor_body = update_cursor
        .into_body()
        .collect()
        .await
        .expect("update cursor body should collect")
        .to_bytes();
    let update_cursor_json: serde_json::Value =
        serde_json::from_slice(&update_cursor_body).expect("updated cursor should be valid json");
    assert_eq!(update_cursor_json["readSeq"], 1);
    assert_eq!(update_cursor_json["unreadCount"], 1);

    let regressed_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cursor/read_cursor")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":0,
                        "lastReadMessageId":"msg_c_cursor_0"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("regressed read cursor should succeed");
    assert_eq!(regressed_cursor.status(), StatusCode::OK);
    let regressed_cursor_body = regressed_cursor
        .into_body()
        .collect()
        .await
        .expect("regressed cursor body should collect")
        .to_bytes();
    let regressed_cursor_json: serde_json::Value = serde_json::from_slice(&regressed_cursor_body)
        .expect("regressed cursor response should be valid json");
    assert_eq!(regressed_cursor_json["readSeq"], 1);
    assert_eq!(regressed_cursor_json["unreadCount"], 1);
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_inbox_view() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_inbox",
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
                .uri("/im/v3/api/chat/conversations/c_inbox/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_inbox_1",
                        "summary":"hello inbox",
                        "text":"hello inbox"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let inbox = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get inbox should succeed");
    assert_eq!(inbox.status(), StatusCode::OK);
    let inbox_body = inbox
        .into_body()
        .collect()
        .await
        .expect("inbox body should collect")
        .to_bytes();
    let inbox_json: serde_json::Value =
        serde_json::from_slice(&inbox_body).expect("inbox should be valid json");
    assert_eq!(inbox_json["items"][0]["conversationId"], "c_inbox");
    assert_eq!(inbox_json["items"][0]["conversationType"], "group");
    assert_eq!(inbox_json["items"][0]["messageCount"], 1);
    assert_eq!(inbox_json["items"][0]["unreadCount"], 1);

    let update_cursor = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_inbox/read_cursor")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":1,
                        "lastReadMessageId":"msg_c_inbox_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("update read cursor should succeed");
    assert_eq!(update_cursor.status(), StatusCode::OK);

    let inbox_after_read = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get inbox after read should succeed");
    assert_eq!(inbox_after_read.status(), StatusCode::OK);
    let inbox_after_read_body = inbox_after_read
        .into_body()
        .collect()
        .await
        .expect("inbox after read body should collect")
        .to_bytes();
    let inbox_after_read_json: serde_json::Value = serde_json::from_slice(&inbox_after_read_body)
        .expect("inbox after read should be valid json");
    assert_eq!(inbox_after_read_json["items"][0]["unreadCount"], 0);
}

#[tokio::test]
async fn test_local_minimal_profile_second_instance_reads_latest_conversation_summary_from_shared_runtime_dir()
 {
    let runtime_dir = unique_test_runtime_dir("projection_summary_second_instance");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let app_b = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cross_instance_summary",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation on first instance should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cross_instance_summary/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cross_instance_summary_1",
                        "summary":"hello from instance a",
                        "text":"hello from instance a"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message on first instance should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let summary_from_second_instance = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_cross_instance_summary")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary on second instance should return response");
    assert_eq!(
        summary_from_second_instance.status(),
        StatusCode::OK,
        "second instance should load latest conversation summary from the shared runtime dir without restart"
    );
    let summary_body = summary_from_second_instance
        .into_body()
        .collect()
        .await
        .expect("conversation summary on second instance body should collect")
        .to_bytes();
    let summary_json: serde_json::Value = serde_json::from_slice(&summary_body)
        .expect("conversation summary on second instance should be valid json");
    assert_eq!(
        summary_json["lastMessageId"],
        "msg_c_cross_instance_summary_1"
    );
    assert_eq!(summary_json["messageCount"], 1);
    assert_eq!(summary_json["lastSummary"], "hello from instance a");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_second_instance_reads_latest_inbox_from_shared_runtime_dir() {
    let runtime_dir = unique_test_runtime_dir("projection_inbox_second_instance");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let app_b = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cross_instance_inbox",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation on first instance should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cross_instance_inbox/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cross_instance_inbox_1",
                        "summary":"hello inbox from instance a",
                        "text":"hello inbox from instance a"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message on first instance should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let inbox_from_second_instance = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("inbox on second instance should return response");
    assert_eq!(
        inbox_from_second_instance.status(),
        StatusCode::OK,
        "second instance should load latest inbox entries from the shared runtime dir without restart"
    );
    let inbox_body = inbox_from_second_instance
        .into_body()
        .collect()
        .await
        .expect("inbox on second instance body should collect")
        .to_bytes();
    let inbox_json: serde_json::Value =
        serde_json::from_slice(&inbox_body).expect("inbox on second instance should be valid json");
    assert_eq!(
        inbox_json["items"][0]["conversationId"],
        "c_cross_instance_inbox"
    );
    assert_eq!(inbox_json["items"][0]["conversationType"], "group");
    assert_eq!(inbox_json["items"][0]["messageCount"], 1);
    assert_eq!(inbox_json["items"][0]["unreadCount"], 1);
    assert_eq!(
        inbox_json["items"][0]["lastMessageId"],
        "msg_c_cross_instance_inbox_1"
    );
    assert_eq!(
        inbox_json["items"][0]["lastSummary"],
        "hello inbox from instance a"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_heartbeats_client_route_and_returns_presence_snapshot() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_resume",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for device_id in ["d_phone", "d_pad"] {
        let register = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/presence/heartbeat")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_demo")
                    .header("x-sdkwork-actor-kind", "user")
                    .header("x-sdkwork-device-id", device_id)
                    .header("content-type", "application/json")
                    .body(Body::from(format!(r#"{{"deviceId":"{device_id}"}}"#)))
                    .unwrap(),
            )
            .await
            .expect("client route heartbeat should succeed");
        assert_eq!(register.status(), StatusCode::OK);
    }

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_resume/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-session-id", "s_phone")
                .header("x-sdkwork-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_resume_1",
                        "summary":"resume hello",
                        "text":"resume hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let heartbeat = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-session-id", "s_pad")
                .header("x-sdkwork-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "deviceId":"d_pad"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("heartbeat should succeed");
    assert_eq!(heartbeat.status(), StatusCode::OK);
    let heartbeat_body = heartbeat
        .into_body()
        .collect()
        .await
        .expect("heartbeat body should collect")
        .to_bytes();
    let heartbeat_json: serde_json::Value =
        serde_json::from_slice(&heartbeat_body).expect("heartbeat should be valid json");
    assert_eq!(heartbeat_json["currentDeviceId"], "d_pad");
    let heartbeat_devices = heartbeat_json["devices"]
        .as_array()
        .expect("heartbeat response should include client route entries");
    assert_eq!(heartbeat_devices.len(), 2);
    for expected_device_id in ["d_pad", "d_phone"] {
        let entry = heartbeat_devices
            .iter()
            .find(|entry| entry["deviceId"] == expected_device_id)
            .expect("heartbeat response should include expected client route entry");
        assert_eq!(entry["status"], "online");
    }

    let presence = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/presence/me")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-session-id", "s_pad")
                .header("x-sdkwork-device-id", "d_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("presence request should succeed");
    assert_eq!(presence.status(), StatusCode::OK);
    let presence_body = presence
        .into_body()
        .collect()
        .await
        .expect("presence body should collect")
        .to_bytes();
    let presence_json: serde_json::Value =
        serde_json::from_slice(&presence_body).expect("presence should be valid json");
    assert_eq!(presence_json["currentDeviceId"], "d_pad");
    assert_eq!(presence_json["devices"][0]["status"], "online");
}

#[tokio::test]
async fn test_local_minimal_profile_preserves_message_post_audit_for_max_length_conversation_ids() {
    let app = local_minimal_node::build_default_app();
    let conversation_id = "c".repeat(256);

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "conversationId": conversation_id,
                        "conversationType": "group",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_client_route = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_phone"}"#))
                .unwrap(),
        )
        .await
        .expect("device register should succeed");
    assert_eq!(register_client_route.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_local_long_message_id",
                        "summary":"hello",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_message_body = post_message
        .into_body()
        .collect()
        .await
        .expect("post message body should collect")
        .to_bytes();
    let post_message_json: serde_json::Value =
        serde_json::from_slice(&post_message_body).expect("post message should be valid json");
    let message_id = post_message_json["messageId"]
        .as_str()
        .expect("message id should be present")
        .to_owned();

    let audit_export = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/export")
                .header("x-sdkwork-permission-scope", "audit.read")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should succeed");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_export_body = audit_export
        .into_body()
        .collect()
        .await
        .expect("audit export body should collect")
        .to_bytes();
    let audit_export_json: serde_json::Value =
        serde_json::from_slice(&audit_export_body).expect("audit export should be valid json");
    let posted_audit = audit_export_json["items"]
        .as_array()
        .expect("audit items should be array")
        .iter()
        .find(|item| {
            item["action"] == "message.posted"
                && item["aggregateId"]
                    .as_str()
                    .is_some_and(|aggregate_id| aggregate_id == conversation_id)
        })
        .expect("message.posted audit should be recorded for legal long conversation ids");
    let posted_payload: serde_json::Value = serde_json::from_str(
        posted_audit["payload"]
            .as_str()
            .expect("payload should be present"),
    )
    .expect("audit payload should be valid json");
    assert_eq!(posted_payload["messageId"], message_id);
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_mount_appbase_owned_notification_routes() {
    let app = local_minimal_node::build_default_app();

    for (method, path, body) in [
        ("GET", "/app/v3/api/notifications", Body::empty()),
        (
            "GET",
            "/app/v3/api/notifications/ntf_local_actor_kind_guard",
            Body::empty(),
        ),
        (
            "POST",
            "/app/v3/api/notifications/requests",
            Body::from(
                r#"{
                    "notificationId":"ntf_local_appbase_boundary",
                    "sourceEventId":"evt_local_appbase_boundary",
                    "sourceEventType":"message.posted",
                    "category":"message.new",
                    "channel":"inapp",
                    "recipientId":"u_demo",
                    "recipientKind":"user",
                    "title":"New message",
                    "body":"hello",
                    "payload":"{\"conversationId\":\"c_demo\"}"
                }"#,
            ),
        ),
    ] {
        let mut builder = Request::builder()
            .method(method)
            .uri(path)
            .header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_demo")
            .header("x-sdkwork-actor-kind", "user");
        if method == "POST" {
            builder = builder
                .header("x-sdkwork-permission-scope", "notification.write")
                .header("content-type", "application/json");
        }
        let response = app
            .clone()
            .oneshot(builder.body(body).unwrap())
            .await
            .expect("appbase-owned notification route should return response");
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{method} {path} must not be mounted by local-minimal-node"
        );
    }
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_generic_stream_frame_transport() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_stream_frames",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_frames_demo",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_frames",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_frames_demo/frames")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hel\"}",
                        "attributes": {
                            "topic": "llm"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);
    let append_frame_body = append_frame
        .into_body()
        .collect()
        .await
        .expect("append frame body should collect")
        .to_bytes();
    let append_frame_json: serde_json::Value =
        serde_json::from_slice(&append_frame_body).expect("append frame should be valid json");
    assert_eq!(append_frame_json["streamId"], "st_frames_demo");
    assert_eq!(append_frame_json["frameSeq"], 1);
    assert_eq!(append_frame_json["sender"]["id"], "u_demo");

    let second_append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_frames_demo/frames")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"lo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second append frame should succeed");
    assert_eq!(second_append_frame.status(), StatusCode::OK);

    let list_frames = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_frames_demo/frames?afterFrameSeq=0&limit=10")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames should succeed");
    assert_eq!(list_frames.status(), StatusCode::OK);
    let list_frames_body = list_frames
        .into_body()
        .collect()
        .await
        .expect("list frames body should collect")
        .to_bytes();
    let list_frames_json: serde_json::Value =
        serde_json::from_slice(&list_frames_body).expect("list frames should be valid json");
    assert_eq!(list_frames_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(list_frames_json["items"][0]["frameSeq"], 1);
    assert_eq!(list_frames_json["items"][1]["frameSeq"], 2);
    assert_eq!(list_frames_json["items"][0]["attributes"]["topic"], "llm");
    assert_eq!(list_frames_json["nextAfterFrameSeq"], 2);
    assert_eq!(list_frames_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_delivers_realtime_events_to_subscribed_client_route_window() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_realtime",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_phone = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register phone should succeed");
    assert_eq!(register_phone.status(), StatusCode::OK);

    let register_pad = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register pad should succeed");
    assert_eq!(register_pad.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_realtime/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_1",
                        "summary":"hello realtime",
                        "text":"hello realtime"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    assert_eq!(realtime_events_json["deviceId"], "d_pad");
    assert_eq!(realtime_events_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        realtime_events_json["items"][0]["eventType"],
        "message.posted"
    );
    assert_eq!(
        realtime_events_json["items"][0]["scopeType"],
        "conversation"
    );
    assert_eq!(realtime_events_json["items"][0]["scopeId"], "c_realtime");
    let payload: serde_json::Value = serde_json::from_str(
        realtime_events_json["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_realtime");
    assert_eq!(payload["messageType"], "standard");
    assert_eq!(payload["body"]["summary"], "hello realtime");
    assert_eq!(payload["body"]["parts"][0]["kind"], "text");
    assert_eq!(payload["body"]["parts"][0]["text"], "hello realtime");
    assert_eq!(realtime_events_json["nextAfterSeq"], 1);
    assert_eq!(realtime_events_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_realtime_limit_above_guardrail_over_http() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=5000")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_demo")
                .header("x-sdkwork-session-id", "s_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime limit request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("realtime limit rejection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("realtime limit rejection body should be valid json");
    assert_eq!(json["code"], "limit_invalid");
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_fan_out_conversation_realtime_to_non_member_same_actor_id_different_actor_kind()
 {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
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
                        "conversationId":"c_realtime_kind_guard",
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
                .uri("/im/v3/api/chat/conversations/c_realtime_kind_guard/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_dual",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let register_user_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_dual")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_dual_user")
                .header("x-sdkwork-session-id", "s_dual_user")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register user device should succeed");
    assert_eq!(register_user_device.status(), StatusCode::OK);

    let register_agent_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_dual")
                .header("x-sdkwork-actor-kind", "agent")
                .header("x-sdkwork-device-id", "d_dual_agent")
                .header("x-sdkwork-session-id", "s_dual_agent")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register agent device should return response");
    assert_eq!(register_agent_device.status(), StatusCode::OK);

    let sync_user_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_dual")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_dual_user")
                .header("x-sdkwork-session-id", "s_dual_user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_kind_guard",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user subscription sync should succeed");
    assert_eq!(sync_user_subscriptions.status(), StatusCode::OK);

    let sync_agent_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_dual")
                .header("x-sdkwork-actor-kind", "agent")
                .header("x-sdkwork-device-id", "d_dual_agent")
                .header("x-sdkwork-session-id", "s_dual_agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_kind_guard",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent subscription sync should succeed");
    assert_eq!(sync_agent_subscriptions.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_realtime_kind_guard/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_kind_guard_1",
                        "summary":"typed fanout",
                        "text":"typed fanout"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let user_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_dual")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_dual_user")
                .header("x-sdkwork-session-id", "s_dual_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("user realtime events should succeed");
    assert_eq!(user_events.status(), StatusCode::OK);
    let user_events_body = user_events
        .into_body()
        .collect()
        .await
        .expect("user realtime events body should collect")
        .to_bytes();
    let user_events_json: serde_json::Value = serde_json::from_slice(&user_events_body)
        .expect("user realtime events should be valid json");
    assert_eq!(user_events_json["deviceId"], "d_dual_user");
    assert_eq!(user_events_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(user_events_json["items"][0]["eventType"], "message.posted");

    let agent_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_dual")
                .header("x-sdkwork-actor-kind", "agent")
                .header("x-sdkwork-device-id", "d_dual_agent")
                .header("x-sdkwork-session-id", "s_dual_agent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("agent realtime events should succeed");
    assert_eq!(agent_events.status(), StatusCode::OK);
    let agent_events_body = agent_events
        .into_body()
        .collect()
        .await
        .expect("agent realtime events body should collect")
        .to_bytes();
    let agent_events_json: serde_json::Value = serde_json::from_slice(&agent_events_body)
        .expect("agent realtime events should be valid json");
    assert_eq!(agent_events_json["deviceId"], "d_dual_agent");
    assert_eq!(
        agent_events_json["items"].as_array().unwrap().len(),
        0,
        "non-member agent principal sharing the same actor id must not receive conversation realtime events"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_refanout_duplicate_message_post_retry() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_post_retry_fanout",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_pad = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register pad should succeed");
    assert_eq!(register_pad.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_post_retry_fanout",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let first_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_post_retry_fanout/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_post_retry_fanout",
                        "summary":"hello retry",
                        "text":"hello retry"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should succeed");
    assert_eq!(first_post.status(), StatusCode::OK);
    let first_post_body = first_post
        .into_body()
        .collect()
        .await
        .expect("first post body should collect")
        .to_bytes();
    let first_post_json: serde_json::Value =
        serde_json::from_slice(&first_post_body).expect("first post should be valid json");
    assert_eq!(first_post_json["deliveryStatus"], "applied");
    assert_eq!(
        first_post_json["proofVersion"],
        "conversation.message.delivery-proof.v1"
    );

    let duplicate_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_post_retry_fanout/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_post_retry_fanout",
                        "summary":"hello retry",
                        "text":"hello retry"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate post should return response");
    assert_eq!(duplicate_post.status(), StatusCode::OK);
    let duplicate_post_body = duplicate_post
        .into_body()
        .collect()
        .await
        .expect("duplicate post body should collect")
        .to_bytes();
    let duplicate_post_json: serde_json::Value =
        serde_json::from_slice(&duplicate_post_body).expect("duplicate post should be valid json");
    assert_eq!(duplicate_post_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_post_json["requestKey"],
        first_post_json["requestKey"]
    );
    assert_eq!(
        duplicate_post_json["messageId"],
        first_post_json["messageId"]
    );

    let history = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_post_retry_fanout/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("history should succeed");
    assert_eq!(history.status(), StatusCode::OK);
    let history_body = history
        .into_body()
        .collect()
        .await
        .expect("history body should collect")
        .to_bytes();
    let history_json: serde_json::Value =
        serde_json::from_slice(&history_body).expect("history should be valid json");
    assert_eq!(history_json["items"].as_array().unwrap().len(), 1);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(
        items.len(),
        1,
        "duplicate idempotent message post must not emit a second realtime fanout event"
    );
    assert_eq!(items[0]["eventType"], "message.posted");
    assert_eq!(items[0]["scopeType"], "conversation");
    assert_eq!(items[0]["scopeId"], "c_post_retry_fanout");
}

#[tokio::test]
async fn test_local_minimal_profile_acks_and_trims_realtime_event_window() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_realtime_ack",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_phone = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register phone should succeed");
    assert_eq!(register_phone.status(), StatusCode::OK);

    let register_pad = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register pad should succeed");
    assert_eq!(register_pad.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_ack",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_realtime_ack/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_ack_1",
                        "summary":"ack me",
                        "text":"ack me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let before_ack = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(before_ack.status(), StatusCode::OK);
    let before_ack_body = before_ack
        .into_body()
        .collect()
        .await
        .expect("before ack body should collect")
        .to_bytes();
    let before_ack_json: serde_json::Value =
        serde_json::from_slice(&before_ack_body).expect("before ack should be valid json");
    assert_eq!(before_ack_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(before_ack_json["ackedThroughSeq"], 0);
    assert_eq!(before_ack_json["trimmedThroughSeq"], 0);

    let ack_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/events/ack")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ackedSeq":1}"#))
                .unwrap(),
        )
        .await
        .expect("ack request should succeed");
    assert_eq!(ack_response.status(), StatusCode::OK);
    let ack_body = ack_response
        .into_body()
        .collect()
        .await
        .expect("ack body should collect")
        .to_bytes();
    let ack_json: serde_json::Value =
        serde_json::from_slice(&ack_body).expect("ack response should be valid json");
    assert_eq!(ack_json["deviceId"], "d_pad");
    assert_eq!(ack_json["ackedThroughSeq"], 1);
    assert_eq!(ack_json["trimmedThroughSeq"], 1);
    assert_eq!(ack_json["retainedEventCount"], 0);

    let after_ack = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events after ack should succeed");
    assert_eq!(after_ack.status(), StatusCode::OK);
    let after_ack_body = after_ack
        .into_body()
        .collect()
        .await
        .expect("after ack body should collect")
        .to_bytes();
    let after_ack_json: serde_json::Value =
        serde_json::from_slice(&after_ack_body).expect("after ack should be valid json");
    assert_eq!(after_ack_json["items"].as_array().unwrap().len(), 0);
    assert_eq!(after_ack_json["ackedThroughSeq"], 1);
    assert_eq!(after_ack_json["trimmedThroughSeq"], 1);
    assert_eq!(after_ack_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_conversation_stream_frames_to_other_member_subscribers()
{
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_stream_realtime_fanout",
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
                .uri("/im/v3/api/chat/conversations/c_stream_realtime_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_realtime_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_realtime_fanout",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"stream",
                                "scopeId":"st_stream_realtime_fanout",
                                "eventTypes":["stream.frame.appended"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_realtime_fanout/frames")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["eventType"], "stream.frame.appended");
    assert_eq!(items[0]["scopeType"], "stream");
    assert_eq!(items[0]["scopeId"], "st_stream_realtime_fanout");

    let payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["streamId"], "st_stream_realtime_fanout");
    assert_eq!(payload["scopeKind"], "conversation");
    assert_eq!(payload["scopeId"], "c_stream_realtime_fanout");
    assert_eq!(payload["frameSeq"], 1);
}

#[tokio::test]
async fn test_local_minimal_profile_derives_standardized_summaries_for_rich_messages() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_rich_summary",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_location = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_rich_summary/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_location",
                        "parts":[
                            {
                                "kind":"data",
                                "schemaRef":"urn:sdkwork:craw-chat:message:location",
                                "encoding":"application/json",
                                "payload":"{\"name\":\"The Bund\",\"latitude\":31.2400,\"longitude\":121.4900}"
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("location rich message should succeed");
    assert_eq!(post_location.status(), StatusCode::OK);

    let post_custom = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_rich_summary/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_custom",
                        "parts":[
                            {
                                "kind":"data",
                                "schemaRef":"urn:sdkwork:craw-chat:message:custom:workflow.approval",
                                "encoding":"application/json",
                                "payload":"{\"approvalId\":\"approval_demo\"}"
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("custom rich message should succeed");
    assert_eq!(post_custom.status(), StatusCode::OK);

    let post_ai_image = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_rich_summary/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_ai_image",
                        "parts":[
                            {
                                "kind":"data",
                                "schemaRef":"urn:sdkwork:craw-chat:message:ai_image",
                                "encoding":"application/json",
                                "payload":"{\"prompt\":\"Shanghai skyline at sunset\",\"status\":\"completed\",\"model\":\"gpt-image-1\"}"
                            },
                            {
                                "kind":"media",
                                "mediaRole":"generated_output",
                                "drive":{
                                    "driveUri":"drive://spaces/space_ai_generated_demo/nodes/node_asset_ai_generated_demo",
                                    "spaceId":"space_ai_generated_demo",
                                    "nodeId":"node_asset_ai_generated_demo"
                                },
                                "resource":{
                                    "id":"node_asset_ai_generated_demo",
                                    "kind":"image",
                                    "source":"generated",
                                    "uri":"drive://spaces/space_ai_generated_demo/nodes/node_asset_ai_generated_demo",
                                    "mimeType":"image/png",
                                    "title":"Shanghai skyline at sunset",
                                    "ai":{
                                        "provenance":"generated",
                                        "provider":"openai",
                                        "model":"gpt-image-1",
                                        "promptId":"prompt_ai_image_demo",
                                        "moderationStatus":"approved"
                                    }
                                }
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("ai image rich message should succeed");
    assert_eq!(post_ai_image.status(), StatusCode::OK);

    let timeline = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_rich_summary/messages")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should succeed");
    assert_eq!(timeline.status(), StatusCode::OK);
    let timeline_body = timeline
        .into_body()
        .collect()
        .await
        .expect("timeline body should collect")
        .to_bytes();
    let timeline_json: serde_json::Value =
        serde_json::from_slice(&timeline_body).expect("timeline should be valid json");
    let items = timeline_json["items"]
        .as_array()
        .expect("timeline items should be array");
    assert_eq!(items.len(), 3);
    assert_eq!(items[0]["summary"], "Location: The Bund");
    assert_eq!(items[1]["summary"], "Custom: workflow.approval");
    assert_eq!(items[2]["summary"], "AI image generated");

    let conversation_summary = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_rich_summary")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary should succeed");
    assert_eq!(conversation_summary.status(), StatusCode::OK);
    let conversation_summary_body = conversation_summary
        .into_body()
        .collect()
        .await
        .expect("conversation summary body should collect")
        .to_bytes();
    let conversation_summary_json: serde_json::Value =
        serde_json::from_slice(&conversation_summary_body)
            .expect("conversation summary should be valid json");
    assert_eq!(
        conversation_summary_json["lastSummary"],
        "AI image generated"
    );
    assert_eq!(conversation_summary_json["messageCount"], 3);
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_refanout_duplicate_stream_frame_retry() {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_stream_retry_fanout",
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
                .uri("/im/v3/api/chat/conversations/c_stream_retry_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_retry_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_retry_fanout",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"stream",
                                "scopeId":"st_stream_retry_fanout",
                                "eventTypes":["stream.frame.appended"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let first_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_retry_fanout/frames")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first append should succeed");
    assert_eq!(first_append.status(), StatusCode::OK);
    let first_append_body = first_append
        .into_body()
        .collect()
        .await
        .expect("first append body should collect")
        .to_bytes();
    let first_append_json: serde_json::Value =
        serde_json::from_slice(&first_append_body).expect("first append should be valid json");
    assert_eq!(first_append_json["deliveryStatus"], "applied");
    assert_eq!(
        first_append_json["proofVersion"],
        "stream.frame.delivery-proof.v1"
    );

    let duplicate_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_retry_fanout/frames")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate append should return response");
    assert_eq!(duplicate_append.status(), StatusCode::OK);
    let duplicate_append_body = duplicate_append
        .into_body()
        .collect()
        .await
        .expect("duplicate append body should collect")
        .to_bytes();
    let duplicate_append_json: serde_json::Value = serde_json::from_slice(&duplicate_append_body)
        .expect("duplicate append should be valid json");
    assert_eq!(duplicate_append_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_append_json["requestKey"],
        first_append_json["requestKey"]
    );

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(
        items.len(),
        1,
        "duplicate idempotent stream append must not emit a second realtime fanout event"
    );
    assert_eq!(items[0]["eventType"], "stream.frame.appended");
    assert_eq!(items[0]["scopeId"], "st_stream_retry_fanout");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_conversation_stream_completion_to_other_member_subscribers()
 {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_stream_completion_fanout",
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
                .uri("/im/v3/api/chat/conversations/c_stream_completion_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_completion_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_completion_fanout",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"stream",
                                "scopeId":"st_stream_completion_fanout",
                                "eventTypes":["stream.completed"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_completion_fanout/frames")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let complete_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_completion_fanout/complete")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_result_stream_completion"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream should succeed");
    assert_eq!(complete_stream.status(), StatusCode::OK);

    let realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["eventType"], "stream.completed");
    assert_eq!(items[0]["scopeType"], "stream");
    assert_eq!(items[0]["scopeId"], "st_stream_completion_fanout");

    let payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["streamId"], "st_stream_completion_fanout");
    assert_eq!(payload["scopeKind"], "conversation");
    assert_eq!(payload["scopeId"], "c_stream_completion_fanout");
    assert_eq!(payload["state"], "completed");
    assert_eq!(payload["lastFrameSeq"], 1);
    assert_eq!(payload["resultMessageId"], "msg_result_stream_completion");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_conversation_stream_abort_to_other_member_subscribers()
{
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_stream_abort_fanout",
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
                .uri("/im/v3/api/chat/conversations/c_stream_abort_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_stream_abort_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_abort_fanout",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"stream",
                                "scopeId":"st_stream_abort_fanout",
                                "eventTypes":["stream.aborted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_abort_fanout/frames")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let abort_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_stream_abort_fanout/abort")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "reason": "user_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("abort stream should succeed");
    assert_eq!(abort_stream.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["eventType"], "stream.aborted");
    assert_eq!(items[0]["scopeType"], "stream");
    assert_eq!(items[0]["scopeId"], "st_stream_abort_fanout");

    let payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["streamId"], "st_stream_abort_fanout");
    assert_eq!(payload["scopeKind"], "conversation");
    assert_eq!(payload["scopeId"], "c_stream_abort_fanout");
    assert_eq!(payload["state"], "aborted");
    assert_eq!(payload["lastFrameSeq"], 1);
    assert_eq!(payload["reason"], "user_cancelled");
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_refanout_duplicate_stream_abort_retry() {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_stream_abort_idempotent",
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
                .uri("/im/v3/api/chat/conversations/c_stream_abort_idempotent/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_abort_retry_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_abort_idempotent",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"stream",
                                "scopeId":"st_abort_retry_fanout",
                                "eventTypes":["stream.aborted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort_retry_fanout/frames")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let first_abort = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort_retry_fanout/abort")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "reason": "user_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first abort should return response");
    assert_eq!(first_abort.status(), StatusCode::OK);
    let first_abort_body = first_abort
        .into_body()
        .collect()
        .await
        .expect("first abort body should collect")
        .to_bytes();
    let first_abort_json: serde_json::Value =
        serde_json::from_slice(&first_abort_body).expect("first abort should be valid json");
    assert_eq!(first_abort_json["deliveryStatus"], "applied");
    assert_eq!(
        first_abort_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let duplicate_abort = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_abort_retry_fanout/abort")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "reason": "user_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate abort should return response");
    assert_eq!(duplicate_abort.status(), StatusCode::OK);
    let duplicate_abort_body = duplicate_abort
        .into_body()
        .collect()
        .await
        .expect("duplicate abort body should collect")
        .to_bytes();
    let duplicate_abort_json: serde_json::Value = serde_json::from_slice(&duplicate_abort_body)
        .expect("duplicate abort should be valid json");
    assert_eq!(duplicate_abort_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_abort_json["requestKey"],
        first_abort_json["requestKey"]
    );

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["eventType"], "stream.aborted");

    let payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["streamId"], "st_abort_retry_fanout");
    assert_eq!(payload["reason"], "user_cancelled");
}

#[tokio::test]
async fn test_local_minimal_profile_replays_duplicate_checkpoint_retry_after_complete() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_stream_checkpoint_idempotent",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_checkpoint_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_checkpoint_idempotent",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let first_checkpoint = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_local_checkpoint_idempotent/checkpoint")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first checkpoint should return response");
    assert_eq!(first_checkpoint.status(), StatusCode::OK);
    let first_checkpoint_body = first_checkpoint
        .into_body()
        .collect()
        .await
        .expect("first checkpoint body should collect")
        .to_bytes();
    let first_checkpoint_json: serde_json::Value = serde_json::from_slice(&first_checkpoint_body)
        .expect("first checkpoint should be valid json");
    assert_eq!(first_checkpoint_json["deliveryStatus"], "applied");
    assert_eq!(
        first_checkpoint_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let complete_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_local_checkpoint_idempotent/complete")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 5,
                        "resultMessageId": "msg_checkpoint_complete"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream should return response");
    assert_eq!(complete_stream.status(), StatusCode::OK);

    let duplicate_checkpoint = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_local_checkpoint_idempotent/checkpoint")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate checkpoint should return response");
    assert_eq!(duplicate_checkpoint.status(), StatusCode::OK);
    let duplicate_checkpoint_body = duplicate_checkpoint
        .into_body()
        .collect()
        .await
        .expect("duplicate checkpoint body should collect")
        .to_bytes();
    let duplicate_checkpoint_json: serde_json::Value =
        serde_json::from_slice(&duplicate_checkpoint_body)
            .expect("duplicate checkpoint should be valid json");
    assert_eq!(duplicate_checkpoint_json["state"], "completed");
    assert_eq!(duplicate_checkpoint_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_checkpoint_json["requestKey"],
        first_checkpoint_json["requestKey"]
    );
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_realtime_message_events_to_other_conversation_member() {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_realtime_fanout",
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
                .uri("/im/v3/api/chat/conversations/c_realtime_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_fanout",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_realtime_fanout/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_fanout_1",
                        "summary":"fanout hello",
                        "text":"fanout hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    assert_eq!(realtime_events_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        realtime_events_json["items"][0]["eventType"],
        "message.posted"
    );
    assert_eq!(
        realtime_events_json["items"][0]["scopeId"],
        "c_realtime_fanout"
    );
    let payload: serde_json::Value = serde_json::from_str(
        realtime_events_json["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_realtime_fanout");
    assert_eq!(payload["summary"], "fanout hello");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_member_joined_to_added_user_scope_for_unknown_group_lists()
 {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_member_joined_user_scope",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"user",
                                "scopeId":"u_other_demo",
                                "eventTypes":["conversation.member_joined"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user-scope subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_joined_user_scope/members/add")
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

    let inbox = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invitee inbox should succeed");
    assert_eq!(inbox.status(), StatusCode::OK);
    let inbox_body = inbox
        .into_body()
        .collect()
        .await
        .expect("invitee inbox body should collect")
        .to_bytes();
    let inbox_json: serde_json::Value =
        serde_json::from_slice(&inbox_body).expect("invitee inbox should be valid json");
    let inbox_items = inbox_json["items"]
        .as_array()
        .expect("inbox items should be array");
    assert!(
        inbox_items
            .iter()
            .any(|item| item["conversationId"] == "c_member_joined_user_scope"),
        "added user inbox must contain the group immediately after membership join"
    );

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    assert_eq!(
        realtime_events_json["items"].as_array().unwrap().len(),
        1,
        "user-scope chat-list subscriptions must receive member joined events for newly available groups"
    );
    assert_eq!(
        realtime_events_json["items"][0]["eventType"],
        "conversation.member_joined"
    );
    assert_eq!(realtime_events_json["items"][0]["scopeType"], "user");
    assert_eq!(realtime_events_json["items"][0]["scopeId"], "u_other_demo");
    let payload: serde_json::Value = serde_json::from_str(
        realtime_events_json["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_member_joined_user_scope");
    assert_eq!(payload["member"]["principalId"], "u_other_demo");
    assert_eq!(payload["member"]["state"], "joined");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_owner_transfer_to_new_owner_user_scope_for_group_management_refresh()
 {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_owner_transfer_user_scope",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_new_owner = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_owner_transfer_user_scope/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_new_owner_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add new owner should succeed");
    assert_eq!(add_new_owner.status(), StatusCode::OK);

    let register_new_owner_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_new_owner_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_new_owner")
                .header("x-sdkwork-session-id", "s_new_owner")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register new owner device should succeed");
    assert_eq!(register_new_owner_device.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_new_owner_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_new_owner")
                .header("x-sdkwork-session-id", "s_new_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"user",
                                "scopeId":"u_new_owner_demo",
                                "eventTypes":["conversation.owner_transferred"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("new-owner user-scope subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let transfer_owner = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_owner_transfer_user_scope/members/transfer_owner")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_owner_transfer_user_scope_user_u_new_owner_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("transfer owner should succeed");
    assert_eq!(transfer_owner.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_new_owner_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_new_owner")
                .header("x-sdkwork-session-id", "s_new_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    assert_eq!(
        realtime_events_json["items"].as_array().unwrap().len(),
        1,
        "new owner user-scope chat-list subscriptions must receive owner transfer events"
    );
    assert_eq!(
        realtime_events_json["items"][0]["eventType"],
        "conversation.owner_transferred"
    );
    assert_eq!(realtime_events_json["items"][0]["scopeType"], "user");
    assert_eq!(
        realtime_events_json["items"][0]["scopeId"],
        "u_new_owner_demo"
    );
    let payload: serde_json::Value = serde_json::from_str(
        realtime_events_json["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_owner_transfer_user_scope");
    assert_eq!(payload["previousOwner"]["principalId"], "u_demo");
    assert_eq!(payload["newOwner"]["principalId"], "u_new_owner_demo");
    assert_eq!(payload["actor"]["id"], "u_demo");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_member_management_events_to_target_user_scopes_for_chat_list_refresh()
 {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_member_target_user_scope",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_target_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_target_user_scope/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_target_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add target member should succeed");
    assert_eq!(add_target_member.status(), StatusCode::OK);

    let add_leaver_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_target_user_scope/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_target_leave_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add leaver member should succeed");
    assert_eq!(add_leaver_member.status(), StatusCode::OK);

    register_client_route_for_test(&app, "u_target_demo", "d_target").await;
    register_client_route_for_test(&app, "u_target_leave_demo", "d_target_leave").await;

    let sync_target_subscriptions = sync_user_realtime_subscription_for_test(
        &app,
        "u_target_demo",
        "d_target",
        &[
            "conversation.member_role_changed",
            "conversation.member_removed",
        ],
    )
    .await;
    assert_eq!(sync_target_subscriptions.status(), StatusCode::OK);
    let sync_leaver_subscriptions = sync_user_realtime_subscription_for_test(
        &app,
        "u_target_leave_demo",
        "d_target_leave",
        &["conversation.member_left"],
    )
    .await;
    assert_eq!(sync_leaver_subscriptions.status(), StatusCode::OK);

    let change_target_role = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_target_user_scope/members/change_role")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_target_user_scope_user_u_target_demo",
                        "role":"admin"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("change target role should succeed");
    assert_eq!(change_target_role.status(), StatusCode::OK);

    let remove_target_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_target_user_scope/members/remove")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_target_user_scope_user_u_target_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove target member should succeed");
    assert_eq!(remove_target_member.status(), StatusCode::OK);

    let leave_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_target_user_scope/members/leave")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_target_leave_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_target_leave")
                .header("x-sdkwork-session-id", "s_d_target_leave")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave conversation should succeed");
    assert_eq!(leave_conversation.status(), StatusCode::OK);

    let target_realtime_events = list_realtime_events_with_session_for_test(
        &app,
        "u_target_demo",
        "d_target",
        Some("s_d_target"),
        0,
    )
    .await;
    assert_eq!(target_realtime_events.status(), StatusCode::OK);
    let target_realtime_events_body = target_realtime_events
        .into_body()
        .collect()
        .await
        .expect("target realtime events body should collect")
        .to_bytes();
    let target_realtime_events_json: serde_json::Value =
        serde_json::from_slice(&target_realtime_events_body)
            .expect("target realtime events should be valid json");
    let target_items = target_realtime_events_json["items"]
        .as_array()
        .expect("target items should be array");
    assert_eq!(
        target_items.len(),
        2,
        "target user-scope chat-list subscriptions must receive role and removal events"
    );
    assert_eq!(
        target_items[0]["eventType"],
        "conversation.member_role_changed"
    );
    assert_eq!(target_items[0]["scopeType"], "user");
    assert_eq!(target_items[0]["scopeId"], "u_target_demo");
    let role_payload: serde_json::Value = serde_json::from_str(
        target_items[0]["payload"]
            .as_str()
            .expect("role changed payload should be string"),
    )
    .expect("role changed payload should be valid json");
    assert_eq!(role_payload["conversationId"], "c_member_target_user_scope");
    assert_eq!(
        role_payload["updatedMember"]["principalId"],
        "u_target_demo"
    );
    assert_eq!(role_payload["updatedMember"]["role"], "admin");
    assert_eq!(target_items[1]["eventType"], "conversation.member_removed");
    assert_eq!(target_items[1]["scopeType"], "user");
    assert_eq!(target_items[1]["scopeId"], "u_target_demo");
    let removed_payload: serde_json::Value = serde_json::from_str(
        target_items[1]["payload"]
            .as_str()
            .expect("removed payload should be string"),
    )
    .expect("removed payload should be valid json");
    assert_eq!(
        removed_payload["conversationId"],
        "c_member_target_user_scope"
    );
    assert_eq!(removed_payload["member"]["principalId"], "u_target_demo");
    assert_eq!(removed_payload["member"]["state"], "removed");

    let leaver_realtime_events = list_realtime_events_with_session_for_test(
        &app,
        "u_target_leave_demo",
        "d_target_leave",
        Some("s_d_target_leave"),
        0,
    )
    .await;
    let leaver_realtime_events_status = leaver_realtime_events.status();
    let leaver_realtime_events_body = leaver_realtime_events
        .into_body()
        .collect()
        .await
        .expect("leaver realtime events body should collect")
        .to_bytes();
    assert_eq!(
        leaver_realtime_events_status,
        StatusCode::OK,
        "leaver realtime events response: {}",
        String::from_utf8_lossy(&leaver_realtime_events_body)
    );
    let leaver_realtime_events_json: serde_json::Value =
        serde_json::from_slice(&leaver_realtime_events_body)
            .expect("leaver realtime events should be valid json");
    let leaver_items = leaver_realtime_events_json["items"]
        .as_array()
        .expect("leaver items should be array");
    assert_eq!(
        leaver_items.len(),
        1,
        "leaver user-scope chat-list subscriptions must receive member-left events"
    );
    assert_eq!(leaver_items[0]["eventType"], "conversation.member_left");
    assert_eq!(leaver_items[0]["scopeType"], "user");
    assert_eq!(leaver_items[0]["scopeId"], "u_target_leave_demo");
    let left_payload: serde_json::Value = serde_json::from_str(
        leaver_items[0]["payload"]
            .as_str()
            .expect("left payload should be string"),
    )
    .expect("left payload should be valid json");
    assert_eq!(left_payload["conversationId"], "c_member_target_user_scope");
    assert_eq!(left_payload["member"]["principalId"], "u_target_leave_demo");
    assert_eq!(left_payload["member"]["state"], "left");
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_message_events_to_member_user_scope_for_unknown_conversation_lists()
 {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_realtime_user_scope_fanout",
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
                .uri("/im/v3/api/chat/conversations/c_realtime_user_scope_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"user",
                                "scopeId":"u_other_demo",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user-scope subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_realtime_user_scope_fanout/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_user_scope_fanout_1",
                        "summary":"user scope fanout hello",
                        "text":"user scope fanout hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    assert_eq!(
        realtime_events_json["items"].as_array().unwrap().len(),
        1,
        "user-scope chat-list subscriptions must receive message events even before the client knows the conversation id"
    );
    assert_eq!(
        realtime_events_json["items"][0]["eventType"],
        "message.posted"
    );
    assert_eq!(realtime_events_json["items"][0]["scopeType"], "user");
    assert_eq!(realtime_events_json["items"][0]["scopeId"], "u_other_demo");
    let payload: serde_json::Value = serde_json::from_str(
        realtime_events_json["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_realtime_user_scope_fanout");
    assert_eq!(payload["summary"], "user scope fanout hello");
}

#[tokio::test]
async fn test_local_minimal_profile_replays_pending_message_events_to_member_user_scope_for_unknown_conversation_lists()
 {
    let runtime_dir = unique_test_runtime_dir("realtime_user_scope_outbox_replay");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

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
                        "conversationId":"c_realtime_user_scope_replay",
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
                .uri("/im/v3/api/chat/conversations/c_realtime_user_scope_replay/members/add")
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

    let create_trigger_conversation = app
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
                        "conversationId":"c_realtime_user_scope_replay_trigger",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create trigger conversation should succeed");
    assert_eq!(create_trigger_conversation.status(), StatusCode::OK);

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"user",
                                "scopeId":"u_other_demo",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user-scope subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let pending_payload = serde_json::json!({
        "body": {
            "parts": [{ "kind": "text", "text": "pending user scope replay hello" }],
            "summary": "pending user scope replay hello"
        },
        "conversationId": "c_realtime_user_scope_replay",
        "deliveryMode": "discrete",
        "messageId": "msg_pending_user_scope_replay",
        "messageSeq": 7,
        "messageType": "standard",
        "occurredAt": "2026-06-08T10:00:00.000Z",
        "sender": {
            "id": "u_demo",
            "kind": "user",
            "metadata": {}
        },
        "summary": "pending user scope replay hello"
    })
    .to_string();
    let outbox_path = runtime_dir
        .join("state")
        .join("message-side-effect-outbox.json");
    fs::write(
        outbox_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "outbox_pending_user_scope_replay": {
                "outboxId": "outbox_pending_user_scope_replay",
                "tenantId": "t_demo",
                "actorId": "u_demo",
                "actorKind": "user",
                "actorSessionId": "s_owner",
                "actorDeviceId": "d_owner",
                "conversationId": "c_realtime_user_scope_replay",
                "messageId": "msg_pending_user_scope_replay",
                "messageSeq": 7,
                "sideEffect": "realtime_delivery",
                "scopeType": "conversation",
                "scopeId": "c_realtime_user_scope_replay",
                "eventType": "message.posted",
                "payload": pending_payload,
                "status": "pending",
                "attemptCount": 0,
                "lastErrorCode": null,
                "lastErrorMessage": null,
                "createdAt": "2026-06-08T10:00:00.000Z",
                "updatedAt": "2026-06-08T10:00:00.000Z"
            }
        }))
        .expect("outbox record should serialize"),
    )
    .expect("pending outbox record should be written");

    let trigger_drain = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_realtime_user_scope_replay_trigger/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_user_scope_replay_trigger_1",
                        "summary":"trigger outbox replay",
                        "text":"trigger outbox replay"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("trigger message should succeed");
    assert_eq!(trigger_drain.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    assert_eq!(
        realtime_events_json["items"].as_array().unwrap().len(),
        1,
        "pending message realtime outbox replay must self-heal user-scope chat lists"
    );
    assert_eq!(
        realtime_events_json["items"][0]["eventType"],
        "message.posted"
    );
    assert_eq!(realtime_events_json["items"][0]["scopeType"], "user");
    assert_eq!(realtime_events_json["items"][0]["scopeId"], "u_other_demo");
    let payload: serde_json::Value = serde_json::from_str(
        realtime_events_json["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_realtime_user_scope_replay");
    assert_eq!(payload["messageId"], "msg_pending_user_scope_replay");
    assert_eq!(payload["summary"], "pending user scope replay hello");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_message_mutation_realtime_events_to_other_conversation_member()
 {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_realtime_mutation_fanout",
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
                .uri("/im/v3/api/chat/conversations/c_realtime_mutation_fanout/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_realtime_mutation_fanout",
                                "eventTypes":["message.posted","message.edited","message.recalled"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_realtime_mutation_fanout/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_realtime_mutation_fanout_1",
                        "summary":"posted hello",
                        "text":"posted hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let edit_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/messages/msg_c_realtime_mutation_fanout_1/edit")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "summary":"edited hello",
                        "text":"edited hello"
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
                .uri("/im/v3/api/chat/messages/msg_c_realtime_mutation_fanout_1/recall")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("recall message should succeed");
    assert_eq!(recall_message.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 3);
    assert_eq!(items[0]["eventType"], "message.posted");
    assert_eq!(items[1]["eventType"], "message.edited");
    assert_eq!(items[2]["eventType"], "message.recalled");

    let posted_payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("posted payload should be string"),
    )
    .expect("posted payload should be valid json");
    assert_eq!(
        posted_payload["conversationId"],
        "c_realtime_mutation_fanout"
    );
    assert_eq!(posted_payload["summary"], "posted hello");

    let edited_payload: serde_json::Value = serde_json::from_str(
        items[1]["payload"]
            .as_str()
            .expect("edited payload should be string"),
    )
    .expect("edited payload should be valid json");
    assert_eq!(
        edited_payload["conversationId"],
        "c_realtime_mutation_fanout"
    );
    assert_eq!(edited_payload["summary"], "edited hello");
    assert_eq!(
        edited_payload["messageId"],
        "msg_c_realtime_mutation_fanout_1"
    );

    let recalled_payload: serde_json::Value = serde_json::from_str(
        items[2]["payload"]
            .as_str()
            .expect("recalled payload should be string"),
    )
    .expect("recalled payload should be valid json");
    assert_eq!(
        recalled_payload["conversationId"],
        "c_realtime_mutation_fanout"
    );
    assert_eq!(
        recalled_payload["messageId"],
        "msg_c_realtime_mutation_fanout_1"
    );
    assert_eq!(realtime_events_json["nextAfterSeq"], 3);
    assert_eq!(realtime_events_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_client_route()
 {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_member_realtime",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_member_realtime",
                                "eventTypes":[
                                    "conversation.member_joined",
                                    "conversation.member_role_changed",
                                    "conversation.member_removed",
                                    "conversation.member_left"
                                ]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let add_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_realtime/members/add")
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
        .expect("add other member should succeed");
    assert_eq!(add_other_member.status(), StatusCode::OK);

    let change_other_role = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_realtime/members/change_role")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_realtime_user_u_other_demo",
                        "role":"admin"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("change role should succeed");
    assert_eq!(change_other_role.status(), StatusCode::OK);

    let remove_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_realtime/members/remove")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_realtime_user_u_other_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove other member should succeed");
    assert_eq!(remove_other_member.status(), StatusCode::OK);

    let add_leaver = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_realtime/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_leave_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add leaver should succeed");
    assert_eq!(add_leaver.status(), StatusCode::OK);

    let leave_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_realtime/members/leave")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_leave_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_leave")
                .header("x-sdkwork-session-id", "s_leave")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave conversation should succeed");
    assert_eq!(leave_conversation.status(), StatusCode::OK);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 5);
    assert_eq!(items[0]["eventType"], "conversation.member_joined");
    assert_eq!(items[1]["eventType"], "conversation.member_role_changed");
    assert_eq!(items[2]["eventType"], "conversation.member_removed");
    assert_eq!(items[3]["eventType"], "conversation.member_joined");
    assert_eq!(items[4]["eventType"], "conversation.member_left");

    let joined_payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("joined payload should be string"),
    )
    .expect("joined payload should be valid json");
    assert_eq!(joined_payload["conversationId"], "c_member_realtime");
    assert_eq!(joined_payload["member"]["principalId"], "u_other_demo");
    assert_eq!(joined_payload["member"]["state"], "joined");
    assert_eq!(joined_payload["actor"]["id"], "u_demo");

    let role_changed_payload: serde_json::Value = serde_json::from_str(
        items[1]["payload"]
            .as_str()
            .expect("role changed payload should be string"),
    )
    .expect("role changed payload should be valid json");
    assert_eq!(role_changed_payload["conversationId"], "c_member_realtime");
    assert_eq!(role_changed_payload["previousMember"]["role"], "member");
    assert_eq!(role_changed_payload["updatedMember"]["role"], "admin");
    assert_eq!(role_changed_payload["actor"]["id"], "u_demo");

    let removed_payload: serde_json::Value = serde_json::from_str(
        items[2]["payload"]
            .as_str()
            .expect("removed payload should be string"),
    )
    .expect("removed payload should be valid json");
    assert_eq!(removed_payload["member"]["principalId"], "u_other_demo");
    assert_eq!(removed_payload["member"]["state"], "removed");

    let left_payload: serde_json::Value = serde_json::from_str(
        items[4]["payload"]
            .as_str()
            .expect("left payload should be string"),
    )
    .expect("left payload should be valid json");
    assert_eq!(left_payload["member"]["principalId"], "u_leave_demo");
    assert_eq!(left_payload["member"]["state"], "left");
    assert_eq!(left_payload["actor"]["id"], "u_leave_demo");
    assert_eq!(realtime_events_json["nextAfterSeq"], 5);
    assert_eq!(realtime_events_json["hasMore"], false);
}

#[tokio::test]
async fn test_local_minimal_profile_member_governance_rejects_actor_kind_mismatch_before_side_effects()
 {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_member_actor_kind_sync",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_member_actor_kind_sync",
                                "eventTypes":[
                                    "conversation.member_joined",
                                    "conversation.member_role_changed",
                                    "conversation.member_removed",
                                    "conversation.member_left"
                                ]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let add_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_actor_kind_sync/members/add")
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
        .expect("add other member should succeed");
    assert_eq!(add_other_member.status(), StatusCode::OK);

    let add_leaver = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_actor_kind_sync/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_leave_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add leaver should succeed");
    assert_eq!(add_leaver.status(), StatusCode::OK);

    let change_other_role = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_actor_kind_sync/members/change_role")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "agent")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_actor_kind_sync_user_u_other_demo",
                        "role":"admin"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("change role should return response");
    assert_eq!(change_other_role.status(), StatusCode::FORBIDDEN);

    let remove_other_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_actor_kind_sync/members/remove")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "agent")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_member_actor_kind_sync_user_u_other_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("remove other member should return response");
    assert_eq!(remove_other_member.status(), StatusCode::FORBIDDEN);

    let leave_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_member_actor_kind_sync/members/leave")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_leave_demo")
                .header("x-sdkwork-actor-kind", "system")
                .header("x-sdkwork-device-id", "d_leave")
                .header("x-sdkwork-session-id", "s_leave")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("leave conversation should return response");
    assert_eq!(leave_conversation.status(), StatusCode::FORBIDDEN);

    let realtime_events = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_pad")
                .header("x-sdkwork-session-id", "s_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["eventType"], "conversation.member_joined");
    assert_eq!(items[1]["eventType"], "conversation.member_joined");

    let first_joined_payload: serde_json::Value = serde_json::from_str(
        items[0]["payload"]
            .as_str()
            .expect("first joined payload should be string"),
    )
    .expect("first joined payload should be valid json");
    assert_eq!(first_joined_payload["actor"]["id"], "u_demo");
    assert_eq!(first_joined_payload["actor"]["kind"], "user");
    assert_eq!(
        first_joined_payload["member"]["principalId"],
        "u_other_demo"
    );

    let second_joined_payload: serde_json::Value = serde_json::from_str(
        items[1]["payload"]
            .as_str()
            .expect("second joined payload should be string"),
    )
    .expect("second joined payload should be valid json");
    assert_eq!(second_joined_payload["actor"]["id"], "u_demo");
    assert_eq!(second_joined_payload["actor"]["kind"], "user");
    assert_eq!(
        second_joined_payload["member"]["principalId"],
        "u_leave_demo"
    );
    assert_eq!(realtime_events_json["nextAfterSeq"], 2);
    assert_eq!(realtime_events_json["hasMore"], false);

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_member_actor_kind_sync/members")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    let member_items = members_json["items"]
        .as_array()
        .expect("member items should be array");
    assert_eq!(member_items.len(), 3);
    let other_member = member_items
        .iter()
        .find(|item| item["principalId"] == "u_other_demo")
        .expect("other member should exist");
    assert_eq!(other_member["role"], "member");
    assert_eq!(other_member["state"], "joined");
    let leave_member = member_items
        .iter()
        .find(|item| item["principalId"] == "u_leave_demo")
        .expect("leave member should exist");
    assert_eq!(leave_member["state"], "joined");

    let audit_export = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/export")
                .header("x-sdkwork-permission-scope", "audit.read")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should succeed");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_export_body = audit_export
        .into_body()
        .collect()
        .await
        .expect("audit export body should collect")
        .to_bytes();
    let audit_export_json: serde_json::Value =
        serde_json::from_slice(&audit_export_body).expect("audit export should be valid json");
    let audit_items = audit_export_json["items"]
        .as_array()
        .expect("audit items should be array");
    let governance_actions = [
        "conversation.member_joined",
        "conversation.member_role_changed",
        "conversation.member_removed",
        "conversation.member_left",
    ];
    let governance_items: Vec<&serde_json::Value> = audit_items
        .iter()
        .filter(|item| {
            item["action"]
                .as_str()
                .is_some_and(|action| governance_actions.contains(&action))
        })
        .collect();
    assert_eq!(governance_items.len(), 2);
    for item in governance_items {
        assert_eq!(item["actorKind"], "user");
        assert_eq!(item["action"], "conversation.member_joined");
    }
}

#[tokio::test]
async fn test_local_minimal_profile_owner_transfer_rejects_actor_kind_mismatch_before_audit() {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_owner_transfer_actor_kind_sync",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_target_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_owner_transfer_actor_kind_sync/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_target_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add target member should succeed");
    assert_eq!(add_target_member.status(), StatusCode::OK);

    let transfer_owner = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(
                    "/im/v3/api/chat/conversations/c_owner_transfer_actor_kind_sync/members/transfer_owner",
                )
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "agent")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "memberId":"cm_c_owner_transfer_actor_kind_sync_user_u_target_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("transfer owner should return response");
    assert_eq!(transfer_owner.status(), StatusCode::FORBIDDEN);

    let audit_export = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/export")
                .header("x-sdkwork-permission-scope", "audit.read")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should succeed");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_export_body = audit_export
        .into_body()
        .collect()
        .await
        .expect("audit export body should collect")
        .to_bytes();
    let audit_export_json: serde_json::Value =
        serde_json::from_slice(&audit_export_body).expect("audit export should be valid json");
    let owner_transfer_item = audit_export_json["items"]
        .as_array()
        .expect("audit items should be array")
        .iter()
        .find(|item| item["action"] == "conversation.owner_transferred");
    assert!(owner_transfer_item.is_none());

    let members = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_owner_transfer_actor_kind_sync/members")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should succeed");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members should be valid json");
    let member_items = members_json["items"]
        .as_array()
        .expect("member items should be array");
    let owner = member_items
        .iter()
        .find(|item| item["principalId"] == "u_demo")
        .expect("owner should exist");
    assert_eq!(owner["role"], "owner");
    let target = member_items
        .iter()
        .find(|item| item["principalId"] == "u_target_demo")
        .expect("target should exist");
    assert_eq!(target["role"], "member");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_open_stream_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let first_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_open_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first open stream should succeed");
    assert_eq!(first_open.status(), StatusCode::OK);
    let first_open_body = first_open
        .into_body()
        .collect()
        .await
        .expect("first open body should collect")
        .to_bytes();
    let first_open_json: serde_json::Value =
        serde_json::from_slice(&first_open_body).expect("first open should be valid json");
    assert_eq!(first_open_json["deliveryStatus"], "applied");
    assert_eq!(
        first_open_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_local_open_idempotent/frames")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let idempotent_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_open_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent open stream should return response");
    assert_eq!(idempotent_open.status(), StatusCode::OK);
    let idempotent_open_body = idempotent_open
        .into_body()
        .collect()
        .await
        .expect("idempotent open body should collect")
        .to_bytes();
    let idempotent_open_json: serde_json::Value = serde_json::from_slice(&idempotent_open_body)
        .expect("idempotent open should be valid json");
    assert_eq!(idempotent_open_json["state"], "active");
    assert_eq!(idempotent_open_json["lastFrameSeq"], 1);
    assert_eq!(idempotent_open_json["deliveryStatus"], "replayed");
    assert_eq!(
        idempotent_open_json["requestKey"],
        first_open_json["requestKey"]
    );
    assert_eq!(
        idempotent_open_json["proofVersion"],
        first_open_json["proofVersion"]
    );

    let list_frames = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_local_open_idempotent/frames?afterFrameSeq=0&limit=10")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames should return response");
    assert_eq!(list_frames.status(), StatusCode::OK);
    let list_frames_body = list_frames
        .into_body()
        .collect()
        .await
        .expect("list frames body should collect")
        .to_bytes();
    let list_frames_json: serde_json::Value =
        serde_json::from_slice(&list_frames_body).expect("list frames should be valid json");
    assert_eq!(list_frames_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(list_frames_json["items"][0]["frameSeq"], 1);

    let conflicting_open = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_open_idempotent",
                        "streamType":"custom.delta.binary",
                        "scopeKind":"conversation",
                        "scopeId":"c_other",
                        "durabilityClass":"eventLog",
                        "schemaRef":"custom.delta.binary.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting open stream should return response");
    assert_eq!(conflicting_open.status(), StatusCode::CONFLICT);
    let conflicting_open_body = conflicting_open
        .into_body()
        .collect()
        .await
        .expect("conflicting open body should collect")
        .to_bytes();
    let conflicting_open_json: serde_json::Value = serde_json::from_slice(&conflicting_open_body)
        .expect("conflicting open should be valid json");
    assert_eq!(conflicting_open_json["code"], "stream_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_duplicate_open_stream_from_different_actor() {
    let app = local_minimal_node::build_default_app();

    let first_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_actor_scope_open",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first open stream should succeed");
    assert_eq!(first_open.status(), StatusCode::OK);
    let first_open_body = first_open
        .into_body()
        .collect()
        .await
        .expect("first open body should collect")
        .to_bytes();
    let first_open_json: serde_json::Value =
        serde_json::from_slice(&first_open_body).expect("first open should be valid json");
    assert_eq!(first_open_json["deliveryStatus"], "applied");
    assert!(
        first_open_json["requestKey"]
            .as_str()
            .expect("first open requestKey should be present")
            .contains("6#u_demo4#open25#st_local_actor_scope_open")
    );

    let conflicting_open = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_actor_scope_open",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("different actor open stream should return response");
    assert_eq!(conflicting_open.status(), StatusCode::CONFLICT);
    let conflicting_open_body = conflicting_open
        .into_body()
        .collect()
        .await
        .expect("different actor open body should collect")
        .to_bytes();
    let conflicting_open_json: serde_json::Value = serde_json::from_slice(&conflicting_open_body)
        .expect("different actor open should be valid json");
    assert_eq!(conflicting_open_json["code"], "stream_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_request_stream_list_from_different_actor() {
    let app = local_minimal_node::build_default_app();

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_local_request_scope_owner_only_list",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_local_request_scope_owner_only_list/frames")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("owner append should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let list_frames = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/streams/st_local_request_scope_owner_only_list/frames?afterFrameSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("different actor list should return response");
    assert_eq!(list_frames.status(), StatusCode::NOT_FOUND);
    let list_frames_body = list_frames
        .into_body()
        .collect()
        .await
        .expect("different actor list body should collect")
        .to_bytes();
    let list_frames_json: serde_json::Value = serde_json::from_slice(&list_frames_body)
        .expect("different actor list should be valid json");
    assert_eq!(list_frames_json["code"], "stream_not_found");
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_refanout_duplicate_stream_complete_retry() {
    let app = local_minimal_node::build_default_app();

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
                        "conversationId":"c_stream_complete_idempotent",
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
                .uri("/im/v3/api/chat/conversations/c_stream_complete_idempotent/members/add")
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

    let register_other_device = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register other device should succeed");
    assert_eq!(register_other_device.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_complete_retry_fanout",
                        "streamType":"custom.delta.text",
                        "scopeKind":"conversation",
                        "scopeId":"c_stream_complete_idempotent",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let sync_subscriptions = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"stream",
                                "scopeId":"st_complete_retry_fanout",
                                "eventTypes":["stream.completed"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync should succeed");
    assert_eq!(sync_subscriptions.status(), StatusCode::OK);

    let append_frame = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_complete_retry_fanout/frames")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame should succeed");
    assert_eq!(append_frame.status(), StatusCode::OK);

    let first_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_complete_retry_fanout/complete")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_complete_retry_fanout"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first complete should return response");
    assert_eq!(first_complete.status(), StatusCode::OK);
    let first_complete_body = first_complete
        .into_body()
        .collect()
        .await
        .expect("first complete body should collect")
        .to_bytes();
    let first_complete_json: serde_json::Value =
        serde_json::from_slice(&first_complete_body).expect("first complete should be valid json");
    assert_eq!(first_complete_json["deliveryStatus"], "applied");
    assert_eq!(
        first_complete_json["proofVersion"],
        "stream.session.delivery-proof.v1"
    );

    let duplicate_complete = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams/st_complete_retry_fanout/complete")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_complete_retry_fanout"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate complete should return response");
    assert_eq!(duplicate_complete.status(), StatusCode::OK);
    let duplicate_complete_body = duplicate_complete
        .into_body()
        .collect()
        .await
        .expect("duplicate complete body should collect")
        .to_bytes();
    let duplicate_complete_json: serde_json::Value =
        serde_json::from_slice(&duplicate_complete_body)
            .expect("duplicate complete should be valid json");
    assert_eq!(duplicate_complete_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_complete_json["requestKey"],
        first_complete_json["requestKey"]
    );

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_other_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_other")
                .header("x-sdkwork-session-id", "s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    let items = realtime_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["eventType"], "stream.completed");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_conflicting_invite_after_accept_without_new_signal() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_rtc_invite_conflict",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_invite_conflict",
                        "conversationId":"c_rtc_invite_conflict",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let first_invite = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_invite_conflict/invite")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_invite_initial"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first invite should succeed");
    assert_eq!(first_invite.status(), StatusCode::OK);

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_invite_conflict/accept")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_accept_once"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept should succeed");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let conflicting_invite = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_invite_conflict/invite")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_invite_conflict"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting invite should return response");
    assert_eq!(conflicting_invite.status(), StatusCode::CONFLICT);
    let conflicting_invite_body = conflicting_invite
        .into_body()
        .collect()
        .await
        .expect("conflicting invite body should collect")
        .to_bytes();
    let conflicting_invite_json: serde_json::Value =
        serde_json::from_slice(&conflicting_invite_body)
            .expect("conflicting invite should be valid json");
    assert_eq!(
        conflicting_invite_json["code"],
        "rtc_session_state_conflict"
    );

    let timeline = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_rtc_invite_conflict/messages")
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline should succeed");
    assert_eq!(timeline.status(), StatusCode::OK);
    let timeline_body = timeline
        .into_body()
        .collect()
        .await
        .expect("timeline body should collect")
        .to_bytes();
    let timeline_json: serde_json::Value =
        serde_json::from_slice(&timeline_body).expect("timeline should be valid json");
    let items = timeline_json["items"]
        .as_array()
        .expect("timeline items should be array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["summary"], "rtc.invite");
    assert_eq!(items[1]["summary"], "rtc.accept");
}

#[tokio::test]
async fn test_local_minimal_profile_issues_rtc_participant_credential_over_http() {
    let app = local_minimal_node::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_local_provider_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let credential_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions/rtc_local_provider_http/credentials")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "participantId":"u_peer"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("issue rtc credential request should return response");

    assert_eq!(credential_response.status(), StatusCode::OK);
    let credential_body = credential_response
        .into_body()
        .collect()
        .await
        .expect("credential body should collect")
        .to_bytes();
    let credential_json: serde_json::Value =
        serde_json::from_slice(&credential_body).expect("credential response should be valid json");

    assert_eq!(credential_json["tenantId"], "t_demo");
    assert_eq!(credential_json["rtcSessionId"], "rtc_local_provider_http");
    assert_eq!(credential_json["participantId"], "u_peer");
    assert_eq!(
        credential_json["credential"],
        "volcengine-token:t_demo:rtc_local_provider_http:u_peer"
    );
    assert!(credential_json["expiresAt"].as_str().is_some());
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_audit_payload_over_http() {
    let app = local_minimal_node::build_default_app();
    let request_body = serde_json::json!({
        "recordId": "audit_local_oversized_payload",
        "aggregateType": "notification",
        "aggregateId": "ntf_local_oversized_payload",
        "action": "notification.requested",
        "payload": "x".repeat(200_000)
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized local audit payload should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("payload")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_audit_anchor_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_local_idempotent",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_local_idempotent",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"u_target\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first local audit record should succeed");
    assert_eq!(first_record.status(), StatusCode::OK);
    let first_record_body = first_record
        .into_body()
        .collect()
        .await
        .expect("first local audit body should collect")
        .to_bytes();
    let first_record_json: serde_json::Value =
        serde_json::from_slice(&first_record_body).expect("first local audit should be valid json");
    assert_eq!(first_record_json["deliveryStatus"], "applied");
    assert_eq!(
        first_record_json["proofVersion"],
        "audit.record.delivery-proof.v1"
    );

    let duplicate_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_local_idempotent",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_local_idempotent",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"u_target\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate local audit record should return response");
    assert_eq!(duplicate_record.status(), StatusCode::OK);
    let duplicate_record_body = duplicate_record
        .into_body()
        .collect()
        .await
        .expect("duplicate local audit body should collect")
        .to_bytes();
    let duplicate_record_json: serde_json::Value = serde_json::from_slice(&duplicate_record_body)
        .expect("duplicate local audit should be valid json");
    assert_eq!(duplicate_record_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_record_json["requestKey"],
        first_record_json["requestKey"]
    );

    let list_records = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/records")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list local audit records should succeed");
    assert_eq!(list_records.status(), StatusCode::OK);
    let list_records_body = list_records
        .into_body()
        .collect()
        .await
        .expect("list local audit body should collect")
        .to_bytes();
    let list_records_json: serde_json::Value =
        serde_json::from_slice(&list_records_body).expect("list local audit should be valid json");
    assert_eq!(list_records_json["items"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn test_local_minimal_profile_replays_duplicate_audit_anchor_after_session_rotation() {
    let app = local_minimal_node::build_default_app();

    let first_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-session-id", "s_before")
                .header("x-sdkwork-permission-scope", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_local_session_rotation",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_local_session_rotation",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"u_target\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first local audit record should succeed");
    assert_eq!(first_record.status(), StatusCode::OK);
    let first_record_body = first_record
        .into_body()
        .collect()
        .await
        .expect("first local audit body should collect")
        .to_bytes();
    let first_record_json: serde_json::Value =
        serde_json::from_slice(&first_record_body).expect("first local audit should be valid json");
    assert_eq!(first_record_json["deliveryStatus"], "applied");

    let duplicate_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-session-id", "s_after")
                .header("x-sdkwork-permission-scope", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_local_session_rotation",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_local_session_rotation",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"u_target\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate local audit record should return response after session rotation");
    assert_eq!(duplicate_record.status(), StatusCode::OK);
    let duplicate_record_body = duplicate_record
        .into_body()
        .collect()
        .await
        .expect("duplicate local audit body should collect")
        .to_bytes();
    let duplicate_record_json: serde_json::Value = serde_json::from_slice(&duplicate_record_body)
        .expect("duplicate local audit should be valid json");
    assert_eq!(duplicate_record_json["deliveryStatus"], "replayed");
    assert_eq!(
        duplicate_record_json["requestKey"],
        first_record_json["requestKey"]
    );

    let list_records = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/records")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list local audit records should succeed");
    assert_eq!(list_records.status(), StatusCode::OK);
    let list_records_body = list_records
        .into_body()
        .collect()
        .await
        .expect("list local audit body should collect")
        .to_bytes();
    let list_records_json: serde_json::Value =
        serde_json::from_slice(&list_records_body).expect("list local audit should be valid json");
    assert_eq!(list_records_json["items"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_client_route_id_on_register_over_http() {
    let app = local_minimal_node::build_default_app();
    let request_body = serde_json::json!({
        "deviceId": "d".repeat(2048)
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized device register should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("deviceId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_conversation_id_on_timeline_query_over_http()
{
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}/messages",
                    "c".repeat(2048)
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oversized local timeline query should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("conversationId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_stream_id_on_list_frames_over_http() {
    let app = local_minimal_node::build_default_app();
    let oversized_stream_id = "s".repeat(2048);

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/streams/{oversized_stream_id}/frames?afterFrameSeq=0&limit=10"
                ))
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oversized local list frames request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("oversized local list frames body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("oversized local list frames should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("streamId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_projection_read_routes_for_contacts_directory_and_interactions()
 {
    let contacts_app = local_minimal_node::build_default_app();
    let contacts_fixture = create_active_friendship_direct_chat_fixture(&contacts_app).await;

    let service = Arc::new(TimelineProjectionService::default());
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_local_projection_owner_joined",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_projection_local",
                1,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_projection_local",
                    "memberId":"cm_u_owner",
                    "principalId":"u_owner",
                    "principalKind":"user",
                    "role":"owner",
                    "state":"joined",
                    "invitedBy":null,
                    "joinedAt":"2026-04-10T12:00:00Z",
                    "removedAt":null,
                    "attributes":{"displayName":"Owner"}
                }"#,
            ),
        )
        .expect("owner projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_local_projection_member_joined",
                "t_demo",
                "conversation.member_joined",
                "conversation",
                "c_projection_local",
                2,
            )
            .with_payload(
                "conversation.member.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_projection_local",
                    "memberId":"cm_u_member",
                    "principalId":"u_member",
                    "principalKind":"user",
                    "role":"member",
                    "state":"joined",
                    "invitedBy":"u_owner",
                    "joinedAt":"2026-04-10T12:00:01Z",
                    "removedAt":null,
                    "attributes":{"displayName":"Member"}
                }"#,
            ),
        )
        .expect("member projection should succeed");
    service
        .apply(
            &im_domain_events::CommitEnvelope::minimal(
                "evt_local_projection_message_posted",
                "t_demo",
                "message.posted",
                "conversation",
                "c_projection_local",
                3,
            )
            .with_payload(
                "message.posted.v1",
                r#"{
                    "tenantId":"t_demo",
                    "conversationId":"c_projection_local",
                    "messageId":"msg_c_projection_local_1",
                    "messageSeq":1,
                    "sender":{"id":"u_owner","kind":"user","memberId":"cm_u_owner","deviceId":"d_u_owner","sessionId":"s_u_owner","metadata":{}},
                    "messageType":"standard",
                    "deliveryMode":"discrete",
                    "clientMsgId":"client_projection_local_1",
                    "streamSessionId":null,
                    "rtcSessionId":null,
                    "body":{"summary":"projection local","parts":[{"kind":"text","text":"projection local"}],"renderHints":{}},
                    "attributes":{},
                    "metadata":{},
                    "occurredAt":"2026-04-10T12:00:02Z",
                    "committedAt":"2026-04-10T12:00:02Z"
                }"#,
            ),
        )
        .expect("message projection should succeed");
    service
        .apply(&message_reaction_added_event(
            "t_demo",
            "c_projection_local",
            "msg_c_projection_local_1",
            1,
            "thumbs_up",
            "u_owner",
            "2026-04-10T12:00:10Z",
        ))
        .expect("first reaction projection should succeed");
    service
        .apply(&message_reaction_added_event(
            "t_demo",
            "c_projection_local",
            "msg_c_projection_local_1",
            1,
            "thumbs_up",
            "u_member",
            "2026-04-10T12:00:11Z",
        ))
        .expect("second reaction projection should succeed");
    service
        .apply(&message_pinned_event(
            "t_demo",
            "c_projection_local",
            "msg_c_projection_local_1",
            1,
            "u_owner",
            "2026-04-10T12:00:20Z",
        ))
        .expect("pin projection should succeed");

    let projection_app = local_minimal_node::build_app_with_dependencies(
        "local_projection_routes",
        "127.0.0.1:18124",
        service,
        Arc::new(RealtimeClusterBridge::default()),
    );

    let contacts_response = contacts_app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts request should return response");
    assert_eq!(contacts_response.status(), StatusCode::OK);
    let contacts_body = contacts_response
        .into_body()
        .collect()
        .await
        .expect("contacts body should collect")
        .to_bytes();
    let contacts_value: serde_json::Value =
        serde_json::from_slice(&contacts_body).expect("contacts body should be valid json");
    let contacts_items = contacts_value["items"]
        .as_array()
        .expect("contacts items should be an array");
    assert_eq!(contacts_items.len(), 1);
    assert_eq!(contacts_items[0]["targetUserId"], "u_bob");
    assert_eq!(
        contacts_items[0]["conversationId"],
        contacts_fixture.conversation_id
    );
    assert_eq!(
        contacts_items[0]["friendshipId"],
        contacts_fixture.friendship_id
    );

    let member_directory_response = projection_app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_projection_local/member_directory")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member directory request should return response");
    assert_eq!(member_directory_response.status(), StatusCode::OK);
    let member_directory_body = member_directory_response
        .into_body()
        .collect()
        .await
        .expect("member directory body should collect")
        .to_bytes();
    let member_directory_value: serde_json::Value = serde_json::from_slice(&member_directory_body)
        .expect("member directory body should be valid json");
    let directory_items = member_directory_value["items"]
        .as_array()
        .expect("member directory items should be an array");
    assert_eq!(directory_items.len(), 2);
    assert_eq!(directory_items[0]["principalId"], "u_owner");
    assert_eq!(directory_items[1]["attributes"]["displayName"], "Member");

    let interaction_response = projection_app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_projection_local/messages/msg_c_projection_local_1/interaction_summary")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("interaction summary request should return response");
    assert_eq!(interaction_response.status(), StatusCode::OK);
    let interaction_body = interaction_response
        .into_body()
        .collect()
        .await
        .expect("interaction summary body should collect")
        .to_bytes();
    let interaction_value: serde_json::Value = serde_json::from_slice(&interaction_body)
        .expect("interaction summary body should be valid json");
    assert_eq!(interaction_value["messageId"], "msg_c_projection_local_1");
    assert_eq!(interaction_value["totalReactionCount"], 2);
    assert_eq!(
        interaction_value["reactionCounts"][0]["reactionKey"],
        "thumbs_up"
    );
    assert_eq!(interaction_value["pin"]["pinnedBy"]["id"], "u_owner");

    let pins_response = projection_app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_projection_local/pins")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_member")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pins request should return response");
    assert_eq!(pins_response.status(), StatusCode::OK);
    let pins_body = pins_response
        .into_body()
        .collect()
        .await
        .expect("pins body should collect")
        .to_bytes();
    let pins_value: serde_json::Value =
        serde_json::from_slice(&pins_body).expect("pins body should be valid json");
    let pin_items = pins_value["items"]
        .as_array()
        .expect("pins items should be an array");
    assert_eq!(pin_items.len(), 1);
    assert_eq!(pin_items[0]["messageId"], "msg_c_projection_local_1");
    assert_eq!(pin_items[0]["pin"]["pinnedAt"], "2026-04-10T12:00:20Z");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_same_actor_id_with_different_actor_kind_on_contacts_query()
 {
    let service = Arc::new(TimelineProjectionService::default());
    service
        .apply(&friendship_activated_event(
            "t_demo",
            "fs_local_actor_kind_contacts",
            "u_alice",
            "u_bob",
            Some("dc_local_actor_kind_contacts"),
            "2026-04-13T12:00:00Z",
        ))
        .expect("friendship projection should succeed");
    service
        .apply(&direct_chat_bound_event(
            "t_demo",
            "dc_local_actor_kind_contacts",
            "c_local_actor_kind_contacts",
            "2026-04-13T12:05:00Z",
        ))
        .expect("direct chat bind projection should succeed");

    let app = local_minimal_node::build_app_with_dependencies(
        "local_projection_actor_kind_contacts",
        "127.0.0.1:18125",
        service,
        Arc::new(RealtimeClusterBridge::default()),
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("actor-kind mismatch contacts request should return response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "contact_scope_forbidden");
}

#[tokio::test]
async fn test_local_minimal_profile_hides_removed_friendship_from_contacts() {
    let app = local_minimal_node::build_default_app();
    let fixture = create_active_friendship_direct_chat_fixture(&app).await;

    let contacts_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts request before removal should return response");
    assert_eq!(contacts_before.status(), StatusCode::OK);
    let contacts_before_body = contacts_before
        .into_body()
        .collect()
        .await
        .expect("contacts before body should collect")
        .to_bytes();
    let contacts_before_json: serde_json::Value = serde_json::from_slice(&contacts_before_body)
        .expect("contacts before body should be valid json");
    let contacts_before_items = contacts_before_json["items"]
        .as_array()
        .expect("contacts before items should be an array");
    assert_eq!(contacts_before_items.len(), 1);
    assert_eq!(contacts_before_items[0]["targetUserId"], "u_bob");
    assert_eq!(
        contacts_before_items[0]["conversationId"],
        fixture.conversation_id
    );

    let remove_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friendships/{}/remove",
                    fixture.friendship_id
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remove friendship should return response");
    assert_eq!(remove_request.status(), StatusCode::OK);

    let contacts_after = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts request after removal should return response");
    assert_eq!(contacts_after.status(), StatusCode::OK);
    let contacts_after_body = contacts_after
        .into_body()
        .collect()
        .await
        .expect("contacts after body should collect")
        .to_bytes();
    let contacts_after_json: serde_json::Value = serde_json::from_slice(&contacts_after_body)
        .expect("contacts after body should be valid json");
    let contacts_after_items = contacts_after_json["items"]
        .as_array()
        .expect("contacts after items should be an array");
    assert!(
        contacts_after_items.is_empty(),
        "removed friendship must no longer appear in active contacts"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_friend_request_submit_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_submit = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first submit friend request should return response");
    assert_eq!(first_submit.status(), StatusCode::OK);
    let first_submit_body = first_submit
        .into_body()
        .collect()
        .await
        .expect("first submit friend request body should collect")
        .to_bytes();
    let first_submit_json: serde_json::Value = serde_json::from_slice(&first_submit_body)
        .expect("first submit friend request body should be valid json");

    let second_submit = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second submit friend request should return response");
    assert_eq!(second_submit.status(), StatusCode::OK);
    let second_submit_body = second_submit
        .into_body()
        .collect()
        .await
        .expect("second submit friend request body should collect")
        .to_bytes();
    let second_submit_json: serde_json::Value = serde_json::from_slice(&second_submit_body)
        .expect("second submit friend request body should be valid json");

    assert_eq!(first_submit_json["friendRequest"]["status"], "pending");
    assert_eq!(second_submit_json["friendRequest"]["status"], "pending");
    assert_eq!(
        second_submit_json["friendRequest"]["requestId"],
        first_submit_json["friendRequest"]["requestId"]
    );
}

#[tokio::test]
async fn test_local_minimal_profile_recovers_existing_pending_friend_request_for_same_pair() {
    let app = local_minimal_node::build_default_app();

    let control_plane_submit = app
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
                        "requestId":"fr_legacy_same_pair",
                        "eventId":"evt_legacy_same_pair_submit",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestMessage":"legacy hello",
                        "requestedAt":"2026-04-15T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("control-plane friend request seed should return response");
    assert_eq!(control_plane_submit.status(), StatusCode::OK);

    let app_submit = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("IM open-platform friend request submit should return response");
    assert_eq!(app_submit.status(), StatusCode::OK);
    let app_submit_body = app_submit
        .into_body()
        .collect()
        .await
        .expect("IM open-platform friend request submit body should collect")
        .to_bytes();
    let app_submit_json: serde_json::Value = serde_json::from_slice(&app_submit_body)
        .expect("IM open-platform friend request submit body should be valid json");

    assert_eq!(app_submit_json["friendRequest"]["status"], "pending");
    assert_eq!(
        app_submit_json["friendRequest"]["requestId"],
        "fr_legacy_same_pair"
    );
    assert_eq!(
        app_submit_json["friendRequest"]["requestMessage"],
        "legacy hello"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_friend_request_acceptance_creates_direct_chat_contact() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();
    assert_eq!(submit_request_json["friendRequest"]["status"], "pending");
    assert_eq!(
        submit_request_json["friendRequest"]["requesterUserId"],
        "u_alice"
    );
    assert_eq!(
        submit_request_json["friendRequest"]["targetUserId"],
        "u_bob"
    );

    let accept_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("accept friend request should return response");
    assert_eq!(accept_request.status(), StatusCode::OK);
    let accept_request_body = accept_request
        .into_body()
        .collect()
        .await
        .expect("accept friend request body should collect")
        .to_bytes();
    let accept_request_json: serde_json::Value = serde_json::from_slice(&accept_request_body)
        .expect("accept friend request body should be valid json");
    assert_eq!(accept_request_json["friendRequest"]["status"], "accepted");
    assert_eq!(accept_request_json["friendship"]["status"], "active");
    let friendship_id = accept_request_json["friendship"]["friendshipId"]
        .as_str()
        .expect("friendship id should be present")
        .to_owned();
    let conversation_id = accept_request_json["conversation"]["conversationId"]
        .as_str()
        .expect("conversation id should be present")
        .to_owned();
    assert!(
        !friendship_id.is_empty(),
        "friendship id should be generated"
    );

    let contacts = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts request should return response");
    assert_eq!(contacts.status(), StatusCode::OK);
    let contacts_body = contacts
        .into_body()
        .collect()
        .await
        .expect("contacts body should collect")
        .to_bytes();
    let contacts_json: serde_json::Value =
        serde_json::from_slice(&contacts_body).expect("contacts body should be valid json");
    let contact_items = contacts_json["items"]
        .as_array()
        .expect("contacts items should be an array");
    assert_eq!(contact_items.len(), 1);
    assert_eq!(contact_items[0]["targetUserId"], "u_bob");
    assert_eq!(contact_items[0]["conversationId"], conversation_id);
    assert_eq!(contact_items[0]["friendshipId"], friendship_id);

    let members = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members request should return response");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("members body should collect")
        .to_bytes();
    let members_json: serde_json::Value =
        serde_json::from_slice(&members_body).expect("members body should be valid json");
    let member_items = members_json["items"]
        .as_array()
        .expect("member items should be an array");
    assert_eq!(member_items.len(), 2);
    assert!(
        member_items
            .iter()
            .any(|member| member["principalId"] == "u_alice"),
        "alice should be a direct chat member"
    );
    assert!(
        member_items
            .iter()
            .any(|member| member["principalId"] == "u_bob"),
        "bob should be a direct chat member"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_friend_acceptance_direct_chat_supports_two_user_messages() {
    let app = local_minimal_node::build_default_app();
    let fixture = create_active_friendship_direct_chat_fixture(&app).await;

    let alice_post = post_standard_message_for_test(
        &app,
        fixture.conversation_id.as_str(),
        "u_alice",
        "client_direct_chat_alice_1",
        "hello bob from alice",
    )
    .await;
    assert_eq!(alice_post.status(), StatusCode::OK);

    let bob_post = post_standard_message_for_test(
        &app,
        fixture.conversation_id.as_str(),
        "u_bob",
        "client_direct_chat_bob_1",
        "hello alice from bob",
    )
    .await;
    assert_eq!(bob_post.status(), StatusCode::OK);

    let alice_summaries =
        list_message_summaries_for_test(&app, fixture.conversation_id.as_str(), "u_alice").await;
    assert!(
        alice_summaries
            .iter()
            .any(|summary| summary == "hello bob from alice"),
        "alice timeline must include alice's direct chat message"
    );
    assert!(
        alice_summaries
            .iter()
            .any(|summary| summary == "hello alice from bob"),
        "alice timeline must include bob's direct chat message"
    );

    let bob_summaries =
        list_message_summaries_for_test(&app, fixture.conversation_id.as_str(), "u_bob").await;
    assert!(
        bob_summaries
            .iter()
            .any(|summary| summary == "hello bob from alice"),
        "bob timeline must include alice's direct chat message"
    );
    assert!(
        bob_summaries
            .iter()
            .any(|summary| summary == "hello alice from bob"),
        "bob timeline must include bob's direct chat message"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_friend_request_submit_for_blocked_pair() {
    let app = local_minimal_node::build_default_app();

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_app_submit_blocked",
                        "eventId":"evt_ub_app_submit_blocked",
                        "blockerUserId":"u_bob",
                        "blockedUserId":"u_alice",
                        "scope":"friendship",
                        "effectiveAt":"2026-04-10T09:59:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("control-plane user block before app submit should return response");
    assert_eq!(block_response.status(), StatusCode::OK);

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("blocked IM open-platform submit should return response");
    assert_eq!(submit_request.status(), StatusCode::CONFLICT);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("blocked IM open-platform submit body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("blocked IM open-platform submit body should be valid json");
    assert_eq!(submit_request_json["code"], "friend_request_blocked");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_friend_request_accept_for_blocked_pair() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before blocked app accept should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before blocked app accept body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before blocked app accept body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("blocked app accept request id should be present")
        .to_owned();

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_app_accept_blocked",
                        "eventId":"evt_ub_app_accept_blocked",
                        "blockerUserId":"u_bob",
                        "blockedUserId":"u_alice",
                        "scope":"friendship",
                        "effectiveAt":"2026-04-10T10:01:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("control-plane user block before app accept should return response");
    assert_eq!(block_response.status(), StatusCode::OK);

    let accept_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("blocked IM open-platform accept should return response");
    assert_eq!(accept_request.status(), StatusCode::CONFLICT);
    let accept_request_body = accept_request
        .into_body()
        .collect()
        .await
        .expect("blocked IM open-platform accept body should collect")
        .to_bytes();
    let accept_request_json: serde_json::Value = serde_json::from_slice(&accept_request_body)
        .expect("blocked IM open-platform accept body should be valid json");
    assert_eq!(accept_request_json["code"], "friend_request_blocked");

    let request_snapshot = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{request_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after blocked app accept should return response");
    assert_eq!(request_snapshot.status(), StatusCode::OK);
    let request_snapshot_body = request_snapshot
        .into_body()
        .collect()
        .await
        .expect("friend request snapshot after blocked app accept should collect")
        .to_bytes();
    let request_snapshot_json: serde_json::Value = serde_json::from_slice(&request_snapshot_body)
        .expect("friend request snapshot after blocked app accept should be valid json");
    assert_eq!(request_snapshot_json["friendRequest"]["status"], "pending");
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_friend_request_acceptance_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();

    let accept_uri = format!("/im/v3/api/social/friend_requests/{request_id}/accept");
    let first_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&accept_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first accept friend request should return response");
    assert_eq!(first_accept.status(), StatusCode::OK);
    let first_accept_body = first_accept
        .into_body()
        .collect()
        .await
        .expect("first accept friend request body should collect")
        .to_bytes();
    let first_accept_json: serde_json::Value = serde_json::from_slice(&first_accept_body)
        .expect("first accept friend request body should be valid json");

    let second_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&accept_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second accept friend request should return response");
    let second_accept_status = second_accept.status();
    let second_accept_body = second_accept
        .into_body()
        .collect()
        .await
        .expect("second accept friend request body should collect")
        .to_bytes();
    assert_eq!(second_accept_status, StatusCode::OK);
    let second_accept_json: serde_json::Value = serde_json::from_slice(&second_accept_body)
        .expect("second accept friend request body should be valid json");

    assert_eq!(second_accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(
        second_accept_json["friendship"]["friendshipId"],
        first_accept_json["friendship"]["friendshipId"]
    );
    assert_eq!(
        second_accept_json["directChat"]["directChatId"],
        first_accept_json["directChat"]["directChatId"]
    );
    assert_eq!(
        second_accept_json["conversation"]["conversationId"],
        first_accept_json["conversation"]["conversationId"]
    );
}

#[tokio::test]
async fn test_local_minimal_profile_accept_converges_to_existing_external_friendship_and_direct_chat()
 {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"external social orchestration"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before external convergence test should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before external convergence test body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before external convergence test body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();

    let activate_friendship = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friendships")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "friendshipId":"fs_external_accept_001",
                        "eventId": format!("evt_external_activate_{request_id}"),
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "directChatId":"dc_external_accept_001",
                        "establishedAt":"2026-04-16T10:00:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("external friendship activation should return response");
    assert_eq!(activate_friendship.status(), StatusCode::OK);

    let bind_direct_chat = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/direct_chats/bindings")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "directChatId":"dc_external_accept_001",
                        "eventId": format!("evt_external_bind_{request_id}"),
                        "leftActorId":"u_alice",
                        "rightActorId":"u_bob",
                        "conversationId":"c_external_accept_001",
                        "boundAt":"2026-04-16T10:00:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("external direct chat bind should return response");
    assert_eq!(bind_direct_chat.status(), StatusCode::OK);

    let accept_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect(
            "IM open-platform accept should return response when external social graph already exists",
        );
    assert_eq!(accept_request.status(), StatusCode::OK);
    let accept_request_body = accept_request
        .into_body()
        .collect()
        .await
        .expect("IM open-platform accept after external convergence should collect")
        .to_bytes();
    let accept_request_json: serde_json::Value = serde_json::from_slice(&accept_request_body)
        .expect("IM open-platform accept after external convergence should be valid json");
    assert_eq!(accept_request_json["friendRequest"]["status"], "accepted");
    assert_eq!(
        accept_request_json["friendship"]["friendshipId"],
        "fs_external_accept_001"
    );
    assert_eq!(
        accept_request_json["directChat"]["directChatId"],
        "dc_external_accept_001"
    );
    assert_eq!(
        accept_request_json["conversation"]["conversationId"],
        "c_external_accept_001"
    );

    let members = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_external_accept_001/members")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation members after external convergence accept should return response");
    assert_eq!(members.status(), StatusCode::OK);
    let members_body = members
        .into_body()
        .collect()
        .await
        .expect("conversation members after external convergence accept body should collect")
        .to_bytes();
    let members_json: serde_json::Value = serde_json::from_slice(&members_body)
        .expect("conversation members after external convergence accept body should be valid json");
    let member_items = members_json["items"]
        .as_array()
        .expect("conversation members after external convergence should include items");
    assert_eq!(member_items.len(), 2);
}

#[tokio::test]
async fn test_local_minimal_profile_accept_converges_when_request_was_externally_accepted_after_app_snapshot()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _accept_snapshot_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_ACCEPT_PRE_COMMIT_DELAY_MS", "200");

    let runtime_dir = unique_test_runtime_dir("friend_request_accept_external_after_snapshot");
    fs::create_dir_all(runtime_dir.join("state")).expect("runtime dir state should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"external accept after stale app snapshot"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before external accept stale snapshot test should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before external accept stale snapshot test should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect(
            "submit friend request before external accept stale snapshot test should be valid json",
        );
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id for external accept stale snapshot test should be present")
        .to_owned();

    let accept_task = tokio::spawn({
        let app = app.clone();
        let request_id = request_id.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/im/v3/api/social/friend_requests/{request_id}/accept"
                    ))
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("IM open-platform accept after stale snapshot should return response")
        }
    });

    sleep(Duration::from_millis(50)).await;

    let external_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "eventId": format!("evt_external_accept_after_snapshot_{request_id}"),
                        "acceptedByUserId": "u_bob",
                        "acceptedAt": "2026-04-16T10:00:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("external accept after stale app snapshot should return response");
    assert_eq!(external_accept.status(), StatusCode::OK);

    let accept_response = accept_task
        .await
        .expect("accept task after stale app snapshot should join");
    assert_eq!(
        accept_response.status(),
        StatusCode::OK,
        "IM open-platform accept should converge to the externally accepted request instead of surfacing friend_request_not_pending"
    );
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("accept response after stale app snapshot should collect")
        .to_bytes();
    let accept_json: serde_json::Value = serde_json::from_slice(&accept_body)
        .expect("accept response after stale app snapshot should be valid json");
    assert_eq!(accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(accept_json["friendship"]["status"], "active");
    assert_eq!(accept_json["directChat"]["status"], "active");
    assert!(
        accept_json["conversation"]["conversationId"]
            .as_str()
            .is_some_and(|value| !value.trim().is_empty()),
        "IM open-platform accept should still bind a conversation after converging to external acceptance"
    );

    let contacts = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts after stale app snapshot accept convergence should return response");
    assert_eq!(contacts.status(), StatusCode::OK);
    let contacts_body = contacts
        .into_body()
        .collect()
        .await
        .expect("contacts after stale app snapshot accept convergence should collect")
        .to_bytes();
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
        .expect("contacts after stale app snapshot accept convergence should be valid json");
    let contacts_items = contacts_json["items"]
        .as_array()
        .expect("contacts after stale app snapshot accept convergence should include items");
    assert_eq!(contacts_items.len(), 1);
    assert_eq!(contacts_items[0]["targetUserId"], "u_bob");

    let repair_store_path = runtime_dir
        .join("state")
        .join("social-friend-request-accept-repairs.json");
    assert!(
        !repair_store_path.exists(),
        "accept repair entry should be cleared after IM open-platform accept converges to external acceptance"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_decline_converges_when_request_was_externally_declined_after_app_snapshot()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _decline_snapshot_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_DECLINE_PRE_COMMIT_DELAY_MS", "200");

    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"external decline after stale app snapshot"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before external decline stale snapshot test should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before external decline stale snapshot test should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect(
            "submit friend request before external decline stale snapshot test should be valid json",
        );
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id for external decline stale snapshot test should be present")
        .to_owned();

    let decline_task = tokio::spawn({
        let app = app.clone();
        let request_id = request_id.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/im/v3/api/social/friend_requests/{request_id}/decline"
                    ))
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("IM open-platform decline after stale snapshot should return response")
        }
    });

    sleep(Duration::from_millis(50)).await;

    let external_decline = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{request_id}/decline"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "eventId": format!("evt_external_decline_after_snapshot_{request_id}"),
                        "declinedByUserId": "u_bob",
                        "declinedAt": "2026-04-16T10:10:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("external decline after stale app snapshot should return response");
    assert_eq!(external_decline.status(), StatusCode::OK);

    let decline_response = decline_task
        .await
        .expect("decline task after stale app snapshot should join");
    assert_eq!(
        decline_response.status(),
        StatusCode::OK,
        "IM open-platform decline should converge to the externally declined request instead of surfacing friend_request_not_pending"
    );
    let decline_body = decline_response
        .into_body()
        .collect()
        .await
        .expect("decline response after stale app snapshot should collect")
        .to_bytes();
    let decline_json: serde_json::Value = serde_json::from_slice(&decline_body)
        .expect("decline response after stale app snapshot should be valid json");
    assert_eq!(decline_json["friendRequest"]["status"], "declined");
    assert_eq!(decline_json["friendRequest"]["targetUserId"], "u_bob");

    let request_snapshot = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{request_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request snapshot after stale app decline convergence should return response");
    assert_eq!(request_snapshot.status(), StatusCode::OK);
    let request_snapshot_body = request_snapshot
        .into_body()
        .collect()
        .await
        .expect("request snapshot after stale app decline convergence should collect")
        .to_bytes();
    let request_snapshot_json: serde_json::Value = serde_json::from_slice(&request_snapshot_body)
        .expect("request snapshot after stale app decline convergence should be valid json");
    assert_eq!(request_snapshot_json["friendRequest"]["status"], "declined");
}

#[tokio::test]
async fn test_local_minimal_profile_cancel_converges_when_request_was_externally_canceled_after_app_snapshot()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _cancel_snapshot_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_CANCEL_PRE_COMMIT_DELAY_MS", "200");

    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"external cancel after stale app snapshot"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before external cancel stale snapshot test should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before external cancel stale snapshot test should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect(
            "submit friend request before external cancel stale snapshot test should be valid json",
        );
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id for external cancel stale snapshot test should be present")
        .to_owned();

    let cancel_task = tokio::spawn({
        let app = app.clone();
        let request_id = request_id.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/im/v3/api/social/friend_requests/{request_id}/cancel"
                    ))
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_alice")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("IM open-platform cancel after stale snapshot should return response")
        }
    });

    sleep(Duration::from_millis(50)).await;

    let external_cancel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{request_id}/cancel"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "eventId": format!("evt_external_cancel_after_snapshot_{request_id}"),
                        "canceledByUserId": "u_alice",
                        "canceledAt": "2026-04-16T10:20:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("external cancel after stale app snapshot should return response");
    assert_eq!(external_cancel.status(), StatusCode::OK);

    let cancel_response = cancel_task
        .await
        .expect("cancel task after stale app snapshot should join");
    assert_eq!(
        cancel_response.status(),
        StatusCode::OK,
        "IM open-platform cancel should converge to the externally canceled request instead of surfacing friend_request_not_pending"
    );
    let cancel_body = cancel_response
        .into_body()
        .collect()
        .await
        .expect("cancel response after stale app snapshot should collect")
        .to_bytes();
    let cancel_json: serde_json::Value = serde_json::from_slice(&cancel_body)
        .expect("cancel response after stale app snapshot should be valid json");
    assert_eq!(cancel_json["friendRequest"]["status"], "canceled");
    assert_eq!(cancel_json["friendRequest"]["requesterUserId"], "u_alice");

    let request_snapshot = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{request_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request snapshot after stale app cancel convergence should return response");
    assert_eq!(request_snapshot.status(), StatusCode::OK);
    let request_snapshot_body = request_snapshot
        .into_body()
        .collect()
        .await
        .expect("request snapshot after stale app cancel convergence should collect")
        .to_bytes();
    let request_snapshot_json: serde_json::Value = serde_json::from_slice(&request_snapshot_body)
        .expect("request snapshot after stale app cancel convergence should be valid json");
    assert_eq!(request_snapshot_json["friendRequest"]["status"], "canceled");
}

#[tokio::test]
async fn test_local_minimal_profile_cancel_after_accept_commit_is_rejected_without_tearing_social_graph()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _accept_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS", "50");

    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();
    let friendship_id = deterministic_social_id_for_test("fs_", request_id.as_str());

    let accept_uri = format!("/im/v3/api/social/friend_requests/{request_id}/accept");
    let cancel_uri = format!("/im/v3/api/social/friend_requests/{request_id}/cancel");
    let friendship_snapshot_uri =
        format!("/backend/v3/api/control/social/friendships/{friendship_id}");
    let request_snapshot_uri =
        format!("/backend/v3/api/control/social/friend_requests/{request_id}");

    let accept_app = app.clone();
    let accept_task = tokio::spawn(async move {
        accept_app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("accept friend request should return response")
    });

    let mut accepted_visible = false;
    for _ in 0..200 {
        let request_snapshot = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&request_snapshot_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "user")
                    .header("x-sdkwork-permission-scope", "control.read")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("friend request snapshot poll should return response");
        if request_snapshot.status() == StatusCode::OK {
            let request_snapshot_body = request_snapshot
                .into_body()
                .collect()
                .await
                .expect("friend request snapshot poll body should collect")
                .to_bytes();
            let request_snapshot_json: serde_json::Value =
                serde_json::from_slice(&request_snapshot_body)
                    .expect("friend request snapshot poll body should be valid json");
            if request_snapshot_json["friendRequest"]["status"] == "accepted" {
                accepted_visible = true;
                break;
            }
        }
        sleep(Duration::from_millis(1)).await;
    }
    assert!(
        accepted_visible,
        "friend request should become accepted before social side effects are created"
    );

    let friendship_before_accept_completion = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot before accept completion should return response");
    assert_eq!(
        friendship_before_accept_completion.status(),
        StatusCode::OK,
        "atomic accept should materialize friendship before the delayed local side effects finish"
    );

    let cancel_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(cancel_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cancel friend request should return response");
    assert_eq!(
        cancel_response.status(),
        StatusCode::CONFLICT,
        "cancel must be rejected once accept has already committed the request"
    );
    let cancel_response_body = cancel_response
        .into_body()
        .collect()
        .await
        .expect("cancel response body should collect")
        .to_bytes();
    let cancel_response_json: serde_json::Value = serde_json::from_slice(&cancel_response_body)
        .expect("cancel response body should be valid json");
    assert_eq!(cancel_response_json["code"], "friend_request_not_pending");

    let accept_response = accept_task
        .await
        .expect("accept task should join successfully");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let request_snapshot = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(request_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot should return response");
    assert_eq!(request_snapshot.status(), StatusCode::OK);
    let request_snapshot_body = request_snapshot
        .into_body()
        .collect()
        .await
        .expect("friend request snapshot body should collect")
        .to_bytes();
    let request_snapshot_json: serde_json::Value = serde_json::from_slice(&request_snapshot_body)
        .expect("friend request snapshot body should be valid json");
    assert_eq!(request_snapshot_json["friendRequest"]["status"], "accepted");

    let friendship_after_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friendships/{friendship_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot after accept should return response");
    assert_eq!(
        friendship_after_accept.status(),
        StatusCode::OK,
        "accepted request must converge to an active friendship"
    );

    let alice_contacts = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("alice contacts request should return response");
    assert_eq!(alice_contacts.status(), StatusCode::OK);
    let alice_contacts_body = alice_contacts
        .into_body()
        .collect()
        .await
        .expect("alice contacts body should collect")
        .to_bytes();
    let alice_contacts_json: serde_json::Value = serde_json::from_slice(&alice_contacts_body)
        .expect("alice contacts body should be valid json");
    let alice_contact_items = alice_contacts_json["items"]
        .as_array()
        .expect("alice contacts items should be an array");
    assert_eq!(alice_contact_items.len(), 1);
    assert_eq!(alice_contact_items[0]["targetUserId"], "u_bob");

    let bob_contacts = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("bob contacts request should return response");
    assert_eq!(bob_contacts.status(), StatusCode::OK);
    let bob_contacts_body = bob_contacts
        .into_body()
        .collect()
        .await
        .expect("bob contacts body should collect")
        .to_bytes();
    let bob_contacts_json: serde_json::Value =
        serde_json::from_slice(&bob_contacts_body).expect("bob contacts body should be valid json");
    let bob_contact_items = bob_contacts_json["items"]
        .as_array()
        .expect("bob contacts items should be an array");
    assert_eq!(bob_contact_items.len(), 1);
    assert_eq!(bob_contact_items[0]["targetUserId"], "u_alice");
}

#[tokio::test]
async fn test_local_minimal_profile_repairs_pending_friend_request_acceptance_after_restart() {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _accept_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS", "500");

    let runtime_dir = unique_test_runtime_dir("friend_request_accept_repair");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"repair after restart"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before restart repair should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before restart repair body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before restart repair body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();
    let friendship_id = deterministic_social_id_for_test("fs_", request_id.as_str());

    let accept_uri = format!("/im/v3/api/social/friend_requests/{request_id}/accept");
    let request_snapshot_uri =
        format!("/backend/v3/api/control/social/friend_requests/{request_id}");
    let friendship_snapshot_uri =
        format!("/backend/v3/api/control/social/friendships/{friendship_id}");

    let accept_app = app.clone();
    let accept_task = tokio::spawn(async move {
        accept_app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("accept friend request before restart repair should return response")
    });

    let mut accepted_visible = false;
    for _ in 0..200 {
        let request_snapshot = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&request_snapshot_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "user")
                    .header("x-sdkwork-permission-scope", "control.read")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request snapshot before restart repair should return response");
        if request_snapshot.status() == StatusCode::OK {
            let request_snapshot_body = request_snapshot
                .into_body()
                .collect()
                .await
                .expect("request snapshot before restart repair body should collect")
                .to_bytes();
            let request_snapshot_json: serde_json::Value =
                serde_json::from_slice(&request_snapshot_body)
                    .expect("request snapshot before restart repair body should be valid json");
            if request_snapshot_json["friendRequest"]["status"] == "accepted" {
                accepted_visible = true;
                break;
            }
        }
        sleep(Duration::from_millis(5)).await;
    }
    assert!(
        accepted_visible,
        "friend request should become accepted before aborting accept flow"
    );

    let friendship_snapshot = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot before abort should return response");
    assert_eq!(
        friendship_snapshot.status(),
        StatusCode::OK,
        "control-plane friendship should already exist before aborting the delayed local accept flow"
    );

    accept_task.abort();
    let _ = accept_task.await;
    drop(app);

    let app_after_restart =
        local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let mut repaired = false;
    for _ in 0..50 {
        let contacts = app_after_restart
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/im/v3/api/chat/contacts")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_alice")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("contacts after restart repair should return response");
        assert_eq!(contacts.status(), StatusCode::OK);
        let contacts_body = contacts
            .into_body()
            .collect()
            .await
            .expect("contacts after restart repair body should collect")
            .to_bytes();
        let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
            .expect("contacts after restart repair body should be valid json");
        let contact_items = contacts_json["items"]
            .as_array()
            .expect("contacts after restart repair should include items");
        if contact_items.len() == 1 && contact_items[0]["targetUserId"] == "u_bob" {
            repaired = true;
            break;
        }
        sleep(Duration::from_millis(20)).await;
    }
    assert!(
        repaired,
        "restart should automatically repair accepted friend request side effects"
    );

    let friendship_after_restart = app_after_restart
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot after restart repair should return response");
    assert_eq!(friendship_after_restart.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_contacts_read_repairs_pending_friend_request_acceptance_immediately_after_restart()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _accept_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS", "500");

    let runtime_dir = unique_test_runtime_dir("friend_request_accept_repair_contacts");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"repair on first contacts read"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before contacts repair should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before contacts repair body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before contacts repair body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();

    let request_snapshot_uri =
        format!("/backend/v3/api/control/social/friend_requests/{request_id}");
    let accept_uri = format!("/im/v3/api/social/friend_requests/{request_id}/accept");

    let accept_app = app.clone();
    let accept_task = tokio::spawn(async move {
        accept_app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("accept friend request before contacts repair should return response")
    });

    let mut accepted_visible = false;
    for _ in 0..200 {
        let request_snapshot = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&request_snapshot_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "user")
                    .header("x-sdkwork-permission-scope", "control.read")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request snapshot before contacts repair should return response");
        if request_snapshot.status() == StatusCode::OK {
            let request_snapshot_body = request_snapshot
                .into_body()
                .collect()
                .await
                .expect("request snapshot before contacts repair body should collect")
                .to_bytes();
            let request_snapshot_json: serde_json::Value =
                serde_json::from_slice(&request_snapshot_body)
                    .expect("request snapshot before contacts repair body should be valid json");
            if request_snapshot_json["friendRequest"]["status"] == "accepted" {
                accepted_visible = true;
                break;
            }
        }
        sleep(Duration::from_millis(5)).await;
    }
    assert!(
        accepted_visible,
        "friend request should become accepted before aborting accept flow"
    );

    accept_task.abort();
    let _ = accept_task.await;
    drop(app);

    let app_after_restart =
        local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let contacts = app_after_restart
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first contacts read after restart should return response");
    assert_eq!(contacts.status(), StatusCode::OK);
    let contacts_body = contacts
        .into_body()
        .collect()
        .await
        .expect("first contacts read after restart body should collect")
        .to_bytes();
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
        .expect("first contacts read after restart body should be valid json");
    let contact_items = contacts_json["items"]
        .as_array()
        .expect("first contacts read after restart should include items");
    assert_eq!(
        contact_items.len(),
        1,
        "first contacts read after restart should synchronously converge pending accepted friendship"
    );
    assert_eq!(contact_items[0]["targetUserId"], "u_bob");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_second_instance_contacts_read_repairs_pending_friend_request_acceptance()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _accept_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS", "500");

    let runtime_dir = unique_test_runtime_dir("friend_request_accept_repair_second_instance");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let app_b = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app_a
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
                        "targetUserId":"u_bob",
                        "requestMessage":"repair from second instance"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before second instance repair should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before second instance repair body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before second instance repair body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();
    let friendship_id = deterministic_social_id_for_test("fs_", request_id.as_str());
    let repair_store_path = runtime_dir
        .join("state")
        .join("social-friend-request-accept-repairs.json");

    let accept_uri = format!("/im/v3/api/social/friend_requests/{request_id}/accept");
    let request_snapshot_uri =
        format!("/backend/v3/api/control/social/friend_requests/{request_id}");
    let friendship_snapshot_uri =
        format!("/backend/v3/api/control/social/friendships/{friendship_id}");

    let accept_app = app_a.clone();
    let accept_task = tokio::spawn(async move {
        accept_app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("accept friend request before second instance repair should return response")
    });

    let mut accepted_visible = false;
    for _ in 0..200 {
        let request_snapshot = app_b
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&request_snapshot_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "user")
                    .header("x-sdkwork-permission-scope", "control.read")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request snapshot before second instance repair should return response");
        if request_snapshot.status() == StatusCode::OK {
            let request_snapshot_body = request_snapshot
                .into_body()
                .collect()
                .await
                .expect("request snapshot before second instance repair body should collect")
                .to_bytes();
            let request_snapshot_json: serde_json::Value = serde_json::from_slice(
                &request_snapshot_body,
            )
            .expect("request snapshot before second instance repair body should be valid json");
            if request_snapshot_json["friendRequest"]["status"] == "accepted" {
                accepted_visible = true;
                break;
            }
        }
        sleep(Duration::from_millis(5)).await;
    }
    assert!(
        accepted_visible,
        "friend request should become accepted before aborting first instance accept flow"
    );
    assert!(
        repair_store_path.exists(),
        "pending accept repair store should be materialized for the second instance to observe"
    );

    let friendship_snapshot = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot before second instance repair should return response");
    assert_eq!(
        friendship_snapshot.status(),
        StatusCode::OK,
        "control-plane friendship should already exist before the second instance repairs local contacts"
    );

    accept_task.abort();
    let _ = accept_task.await;
    drop(app_a);

    let contacts = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second instance contacts read should return response");
    assert_eq!(contacts.status(), StatusCode::OK);
    let contacts_body = contacts
        .into_body()
        .collect()
        .await
        .expect("second instance contacts read body should collect")
        .to_bytes();
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
        .expect("second instance contacts read body should be valid json");
    let contact_items = contacts_json["items"]
        .as_array()
        .expect("second instance contacts read should include items");
    assert_eq!(
        contact_items.len(),
        1,
        "second instance should synchronously observe and repair the shared pending accept entry"
    );
    assert_eq!(contact_items[0]["targetUserId"], "u_bob");

    let friendship_after_repair = app_b
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot after second instance repair should return response");
    assert_eq!(friendship_after_repair.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_same_instance_concurrent_contacts_wait_for_pending_accept_repair()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _accept_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS", "500");

    let runtime_dir =
        unique_test_runtime_dir("friend_request_accept_repair_same_instance_contacts_wait");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let app_b = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app_a
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
                        "targetUserId":"u_bob",
                        "requestMessage":"same instance contacts wait"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before same-instance contacts wait should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before same-instance contacts wait body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect(
            "submit friend request before same-instance contacts wait body should be valid json",
        );
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();
    let friendship_id = deterministic_social_id_for_test("fs_", request_id.as_str());

    let accept_uri = format!("/im/v3/api/social/friend_requests/{request_id}/accept");
    let request_snapshot_uri =
        format!("/backend/v3/api/control/social/friend_requests/{request_id}");
    let friendship_snapshot_uri =
        format!("/backend/v3/api/control/social/friendships/{friendship_id}");

    let accept_app = app_a.clone();
    let accept_task = tokio::spawn(async move {
        accept_app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect(
                "accept friend request before same-instance contacts wait should return response",
            )
    });

    let mut accepted_visible = false;
    for _ in 0..200 {
        let request_snapshot = app_b
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&request_snapshot_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "user")
                    .header("x-sdkwork-permission-scope", "control.read")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request snapshot before same-instance contacts wait should return response");
        if request_snapshot.status() == StatusCode::OK {
            let request_snapshot_body = request_snapshot
                .into_body()
                .collect()
                .await
                .expect("request snapshot before same-instance contacts wait body should collect")
                .to_bytes();
            let request_snapshot_json: serde_json::Value =
                serde_json::from_slice(&request_snapshot_body).expect(
                    "request snapshot before same-instance contacts wait body should be valid json",
                );
            if request_snapshot_json["friendRequest"]["status"] == "accepted" {
                accepted_visible = true;
                break;
            }
        }
        sleep(Duration::from_millis(5)).await;
    }
    assert!(
        accepted_visible,
        "friend request should become accepted before aborting accept flow"
    );

    let friendship_snapshot = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot before same-instance contacts wait should return response");
    assert_eq!(
        friendship_snapshot.status(),
        StatusCode::OK,
        "control-plane friendship should already exist before same-instance repair begins"
    );

    accept_task.abort();
    let _ = accept_task.await;
    drop(app_a);

    let run_lock_path = runtime_dir
        .join("state")
        .join("social-friend-request-accept-repairs.run.lock");
    fs::create_dir_all(
        run_lock_path
            .parent()
            .expect("repair run lock path should have parent dir"),
    )
    .expect("repair run lock parent dir should be created");
    let run_lock_file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(run_lock_path.as_path())
        .expect("repair run lock file should open");
    run_lock_file
        .lock_exclusive()
        .expect("test should acquire repair run lock");

    let first_contacts_app = app_b.clone();
    let mut first_contacts_task = tokio::spawn(async move {
        first_contacts_app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/im/v3/api/chat/contacts")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_alice")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("first contacts read while holding repair run lock should return response")
    });
    assert!(
        timeout(Duration::from_millis(50), &mut first_contacts_task)
            .await
            .is_err(),
        "first contacts read should block while the repair run lock is held"
    );

    let second_contacts_app = app_b.clone();
    let mut second_contacts_task = tokio::spawn(async move {
        second_contacts_app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/im/v3/api/chat/contacts")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_alice")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("second contacts read while repair is in-flight should return response")
    });
    assert!(
        timeout(Duration::from_millis(50), &mut second_contacts_task)
            .await
            .is_err(),
        "second concurrent contacts read on the same instance should wait for the in-flight repair"
    );

    run_lock_file
        .unlock()
        .expect("test should release repair run lock");
    drop(run_lock_file);

    let first_contacts = first_contacts_task
        .await
        .expect("first contacts read task should complete after run lock release");
    assert_eq!(first_contacts.status(), StatusCode::OK);
    let first_contacts_body = first_contacts
        .into_body()
        .collect()
        .await
        .expect("first contacts read body should collect")
        .to_bytes();
    let first_contacts_json: serde_json::Value = serde_json::from_slice(&first_contacts_body)
        .expect("first contacts read body should be valid json");
    let first_items = first_contacts_json["items"]
        .as_array()
        .expect("first contacts read should include items");
    assert_eq!(
        first_items.len(),
        1,
        "first contacts read should converge the pending accept repair"
    );
    assert_eq!(first_items[0]["targetUserId"], "u_bob");

    let second_contacts = second_contacts_task
        .await
        .expect("second contacts read task should complete after run lock release");
    assert_eq!(second_contacts.status(), StatusCode::OK);
    let second_contacts_body = second_contacts
        .into_body()
        .collect()
        .await
        .expect("second contacts read body should collect")
        .to_bytes();
    let second_contacts_json: serde_json::Value = serde_json::from_slice(&second_contacts_body)
        .expect("second contacts read body should be valid json");
    let second_items = second_contacts_json["items"]
        .as_array()
        .expect("second contacts read should include items");
    assert_eq!(
        second_items.len(),
        1,
        "second contacts read should observe the converged friendship after waiting for repair"
    );
    assert_eq!(second_items[0]["targetUserId"], "u_bob");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_healthz_stays_responsive_while_repair_store_io_is_delayed() {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _accept_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS", "500");
    let _repair_store_delay = set_scoped_env_var(
        "CRAW_CHAT_TEST_SOCIAL_ACCEPT_REPAIR_STORE_IO_DELAY_MS",
        "250",
    );

    let runtime_dir = unique_test_runtime_dir("friend_request_accept_repair_store_io_delay");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let app_b = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app_a
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
                        "targetUserId":"u_bob",
                        "requestMessage":"repair store io delay"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before repair store io delay should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before repair store io delay body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before repair store io delay body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();
    let friendship_id = deterministic_social_id_for_test("fs_", request_id.as_str());

    let accept_uri = format!("/im/v3/api/social/friend_requests/{request_id}/accept");
    let request_snapshot_uri =
        format!("/backend/v3/api/control/social/friend_requests/{request_id}");
    let friendship_snapshot_uri =
        format!("/backend/v3/api/control/social/friendships/{friendship_id}");

    let accept_app = app_a.clone();
    let accept_task = tokio::spawn(async move {
        accept_app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("accept friend request before repair store io delay should return response")
    });

    let mut accepted_visible = false;
    for _ in 0..200 {
        let request_snapshot = app_b
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&request_snapshot_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                    .header("x-sdkwork-actor-kind", "user")
                    .header("x-sdkwork-permission-scope", "control.read")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("request snapshot before repair store io delay should return response");
        if request_snapshot.status() == StatusCode::OK {
            let request_snapshot_body = request_snapshot
                .into_body()
                .collect()
                .await
                .expect("request snapshot before repair store io delay body should collect")
                .to_bytes();
            let request_snapshot_json: serde_json::Value = serde_json::from_slice(
                &request_snapshot_body,
            )
            .expect("request snapshot before repair store io delay body should be valid json");
            if request_snapshot_json["friendRequest"]["status"] == "accepted" {
                accepted_visible = true;
                break;
            }
        }
        sleep(Duration::from_millis(5)).await;
    }
    assert!(
        accepted_visible,
        "friend request should become accepted before aborting accept flow"
    );

    let friendship_snapshot = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot before repair store io delay should return response");
    assert_eq!(
        friendship_snapshot.status(),
        StatusCode::OK,
        "control-plane friendship should already exist before local repair store convergence starts"
    );

    accept_task.abort();
    let _ = accept_task.await;
    drop(app_a);

    let contacts_app = app_b.clone();
    let mut contacts_task = tokio::spawn(async move {
        contacts_app
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/im/v3/api/chat/contacts")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_alice")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("contacts read during repair store io delay should return response")
    });
    assert!(
        timeout(Duration::from_millis(50), &mut contacts_task)
            .await
            .is_err(),
        "contacts repair should still be waiting while repair store io delay is active"
    );

    let healthz = timeout(
        Duration::from_millis(50),
        app_b.clone().oneshot(
            Request::builder()
                .method("GET")
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        ),
    )
    .await
    .expect("healthz should stay responsive while repair store io is delayed")
    .expect("healthz should return response while repair store io is delayed");
    assert_eq!(healthz.status(), StatusCode::OK);

    let contacts = contacts_task
        .await
        .expect("contacts task should finish after delayed repair store io completes");
    assert_eq!(contacts.status(), StatusCode::OK);
    let contacts_body = contacts
        .into_body()
        .collect()
        .await
        .expect("contacts body after delayed repair should collect")
        .to_bytes();
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
        .expect("contacts body after delayed repair should be valid json");
    let items = contacts_json["items"]
        .as_array()
        .expect("contacts body after delayed repair should include items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["targetUserId"], "u_bob");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_cross_instance_pending_accept_repairs_preserve_multiple_entries()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _accept_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS", "500");

    let runtime_dir = unique_test_runtime_dir("friend_request_accept_repair_multi_instance");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let app_b = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_first = app_a
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
                        "targetUserId":"u_bob",
                        "requestMessage":"first pending repair"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first submit before multi-instance repair preservation should return response");
    assert_eq!(submit_first.status(), StatusCode::OK);
    let submit_first_body = submit_first
        .into_body()
        .collect()
        .await
        .expect("first submit before multi-instance repair preservation body should collect")
        .to_bytes();
    let submit_first_json: serde_json::Value = serde_json::from_slice(&submit_first_body)
        .expect("first submit before multi-instance repair preservation body should be valid json");
    let first_request_id = submit_first_json["friendRequest"]["requestId"]
        .as_str()
        .expect("first request id should be present")
        .to_owned();
    let first_friendship_id = deterministic_social_id_for_test("fs_", first_request_id.as_str());
    let first_request_snapshot_uri =
        format!("/backend/v3/api/control/social/friend_requests/{first_request_id}");
    let first_friendship_snapshot_uri =
        format!("/backend/v3/api/control/social/friendships/{first_friendship_id}");

    let first_accept_uri = format!("/im/v3/api/social/friend_requests/{first_request_id}/accept");
    let first_accept_app = app_a.clone();
    let first_accept_task = tokio::spawn(async move {
        first_accept_app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(first_accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("first accept before multi-instance repair preservation should return response")
    });

    let mut first_accepted_visible = false;
    for _ in 0..200 {
        let request_snapshot = app_a
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&first_request_snapshot_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                    .header("x-sdkwork-permission-scope", "control.read")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("first request snapshot before multi-instance repair preservation should return response");
        if request_snapshot.status() == StatusCode::OK {
            let request_snapshot_body = request_snapshot
                .into_body()
                .collect()
                .await
                .expect("first request snapshot before multi-instance repair preservation body should collect")
                .to_bytes();
            let request_snapshot_json: serde_json::Value =
                serde_json::from_slice(&request_snapshot_body)
                    .expect("first request snapshot before multi-instance repair preservation body should be valid json");
            if request_snapshot_json["friendRequest"]["status"] == "accepted" {
                first_accepted_visible = true;
                break;
            }
        }
        sleep(Duration::from_millis(5)).await;
    }
    assert!(
        first_accepted_visible,
        "first friend request should become accepted before aborting accept flow"
    );
    let first_friendship_snapshot = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&first_friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first friendship snapshot before multi-instance repair preservation should return response");
    assert_eq!(first_friendship_snapshot.status(), StatusCode::OK);
    first_accept_task.abort();
    let _ = first_accept_task.await;

    let submit_second = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_carol")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "targetUserId":"u_bob",
                        "requestMessage":"second pending repair"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second submit before multi-instance repair preservation should return response");
    assert_eq!(submit_second.status(), StatusCode::OK);
    let submit_second_body = submit_second
        .into_body()
        .collect()
        .await
        .expect("second submit before multi-instance repair preservation body should collect")
        .to_bytes();
    let submit_second_json: serde_json::Value = serde_json::from_slice(&submit_second_body).expect(
        "second submit before multi-instance repair preservation body should be valid json",
    );
    let second_request_id = submit_second_json["friendRequest"]["requestId"]
        .as_str()
        .expect("second request id should be present")
        .to_owned();
    let second_friendship_id = deterministic_social_id_for_test("fs_", second_request_id.as_str());
    let second_request_snapshot_uri =
        format!("/backend/v3/api/control/social/friend_requests/{second_request_id}");
    let second_friendship_snapshot_uri =
        format!("/backend/v3/api/control/social/friendships/{second_friendship_id}");

    let second_accept_uri = format!("/im/v3/api/social/friend_requests/{second_request_id}/accept");
    let second_accept_app = app_b.clone();
    let second_accept_task = tokio::spawn(async move {
        second_accept_app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(second_accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect(
                "second accept before multi-instance repair preservation should return response",
            )
    });

    let mut second_accepted_visible = false;
    for _ in 0..200 {
        let request_snapshot = app_b
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri(&second_request_snapshot_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                    .header("x-sdkwork-permission-scope", "control.read")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("second request snapshot before multi-instance repair preservation should return response");
        if request_snapshot.status() == StatusCode::OK {
            let request_snapshot_body = request_snapshot
                .into_body()
                .collect()
                .await
                .expect("second request snapshot before multi-instance repair preservation body should collect")
                .to_bytes();
            let request_snapshot_json: serde_json::Value =
                serde_json::from_slice(&request_snapshot_body)
                    .expect("second request snapshot before multi-instance repair preservation body should be valid json");
            if request_snapshot_json["friendRequest"]["status"] == "accepted" {
                second_accepted_visible = true;
                break;
            }
        }
        sleep(Duration::from_millis(5)).await;
    }
    assert!(
        second_accepted_visible,
        "second friend request should become accepted before aborting accept flow"
    );
    let second_friendship_snapshot = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&second_friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second friendship snapshot before multi-instance repair preservation should return response");
    assert_eq!(second_friendship_snapshot.status(), StatusCode::OK);
    second_accept_task.abort();
    let _ = second_accept_task.await;

    drop(app_a);
    drop(app_b);

    let app_after_restart =
        local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let alice_contacts = app_after_restart
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("alice contacts after multi-instance repair preservation should return response");
    assert_eq!(alice_contacts.status(), StatusCode::OK);
    let alice_contacts_body = alice_contacts
        .into_body()
        .collect()
        .await
        .expect("alice contacts after multi-instance repair preservation body should collect")
        .to_bytes();
    let alice_contacts_json: serde_json::Value = serde_json::from_slice(&alice_contacts_body)
        .expect(
            "alice contacts after multi-instance repair preservation body should be valid json",
        );
    let alice_items = alice_contacts_json["items"]
        .as_array()
        .expect("alice contacts after multi-instance repair preservation should include items");
    assert_eq!(
        alice_items.len(),
        1,
        "first pending repair entry must survive the second instance write"
    );
    assert_eq!(alice_items[0]["targetUserId"], "u_bob");

    let carol_contacts = app_after_restart
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_carol")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("carol contacts after multi-instance repair preservation should return response");
    assert_eq!(carol_contacts.status(), StatusCode::OK);
    let carol_contacts_body = carol_contacts
        .into_body()
        .collect()
        .await
        .expect("carol contacts after multi-instance repair preservation body should collect")
        .to_bytes();
    let carol_contacts_json: serde_json::Value = serde_json::from_slice(&carol_contacts_body)
        .expect(
            "carol contacts after multi-instance repair preservation body should be valid json",
        );
    let carol_items = carol_contacts_json["items"]
        .as_array()
        .expect("carol contacts after multi-instance repair preservation should include items");
    assert_eq!(
        carol_items.len(),
        1,
        "second pending repair entry must also converge after restart"
    );
    assert_eq!(carol_items[0]["targetUserId"], "u_bob");

    let first_friendship_after_restart = app_after_restart
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&first_friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first friendship snapshot after multi-instance repair preservation should return response");
    assert_eq!(first_friendship_after_restart.status(), StatusCode::OK);

    let second_friendship_after_restart = app_after_restart
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(&second_friendship_snapshot_uri)
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second friendship snapshot after multi-instance repair preservation should return response");
    assert_eq!(second_friendship_after_restart.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_discards_stale_pending_friend_request_accept_repair_when_friendship_was_removed()
 {
    let runtime_dir = unique_test_runtime_dir("friend_request_accept_repair_removed");
    fs::create_dir_all(runtime_dir.join("state")).expect("runtime dir state should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"repair should discard stale removed friendship"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before stale repair scenario should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before stale repair scenario body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before stale repair scenario body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();
    let friendship_id = deterministic_social_id_for_test("fs_", request_id.as_str());

    let control_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "eventId": format!("evt_control_accept_{request_id}"),
                        "acceptedByUserId": "u_bob",
                        "acceptedAt": "2026-04-16T09:00:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("control accept for stale repair scenario should return response");
    assert_eq!(control_accept.status(), StatusCode::OK);

    let friendship_snapshot_before_remove = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friendships/{friendship_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot before stale repair removal should return response");
    assert_eq!(friendship_snapshot_before_remove.status(), StatusCode::OK);

    let remove_friendship = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/backend/v3/api/control/social/friendships/{friendship_id}/remove"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "eventId": format!("evt_remove_{request_id}"),
                        "removedByUserId": "u_alice",
                        "removedAt": "2026-04-16T09:05:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("friendship removal for stale repair scenario should return response");
    assert_eq!(remove_friendship.status(), StatusCode::OK);

    drop(app);

    let repair_store_path = runtime_dir
        .join("state")
        .join("social-friend-request-accept-repairs.json");
    fs::write(
        repair_store_path.as_path(),
        serde_json::json!({
            request_id.as_str(): {
                "tenant_id": "t_demo",
                "request_id": request_id,
                "requester_user_id": "u_alice",
                "target_user_id": "u_bob",
                "accepted_at": "2026-04-16T09:00:00Z"
            }
        })
        .to_string(),
    )
    .expect("stale repair store should be writable");

    let app_after_restart =
        local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let contacts = app_after_restart
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts after stale repair restart should return response");
    assert_eq!(contacts.status(), StatusCode::OK);
    let contacts_body = contacts
        .into_body()
        .collect()
        .await
        .expect("contacts after stale repair restart body should collect")
        .to_bytes();
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
        .expect("contacts after stale repair restart body should be valid json");
    assert!(
        contacts_json["items"]
            .as_array()
            .expect("contacts after stale repair restart should include items")
            .is_empty(),
        "removed friendship should not be resurrected by stale pending repair"
    );
    assert!(
        !repair_store_path.exists(),
        "stale pending repair should be discarded after convergence"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_discards_blocked_pending_friend_request_accept_repair() {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _accept_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS", "400");

    let runtime_dir = unique_test_runtime_dir("friend_request_accept_repair_blocked");
    fs::create_dir_all(runtime_dir.join("state")).expect("runtime dir state should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"repair should discard blocked acceptance"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before blocked repair scenario should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before blocked repair scenario body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before blocked repair scenario body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("blocked repair request id should be present")
        .to_owned();

    let accept_task = tokio::spawn({
        let app = app.clone();
        let request_id = request_id.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/im/v3/api/social/friend_requests/{request_id}/accept"
                    ))
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("blocked accept should return response")
        }
    });

    sleep(Duration::from_millis(50)).await;

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "blockId": format!("ub_blocked_repair_{request_id}"),
                        "eventId": format!("evt_blocked_repair_{request_id}"),
                        "blockerUserId": "u_bob",
                        "blockedUserId": "u_alice",
                        "scope": "friendship",
                        "effectiveAt": "2026-04-16T09:11:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("friendship block for blocked repair scenario should return response");
    assert_eq!(block_response.status(), StatusCode::OK);

    let accept_response = accept_task
        .await
        .expect("blocked accept task should join successfully");
    let accept_status = accept_response.status();
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("blocked accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value =
        serde_json::from_slice(&accept_body).expect("blocked accept body should be json");
    assert_eq!(
        accept_status,
        StatusCode::OK,
        "late friendship block should not roll back an already committed atomic accept: {accept_json}"
    );
    assert_eq!(accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(accept_json["friendship"]["status"], "active");

    let contacts = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts should return response for blocked repair scenario");
    assert_eq!(
        contacts.status(),
        StatusCode::OK,
        "blocked pending repair should be discarded instead of permanently failing contacts reads"
    );
    let contacts_body = contacts
        .into_body()
        .collect()
        .await
        .expect("contacts after blocked repair should collect")
        .to_bytes();
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
        .expect("contacts after blocked repair should be json");
    let contact_items = contacts_json["items"]
        .as_array()
        .expect("contacts after blocked repair should include items");
    assert!(
        contact_items.is_empty()
            || (contact_items.len() == 1 && contact_items[0]["targetUserId"] == "u_bob"),
        "late block should not poison contact convergence regardless of whether friendship-scope blocks hide contacts: {contacts_json}"
    );

    let repair_store_path = runtime_dir
        .join("state")
        .join("social-friend-request-accept-repairs.json");
    assert!(
        !repair_store_path.exists(),
        "blocked stale repair should be discarded after convergence"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_discards_canceled_pending_friend_request_accept_repair_after_stale_pending_snapshot()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _repair_snapshot_delay = set_scoped_env_var(
        "CRAW_CHAT_TEST_SOCIAL_ACCEPT_REPAIR_POST_SNAPSHOT_DELAY_MS",
        "200",
    );

    let runtime_dir = unique_test_runtime_dir("friend_request_accept_repair_canceled");
    fs::create_dir_all(runtime_dir.join("state")).expect("runtime dir state should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"repair should discard canceled acceptance after stale pending snapshot"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before canceled stale repair scenario should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before canceled stale repair scenario body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect(
            "submit friend request before canceled stale repair scenario body should be valid json",
        );
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("canceled stale repair request id should be present")
        .to_owned();

    let repair_store_path = runtime_dir
        .join("state")
        .join("social-friend-request-accept-repairs.json");
    fs::write(
        repair_store_path.as_path(),
        serde_json::json!({
            request_id.as_str(): {
                "tenant_id": "t_demo",
                "request_id": request_id,
                "requester_user_id": "u_alice",
                "target_user_id": "u_bob",
                "accepted_at": "2026-04-16T09:30:00Z"
            }
        })
        .to_string(),
    )
    .expect("pending repair store for canceled stale repair scenario should be writable");

    let contacts_task = tokio::spawn({
        let app = app.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/im/v3/api/chat/contacts")
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_alice")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("contacts during canceled stale repair scenario should return response")
        }
    });

    sleep(Duration::from_millis(50)).await;

    let cancel_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{request_id}/cancel"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "eventId": format!("evt_cancel_stale_repair_{request_id}"),
                        "canceledByUserId": "u_alice",
                        "canceledAt": "2026-04-16T09:31:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("control cancel during canceled stale repair scenario should return response");
    assert_eq!(cancel_response.status(), StatusCode::OK);

    let contacts_response = contacts_task
        .await
        .expect("contacts task for canceled stale repair scenario should join");
    assert_eq!(
        contacts_response.status(),
        StatusCode::OK,
        "stale pending repair canceled after snapshot should be discarded instead of breaking contacts reads"
    );
    let contacts_body = contacts_response
        .into_body()
        .collect()
        .await
        .expect("contacts after canceled stale repair scenario should collect")
        .to_bytes();
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
        .expect("contacts after canceled stale repair scenario should be valid json");
    assert!(
        contacts_json["items"]
            .as_array()
            .expect("contacts after canceled stale repair scenario should include items")
            .is_empty(),
        "canceled friendship request must not materialize a contact after stale repair convergence"
    );
    assert!(
        !repair_store_path.exists(),
        "stale pending repair should be cleared after the request is canceled during repair"
    );

    let request_snapshot = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/backend/v3/api/control/social/friend_requests/{request_id}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request snapshot after canceled stale repair scenario should return response");
    assert_eq!(request_snapshot.status(), StatusCode::OK);
    let request_snapshot_body = request_snapshot
        .into_body()
        .collect()
        .await
        .expect("request snapshot after canceled stale repair scenario should collect")
        .to_bytes();
    let request_snapshot_json: serde_json::Value = serde_json::from_slice(&request_snapshot_body)
        .expect("request snapshot after canceled stale repair scenario should be valid json");
    assert_eq!(request_snapshot_json["friendRequest"]["status"], "canceled");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_discards_pre_accept_blocked_pending_friend_request_accept_repair()
 {
    let runtime_dir = unique_test_runtime_dir("friend_request_accept_repair_pre_accept_blocked");
    fs::create_dir_all(runtime_dir.join("state")).expect("runtime dir state should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"repair should discard blocked pending acceptance before accept commit"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before pre-accept blocked repair scenario should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect(
            "submit friend request before pre-accept blocked repair scenario body should collect",
        )
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before pre-accept blocked repair scenario body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("pre-accept blocked repair request id should be present")
        .to_owned();

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "blockId": format!("ub_pre_accept_blocked_repair_{request_id}"),
                        "eventId": format!("evt_pre_accept_blocked_repair_{request_id}"),
                        "blockerUserId": "u_bob",
                        "blockedUserId": "u_alice",
                        "scope": "friendship",
                        "effectiveAt": "2026-04-16T09:13:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect(
            "friendship block before pre-accept blocked repair scenario should return response",
        );
    assert_eq!(block_response.status(), StatusCode::OK);

    let request_snapshot = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/backend/v3/api/control/social/friend_requests/{request_id}"))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot before pre-accept blocked repair restart should return response");
    assert_eq!(request_snapshot.status(), StatusCode::OK);
    let request_snapshot_body = request_snapshot
        .into_body()
        .collect()
        .await
        .expect(
            "friend request snapshot before pre-accept blocked repair restart body should collect",
        )
        .to_bytes();
    let request_snapshot_json: serde_json::Value =
        serde_json::from_slice(&request_snapshot_body).expect(
            "friend request snapshot before pre-accept blocked repair restart body should be valid json",
        );
    assert_eq!(request_snapshot_json["friendRequest"]["status"], "pending");

    drop(app);

    let repair_store_path = runtime_dir
        .join("state")
        .join("social-friend-request-accept-repairs.json");
    fs::write(
        repair_store_path.as_path(),
        serde_json::json!({
            request_id.as_str(): {
                "tenant_id": "t_demo",
                "request_id": request_id,
                "requester_user_id": "u_alice",
                "target_user_id": "u_bob",
                "accepted_at": "2026-04-16T09:14:00Z"
            }
        })
        .to_string(),
    )
    .expect("pre-accept blocked repair store should be writable");

    let app_after_restart =
        local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let contacts = app_after_restart
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts after pre-accept blocked repair restart should return response");
    assert_eq!(contacts.status(), StatusCode::OK);
    let contacts_body = contacts
        .into_body()
        .collect()
        .await
        .expect("contacts after pre-accept blocked repair restart body should collect")
        .to_bytes();
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
        .expect("contacts after pre-accept blocked repair restart body should be valid json");
    assert!(
        contacts_json["items"]
            .as_array()
            .expect("contacts after pre-accept blocked repair restart should include items")
            .is_empty(),
        "pre-accept blocked pending repair should not materialize a friendship contact"
    );
    assert!(
        !repair_store_path.exists(),
        "pre-accept blocked pending repair should be discarded after convergence"
    );

    let request_snapshot_after_restart = app_after_restart
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/backend/v3/api/control/social/friend_requests/{request_id}"))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after pre-accept blocked repair restart should return response");
    assert_eq!(request_snapshot_after_restart.status(), StatusCode::OK);
    let request_snapshot_after_restart_body = request_snapshot_after_restart
        .into_body()
        .collect()
        .await
        .expect(
            "friend request snapshot after pre-accept blocked repair restart body should collect",
        )
        .to_bytes();
    let request_snapshot_after_restart_json: serde_json::Value = serde_json::from_slice(
        &request_snapshot_after_restart_body,
    )
    .expect(
        "friend request snapshot after pre-accept blocked repair restart body should be valid json",
    );
    assert_eq!(
        request_snapshot_after_restart_json["friendRequest"]["status"], "pending",
        "pre-accept blocked repair should leave the original request pending"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_concurrent_accepts_converge_idempotently_across_instances() {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _repair_store_delay = set_scoped_env_var(
        "CRAW_CHAT_TEST_SOCIAL_ACCEPT_REPAIR_STORE_IO_DELAY_MS",
        "250",
    );

    let runtime_dir = unique_test_runtime_dir("friend_request_accept_concurrent_idempotent");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_a = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let app_b = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let submit_request = app_a
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
                        "targetUserId":"u_bob",
                        "requestMessage":"concurrent accept idempotent"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before concurrent accept idempotent scenario should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before concurrent accept idempotent scenario body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before concurrent accept idempotent scenario body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("concurrent accept idempotent request id should be present")
        .to_owned();

    let accept_uri = format!("/im/v3/api/social/friend_requests/{request_id}/accept");

    let first_accept_task = tokio::spawn({
        let app = app_a.clone();
        let accept_uri = accept_uri.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .method("POST")
                    .uri(accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("first concurrent accept should return response")
        }
    });

    let second_accept_task = tokio::spawn({
        let app = app_b.clone();
        let accept_uri = accept_uri.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .method("POST")
                    .uri(accept_uri)
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_bob")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("second concurrent accept should return response")
        }
    });

    let first_accept = first_accept_task
        .await
        .expect("first concurrent accept task should join");
    let second_accept = second_accept_task
        .await
        .expect("second concurrent accept task should join");

    let first_accept_status = first_accept.status();
    let first_accept_body = first_accept
        .into_body()
        .collect()
        .await
        .expect("first concurrent accept body should collect")
        .to_bytes();
    let first_accept_json: serde_json::Value = serde_json::from_slice(&first_accept_body)
        .expect("first concurrent accept body should be valid json");
    assert_eq!(
        first_accept_status,
        StatusCode::OK,
        "first concurrent accept should converge successfully: {first_accept_json}"
    );

    let second_accept_status = second_accept.status();
    let second_accept_body = second_accept
        .into_body()
        .collect()
        .await
        .expect("second concurrent accept body should collect")
        .to_bytes();
    let second_accept_json: serde_json::Value = serde_json::from_slice(&second_accept_body)
        .expect("second concurrent accept body should be valid json");
    assert_eq!(
        second_accept_status,
        StatusCode::OK,
        "second concurrent accept should converge successfully instead of surfacing a stale not-pending conflict: {second_accept_json}"
    );

    assert_eq!(first_accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(second_accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(
        second_accept_json["friendship"]["friendshipId"],
        first_accept_json["friendship"]["friendshipId"]
    );
    assert_eq!(
        second_accept_json["directChat"]["directChatId"],
        first_accept_json["directChat"]["directChatId"]
    );
    assert_eq!(
        second_accept_json["conversation"]["conversationId"],
        first_accept_json["conversation"]["conversationId"]
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_friendship_removal_hides_contacts_via_app_social_route() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"hello bob"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id should be present")
        .to_owned();

    let accept_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("accept friend request should return response");
    assert_eq!(accept_request.status(), StatusCode::OK);
    let accept_request_body = accept_request
        .into_body()
        .collect()
        .await
        .expect("accept friend request body should collect")
        .to_bytes();
    let accept_request_json: serde_json::Value = serde_json::from_slice(&accept_request_body)
        .expect("accept friend request body should be valid json");
    let friendship_id = accept_request_json["friendship"]["friendshipId"]
        .as_str()
        .expect("friendship id should be present")
        .to_owned();

    let remove_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friendships/{friendship_id}/remove"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remove friendship should return response");
    assert_eq!(remove_request.status(), StatusCode::OK);
    let remove_request_body = remove_request
        .into_body()
        .collect()
        .await
        .expect("remove friendship body should collect")
        .to_bytes();
    let remove_request_json: serde_json::Value = serde_json::from_slice(&remove_request_body)
        .expect("remove friendship body should be valid json");
    assert_eq!(remove_request_json["friendship"]["status"], "removed");

    let alice_contacts = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("alice contacts request should return response");
    assert_eq!(alice_contacts.status(), StatusCode::OK);
    let alice_contacts_body = alice_contacts
        .into_body()
        .collect()
        .await
        .expect("alice contacts body should collect")
        .to_bytes();
    let alice_contacts_json: serde_json::Value = serde_json::from_slice(&alice_contacts_body)
        .expect("alice contacts body should be valid json");
    let alice_contact_items = alice_contacts_json["items"]
        .as_array()
        .expect("alice contacts items should be an array");
    assert!(
        alice_contact_items.is_empty(),
        "removed friendship must disappear from requester contacts"
    );

    let bob_contacts = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("bob contacts request should return response");
    assert_eq!(bob_contacts.status(), StatusCode::OK);
    let bob_contacts_body = bob_contacts
        .into_body()
        .collect()
        .await
        .expect("bob contacts body should collect")
        .to_bytes();
    let bob_contacts_json: serde_json::Value =
        serde_json::from_slice(&bob_contacts_body).expect("bob contacts body should be valid json");
    let bob_contact_items = bob_contacts_json["items"]
        .as_array()
        .expect("bob contacts items should be an array");
    assert!(
        bob_contact_items.is_empty(),
        "removed friendship must disappear from target contacts"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_friendship_scope_block_hides_contacts() {
    let app = local_minimal_node::build_default_app();
    let fixture = create_active_friendship_direct_chat_fixture(&app).await;

    let contacts_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts request before friendship block should return response");
    assert_eq!(contacts_before.status(), StatusCode::OK);
    let contacts_before_body = contacts_before
        .into_body()
        .collect()
        .await
        .expect("contacts before friendship block body should collect")
        .to_bytes();
    let contacts_before_json: serde_json::Value = serde_json::from_slice(&contacts_before_body)
        .expect("contacts before friendship block body should be valid json");
    let contacts_before_items = contacts_before_json["items"]
        .as_array()
        .expect("contacts before friendship block items should be an array");
    assert_eq!(contacts_before_items.len(), 1);
    assert_eq!(contacts_before_items[0]["targetUserId"], "u_bob");
    assert_eq!(
        contacts_before_items[0]["friendshipId"],
        fixture.friendship_id.as_str()
    );

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/user_blocks")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_admin")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "blockId": "ub_friendship_contacts_blocked",
                        "eventId": "evt_ub_friendship_contacts_blocked",
                        "blockerUserId": "u_bob",
                        "blockedUserId": "u_alice",
                        "scope": "friendship",
                        "effectiveAt": "2026-04-16T12:10:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("friendship scope block should return response");
    assert_eq!(block_response.status(), StatusCode::OK);

    let alice_contacts_after = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("alice contacts request after friendship block should return response");
    assert_eq!(alice_contacts_after.status(), StatusCode::OK);
    let alice_contacts_after_body = alice_contacts_after
        .into_body()
        .collect()
        .await
        .expect("alice contacts after friendship block body should collect")
        .to_bytes();
    let alice_contacts_after_json: serde_json::Value =
        serde_json::from_slice(&alice_contacts_after_body)
            .expect("alice contacts after friendship block body should be valid json");
    assert!(
        alice_contacts_after_json["items"]
            .as_array()
            .expect("alice contacts after friendship block items should be an array")
            .is_empty(),
        "friendship scope block must hide the blocked pair from requester contacts"
    );

    let bob_contacts_after = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("bob contacts request after friendship block should return response");
    assert_eq!(bob_contacts_after.status(), StatusCode::OK);
    let bob_contacts_after_body = bob_contacts_after
        .into_body()
        .collect()
        .await
        .expect("bob contacts after friendship block body should collect")
        .to_bytes();
    let bob_contacts_after_json: serde_json::Value =
        serde_json::from_slice(&bob_contacts_after_body)
            .expect("bob contacts after friendship block body should be valid json");
    assert!(
        bob_contacts_after_json["items"]
            .as_array()
            .expect("bob contacts after friendship block items should be an array")
            .is_empty(),
        "friendship scope block must hide the blocked pair from target contacts"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_friendship_remove_converges_when_friendship_was_externally_removed_after_app_snapshot()
 {
    let _env_lock = lock_social_accept_delay_env_guard().await;
    let _remove_snapshot_delay =
        set_scoped_env_var("CRAW_CHAT_TEST_SOCIAL_REMOVE_PRE_COMMIT_DELAY_MS", "200");

    let app = local_minimal_node::build_default_app();
    let fixture = create_active_friendship_direct_chat_fixture(&app).await;

    let remove_task = tokio::spawn({
        let app = app.clone();
        let friendship_id = fixture.friendship_id.clone();
        async move {
            app.oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/im/v3/api/social/friendships/{friendship_id}/remove"
                    ))
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_alice")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect(
                "IM open-platform friendship remove after stale snapshot should return response",
            )
        }
    });

    sleep(Duration::from_millis(50)).await;

    let external_remove = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/backend/v3/api/control/social/friendships/{}/remove",
                    fixture.friendship_id
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "eventId": format!("evt_external_remove_after_snapshot_{}", fixture.friendship_id),
                        "removedByUserId": "u_alice",
                        "removedAt": "2026-04-16T10:30:00Z"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("external remove after stale app snapshot should return response");
    assert_eq!(external_remove.status(), StatusCode::OK);

    let remove_response = remove_task
        .await
        .expect("remove task after stale app snapshot should join");
    assert_eq!(
        remove_response.status(),
        StatusCode::OK,
        "IM open-platform remove should converge to the externally removed friendship instead of surfacing friendship_not_active"
    );
    let remove_body = remove_response
        .into_body()
        .collect()
        .await
        .expect("remove response after stale app snapshot should collect")
        .to_bytes();
    let remove_json: serde_json::Value = serde_json::from_slice(&remove_body)
        .expect("remove response after stale app snapshot should be valid json");
    assert_eq!(remove_json["friendship"]["status"], "removed");
    assert_eq!(
        remove_json["friendship"]["friendshipId"],
        fixture.friendship_id
    );

    let contacts = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts after stale app remove convergence should return response");
    assert_eq!(contacts.status(), StatusCode::OK);
    let contacts_body = contacts
        .into_body()
        .collect()
        .await
        .expect("contacts after stale app remove convergence should collect")
        .to_bytes();
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_body)
        .expect("contacts after stale app remove convergence should be valid json");
    assert!(
        contacts_json["items"]
            .as_array()
            .expect("contacts after stale app remove convergence should include items")
            .is_empty(),
        "removed friendship must not remain in contacts after stale app remove convergence"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_can_submit_new_friend_request_after_friendship_removal() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"first hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first submit friend request should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("first submit friend request body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("first submit friend request body should be valid json");
    let first_request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("first request id should be present")
        .to_owned();

    let accept_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{first_request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("accept friend request should return response");
    assert_eq!(accept_request.status(), StatusCode::OK);
    let accept_request_body = accept_request
        .into_body()
        .collect()
        .await
        .expect("accept friend request body should collect")
        .to_bytes();
    let accept_request_json: serde_json::Value = serde_json::from_slice(&accept_request_body)
        .expect("accept friend request body should be valid json");
    let friendship_id = accept_request_json["friendship"]["friendshipId"]
        .as_str()
        .expect("friendship id should be present")
        .to_owned();

    let remove_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friendships/{friendship_id}/remove"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remove friendship should return response");
    assert_eq!(remove_request.status(), StatusCode::OK);

    let resubmit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"second hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("resubmit friend request should return response");
    assert_eq!(resubmit_request.status(), StatusCode::OK);
    let resubmit_request_body = resubmit_request
        .into_body()
        .collect()
        .await
        .expect("resubmit friend request body should collect")
        .to_bytes();
    let resubmit_request_json: serde_json::Value = serde_json::from_slice(&resubmit_request_body)
        .expect("resubmit friend request body should be valid json");

    assert_eq!(resubmit_request_json["friendRequest"]["status"], "pending");
    assert_ne!(
        resubmit_request_json["friendRequest"]["requestId"],
        first_request_id
    );
    assert_eq!(
        resubmit_request_json["friendRequest"]["requestMessage"],
        "second hello"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_can_accept_resubmitted_friend_request_after_friendship_removal()
{
    let app = local_minimal_node::build_default_app();

    let first_submit = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"first hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first submit before removal reaccept should return response");
    assert_eq!(first_submit.status(), StatusCode::OK);
    let first_submit_body = first_submit
        .into_body()
        .collect()
        .await
        .expect("first submit before removal reaccept body should collect")
        .to_bytes();
    let first_submit_json: serde_json::Value = serde_json::from_slice(&first_submit_body)
        .expect("first submit before removal reaccept body should be valid json");
    let first_request_id = first_submit_json["friendRequest"]["requestId"]
        .as_str()
        .expect("first request id should be present")
        .to_owned();

    let first_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{first_request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first accept before removal reaccept should return response");
    assert_eq!(first_accept.status(), StatusCode::OK);
    let first_accept_body = first_accept
        .into_body()
        .collect()
        .await
        .expect("first accept before removal reaccept body should collect")
        .to_bytes();
    let first_accept_json: serde_json::Value = serde_json::from_slice(&first_accept_body)
        .expect("first accept before removal reaccept body should be valid json");
    let first_friendship_id = first_accept_json["friendship"]["friendshipId"]
        .as_str()
        .expect("first friendship id should be present")
        .to_owned();

    let remove_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friendships/{first_friendship_id}/remove"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remove friendship before removal reaccept should return response");
    assert_eq!(remove_response.status(), StatusCode::OK);

    let second_submit = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"second hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second submit before removal reaccept should return response");
    assert_eq!(second_submit.status(), StatusCode::OK);
    let second_submit_body = second_submit
        .into_body()
        .collect()
        .await
        .expect("second submit before removal reaccept body should collect")
        .to_bytes();
    let second_submit_json: serde_json::Value = serde_json::from_slice(&second_submit_body)
        .expect("second submit before removal reaccept body should be valid json");
    let second_request_id = second_submit_json["friendRequest"]["requestId"]
        .as_str()
        .expect("second request id should be present")
        .to_owned();

    let second_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{second_request_id}/accept"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second accept after friendship removal should return response");
    assert_eq!(
        second_accept.status(),
        StatusCode::OK,
        "resubmitted friend request should be acceptable after friendship removal"
    );
    let second_accept_body = second_accept
        .into_body()
        .collect()
        .await
        .expect("second accept after friendship removal body should collect")
        .to_bytes();
    let second_accept_json: serde_json::Value = serde_json::from_slice(&second_accept_body)
        .expect("second accept after friendship removal body should be valid json");
    assert_eq!(second_accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(second_accept_json["friendship"]["status"], "active");

    let alice_contacts = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/contacts")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("contacts after removal reaccept should return response");
    assert_eq!(alice_contacts.status(), StatusCode::OK);
    let alice_contacts_body = alice_contacts
        .into_body()
        .collect()
        .await
        .expect("contacts after removal reaccept body should collect")
        .to_bytes();
    let alice_contacts_json: serde_json::Value = serde_json::from_slice(&alice_contacts_body)
        .expect("contacts after removal reaccept body should be valid json");
    let alice_contact_items = alice_contacts_json["items"]
        .as_array()
        .expect("contacts after removal reaccept should include items");
    assert_eq!(alice_contact_items.len(), 1);
    assert_eq!(alice_contact_items[0]["targetUserId"], "u_bob");
}

#[tokio::test]
async fn test_local_minimal_profile_hides_archived_direct_chat_from_inbox_after_friendship_removal()
{
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    let initial_post = post_standard_message_for_test(
        &app,
        conversation_id.as_str(),
        "u_alice",
        "client_archived_inbox_1",
        "before removal",
    )
    .await;
    assert_eq!(initial_post.status(), StatusCode::OK);

    let inbox_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("inbox before friendship removal should return response");
    assert_eq!(inbox_before.status(), StatusCode::OK);
    let inbox_before_body = inbox_before
        .into_body()
        .collect()
        .await
        .expect("inbox before friendship removal body should collect")
        .to_bytes();
    let inbox_before_json: serde_json::Value = serde_json::from_slice(&inbox_before_body)
        .expect("inbox before friendship removal body should be valid json");
    assert!(
        inbox_before_json["items"]
            .as_array()
            .expect("inbox before friendship removal should include items")
            .iter()
            .any(|item| item["conversationId"] == conversation_id),
        "active direct chat should appear in inbox before friendship removal"
    );

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let inbox_after = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("inbox after friendship removal should return response");
    assert_eq!(inbox_after.status(), StatusCode::OK);
    let inbox_after_body = inbox_after
        .into_body()
        .collect()
        .await
        .expect("inbox after friendship removal body should collect")
        .to_bytes();
    let inbox_after_json: serde_json::Value = serde_json::from_slice(&inbox_after_body)
        .expect("inbox after friendship removal body should be valid json");
    assert!(
        inbox_after_json["items"]
            .as_array()
            .expect("inbox after friendship removal should include items")
            .iter()
            .all(|item| item["conversationId"] != conversation_id),
        "archived direct chat must disappear from inbox after friendship removal"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_archived_direct_chat_summary_after_friendship_removal()
{
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    let initial_post = post_standard_message_for_test(
        &app,
        conversation_id.as_str(),
        "u_alice",
        "client_archived_summary_1",
        "before removal",
    )
    .await;
    assert_eq!(initial_post.status(), StatusCode::OK);

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let summary_response = app
        .oneshot(
            Request::builder()
                .uri(format!("/im/v3/api/chat/conversations/{conversation_id}"))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary after friendship removal should return response");
    assert_eq!(summary_response.status(), StatusCode::FORBIDDEN);
    let summary_body = summary_response
        .into_body()
        .collect()
        .await
        .expect("conversation summary after friendship removal body should collect")
        .to_bytes();
    let summary_json: serde_json::Value = serde_json::from_slice(&summary_body)
        .expect("conversation summary after friendship removal body should be valid json");
    assert_eq!(summary_json["code"], "conversation_archived");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_archived_direct_chat_edge_reads_after_friendship_removal()
 {
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    let members_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/members"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members before friendship removal should return response");
    assert_eq!(members_before.status(), StatusCode::OK);

    let member_directory_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/member_directory"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member directory before friendship removal should return response");
    assert_eq!(member_directory_before.status(), StatusCode::OK);

    let pins_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/pins"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("pins before friendship removal should return response");
    assert_eq!(pins_before.status(), StatusCode::OK);

    let post_message = post_standard_message_for_test(
        &app,
        conversation_id.as_str(),
        "u_alice",
        "client_archived_edge_reads_1",
        "before removal",
    )
    .await;
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_message_body = post_message
        .into_body()
        .collect()
        .await
        .expect("edge read seed message body should collect")
        .to_bytes();
    let post_message_json: serde_json::Value = serde_json::from_slice(&post_message_body)
        .expect("edge read seed message body should be valid json");
    let message_id = post_message_json["messageId"]
        .as_str()
        .expect("edge read seed message id should exist")
        .to_owned();

    let interaction_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages/{message_id}/interaction_summary"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("interaction summary before friendship removal should return response");
    assert_eq!(interaction_before.status(), StatusCode::OK);

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    for uri in [
        format!("/im/v3/api/chat/conversations/{conversation_id}/members"),
        format!("/im/v3/api/chat/conversations/{conversation_id}/member_directory"),
        format!("/im/v3/api/chat/conversations/{conversation_id}/pins"),
        format!(
            "/im/v3/api/chat/conversations/{conversation_id}/messages/{message_id}/interaction_summary"
        ),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(uri.as_str())
                    .header("x-sdkwork-tenant-id", "t_demo")
                    .header("x-sdkwork-user-id", "u_alice")
                    .header("x-sdkwork-actor-kind", "user")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("archived edge read should return response");
        assert_eq!(
            response.status(),
            StatusCode::FORBIDDEN,
            "archived direct chat edge read must be forbidden: {uri}"
        );
        let body = response
            .into_body()
            .collect()
            .await
            .expect("archived edge read body should collect")
            .to_bytes();
        let json: serde_json::Value =
            serde_json::from_slice(&body).expect("archived edge read body should be valid json");
        assert_eq!(
            json["code"], "conversation_archived",
            "archived direct chat edge read must expose conversation_archived: {uri}"
        );
    }
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_archived_direct_chat_timeline_after_friendship_removal()
{
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    let initial_post = post_standard_message_for_test(
        &app,
        conversation_id.as_str(),
        "u_alice",
        "client_archived_timeline_1",
        "before removal",
    )
    .await;
    assert_eq!(initial_post.status(), StatusCode::OK);

    let timeline_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline before friendship removal should return response");
    assert_eq!(timeline_before.status(), StatusCode::OK);

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let timeline_after = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/messages"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline after friendship removal should return response");
    assert_eq!(timeline_after.status(), StatusCode::FORBIDDEN);
    let timeline_after_body = timeline_after
        .into_body()
        .collect()
        .await
        .expect("timeline after friendship removal body should collect")
        .to_bytes();
    let timeline_after_json: serde_json::Value = serde_json::from_slice(&timeline_after_body)
        .expect("timeline after friendship removal body should be valid json");
    assert_eq!(timeline_after_json["code"], "conversation_archived");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_archived_direct_chat_read_cursor_access_after_friendship_removal()
 {
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    register_client_route_for_test(&app, "u_alice", "d_alice_phone").await;

    let post_message = post_standard_message_for_test(
        &app,
        conversation_id.as_str(),
        "u_alice",
        "client_archived_read_cursor_1",
        "before removal",
    )
    .await;
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_message_body = post_message
        .into_body()
        .collect()
        .await
        .expect("read cursor seed message body should collect")
        .to_bytes();
    let post_message_json: serde_json::Value = serde_json::from_slice(&post_message_body)
        .expect("read cursor seed message body should be valid json");
    let message_id = post_message_json["messageId"]
        .as_str()
        .expect("read cursor seed message id should exist")
        .to_owned();

    let read_cursor_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/read_cursor"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("read cursor before friendship removal should return response");
    assert_eq!(read_cursor_before.status(), StatusCode::OK);

    let update_read_cursor_before = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/read_cursor"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_alice_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "readSeq": 1,
                        "lastReadMessageId": message_id,
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("read cursor update before friendship removal should return response");
    assert_eq!(update_read_cursor_before.status(), StatusCode::OK);

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let read_cursor_after = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/read_cursor"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("read cursor after friendship removal should return response");
    assert_eq!(read_cursor_after.status(), StatusCode::FORBIDDEN);
    let read_cursor_after_body = read_cursor_after
        .into_body()
        .collect()
        .await
        .expect("read cursor after friendship removal body should collect")
        .to_bytes();
    let read_cursor_after_json: serde_json::Value = serde_json::from_slice(&read_cursor_after_body)
        .expect("read cursor after friendship removal body should be valid json");
    assert_eq!(read_cursor_after_json["code"], "conversation_archived");

    let update_read_cursor_after = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/chat/conversations/{conversation_id}/read_cursor"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_alice_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "readSeq": 1,
                        "lastReadMessageId": message_id,
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("read cursor update after friendship removal should return response");
    assert_eq!(update_read_cursor_after.status(), StatusCode::FORBIDDEN);
    let update_read_cursor_after_body = update_read_cursor_after
        .into_body()
        .collect()
        .await
        .expect("read cursor update after friendship removal body should collect")
        .to_bytes();
    let update_read_cursor_after_json: serde_json::Value =
        serde_json::from_slice(&update_read_cursor_after_body)
            .expect("read cursor update after friendship removal body should be valid json");
    assert_eq!(
        update_read_cursor_after_json["code"],
        "conversation_archived"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_post_message_to_archived_direct_chat_after_friendship_removal()
 {
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    let initial_post = post_standard_message_for_test(
        &app,
        conversation_id.as_str(),
        "u_alice",
        "client_archived_write_1",
        "before removal",
    )
    .await;
    assert_eq!(initial_post.status(), StatusCode::OK);

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let archived_post = post_standard_message_for_test(
        &app,
        conversation_id.as_str(),
        "u_alice",
        "client_archived_write_2",
        "after removal",
    )
    .await;
    assert_eq!(archived_post.status(), StatusCode::FORBIDDEN);
    let archived_post_body = archived_post
        .into_body()
        .collect()
        .await
        .expect("archived direct chat post body should collect")
        .to_bytes();
    let archived_post_json: serde_json::Value = serde_json::from_slice(&archived_post_body)
        .expect("archived direct chat post body should be valid json");
    assert_eq!(archived_post_json["code"], "conversation_archived");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_archived_direct_chat_rtc_create_after_friendship_removal()
 {
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    let create_rtc_before = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "rtcSessionId": "rtc_archived_before_removal",
                        "conversationId": conversation_id,
                        "rtcMode": "voice",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("rtc create before friendship removal should return response");
    assert_eq!(create_rtc_before.status(), StatusCode::OK);

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let create_rtc_after = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "rtcSessionId": "rtc_archived_after_removal",
                        "conversationId": conversation_id,
                        "rtcMode": "voice",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("rtc create after friendship removal should return response");
    assert_eq!(create_rtc_after.status(), StatusCode::FORBIDDEN);
    let create_rtc_after_body = create_rtc_after
        .into_body()
        .collect()
        .await
        .expect("rtc create after friendship removal body should collect")
        .to_bytes();
    let create_rtc_after_json: serde_json::Value = serde_json::from_slice(&create_rtc_after_body)
        .expect("rtc create after friendship removal body should be valid json");
    assert_eq!(create_rtc_after_json["code"], "conversation_archived");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_archived_direct_chat_stream_open_after_friendship_removal()
 {
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    let open_stream_before = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "streamId": "st_archived_before_removal",
                        "streamType": "custom.delta.text",
                        "scopeKind": "conversation",
                        "scopeId": conversation_id,
                        "durabilityClass": "durableSession",
                        "schemaRef": "custom.delta.text.v1",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("stream open before friendship removal should return response");
    assert_eq!(open_stream_before.status(), StatusCode::OK);

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let open_stream_after = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "streamId": "st_archived_after_removal",
                        "streamType": "custom.delta.text",
                        "scopeKind": "conversation",
                        "scopeId": conversation_id,
                        "durabilityClass": "durableSession",
                        "schemaRef": "custom.delta.text.v1",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("stream open after friendship removal should return response");
    assert_eq!(open_stream_after.status(), StatusCode::FORBIDDEN);
    let open_stream_after_body = open_stream_after
        .into_body()
        .collect()
        .await
        .expect("stream open after friendship removal body should collect")
        .to_bytes();
    let open_stream_after_json: serde_json::Value = serde_json::from_slice(&open_stream_after_body)
        .expect("stream open after friendship removal body should be valid json");
    assert_eq!(open_stream_after_json["code"], "conversation_archived");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_archived_direct_chat_existing_stream_access_after_friendship_removal()
 {
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;
    let stream_id = "st_archived_existing_stream";

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "streamId": stream_id,
                        "streamType": "custom.delta.text",
                        "scopeKind": "conversation",
                        "scopeId": conversation_id,
                        "durabilityClass": "durableSession",
                        "schemaRef": "custom.delta.text.v1",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("existing stream open before friendship removal should return response");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let append_before = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/streams/{stream_id}/frames"))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"before removal\"}",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("existing stream append before friendship removal should return response");
    assert_eq!(append_before.status(), StatusCode::OK);

    let list_before = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/streams/{stream_id}/frames?afterFrameSeq=0&limit=10"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("existing stream list before friendship removal should return response");
    assert_eq!(list_before.status(), StatusCode::OK);

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let list_after = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/streams/{stream_id}/frames?afterFrameSeq=0&limit=10"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("existing stream list after friendship removal should return response");
    assert_eq!(list_after.status(), StatusCode::FORBIDDEN);
    let list_after_body = list_after
        .into_body()
        .collect()
        .await
        .expect("existing stream list after friendship removal body should collect")
        .to_bytes();
    let list_after_json: serde_json::Value = serde_json::from_slice(&list_after_body)
        .expect("existing stream list after friendship removal body should be valid json");
    assert_eq!(list_after_json["code"], "conversation_archived");

    let append_after = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/im/v3/api/streams/{stream_id}/frames"))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "frameSeq": 2,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"after removal\"}",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("existing stream append after friendship removal should return response");
    assert_eq!(append_after.status(), StatusCode::FORBIDDEN);
    let append_after_body = append_after
        .into_body()
        .collect()
        .await
        .expect("existing stream append after friendship removal body should collect")
        .to_bytes();
    let append_after_json: serde_json::Value = serde_json::from_slice(&append_after_body)
        .expect("existing stream append after friendship removal body should be valid json");
    assert_eq!(append_after_json["code"], "conversation_archived");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_archived_direct_chat_existing_rtc_capability_after_friendship_removal()
 {
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;
    let rtc_session_id = "rtc_archived_existing_session";

    let create_rtc = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/calls/sessions")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "rtcSessionId": rtc_session_id,
                        "conversationId": conversation_id,
                        "rtcMode": "voice",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("existing rtc session create before friendship removal should return response");
    assert_eq!(create_rtc.status(), StatusCode::OK);

    let credential_before = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/calls/sessions/{rtc_session_id}/credentials"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "participantId": "u_bob",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("existing rtc credential before friendship removal should return response");
    assert_eq!(credential_before.status(), StatusCode::OK);

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let credential_after = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/calls/sessions/{rtc_session_id}/credentials"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "participantId": "u_bob",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("existing rtc credential after friendship removal should return response");
    assert_eq!(credential_after.status(), StatusCode::FORBIDDEN);
    let credential_after_body = credential_after
        .into_body()
        .collect()
        .await
        .expect("existing rtc credential after friendship removal body should collect")
        .to_bytes();
    let credential_after_json: serde_json::Value = serde_json::from_slice(&credential_after_body)
        .expect("existing rtc credential after friendship removal body should be valid json");
    assert_eq!(credential_after_json["code"], "conversation_archived");
}

#[tokio::test]
async fn test_local_minimal_profile_hides_archived_direct_chat_realtime_events_after_friendship_removal()
 {
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    register_client_route_for_test(&app, "u_alice", "d_alice_phone").await;
    register_client_route_for_test(&app, "u_alice", "d_alice_pad").await;

    let sync_subscription = sync_conversation_realtime_subscription_for_test(
        &app,
        "u_alice",
        "d_alice_pad",
        conversation_id.as_str(),
    )
    .await;
    assert_eq!(sync_subscription.status(), StatusCode::OK);

    let post_message = post_standard_message_for_test(
        &app,
        conversation_id.as_str(),
        "u_alice",
        "client_archived_realtime_1",
        "before removal",
    )
    .await;
    assert_eq!(post_message.status(), StatusCode::OK);

    let realtime_before = list_realtime_events_for_test(&app, "u_alice", "d_alice_pad", 0).await;
    assert_eq!(realtime_before.status(), StatusCode::OK);
    let realtime_before_body = realtime_before
        .into_body()
        .collect()
        .await
        .expect("realtime events before friendship removal body should collect")
        .to_bytes();
    let realtime_before_json: serde_json::Value = serde_json::from_slice(&realtime_before_body)
        .expect("realtime events before friendship removal body should be valid json");
    assert!(
        realtime_before_json["items"]
            .as_array()
            .expect("realtime events before friendship removal should include items")
            .iter()
            .any(|item| item["scopeId"] == conversation_id),
        "active direct chat should be visible in realtime events before friendship removal"
    );

    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let realtime_after = list_realtime_events_for_test(&app, "u_alice", "d_alice_pad", 0).await;
    assert_eq!(realtime_after.status(), StatusCode::OK);
    let realtime_after_body = realtime_after
        .into_body()
        .collect()
        .await
        .expect("realtime events after friendship removal body should collect")
        .to_bytes();
    let realtime_after_json: serde_json::Value = serde_json::from_slice(&realtime_after_body)
        .expect("realtime events after friendship removal body should be valid json");
    assert!(
        realtime_after_json["items"]
            .as_array()
            .expect("realtime events after friendship removal should include items")
            .iter()
            .all(|item| item["scopeId"] != conversation_id),
        "archived direct chat must disappear from realtime event windows after friendship removal"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_archived_direct_chat_realtime_subscription_after_friendship_removal()
 {
    let app = local_minimal_node::build_default_app();
    let (friendship_id, conversation_id) = create_active_friendship_direct_chat(&app).await;

    register_client_route_for_test(&app, "u_alice", "d_alice_pad").await;
    remove_friendship_for_test(&app, friendship_id.as_str(), "u_alice").await;

    let sync_subscription = sync_conversation_realtime_subscription_for_test(
        &app,
        "u_alice",
        "d_alice_pad",
        conversation_id.as_str(),
    )
    .await;
    assert_eq!(sync_subscription.status(), StatusCode::FORBIDDEN);
    let sync_subscription_body = sync_subscription
        .into_body()
        .collect()
        .await
        .expect("archived realtime subscription body should collect")
        .to_bytes();
    let sync_subscription_json: serde_json::Value = serde_json::from_slice(&sync_subscription_body)
        .expect("archived realtime subscription body should be valid json");
    assert_eq!(sync_subscription_json["code"], "conversation_archived");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_direct_chat_summary_when_direct_chat_scope_block_is_active()
 {
    let app = local_minimal_node::build_default_app();
    let fixture = create_active_friendship_direct_chat_fixture(&app).await;

    let initial_post = post_standard_message_for_test(
        &app,
        fixture.conversation_id.as_str(),
        "u_alice",
        "client_direct_chat_block_summary_1",
        "before block",
    )
    .await;
    assert_eq!(initial_post.status(), StatusCode::OK);

    block_direct_chat_for_test(
        &app,
        "ub_direct_chat_summary_blocked",
        "u_bob",
        "u_alice",
        fixture.direct_chat_id.as_str(),
    )
    .await;

    let summary_response = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/chat/conversations/{}",
                    fixture.conversation_id
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("blocked direct chat summary should return response");
    assert_eq!(summary_response.status(), StatusCode::FORBIDDEN);
    let summary_body = summary_response
        .into_body()
        .collect()
        .await
        .expect("blocked direct chat summary body should collect")
        .to_bytes();
    let summary_json: serde_json::Value =
        serde_json::from_slice(&summary_body).expect("blocked direct chat summary should be json");
    assert_eq!(summary_json["code"], "conversation_blocked");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_post_message_when_direct_chat_scope_block_is_active() {
    let app = local_minimal_node::build_default_app();
    let fixture = create_active_friendship_direct_chat_fixture(&app).await;

    block_direct_chat_for_test(
        &app,
        "ub_direct_chat_post_blocked",
        "u_bob",
        "u_alice",
        fixture.direct_chat_id.as_str(),
    )
    .await;

    let blocked_post = post_standard_message_for_test(
        &app,
        fixture.conversation_id.as_str(),
        "u_alice",
        "client_direct_chat_block_post_1",
        "after block",
    )
    .await;
    assert_eq!(blocked_post.status(), StatusCode::FORBIDDEN);
    let blocked_post_body = blocked_post
        .into_body()
        .collect()
        .await
        .expect("blocked direct chat post body should collect")
        .to_bytes();
    let blocked_post_json: serde_json::Value = serde_json::from_slice(&blocked_post_body)
        .expect("blocked direct chat post should be json");
    assert_eq!(blocked_post_json["code"], "conversation_blocked");
}

#[tokio::test]
async fn test_local_minimal_profile_hides_direct_chat_from_inbox_when_direct_chat_scope_block_is_active()
 {
    let app = local_minimal_node::build_default_app();
    let fixture = create_active_friendship_direct_chat_fixture(&app).await;

    let initial_post = post_standard_message_for_test(
        &app,
        fixture.conversation_id.as_str(),
        "u_alice",
        "client_direct_chat_block_inbox_1",
        "before block",
    )
    .await;
    assert_eq!(initial_post.status(), StatusCode::OK);

    block_direct_chat_for_test(
        &app,
        "ub_direct_chat_inbox_blocked",
        "u_bob",
        "u_alice",
        fixture.direct_chat_id.as_str(),
    )
    .await;

    let inbox_response = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("blocked direct chat inbox should return response");
    assert_eq!(inbox_response.status(), StatusCode::OK);
    let inbox_body = inbox_response
        .into_body()
        .collect()
        .await
        .expect("blocked direct chat inbox body should collect")
        .to_bytes();
    let inbox_json: serde_json::Value =
        serde_json::from_slice(&inbox_body).expect("blocked direct chat inbox should be json");
    assert!(
        inbox_json["items"]
            .as_array()
            .expect("blocked direct chat inbox should include items")
            .iter()
            .all(|item| item["conversationId"] != fixture.conversation_id),
        "blocked direct chat must disappear from inbox"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_filters_direct_chat_realtime_events_when_direct_chat_scope_block_is_active()
 {
    let app = local_minimal_node::build_default_app();
    let fixture = create_active_friendship_direct_chat_fixture(&app).await;

    register_client_route_for_test(&app, "u_alice", "d_alice_phone").await;
    register_client_route_for_test(&app, "u_alice", "d_alice_pad").await;

    let sync_subscription = sync_conversation_realtime_subscription_for_test(
        &app,
        "u_alice",
        "d_alice_pad",
        fixture.conversation_id.as_str(),
    )
    .await;
    assert_eq!(sync_subscription.status(), StatusCode::OK);

    let post_message = post_standard_message_for_test(
        &app,
        fixture.conversation_id.as_str(),
        "u_bob",
        "client_direct_chat_block_realtime_1",
        "before block",
    )
    .await;
    assert_eq!(post_message.status(), StatusCode::OK);

    block_direct_chat_for_test(
        &app,
        "ub_direct_chat_realtime_blocked",
        "u_bob",
        "u_alice",
        fixture.direct_chat_id.as_str(),
    )
    .await;

    let realtime_after = list_realtime_events_for_test(&app, "u_alice", "d_alice_pad", 0).await;
    assert_eq!(realtime_after.status(), StatusCode::OK);
    let realtime_after_body = realtime_after
        .into_body()
        .collect()
        .await
        .expect("blocked realtime events body should collect")
        .to_bytes();
    let realtime_after_json: serde_json::Value = serde_json::from_slice(&realtime_after_body)
        .expect("blocked realtime events should be json");
    assert!(
        realtime_after_json["items"]
            .as_array()
            .expect("blocked realtime events should include items")
            .iter()
            .all(|item| item["scopeId"] != fixture.conversation_id),
        "blocked direct chat must disappear from realtime event windows"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_direct_chat_realtime_subscription_when_direct_chat_scope_block_is_active()
 {
    let app = local_minimal_node::build_default_app();
    let fixture = create_active_friendship_direct_chat_fixture(&app).await;

    register_client_route_for_test(&app, "u_alice", "d_alice_pad").await;
    block_direct_chat_for_test(
        &app,
        "ub_direct_chat_subscription_blocked",
        "u_bob",
        "u_alice",
        fixture.direct_chat_id.as_str(),
    )
    .await;

    let sync_subscription = sync_conversation_realtime_subscription_for_test(
        &app,
        "u_alice",
        "d_alice_pad",
        fixture.conversation_id.as_str(),
    )
    .await;
    assert_eq!(sync_subscription.status(), StatusCode::FORBIDDEN);
    let sync_subscription_body = sync_subscription
        .into_body()
        .collect()
        .await
        .expect("blocked realtime subscription body should collect")
        .to_bytes();
    let sync_subscription_json: serde_json::Value = serde_json::from_slice(&sync_subscription_body)
        .expect("blocked realtime subscription body should be valid json");
    assert_eq!(sync_subscription_json["code"], "conversation_blocked");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_self_friend_request_with_stable_error() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/friend_requests")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "targetUserId":"u_demo",
                        "requestMessage":"please add me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("self friend request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("self friend request body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("self friend request body should be valid json");
    assert_eq!(json["code"], "friend_request_self_not_allowed");
    assert!(
        !json["message"]
            .as_str()
            .unwrap_or_default()
            .contains("IdenticalPair"),
        "self friend request error must not expose domain debug internals"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_can_resubmit_friend_request_after_decline() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"please add me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before decline should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before decline body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before decline body should be valid json");
    let first_request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("first declined request id should be present")
        .to_owned();

    let decline_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{first_request_id}/decline"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("decline friend request should return response");
    assert_eq!(decline_request.status(), StatusCode::OK);
    let decline_request_body = decline_request
        .into_body()
        .collect()
        .await
        .expect("decline friend request body should collect")
        .to_bytes();
    let decline_request_json: serde_json::Value = serde_json::from_slice(&decline_request_body)
        .expect("decline friend request body should be valid json");
    assert_eq!(decline_request_json["friendRequest"]["status"], "declined");

    let resubmit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"please add me again"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("resubmit friend request after decline should return response");
    assert_eq!(resubmit_request.status(), StatusCode::OK);
    let resubmit_request_body = resubmit_request
        .into_body()
        .collect()
        .await
        .expect("resubmit friend request after decline body should collect")
        .to_bytes();
    let resubmit_request_json: serde_json::Value = serde_json::from_slice(&resubmit_request_body)
        .expect("resubmit friend request after decline body should be valid json");

    assert_eq!(resubmit_request_json["friendRequest"]["status"], "pending");
    assert_ne!(
        resubmit_request_json["friendRequest"]["requestId"],
        first_request_id
    );
    assert_eq!(
        resubmit_request_json["friendRequest"]["requestMessage"],
        "please add me again"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_can_resubmit_friend_request_after_cancel() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"please add me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before cancel should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before cancel body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before cancel body should be valid json");
    let first_request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("first canceled request id should be present")
        .to_owned();

    let cancel_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{first_request_id}/cancel"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("cancel friend request should return response");
    assert_eq!(cancel_request.status(), StatusCode::OK);
    let cancel_request_body = cancel_request
        .into_body()
        .collect()
        .await
        .expect("cancel friend request body should collect")
        .to_bytes();
    let cancel_request_json: serde_json::Value = serde_json::from_slice(&cancel_request_body)
        .expect("cancel friend request body should be valid json");
    assert_eq!(cancel_request_json["friendRequest"]["status"], "canceled");

    let resubmit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"please add me again"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("resubmit friend request after cancel should return response");
    assert_eq!(resubmit_request.status(), StatusCode::OK);
    let resubmit_request_body = resubmit_request
        .into_body()
        .collect()
        .await
        .expect("resubmit friend request after cancel body should collect")
        .to_bytes();
    let resubmit_request_json: serde_json::Value = serde_json::from_slice(&resubmit_request_body)
        .expect("resubmit friend request after cancel body should be valid json");

    assert_eq!(resubmit_request_json["friendRequest"]["status"], "pending");
    assert_ne!(
        resubmit_request_json["friendRequest"]["requestId"],
        first_request_id
    );
    assert_eq!(
        resubmit_request_json["friendRequest"]["requestMessage"],
        "please add me again"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_lists_incoming_and_outgoing_friend_requests() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"please add me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before list should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before list body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before list body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id before list should be present")
        .to_owned();

    let incoming_pending = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/friend_requests?direction=incoming")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("incoming friend request list should return response");
    assert_eq!(incoming_pending.status(), StatusCode::OK);
    let incoming_pending_body = incoming_pending
        .into_body()
        .collect()
        .await
        .expect("incoming friend request list body should collect")
        .to_bytes();
    let incoming_pending_json: serde_json::Value = serde_json::from_slice(&incoming_pending_body)
        .expect("incoming friend request list body should be valid json");
    let incoming_pending_items = incoming_pending_json["items"]
        .as_array()
        .expect("incoming friend request list should include items");
    assert_eq!(incoming_pending_items.len(), 1);
    assert_eq!(incoming_pending_items[0]["requestId"], request_id);
    assert_eq!(incoming_pending_items[0]["status"], "pending");

    let outgoing_pending = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/friend_requests?direction=outgoing")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("outgoing friend request list should return response");
    assert_eq!(outgoing_pending.status(), StatusCode::OK);
    let outgoing_pending_body = outgoing_pending
        .into_body()
        .collect()
        .await
        .expect("outgoing friend request list body should collect")
        .to_bytes();
    let outgoing_pending_json: serde_json::Value = serde_json::from_slice(&outgoing_pending_body)
        .expect("outgoing friend request list body should be valid json");
    let outgoing_pending_items = outgoing_pending_json["items"]
        .as_array()
        .expect("outgoing friend request list should include items");
    assert_eq!(outgoing_pending_items.len(), 1);
    assert_eq!(outgoing_pending_items[0]["requestId"], request_id);
    assert_eq!(outgoing_pending_items[0]["status"], "pending");

    let decline_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/decline"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("decline friend request before filtered list should return response");
    assert_eq!(decline_request.status(), StatusCode::OK);

    let incoming_after_decline = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/friend_requests?direction=incoming")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("incoming pending friend request list after decline should return response");
    assert_eq!(incoming_after_decline.status(), StatusCode::OK);
    let incoming_after_decline_body = incoming_after_decline
        .into_body()
        .collect()
        .await
        .expect("incoming pending friend request list after decline body should collect")
        .to_bytes();
    let incoming_after_decline_json: serde_json::Value =
        serde_json::from_slice(&incoming_after_decline_body)
            .expect("incoming pending friend request list after decline body should be valid json");
    let incoming_after_decline_items = incoming_after_decline_json["items"]
        .as_array()
        .expect("incoming pending friend request list after decline should include items");
    assert!(incoming_after_decline_items.is_empty());

    let outgoing_declined = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/friend_requests?direction=outgoing&status=declined")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("outgoing declined friend request list should return response");
    assert_eq!(outgoing_declined.status(), StatusCode::OK);
    let outgoing_declined_body = outgoing_declined
        .into_body()
        .collect()
        .await
        .expect("outgoing declined friend request list body should collect")
        .to_bytes();
    let outgoing_declined_json: serde_json::Value = serde_json::from_slice(&outgoing_declined_body)
        .expect("outgoing declined friend request list body should be valid json");
    let outgoing_declined_items = outgoing_declined_json["items"]
        .as_array()
        .expect("outgoing declined friend request list should include items");
    assert_eq!(outgoing_declined_items.len(), 1);
    assert_eq!(outgoing_declined_items[0]["requestId"], request_id);
    assert_eq!(outgoing_declined_items[0]["status"], "declined");
}

#[tokio::test]
async fn test_local_minimal_profile_friend_request_list_applies_limit_after_sorting() {
    let app = local_minimal_node::build_default_app();

    let first_submit = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"first"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first submit before limited list should return response");
    assert_eq!(first_submit.status(), StatusCode::OK);

    sleep(Duration::from_millis(2)).await;

    let second_submit = app
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
                        "targetUserId":"u_carol",
                        "requestMessage":"second"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second submit before limited list should return response");
    assert_eq!(second_submit.status(), StatusCode::OK);

    let limited_list = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/friend_requests?direction=outgoing&limit=1")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("limited IM open-platform friend request list should return response");
    assert_eq!(limited_list.status(), StatusCode::OK);
    let limited_list_body = limited_list
        .into_body()
        .collect()
        .await
        .expect("limited IM open-platform friend request list body should collect")
        .to_bytes();
    let limited_list_json: serde_json::Value = serde_json::from_slice(&limited_list_body)
        .expect("limited IM open-platform friend request list body should be valid json");
    let items = limited_list_json["items"]
        .as_array()
        .expect("limited IM open-platform friend request list should include items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["targetUserId"], "u_carol");
}

#[tokio::test]
async fn test_local_minimal_profile_friend_request_list_preserves_plus_in_actor_id() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/social/friend_requests")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice+plus")
                .header("x-sdkwork-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "targetUserId":"u_bob",
                        "requestMessage":"plus identity"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request for plus actor should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);

    let list_request = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/friend_requests?direction=outgoing")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice+plus")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request list for plus actor should return response");
    assert_eq!(list_request.status(), StatusCode::OK);
    let list_body = list_request
        .into_body()
        .collect()
        .await
        .expect("friend request list for plus actor body should collect")
        .to_bytes();
    let list_json: serde_json::Value = serde_json::from_slice(&list_body)
        .expect("friend request list for plus actor body should be valid json");
    let items = list_json["items"]
        .as_array()
        .expect("friend request list for plus actor should include items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["requesterUserId"], "u_alice+plus");
    assert_eq!(items[0]["targetUserId"], "u_bob");
}

#[tokio::test]
async fn test_local_minimal_profile_friend_request_list_uses_cursor_for_next_page() {
    let app = local_minimal_node::build_default_app();

    let first_submit = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"first"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first submit before cursor list should return response");
    assert_eq!(first_submit.status(), StatusCode::OK);

    sleep(Duration::from_millis(2)).await;

    let second_submit = app
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
                        "targetUserId":"u_carol",
                        "requestMessage":"second"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second submit before cursor list should return response");
    assert_eq!(second_submit.status(), StatusCode::OK);

    let first_page = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/friend_requests?direction=outgoing&limit=1")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("first IM open-platform cursor page should return response");
    assert_eq!(first_page.status(), StatusCode::OK);
    let first_page_body = first_page
        .into_body()
        .collect()
        .await
        .expect("first IM open-platform cursor page body should collect")
        .to_bytes();
    let first_page_json: serde_json::Value = serde_json::from_slice(&first_page_body)
        .expect("first IM open-platform cursor page body should be valid json");
    let first_items = first_page_json["items"]
        .as_array()
        .expect("first IM open-platform cursor page should include items");
    assert_eq!(first_items.len(), 1);
    assert_eq!(first_items[0]["targetUserId"], "u_carol");
    let next_cursor = first_page_json["nextCursor"]
        .as_str()
        .expect("first IM open-platform cursor page should include nextCursor");

    let second_page = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!(
                    "/im/v3/api/social/friend_requests?direction=outgoing&limit=1&cursor={next_cursor}"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("second IM open-platform cursor page should return response");
    assert_eq!(second_page.status(), StatusCode::OK);
    let second_page_body = second_page
        .into_body()
        .collect()
        .await
        .expect("second IM open-platform cursor page body should collect")
        .to_bytes();
    let second_page_json: serde_json::Value = serde_json::from_slice(&second_page_body)
        .expect("second IM open-platform cursor page body should be valid json");
    let second_items = second_page_json["items"]
        .as_array()
        .expect("second IM open-platform cursor page should include items");
    assert_eq!(second_items.len(), 1);
    assert_eq!(second_items[0]["targetUserId"], "u_bob");
    assert!(second_page_json["nextCursor"].is_null());
}

#[tokio::test]
async fn test_local_minimal_profile_friend_request_list_rejects_invalid_cursor() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/social/friend_requests?direction=outgoing&cursor=not-valid")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("invalid IM open-platform cursor inventory request should return response");
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("invalid IM open-platform cursor inventory body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body)
        .expect("invalid IM open-platform cursor inventory body should be valid json");
    assert_eq!(json["code"], "cursor_invalid");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_friend_request_decline_from_non_target_user() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"please add me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before forbidden decline should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before forbidden decline body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before forbidden decline body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id for forbidden decline should be present")
        .to_owned();

    let forbidden_decline = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/decline"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_charlie")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("forbidden decline should return response");
    assert_eq!(forbidden_decline.status(), StatusCode::FORBIDDEN);
    let forbidden_decline_body = forbidden_decline
        .into_body()
        .collect()
        .await
        .expect("forbidden decline body should collect")
        .to_bytes();
    let forbidden_decline_json: serde_json::Value = serde_json::from_slice(&forbidden_decline_body)
        .expect("forbidden decline body should be valid json");
    assert_eq!(
        forbidden_decline_json["code"],
        "friend_request_decline_forbidden"
    );

    let allowed_decline = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/decline"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("allowed decline should return response");
    assert_eq!(allowed_decline.status(), StatusCode::OK);
    let allowed_decline_body = allowed_decline
        .into_body()
        .collect()
        .await
        .expect("allowed decline body should collect")
        .to_bytes();
    let allowed_decline_json: serde_json::Value = serde_json::from_slice(&allowed_decline_body)
        .expect("allowed decline body should be valid json");
    assert_eq!(allowed_decline_json["friendRequest"]["status"], "declined");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_friend_request_cancel_from_non_requester_user() {
    let app = local_minimal_node::build_default_app();

    let submit_request = app
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
                        "targetUserId":"u_bob",
                        "requestMessage":"please add me"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("submit friend request before forbidden cancel should return response");
    assert_eq!(submit_request.status(), StatusCode::OK);
    let submit_request_body = submit_request
        .into_body()
        .collect()
        .await
        .expect("submit friend request before forbidden cancel body should collect")
        .to_bytes();
    let submit_request_json: serde_json::Value = serde_json::from_slice(&submit_request_body)
        .expect("submit friend request before forbidden cancel body should be valid json");
    let request_id = submit_request_json["friendRequest"]["requestId"]
        .as_str()
        .expect("request id for forbidden cancel should be present")
        .to_owned();

    let forbidden_cancel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/cancel"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_bob")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("forbidden cancel should return response");
    assert_eq!(forbidden_cancel.status(), StatusCode::FORBIDDEN);
    let forbidden_cancel_body = forbidden_cancel
        .into_body()
        .collect()
        .await
        .expect("forbidden cancel body should collect")
        .to_bytes();
    let forbidden_cancel_json: serde_json::Value = serde_json::from_slice(&forbidden_cancel_body)
        .expect("forbidden cancel body should be valid json");
    assert_eq!(
        forbidden_cancel_json["code"],
        "friend_request_cancel_forbidden"
    );

    let allowed_cancel = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/im/v3/api/social/friend_requests/{request_id}/cancel"
                ))
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_alice")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("allowed cancel should return response");
    assert_eq!(allowed_cancel.status(), StatusCode::OK);
    let allowed_cancel_body = allowed_cancel
        .into_body()
        .collect()
        .await
        .expect("allowed cancel body should collect")
        .to_bytes();
    let allowed_cancel_json: serde_json::Value = serde_json::from_slice(&allowed_cancel_body)
        .expect("allowed cancel body should be valid json");
    assert_eq!(allowed_cancel_json["friendRequest"]["status"], "canceled");
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_sender_session_id_on_post_message() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_local_oversized_sender_session",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_client_route = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_phone"}"#))
                .unwrap(),
        )
        .await
        .expect("register device should succeed");
    assert_eq!(register_client_route.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_local_oversized_sender_session/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("x-sdkwork-session-id", "s".repeat(257))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_local_oversized_sender_session",
                        "summary":"oversized sender session",
                        "text":"hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("oversized sender session post should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("senderSessionId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_render_hints_on_post_message() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_local_oversized_render_hints",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_client_route = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_phone"}"#))
                .unwrap(),
        )
        .await
        .expect("register device should succeed");
    assert_eq!(register_client_route.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_local_oversized_render_hints/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "clientMsgId": "client_local_oversized_render_hints",
                        "summary": "oversized render hints",
                        "text": "hello",
                        "renderHints": {
                            "preview": "x".repeat(70 * 1024)
                        }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized render hints post should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "payload_too_large");
    assert!(
        value["message"]
            .as_str()
            .expect("message should be present")
            .contains("renderHints")
    );
}
