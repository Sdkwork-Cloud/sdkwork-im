use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use craw_chat_contract_control::RealtimeDisconnectFenceStore;
use craw_chat_runtime_route::{
    RouteBinding, RouteBindingRequest, RouteDirectory, RouteMigrationResult, RouteNodeLifecycle,
    RouteRuntimeError,
};
use im_time::utc_now_rfc3339_millis;

use crate::{
    RealtimeDeliveryRuntime,
    realtime::{RealtimeDeviceStateSnapshot, RealtimeRuntimeError},
};

mod disconnect;

use disconnect::{ClusterMemoryDisconnectFenceStore, RealtimeDisconnectFence};

fn lock_cluster_mutex<'a, T>(mutex: &'a Mutex<T>, label: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            eprintln!("warning: recovering poisoned mutex in session-gateway/cluster: {label}");
            poisoned.into_inner()
        }
    }
}

pub type RealtimeDeviceRoute = RouteBinding;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeRouteDeliveryResult {
    pub target_node_id: String,
    pub route_state: String,
    pub delivered: usize,
}

pub type RealtimeNodeLifecycleView = RouteNodeLifecycle;
pub type RealtimeRouteMigrationResult = RouteMigrationResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RealtimeClusterError {
    pub code: &'static str,
    pub message: String,
    pub node_id: String,
    pub drain_status: String,
    pub rebalance_state: String,
}

#[derive(Clone)]
pub struct RealtimeClusterBridge {
    node_runtimes: Arc<Mutex<HashMap<String, Arc<RealtimeDeliveryRuntime>>>>,
    route_directory: RouteDirectory,
    disconnect_fences: Arc<Mutex<HashMap<String, RealtimeDisconnectFence>>>,
    disconnect_fence_store: Arc<dyn RealtimeDisconnectFenceStore>,
}

impl Default for RealtimeClusterBridge {
    fn default() -> Self {
        Self::with_disconnect_fence_store(Arc::new(ClusterMemoryDisconnectFenceStore::default()))
    }
}

impl fmt::Debug for RealtimeClusterBridge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RealtimeClusterBridge")
            .finish_non_exhaustive()
    }
}

impl RealtimeClusterBridge {
    pub fn with_disconnect_fence_store(
        disconnect_fence_store: Arc<dyn RealtimeDisconnectFenceStore>,
    ) -> Self {
        Self {
            node_runtimes: Arc::new(Mutex::new(HashMap::new())),
            route_directory: RouteDirectory::default(),
            disconnect_fences: Arc::new(Mutex::new(HashMap::new())),
            disconnect_fence_store,
        }
    }

    pub fn bind_node_runtime(&self, node_id: &str, runtime: Arc<RealtimeDeliveryRuntime>) {
        lock_cluster_mutex(&self.node_runtimes, "node_runtimes").insert(node_id.into(), runtime);
        self.route_directory.register_node(node_id);
    }

    pub fn bind_device_route(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        owner_node_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
    ) -> Result<RealtimeDeviceRoute, RealtimeClusterError> {
        self.require_known_node(owner_node_id)?;
        let previous_route = self.resolve_device_route(tenant_id, principal_id, device_id);
        if let Some(previous_route) = previous_route.as_ref()
            && previous_route.owner_node_id != owner_node_id
        {
            let target_runtime = self.require_runtime(owner_node_id)?;
            match self.require_runtime(previous_route.owner_node_id.as_str()) {
                Ok(source_runtime) => {
                    let snapshot: RealtimeDeviceStateSnapshot = source_runtime
                        .take_device_state(tenant_id, principal_id, device_id)
                        .map_err(|error| {
                            self.runtime_store_error(
                                "take device state for route rebind",
                                previous_route.owner_node_id.as_str(),
                                error,
                            )
                        })?;
                    target_runtime
                        .restore_device_state(snapshot)
                        .map_err(|error| {
                            self.runtime_store_error(
                                "restore device state for route rebind",
                                owner_node_id,
                                error,
                            )
                        })?;
                }
                Err(error) if error.code == "node_runtime_missing" => {
                    // The previous owner runtime is already gone, so in-memory state
                    // cannot be handed off. Keep availability by moving ownership to the
                    // new active node and let the target restore any durable checkpoint
                    // state it knows about.
                    target_runtime
                        .ensure_device_state(tenant_id, principal_id, device_id)
                        .map_err(|error| {
                            self.runtime_store_error(
                                "restore durable checkpoint during route rebind",
                                owner_node_id,
                                error,
                            )
                        })?;
                }
                Err(error) => return Err(error),
            }
        }

        self.route_directory
            .bind(
                RouteBindingRequest::new(tenant_id, principal_id, device_id, owner_node_id)
                    .with_session_id(session_id)
                    .with_connection_kind(connection_kind)
                    .with_bound_at(cluster_timestamp()),
            )
            .map_err(|error| self.route_error(error))
    }

