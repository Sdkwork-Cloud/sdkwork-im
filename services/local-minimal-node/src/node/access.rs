use super::*;

const LOCAL_NODE_MAX_DEVICE_ID_BYTES: usize = 256;

pub(super) fn ensure_audit_read_access(auth: &AppContext) -> Result<(), ApiError> {
    if auth.has_permission("audit.read") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission: audit.read",
    ))
}

pub(super) fn ensure_audit_write_access(auth: &AppContext) -> Result<(), ApiError> {
    if auth.has_permission("audit.write") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission: audit.write",
    ))
}

pub(super) fn ensure_ops_read_access(auth: &AppContext) -> Result<(), ApiError> {
    if auth.has_permission("ops.read") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission: ops.read",
    ))
}

pub(super) fn ensure_client_route_key(state: &AppState, auth: &AppContext) -> Result<(), ApiError> {
    state.require_client_route_key_binding(auth)
}

fn ensure_conversation_not_archived(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
) -> Result<(), ApiError> {
    if state
        .projection_service
        .is_archived_direct_chat_conversation(auth.tenant_id.as_str(), conversation_id)
    {
        return Err(ApiError::forbidden(
            "conversation_archived",
            format!("direct chat conversation is archived: {conversation_id}"),
        ));
    }

    Ok(())
}

fn ensure_conversation_not_blocked(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
) -> Result<(), ApiError> {
    let Some(user_block) =
        direct_chat_access_block_for_conversation(state, auth.tenant_id.as_str(), conversation_id)
    else {
        return Ok(());
    };

    Err(ApiError::forbidden(
        "conversation_blocked",
        format!(
            "direct chat conversation is blocked by user block {}: {conversation_id}",
            user_block.block_id
        ),
    ))
}

pub(super) fn direct_chat_access_block_for_conversation(
    state: &AppState,
    tenant_id: &str,
    conversation_id: &str,
) -> Option<im_domain_core::social::UserBlock> {
    let direct_chat_id = state
        .projection_service
        .direct_chat_id_for_conversation(tenant_id, conversation_id)?;
    state
        .social_runtime
        .active_direct_chat_access_block(tenant_id, direct_chat_id.as_str())
}

