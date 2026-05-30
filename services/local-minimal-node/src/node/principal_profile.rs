use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use im_domain_core::message::Sender;
use im_platform_contracts::{
    PrincipalProfile, PrincipalProfileProvider, ProviderDomain, ProviderHealthSnapshot,
    ProviderPluginDescriptor,
};
use serde::Deserialize;

use super::*;

const PRINCIPAL_PROFILE_PROVIDER_ENV: &str = "CRAW_CHAT_PRINCIPAL_PROFILE_PROVIDER";
const PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH_ENV: &str =
    "CRAW_CHAT_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH";
const PRINCIPAL_PROFILE_EXTERNAL_SYSTEM_ENV: &str = "CRAW_CHAT_PRINCIPAL_PROFILE_EXTERNAL_SYSTEM";

pub(super) fn build_default_principal_profile_provider() -> Arc<dyn PrincipalProfileProvider> {
    match resolve_default_principal_profile_provider_mode() {
        Ok(DefaultPrincipalProfileProviderMode::UpstreamContext) => {
            Arc::new(UpstreamContextPrincipalProfileProvider::default())
        }
        Ok(DefaultPrincipalProfileProviderMode::ExternalCatalog) => {
            let default_external_system = resolve_external_principal_profile_system();
            match resolve_external_principal_profile_catalog_path() {
                Ok(catalog_path) => Arc::new(ExternalCatalogPrincipalProfileProvider::new(
                    catalog_path,
                    default_external_system,
                )),
                Err(error_message) => Arc::new(UnavailablePrincipalProfileProvider::new(
                    ExternalCatalogPrincipalProfileProvider::descriptor_static(),
                    error_message,
                    BTreeMap::from([
                        ("providerMode".into(), "external-catalog".into()),
                        (
                            "configKey".into(),
                            PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH_ENV.into(),
                        ),
                    ]),
                )),
            }
        }
        Err((configured_value, error_message)) => {
            Arc::new(UnavailablePrincipalProfileProvider::new(
                UnavailablePrincipalProfileProvider::invalid_config_descriptor_static(),
                error_message,
                BTreeMap::from([
                    ("configKey".into(), PRINCIPAL_PROFILE_PROVIDER_ENV.into()),
                    ("configuredValue".into(), configured_value),
                ]),
            ))
        }
    }
}

enum DefaultPrincipalProfileProviderMode {
    UpstreamContext,
    ExternalCatalog,
}

fn resolve_default_principal_profile_provider_mode()
-> Result<DefaultPrincipalProfileProviderMode, (String, String)> {
    let configured_value =
        std::env::var(PRINCIPAL_PROFILE_PROVIDER_ENV).unwrap_or_else(|_| "upstream-context".into());
    match configured_value.trim().to_ascii_lowercase().as_str() {
        "" | "upstream-context" => Ok(DefaultPrincipalProfileProviderMode::UpstreamContext),
        "external-catalog" => Ok(DefaultPrincipalProfileProviderMode::ExternalCatalog),
        other => Err((
            other.into(),
            format!(
                "{PRINCIPAL_PROFILE_PROVIDER_ENV} must be one of: upstream-context, external-catalog; received {other}"
            ),
        )),
    }
}

fn resolve_external_principal_profile_catalog_path() -> Result<PathBuf, String> {
    std::env::var(PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "{PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH_ENV} is required when {PRINCIPAL_PROFILE_PROVIDER_ENV}=external-catalog"
            )
        })
}

fn resolve_external_principal_profile_system() -> String {
    std::env::var(PRINCIPAL_PROFILE_EXTERNAL_SYSTEM_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "external-directory".into())
}

pub(super) async fn get_principal_profile_provider_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderHealthSnapshot>, ApiError> {
    let _auth = resolve_app_context(&headers)?;
    Ok(Json(state.principal_profile_provider_health()))
}

