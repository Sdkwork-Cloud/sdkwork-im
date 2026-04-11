use std::sync::Arc;

use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::WebSocket;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use craw_chat_runtime_link::{
    LinkWebsocketMode, LinkWebsocketUpgradeHandoff, prepare_websocket_upgrade,
    supported_websocket_subprotocols,
};
use im_auth_context::AuthContext;

use crate::websocket::{RealtimeWebsocketMode, serve_realtime_websocket};
use crate::websocket_route;
use crate::{ApiError, AppState, RealtimeDeliveryRuntime};

pub(crate) struct RealtimeWebsocketUpgradeContext {
    auth: AuthContext,
    device_id: String,
    runtime: Arc<RealtimeDeliveryRuntime>,
}

pub(crate) async fn realtime_websocket(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Response, ApiError> {
    let context = websocket_route::prepare_realtime_websocket_route(&headers, &state)?;
    Ok(upgrade_realtime_websocket(
        ws,
        context.auth,
        context.device_id,
        context.runtime,
    ))
}

pub(crate) fn upgrade_realtime_websocket(
    ws: WebSocketUpgrade,
    auth: AuthContext,
    device_id: String,
    runtime: Arc<RealtimeDeliveryRuntime>,
) -> Response {
    let ws = ws.protocols(realtime_websocket_subprotocols());
    let upgrade = prepare_realtime_websocket_upgrade(
        ws.selected_protocol()
            .and_then(|selected| selected.to_str().ok()),
        auth,
        device_id,
        runtime,
    );
    ws.on_upgrade(move |socket| upgrade.execute(socket, serve_realtime_websocket_upgrade))
        .into_response()
}

pub(crate) fn realtime_websocket_subprotocols() -> [&'static str; 1] {
    supported_websocket_subprotocols()
}

#[cfg(test)]
pub(crate) fn select_realtime_websocket_mode(
    selected_protocol: Option<&str>,
) -> RealtimeWebsocketMode {
    map_runtime_link_websocket_mode(craw_chat_runtime_link::select_websocket_mode(
        selected_protocol,
    ))
}

fn map_runtime_link_websocket_mode(mode: LinkWebsocketMode) -> RealtimeWebsocketMode {
    match mode {
        LinkWebsocketMode::LegacyJson => RealtimeWebsocketMode::LegacyJson,
        LinkWebsocketMode::CcpJson => RealtimeWebsocketMode::CcpJson,
    }
}

pub(crate) fn prepare_realtime_websocket_upgrade(
    selected_protocol: Option<&str>,
    auth: AuthContext,
    device_id: String,
    runtime: Arc<RealtimeDeliveryRuntime>,
) -> LinkWebsocketUpgradeHandoff<RealtimeWebsocketUpgradeContext> {
    prepare_websocket_upgrade(
        selected_protocol,
        RealtimeWebsocketUpgradeContext {
            auth,
            device_id,
            runtime,
        },
    )
}

async fn serve_realtime_websocket_upgrade(
    socket: WebSocket,
    context: RealtimeWebsocketUpgradeContext,
    mode: LinkWebsocketMode,
) {
    serve_realtime_websocket(
        socket,
        context.auth,
        context.device_id,
        context.runtime,
        map_runtime_link_websocket_mode(mode),
    )
    .await;
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::sync::Arc;

    use craw_chat_runtime_link::LinkWebsocketMode;
    use im_auth_context::AuthContext;

    use super::{
        prepare_realtime_websocket_upgrade, realtime_websocket_subprotocols,
        select_realtime_websocket_mode,
    };
    use crate::{RealtimeDeliveryRuntime, RealtimeWebsocketMode};

    #[test]
    fn test_realtime_websocket_upgrade_uses_runtime_link_owner_contract() {
        assert_eq!(
            realtime_websocket_subprotocols(),
            [crate::CCP_WEBSOCKET_SUBPROTOCOL]
        );
        assert_eq!(
            select_realtime_websocket_mode(Some(crate::CCP_WEBSOCKET_SUBPROTOCOL)),
            RealtimeWebsocketMode::CcpJson
        );
        assert_eq!(
            select_realtime_websocket_mode(Some("legacy.json")),
            RealtimeWebsocketMode::LegacyJson
        );
        assert_eq!(
            select_realtime_websocket_mode(None),
            RealtimeWebsocketMode::LegacyJson
        );
    }

    #[test]
    fn test_realtime_websocket_upgrade_prepares_runtime_link_handoff_owner() {
        let handoff = prepare_realtime_websocket_upgrade(
            Some(crate::CCP_WEBSOCKET_SUBPROTOCOL),
            AuthContext {
                tenant_id: "t_demo".into(),
                actor_id: "u_demo".into(),
                actor_kind: "user".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_demo".into()),
                permissions: BTreeSet::new(),
            },
            "d_pad".into(),
            Arc::new(RealtimeDeliveryRuntime::default()),
        );

        assert_eq!(handoff.mode(), LinkWebsocketMode::CcpJson);
        assert_eq!(handoff.context().auth.tenant_id, "t_demo");
        assert_eq!(handoff.context().auth.actor_id, "u_demo");
        assert_eq!(handoff.context().device_id, "d_pad");
    }
}
