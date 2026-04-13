use im_domain_events::CommitEnvelope;
use im_time::utc_now_rfc3339_millis;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct DevicePrincipalScopeKey {
    pub(super) tenant_id: String,
    pub(super) principal_id: String,
    pub(super) principal_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct DeviceFeedScopeKey {
    pub(super) tenant_id: String,
    pub(super) principal_id: String,
    pub(super) principal_kind: Option<String>,
    pub(super) device_id: String,
}

pub(super) fn scope_key(tenant_id: &str, conversation_id: &str) -> String {
    format!("{tenant_id}:{conversation_id}")
}

pub(super) fn principal_scope_key(tenant_id: &str, principal_id: &str) -> String {
    format!("{tenant_id}:{principal_id}")
}

pub(super) fn device_principal_scope_key(
    tenant_id: &str,
    principal_id: &str,
    principal_kind: Option<&str>,
) -> DevicePrincipalScopeKey {
    DevicePrincipalScopeKey {
        tenant_id: tenant_id.into(),
        principal_id: principal_id.into(),
        principal_kind: principal_kind.map(str::to_owned),
    }
}

pub(super) fn device_feed_scope_key(
    tenant_id: &str,
    principal_id: &str,
    principal_kind: Option<&str>,
    device_id: &str,
) -> DeviceFeedScopeKey {
    DeviceFeedScopeKey {
        tenant_id: tenant_id.into(),
        principal_id: principal_id.into(),
        principal_kind: principal_kind.map(str::to_owned),
        device_id: device_id.into(),
    }
}

pub(super) fn registered_device_at() -> String {
    utc_now_rfc3339_millis()
}

pub(super) fn tracked_live_projection_lag_scope_id(event: &CommitEnvelope) -> Option<String> {
    if event.scope_type != "conversation" {
        return None;
    }

    if matches!(
        event.event_type.as_str(),
        "conversation.created"
            | "conversation.agent_handoff_status_changed"
            | "message.posted"
            | "message.edited"
            | "message.recalled"
            | "message.reaction_added"
            | "message.reaction_removed"
            | "message.pin_added"
            | "message.pin_removed"
            | "conversation.member_joined"
            | "conversation.member_role_changed"
            | "conversation.member_removed"
            | "conversation.member_left"
            | "conversation.read_cursor_updated"
    ) {
        Some(scope_key(event.tenant_id.as_str(), event.scope_id.as_str()))
    } else {
        None
    }
}
