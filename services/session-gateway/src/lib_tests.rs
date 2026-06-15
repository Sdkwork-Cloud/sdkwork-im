//! White-box unit tests for session-gateway app state and guardrails.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "lib_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use std::time::Duration;

use axum::http::{HeaderMap, HeaderValue};

use super::{
    WebSocketUpgradeRateLimiter, has_access_token_header, has_bearer_auth_token,
    parse_truthy_env_flag,
};

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
        HeaderValue::from_static("Bearer auth_token"),
    );
    headers.insert("access-token", HeaderValue::from_static("access_token"));
    assert!(has_bearer_auth_token(&headers));
    assert!(has_access_token_header(&headers));
}

#[test]
fn websocket_upgrade_rate_limiter_admits_until_limit_then_rejects() {
    // A 1s window keeps the test fast while still exercising rollover logic.
    let limiter = WebSocketUpgradeRateLimiter::new(3, Duration::from_secs(1));

    assert!(limiter.try_acquire("ip:10.0.0.1").is_ok());
    assert!(limiter.try_acquire("ip:10.0.0.1").is_ok());
    assert!(limiter.try_acquire("ip:10.0.0.1").is_ok());
    // 4th upgrade from the same key within the window is rejected.
    let rejected = limiter.try_acquire("ip:10.0.0.1");
    assert!(
        matches!(rejected, Err(ms) if ms >= 1),
        "over-limit upgrade must return a retry hint, got {rejected:?}"
    );

    // A different key is unaffected — the limit is per-key, not global.
    assert!(
        limiter.try_acquire("ip:10.0.0.2").is_ok(),
        "a distinct key must not inherit another key's budget"
    );
}

#[test]
fn websocket_upgrade_rate_limiter_key_is_independent_across_distinct_keys() {
    let limiter = WebSocketUpgradeRateLimiter::new(2, Duration::from_secs(60));
    assert!(limiter.try_acquire("tenant:t_a").is_ok());
    assert!(limiter.try_acquire("tenant:t_a").is_ok());
    // t_a is saturated, but t_b still has its own budget.
    assert!(limiter.try_acquire("tenant:t_b").is_ok());
    assert!(limiter.try_acquire("tenant:t_b").is_ok());
    assert!(limiter.try_acquire("tenant:t_b").is_err());
}
