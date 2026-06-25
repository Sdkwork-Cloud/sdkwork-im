use std::sync::Arc;

use im_platform_contracts::ContractError;
use r2d2_postgres::postgres::Row;
use sdkwork_im_runtime_route::{
    normalize_route_organization_id, RouteBinding, RouteBindingRequest, RouteDirectory,
    RouteMigrationResult, RouteNodeLifecycle, RouteRuntimeError, RouteStore,
};

use crate::{run_postgres_io, PostgresRealtimePool};

const UPSERT_ROUTE_BINDING_SQL: &str = r#"
insert into im_route_bindings (
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    owner_node_id,
    session_id,
    connection_kind,
    route_epoch,
    bound_at,
    created_at,
    updated_at
) values (
    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10::timestamptz, now(), now()
)
on conflict (tenant_id, organization_id, principal_kind, principal_id, device_id) do update set
    owner_node_id = excluded.owner_node_id,
    session_id = excluded.session_id,
    connection_kind = excluded.connection_kind,
    route_epoch = excluded.route_epoch,
    bound_at = excluded.bound_at,
    updated_at = now()
where im_route_bindings.route_epoch <= excluded.route_epoch
"#;

const DELETE_ROUTE_BINDING_SQL: &str = r#"
delete from im_route_bindings
where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
  and device_id = $5
  and owner_node_id = $6
"#;

