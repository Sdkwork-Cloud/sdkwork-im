use super::*;
use im_platform_contracts::{
    ContractError, DeviceAccessOwnerBindingRequest, DeviceAccessProvider,
    DeviceAccessRegistrationRequest, ProviderHealthSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DisconnectActiveDeviceRouteOutcome {
    FenceMatchedSession,
    DeviceDisconnected,
}

const LOCAL_MINIMAL_DEVICE_PRODUCT_ID: &str = "local-minimal-device";
const LOCAL_MINIMAL_DEVICE_CREDENTIAL_KIND: &str = "session";

#[derive(Clone)]
pub(crate) struct LocalNodeDeviceRegistration {
    node_id: String,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    session_presence_runtime: Arc<SessionPresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    projection_service: Arc<TimelineProjectionService>,
    snapshot_stores: Option<ProjectionSnapshotStores>,
    device_access_provider: Arc<dyn DeviceAccessProvider>,
}

impl LocalNodeDeviceRegistration {
    pub(crate) fn new(
        node_id: String,
        realtime_cluster: Arc<RealtimeClusterBridge>,
        session_presence_runtime: Arc<SessionPresenceRuntime>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        projection_service: Arc<TimelineProjectionService>,
        snapshot_stores: Option<ProjectionSnapshotStores>,
        device_access_provider: Arc<dyn DeviceAccessProvider>,
    ) -> Self {
        Self {
            node_id,
            realtime_cluster,
            session_presence_runtime,
            realtime_runtime,
            projection_service,
            snapshot_stores,
            device_access_provider,
        }
    }

    pub(crate) fn ensure_registered_device(
        &self,
        state: &AppState,
        auth: &AuthContext,
    ) -> Result<(), ApiError> {
        if let Some(device_id) = auth.device_id.as_deref() {
            self.bind_registered_device(
                state,
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                device_id,
                auth.session_id.as_deref(),
                "command",
                false,
            )?;
        }

        Ok(())
    }

    fn ensure_route_session_current(
        &self,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), ApiError> {
        self.realtime_cluster.ensure_route_session_current(
            tenant_id,
            principal_id,
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
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<projection_service::RegisteredDeviceView, ApiError> {
        if !allow_session_takeover {
            self.realtime_cluster.ensure_device_resume_not_required(
                tenant_id,
                principal_id,
                device_id,
            )?;
            self.session_presence_runtime
                .ensure_device_resume_not_required(tenant_id, principal_id, device_id)?;
            self.ensure_route_session_current(tenant_id, principal_id, device_id, session_id)?;
            self.realtime_cluster.ensure_device_route_local(
                tenant_id,
                principal_id,
                device_id,
                self.node_id.as_str(),
            )?;
        }

        self.ensure_device_access_registered(tenant_id, principal_id, device_id, session_id)?;
        self.session_presence_runtime
            .register_device(tenant_id, principal_id, device_id)?;
        self.realtime_runtime
            .ensure_device_state(tenant_id, principal_id, device_id)?;
        let projection_service = self.projection_service.clone();
        let device = projection_service.register_device(tenant_id, principal_id, device_id);
        if let Some(snapshot_stores) = self.snapshot_stores.as_ref() {
            snapshot_stores.persist_device_sync_snapshot(projection_service.as_ref());
        }
        self.realtime_cluster.bind_device_route(
            tenant_id,
            principal_id,
            device_id,
            self.node_id.as_str(),
            session_id,
            connection_kind,
        )?;

        if allow_session_takeover {
            self.realtime_cluster.clear_device_disconnect_fence(
                tenant_id,
                principal_id,
                device_id,
            )?;
        }

        platform::refresh_node_operational_view(state);
        Ok(device)
    }

    pub(crate) fn prepare_active_device_route(
        &self,
        state: &AppState,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
    ) -> Result<projection_service::RegisteredDeviceView, ApiError> {
        self.bind_registered_device(
            state,
            tenant_id,
            principal_id,
            device_id,
            session_id,
            connection_kind,
            false,
        )
    }

    pub(crate) fn disconnect_active_device_route(
        &self,
        state: &AppState,
        tenant_id: &str,
        principal_id: &str,
        device_id: &str,
        session_id: Option<&str>,
        connection_kind: &str,
    ) -> Result<DisconnectActiveDeviceRouteOutcome, ApiError> {
        if self.realtime_cluster.disconnect_fence_matches_session(
            tenant_id,
            principal_id,
            device_id,
            session_id,
        )? {
            self.realtime_runtime
                .signal_device_disconnect(tenant_id, principal_id, device_id)?;
            return Ok(DisconnectActiveDeviceRouteOutcome::FenceMatchedSession);
        }

        self.prepare_active_device_route(
            state,
            tenant_id,
            principal_id,
            device_id,
            session_id,
            connection_kind,
        )?;
        self.realtime_runtime
            .clear_device_subscriptions(tenant_id, principal_id, device_id)?;
        let _ = self.realtime_cluster.release_device_route(
            tenant_id,
            principal_id,
            device_id,
            self.node_id.as_str(),
        );
        self.realtime_cluster.mark_device_disconnected(
            tenant_id,
            principal_id,
            device_id,
            session_id,
            self.node_id.as_str(),
        )?;
        self.realtime_runtime
            .signal_device_disconnect(tenant_id, principal_id, device_id)?;
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