    pub fn ensure_route_session_current(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), RealtimeClusterError> {
        let Some(route) = self.resolve_device_route(tenant_id, principal_id, device_id) else {
            return Ok(());
        };
        let Some(current_session_id) = route.session_id.as_deref() else {
            return Ok(());
        };
        let Some(requested_session_id) = session_id else {
            return Err(self.node_error(
                "session_id_required",
                route.owner_node_id.as_str(),
                format!(
                    "device session id is required because the route is currently owned by node {}",
                    route.owner_node_id
                ),
            ));
        };
        if current_session_id == requested_session_id {
            return Ok(());
        }

        Err(self.node_error(
            "stale_session",
            route.owner_node_id.as_str(),
            format!(
                "device session is owned by a newer session on node {}",
                route.owner_node_id
            ),
        ))
    }

    pub fn ensure_device_route_local(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        local_node_id: &str,
    ) -> Result<(), RealtimeClusterError> {
        let Some(route) = self.resolve_device_route(tenant_id, principal_id, device_id) else {
            return Ok(());
        };
        if route.owner_node_id == local_node_id {
            return Ok(());
        }

        if self
            .node_lifecycle(local_node_id)
            .is_some_and(|node| node.drain_status != "active")
        {
            return Err(self.node_error(
                "node_draining",
                local_node_id,
                format!(
                    "node {local_node_id} cannot rebind a device route currently owned by node {} while draining",
                    route.owner_node_id
                ),
            ));
        }

        Err(self.node_error(
            "route_owned_by_other_node",
            route.owner_node_id.as_str(),
            format!(
                "device route is currently owned by node {}",
                route.owner_node_id
            ),
        ))
    }