pub(super) fn resolve_sender_from_auth_context(
    state: &AppState,
    auth: &AppContext,
) -> Result<Sender, ApiError> {
    let mut sender = Sender {
        id: auth.actor_id.clone(),
        kind: auth.actor_kind.clone(),
        member_id: None,
        device_id: auth.device_id.clone(),
        session_id: auth.session_id.clone(),
        metadata: BTreeMap::new(),
    };

    if auth.actor_kind != "user" {
        return Ok(sender);
    }

    let descriptor = state.principal_profile_provider.descriptor();
    let profile = resolve_profile(
        state.principal_profile_provider.as_ref(),
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
    )?;
    sender.metadata = profile_metadata(&descriptor, &profile);
    Ok(sender)
}

pub(super) fn resolve_member_principal(
    state: &AppState,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> Result<(String, BTreeMap<String, String>), ApiError> {
    if principal_kind != "user" {
        return Ok((principal_kind.into(), BTreeMap::new()));
    }

    let descriptor = state.principal_profile_provider.descriptor();
    let profile = resolve_profile(
        state.principal_profile_provider.as_ref(),
        tenant_id,
        principal_id,
        principal_kind,
    )?;
    Ok(("user".into(), profile_metadata(&descriptor, &profile)))
}

pub(super) fn ensure_active_user(
    state: &AppState,
    tenant_id: &str,
    user_id: &str,
) -> Result<PrincipalProfile, ApiError> {
    resolve_profile(
        state.principal_profile_provider.as_ref(),
        tenant_id,
        user_id,
        "user",
    )
}

pub(super) fn ensure_active_principal(
    state: &AppState,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> Result<(), ApiError> {
    if principal_kind != "user" {
        return Ok(());
    }

    resolve_profile(
        state.principal_profile_provider.as_ref(),
        tenant_id,
        principal_id,
        principal_kind,
    )?;
    Ok(())
}

fn resolve_profile(
    provider: &dyn PrincipalProfileProvider,
    tenant_id: &str,
    principal_id: &str,
    principal_kind: &str,
) -> Result<PrincipalProfile, ApiError> {
    let profile = provider.get_profile(tenant_id, principal_id, principal_kind)?;
    let profile = profile.ok_or_else(|| {
        ApiError::bad_request(
            "principal_profile_not_found",
            format!("principal profile not found: {principal_id}"),
        )
    })?;
    if profile.inactive {
        return Err(ApiError::forbidden(
            "principal_profile_inactive",
            format!("principal profile inactive: {principal_id}"),
        ));
    }

    Ok(profile)
}

fn profile_metadata(
    descriptor: &ProviderPluginDescriptor,
    profile: &PrincipalProfile,
) -> BTreeMap<String, String> {
    let mut metadata = profile.attributes.clone();
    metadata.insert("displayName".into(), profile.display_name.clone());
    metadata.insert(
        "principalProfilePluginId".into(),
        descriptor.plugin_id.clone(),
    );
    metadata.insert(
        "principalProfileProviderKind".into(),
        descriptor.provider_kind.clone(),
    );
    if let Some(external_system) = profile.external_system.as_ref() {
        metadata.insert("externalSystem".into(), external_system.clone());
    }
    if let Some(external_principal_id) = profile.external_principal_id.as_ref() {
        metadata.insert("externalPrincipalId".into(), external_principal_id.clone());
    }
    metadata
}

#[derive(Default)]
struct UpstreamContextPrincipalProfileProvider;

impl UpstreamContextPrincipalProfileProvider {
    fn descriptor_static() -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-upstream-context",
            ProviderDomain::PrincipalProfile,
            "upstream-context",
            "Upstream Context Principal Profile",
        )
        .with_default_selected(true)
        .with_required_capabilities(["read", "profile"])
    }

    fn default_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        _principal_kind: &str,
    ) -> PrincipalProfile {
        PrincipalProfile {
            tenant_id: tenant_id.into(),
            principal_id: principal_id.into(),
            display_name: principal_id.into(),
            external_system: None,
            external_principal_id: None,
            attributes: BTreeMap::new(),
            inactive: false,
        }
    }
}

