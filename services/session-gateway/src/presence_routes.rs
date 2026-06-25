use axum::extract::{Extension, State};
use axum::http::HeaderMap;
use axum::Json;
use im_app_context::AppContext;

use crate::client_route_state::ClientRouteStateSnapshot;
use crate::{resolve_request_app_context, resolve_requested_device_id, ApiError, AppState, PresenceHeartbeatRequest};
fn client_route_state_snapshot(
    state: &AppState,
    auth: &AppContext,
    requested_device_id: Option<&str>,
) -> Result<ClientRouteStateSnapshot, ApiError> {
    state.client_route_state_snapshot(auth, requested_device_id)
}

pub(crate) async fn get_presence_me(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::presence::PresenceSnapshotView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let sync_state = client_route_state_snapshot(&state, &auth, auth.device_id.as_deref())?;

    Ok(Json(state.presence_runtime.presence_snapshot(
        &auth,
        auth.device_id.clone(),
        sync_state.registered_route_keys,
    )?))
}

pub(crate) async fn heartbeat_presence(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PresenceHeartbeatRequest>,
) -> Result<Json<im_domain_core::presence::PresenceSnapshotView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "http", false)?;
    let sync_state = client_route_state_snapshot(&state, &auth, Some(device_id.as_str()))?;

    Ok(Json(state.presence_runtime.heartbeat(
        &auth,
        device_id,
        sync_state.latest_sync_seq,
        sync_state.registered_route_keys,
    )?))
}
