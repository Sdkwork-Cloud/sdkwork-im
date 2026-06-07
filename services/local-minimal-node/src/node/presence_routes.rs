use super::*;

fn client_route_state_snapshot(
    state: &AppState,
    auth: &AppContext,
    requested_device_id: Option<&str>,
) -> Result<projection_service::ClientRouteSyncStateSnapshot, ApiError> {
    Ok(state
        .projection_service
        .client_route_sync_state_snapshot_from_auth_context(auth, requested_device_id)?)
}

pub(super) async fn get_presence_me(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let sync_state = client_route_state_snapshot(&state, &auth, auth.device_id.as_deref())?;
    Ok(Json(state.presence_runtime.presence_snapshot(
        &auth,
        auth.device_id.clone(),
        sync_state.registered_client_routes,
    )?))
}

pub(super) async fn heartbeat_presence(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<PresenceHeartbeatRequest>,
) -> Result<Json<PresenceSnapshotView>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "http")?;
    let sync_state = client_route_state_snapshot(&state, &auth, Some(device_id.as_str()))?;

    Ok(Json(state.presence_runtime.heartbeat(
        &auth,
        device_id.clone(),
        sync_state.latest_sync_seq.unwrap_or_default(),
        sync_state.registered_client_routes,
    )?))
}

pub(super) async fn sync_realtime_subscriptions(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<SyncRealtimeSubscriptionsRequest>,
) -> Result<Json<im_domain_core::realtime::RealtimeSubscriptionSnapshot>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "http")?;

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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<im_domain_core::realtime::RealtimeEventWindow>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let device_id = access::resolve_requested_device_id(&auth, None)?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "http_poll")?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<AckRealtimeEventsRequest>,
) -> Result<Json<im_domain_core::realtime::RealtimeAckState>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let device_id = access::resolve_requested_device_id(&auth, request.device_id)?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "http")?;

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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<axum::response::Response, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let device_id = access::resolve_requested_device_id(&auth, None)?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "websocket")?;
    let runtime = state.realtime_runtime.clone();
    let route_owner = state.client_route_registration.clone();
    let ws = ws.protocols([session_gateway::CCP_WEBSOCKET_SUBPROTOCOL]);
    let wire_mode = if ws.selected_protocol().is_some() {
        session_gateway::RealtimeWebsocketMode::CcpJson
    } else {
        session_gateway::RealtimeWebsocketMode::LegacyJson
    };

    Ok(ws
        .on_upgrade(move |socket| {
            let cleanup_auth = auth.clone();
            let cleanup_device_id = device_id.clone();
            let cleanup_route_owner = route_owner.clone();
            async move {
                serve_realtime_websocket(socket, auth, device_id, runtime, route_owner, wire_mode)
                    .await;
                session_gateway::RealtimeRouteOwner::release_active_client_route_if_current_session(
                    &cleanup_route_owner,
                    &cleanup_auth,
                    cleanup_device_id.as_str(),
                );
            }
        })
        .into_response())
}
