pub(crate) fn typed_principal_scope_key(
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> String {
    scope_key(&[tenant_id, principal_kind, principal_id])
}

pub(crate) fn typed_client_route_scope_key(
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> String {
    scope_key(&[tenant_id, principal_kind, principal_id, device_id])
}

pub(crate) fn tenant_client_route_scope_key(tenant_id: &str, device_id: &str) -> String {
    scope_key(&[tenant_id, device_id])
}

fn scope_key(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

#[cfg(test)]
mod tests {
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
}
