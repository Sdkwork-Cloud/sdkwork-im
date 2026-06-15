//! White-box unit tests for session-gateway principal scope keys.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "principal_scope_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;

#[test]
fn test_principal_scope_keys_are_not_delimiter_collision_prone() {
    assert_ne!(
        typed_principal_scope_key("t:a", "b", "c"),
        typed_principal_scope_key("t", "a:b", "c"),
        "typed principal scope keys must encode tenant/principal boundaries unambiguously"
    );
    assert_ne!(
        typed_client_route_scope_key("t", "u:d", "user", "1"),
        typed_client_route_scope_key("t", "u", "user", "d:1"),
        "typed device scope keys must encode principal/device boundaries unambiguously"
    );
    assert_ne!(
        tenant_client_route_scope_key("t:d", "1"),
        tenant_client_route_scope_key("t", "d:1"),
        "tenant device scope keys must encode tenant/device boundaries unambiguously"
    );
}
