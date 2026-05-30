use super::device_registration::DisconnectActiveDeviceRouteOutcome;
use super::device_sync_state::DeviceSyncStateSnapshot;
use super::*;

fn device_sync_state_snapshot(
    state: &AppState,
    auth: &AppContext,
    requested_device_id: Option<&str>,
) -> Result<DeviceSyncStateSnapshot, ApiError> {
    state.device_sync_state_snapshot(auth, requested_device_id)
}

pub(crate) async fn resume_device_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ResumeDeviceSessionRequest>,
) -> Result<Json<im_domain_core::device_session::DeviceSessionResumeView>, ApiError> {
    let auth = resolve_app_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state.register_device(&auth, device_id.as_str(), "http", true)?;
    let sync_state = device_sync_state_snapshot(&state, &auth, Some(device_id.as_str()))?;

    Ok(Json(state.presence_runtime.resume(
        &auth,
        device_id,
        request.last_seen_sync_seq.unwrap_or_default(),
        sync_state.latest_sync_seq,
        sync_state.registered_devices,
    )?))
}

pub(crate) async fn get_presence_me(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::device_session::PresenceSnapshotView>, ApiError> {
    let auth = resolve_app_context(&headers)?;
    let sync_state = device_sync_state_snapshot(&state, &auth, auth.device_id.as_deref())?;

    Ok(Json(state.presence_runtime.presence_snapshot(
        &auth,
        auth.device_id.clone(),
        sync_state.registered_devices,
    )?))
}

pub(crate) async fn heartbeat_presence(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<DevicePresenceRequest>,
) -> Result<Json<im_domain_core::device_session::PresenceSnapshotView>, ApiError> {
    let auth = resolve_app_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "http", false)?;
    let sync_state = device_sync_state_snapshot(&state, &auth, Some(device_id.as_str()))?;

    Ok(Json(state.presence_runtime.heartbeat(
        &auth,
        device_id,
        sync_state.latest_sync_seq,
        sync_state.registered_devices,
    )?))
}

pub(crate) async fn disconnect_device_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<DevicePresenceRequest>,
) -> Result<Json<im_domain_core::device_session::PresenceSnapshotView>, ApiError> {
    let auth = resolve_app_context(&headers)?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    let outcome = state.disconnect_active_device_route(&auth, device_id.as_str(), "http")?;
    let sync_state = device_sync_state_snapshot(&state, &auth, Some(device_id.as_str()))?;

    match outcome {
        DisconnectActiveDeviceRouteOutcome::FenceMatchedDeviceSession => {
            Ok(Json(state.presence_runtime.presence_snapshot(
                &auth,
                Some(device_id),
                sync_state.registered_devices,
            )?))
        }
        DisconnectActiveDeviceRouteOutcome::DeviceDisconnected => Ok(Json(
            state
                .presence_runtime
                .disconnect(&auth, device_id, sync_state.registered_devices)?,
        )),
    }
}
