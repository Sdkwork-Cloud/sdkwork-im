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
fn test_session_gateway_realtime_cluster_rejects_implicit_user_identity_surfaces() {
    let cluster_source = include_str!("../src/cluster.rs").replace("\r\n", "\n");
    let disconnect_source = include_str!("../src/cluster/disconnect.rs")
        .replace("\r\n", "\n")
        .split("#[cfg(test)]")
        .next()
        .expect("disconnect source should contain production module")
        .to_owned();

    for forbidden_symbol in [
        "pub fn bind_device_route(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn resolve_device_route(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn release_device_route(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn ensure_route_session_current(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn ensure_device_route_local(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn publish_device_event(\n        &self,\n        origin_node_id: &str,\n        tenant_id: &str,\n        principal_id: &str,",
    ] {
        assert!(
            !cluster_source.contains(forbidden_symbol),
            "RealtimeClusterBridge must require explicit principal_kind and reject legacy implicit-user route API: {forbidden_symbol}"
        );
    }

    for forbidden_symbol in [
        "pub fn mark_device_disconnected(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn clear_device_disconnect_fence(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn ensure_device_resume_not_required(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
        "pub fn disconnect_fence_matches_session(\n        &self,\n        tenant_id: &str,\n        principal_id: &str,\n        device_id: &str,",
    ] {
        assert!(
            !disconnect_source.contains(forbidden_symbol),
            "disconnect fence runtime must require explicit principal_kind and reject implicit user/default identity: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_disconnect_fence_token_uses_segment_safe_encoding() {
    let disconnect_source = include_str!("../src/cluster/disconnect.rs")
        .replace("\r\n", "\n")
        .split("#[cfg(test)]")
        .next()
        .expect("disconnect source should contain production module")
        .to_owned();

    for forbidden_symbol in [
        "\"fence:{tenant_id}:",
        "session_id.unwrap_or(\"sessionless\")",
        "session_id.unwrap_or(\"\")",
        "format!(\n        \"fence:",
    ] {
        assert!(
            !disconnect_source.contains(forbidden_symbol),
            "disconnect fence token must use segment-safe encoding instead of delimiter/default sentinels: {forbidden_symbol}"
        );
    }

    assert!(
        disconnect_source.contains("encode_disconnect_fence_token_segments("),
        "disconnect fence token should be built with the shared segment-safe token encoder"
    );
    assert!(
        disconnect_source.contains("Some(session_id) => (\"some-session\", session_id)")
            && disconnect_source.contains("None => (\"no-session\", \"\")"),
        "disconnect fence token must encode Some/None session state as an explicit segment"
    );
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
fn test_realtime_control_contracts_use_explicit_principal_kind() {
    let contract_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/craw-chat-contract-control/src/lib.rs"),
    )
    .expect("craw-chat-contract-control source should exist")
    .replace("\r\n", "\n");

    for required_symbol in [
        "pub struct RealtimeCheckpointRecord {\n    pub tenant_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,",
        "pub struct RealtimeDisconnectFenceRecord {\n    pub tenant_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,",
        "pub struct RealtimeSubscriptionRecord {\n    pub tenant_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,",
        "pub struct PresenceStateRecord {\n    pub tenant_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,",
        "fn load_checkpoint(\n        &self,\n        tenant_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
        "fn save_checkpoints(&self, records: Vec<RealtimeCheckpointRecord>)",
        "fn load_fence(\n        &self,\n        tenant_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
        "fn load_subscriptions(\n        &self,\n        tenant_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
        "fn load_state(\n        &self,\n        tenant_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
        "fn list_states_for_principal(\n        &self,\n        tenant_id: &str,\n        principal_kind: &str,\n        principal_id: &str,",
    ] {
        assert!(
            contract_source.contains(required_symbol),
            "realtime/presence control contract must expose explicit principal_kind: {required_symbol}"
        );
    }
}

#[test]
fn test_realtime_subscription_store_requires_durable_fanout_query_implementation() {
    let contract_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/craw-chat-contract-control/src/lib.rs"),
    )
    .expect("craw-chat-contract-control source should exist")
    .replace("\r\n", "\n");
    let trait_source = contract_source
        .split("pub trait RealtimeSubscriptionStore")
        .nth(1)
        .expect("RealtimeSubscriptionStore trait should exist")
        .split("pub trait PresenceStateStore")
        .next()
        .expect("RealtimeSubscriptionStore trait should precede PresenceStateStore");

    assert!(
        trait_source.contains("fn load_matching_subscriptions("),
        "RealtimeSubscriptionStore must expose a durable scope/event fanout query"
    );
    assert!(
        !trait_source.contains("for device_id in candidate_device_ids"),
        "RealtimeSubscriptionStore must not provide an N-device default implementation for fanout queries"
    );
}

#[test]
fn test_session_gateway_realtime_runtime_requires_explicit_principal_kind() {
    let realtime_source = include_str!("../src/realtime.rs").replace("\r\n", "\n");
    let realtime_storage_source = include_str!("../src/realtime/storage.rs").replace("\r\n", "\n");

    for forbidden_symbol in [
        "actor_device_scope_key",
        "principal_kind: Option<&str>",
        "principal_kind.unwrap_or(\"user\")",
        "fn device_scope_key(\n    tenant_id: &str,\n    principal_id: &str,\n    principal_kind: Option<&str>,",
        "pub fn ensure_device_state(",
        "pub fn subscribe_device(",
        "pub fn subscribe_disconnect_signal(",
        "pub fn disconnect_generation(",
        "pub fn signal_device_disconnect(",
        "pub fn window_checkpoint(",
        "pub fn sync_subscriptions(",
        "pub fn clear_device_subscriptions(",
        "pub fn list_events(",
        "pub fn ack_events(",
        "pub fn take_device_state(",
        "pub fn publish_scope_event(",
        "restore_device_state_for_principal_kind(",
    ] {
        assert!(
            !realtime_source.contains(forbidden_symbol),
            "services/session-gateway/src/realtime.rs must require explicit principal_kind and avoid legacy realtime identity surface: {forbidden_symbol}"
        );
    }

    for forbidden_symbol in [
        "principal_kind: Option<&str>",
        "principal_kind.unwrap_or(\"user\")",
        "Some(principal_kind)",
        "Some(record.principal_kind.as_str())",
    ] {
        assert!(
            !realtime_storage_source.contains(forbidden_symbol),
            "services/session-gateway/src/realtime/storage.rs must persist realtime state with required principal_kind: {forbidden_symbol}"
        );
    }

    assert!(
        realtime_source.contains(
            "pub struct RealtimeDeviceStateSnapshot {\n    pub tenant_id: String,\n    pub principal_kind: String,\n    pub principal_id: String,"
        ),
        "RealtimeDeviceStateSnapshot must carry principal_kind so route migration cannot restore into an implicit default identity"
    );
    assert!(
        realtime_source.contains("pub disconnect_generation: u64,"),
        "RealtimeDeviceStateSnapshot must carry disconnect_generation so runtime migration preserves websocket disconnect signal epochs"
    );
}

#[test]
fn test_session_gateway_realtime_window_store_uses_sequence_index() {
    let realtime_source = include_str!("../src/realtime.rs").replace("\r\n", "\n");

    assert!(
        !realtime_source.contains("windows: Arc<Mutex<HashMap<String, Vec<RealtimeEvent>>>>"),
        "realtime delivery windows must not store device events in a Vec; cursor reads need a sequence index"
    );
    assert!(
        realtime_source
            .contains("windows: Arc<Mutex<HashMap<String, BTreeMap<u64, RealtimeEvent>>>>"),
        "realtime delivery windows should use BTreeMap<u64, RealtimeEvent> per device scope"
    );
    assert!(
        realtime_source.contains("let effective_after_seq = after_seq.max(trimmed_through_seq);"),
        "realtime list_events should clamp the read cursor to the trimmed boundary"
    );
    assert!(
        realtime_source.contains(".range((Excluded(effective_after_seq), Unbounded))"),
        "realtime list_events should range-seek from the effective cursor"
    );
}

#[test]
fn test_session_gateway_realtime_subscription_store_uses_scope_index() {
    let realtime_source = include_str!("../src/realtime.rs").replace("\r\n", "\n");

    assert!(
        realtime_source.contains("fn realtime_subscription_scope_key("),
        "realtime subscription duplicate detection should use a centralized segment-safe scope key"
    );
    assert!(
        realtime_source.contains("encode_realtime_key_segments([scope_type, scope_id])"),
        "realtime subscription scope keys should encode type/id boundaries explicitly"
    );
    assert!(
        !realtime_source.contains("format!(\"{}:{}\", item.scope_type, item.scope_id)"),
        "realtime subscription duplicate detection must not collapse delimiter-shaped scope segments"
    );
    assert!(
        !realtime_source
            .contains("subscriptions: Arc<Mutex<HashMap<String, Vec<RealtimeSubscription>>>>"),
        "realtime subscriptions must not store each device's subscriptions as a Vec; fanout needs scope lookup"
    );
    assert!(
        realtime_source
            .contains("subscriptions: Arc<Mutex<HashMap<String, RealtimeDeviceSubscriptions>>>"),
        "realtime subscriptions should use an indexed per-device subscription store"
    );
    assert!(
        realtime_source
            .contains("by_scope: HashMap<RealtimeSubscriptionScopeKey, RealtimeSubscription>"),
        "per-device realtime subscriptions should index by scope type/id"
    );
    assert!(
        realtime_source.contains("fn subscription_matches_event("),
        "realtime fanout should evaluate event type filters from indexed subscription records"
    );
    assert!(
        realtime_source.contains("candidate_subscriptions\n        .into_iter()"),
        "realtime fanout should iterate scope-index candidates instead of scanning per-device subscriptions"
    );
}

#[test]
fn test_session_gateway_realtime_fanout_uses_scope_device_index() {
    let realtime_source = include_str!("../src/realtime.rs").replace("\r\n", "\n");

    assert!(
        realtime_source.contains(
            "subscription_scope_index:\n        Arc<Mutex<HashMap<RealtimePrincipalScopeKey, BTreeMap<String, RealtimeSubscription>>>>,"
        ),
        "realtime runtime should keep a scope -> device index so publish fanout avoids probing every registered device"
    );
    assert!(
        realtime_source.contains("fn index_device_subscriptions("),
        "realtime runtime should centralize subscription scope index maintenance"
    );
    assert!(
        realtime_source.contains("fn remove_device_subscription_index("),
        "realtime runtime should remove stale scope-index entries when subscriptions clear, move, or restore"
    );
    assert!(
        realtime_source
            .contains("subscription_scope_index\n        .get(&RealtimePrincipalScopeKey::new("),
        "realtime publish should read candidate devices from the scope index"
    );
    let publish_source = realtime_source
        .split("fn publish_scope_event_internal(")
        .nth(1)
        .expect("realtime runtime should keep publish_scope_event_internal")
        .split("fn index_device_subscriptions(")
        .next()
        .expect("publish implementation should precede subscription index helpers");
    assert!(
        publish_source.contains(".load_matching_subscriptions("),
        "realtime publish should use the durable subscription store's scope/event fanout query before restoring cold devices"
    );
    assert!(
        publish_source
            .matches("collect_matched_delivery_targets(")
            .count()
            >= 2,
        "realtime publish should re-read the scope fanout index after restoring durable matching devices"
    );
    assert!(
        publish_source.contains("unmatched_registered_devices"),
        "realtime publish should only ask durable storage for devices missing from the hot fanout index"
    );
    assert!(
        !publish_source.contains(
            "for device_id in &registered_devices {\n            self.ensure_device_state_internal("
        ),
        "realtime publish must not restore every registered device before checking durable scope/event matches"
    );
    assert!(
        !realtime_source.contains("subscriptions: &HashMap<String, RealtimeDeviceSubscriptions>,\n    tenant_id: &str,\n    principal_id: &str,\n    principal_kind: &str,"),
        "collect_matched_delivery_targets must not require the full subscription map once a scope fanout index exists"
    );
    assert!(
        !realtime_source.contains("registered_devices\n        .into_iter()\n        .collect::<BTreeSet<_>>()\n        .into_iter()\n        .filter_map(|device_id|"),
        "realtime publish must not iterate every registered device to discover subscriptions"
    );
}

#[test]
fn test_cluster_delivery_result_separates_route_state_from_runtime_error() {
    let cluster_source = include_str!("../src/cluster.rs").replace("\r\n", "\n");
    assert!(
        cluster_source.contains("pub delivery_error_code: Option<String>,"),
        "RealtimeRouteDeliveryResult should expose runtime delivery errors separately from route_state"
    );
    assert!(
        cluster_source.contains("pub delivery_error_message: Option<String>,"),
        "RealtimeRouteDeliveryResult should preserve runtime delivery error details for diagnostics"
    );
    assert!(
        cluster_source.contains("route_state: route_state.to_string(),"),
        "publish results should keep route resolution state even when runtime delivery fails"
    );
    assert!(
        !cluster_source.contains("Err(error) => (error.code.to_string(), 0)"),
        "runtime delivery errors must not overwrite route_state or collapse into delivered=0"
    );
}

#[test]
fn test_session_gateway_presence_memory_store_uses_principal_index() {
    let presence_source = include_str!("../src/presence.rs").replace("\r\n", "\n");

    assert!(
        presence_source.contains("by_principal: HashMap<String, BTreeSet<String>>"),
        "presence memory state store should maintain a tenant/principal -> device-key index"
    );
    assert!(
        presence_source.contains("by_device: HashMap<String, PresenceStateRecord>"),
        "presence memory state store should keep device records in the same indexed state object"
    );
    assert!(
        presence_source.contains("online_by_seen_at: BTreeSet<PresenceOnlineSeenAtKey>"),
        "presence memory state store should maintain an online last-seen index for lease expiration"
    );
    assert!(
        presence_source.contains("fn list_online_states_seen_at_or_before("),
        "presence state store should expose indexed stale-online listing for expiration jobs"
    );
    assert!(
        !presence_source.contains(".values()\n            .filter(|record| record.tenant_id == tenant_id && record.principal_id == principal_id)"),
        "presence memory state store must not full-scan all device records for principal snapshots"
    );
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
        "resolve_app_context(&headers)?",
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
        "resolve_app_context(",
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
            ".on_upgrade(move |socket| {\n        upgrade.execute(socket, move |socket, context, mode| {"
        ),
        "services/session-gateway/src/websocket_upgrade.rs should keep the Axum on_upgrade seam"
    );
}

