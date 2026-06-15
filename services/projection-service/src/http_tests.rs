//! White-box unit tests for projection HTTP layer.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "http_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use axum::http::{HeaderMap, HeaderValue};

use super::*;

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
