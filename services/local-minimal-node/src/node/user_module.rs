use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

use im_domain_core::message::Sender;
use im_platform_contracts::{
    ProviderDomain, ProviderHealthSnapshot, ProviderPluginDescriptor,
    UserModuleCreateOrBindRequest, UserModuleProvider, UserModuleUpdateProfileRequest,
    UserModuleUser,
};
use serde::Deserialize;

use super::*;

const USER_MODULE_PROVIDER_ENV: &str = "CRAW_CHAT_USER_MODULE_PROVIDER";
const USER_MODULE_EXTERNAL_CATALOG_PATH_ENV: &str = "CRAW_CHAT_USER_MODULE_EXTERNAL_CATALOG_PATH";
const USER_MODULE_EXTERNAL_SYSTEM_ENV: &str = "CRAW_CHAT_USER_MODULE_EXTERNAL_SYSTEM";

#[derive(Default)]
pub(super) struct LocalUserModuleProvider {
    users: Mutex<BTreeMap<String, BTreeMap<String, UserModuleUser>>>,
}

pub(super) fn build_default_user_module_provider() -> Arc<dyn UserModuleProvider> {
    match resolve_default_user_module_provider_mode() {
        DefaultUserModuleProviderMode::Local => Arc::new(LocalUserModuleProvider::default()),
        DefaultUserModuleProviderMode::External => {
            let default_external_system = resolve_external_user_module_system();
            match resolve_external_user_module_catalog_path() {
                Ok(catalog_path) => Arc::new(ExternalUserModuleProvider::new(
                    catalog_path,
                    default_external_system,
                )),
                Err(error_message) => Arc::new(UnavailableUserModuleProvider::new(
                    ExternalUserModuleProvider::descriptor_static(),
                    error_message,
                    BTreeMap::from([
                        ("providerMode".into(), "external".into()),
                        (
                            "configKey".into(),
                            USER_MODULE_EXTERNAL_CATALOG_PATH_ENV.into(),
                        ),
                    ]),
                )),
            }
        }
    }
}

enum DefaultUserModuleProviderMode {
    Local,
    External,
}

fn resolve_default_user_module_provider_mode() -> DefaultUserModuleProviderMode {
    match std::env::var(USER_MODULE_PROVIDER_ENV)
        .unwrap_or_else(|_| "local".into())
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "" | "local" => DefaultUserModuleProviderMode::Local,
        "external" => DefaultUserModuleProviderMode::External,
        other => {
            panic!("{USER_MODULE_PROVIDER_ENV} must be one of: local, external; received {other}")
        }
    }
}

fn resolve_external_user_module_catalog_path() -> Result<PathBuf, String> {
    std::env::var(USER_MODULE_EXTERNAL_CATALOG_PATH_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "{USER_MODULE_EXTERNAL_CATALOG_PATH_ENV} is required when {USER_MODULE_PROVIDER_ENV}=external"
            )
        })
}

fn resolve_external_user_module_system() -> String {
    std::env::var(USER_MODULE_EXTERNAL_SYSTEM_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "external-directory".into())
}

pub(super) async fn get_user_module_provider_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderHealthSnapshot>, ApiError> {
    let _auth = resolve_auth_context(&headers)?;
    Ok(Json(state.user_module_provider_health()))
}

pub(super) fn resolve_sender_from_auth_context(
    state: &AppState,
    auth: &AuthContext,
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

    let descriptor = state.user_module_provider.descriptor();
    let user = resolve_user(
        state.user_module_provider.as_ref(),
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
    )?;
    sender.metadata = user_metadata(&descriptor, &user);
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

    let descriptor = state.user_module_provider.descriptor();
    let user = resolve_user(state.user_module_provider.as_ref(), tenant_id, principal_id)?;
    Ok(("user".into(), user_metadata(&descriptor, &user)))
}

fn resolve_user(
    provider: &dyn UserModuleProvider,
    tenant_id: &str,
    user_id: &str,
) -> Result<UserModuleUser, ApiError> {
    let user = provider.get_user(tenant_id, user_id)?;
    let user = user.ok_or_else(|| {
        ApiError::bad_request(
            "user_module_user_not_found",
            format!("user not found in provider: {user_id}"),
        )
    })?;
    if user.disabled {
        return Err(ApiError::forbidden(
            "user_module_user_disabled",
            format!("user disabled in provider: {user_id}"),
        ));
    }

    Ok(user)
}

