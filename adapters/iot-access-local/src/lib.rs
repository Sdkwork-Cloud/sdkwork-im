use std::collections::BTreeMap;

use craw_chat_contract_core::ContractError;
use im_platform_contracts::{
    DeviceAccessOwnerBindingRequest, DeviceAccessProvider, DeviceAccessRegistration,
    DeviceAccessRegistrationRequest, ProviderDomain, ProviderHealthSnapshot,
    ProviderPluginDescriptor,
};
use im_time::utc_now_rfc3339_millis;

pub const LOCAL_IOT_ACCESS_PLUGIN_ID: &str = "iot-access-local";
const DEFAULT_ASSIGNED_PROTOCOLS: [&str; 2] = ["mqtt", "xiaozhi"];
const DEFAULT_SECRET_PREFIX: &str = "local-device-secret";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalDeviceAccessProviderConfig {
    pub assigned_protocols: Vec<String>,
    pub credential_secret_prefix: String,
}

impl Default for LocalDeviceAccessProviderConfig {
    fn default() -> Self {
        let assigned_protocols = std::env::var("CRAW_CHAT_IOT_ACCESS_LOCAL_PROTOCOLS")
            .ok()
            .map(|value| {
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| {
                DEFAULT_ASSIGNED_PROTOCOLS
                    .iter()
                    .map(|value| (*value).to_owned())
                    .collect()
            });
        Self {
            assigned_protocols,
            credential_secret_prefix: std::env::var("CRAW_CHAT_IOT_ACCESS_LOCAL_SECRET_PREFIX")
                .ok()
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| DEFAULT_SECRET_PREFIX.into()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LocalDeviceAccessProvider {
    config: LocalDeviceAccessProviderConfig,
}

impl LocalDeviceAccessProvider {
    pub fn new(config: LocalDeviceAccessProviderConfig) -> Self {
        Self { config }
    }

    fn descriptor_with_defaults(&self) -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            LOCAL_IOT_ACCESS_PLUGIN_ID,
            ProviderDomain::IotAccess,
            "local",
            "Local Device Access",
        )
        .with_default_selected(true)
        .with_required_capabilities(["registry", "credential", "binding", "twin"])
        .with_optional_capabilities(["session", "owner-binding", "protocol-assignment"])
    }

    fn require_non_empty(&self, field_name: &str, value: &str) -> Result<(), ContractError> {
        if value.trim().is_empty() {
            return Err(ContractError::Conflict(format!(
                "local device access requires non-empty {field_name}"
            )));
        }
        Ok(())
    }
}

impl DeviceAccessProvider for LocalDeviceAccessProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        self.descriptor_with_defaults()
    }

    fn register_device(
        &self,
        request: DeviceAccessRegistrationRequest,
    ) -> Result<DeviceAccessRegistration, ContractError> {
        self.require_non_empty("tenant_id", request.tenant_id.as_str())?;
        self.require_non_empty("device_id", request.device_id.as_str())?;
        self.require_non_empty("product_id", request.product_id.as_str())?;
        self.require_non_empty("credential_kind", request.credential_kind.as_str())?;

        Ok(DeviceAccessRegistration {
            tenant_id: request.tenant_id.clone(),
            device_id: request.device_id.clone(),
            product_id: request.product_id,
            owner_principal_id: request.owner_principal_id,
            credential_secret: Some(format!(
                "{}:{}:{}:{}",
                self.config.credential_secret_prefix,
                request.tenant_id,
                request.device_id,
                request.credential_kind
            )),
            assigned_protocols: self.config.assigned_protocols.clone(),
        })
    }

    fn bind_owner(&self, request: DeviceAccessOwnerBindingRequest) -> Result<bool, ContractError> {
        self.require_non_empty("tenant_id", request.tenant_id.as_str())?;
        self.require_non_empty("device_id", request.device_id.as_str())?;
        self.require_non_empty("owner_principal_id", request.owner_principal_id.as_str())?;
        Ok(true)
    }

    fn disable_device(&self, tenant_id: &str, device_id: &str) -> Result<bool, ContractError> {
        self.require_non_empty("tenant_id", tenant_id)?;
        self.require_non_empty("device_id", device_id)?;
        Ok(true)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        let mut details = BTreeMap::new();
        details.insert("providerKind".into(), "local".into());
        details.insert(
            "assignedProtocols".into(),
            self.config.assigned_protocols.join(","),
        );
        details.insert(
            "credentialSecretPrefix".into(),
            self.config.credential_secret_prefix.clone(),
        );
        ProviderHealthSnapshot {
            plugin_id: LOCAL_IOT_ACCESS_PLUGIN_ID.into(),
            status: "healthy".into(),
            checked_at: utc_now_rfc3339_millis(),
            details,
        }
    }
}
