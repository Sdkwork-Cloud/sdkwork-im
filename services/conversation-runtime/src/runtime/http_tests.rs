//! White-box unit tests for conversation-runtime HTTP layer.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "http_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;
use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue, Request, StatusCode};
use http_body_util::BodyExt;
use im_app_context::DualTokenRequestBuilderExt;
use std::collections::BTreeSet;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tower::ServiceExt;

#[derive(Clone)]
struct StrictKnownPrincipalDirectory {
    known_user_ids: Vec<&'static str>,
}

impl StrictKnownPrincipalDirectory {
    fn new(known_user_ids: &[&'static str]) -> Self {
        Self {
            known_user_ids: known_user_ids.to_vec(),
        }
    }
}

impl PrincipalDirectory for StrictKnownPrincipalDirectory {
    fn ensure_active_principal(
        &self,
        _tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<(), PrincipalDirectoryError> {
        if principal_kind != "user" {
            return Ok(());
        }
        if self.known_user_ids.contains(&principal_id) {
            return Ok(());
        }

        Err(PrincipalDirectoryError::PrincipalNotFound {
            tenant_id: "t_demo".into(),
            principal_id: principal_id.into(),
            principal_kind: principal_kind.into(),
        })
    }
}

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

    fn remove(name: &'static str) -> Self {
        let previous = std::env::var(name).ok();
        unsafe {
            std::env::remove_var(name);
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

fn rate_limit_env_guard<'a>() -> std::sync::MutexGuard<'a, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("env lock")
}

fn build_test_app_with_runtime_and_directory(
    runtime: Arc<ConversationRuntime<InMemoryJournal>>,
    principal_directory: Arc<dyn PrincipalDirectory>,
) -> Router {
    build_app(AppState {
        runtime,
        principal_directory,
        shared_channel_sync_rate_limiter: SharedChannelSyncRateLimiter::from_env(),
    })
}

fn seed_group_conversation_with_ghost_member(
    runtime: &ConversationRuntime<InMemoryJournal>,
    conversation_id: &str,
) -> String {
    let owner_auth = AppContext {
        tenant_id: "t_demo".into(),
        organization_id: None,
        user_id: "u_owner".into(),
        actor_id: "u_owner".into(),
        actor_kind: "user".into(),
        session_id: None,
        app_id: None,
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: BTreeSet::new(),
        permission_scope: BTreeSet::new(),
        device_id: None,
    };
    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: conversation_id.into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("seed create conversation should succeed");
    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: conversation_id.into(),
            principal_id: "u_missing".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("seed add ghost member should succeed");

    runtime
        .post_message(PostMessageCommand::from_auth_context(
            &owner_auth,
            conversation_id.into(),
            Some(format!("seed_{conversation_id}")),
            MessageType::Standard,
            build_message_body(
                Some("seed root".into()),
                Some("seed root".into()),
                None,
                Vec::new(),
                BTreeMap::new(),
            )
            .expect("seed message body should build"),
        ))
        .expect("seed root message should succeed")
        .message_id
}

#[test]
fn test_unix_epoch_millis_clamps_pre_epoch_time_to_zero() {
    let before_epoch = UNIX_EPOCH
        .checked_sub(Duration::from_millis(1))
        .expect("test pre-epoch timestamp should construct");
    assert_eq!(unix_epoch_millis(before_epoch), 0);
}

#[test]
fn test_unix_epoch_millis_preserves_post_epoch_time() {
    let after_epoch = UNIX_EPOCH + Duration::from_millis(1_234);
    assert_eq!(unix_epoch_millis(after_epoch), 1_234);
}

#[test]
fn test_shared_channel_sync_rate_limiter_clamps_env_values_to_safe_bounds() {
    let _guard = rate_limit_env_guard();
    let _max_requests =
        ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS_ENV, "999999");
    let _window_seconds =
        ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS_ENV, "999999");
    let _max_buckets = ScopedEnvVar::set(SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS_ENV, "999999");

