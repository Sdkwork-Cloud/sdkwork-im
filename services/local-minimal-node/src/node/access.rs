use super::*;

const DEVICE_SCOPE_KIND: &str = "device";
const DEVICE_TELEMETRY_STREAM_TYPE: &str = "device.telemetry";
const DEVICE_COMMAND_STREAM_TYPE: &str = "device.command";

pub(super) fn ensure_audit_read_access(auth: &AuthContext) -> Result<(), ApiError> {
    if auth.has_permission("audit.read") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission: audit.read",
    ))
}

pub(super) fn ensure_audit_write_access(auth: &AuthContext) -> Result<(), ApiError> {
    if auth.has_permission("audit.write") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission: audit.write",
    ))
}

pub(super) fn ensure_ops_read_access(auth: &AuthContext) -> Result<(), ApiError> {
    if auth.has_permission("ops.read") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "permission_denied",
        "missing required permission: ops.read",
    ))
}

pub(super) fn ensure_registered_device(
    state: &AppState,
    auth: &AuthContext,
) -> Result<(), ApiError> {
    state.require_registered_device_binding(auth)
}

pub(super) fn resolve_requested_device_id(
    auth: &AuthContext,
    requested_device_id: Option<String>,
) -> Result<String, ApiError> {
    match (requested_device_id, auth.device_id.clone()) {
        (Some(requested), Some(bound)) => {
            if requested != bound {
                return Err(ApiError::bad_request(
                    "device_id_mismatch",
                    format!("device id does not match auth context: {requested}"),
                ));
            }
            Ok(requested)
        }
        (Some(requested), None) => Ok(requested),
        (None, Some(bound)) => Ok(bound),
        (None, None) => Err(ApiError::bad_request(
            "device_id_missing",
            "device id must be provided by auth context or request body",
        )),
    }
}

pub(super) fn ensure_conversation_member(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
) -> Result<(), ApiError> {
    state
        .conversation_runtime
        .require_active_member_from_auth_context(auth, conversation_id)?;
    Ok(())
}

pub(super) fn resolve_conversation_actor_auth_context(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
) -> Result<AuthContext, ApiError> {
    let actor_member = state
        .conversation_runtime
        .require_active_member_from_auth_context(auth, conversation_id)?;
    let mut actor_auth = auth.clone();
    actor_auth.actor_kind = actor_member.principal_kind;
    Ok(actor_auth)
}

fn ensure_conversation_bound_write_access(
    state: &AppState,
    auth: &AuthContext,
    conversation_id: &str,
    capability: &str,
) -> Result<(), ApiError> {
    state
        .conversation_runtime
        .ensure_conversation_bound_write_allowed_from_auth_context(
            auth,
            conversation_id,
            capability,
        )?;
    Ok(())
}

pub(super) fn ensure_rtc_create_access(
    state: &AppState,
    auth: &AuthContext,
    request: &CreateRtcSessionRequest,
) -> Result<(), ApiError> {
    match state
        .rtc_runtime
        .session(auth, request.rtc_session_id.as_str())
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
    auth: &AuthContext,
    rtc_session_id: &str,
    capability: &str,
) -> Result<(), ApiError> {
    let session = state.rtc_runtime.session(auth, rtc_session_id)?;
    if let Some(conversation_id) = session.conversation_id.as_deref() {
        ensure_conversation_bound_write_access(state, auth, conversation_id, capability)?;
    }

    Ok(())
}

pub(super) fn ensure_stream_open_access(
    state: &AppState,
    auth: &AuthContext,
    request: &OpenStreamRequest,
) -> Result<(), ApiError> {
    ensure_device_stream_shape(request)?;

    match state
        .streaming_runtime
        .session(auth, request.stream_id.as_str())
    {
        Ok(session) => {
            if session.scope_kind == "conversation" {
                ensure_conversation_bound_write_access(
                    state,
                    auth,
                    session.scope_id.as_str(),
                    "stream.open",
                )?;
            } else if session.scope_kind == DEVICE_SCOPE_KIND {
                ensure_device_stream_write_access(state, auth, &session)?;
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
            } else if request.scope_kind == DEVICE_SCOPE_KIND {
                ensure_device_stream_open_access(state, auth, request)?;
            }
        }
        Err(error) => return Err(error.into()),
    }

    Ok(())
}

