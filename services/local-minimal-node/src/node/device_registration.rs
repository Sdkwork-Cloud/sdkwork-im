use super::*;
use im_platform_contracts::{
    ContractError, DeviceAccessOwnerBindingRequest, DeviceAccessProvider,
    DeviceAccessRegistrationRequest, ProviderHealthSnapshot,
};
use tokio::sync::watch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DisconnectActiveDeviceRouteOutcome {
    FenceMatchedDeviceSession,
    DeviceDisconnected,
}

const LOCAL_MINIMAL_DEVICE_PRODUCT_ID: &str = "local-minimal-device";
const LOCAL_MINIMAL_DEVICE_CREDENTIAL_KIND: &str = "device_route";

#[derive(Clone)]
pub(crate) struct LocalNodeDeviceRegistration {
    node_id: String,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    device_presence_runtime: Arc<DevicePresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    projection_service: Arc<TimelineProjectionService>,
    snapshot_stores: Option<ProjectionSnapshotStores>,
    device_access_provider: Arc<dyn DeviceAccessProvider>,
}

impl LocalNodeDeviceRegistration {
    pub(crate) fn new(
        node_id: String,
        realtime_cluster: Arc<RealtimeClusterBridge>,
        device_presence_runtime: Arc<DevicePresenceRuntime>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        projection_service: Arc<TimelineProjectionService>,
        snapshot_stores: Option<ProjectionSnapshotStores>,
        device_access_provider: Arc<dyn DeviceAccessProvider>,
    ) -> Self {
        Self {
            node_id,
            realtime_cluster,
            device_presence_runtime,
            realtime_runtime,
            projection_service,
            snapshot_stores,
            device_access_provider,
        }
    }