#[test]
fn test_session_gateway_session_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");
    let session_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/device_session.rs"),
    )
    .expect("services/session-gateway/src/device_session.rs should exist");

    for forbidden_symbol in [
        "async fn resume_device_session(",
        "async fn get_presence_me(",
        "async fn heartbeat_presence(",
        "async fn disconnect_device_session(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/session-gateway/src/lib.rs should not keep session/presence handler symbol: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) async fn resume_device_session(",
        "pub(crate) async fn get_presence_me(",
        "pub(crate) async fn heartbeat_presence(",
        "pub(crate) async fn disconnect_device_session(",
    ] {
        assert!(
            session_source.contains(required_symbol),
            "services/session-gateway/src/device_session.rs should host session/presence handler symbol: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_session_paths_use_device_sync_state_snapshot_owner_seam() {
    let session_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/device_session.rs"),
    )
    .expect("services/session-gateway/src/device_session.rs should exist");

    for required_symbol in [
        "fn device_sync_state_snapshot(",
        "state.device_sync_state_snapshot(",
    ] {
        assert!(
            session_source.contains(required_symbol),
            "services/session-gateway/src/device_session.rs should consume the shared session sync-state owner seam: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "state.registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str())",
        "state.latest_device_sync_seq(",
    ] {
        assert!(
            !session_source.contains(forbidden_symbol),
            "services/session-gateway/src/device_session.rs should not keep raw session sync-state reads once the owner seam exists: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_sync_state_owner_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs").replace("\r\n", "\n");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/device_sync_state.rs"),
    )
    .expect("services/session-gateway/src/device_sync_state.rs should exist")
    .replace("\r\n", "\n");

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
        "mod device_sync_state;",
        "device_sync_state: DeviceSyncState,",
        "self.device_sync_state\n            .device_sync_state_snapshot(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should delegate session sync-state ownership through DeviceSyncState: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct DeviceSyncState",
        "pub(crate) fn ensure_device_kind_available(",
        "pub(crate) fn register_device(",
        "pub(crate) fn device_sync_state_snapshot(",
        "fn registered_devices(&self, tenant_id: &str, principal_id: &str, principal_kind: &str) -> Vec<String> {",
        "fn latest_device_sync_seq(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/device_sync_state.rs should host session sync-state owner implementation: {required_symbol}"
        );
    }
}