pub(super) fn ensure_stream_session_conversation_member(
    state: &AppState,
    auth: &AuthContext,
    stream_id: &str,
) -> Result<(), ApiError> {
    let session = state.streaming_runtime.session(auth, stream_id)?;
    if session.scope_kind == "conversation" {
        ensure_conversation_member(state, auth, session.scope_id.as_str())?;
    } else if session.scope_kind == DEVICE_SCOPE_KIND {
        ensure_device_stream_read_access(state, auth, &session)?;
    }

    Ok(())
}

pub(super) fn ensure_stream_session_write_access(
    state: &AppState,
    auth: &AuthContext,
    stream_id: &str,
    capability: &str,
) -> Result<(), ApiError> {
    let session = state.streaming_runtime.session(auth, stream_id)?;
    if session.scope_kind == "conversation" {
        ensure_conversation_bound_write_access(state, auth, session.scope_id.as_str(), capability)?;
    } else if session.scope_kind == DEVICE_SCOPE_KIND {
        ensure_device_stream_write_access(state, auth, &session)?;
    }

    Ok(())
}

pub(super) fn ensure_iot_protocol_uplink_access(
    state: &AppState,
    auth: &AuthContext,
    device_id: &str,
) -> Result<(), ApiError> {
    ensure_device_stream_registration(state, auth, device_id)?;
    ensure_bound_device_actor(auth, device_id)
}

pub(super) fn ensure_iot_protocol_downlink_access(
    state: &AppState,
    auth: &AuthContext,
    device_id: &str,
) -> Result<(), ApiError> {
    ensure_device_stream_registration(state, auth, device_id)?;
    if auth.has_permission("device.command.send") {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "device_permission_denied",
        "missing required permission: device.command.send",
    ))
}

pub(super) fn ensure_iot_protocol_uplink_actor_preflight(
    auth: &AuthContext,
) -> Result<(), ApiError> {
    if auth.actor_kind != "device" {
        return Err(ApiError::forbidden(
            "device_permission_denied",
            "device telemetry writes must come from a bound device actor",
        ));
    }

    if auth.device_id.is_none() {
        return Err(ApiError::bad_request(
            "device_id_missing",
            "device stream access requires an auth context device id",
        ));
    }

    Ok(())
}

pub(super) fn ensure_iot_protocol_uplink_decoded_device_matches_preflight(
    expected_device_id: &str,
    decoded_device_id: &str,
) -> Result<(), ApiError> {
    if expected_device_id == decoded_device_id {
        return Ok(());
    }

    Err(ApiError::bad_request(
        "device_id_mismatch",
        format!("device id does not match auth context: {decoded_device_id}"),
    ))
}

pub(super) fn ensure_device_twin_read_access(
    state: &AppState,
    auth: &AuthContext,
    device_id: &str,
) -> Result<(), ApiError> {
    ensure_device_stream_registration(state, auth, device_id)
}

pub(super) fn ensure_device_twin_desired_write_access(
    state: &AppState,
    auth: &AuthContext,
    device_id: &str,
) -> Result<(), ApiError> {
    ensure_device_stream_registration(state, auth, device_id)?;

    if auth.actor_kind == "device" {
        return Err(ApiError::forbidden(
            "device_permission_denied",
            "device actor cannot write desired twin state",
        ));
    }

    Ok(())
}

pub(super) fn ensure_device_twin_reported_write_access(
    state: &AppState,
    auth: &AuthContext,
    device_id: &str,
) -> Result<(), ApiError> {
    ensure_device_stream_registration(state, auth, device_id)?;
    ensure_bound_device_actor(auth, device_id)
}