    pub(crate) fn ensure_registered_device(
        &self,
        state: &AppState,
        auth: &AppContext,
    ) -> Result<(), ApiError> {
        if let Some(device_id) = auth.device_id.as_deref() {
            self.bind_registered_device(state, auth, device_id, "command", false)?;
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

    fn projection_device_registered(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
    ) -> bool {
        self.projection_service
            .registered_devices(tenant_id, principal_id)
            .into_iter()
            .any(|registered| registered.device_id == device_id)
    }

    fn ensure_device_access_registered(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), ApiError> {
        if self.projection_device_registered(tenant_id, principal_id, device_id) {
            return Ok(());
        }

        self.device_access_provider
            .register_device(DeviceAccessRegistrationRequest {
                tenant_id: tenant_id.into(),
                device_id: device_id.into(),
                product_id: LOCAL_MINIMAL_DEVICE_PRODUCT_ID.into(),
                credential_kind: LOCAL_MINIMAL_DEVICE_CREDENTIAL_KIND.into(),
                owner_principal_id: Some(principal_id.into()),
            })?;
        let owner_bound =
            self.device_access_provider
                .bind_owner(DeviceAccessOwnerBindingRequest {
                    tenant_id: tenant_id.into(),
                    device_id: device_id.into(),
                    owner_principal_id: principal_id.into(),
                    session_id: session_id.map(str::to_owned),
                })?;
        if !owner_bound {
            return Err(ContractError::Conflict(format!(
                "device access provider declined owner binding for device {device_id}"
            ))
            .into());
        }

        Ok(())
    }

    // Device registration keeps routing and session ownership inputs explicit so
    // HTTP/websocket callers can share one preflight path without hidden state.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn bind_registered_device(
        &self,
        state: &AppState,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<projection_service::RegisteredDeviceView, ApiError> {
        let tenant_id = auth.tenant_id.as_str();
        let principal_id = auth.actor_id.as_str();
        let principal_kind = auth.actor_kind.as_str();
        let session_id = auth.session_id.as_deref();
        if !allow_session_takeover {
            self.realtime_cluster
                .ensure_device_resume_not_required_for_principal_kind(
                    tenant_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )?;
            self.device_presence_runtime
                .ensure_device_resume_not_required(auth, device_id)?;
            self.ensure_route_session_current(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
                session_id,
            )?;
            self.realtime_cluster
                .ensure_device_route_local_for_principal_kind(
                    tenant_id,
                    principal_id,
                    principal_kind,
                    device_id,
                    self.node_id.as_str(),
                )?;
        }

        let projection_service = self.projection_service.clone();
        projection_service.ensure_device_registration_allowed_from_auth_context(
            auth,
            Some(device_id.to_owned()),
        )?;
        self.ensure_device_access_registered(tenant_id, principal_id, device_id, session_id)?;
        let device = projection_service
            .register_device_from_auth_context(auth, Some(device_id.to_owned()))?;
        if principal_kind == "device" {
            projection_service.register_device_for_principal_kind(
                tenant_id,
                principal_id,
                "user",
                device_id,
            );
        }
        self.device_presence_runtime
            .register_device(auth, device_id)?;
        self.realtime_runtime
            .ensure_device_state_for_principal_kind(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
            )?;
        if let Some(snapshot_stores) = self.snapshot_stores.as_ref() {
            snapshot_stores.persist_device_sync_snapshot(projection_service.as_ref());
        }
        self.realtime_cluster.bind_device_route_for_principal_kind(
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
                .clear_device_disconnect_fence_for_principal_kind(
                    tenant_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )?;
        }

        platform::refresh_node_operational_view(state);
        Ok(device)
    }

    pub(crate) fn prepare_active_device_route(
        &self,
        state: &AppState,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<projection_service::RegisteredDeviceView, ApiError> {
        self.bind_registered_device(state, auth, device_id, connection_kind, false)
    }

    pub(crate) fn disconnect_active_device_route(
        &self,
        state: &AppState,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<DisconnectActiveDeviceRouteOutcome, ApiError> {
        let tenant_id = auth.tenant_id.as_str();
        let principal_id = auth.actor_id.as_str();
        let principal_kind = auth.actor_kind.as_str();
        let session_id = auth.session_id.as_deref();
        if self
            .realtime_cluster
            .disconnect_fence_matches_session_for_principal_kind(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
                session_id,
            )?
        {
            self.realtime_runtime
                .signal_device_disconnect_for_principal_kind(
                    tenant_id,
                    principal_id,
                    principal_kind,
                    device_id,
                )?;
            return Ok(DisconnectActiveDeviceRouteOutcome::FenceMatchedDeviceSession);
        }

        self.prepare_active_device_route(state, auth, device_id, connection_kind)?;
        self.realtime_runtime
            .clear_device_subscriptions_for_principal_kind(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
            )?;
        let _ = self
            .realtime_cluster
            .release_device_route_for_principal_kind(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
                self.node_id.as_str(),
            );
        self.realtime_cluster
            .mark_device_disconnected_for_principal_kind(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
                session_id,
                self.node_id.as_str(),
            )?;
        self.realtime_runtime
            .signal_device_disconnect_for_principal_kind(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
            )?;
        platform::refresh_node_operational_view(state);
        Ok(DisconnectActiveDeviceRouteOutcome::DeviceDisconnected)
    }

    pub(crate) fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        self.device_access_provider.provider_health_snapshot()
    }

    pub(crate) fn provider_descriptor(&self) -> im_platform_contracts::ProviderPluginDescriptor {
        self.device_access_provider.descriptor()
    }
}

impl session_gateway::RealtimeRouteOwner for LocalNodeDeviceRegistration {
    fn ensure_active_device_route_current_session(
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

    fn subscribe_active_device_route_epoch(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, session_gateway::RealtimeRouteOwnerError> {
        Ok(self
            .realtime_cluster
            .subscribe_device_route_epoch_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
            ))
    }

    fn release_active_device_route_if_current_session(&self, auth: &AppContext, device_id: &str) {
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
            .release_device_route_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
                self.node_id.as_str(),
            );
    }
}