#[test]
fn test_session_gateway_device_registration_owner_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs").replace("\r\n", "\n");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/device_registration.rs"),
    )
    .expect("services/session-gateway/src/device_registration.rs should exist")
    .replace("\r\n", "\n");

    for forbidden_symbol in [
        "self.presence_runtime\n            .register_device(",
        "self.realtime_runtime\n            .ensure_device_state(",
        "self.device_sync_state\n            .register_device(",
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
        "device_registration: DeviceRouteRegistration,",
        "self.device_registration.register_device(",
    ] {
        assert!(
            lib_source.contains(required_symbol),
            "services/session-gateway/src/lib.rs should delegate device registration ownership through DeviceRouteRegistration: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct DeviceRouteRegistration",
        "pub(crate) fn new(",
        "pub(crate) fn register_device(",
        "pub(crate) fn prepare_active_device_route(",
        "self.presence_runtime\n            .register_device(",
        "self.realtime_runtime\n            .ensure_device_state_for_principal_kind(",
        "self.device_sync_state.register_device(",
        "self.realtime_cluster.bind_device_route_for_principal_kind(",
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
    let session_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/device_session.rs"),
    )
    .expect("services/session-gateway/src/device_session.rs should exist");
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
            "services/session-gateway/src/device_session.rs should not keep raw route preflight glue once the owner seam exists: {forbidden_symbol}"
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
            "services/session-gateway/src/device_session.rs should consume the shared route preflight owner seam: {required_symbol}"
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
    let session_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/device_session.rs"),
    )
    .expect("services/session-gateway/src/device_session.rs should exist");
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
            "services/session-gateway/src/device_session.rs should not keep raw disconnect lifecycle glue once the owner seam exists: {forbidden_symbol}"
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
        "services/session-gateway/src/device_session.rs should consume the shared disconnect lifecycle owner seam"
    );

    for required_symbol in [
        "pub(crate) enum DisconnectActiveDeviceRouteOutcome",
        "pub(crate) fn disconnect_active_device_route(",
        "disconnect_fence_matches_session_for_principal_kind(",
        "clear_device_subscriptions_for_principal_kind(",
        "release_device_route_for_principal_kind(",
        "mark_device_disconnected_for_principal_kind(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/session-gateway/src/device_registration.rs should host disconnect lifecycle owner detail: {required_symbol}"
        );
    }
}

use std::path::PathBuf;
