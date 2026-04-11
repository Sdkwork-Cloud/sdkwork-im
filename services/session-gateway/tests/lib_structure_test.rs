#[test]
fn test_session_gateway_lib_rs_stays_below_step02_redline() {
    let line_count = include_str!("../src/lib.rs").lines().count();

    assert!(
        line_count <= 1000,
        "services/session-gateway/src/lib.rs must stay below 1000 lines for Step 02, found {line_count}"
    );
}

#[test]
fn test_session_gateway_cluster_disconnect_surface_moves_out_of_cluster_impl() {
    let cluster_source = include_str!("../src/cluster.rs");

    for forbidden_symbol in [
        "struct RealtimeDisconnectFence {",
        "struct ClusterMemoryDisconnectFenceStore {",
        "pub fn mark_device_disconnected(",
        "pub fn clear_device_disconnect_fence(",
        "pub fn ensure_device_resume_not_required(",
        "pub fn disconnect_fence_matches_session(",
        "fn load_disconnect_fence(",
        "fn disconnect_fence_store_error(",
    ] {
        assert!(
            !cluster_source.contains(forbidden_symbol),
            "services/session-gateway/src/cluster.rs should not keep disconnect-fence symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_realtime_storage_surface_moves_out_of_realtime_impl() {
    let realtime_source = include_str!("../src/realtime.rs");

    for forbidden_symbol in [
        "struct RuntimeMemoryCheckpointStore {",
        "struct RuntimeMemorySubscriptionStore {",
        "fn persist_checkpoint(",
        "fn persist_subscriptions(",
        "fn checkpoint_record(",
        "fn subscription_record(",
    ] {
        assert!(
            !realtime_source.contains(forbidden_symbol),
            "services/session-gateway/src/realtime.rs should not keep realtime storage symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_websocket_upgrade_transport_seam_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "fn realtime_websocket_subprotocols()",
        "fn select_realtime_websocket_mode(",
        "fn map_runtime_link_websocket_mode(",
        "fn prepare_realtime_websocket_upgrade(",
        "async fn serve_realtime_websocket_upgrade(",
        "let ws = ws.protocols(realtime_websocket_subprotocols());",
        ".on_upgrade(move |socket| upgrade.execute(socket, serve_realtime_websocket_upgrade))",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep websocket upgrade seam symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_websocket_route_handler_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");
    let upgrade_source = include_str!("../src/websocket_upgrade.rs");

    for forbidden_symbol in [
        "use axum::extract::ws::WebSocketUpgrade;",
        "async fn realtime_websocket(",
        "websocket_upgrade::upgrade_realtime_websocket(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep websocket route handler symbol: {forbidden_symbol}"
        );
    }

    assert!(
        upgrade_source.contains("pub(crate) async fn realtime_websocket("),
        "services/session-gateway/src/websocket_upgrade.rs should host realtime websocket route handler"
    );
}

#[test]
fn test_session_gateway_websocket_pending_backlog_math_moves_out_of_service_impl() {
    let websocket_source = include_str!("../src/websocket.rs");

    assert!(
        !websocket_source.contains("fn outbound_pending_events("),
        "services/session-gateway/src/websocket.rs should not keep local backlog math helper"
    );
}

#[test]
fn test_session_gateway_websocket_outbound_queue_state_moves_out_of_service_impl() {
    let websocket_source = include_str!("../src/websocket.rs");

    for forbidden_symbol in [
        "let mut last_sent_seq =",
        "let mut push_cursor =",
        "push_cursor: &mut LinkPushCursor",
        "last_sent_seq: &mut u64",
    ] {
        assert!(
            !websocket_source.contains(forbidden_symbol),
            "services/session-gateway/src/websocket.rs should not keep local outbound queue state symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_websocket_buffered_push_drain_loop_moves_out_of_service_impl() {
    let websocket_source = include_str!("../src/websocket.rs");

    assert!(
        !websocket_source.contains("async fn flush_buffered_push_windows("),
        "services/session-gateway/src/websocket.rs should not keep the buffered push drain loop"
    );
}

#[test]
fn test_session_gateway_websocket_upgrade_module_stays_pure_axum_adapter() {
    let upgrade_source = include_str!("../src/websocket_upgrade.rs");
    let route_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/websocket_route.rs"),
    )
    .expect("services/session-gateway/src/websocket_route.rs should exist");

    for forbidden_symbol in [
        "resolve_auth_context(&headers)?",
        "resolve_requested_device_id(",
    ] {
        assert!(
            !upgrade_source.contains(forbidden_symbol),
            "services/session-gateway/src/websocket_upgrade.rs should keep only the Axum transport adapter, found: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct RealtimeWebsocketRouteContext",
        "pub(crate) fn prepare_realtime_websocket_route(",
        "resolve_auth_context(",
        "resolve_requested_device_id(&auth, None)?",
        "state.prepare_active_device_route(",
    ] {
        assert!(
            route_source.contains(required_symbol),
            "services/session-gateway/src/websocket_route.rs should host websocket route preflight symbol: {required_symbol}"
        );
    }

    assert!(
        !route_source.contains("WebSocketUpgrade"),
        "services/session-gateway/src/websocket_route.rs should not own the Axum WebSocketUpgrade adapter"
    );
    assert!(
        upgrade_source.contains("use axum::extract::WebSocketUpgrade;"),
        "services/session-gateway/src/websocket_upgrade.rs should remain the only Axum WebSocketUpgrade adapter seam"
    );
    assert!(
        upgrade_source.contains("pub(crate) async fn realtime_websocket("),
        "services/session-gateway/src/websocket_upgrade.rs should still host the Axum route entrypoint"
    );
    assert!(
        upgrade_source
            .contains("websocket_route::prepare_realtime_websocket_route(&headers, &state)?"),
        "services/session-gateway/src/websocket_upgrade.rs should delegate route preflight into websocket_route"
    );
    assert!(
        upgrade_source.contains(
            ".on_upgrade(move |socket| upgrade.execute(socket, serve_realtime_websocket_upgrade))"
        ),
        "services/session-gateway/src/websocket_upgrade.rs should keep the Axum on_upgrade seam"
    );
}

#[test]
fn test_session_gateway_session_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");
    let session_source =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/session.rs"))
            .expect("services/session-gateway/src/session.rs should exist");

    for forbidden_symbol in [
        "async fn resume_session(",
        "async fn get_presence_me(",
        "async fn heartbeat_presence(",
        "async fn disconnect_session(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep session/presence handler symbol: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) async fn resume_session(",
        "pub(crate) async fn get_presence_me(",
        "pub(crate) async fn heartbeat_presence(",
        "pub(crate) async fn disconnect_session(",
    ] {
        assert!(
            session_source.contains(required_symbol),
            "services/session-gateway/src/session.rs should host session/presence handler symbol: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_session_paths_use_device_sync_session_state_owner_seam() {
    let session_source =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/session.rs"))
            .expect("services/session-gateway/src/session.rs should exist");

    for required_symbol in [
        "fn device_sync_session_state(",
        "state.device_sync_session_state(",
    ] {
        assert!(
            session_source.contains(required_symbol),
            "services/session-gateway/src/session.rs should consume the shared session sync-state owner seam: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "state.registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str())",
        "state.latest_device_sync_seq(",
    ] {
        assert!(
            !session_source.contains(forbidden_symbol),
            "services/session-gateway/src/session.rs should not keep raw session sync-state reads once the owner seam exists: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_sync_state_owner_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/session_state.rs"),
    )
    .expect("services/session-gateway/src/session_state.rs should exist");

    for forbidden_symbol in [
        "registered_devices: Arc<Mutex<HashMap<String, BTreeSet<String>>>>",
        "latest_sync_sequences: Arc<Mutex<HashMap<String, u64>>>",
        "fn registered_devices(&self, tenant_id: &str, principal_id: &str) -> Vec<String> {",
        "fn latest_device_sync_seq(&self, tenant_id: &str, principal_id: &str, device_id: &str) -> u64 {",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep raw session sync-state owner detail: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "mod session_state;",
        "session_state: SessionSyncState,",
        "self.session_state\n            .device_sync_session_state(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should delegate session sync-state ownership through SessionSyncState: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct SessionSyncState",
        "pub(crate) fn register_device(",
        "pub(crate) fn device_sync_session_state(",
        "fn registered_devices(",
        "fn latest_device_sync_seq(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/session_state.rs should host session sync-state owner implementation: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_device_registration_owner_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/device_registration.rs"),
    )
    .expect("services/session-gateway/src/device_registration.rs should exist");

    for forbidden_symbol in [
        "self.presence_runtime\n            .register_device(",
        "self.realtime_runtime\n            .ensure_device_state(",
        "self.session_state\n            .register_device(",
        "self.realtime_cluster.bind_device_route(",
        "self.realtime_cluster.clear_device_disconnect_fence(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep device registration owner detail: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "mod device_registration;",
        "device_registration: SessionDeviceRegistration,",
        "self.device_registration.register_device(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should delegate device registration ownership through SessionDeviceRegistration: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct SessionDeviceRegistration",
        "pub(crate) fn new(",
        "pub(crate) fn register_device(",
        "pub(crate) fn prepare_active_device_route(",
        "self.presence_runtime\n            .register_device(",
        "self.realtime_runtime\n            .ensure_device_state(",
        "self.session_state\n            .register_device(",
        "self.realtime_cluster.bind_device_route(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/device_registration.rs should host device registration owner implementation: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_route_preflight_owner_moves_out_of_entrypoints() {
    let lib_source = include_str!("../src/lib.rs");
    let session_source =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/session.rs"))
            .expect("services/session-gateway/src/session.rs should exist");
    let route_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/websocket_route.rs"),
    )
    .expect("services/session-gateway/src/websocket_route.rs should exist");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/device_registration.rs"),
    )
    .expect("services/session-gateway/src/device_registration.rs should exist");

    {
        let forbidden_symbol = "state.realtime_cluster.ensure_route_session_current(";
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep raw route preflight glue once the owner seam exists: {forbidden_symbol}"
        );
        assert!(
            !session_source.contains(forbidden_symbol),
            "services/session-gateway/src/session.rs should not keep raw route preflight glue once the owner seam exists: {forbidden_symbol}"
        );
        assert!(
            !route_source.contains(forbidden_symbol),
            "services/session-gateway/src/websocket_route.rs should not keep raw route preflight glue once the owner seam exists: {forbidden_symbol}"
        );
    }

    assert!(
        !route_source.contains("state.register_device("),
        "services/session-gateway/src/websocket_route.rs should delegate route preflight instead of calling register_device directly"
    );

    for required_symbol in [
        "fn prepare_active_device_route(",
        "state.prepare_active_device_route(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should expose the route preflight owner seam: {required_symbol}"
        );
    }

    {
        let required_symbol = "state.prepare_active_device_route(";
        assert!(
            session_source.contains(required_symbol),
            "services/session-gateway/src/session.rs should consume the shared route preflight owner seam: {required_symbol}"
        );
        assert!(
            route_source.contains(required_symbol),
            "services/session-gateway/src/websocket_route.rs should consume the shared route preflight owner seam: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) fn prepare_active_device_route(",
        "fn ensure_route_session_current(",
        "self.ensure_route_session_current(",
        "self.register_device(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/device_registration.rs should host route preflight owner detail: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_disconnect_lifecycle_owner_moves_out_of_session_entrypoints() {
    let lib_source = include_str!("../src/lib.rs");
    let session_source =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/session.rs"))
            .expect("services/session-gateway/src/session.rs should exist");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/device_registration.rs"),
    )
    .expect("services/session-gateway/src/device_registration.rs should exist");

    for forbidden_symbol in [
        "state.realtime_cluster.disconnect_fence_matches_session(",
        "state.realtime_runtime.clear_device_subscriptions(",
        "state.realtime_cluster.release_device_route(",
        "state.realtime_cluster.mark_device_disconnected(",
    ] {
        assert!(
            !session_source.contains(forbidden_symbol),
            "services/session-gateway/src/session.rs should not keep raw disconnect lifecycle glue once the owner seam exists: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "fn disconnect_active_device_route(",
        "self.device_registration.disconnect_active_device_route(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should expose the disconnect lifecycle owner seam: {required_symbol}"
        );
    }

    assert!(
        session_source.contains("state.disconnect_active_device_route("),
        "services/session-gateway/src/session.rs should consume the shared disconnect lifecycle owner seam"
    );

    for required_symbol in [
        "pub(crate) enum DisconnectActiveDeviceRouteOutcome",
        "pub(crate) fn disconnect_active_device_route(",
        "disconnect_fence_matches_session(",
        "clear_device_subscriptions(",
        "release_device_route(",
        "mark_device_disconnected(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/device_registration.rs should host disconnect lifecycle owner detail: {required_symbol}"
        );
    }
}

use std::path::PathBuf;