    pub fn resolve_device_route(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RealtimeDeviceRoute> {
        self.route_directory
            .lookup(tenant_id, principal_id, device_id)
    }

    pub fn release_device_route(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        owner_node_id: &str,
    ) -> Option<RealtimeDeviceRoute> {
        self.route_directory
            .release(tenant_id, principal_id, device_id, owner_node_id)
    }

    pub fn routes_for_node(&self, owner_node_id: &str) -> Vec<RealtimeDeviceRoute> {
        self.route_directory.routes_for_node(owner_node_id)
    }

    pub fn node_lifecycle(&self, node_id: &str) -> Option<RealtimeNodeLifecycleView> {
        self.route_directory.node_lifecycle(node_id)
    }

    pub fn mark_node_draining(
        &self,
        node_id: &str,
    ) -> Result<RealtimeNodeLifecycleView, RealtimeClusterError> {
        self.route_directory
            .mark_node_draining(node_id)
            .map_err(|error| self.route_error(error))
    }

    pub fn activate_node(
        &self,
        node_id: &str,
    ) -> Result<RealtimeNodeLifecycleView, RealtimeClusterError> {
        self.route_directory
            .activate_node(node_id)
            .map_err(|error| self.route_error(error))
    }

    pub fn migrate_node_routes(
        &self,
        source_node_id: &str,
        target_node_id: &str,
    ) -> Result<RealtimeRouteMigrationResult, RealtimeClusterError> {
        if source_node_id == target_node_id {
            return Err(self.node_error(
                "same_node_migration",
                source_node_id,
                "source and target nodes must be different".into(),
            ));
        }

        let source_state = self.node_lifecycle(source_node_id).ok_or_else(|| {
            self.node_error(
                "node_not_found",
                source_node_id,
                format!("source node not found: {source_node_id}"),
            )
        })?;
        if source_state.drain_status != "draining" {
            return Err(self.node_error(
                "node_not_draining",
                source_node_id,
                format!("source node must be draining before migration: {source_node_id}"),
            ));
        }

        let target_state = self.node_lifecycle(target_node_id).ok_or_else(|| {
            self.node_error(
                "target_node_not_found",
                target_node_id,
                format!("target node not found: {target_node_id}"),
            )
        })?;
        if target_state.drain_status != "active" || target_state.rebalance_state != "stable" {
            return Err(self.node_error(
                "target_node_unavailable",
                target_node_id,
                format!("target node is not available for route migration: {target_node_id}"),
            ));
        }

        let routes = self.routes_for_node(source_node_id);
        if !routes.is_empty() {
            let source_runtime = self.require_runtime(source_node_id)?;
            let target_runtime = self.require_runtime(target_node_id)?;

            for route in &routes {
                let snapshot: RealtimeDeviceStateSnapshot = source_runtime
                    .take_device_state(
                        route.tenant_id.as_str(),
                        route.principal_id.as_str(),
                        route.device_id.as_str(),
                    )
                    .map_err(|error| {
                        self.runtime_store_error(
                            "take device state for route migration",
                            source_node_id,
                            error,
                        )
                    })?;
                target_runtime
                    .restore_device_state(snapshot)
                    .map_err(|error| {
                        self.runtime_store_error(
                            "restore device state for route migration",
                            target_node_id,
                            error,
                        )
                    })?;
            }
        }

        self.route_directory
            .migrate_routes_at(source_node_id, target_node_id, cluster_timestamp().as_str())
            .map_err(|error| self.route_error(error))
    }

    // The route publish entrypoint intentionally keeps the delivery identity
    // fields explicit so callers cannot accidentally reorder device targeting
    // and event metadata during cross-node fanout.
    #[allow(clippy::too_many_arguments)]
    pub fn publish_device_event(
        &self,
        origin_node_id: &str,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        scope_type: &str,
        scope_id: &str,
        event_type: &str,
        payload: String,
    ) -> RealtimeRouteDeliveryResult {
        let route = self.resolve_device_route(tenant_id, principal_id, device_id);
        let runtimes = lock_cluster_mutex(&self.node_runtimes, "node_runtimes");
        let (target_node_id, route_state, runtime) = match route {
            Some(route) => {
                let target_node_id = route.owner_node_id;
                let runtime = runtimes.get(target_node_id.as_str()).cloned();
                let route_state = if runtime.is_some() {
                    "resolved"
                } else {
                    "target_runtime_missing"
                };
                (target_node_id, route_state, runtime)
            }
            None => {
                let runtime = runtimes.get(origin_node_id).cloned();
                let route_state = if runtime.is_some() {
                    "local_fallback"
                } else {
                    "origin_runtime_missing"
                };
                (origin_node_id.into(), route_state, runtime)
            }
        };
        drop(runtimes);

        let delivered = runtime
            .map(|runtime| {
                runtime.publish_scope_event(
                    tenant_id,
                    principal_id,
                    scope_type,
                    scope_id,
                    event_type,
                    payload,
                    vec![device_id.into()],
                )
            })
            .map(|result| match result {
                Ok(delivered) => (route_state.to_string(), delivered),
                Err(error) => (error.code.to_string(), 0),
            })
            .unwrap_or_else(|| (route_state.to_string(), 0));

        RealtimeRouteDeliveryResult {
            target_node_id,
            route_state: delivered.0,
            delivered: delivered.1,
        }
    }

    fn require_known_node(&self, node_id: &str) -> Result<(), RealtimeClusterError> {
        if self.route_directory.node_lifecycle(node_id).is_some() {
            return Ok(());
        }

        Err(self.node_error(
            "node_not_found",
            node_id,
            format!("node not found: {node_id}"),
        ))
    }

    fn require_runtime(
        &self,
        node_id: &str,
    ) -> Result<Arc<RealtimeDeliveryRuntime>, RealtimeClusterError> {
        lock_cluster_mutex(&self.node_runtimes, "node_runtimes")
            .get(node_id)
            .cloned()
            .ok_or_else(|| {
                self.node_error(
                    "node_runtime_missing",
                    node_id,
                    format!("node runtime not found: {node_id}"),
                )
            })
    }

    fn node_error(
        &self,
        code: &'static str,
        node_id: &str,
        message: String,
    ) -> RealtimeClusterError {
        let lifecycle = self.node_lifecycle(node_id);
        RealtimeClusterError {
            code,
            message,
            node_id: node_id.into(),
            drain_status: lifecycle
                .as_ref()
                .map(|item| item.drain_status.clone())
                .unwrap_or_else(|| "unknown".into()),
            rebalance_state: lifecycle
                .as_ref()
                .map(|item| item.rebalance_state.clone())
                .unwrap_or_else(|| "unknown".into()),
        }
    }

    fn runtime_store_error(
        &self,
        action: &str,
        node_id: &str,
        error: RealtimeRuntimeError,
    ) -> RealtimeClusterError {
        self.node_error(
            error.code,
            node_id,
            format!("{action} failed: {}", error.message),
        )
    }

    fn route_error(&self, error: RouteRuntimeError) -> RealtimeClusterError {
        self.node_error(error.code, error.node_id.as_str(), error.message)
    }
}

fn device_scope_key(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{device_id}")
}

fn cluster_timestamp() -> String {
    utc_now_rfc3339_millis()
}

#[cfg(test)]
mod tests {
    use std::panic::{self, AssertUnwindSafe};
    use std::sync::{Arc, Mutex};

