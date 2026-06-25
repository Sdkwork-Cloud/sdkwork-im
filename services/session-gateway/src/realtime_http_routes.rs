use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::{Extension, Json};
use im_app_context::AppContext;
use im_domain_core::realtime::{
    RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot,
};

use crate::api_error::ApiError;
use crate::realtime::{self, AckRealtimeEventsRequest, ListRealtimeEventsQuery, SyncRealtimeSubscriptionsRequest};
use crate::{resolve_request_app_context, resolve_requested_device_id, AppState};

pub async fn sync_realtime_subscriptions(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<SyncRealtimeSubscriptionsRequest>,
) -> Result<Json<RealtimeSubscriptionSnapshot>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    state
        .realtime_runtime
        .validate_subscriptions_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            &request.items,
        )?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "http", false)?;
    Ok(Json(
        state
            .realtime_runtime
            .sync_subscriptions_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.organization_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id.as_str(),
                request.items,
            )?,
    ))
}

pub async fn list_realtime_events(
    Query(query): Query<ListRealtimeEventsQuery>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RealtimeEventWindow>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let device_id = resolve_requested_device_id(&auth, None)?;
    let limit = query.limit.unwrap_or(100);
    realtime::validate_realtime_event_limit(limit)?;
    state.prepare_active_client_route(&auth, device_id.as_str(), "http_poll", false)?;
    Ok(Json(
        state.realtime_runtime.list_events_for_principal_kind(
            auth.tenant_id.as_str(),
            auth.organization_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id.as_str(),
            query.after_seq.unwrap_or_default(),
            limit,
        )?,
    ))
}

pub async fn ack_realtime_events(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AckRealtimeEventsRequest>,
) -> Result<Json<RealtimeAckState>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers, &state.auth_resolver).await?;
    let device_id = resolve_requested_device_id(&auth, request.device_id)?;
    let previous_route = state.current_active_client_route(&auth, device_id.as_str());
    state.prepare_active_client_route(&auth, device_id.as_str(), "http", false)?;
    let bound_route = state.current_active_client_route(&auth, device_id.as_str());
    let ack = state.realtime_runtime.ack_events_for_principal_kind(
        auth.tenant_id.as_str(),
        auth.organization_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
        device_id.as_str(),
        request.acked_seq,
    );
    match ack {
        Ok(ack) => Ok(Json(ack)),
        Err(error) => {
            match (previous_route, bound_route) {
                (Some(previous_route), Some(bound_route)) => {
                    state.restore_active_client_route_if_current(&bound_route, previous_route);
                }
                (None, _) => {
                    state.release_active_client_route_if_current_session(&auth, device_id.as_str());
                }
                _ => {}
            }
            Err(error.into())
        }
    }
}