pub(super) fn resolve_requested_device_id(
    auth: &AppContext,
    requested_device_id: Option<String>,
) -> Result<String, ApiError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            validate_device_id(requested.as_str())?;
            validate_device_id(bound.as_str())?;
            if requested != bound {
                return Err(ApiError::bad_request(
                    "device_id_mismatch",
                    format!("device id does not match auth context: {requested}"),
                ));
            }
            Ok(requested)
        }
        (Some(requested), None) => {
            validate_device_id(requested.as_str())?;
            Ok(requested)
        }
        (None, Some(bound)) => {
            validate_device_id(bound.as_str())?;
            Ok(bound)
        }
        (None, None) => Err(ApiError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

pub(super) fn resolve_active_auth_context(
    state: &AppState,
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, ApiError> {
    let auth = resolve_request_app_context(auth, headers)?;
    ensure_active_auth_principal(state, &auth)?;
    Ok(auth)
}

pub(super) fn ensure_active_auth_principal(
    state: &AppState,
    auth: &AppContext,
) -> Result<(), ApiError> {
    state.refresh_projection_state_from_runtime_dir()?;
    principal_profile::ensure_active_principal(
        state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
    )
}

pub(super) fn ensure_conversation_member(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
) -> Result<(), ApiError> {
    ensure_active_auth_principal(state, auth)?;
    ensure_domain_conversation_member(state, auth, conversation_id)?;
    ensure_conversation_not_archived(state, auth, conversation_id)?;
    ensure_conversation_not_blocked(state, auth, conversation_id)?;
    Ok(())
}

pub(super) fn ensure_conversation_read_access(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
) -> Result<(), ApiError> {
    ensure_active_auth_principal(state, auth)?;
    match ensure_domain_conversation_member(state, auth, conversation_id) {
        Ok(()) => {}
        Err(domain_error)
            if matches!(
                domain_error.code,
                "conversation_not_found"
                    | "conversation_member_not_found"
                    | "conversation_permission_denied"
            ) =>
        {
            state
                .projection_service
                .ensure_history_reader_from_auth_context(auth, conversation_id)
                .map_err(ApiError::from)?;
        }
        Err(domain_error) => return Err(domain_error),
    }
    ensure_conversation_not_archived(state, auth, conversation_id)?;
    ensure_conversation_not_blocked(state, auth, conversation_id)?;
    Ok(())
}

fn ensure_domain_conversation_member(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
) -> Result<(), ApiError> {
    state
        .conversation_runtime
        .require_active_member_from_auth_context(auth, conversation_id)?;
    Ok(())
}

pub(super) fn resolve_conversation_actor_auth_context(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
) -> Result<AppContext, ApiError> {
    ensure_active_auth_principal(state, auth)?;
    let actor_member = state
        .conversation_runtime
        .require_active_member_from_auth_context(auth, conversation_id)?;
    ensure_conversation_not_archived(state, auth, conversation_id)?;
    ensure_conversation_not_blocked(state, auth, conversation_id)?;
    let mut actor_auth = auth.clone();
    actor_auth.actor_kind = actor_member.principal_kind;
    Ok(actor_auth)
}

fn ensure_conversation_bound_write_access(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    capability: &str,
) -> Result<(), ApiError> {
    ensure_active_auth_principal(state, auth)?;
    state
        .conversation_runtime
        .ensure_conversation_bound_write_allowed_from_auth_context(
            auth,
            conversation_id,
            capability,
        )?;
    ensure_conversation_not_archived(state, auth, conversation_id)?;
    ensure_conversation_not_blocked(state, auth, conversation_id)?;
    Ok(())
}

pub(super) fn ensure_rtc_create_access(
    state: &AppState,
    auth: &AppContext,
    request: &CreateRtcSessionRequest,
) -> Result<(), ApiError> {
    let rtc_auth = rtc_app_context_from_auth(auth);
    match state
        .call_runtime
        .session(&rtc_auth, request.rtc_session_id.as_str())
    {
        Ok(session) => {
            if let Some(conversation_id) = session.conversation_id.as_deref() {
                ensure_conversation_bound_write_access(state, auth, conversation_id, "rtc.create")?;
            }
        }
        Err(error) if error.code() == "rtc_session_not_found" => {
            if let Some(conversation_id) = request.conversation_id.as_deref() {
                ensure_conversation_bound_write_access(state, auth, conversation_id, "rtc.create")?;
            }
        }
        Err(error) => return Err(error.into()),
    }

    Ok(())
}

pub(super) fn ensure_rtc_session_conversation_write_access(
    state: &AppState,
    auth: &AppContext,
    rtc_session_id: &str,
    capability: &str,
) -> Result<(), ApiError> {
    let rtc_auth = rtc_app_context_from_auth(auth);
    let session = state.call_runtime.session(&rtc_auth, rtc_session_id)?;
    if let Some(conversation_id) = session.conversation_id.as_deref() {
        ensure_conversation_bound_write_access(state, auth, conversation_id, capability)?;
    }

    Ok(())
}

pub(super) fn ensure_stream_open_access(
    state: &AppState,
    auth: &AppContext,
    request: &OpenStreamRequest,
) -> Result<(), ApiError> {
    reject_aiot_owned_stream_scope(request.scope_kind.as_str(), request.stream_type.as_str())?;

    match state
        .streaming_runtime
        .session(auth, request.stream_id.as_str())
    {
        Ok(session) => {
            reject_aiot_owned_stream_scope(
                session.scope_kind.as_str(),
                session.stream_type.as_str(),
            )?;
            if session.scope_kind == "conversation" {
                ensure_conversation_bound_write_access(
                    state,
                    auth,
                    session.scope_id.as_str(),
                    "stream.open",
                )?;
            }
        }
        Err(error) if error.code() == "stream_not_found" => {
            if request.scope_kind == "conversation" {
                ensure_conversation_bound_write_access(
                    state,
                    auth,
                    request.scope_id.as_str(),
                    "stream.open",
                )?;
            }
        }
        Err(error) => return Err(error.into()),
    }

    Ok(())
}

pub(super) fn ensure_stream_session_conversation_member(
    state: &AppState,
    auth: &AppContext,
    stream_id: &str,
) -> Result<(), ApiError> {
    let session = state.streaming_runtime.session(auth, stream_id)?;
    reject_aiot_owned_stream_scope(session.scope_kind.as_str(), session.stream_type.as_str())?;
    if session.scope_kind == "conversation" {
        ensure_conversation_member(state, auth, session.scope_id.as_str())?;
    }

    Ok(())
}

pub(super) fn ensure_stream_session_write_access(
    state: &AppState,
    auth: &AppContext,
    stream_id: &str,
    capability: &str,
) -> Result<(), ApiError> {
    let session = state.streaming_runtime.session(auth, stream_id)?;
    reject_aiot_owned_stream_scope(session.scope_kind.as_str(), session.stream_type.as_str())?;
    if session.scope_kind == "conversation" {
        ensure_conversation_bound_write_access(state, auth, session.scope_id.as_str(), capability)?;
    }

    Ok(())
}

fn reject_aiot_owned_stream_scope(scope_kind: &str, stream_type: &str) -> Result<(), ApiError> {
    if scope_kind == "device" || stream_type.starts_with("device.") {
        return Err(ApiError::bad_request(
            "aiot_stream_scope_unsupported",
            "AIoT stream scopes are owned by sdkwork-aiot",
        ));
    }

    Ok(())
}

fn validate_device_id(device_id: &str) -> Result<(), ApiError> {
    let actual_bytes = device_id.len();
    if actual_bytes > LOCAL_NODE_MAX_DEVICE_ID_BYTES {
        return Err(ApiError::payload_too_large(
            "deviceId",
            LOCAL_NODE_MAX_DEVICE_ID_BYTES,
            actual_bytes,
        ));
    }
    Ok(())
}
