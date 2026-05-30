use std::collections::BTreeSet;

use axum::http::HeaderMap;
use craw_chat_ccp_core::{CcpActor, CcpAuthority, CcpSender};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppContext {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub user_id: String,
    pub session_id: Option<String>,
    pub app_id: Option<String>,
    pub environment: Option<String>,
    pub deployment_mode: Option<String>,
    pub auth_level: Option<String>,
    pub data_scope: BTreeSet<String>,
    pub permission_scope: BTreeSet<String>,
    pub actor_id: String,
    pub actor_kind: String,
    pub device_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppContextError {
    code: &'static str,
    message: String,
}

impl AppContextError {
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    fn missing(message: impl Into<String>) -> Self {
        Self {
            code: "app_context_missing",
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AppContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppContextError {}

impl AppContext {
    pub fn has_permission(&self, permission: &str) -> bool {
        if permission.trim().is_empty() {
            return false;
        }

        if self.permission_scope.contains("*")
            || self.permission_scope.contains("tenant.admin")
            || self.permission_scope.contains(permission)
        {
            return true;
        }

        let segments: Vec<&str> = permission.split('.').collect();
        for index in 1..segments.len() {
            let wildcard = format!("{}.*", segments[..index].join("."));
            if self.permission_scope.contains(wildcard.as_str()) {
                return true;
            }
        }

        false
    }

    pub fn ccp_authority(&self) -> CcpAuthority {
        CcpAuthority::new(
            self.tenant_id.clone(),
            CcpSender::new(
                self.actor_id.clone(),
                self.device_id.clone(),
                self.session_id.clone(),
            ),
            CcpActor::new(self.actor_id.clone(), self.actor_kind.clone()),
        )
    }
}

pub fn resolve_app_context(
    headers: &HeaderMap,
) -> Result<AppContext, AppContextError> {
    resolve_app_context_projection(headers)
}

pub fn resolve_app_context_projection(
    headers: &HeaderMap,
) -> Result<AppContext, AppContextError> {
    let tenant_id = resolve_header(headers, &["x-sdkwork-tenant-id"])?;
    let user_id = resolve_header(headers, &["x-sdkwork-user-id"])?;
    let session_id = resolve_optional_header(headers, &["x-sdkwork-session-id"]);
    let app_id = resolve_optional_header(headers, &["x-sdkwork-app-id"]);
    let environment = resolve_optional_header(headers, &["x-sdkwork-environment"]);
    let deployment_mode = resolve_optional_header(headers, &["x-sdkwork-deployment-mode"]);
    let auth_level = resolve_optional_header(headers, &["x-sdkwork-auth-level"]);
    let actor_id = resolve_optional_header(headers, &["x-sdkwork-actor-id"])
        .unwrap_or_else(|| user_id.clone());
    let actor_kind = resolve_optional_header(headers, &["x-sdkwork-actor-kind"])
        .unwrap_or_else(|| "user".to_owned());
    let organization_id = resolve_optional_header(headers, &["x-sdkwork-organization-id"]);
    let device_id = resolve_optional_header(headers, &["x-sdkwork-device-id"]);
    let data_scope = resolve_scope_from_headers(headers, &["x-sdkwork-data-scope"]);
    let permission_scope = resolve_scope_from_headers(headers, &["x-sdkwork-permission-scope"]);

    Ok(AppContext {
        tenant_id,
        organization_id,
        user_id,
        session_id,
        app_id,
        environment,
        deployment_mode,
        auth_level,
        data_scope,
        permission_scope,
        actor_id,
        actor_kind,
        device_id,
    })
}

fn resolve_header(headers: &HeaderMap, names: &[&str]) -> Result<String, AppContextError> {
    resolve_optional_header(headers, names).ok_or_else(|| {
        AppContextError::missing(format!("missing sdkwork app context header: {}", names[0]))
    })
}

fn resolve_optional_header(headers: &HeaderMap, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        headers
            .get(*name)
            .and_then(|value| value.to_str().ok())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn resolve_scope_from_headers(headers: &HeaderMap, names: &[&str]) -> BTreeSet<String> {
    let mut scope = BTreeSet::new();

    for name in names {
        if let Some(value) = headers.get(*name).and_then(|value| value.to_str().ok()) {
            append_scope_str(&mut scope, value);
        }
    }

    scope
}

fn append_scope_str(scope: &mut BTreeSet<String>, raw: &str) {
    for token in raw.split(|ch: char| ch.is_whitespace() || ch == ',') {
        let item = token.trim();
        if !item.is_empty() {
            scope.insert(item.to_owned());
        }
    }
}