    use im_adapters_local_memory::MemoryRealtimeDisconnectFenceStore;
    use im_platform_contracts::{
        ContractError, RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore,
    };

    use super::*;
    use crate::RealtimeSubscriptionItemInput;

    fn expect_ok<T>(result: Result<T, crate::realtime::RealtimeRuntimeError>) -> T {
        result.expect("realtime runtime operation should succeed")
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_bind_node_runtime_recovers_from_poisoned_runtime_registry_lock() {
        let cluster = RealtimeClusterBridge::default();
        poison_mutex(&cluster.node_runtimes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            cluster.bind_node_runtime("node_a", Arc::new(RealtimeDeliveryRuntime::default()));
        }));
        assert!(
            result.is_ok(),
            "bind_node_runtime should not panic when runtime registry mutex is poisoned"
        );
        assert!(cluster.node_lifecycle("node_a").is_some());
    }

    #[test]
    fn test_route_rebind_recovers_from_poisoned_runtime_registry_lock() {
        let cluster = RealtimeClusterBridge::default();
        cluster.bind_node_runtime("node_a", Arc::new(RealtimeDeliveryRuntime::default()));
        cluster.bind_node_runtime("node_b", Arc::new(RealtimeDeliveryRuntime::default()));
        cluster
            .bind_device_route(
                "t_demo",
                "u_demo",
                "d_pad",
                "node_a",
                Some("s_old"),
                "websocket",
            )
            .expect("initial route bind should succeed");

        poison_mutex(&cluster.node_runtimes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            cluster.bind_device_route("t_demo", "u_demo", "d_pad", "node_b", Some("s_new"), "http")
        }));
        assert!(
            result.is_ok(),
            "route rebind should not panic when runtime registry mutex is poisoned"
        );
        let bind_result = result.expect("panic status should be captured");
        assert!(
            bind_result.is_ok(),
            "route rebind should recover from poisoned runtime registry lock"
        );
    }

    #[test]
    fn test_publish_recovers_from_poisoned_runtime_registry_lock() {
        let cluster = RealtimeClusterBridge::default();
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
        cluster.bind_node_runtime("node_a", runtime_a.clone());
        expect_ok(runtime_a.sync_subscriptions(
            "t_demo",
            "u_demo",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        ));

        poison_mutex(&cluster.node_runtimes);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            cluster.publish_device_event(
                "node_a",
                "t_demo",
                "u_demo",
                "d_pad",
                "conversation",
                "c_demo",
                "message.posted",
                r#"{"messageId":"msg_poison"}"#.into(),
            )
        }));
        assert!(
            result.is_ok(),
            "publish should not panic when runtime registry mutex is poisoned"
        );
        let publish_result = result.expect("panic status should be captured");
        assert_eq!(publish_result.target_node_id, "node_a");
        assert_eq!(publish_result.route_state, "local_fallback");
        assert_eq!(publish_result.delivered, 1);
    }

    #[test]
    fn test_publish_does_not_fallback_to_origin_when_route_points_to_missing_target_runtime() {
        let cluster = RealtimeClusterBridge::default();
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
        let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
        cluster.bind_node_runtime("node_a", runtime_a.clone());
        cluster.bind_node_runtime("node_b", runtime_b);

        expect_ok(runtime_a.sync_subscriptions(
            "t_demo",
            "u_demo",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        ));
        cluster
            .bind_device_route("t_demo", "u_demo", "d_pad", "node_b", None, "websocket")
            .expect("route bind should succeed");

        cluster
            .node_runtimes
            .lock()
            .expect("realtime cluster runtime registry should lock")
            .remove("node_b");

        let result = cluster.publish_device_event(
            "node_a",
            "t_demo",
            "u_demo",
            "d_pad",
            "conversation",
            "c_demo",
            "message.posted",
            r#"{"messageId":"msg_demo_1"}"#.into(),
        );

        assert_eq!(result.target_node_id, "node_b");
        assert_eq!(result.route_state, "target_runtime_missing");
        assert_eq!(result.delivered, 0);

        let origin_window = expect_ok(runtime_a.list_events("t_demo", "u_demo", "d_pad", 0, 10));
        assert_eq!(origin_window.items.len(), 0);
    }

    #[test]
    fn test_direct_rebind_self_heals_stale_route_when_previous_runtime_is_missing() {
        let cluster = RealtimeClusterBridge::default();
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
        let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
        cluster.bind_node_runtime("node_a", runtime_a);
        cluster.bind_node_runtime("node_b", runtime_b.clone());

        cluster
            .bind_device_route(
                "t_demo",
                "u_demo",
                "d_pad",
                "node_a",
                Some("s_old"),
                "websocket",
            )
            .expect("initial route bind should succeed");
        cluster
            .mark_node_draining("node_a")
            .expect("source drain should succeed");

        cluster
            .node_runtimes
            .lock()
            .expect("realtime cluster runtime registry should lock")
            .remove("node_a");

        let rebound = cluster
            .bind_device_route("t_demo", "u_demo", "d_pad", "node_b", Some("s_new"), "http")
            .expect("stale route should not block takeover when previous runtime is missing");
        assert_eq!(rebound.owner_node_id, "node_b");
        assert_eq!(rebound.connection_kind, "http");
        assert_eq!(rebound.session_id.as_deref(), Some("s_new"));

        let source_lifecycle = cluster
            .node_lifecycle("node_a")
            .expect("source lifecycle should remain observable");
        assert_eq!(source_lifecycle.drain_status, "drained");
        assert_eq!(source_lifecycle.rebalance_state, "stable");
        assert_eq!(source_lifecycle.owned_route_count, 0);

        expect_ok(runtime_b.sync_subscriptions(
            "t_demo",
            "u_demo",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        ));

        let publish = cluster.publish_device_event(
            "node_a",
            "t_demo",
            "u_demo",
            "d_pad",
            "conversation",
            "c_demo",
            "message.posted",
            r#"{"messageId":"msg_after_stale_takeover"}"#.into(),
        );

        assert_eq!(publish.target_node_id, "node_b");
        assert_eq!(publish.route_state, "resolved");
        assert_eq!(publish.delivered, 1);

        let target_window = expect_ok(runtime_b.list_events("t_demo", "u_demo", "d_pad", 0, 10));
        assert_eq!(target_window.items.len(), 1);
        assert_eq!(target_window.items[0].event_type, "message.posted");
    }

    #[test]
    fn test_route_session_fence_rejects_stale_session_after_takeover() {
        let cluster = RealtimeClusterBridge::default();
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
        let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
        cluster.bind_node_runtime("node_a", runtime_a);
        cluster.bind_node_runtime("node_b", runtime_b);

        cluster
            .bind_device_route(
                "t_demo",
                "u_demo",
                "d_pad",
                "node_a",
                Some("s_old"),
                "websocket",
            )
            .expect("initial route bind should succeed");
        cluster
            .bind_device_route("t_demo", "u_demo", "d_pad", "node_b", Some("s_new"), "http")
            .expect("takeover route bind should succeed");

        let stale_error = cluster
            .ensure_route_session_current("t_demo", "u_demo", "d_pad", Some("s_old"))
            .expect_err("stale session should be rejected after takeover");
        assert_eq!(stale_error.code, "stale_session");
        assert_eq!(stale_error.node_id, "node_b");

        cluster
            .ensure_route_session_current("t_demo", "u_demo", "d_pad", Some("s_new"))
            .expect("current session should remain valid");
    }

    #[test]
    fn test_route_session_fence_requires_session_id_once_route_is_bound_to_session() {
        let cluster = RealtimeClusterBridge::default();
        let runtime = Arc::new(RealtimeDeliveryRuntime::default());
        cluster.bind_node_runtime("node_a", runtime);

        cluster
            .bind_device_route(
                "t_demo",
                "u_demo",
                "d_pad",
                "node_a",
                Some("s_live"),
                "websocket",
            )
            .expect("initial route bind should succeed");

        let error = cluster
            .ensure_route_session_current("t_demo", "u_demo", "d_pad", None)
            .expect_err("missing session id should be rejected once route has current session");
        assert_eq!(error.code, "session_id_required");
        assert_eq!(error.node_id, "node_a");
    }

    #[test]
    fn test_disconnect_fence_requires_resume_until_cleared() {
        let cluster = RealtimeClusterBridge::default();
        let runtime = Arc::new(RealtimeDeliveryRuntime::default());
        cluster.bind_node_runtime("node_a", runtime);

        cluster
            .mark_device_disconnected("t_demo", "u_demo", "d_pad", Some("s_old"), "node_a")
            .expect("disconnect fence should persist");

        let error = cluster
            .ensure_device_resume_not_required("t_demo", "u_demo", "d_pad")
            .expect_err("disconnect fence should require an explicit resume");
        assert_eq!(error.code, "reconnect_required");
        assert_eq!(error.node_id, "node_a");
        assert!(
            cluster
                .disconnect_fence_matches_session("t_demo", "u_demo", "d_pad", Some("s_old"))
                .expect("session match should load")
        );
        assert!(
            !cluster
                .disconnect_fence_matches_session("t_demo", "u_demo", "d_pad", Some("s_other"))
                .expect("session mismatch should load")
        );

        assert!(
            cluster
                .clear_device_disconnect_fence("t_demo", "u_demo", "d_pad")
                .expect("disconnect fence clear should succeed")
        );
        cluster
            .ensure_device_resume_not_required("t_demo", "u_demo", "d_pad")
            .expect("fresh resume should clear the disconnect fence");
    }

    #[test]
    fn test_disconnect_fence_survives_bridge_rebuild_with_shared_store() {
        let store = Arc::new(MemoryRealtimeDisconnectFenceStore::default());
        let cluster_a = RealtimeClusterBridge::with_disconnect_fence_store(store.clone());
        let runtime_a = Arc::new(RealtimeDeliveryRuntime::default());
        cluster_a.bind_node_runtime("node_a", runtime_a);
        cluster_a
            .mark_device_disconnected("t_demo", "u_demo", "d_pad", Some("s_old"), "node_a")
            .expect("disconnect fence should persist");

        let cluster_b = RealtimeClusterBridge::with_disconnect_fence_store(store);
        let runtime_b = Arc::new(RealtimeDeliveryRuntime::default());
        cluster_b.bind_node_runtime("node_b", runtime_b);

        let error = cluster_b
            .ensure_device_resume_not_required("t_demo", "u_demo", "d_pad")
            .expect_err("persisted disconnect fence should still require a fresh resume");
        assert_eq!(error.code, "reconnect_required");
        assert!(
            cluster_b
                .disconnect_fence_matches_session("t_demo", "u_demo", "d_pad", Some("s_old"))
                .expect("restored session match should load")
        );

        assert!(
            cluster_b
                .clear_device_disconnect_fence("t_demo", "u_demo", "d_pad")
                .expect("restored fence clear should succeed")
        );
        cluster_b
            .ensure_device_resume_not_required("t_demo", "u_demo", "d_pad")
            .expect("clearing the restored fence should allow traffic again");
    }

    #[derive(Clone, Default)]
    struct FailingDisconnectFenceStore;

    impl RealtimeDisconnectFenceStore for FailingDisconnectFenceStore {
        fn load_fence(
            &self,
            _tenant_id: &str,
            _principal_id: &str,
            _device_id: &str,
        ) -> Result<Option<RealtimeDisconnectFenceRecord>, ContractError> {
            Err(ContractError::Unavailable(
                "disconnect fence store load failed".into(),
            ))
        }

        fn save_fence(&self, _record: RealtimeDisconnectFenceRecord) -> Result<(), ContractError> {
            Err(ContractError::Unavailable(
                "disconnect fence store save failed".into(),
            ))
        }

        fn clear_fence(
            &self,
            _tenant_id: &str,
            _principal_id: &str,
            _device_id: &str,
        ) -> Result<bool, ContractError> {
            Err(ContractError::Unavailable(
                "disconnect fence store clear failed".into(),
            ))
        }
    }

    #[test]
    fn test_disconnect_fence_store_failures_surface_as_controlled_cluster_errors() {
        let cluster = RealtimeClusterBridge::with_disconnect_fence_store(Arc::new(
            FailingDisconnectFenceStore,
        ));
        let runtime = Arc::new(RealtimeDeliveryRuntime::default());
        cluster.bind_node_runtime("node_a", runtime);

        let save_error = cluster
            .mark_device_disconnected("t_demo", "u_demo", "d_pad", Some("s_old"), "node_a")
            .expect_err("save failure should not panic");
        assert_eq!(save_error.code, "disconnect_fence_store_unavailable");

        let load_error = cluster
            .ensure_device_resume_not_required("t_demo", "u_demo", "d_pad")
            .expect_err("load failure should surface as a controlled error");
        assert_eq!(load_error.code, "disconnect_fence_store_unavailable");

        let clear_error = cluster
            .clear_device_disconnect_fence("t_demo", "u_demo", "d_pad")
            .expect_err("clear failure should not panic");
        assert_eq!(clear_error.code, "disconnect_fence_store_unavailable");
    }
}