fn user_metadata(
    descriptor: &ProviderPluginDescriptor,
    user: &UserModuleUser,
) -> BTreeMap<String, String> {
    let mut metadata = user.attributes.clone();
    metadata.insert("displayName".into(), user.display_name.clone());
    metadata.insert("userModulePluginId".into(), descriptor.plugin_id.clone());
    metadata.insert(
        "userModuleProviderKind".into(),
        descriptor.provider_kind.clone(),
    );
    if let Some(external_system) = user.external_system.as_ref() {
        metadata.insert("externalSystem".into(), external_system.clone());
    }
    if let Some(external_principal_id) = user.external_principal_id.as_ref() {
        metadata.insert("externalPrincipalId".into(), external_principal_id.clone());
    }
    metadata
}

impl LocalUserModuleProvider {
    fn descriptor_static() -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "user-module-local",
            ProviderDomain::UserModule,
            "local",
            "Local User Module",
        )
        .with_default_selected(true)
        .with_required_capabilities(["query", "profile", "bind"])
    }

    fn get_or_default_user(&self, tenant_id: &str, user_id: &str) -> UserModuleUser {
        self.lock_users("get_or_default_user")
            .get(tenant_id)
            .and_then(|users| users.get(user_id))
            .cloned()
            .unwrap_or_else(|| UserModuleUser {
                tenant_id: tenant_id.into(),
                user_id: user_id.into(),
                display_name: user_id.into(),
                external_system: None,
                external_principal_id: None,
                attributes: BTreeMap::new(),
                disabled: false,
            })
    }

    fn upsert_user(&self, user: UserModuleUser) -> UserModuleUser {
        self.lock_users("upsert_user")
            .entry(user.tenant_id.clone())
            .or_default()
            .insert(user.user_id.clone(), user.clone());
        user
    }

    fn lock_users(
        &self,
        operation: &'static str,
    ) -> MutexGuard<'_, BTreeMap<String, BTreeMap<String, UserModuleUser>>> {
        match self.users.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!(
                    "warning: recovering poisoned local user-module store lock during {operation}"
                );
                poisoned.into_inner()
            }
        }
    }
}

#[derive(Debug)]
struct UnavailableUserModuleProvider {
    descriptor: ProviderPluginDescriptor,
    error_message: String,
    details: BTreeMap<String, String>,
}

