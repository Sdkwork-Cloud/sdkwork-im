use anyhow::{Context, Result};
use std::env;

const DEFAULT_RUNTIME_BIND_ADDR: &str = "127.0.0.1:0";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandaloneConfig {
    pub runtime_bind_addr: String,
    pub admin_proxy_target: String,
    pub admin_sandbox_enabled: bool,
}

#[derive(Clone, Debug, Default)]
pub struct StandaloneConfigLoader;

impl StandaloneConfigLoader {
    pub fn from_env() -> Result<(Self, StandaloneConfig)> {
        Ok((
            Self,
            StandaloneConfig {
                runtime_bind_addr: resolve_runtime_bind_addr(),
                admin_proxy_target: resolve_admin_proxy_target()
                    .context("failed to resolve desktop admin proxy target")?,
                admin_sandbox_enabled: resolve_admin_sandbox_enabled(),
            },
        ))
    }
}

fn resolve_runtime_bind_addr() -> String {
    env::var("SDKWORK_DESKTOP_RUNTIME_BIND")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_RUNTIME_BIND_ADDR.to_owned())
}

fn resolve_admin_proxy_target() -> Result<String> {
    let candidate = ["SDKWORK_ADMIN_PROXY_TARGET", "SDKWORK_ADMIN_BIND"]
        .iter()
        .find_map(|key| env::var(key).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    match candidate {
        Some(value) => normalize_upstream_url(value.as_str()),
        None => Ok(String::new()),
    }
}

fn normalize_upstream_url(value: &str) -> Result<String> {
    let trimmed = value.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        anyhow::bail!("desktop admin proxy target cannot be empty");
    }

    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        return Ok(trimmed.to_owned());
    }

    Ok(format!("http://{trimmed}"))
}

fn resolve_admin_sandbox_enabled() -> bool {
    ["SDKWORK_ADMIN_SANDBOX", "SDKWORK_ADMIN_SANDBOX_MODE"]
        .iter()
        .find_map(|key| env::var(key).ok())
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}
