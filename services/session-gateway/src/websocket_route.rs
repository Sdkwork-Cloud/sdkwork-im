use std::sync::Arc;

use axum::http::HeaderMap;
use im_auth_context::{AuthContext, resolve_auth_context};

use crate::{ApiError, AppState, RealtimeDeliveryRuntime, resolve_requested_device_id};

pub(crate) struct RealtimeWebsocketRouteContext {
    pub auth: AuthContext,
    pub device_id: String,
    pub runtime: Arc<RealtimeDeliveryRuntime>,
}

pub(crate) fn prepare_realtime_websocket_route(
    headers: &HeaderMap,
    state: &AppState,
) -> Result<RealtimeWebsocketRouteContext, ApiError> {
    let auth = resolve_auth_context(headers)?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    state.prepare_active_device_route(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        device_id.as_str(),
        auth.session_id.as_deref(),
        "websocket",
        false,
    )?;
    Ok(RealtimeWebsocketRouteContext {
        auth,
        device_id,
        runtime: state.realtime_runtime.clone(),
    })
}