impl UnavailableUserModuleProvider {
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
struct ExternalUserModuleProvider {
    catalog_path: PathBuf,
    default_external_system: String,
    overrides: Mutex<BTreeMap<String, BTreeMap<String, UserModuleUser>>>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExternalUserModuleCatalog {
    external_system: Option<String>,
    #[serde(default)]
    users: Vec<ExternalUserModuleCatalogUser>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExternalUserModuleCatalogUser {
    tenant_id: String,
    user_id: String,
    display_name: String,
    external_system: Option<String>,
    external_principal_id: Option<String>,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
    #[serde(default)]
    disabled: bool,
}

impl ExternalUserModuleProvider {
    fn new(catalog_path: PathBuf, default_external_system: String) -> Self {
        Self {
            catalog_path,
            default_external_system,
            overrides: Mutex::new(BTreeMap::new()),
        }
    }

    fn descriptor_static() -> ProviderPluginDescriptor {
        ProviderPluginDescriptor::new(
            "user-module-external",
            ProviderDomain::UserModule,
            "external",
            "External User Module",
        )
        .with_required_capabilities(["query", "bind", "external-mapping"])
    }

    fn load_catalog(&self) -> Result<ExternalUserModuleCatalog, ContractError> {
        let content = fs::read_to_string(&self.catalog_path).map_err(|error| {
            ContractError::Unavailable(format!(
                "external user-module catalog unreadable: {} ({error})",
                self.catalog_path.display()
            ))
        })?;
        serde_json::from_str(&content).map_err(|error| {
            ContractError::Unavailable(format!(
                "external user-module catalog invalid json: {} ({error})",
                self.catalog_path.display()
            ))
        })
    }

    fn external_system_for(
        &self,
        catalog: &ExternalUserModuleCatalog,
        entry: &ExternalUserModuleCatalogUser,
    ) -> String {
        entry
            .external_system
            .clone()
            .or_else(|| catalog.external_system.clone())
            .unwrap_or_else(|| self.default_external_system.clone())
    }

    fn map_catalog_user(
        &self,
        catalog: &ExternalUserModuleCatalog,
        entry: &ExternalUserModuleCatalogUser,
    ) -> UserModuleUser {
        UserModuleUser {
            tenant_id: entry.tenant_id.clone(),
            user_id: entry.user_id.clone(),
            display_name: entry.display_name.clone(),
            external_system: Some(self.external_system_for(catalog, entry)),
            external_principal_id: entry.external_principal_id.clone(),
            attributes: entry.attributes.clone(),
            disabled: entry.disabled,
        }
    }

    fn override_user(&self, user: UserModuleUser) -> UserModuleUser {
        self.lock_overrides("override_user")
            .entry(user.tenant_id.clone())
            .or_default()
            .insert(user.user_id.clone(), user.clone());
        user
    }

    fn override_user_lookup(&self, tenant_id: &str, user_id: &str) -> Option<UserModuleUser> {
        self.lock_overrides("override_user_lookup")
            .get(tenant_id)
            .and_then(|users| users.get(user_id))
            .cloned()
    }

    fn override_user_by_external_principal(
        &self,
        tenant_id: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Option<UserModuleUser> {
        self.lock_overrides("override_user_by_external_principal")
            .get(tenant_id)
            .and_then(|users| {
                users
                    .values()
                    .find(|user| {
                        user.external_system.as_deref() == Some(external_system)
                            && user.external_principal_id.as_deref() == Some(external_principal_id)
                    })
                    .cloned()
            })
    }

    fn catalog_user_by_id(
        &self,
        tenant_id: &str,
        user_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        let catalog = self.load_catalog()?;
        Ok(catalog
            .users
            .iter()
            .find(|entry| entry.tenant_id == tenant_id && entry.user_id == user_id)
            .map(|entry| self.map_catalog_user(&catalog, entry)))
    }

    fn catalog_user_by_external_principal(
        &self,
        tenant_id: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        let catalog = self.load_catalog()?;
        Ok(catalog
            .users
            .iter()
            .find(|entry| {
                entry.tenant_id == tenant_id
                    && self.external_system_for(&catalog, entry) == external_system
                    && entry.external_principal_id.as_deref() == Some(external_principal_id)
            })
            .map(|entry| self.map_catalog_user(&catalog, entry)))
    }

    fn placeholder_user(
        &self,
        tenant_id: &str,
        user_id: &str,
        display_name: String,
        external_system: Option<String>,
        external_principal_id: Option<String>,
    ) -> UserModuleUser {
        UserModuleUser {
            tenant_id: tenant_id.into(),
            user_id: user_id.into(),
            display_name,
            external_system: Some(
                external_system.unwrap_or_else(|| self.default_external_system.clone()),
            ),
            external_principal_id,
            attributes: BTreeMap::new(),
            disabled: false,
        }
    }

    fn lock_overrides(
        &self,
        operation: &'static str,
    ) -> MutexGuard<'_, BTreeMap<String, BTreeMap<String, UserModuleUser>>> {
        match self.overrides.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!(
                    "warning: recovering poisoned external user-module override store lock during {operation}"
                );
                poisoned.into_inner()
            }
        }
    }
}

impl UserModuleProvider for LocalUserModuleProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        Self::descriptor_static()
    }

    fn get_user(
        &self,
        tenant_id: &str,
        user_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        Ok(Some(self.get_or_default_user(tenant_id, user_id)))
    }

    fn batch_get_users(
        &self,
        tenant_id: &str,
        user_ids: &[String],
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        Ok(user_ids
            .iter()
            .map(|user_id| self.get_or_default_user(tenant_id, user_id))
            .collect())
    }

    fn search_users(
        &self,
        tenant_id: &str,
        keyword: &str,
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        let mut matches = self
            .lock_users("search_users")
            .get(tenant_id)
            .map(|items| {
                items
                    .values()
                    .filter(|user| {
                        user.user_id.contains(keyword) || user.display_name.contains(keyword)
                    })
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if matches.is_empty() && !keyword.trim().is_empty() {
            matches.push(self.get_or_default_user(tenant_id, keyword));
        }
        Ok(matches)
    }

    fn create_or_bind_user(
        &self,
        request: UserModuleCreateOrBindRequest,
    ) -> Result<UserModuleUser, ContractError> {
        Ok(self.upsert_user(UserModuleUser {
            tenant_id: request.tenant_id,
            user_id: request.user_id,
            display_name: request.display_name,
            external_system: request.external_system,
            external_principal_id: request.external_principal_id,
            attributes: BTreeMap::new(),
            disabled: false,
        }))
    }

    fn update_user_profile(
        &self,
        request: UserModuleUpdateProfileRequest,
    ) -> Result<UserModuleUser, ContractError> {
        let existing =
            self.get_or_default_user(request.tenant_id.as_str(), request.user_id.as_str());
        Ok(self.upsert_user(UserModuleUser {
            tenant_id: request.tenant_id,
            user_id: request.user_id,
            display_name: request
                .display_name
                .unwrap_or_else(|| existing.display_name.clone()),
            external_system: existing.external_system.clone(),
            external_principal_id: existing.external_principal_id.clone(),
            attributes: request.attributes,
            disabled: existing.disabled,
        }))
    }

    fn disable_user(&self, tenant_id: &str, user_id: &str) -> Result<bool, ContractError> {
        let mut user = self.get_or_default_user(tenant_id, user_id);
        user.disabled = true;
        self.upsert_user(user);
        Ok(true)
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        Ok(None)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        ProviderHealthSnapshot {
            plugin_id: "user-module-local".into(),
            status: "healthy".into(),
            checked_at: "2026-04-08T00:00:00Z".into(),
            details: BTreeMap::from([("providerKind".into(), "local".into())]),
        }
    }
}

impl UserModuleProvider for ExternalUserModuleProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        Self::descriptor_static()
    }

    fn get_user(
        &self,
        tenant_id: &str,
        user_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        if let Some(user) = self.override_user_lookup(tenant_id, user_id) {
            return Ok(Some(user));
        }
        self.catalog_user_by_id(tenant_id, user_id)
    }

    fn batch_get_users(
        &self,
        tenant_id: &str,
        user_ids: &[String],
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        user_ids.iter().try_fold(Vec::new(), |mut users, user_id| {
            if let Some(user) = self.get_user(tenant_id, user_id.as_str())? {
                users.push(user);
            }
            Ok(users)
        })
    }

    fn search_users(
        &self,
        tenant_id: &str,
        keyword: &str,
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        let catalog = self.load_catalog()?;
        let mut merged = BTreeMap::new();

        if let Some(users) = self.lock_overrides("search_users").get(tenant_id) {
            for user in users.values().filter(|user| {
                user.user_id.contains(keyword)
                    || user.display_name.contains(keyword)
                    || user
                        .external_principal_id
                        .as_deref()
                        .is_some_and(|principal_id| principal_id.contains(keyword))
            }) {
                merged.insert(user.user_id.clone(), user.clone());
            }
        }

        for entry in catalog.users.iter().filter(|entry| {
            entry.tenant_id == tenant_id
                && (entry.user_id.contains(keyword)
                    || entry.display_name.contains(keyword)
                    || entry
                        .external_principal_id
                        .as_deref()
                        .is_some_and(|principal_id| principal_id.contains(keyword)))
        }) {
            let user = self.map_catalog_user(&catalog, entry);
            merged.entry(user.user_id.clone()).or_insert(user);
        }

        Ok(merged.into_values().collect())
    }

    fn create_or_bind_user(
        &self,
        request: UserModuleCreateOrBindRequest,
    ) -> Result<UserModuleUser, ContractError> {
        let requested_external_system = request
            .external_system
            .clone()
            .unwrap_or_else(|| self.default_external_system.clone());
        let resolved = if let Some(external_principal_id) = request.external_principal_id.as_deref()
        {
            self.catalog_user_by_external_principal(
                request.tenant_id.as_str(),
                requested_external_system.as_str(),
                external_principal_id,
            )?
        } else {
            self.catalog_user_by_id(request.tenant_id.as_str(), request.user_id.as_str())?
        };

        let mut user = resolved.unwrap_or_else(|| {
            self.placeholder_user(
                request.tenant_id.as_str(),
                request.user_id.as_str(),
                request.display_name.clone(),
                request.external_system.clone(),
                request.external_principal_id.clone(),
            )
        });
        user.tenant_id = request.tenant_id;
        user.user_id = request.user_id;
        if user.display_name.trim().is_empty() {
            user.display_name = request.display_name;
        }
        if user.external_system.is_none() {
            user.external_system = Some(requested_external_system);
        }
        if user.external_principal_id.is_none() {
            user.external_principal_id = request.external_principal_id;
        }
        user.attributes
            .entry("bindingMode".into())
            .or_insert_with(|| "external".into());
        Ok(self.override_user(user))
    }

    fn update_user_profile(
        &self,
        request: UserModuleUpdateProfileRequest,
    ) -> Result<UserModuleUser, ContractError> {
        let existing = self
            .get_user(request.tenant_id.as_str(), request.user_id.as_str())?
            .unwrap_or_else(|| {
                self.placeholder_user(
                    request.tenant_id.as_str(),
                    request.user_id.as_str(),
                    request.user_id.clone(),
                    None,
                    None,
                )
            });
        let mut attributes = existing.attributes.clone();
        attributes.extend(request.attributes);
        Ok(self.override_user(UserModuleUser {
            tenant_id: request.tenant_id,
            user_id: request.user_id,
            display_name: request.display_name.unwrap_or(existing.display_name),
            external_system: existing.external_system,
            external_principal_id: existing.external_principal_id,
            attributes,
            disabled: existing.disabled,
        }))
    }

    fn disable_user(&self, tenant_id: &str, user_id: &str) -> Result<bool, ContractError> {
        let Some(mut user) = self.get_user(tenant_id, user_id)? else {
            return Ok(false);
        };
        user.disabled = true;
        self.override_user(user);
        Ok(true)
    }

    fn map_external_principal(
        &self,
        tenant_id: &str,
        external_system: &str,
        external_principal_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        if let Some(user) = self.override_user_by_external_principal(
            tenant_id,
            external_system,
            external_principal_id,
        ) {
            return Ok(Some(user));
        }
        self.catalog_user_by_external_principal(tenant_id, external_system, external_principal_id)
    }

    fn provider_health_snapshot(&self) -> ProviderHealthSnapshot {
        match self.load_catalog() {
            Ok(_) => ProviderHealthSnapshot {
                plugin_id: "user-module-external".into(),
                status: "healthy".into(),
                checked_at: "2026-04-09T00:00:00Z".into(),
                details: BTreeMap::from([
                    ("providerKind".into(), "external".into()),
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
                plugin_id: "user-module-external".into(),
                status: "unavailable".into(),
                checked_at: "2026-04-09T00:00:00Z".into(),
                details: BTreeMap::from([
                    ("providerKind".into(), "external".into()),
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

impl UserModuleProvider for UnavailableUserModuleProvider {
    fn descriptor(&self) -> ProviderPluginDescriptor {
        self.descriptor.clone()
    }

    fn get_user(
        &self,
        _tenant_id: &str,
        _user_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
        self.unavailable()
    }

    fn batch_get_users(
        &self,
        _tenant_id: &str,
        _user_ids: &[String],
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        self.unavailable()
    }

    fn search_users(
        &self,
        _tenant_id: &str,
        _keyword: &str,
    ) -> Result<Vec<UserModuleUser>, ContractError> {
        self.unavailable()
    }

    fn create_or_bind_user(
        &self,
        _request: UserModuleCreateOrBindRequest,
    ) -> Result<UserModuleUser, ContractError> {
        self.unavailable()
    }

    fn update_user_profile(
        &self,
        _request: UserModuleUpdateProfileRequest,
    ) -> Result<UserModuleUser, ContractError> {
        self.unavailable()
    }

    fn disable_user(&self, _tenant_id: &str, _user_id: &str) -> Result<bool, ContractError> {
        self.unavailable()
    }

    fn map_external_principal(
        &self,
        _tenant_id: &str,
        _external_system: &str,
        _external_principal_id: &str,
    ) -> Result<Option<UserModuleUser>, ContractError> {
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