fn ensure_device_stream_shape(request: &OpenStreamRequest) -> Result<(), ApiError> {
    let is_device_stream = matches!(
        request.stream_type.as_str(),
        DEVICE_TELEMETRY_STREAM_TYPE | DEVICE_COMMAND_STREAM_TYPE
    );

    if is_device_stream && request.scope_kind != DEVICE_SCOPE_KIND {
        return Err(ApiError::bad_request(
            "device_scope_invalid",
            "device streams must use device scope",
        ));
    }

    if request.scope_kind == DEVICE_SCOPE_KIND && !is_device_stream {
        return Err(ApiError::bad_request(
            "device_stream_type_invalid",
            "device scope currently supports only device.telemetry and device.command streams",
        ));
    }

    Ok(())
}

fn ensure_device_stream_open_access(
    state: &AppState,
    auth: &AuthContext,
    request: &OpenStreamRequest,
) -> Result<(), ApiError> {
    ensure_device_stream_registration(state, auth, request.scope_id.as_str())?;
    ensure_device_stream_permission(
        auth,
        request.scope_id.as_str(),
        request.stream_type.as_str(),
        true,
    )
}

fn ensure_device_stream_read_access(
    state: &AppState,
    auth: &AuthContext,
    session: &im_domain_core::stream::StreamSession,
) -> Result<(), ApiError> {
    ensure_device_stream_registration(state, auth, session.scope_id.as_str())?;
    ensure_device_stream_permission(
        auth,
        session.scope_id.as_str(),
        session.stream_type.as_str(),
        false,
    )
}

fn ensure_device_stream_write_access(
    state: &AppState,
    auth: &AuthContext,
    session: &im_domain_core::stream::StreamSession,
) -> Result<(), ApiError> {
    ensure_device_stream_registration(state, auth, session.scope_id.as_str())?;
    ensure_device_stream_permission(
        auth,
        session.scope_id.as_str(),
        session.stream_type.as_str(),
        true,
    )
}

fn ensure_device_stream_registration(
    state: &AppState,
    auth: &AuthContext,
    device_id: &str,
) -> Result<(), ApiError> {
    if auth.device_id.as_deref() == Some(device_id) {
        state.require_registered_device_binding(auth)?;
    }

    let owns_device = state
        .projection_service
        .registered_devices_from_auth_context(auth)
        .into_iter()
        .any(|item| item.device_id == device_id);

    if owns_device {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "device_permission_denied",
        format!("device is not registered for principal: {device_id}"),
    ))
}

fn ensure_device_stream_permission(
    auth: &AuthContext,
    device_id: &str,
    stream_type: &str,
    write: bool,
) -> Result<(), ApiError> {
    match stream_type {
        DEVICE_TELEMETRY_STREAM_TYPE => {
            if write {
                ensure_bound_device_actor(auth, device_id)?;
                return Ok(());
            }

            if auth.actor_kind == "device" {
                ensure_bound_device_actor(auth, device_id)?;
                return Ok(());
            }

            if auth.has_permission("device.telemetry.read") {
                return Ok(());
            }

            Err(ApiError::forbidden(
                "device_permission_denied",
                "missing required permission: device.telemetry.read",
            ))
        }
        DEVICE_COMMAND_STREAM_TYPE => {
            if auth.actor_kind == "device" {
                ensure_bound_device_actor(auth, device_id)?;
                return Ok(());
            }

            if auth.has_permission("device.command.send") {
                return Ok(());
            }

            Err(ApiError::forbidden(
                "device_permission_denied",
                "missing required permission: device.command.send",
            ))
        }
        _ => Err(ApiError::bad_request(
            "device_stream_type_invalid",
            format!("unsupported device stream type: {stream_type}"),
        )),
    }
}

fn ensure_bound_device_actor(auth: &AuthContext, device_id: &str) -> Result<(), ApiError> {
    if auth.actor_kind != "device" {
        return Err(ApiError::forbidden(
            "device_permission_denied",
            "device telemetry writes must come from a bound device actor",
        ));
    }

    match auth.device_id.as_deref() {
        Some(bound_device_id) if bound_device_id == device_id => Ok(()),
        Some(bound_device_id) => Err(ApiError::bad_request(
            "device_id_mismatch",
            format!(
                "device stream scope does not match auth context device: expected {bound_device_id}, got {device_id}"
            ),
        )),
        None => Err(ApiError::bad_request(
            "device_id_missing",
            "device stream access requires an auth context device id",
        )),
    }
}
