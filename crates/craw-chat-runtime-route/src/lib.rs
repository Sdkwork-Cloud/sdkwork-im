use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteBinding {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub owner_node_id: String,
    pub session_id: Option<String>,
    pub connection_kind: String,
    pub bound_at: String,
    pub route_epoch: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteBindingRequest {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub owner_node_id: String,
    pub session_id: Option<String>,
    pub connection_kind: String,
    pub bound_at: String,
}

impl RouteBindingRequest {
    pub fn new(tenant_id: &str, principal_id: &str, device_id: &str, owner_node_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            device_id: device_id.into(),
            owner_node_id: owner_node_id.into(),
            session_id: None,
            connection_kind: "unknown".into(),
            bound_at: String::new(),
        }
    }

    pub fn with_session_id(mut self, session_id: Option<&str>) -> Self {
        self.session_id = session_id.map(str::to_owned);
        self
    }

    pub fn with_connection_kind(mut self, connection_kind: &str) -> Self {
        self.connection_kind = connection_kind.into();
        self
    }

    pub fn with_bound_at(mut self, bound_at: impl Into<String>) -> Self {
        self.bound_at = bound_at.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteNodeLifecycle {
    pub node_id: String,
    pub drain_status: String,
    pub rebalance_state: String,
    pub owned_route_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteMigrationResult {
    pub source_node_id: String,
    pub target_node_id: String,
    pub migrated_route_count: usize,
    pub source_drain_status: String,
    pub source_rebalance_state: String,
    pub target_drain_status: String,
    pub target_rebalance_state: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RouteRuntimeError {
    pub code: &'static str,
    pub message: String,
    pub node_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct NodeLifecycleState {
    drain_status: String,
    rebalance_state: String,
}

#[derive(Clone, Default)]
pub struct RouteDirectory {
    routes: Arc<Mutex<HashMap<String, RouteBinding>>>,
    nodes: Arc<Mutex<HashMap<String, NodeLifecycleState>>>,
}

impl RouteDirectory {
    pub fn register_node(&self, node_id: &str) {
        self.nodes
            .lock()
            .expect("route nodes should lock")
            .entry(node_id.into())
            .or_insert_with(|| NodeLifecycleState {
                drain_status: "active".into(),
                rebalance_state: "stable".into(),
            });
    }

    pub fn bind(&self, request: RouteBindingRequest) -> Result<RouteBinding, RouteRuntimeError> {
        let lifecycle = self.node_state(request.owner_node_id.as_str())?;
        if lifecycle.drain_status != "active" {
            return Err(RouteRuntimeError {
                code: "node_draining",
                message: format!(
                    "node {} is not accepting new route binds while {}",
                    request.owner_node_id, lifecycle.drain_status
                ),
                node_id: request.owner_node_id,
            });
        }

        let key = route_key(
            request.tenant_id.as_str(),
            request.principal_id.as_str(),
            request.device_id.as_str(),
        );
        let mut routes = self.routes.lock().expect("route bindings should lock");
        let previous_owner = routes.get(&key).map(|route| route.owner_node_id.clone());
        let next_epoch = routes
            .get(&key)
            .map(|route| {
                if route.owner_node_id == request.owner_node_id {
                    route.route_epoch
                } else {
                    route.route_epoch + 1
                }
            })
            .unwrap_or(1);
        let binding = RouteBinding {
            tenant_id: request.tenant_id,
            principal_id: request.principal_id,
            device_id: request.device_id,
            owner_node_id: request.owner_node_id,
            session_id: request.session_id,
            connection_kind: request.connection_kind,
            bound_at: request.bound_at,
            route_epoch: next_epoch,
        };
        routes.insert(key, binding.clone());
        drop(routes);

        if let Some(previous_owner) = previous_owner
            && previous_owner != binding.owner_node_id
        {
            self.reconcile_departure(previous_owner.as_str())?;
        }
        Ok(binding)
    }

    pub fn mark_node_draining(
        &self,
        node_id: &str,
    ) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        let owned_route_count = self.owned_route_count(node_id);
        let (drain_status, rebalance_state) = if owned_route_count == 0 {
            ("drained", "stable")
        } else {
            ("draining", "moving_routes")
        };
        self.set_node_state(node_id, drain_status, rebalance_state)?;
        self.node_lifecycle_view(node_id)
    }

    pub fn activate_node(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        self.set_node_state(node_id, "active", "stable")?;
        self.node_lifecycle_view(node_id)
    }

    pub fn migrate_routes(
        &self,
        source_node_id: &str,
        target_node_id: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        self.migrate_routes_at(source_node_id, target_node_id, "")
    }

    pub fn migrate_routes_at(
        &self,
        source_node_id: &str,
        target_node_id: &str,
        migrated_at: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        if source_node_id == target_node_id {
            return Err(RouteRuntimeError {
                code: "same_node_migration",
                message: "source and target nodes must be different".into(),
                node_id: source_node_id.into(),
            });
        }

        let source = self.node_state(source_node_id)?;
        if source.drain_status != "draining" {
            return Err(RouteRuntimeError {
                code: "node_not_draining",
                message: format!("source node must be draining before migration: {source_node_id}"),
                node_id: source_node_id.into(),
            });
        }
        let target = self.node_state(target_node_id)?;
        if target.drain_status != "active" || target.rebalance_state != "stable" {
            return Err(RouteRuntimeError {
                code: "target_node_unavailable",
                message: format!("target node is not ready to accept routes: {target_node_id}"),
                node_id: target_node_id.into(),
            });
        }

        let mut migrated = 0;
        let mut routes = self.routes.lock().expect("route bindings should lock");
        for route in routes.values_mut() {
            if route.owner_node_id == source_node_id {
                route.owner_node_id = target_node_id.into();
                route.route_epoch += 1;
                if !migrated_at.is_empty() {
                    route.bound_at = migrated_at.into();
                }
                migrated += 1;
            }
        }
        drop(routes);

        self.set_node_state(target_node_id, "active", "stable")?;
        let source_status = if self.owned_route_count(source_node_id) == 0 {
            self.set_node_state(source_node_id, "drained", "stable")?;
            self.node_state(source_node_id)?
        } else {
            self.set_node_state(source_node_id, "draining", "moving_routes")?;
            self.node_state(source_node_id)?
        };
        let target_status = self.node_state(target_node_id)?;
        Ok(RouteMigrationResult {
            source_node_id: source_node_id.into(),
            target_node_id: target_node_id.into(),
            migrated_route_count: migrated,
            source_drain_status: source_status.drain_status,
            source_rebalance_state: source_status.rebalance_state,
            target_drain_status: target_status.drain_status,
            target_rebalance_state: target_status.rebalance_state,
        })
    }

    pub fn lookup(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> Option<RouteBinding> {
        self.routes
            .lock()
            .expect("route bindings should lock")
            .get(route_key(tenant_id, principal_id, device_id).as_str())
            .cloned()
    }

    pub fn release(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        owner_node_id: &str,
    ) -> Option<RouteBinding> {
        let key = route_key(tenant_id, principal_id, device_id);
        let removed = {
            let mut routes = self.routes.lock().expect("route bindings should lock");
            match routes.get(&key) {
                Some(route) if route.owner_node_id == owner_node_id => routes.remove(&key),
                _ => None,
            }
        };
        if let Some(route) = removed.as_ref() {
            let _ = self.reconcile_departure(route.owner_node_id.as_str());
        }
        removed
    }

    pub fn routes_for_node(&self, node_id: &str) -> Vec<RouteBinding> {
        let mut items = self
            .routes
            .lock()
            .expect("route bindings should lock")
            .values()
            .filter(|route| route.owner_node_id == node_id)
            .cloned()
            .collect::<Vec<_>>();
        items.sort_by(|left, right| {
            left.tenant_id
                .cmp(&right.tenant_id)
                .then_with(|| left.principal_id.cmp(&right.principal_id))
                .then_with(|| left.device_id.cmp(&right.device_id))
        });
        items
    }

    pub fn node_lifecycle(&self, node_id: &str) -> Option<RouteNodeLifecycle> {
        self.node_lifecycle_view(node_id).ok()
    }

    fn owned_route_count(&self, node_id: &str) -> usize {
        self.routes
            .lock()
            .expect("route bindings should lock")
            .values()
            .filter(|route| route.owner_node_id == node_id)
            .count()
    }

    fn node_lifecycle_view(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        let state = self.node_state(node_id)?;
        Ok(RouteNodeLifecycle {
            node_id: node_id.into(),
            drain_status: state.drain_status,
            rebalance_state: state.rebalance_state,
            owned_route_count: self.owned_route_count(node_id),
        })
    }

    fn reconcile_departure(&self, node_id: &str) -> Result<(), RouteRuntimeError> {
        let state = match self.node_state(node_id) {
            Ok(state) => state,
            Err(_) => return Ok(()),
        };
        if state.drain_status != "draining" {
            return Ok(());
        }

        if self.owned_route_count(node_id) == 0 {
            self.set_node_state(node_id, "drained", "stable")?;
        } else {
            self.set_node_state(node_id, "draining", "moving_routes")?;
        }
        Ok(())
    }

    fn node_state(&self, node_id: &str) -> Result<NodeLifecycleState, RouteRuntimeError> {
        self.nodes
            .lock()
            .expect("route nodes should lock")
            .get(node_id)
            .cloned()
            .ok_or_else(|| RouteRuntimeError {
                code: "node_not_found",
                message: format!("node not found: {node_id}"),
                node_id: node_id.into(),
            })
    }

    fn set_node_state(
        &self,
        node_id: &str,
        drain_status: &str,
        rebalance_state: &str,
    ) -> Result<(), RouteRuntimeError> {
        let mut nodes = self.nodes.lock().expect("route nodes should lock");
        let state = nodes.get_mut(node_id).ok_or_else(|| RouteRuntimeError {
            code: "node_not_found",
            message: format!("node not found: {node_id}"),
            node_id: node_id.into(),
        })?;
        state.drain_status = drain_status.into();
        state.rebalance_state = rebalance_state.into();
        Ok(())
    }
}

fn route_key(tenant_id: &str, principal_id: &str, device_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{device_id}")
}
