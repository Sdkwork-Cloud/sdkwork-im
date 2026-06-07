use super::*;
use tokio::sync::watch;

#[derive(Clone)]
pub(crate) struct LocalNodeClientRouteRegistration {
    node_id: String,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    presence_runtime: Arc<PresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    projection_service: Arc<TimelineProjectionService>,
    snapshot_stores: Option<ProjectionSnapshotStores>,
}

impl LocalNodeClientRouteRegistration {
    pub(crate) fn new(
        node_id: String,
        realtime_cluster: Arc<RealtimeClusterBridge>,
        presence_runtime: Arc<PresenceRuntime>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        projection_service: Arc<TimelineProjectionService>,
        snapshot_stores: Option<ProjectionSnapshotStores>,
    ) -> Self {
        Self {
            node_id,
            realtime_cluster,
            presence_runtime,
            realtime_runtime,
            projection_service,
            snapshot_stores,
        }
    }

    pub(crate) fn ensure_client_route_key(
        &self,
        state: &AppState,
        auth: &AppContext,
    ) -> Result<(), ApiError> {
        if let Some(device_id) = auth.device_id.as_deref() {
            self.bind_client_route_key(state, auth, device_id, "command", false)?;
        }

        Ok(())
    }

    fn ensure_route_session_current(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), ApiError> {
        self.realtime_cluster
            .ensure_route_session_current_for_principal_kind(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
                session_id,
            )?;
        Ok(())
    }

    // Client route binding keeps routing and session ownership inputs explicit so
    // HTTP/websocket callers can share one preflight path without hidden state.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn bind_client_route_key(
        &self,
        state: &AppState,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<projection_service::RegisteredClientRouteView, ApiError> {
        let tenant_id = auth.tenant_id.as_str();
        let principal_id = auth.actor_id.as_str();
        let principal_kind = auth.actor_kind.as_str();
        let session_id = auth.session_id.as_deref();
        if !allow_session_takeover {
            self.realtime_cluster
                .ensure_client_route_resume_not_required_for_principal_kind(
                    tenant_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )?;
            self.presence_runtime
                .ensure_client_route_resume_not_required(auth, device_id)?;
            self.ensure_route_session_current(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
                session_id,
            )?;
            self.realtime_cluster
                .ensure_client_route_local_for_principal_kind(
                    tenant_id,
                    principal_id,
                    principal_kind,
                    device_id,
                    self.node_id.as_str(),
                )?;
        }

        let projection_service = self.projection_service.clone();
        projection_service.ensure_client_route_registration_allowed_from_auth_context(
            auth,
            Some(device_id.to_owned()),
        )?;
        let device = projection_service
            .register_client_route_from_auth_context(auth, Some(device_id.to_owned()))?;
        if principal_kind == "device" {
            projection_service.register_client_route_for_principal_kind(
                tenant_id,
                principal_id,
                "user",
                device_id,
            );
        }
        self.presence_runtime
            .register_client_route(auth, device_id)?;
        self.realtime_runtime
            .ensure_client_route_state_for_principal_kind(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
            )?;
        if let Some(snapshot_stores) = self.snapshot_stores.as_ref() {
            snapshot_stores.persist_client_route_sync_snapshot(projection_service.as_ref());
        }
        self.realtime_cluster.bind_client_route_for_principal_kind(
            tenant_id,
            principal_id,
            principal_kind,
            device_id,
            self.node_id.as_str(),
            session_id,
            connection_kind,
        )?;

        if allow_session_takeover {
            self.realtime_cluster
                .clear_client_route_disconnect_fence_for_principal_kind(
                    tenant_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )?;
        }

        platform::refresh_node_operational_view(state);
        Ok(device)
    }

    pub(crate) fn prepare_active_client_route(
        &self,
        state: &AppState,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<projection_service::RegisteredClientRouteView, ApiError> {
        self.bind_client_route_key(state, auth, device_id, connection_kind, false)
    }
}

impl session_gateway::RealtimeRouteOwner for LocalNodeClientRouteRegistration {
    fn ensure_active_client_route_current_session(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<(), session_gateway::RealtimeRouteOwnerError> {
        self.ensure_route_session_current(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            device_id,
            auth.session_id.as_deref(),
        )
        .map_err(|error| session_gateway::RealtimeRouteOwnerError::new(error.code, error.message))
    }

    fn subscribe_active_client_route_epoch(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, session_gateway::RealtimeRouteOwnerError> {
        Ok(self
            .realtime_cluster
            .subscribe_client_route_epoch_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
            ))
    }

    fn release_active_client_route_if_current_session(&self, auth: &AppContext, device_id: &str) {
        if self
            .ensure_route_session_current(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
                auth.session_id.as_deref(),
            )
            .is_err()
        {
            return;
        }

        let _ = self
            .realtime_cluster
            .release_client_route_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
                self.node_id.as_str(),
            );
    }
}