impl PrincipalProfileProvider for UpstreamContextPrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        Self::descriptor_static()
    }

    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(Some(self.default_profile(
            tenant_id,
            principal_id,
            principal_kind,
        )))
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        Ok(principal_ids
            .iter()
            .map(|principal_id| self.default_profile(tenant_id, principal_id, principal_kind))
            .collect())
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        if keyword.trim().is_empty() {
            return Ok(Vec::new());
        }
        Ok(vec![self.default_profile(
            tenant_id,
            keyword,
            principal_kind,
        )])
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        Ok(None)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot {
            plugin_id: "principal-profile-upstream-context".into(),
            status: "healthy".into(),
            checked_at: "2026-04-08T00:00:00Z".into(),
            details: BTreeMap::from([("providerKind".into(), "upstream-context".into())]),
        }
    }
}

#[derive(Debug)]
struct UnavailablePrincipalProfileProvider {
    descriptor: ProviderPluginDescriptor,
    error_message: String,
    details: BTreeMap<String, String>,
}

impl UnavailablePrincipalProfileProvider {
    fn invalid_config_descriptor_static() -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-invalid-config",
            ProviderDomain::PrincipalProfile,
            "invalid-config",
            "Invalid Principal Profile Configuration",
        )
        .with_required_capabilities(["read", "profile"])
    }

    fn new(
        descriptor: ProviderPluginDescriptor,
        error_message: String,
        details: BTreeMap<String, String>,
    ) -> Self {
        Self {
            descriptor,
            error_message,
            details,
        }
    }

    fn unavailable<T>(&self) -> Result<T, ContractError> {
        Err(ContractError::Unavailable(self.error_message.clone()))
    }
}

#[derive(Debug)]
struct ExternalCatalogPrincipalProfileProvider {
    catalog_path: PathBuf,
    default_external_system: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExternalPrincipalProfileCatalog {
    external_system: Option<String>,
    #[serde(default)]
    profiles: Vec<ExternalPrincipalProfileCatalogEntry>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExternalPrincipalProfileCatalogEntry {
    tenant_id: String,
    principal_id: String,
    #[serde(default = "default_principal_kind")]
    principal_kind: String,
    display_name: String,
    external_system: Option<String>,
    external_principal_id: Option<String>,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
    #[serde(default)]
    inactive: bool,
}

fn default_principal_kind() -> String {
    "user".into()
}

impl ExternalCatalogPrincipalProfileProvider {
    fn new(catalog_path: PathBuf, default_external_system: String) -> Self {
        Self {
            catalog_path,
            default_external_system,
        }
    }

    fn descriptor_static() -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "principal-profile-external-catalog",
            ProviderDomain::PrincipalProfile,
            "external-catalog",
            "External Catalog Principal Profile",
        )
        .with_required_capabilities(["read", "profile", "external-mapping"])
    }

    fn load_catalog(&self) -> Result<ExternalPrincipalProfileCatalog, ContractError> {
        let content = fs::read_to_string(&self.catalog_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "external principal-profile catalog unreadable: {} ({error})",
                self.catalog_path.display()
            ))
        })?;
        serde_json::from_str(&content).map_err(|error| {
            ContractError::Unavailable(format!(
                "external principal-profile catalog invalid json: {} ({error})",
                self.catalog_path.display()
            ))
        })
    }

    fn external_system_for(
        &self,
        catalog: &ExternalPrincipalProfileCatalog,
        entry: &ExternalPrincipalProfileCatalogEntry,
    ) -> String {
        entry
            .external_system
            .clone()
            .or_else(|| catalog.external_system.clone())
            .unwrap_or_else(|| self.default_external_system.clone())
    }

    fn map_catalog_profile(
        &self,
        catalog: &ExternalPrincipalProfileCatalog,
        entry: &ExternalPrincipalProfileCatalogEntry,
    ) -> PrincipalProfile {
        PrincipalProfile {
            tenant_id: entry.tenant_id.clone(),
            principal_id: entry.principal_id.clone(),
            display_name: entry.display_name.clone(),
            external_system: Some(self.external_system_for(catalog, entry)),
            external_principal_id: entry.external_principal_id.clone(),
            attributes: entry.attributes.clone(),
            inactive: entry.inactive,
        }
    }

    fn catalog_profile_by_id(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        let catalog = self.load_catalog()?;
        Ok(catalog
            .profiles
            .iter()
            .find(|entry| {
                entry.tenant_id == tenant_id
                    && entry.principal_id == principal_id
                    && entry.principal_kind == principal_kind
            })
            .map(|entry| self.map_catalog_profile(&catalog, entry)))
    }

    fn catalog_profile_by_external_principal(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        let catalog = self.load_catalog()?;
        Ok(catalog
            .profiles
            .iter()
            .find(|entry| {
                entry.tenant_id == tenant_id
                    && entry.principal_kind == principal_kind
                    && self.external_system_for(&catalog, entry) == external_system
                    && entry.external_principal_id.as_deref() == Some(external_principal_id)
            })
            .map(|entry| self.map_catalog_profile(&catalog, entry)))
    }
}

