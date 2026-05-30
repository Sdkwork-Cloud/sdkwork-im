use std::sync::Arc;

use im_app_context::AppContext;
use tokio::sync::watch;

use super::ApiError;
use super::cluster::RealtimeClusterBridge;
use super::device_sync_state::DeviceSyncState;
use super::presence::DevicePresenceRuntime;
use super::realtime::RealtimeDeliveryRuntime;
use im_platform_contracts::{
    ContractError, DeviceAccessOwnerBindingRequest, DeviceAccessProvider,
    DeviceAccessRegistrationRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DisconnectActiveDeviceRouteOutcome {
    FenceMatchedDeviceSession,
    DeviceDisconnected,
}

const DEVICE_ROUTE_GATEWAY_DEVICE_PRODUCT_ID: &str = "device-route-gateway-device";
const DEVICE_ROUTE_GATEWAY_DEVICE_CREDENTIAL_KIND: &str = "device_route";

#[derive(Clone)]
pub(crate) struct DeviceRouteRegistration {
    node_id: String,
    realtime_cluster: Arc<RealtimeClusterBridge>,
    presence_runtime: Arc<DevicePresenceRuntime>,
    realtime_runtime: Arc<RealtimeDeliveryRuntime>,
    device_sync_state: DeviceSyncState,
    device_access_provider: Arc<dyn DeviceAccessProvider>,
}

impl DeviceRouteRegistration {
    pub(crate) fn new(
        node_id: String,
        realtime_cluster: Arc<RealtimeClusterBridge>,
        presence_runtime: Arc<DevicePresenceRuntime>,
        realtime_runtime: Arc<RealtimeDeliveryRuntime>,
        device_sync_state: DeviceSyncState,
        device_access_provider: Arc<dyn DeviceAccessProvider>,
    ) -> Self {
        Self {
            node_id,
            realtime_cluster,
            presence_runtime,
            realtime_runtime,
            device_sync_state,
            device_access_provider,
        }
    }

    #[rustfmt::skip]
    pub(crate) fn register_device(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<(), ApiError> {
        self.device_sync_state
            .ensure_device_kind_available(auth, device_id)?;
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
            self.presence_runtime
                .ensure_device_resume_not_required(auth, device_id)?;
        }
        self.ensure_device_access_registered(auth, device_id, session_id)?;
        self.presence_runtime
            .register_device(auth, device_id)?;
        self.realtime_runtime
            .ensure_device_state_for_principal_kind(
                tenant_id,
                principal_id,
                principal_kind,
                device_id,
            )?;
        self.device_sync_state.register_device(auth, device_id);
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
                .clear_device_disconnect_fence_for_current_session(
                    tenant_id,
                    principal_id,
                    principal_kind,
                    device_id,
                    session_id,
                )?;
        }
        Ok(())
    }

    pub(crate) fn prepare_active_device_route(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
        allow_session_takeover: bool,
    ) -> Result<(), ApiError> {
        self.device_sync_state
            .ensure_device_kind_available(auth, device_id)?;
        self.ensure_route_session_current(auth, device_id, auth.session_id.as_deref())?;
        self.register_device(auth, device_id, connection_kind, allow_session_takeover)?;
        Ok(())
    }

    pub(crate) fn current_active_device_route(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Option<super::RealtimeDeviceRoute> {
        self.realtime_cluster
            .resolve_device_route_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
            )
    }

    pub(crate) fn restore_active_device_route_if_current(
        &self,
        expected_current: &super::RealtimeDeviceRoute,
        restore_to: super::RealtimeDeviceRoute,
    ) -> Option<super::RealtimeDeviceRoute> {
        self.realtime_cluster
            .restore_device_route_if_current(expected_current, restore_to)
    }

    pub(crate) fn release_active_device_route_if_current_session(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) {
        if self
            .ensure_route_session_current(auth, device_id, auth.session_id.as_deref())
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

    pub(crate) fn ensure_active_device_route_current_session(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<(), ApiError> {
        self.device_sync_state
            .ensure_device_kind_available(auth, device_id)?;
        self.ensure_route_session_current(auth, device_id, auth.session_id.as_deref())
    }

    pub(crate) fn subscribe_active_device_route_epoch(
        &self,
        auth: &AppContext,
        device_id: &str,
    ) -> Result<watch::Receiver<u64>, ApiError> {
        self.device_sync_state
            .ensure_device_kind_available(auth, device_id)?;
        Ok(self
            .realtime_cluster
            .subscribe_device_route_epoch_for_principal_kind(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                auth.actor_kind.as_str(),
                device_id,
            ))
    }

    pub(crate) fn disconnect_active_device_route(
        &self,
        auth: &AppContext,
        device_id: &str,
        connection_kind: &str,
    ) -> Result<DisconnectActiveDeviceRouteOutcome, ApiError> {
        self.device_sync_state
            .ensure_device_kind_available(auth, device_id)?;
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

        self.prepare_active_device_route(auth, device_id, connection_kind, false)?;
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
        Ok(DisconnectActiveDeviceRouteOutcome::DeviceDisconnected)
    }

    fn ensure_route_session_current(
        &self,
        auth: &AppContext,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), ApiError> {
        let tenant_id = auth.tenant_id.as_str();
        let principal_id = auth.actor_id.as_str();
        self.realtime_cluster
            .ensure_route_session_current_for_principal_kind(
                tenant_id,
                principal_id,
                auth.actor_kind.as_str(),
                device_id,
                session_id,
            )?;
        Ok(())
    }

    fn ensure_device_access_registered(
        &self,
        auth: &AppContext,
        device_id: &str,
        session_id: Option<&str>,
    ) -> Result<(), ApiError> {
        if self
            .device_sync_state
            .has_registered_device(auth, device_id)
        {
            return Ok(());
        }

        let tenant_id = auth.tenant_id.as_str();
        let principal_id = auth.actor_id.as_str();
        self.device_access_provider
            .register_device(DeviceAccessRegistrationRequest {
                tenant_id: tenant_id.into(),
                device_id: device_id.into(),
                product_id: DEVICE_ROUTE_GATEWAY_DEVICE_PRODUCT_ID.into(),
                credential_kind: DEVICE_ROUTE_GATEWAY_DEVICE_CREDENTIAL_KIND.into(),
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
}
