use super::device_registration::DisconnectActiveDeviceRouteOutcome;
use super::*;

fn device_sync_session_state(
    state: &AppState,
    auth: &AuthContext,
    requested_device_id: Option<&str>,
) -> Result<projection_service::DeviceSyncSessionState, ApiError> {
    Ok(state
        .projection_service
        .device_sync_session_state_from_auth_context(auth, requested_device_id)?)
}

pub(super) async fn resume_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<ResumeSessionRequest>,
) -> Result<Json<SessionResumeView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    state.bind_device_registration(&auth, device_id.as_str(), "http", true)?;
    let sync_state = device_sync_session_state(&state, &auth, Some(device_id.as_str()))?;

    Ok(Json(state.session_presence_runtime.resume(
        &auth,
        device_id,
        request.last_seen_sync_seq.unwrap_or_default(),
        sync_state.latest_sync_seq.unwrap_or_default(),
        sync_state.registered_devices,
    )?))
}

pub(super) async fn get_presence_me(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let sync_state = device_sync_session_state(&state, &auth, auth.device_id.as_deref())?;
    Ok(Json(state.session_presence_runtime.presence_snapshot(
        &auth,
        auth.device_id.clone(),
        sync_state.registered_devices,
    )?))
}

pub(super) async fn heartbeat_presence(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PresenceDeviceRequest>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "http")?;
    let sync_state = device_sync_session_state(&state, &auth, Some(device_id.as_str()))?;

    Ok(Json(state.session_presence_runtime.heartbeat(
        &auth,
        device_id.clone(),
        sync_state.latest_sync_seq.unwrap_or_default(),
        sync_state.registered_devices,
    )?))
}

pub(super) async fn disconnect_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PresenceDeviceRequest>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    let outcome = state.disconnect_active_device_route(&auth, device_id.as_str(), "http")?;
    let sync_state = device_sync_session_state(&state, &auth, Some(device_id.as_str()))?;

    match outcome {
        DisconnectActiveDeviceRouteOutcome::FenceMatchedSession => {
            Ok(Json(state.session_presence_runtime.presence_snapshot(
                &auth,
                Some(device_id),
                sync_state.registered_devices,
            )?))
        }
        DisconnectActiveDeviceRouteOutcome::DeviceDisconnected => {
            Ok(Json(state.session_presence_runtime.disconnect(
                &auth,
                device_id,
                sync_state.registered_devices,
            )?))
        }
    }
}

pub(super) async fn register_device(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RegisterDeviceRequest>,
) -> Result<Json<projection_service::RegisteredDeviceView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    Ok(Json(state.prepare_active_device_route(
        &auth,
        device_id.as_str(),
        "http",
    )?))
}

pub(super) async fn sync_realtime_subscriptions(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SyncRealtimeSubscriptionsRequest>,
) -> Result<Json<im_domain_core::realtime::RealtimeSubscriptionSnapshot>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "http")?;

    Ok(Json(
        state
            .realtime_runtime
            .sync_subscriptions_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id.as_str(),
                request.items,
            )?,
    ))
}

pub(super) async fn list_realtime_events(
    Query(query): Query<ListRealtimeEventsQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::realtime::RealtimeEventWindow>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = access::resolve_requested_device_id(&auth, None)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "http_poll")?;
    let limit = query.limit.unwrap_or(100);

    if limit == 0 {
        return Err(ApiError::bad_request(
            "limit_invalid",
            "limit must be greater than 0",
        ));
    }

    Ok(Json(
        state.realtime_runtime.list_events_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id.as_str(),
            query.after_seq.unwrap_or_default(),
            limit,
        )?,
    ))
}

pub(super) async fn ack_realtime_events(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AckRealtimeEventsRequest>,
) -> Result<Json<im_domain_core::realtime::RealtimeAckState>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "http")?;

    Ok(Json(state.realtime_runtime.ack_events_for_principal_kind(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
        device_id.as_str(),
        request.acked_seq,
    )?))
}

pub(super) async fn realtime_websocket(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<axum::response::Response, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let device_id = access::resolve_requested_device_id(&auth, None)?;
    state.prepare_active_device_route(&auth, device_id.as_str(), "websocket")?;
    let runtime = state.realtime_runtime.clone();
    let ws = ws.protocols([session_gateway::CCP_WEBSOCKET_SUBPROTOCOL]);
    let wire_mode = if ws.selected_protocol().is_some() {
        session_gateway::RealtimeWebsocketMode::CcpJson
    } else {
        session_gateway::RealtimeWebsocketMode::LegacyJson
    };

    Ok(ws
        .on_upgrade(move |socket| {
            serve_realtime_websocket(socket, auth, device_id, runtime, wire_mode)
        })
        .into_response())
}

pub(super) async fn get_device_sync_feed(
    Path(device_id): Path<String>,
    Query(query): Query<SyncFeedQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<DeviceSyncFeedResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;

    Ok(Json(DeviceSyncFeedResponse {
        items: state
            .projection_service
            .device_sync_feed_from_auth_context(&auth, device_id.as_str(), query.after_seq)?,
    }))
}