impl PrincipalProfileProvider for ExternalCatalogPrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        Self::descriptor_static()
    }

    fn get_profile(
        &self,
        tenant_id: &str,
        principal_id: &str,
        principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        self.catalog_profile_by_id(tenant_id, principal_id, principal_kind)
    }

    fn batch_get_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        principal_ids
            .iter()
            .try_fold(Vec::new(), |mut profiles, principal_id| {
                if let Some(profile) =
                    self.get_profile(tenant_id, principal_id.as_str(), principal_kind)?
                {
                    profiles.push(profile);
                }
                Ok(profiles)
            })
    }

    fn search_profiles(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        let catalog = self.load_catalog()?;
        Ok(catalog
            .profiles
            .iter()
            .filter(|entry| {
                entry.tenant_id == tenant_id
                    && entry.principal_kind == principal_kind
                    && (entry.principal_id.contains(keyword)
                        || entry.display_name.contains(keyword)
                        || entry
                            .external_principal_id
                            .as_deref()
                            .is_some_and(|principal_id| principal_id.contains(keyword)))
            })
            .map(|entry| self.map_catalog_profile(&catalog, entry))
            .collect())
    }

    fn map_external_principal(
        &self,
        tenant_id: &str,
        principal_kind: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        self.catalog_profile_by_external_principal(
            tenant_id,
            principal_kind,
            external_system,
            external_principal_id,
        )
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        match self.load_catalog() {
            Ok(_) => ProviderHealthSnapshot {
                plugin_id: "principal-profile-external-catalog".into(),
                status: "healthy".into(),
                checked_at: "2026-04-09T00:00:00Z".into(),
                details: BTreeMap::from([
                    ("providerKind".into(), "external-catalog".into()),
                    (
                        "catalogPath".into(),
                        self.catalog_path.display().to_string(),
                    ),
                    (
                        "defaultExternalSystem".into(),
                        self.default_external_system.clone(),
                    ),
                ]),
            },
            Err(error) => ProviderHealthSnapshot {
                plugin_id: "principal-profile-external-catalog".into(),
                status: "unavailable".into(),
                checked_at: "2026-04-09T00:00:00Z".into(),
                details: BTreeMap::from([
                    ("providerKind".into(), "external-catalog".into()),
                    (
                        "catalogPath".into(),
                        self.catalog_path.display().to_string(),
                    ),
                    ("error".into(), format!("{error:?}")),
                ]),
            },
        }
    }
}

impl PrincipalProfileProvider for UnavailablePrincipalProfileProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        self.descriptor.clone()
    }

    fn get_profile(
        &self,
        _tenant_id: &str,
        _principal_id: &str,
        _principal_kind: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        self.unavailable()
    }

    fn batch_get_profiles(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _principal_ids: &[String],
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        self.unavailable()
    }

    fn search_profiles(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _keyword: &str,
    ) -> Result<Vec<PrincipalProfile>, ContractError> {
        self.unavailable()
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<PrincipalProfile>, ContractError> {
        self.unavailable()
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        let mut details = self.details.clone();
        details.insert("providerKind".into(), self.descriptor.provider_kind.clone());
        details.insert("error".into(), self.error_message.clone());
        ProviderHealthSnapshot {
            plugin_id: self.descriptor.plugin_id.clone(),
            status: "unavailable".into(),
            checked_at: "2026-04-09T00:00:00Z".into(),
            details,
        }
    }
}