    let limiter = SharedChannelSyncRateLimiter::from_env();
    assert_eq!(
        limiter.max_requests,
        SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_MAX_REQUESTS
    );
    assert_eq!(
        limiter.window_millis,
        (SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_WINDOW_SECONDS as u128) * 1000
    );
    assert_eq!(
        limiter.max_buckets,
        SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_ALLOWED_BUCKETS
    );
}

#[test]
fn test_shared_channel_sync_rate_limiter_rejects_new_tenant_when_bucket_cap_is_reached() {
    let limiter = SharedChannelSyncRateLimiter {
        max_requests: 2,
        window_millis: 60_000,
        max_buckets: 2,
        buckets: Arc::new(Mutex::new(BTreeMap::new())),
    };

    assert!(limiter.try_acquire("tenant_a"));
    assert!(limiter.try_acquire("tenant_b"));
    assert!(
        !limiter.try_acquire("tenant_c"),
        "new tenant should be rejected when rate-limit bucket cap is reached"
    );
    assert!(
        limiter.try_acquire("tenant_a"),
        "existing tenant should still be serviceable when cap is reached"
    );
}

#[test]
fn parse_truthy_env_flag_accepts_common_truthy_values() {
    for value in ["1", "true", "TRUE", " yes ", "On"] {
        assert!(parse_truthy_env_flag(Some(value.to_owned())));
    }
    for value in ["0", "false", "off", "no", "", "  "] {
        assert!(!parse_truthy_env_flag(Some(value.to_owned())));
    }
    assert!(!parse_truthy_env_flag(None));
}

#[test]
fn dual_token_header_helpers_validate_auth_and_access_headers() {
    let mut headers = HeaderMap::new();
    assert!(!has_bearer_auth_token(&headers));
    assert!(!has_access_token_header(&headers));

    headers.insert(
        axum::http::header::AUTHORIZATION,
        HeaderValue::from_static("Bearer token"),
    );
    assert!(has_bearer_auth_token(&headers));
    assert!(!has_access_token_header(&headers));
    let error = require_dual_token_headers(&headers).expect_err("access-token should be required");
    assert_eq!(error.status, StatusCode::UNAUTHORIZED);
    assert_eq!(error.code, "access_token_missing");

    headers.insert("access-token", HeaderValue::from_static("access"));
    assert!(has_access_token_header(&headers));
    require_dual_token_headers(&headers).expect("dual token headers should pass");
}

#[test]
fn dual_token_guardrail_defaults_to_app_context_projection() {
    let _guard = rate_limit_env_guard();
    let _env = ScopedEnvVar::remove(CONVERSATION_RUNTIME_REQUIRE_DUAL_TOKEN_HEADERS_ENV);

    assert!(
        !resolve_require_dual_token_headers(),
        "conversation runtime should default to SDKWork AppContext projection without legacy bearer/access-token headers"
    );
}

#[test]
fn test_shared_channel_sync_rate_limiter_prunes_expired_buckets_before_rejecting_new_tenant() {
    let limiter = SharedChannelSyncRateLimiter {
        max_requests: 1,
        window_millis: 1,
        max_buckets: 2,
        buckets: Arc::new(Mutex::new(BTreeMap::new())),
    };
    {
        let mut buckets = lock_shared_channel_rate_limit_mutex(
            &limiter.buckets,
            "shared-channel-sync-rate-limit",
        );
        buckets.insert(
            "tenant_expired_a".into(),
            SharedChannelSyncRateLimitBucket {
                window_started_at_millis: 0,
                request_count: 1,
            },
        );
        buckets.insert(
            "tenant_expired_b".into(),
            SharedChannelSyncRateLimitBucket {
                window_started_at_millis: 0,
                request_count: 1,
            },
        );
    }

    assert!(
        limiter.try_acquire("tenant_new"),
        "expired buckets should be swept before enforcing max bucket cap"
    );
}

#[test]
fn test_build_message_body_derives_summary_for_structured_message_when_missing() {
    let body = build_message_body(
        None,
        None,
        None,
        vec![ContentPart::Data(im_domain_core::message::DataPart {
            schema_ref: im_domain_core::message::SDKWORK_IM_MESSAGE_SCHEMA_LOCATION.into(),
            encoding: "application/json".into(),
            payload: serde_json::json!({
                "name": "The Bund",
                "latitude": 31.2400,
                "longitude": 121.4900
            })
            .to_string(),
        })],
        BTreeMap::new(),
    )
    .expect("rich message body should build");

    assert_eq!(body.summary.as_deref(), Some("Location: The Bund"));
}

#[test]
fn test_build_message_body_preserves_explicit_summary_over_derived_summary() {
    let body = build_message_body(
        Some("Pinned location".into()),
        Some("caption".into()),
        None,
        vec![ContentPart::Data(im_domain_core::message::DataPart {
            schema_ref: im_domain_core::message::SDKWORK_IM_MESSAGE_SCHEMA_LOCATION.into(),
            encoding: "application/json".into(),
            payload: serde_json::json!({
                "name": "West Lake",
                "latitude": 30.2528,
                "longitude": 120.1551
            })
            .to_string(),
        })],
        BTreeMap::new(),
    )
    .expect("rich message body should build");

    assert_eq!(body.summary.as_deref(), Some("Pinned location"));
}

#[tokio::test]
async fn test_post_message_rejects_unknown_user_member_with_strict_principal_directory() {
    let runtime = Arc::new(ConversationRuntime::new(InMemoryJournal::default()));
    seed_group_conversation_with_ghost_member(runtime.as_ref(), "c_ghost_post_http");
    let app = build_test_app_with_runtime_and_directory(
        runtime,
        Arc::new(StrictKnownPrincipalDirectory::new(&["u_owner"])),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_ghost_post_http/messages")
                .with_dual_token_context("t_demo", "u_missing", "user", None, ["*"])
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                            "clientMsgId":"ghost_http_post",
                            "summary":"ghost",
                            "text":"ghost"
                        }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("ghost member post request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_principal_not_found");
}

#[tokio::test]
async fn test_list_messages_rejects_unknown_user_member_with_strict_principal_directory() {
    let runtime = Arc::new(ConversationRuntime::new(InMemoryJournal::default()));
    seed_group_conversation_with_ghost_member(runtime.as_ref(), "c_ghost_history_http");
    let app = build_test_app_with_runtime_and_directory(
        runtime,
        Arc::new(StrictKnownPrincipalDirectory::new(&["u_owner"])),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/im/v3/api/chat/conversations/c_ghost_history_http/messages")
                .with_dual_token_context("t_demo", "u_missing", "user", None, ["*"])
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ghost member history request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(value["code"], "conversation_principal_not_found");
}
