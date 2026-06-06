use std::sync::Arc;

use axum::extract::Extension;
use axum::http::HeaderMap;
use im_app_context::AppContext;

use crate::device_registration::DeviceRouteRegistration;
use crate::{ApiError, AppState, RealtimeDeliveryRuntime, resolve_requested_device_id};

pub(crate) struct RealtimeWebsocketRouteContext {
    pub auth: AppContext,
    pub device_id: String,
    pub runtime: Arc<RealtimeDeliveryRuntime>,
    pub route_owner: DeviceRouteRegistration,
}

pub(crate) fn prepare_realtime_websocket_route(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
    state: &AppState,
) -> Result<RealtimeWebsocketRouteContext, ApiError> {
    let auth = crate::resolve_request_app_context(auth, headers)?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "websocket", false)?;
    Ok(RealtimeWebsocketRouteContext {
        auth,
        device_id,
        runtime: state.realtime_runtime.clone(),
        route_owner: state.device_registration.clone(),
    })
}