const LOAD_ROUTE_BINDING_SQL: &str = r#"
select
    tenant_id,
    organization_id,
    principal_kind,
    principal_id,
    device_id,
    owner_node_id,
    session_id,
    connection_kind,
    route_epoch,
    to_char(bound_at at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS.MS"Z"') as bound_at
from im_route_bindings
where tenant_id = $1
  and organization_id = $2
  and principal_kind = $3
  and principal_id = $4
  and device_id = $5
"#;

#[derive(Clone)]
pub struct PostgresRoutePersistence {
    pool: PostgresRealtimePool,
}

impl PostgresRoutePersistence {
    pub fn from_pool(pool: PostgresRealtimePool) -> Self {
        Self { pool }
    }

    fn map_contract_error(
        error: ContractError,
        code: &'static str,
        message_prefix: &str,
        node_id: &str,
    ) -> RouteRuntimeError {
        RouteRuntimeError {
            code,
            message: format!("{message_prefix}: {error:?}"),
            node_id: node_id.to_owned(),
        }
    }

    pub fn persist(&self, binding: &RouteBinding) -> Result<(), RouteRuntimeError> {
        let pool = self.pool.clone();
        let binding = binding.clone();
        let node_id = binding.owner_node_id.clone();
        run_postgres_io(move || persist_binding(&pool, &binding)).map_err(|error| {
            Self::map_contract_error(
                error,
                "route_store_write_failed",
                "persist route binding failed",
                node_id.as_str(),
            )
        })
    }

    pub fn remove(&self, binding: &RouteBinding) -> Result<(), RouteRuntimeError> {
        let pool = self.pool.clone();
        let binding = binding.clone();
        let node_id = binding.owner_node_id.clone();
        run_postgres_io(move || remove_binding(&pool, &binding)).map_err(|error| {
            Self::map_contract_error(
                error,
                "route_store_delete_failed",
                "delete route binding failed",
                node_id.as_str(),
            )
        })
    }

    pub fn load(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RouteBinding> {
        let pool = self.pool.clone();
        let tenant_id = tenant_id.to_owned();
        let organization_id = organization_id.to_owned();
        let principal_id = principal_id.to_owned();
        let principal_kind = principal_kind.to_owned();
        let device_id = device_id.to_owned();
        run_postgres_io(move || {
            load_binding(
                &pool,
                tenant_id.as_str(),
                organization_id.as_str(),
                principal_id.as_str(),
                principal_kind.as_str(),
                device_id.as_str(),
            )
        })
        .ok()
        .flatten()
    }
}

fn persist_binding(
    pool: &PostgresRealtimePool,
    binding: &RouteBinding,
) -> Result<(), ContractError> {
    let mut conn = pool
        .get()
        .map_err(|error| route_pool_unavailable(binding.owner_node_id.as_str(), error))?;
    let bound_at = if binding.bound_at.is_empty() {
        chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
    } else {
        binding.bound_at.clone()
    };
    conn.execute(
        UPSERT_ROUTE_BINDING_SQL,
        &[
            &binding.tenant_id,
            &normalize_route_organization_id(binding.organization_id.as_str()),
            &binding.principal_kind,
            &binding.principal_id,
            &binding.device_id,
            &binding.owner_node_id,
            &binding.session_id,
            &binding.connection_kind,
            &(binding.route_epoch as i64),
            &bound_at,
        ],
    )
    .map_err(|error| route_write_failed(binding.owner_node_id.as_str(), error))?;
    Ok(())
}

fn remove_binding(
    pool: &PostgresRealtimePool,
    binding: &RouteBinding,
) -> Result<(), ContractError> {
    let mut conn = pool
        .get()
        .map_err(|error| route_pool_unavailable(binding.owner_node_id.as_str(), error))?;
    conn.execute(
        DELETE_ROUTE_BINDING_SQL,
        &[
            &binding.tenant_id,
            &normalize_route_organization_id(binding.organization_id.as_str()),
            &binding.principal_kind,
            &binding.principal_id,
            &binding.device_id,
            &binding.owner_node_id,
        ],
    )
    .map_err(|error| route_delete_failed(binding.owner_node_id.as_str(), error))?;
    Ok(())
}

fn load_binding(
    pool: &PostgresRealtimePool,
    tenant_id: &str,
    organization_id: &str,
    principal_id: &str,
    principal_kind: &str,
    device_id: &str,
) -> Result<Option<RouteBinding>, ContractError> {
    let mut conn = pool
        .get()
        .map_err(|error| route_pool_unavailable("_hydrate", error))?;
    let row = conn
        .query_opt(
            LOAD_ROUTE_BINDING_SQL,
            &[
                &tenant_id,
                &normalize_route_organization_id(organization_id),
                &principal_kind,
                &principal_id,
                &device_id,
            ],
        )
        .map_err(|error| route_load_failed(error))?;
    Ok(row.as_ref().map(binding_from_row))
}

fn route_pool_unavailable(node_id: &str, error: impl std::fmt::Display) -> ContractError {
    ContractError::Unavailable(format!(
        "postgres route store pool unavailable for node `{node_id}`: {error}"
    ))
}

fn route_write_failed(node_id: &str, error: impl std::fmt::Display) -> ContractError {
    ContractError::Unavailable(format!(
        "persist route binding failed for node `{node_id}`: {error}"
    ))
}

fn route_delete_failed(node_id: &str, error: impl std::fmt::Display) -> ContractError {
    ContractError::Unavailable(format!(
        "delete route binding failed for node `{node_id}`: {error}"
    ))
}

fn route_load_failed(error: impl std::fmt::Display) -> ContractError {
    ContractError::Unavailable(format!("load route binding failed: {error}"))
}

#[derive(Clone)]
pub struct PostgresBackedRouteStore {
    memory: RouteDirectory,
    persistence: PostgresRoutePersistence,
}

impl PostgresBackedRouteStore {
    pub fn from_pool(pool: PostgresRealtimePool) -> Self {
        Self {
            memory: RouteDirectory::default(),
            persistence: PostgresRoutePersistence::from_pool(pool),
        }
    }

    pub fn into_arc(self) -> Arc<dyn RouteStore> {
        Arc::new(self)
    }

    fn hydrate_from_postgres(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RouteBinding> {
        let binding = self.persistence.load(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
        )?;
        self.memory.register_node(binding.owner_node_id.as_str());
        self.memory.observe_external_binding(binding.clone());
        Some(binding)
    }
}

fn binding_from_row(row: &Row) -> RouteBinding {
    RouteBinding {
        tenant_id: row.get(0),
        organization_id: row.get(1),
        principal_kind: row.get(2),
        principal_id: row.get(3),
        device_id: row.get(4),
        owner_node_id: row.get(5),
        session_id: row.get(6),
        connection_kind: row.get(7),
        route_epoch: row.get::<_, i64>(8) as u64,
        bound_at: row.get(9),
    }
}

impl RouteStore for PostgresBackedRouteStore {
    fn register_node(&self, node_id: &str) {
        self.memory.register_node(node_id);
    }

    fn bind(&self, request: RouteBindingRequest) -> Result<RouteBinding, RouteRuntimeError> {
        let binding = self.memory.bind(request)?;
        if let Err(error) = self.persistence.persist(&binding) {
            let _ = self.memory.release(
                binding.tenant_id.as_str(),
                binding.organization_id.as_str(),
                binding.principal_id.as_str(),
                binding.principal_kind.as_str(),
                binding.device_id.as_str(),
                binding.owner_node_id.as_str(),
            );
            return Err(error);
        }
        Ok(binding)
    }

    fn mark_node_draining(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        self.memory.mark_node_draining(node_id)
    }

    fn activate_node(&self, node_id: &str) -> Result<RouteNodeLifecycle, RouteRuntimeError> {
        self.memory.activate_node(node_id)
    }

    fn migrate_routes(
        &self,
        source_node_id: &str,
        target_node_id: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        self.migrate_routes_at(source_node_id, target_node_id, "")
    }

    fn migrate_routes_at(
        &self,
        source_node_id: &str,
        target_node_id: &str,
        migrated_at: &str,
    ) -> Result<RouteMigrationResult, RouteRuntimeError> {
        let migration =
            self.memory
                .migrate_routes_at(source_node_id, target_node_id, migrated_at)?;
        for route in self.memory.routes_for_node(target_node_id) {
            self.persistence.persist(&route)?;
        }
        Ok(migration)
    }

    fn lookup(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
    ) -> Option<RouteBinding> {
        self.memory
            .lookup(
                tenant_id,
                organization_id,
                principal_id,
                principal_kind,
                device_id,
            )
            .or_else(|| {
                self.hydrate_from_postgres(
                    tenant_id,
                    organization_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )
            })
    }

    fn release(
        &self,
        tenant_id: &str,
        organization_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        owner_node_id: &str,
    ) -> Option<RouteBinding> {
        let removed = self.memory.release(
            tenant_id,
            organization_id,
            principal_id,
            principal_kind,
            device_id,
            owner_node_id,
        )?;
        let _ = self.persistence.remove(&removed);
        Some(removed)
    }

    fn restore_if_current(
        &self,
        expected_current: &RouteBinding,
        restore_to: RouteBinding,
    ) -> Option<RouteBinding> {
        let restored = self.memory.restore_if_current(expected_current, restore_to)?;
        let _ = self.persistence.persist(&restored);
        Some(restored)
    }

    fn routes_for_node(&self, node_id: &str) -> Vec<RouteBinding> {
        self.memory.routes_for_node(node_id)
    }

    fn node_lifecycle(&self, node_id: &str) -> Option<RouteNodeLifecycle> {
        self.memory.node_lifecycle(node_id)
    }
}

#[cfg(test)]
mod tests {
    use super::{DELETE_ROUTE_BINDING_SQL, LOAD_ROUTE_BINDING_SQL, UPSERT_ROUTE_BINDING_SQL};

    #[test]
    fn test_route_binding_sql_contracts_use_organization_scoped_primary_key() {
        assert!(UPSERT_ROUTE_BINDING_SQL.contains("organization_id"));
        assert!(DELETE_ROUTE_BINDING_SQL.contains("organization_id"));
        assert!(LOAD_ROUTE_BINDING_SQL.contains("organization_id"));
        assert!(UPSERT_ROUTE_BINDING_SQL.contains("route_epoch <= excluded.route_epoch"));
    }
}
