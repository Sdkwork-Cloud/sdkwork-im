pub(crate) fn typed_principal_id(principal_id: &str, principal_kind: &str) -> String {
    format!("{principal_kind}:{principal_id}")
}

pub(crate) fn typed_principal_scope_key(
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> String {
    format!(
        "{tenant_id}:{}",
        typed_principal_id(principal_id, principal_kind)
    )
}

pub(crate) fn typed_device_scope_key(
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> String {
    format!(
        "{tenant_id}:{}:{device_id}",
        typed_principal_id(principal_id, principal_kind)
    )
}

pub(crate) fn actor_device_scope_key(
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
) -> String {
    format!("{tenant_id}:{principal_id}:{device_id}")
}

pub(crate) fn tenant_device_scope_key(tenant_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{device_id}")
}
