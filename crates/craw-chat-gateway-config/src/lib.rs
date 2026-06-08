use std::env;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

const DEFAULT_GATEWAY_BIND_ADDR: &str = "127.0.0.1:18079";
const DEFAULT_APPBASE_APP_API_UPSTREAM: &str = "http://127.0.0.1:18090";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GatewayRuntimeMode {
    Split,
    Embedded,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUpstreamConfig {
    pub service_id: String,
    pub base_url: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebGatewayConfig {
    pub bind_addr: String,
    pub runtime_mode: GatewayRuntimeMode,
    pub strict_startup: bool,
    pub upstreams: Vec<ServiceUpstreamConfig>,
}

impl WebGatewayConfig {
    pub fn from_env() -> Self {
        let bind_addr = first_env_value(&[
            "SDKWORK_CHAT_SERVER_BIND",
            "CRAW_CHAT_WEB_GATEWAY_BIND",
            "CRAW_CHAT_SERVER_BIND_ADDRESS",
        ])
        .unwrap_or_else(|| DEFAULT_GATEWAY_BIND_ADDR.to_owned());
        let runtime_mode = resolve_runtime_mode_from_env();
        Self::with_bind_addr_and_runtime_mode(bind_addr, runtime_mode)
    }

    pub fn from_server_config_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|error| {
            format!(
                "failed to read server config file {}: {error}",
                path.display()
            )
        })?;
        let bind_addr = parse_server_config_bind_addr(&content)
            .unwrap_or_else(|| DEFAULT_GATEWAY_BIND_ADDR.to_owned());
        Ok(Self::with_bind_addr_and_runtime_mode(
            bind_addr,
            GatewayRuntimeMode::Split,
        ))
    }

    pub fn upstream_base_url(&self, service_id: &str) -> Option<&str> {
        self.upstreams
            .iter()
            .find(|upstream| upstream.service_id == service_id)
            .map(|upstream| upstream.base_url.as_str())
    }

    fn with_bind_addr_and_runtime_mode(
        bind_addr: String,
        runtime_mode: GatewayRuntimeMode,
    ) -> Self {
        let upstreams = match runtime_mode {
            GatewayRuntimeMode::Split => default_split_upstreams(),
            GatewayRuntimeMode::Embedded => default_embedded_upstreams(),
        };
        Self {
            bind_addr,
            runtime_mode,
            strict_startup: false,
            upstreams,
        }
    }
}

fn resolve_runtime_mode_from_env() -> GatewayRuntimeMode {
    first_env_value(&[
        "SDKWORK_CHAT_WEB_GATEWAY_RUNTIME_MODE",
        "CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE",
    ])
    .map(|value| value.trim().to_ascii_lowercase())
    .filter(|value| !value.is_empty())
    .map(|value| match value.as_str() {
        "embedded" | "unified" | "local" => GatewayRuntimeMode::Embedded,
        _ => GatewayRuntimeMode::Split,
    })
    .unwrap_or(GatewayRuntimeMode::Split)
}

fn first_env_value(names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        env::var(name)
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    })
}

fn parse_server_config_bind_addr(content: &str) -> Option<String> {
    let mut in_network_block = false;
    let mut in_server_block = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_network_block = false;
            in_server_block = trimmed == "[server]";
            continue;
        }

        let is_top_level = !line.starts_with(' ') && !line.starts_with('\t');
        if is_top_level {
            if trimmed == "network:" {
                in_network_block = true;
                in_server_block = false;
                continue;
            }
            if let Some(value) =
                parse_toml_key_value(trimmed, &["bind_address", "bind", "bindAddress"])
            {
                return Some(value);
            }
            in_network_block = false;
        }

        if in_network_block && trimmed.starts_with("bindAddress:") {
            let value = trimmed
                .trim_start_matches("bindAddress:")
                .trim()
                .trim_matches('"')
                .trim_matches('\'');
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }

        if in_server_block
            && let Some(value) =
                parse_toml_key_value(trimmed, &["bind_address", "bind", "bindAddress"])
        {
            return Some(value);
        }
    }

    None
}

fn parse_toml_key_value(trimmed: &str, keys: &[&str]) -> Option<String> {
    let (key, raw_value) = trimmed.split_once('=')?;
    if !keys.iter().any(|candidate| key.trim() == *candidate) {
        return None;
    }
    let value = raw_value
        .split('#')
        .next()
        .unwrap_or(raw_value)
        .trim()
        .trim_matches('"')
        .trim_matches('\'');
    if value.is_empty() {
        return None;
    }
    Some(value.to_owned())
}

pub fn default_split_upstreams() -> Vec<ServiceUpstreamConfig> {
    let appbase_upstream = default_appbase_app_api_upstream();
    vec![
        service_upstream("sdkwork-appbase-app-api", appbase_upstream.as_str()),
        service_upstream("session-gateway", "http://127.0.0.1:18080"),
        service_upstream("control-plane-api", "http://127.0.0.1:18081"),
        service_upstream("conversation-runtime", "http://127.0.0.1:18082"),
        service_upstream("projection-service", "http://127.0.0.1:18083"),
        service_upstream("streaming-service", "http://127.0.0.1:18084"),
        service_upstream("sdkwork-rtc-signaling-service", "http://127.0.0.1:18085"),
        service_upstream("media-service", "http://127.0.0.1:18086"),
        service_upstream("notification-service", "http://127.0.0.1:18087"),
        service_upstream("automation-service", "http://127.0.0.1:18088"),
        service_upstream("audit-service", "http://127.0.0.1:18089"),
        service_upstream("ops-service", "http://127.0.0.1:18091"),
    ]
}

pub fn default_embedded_upstreams() -> Vec<ServiceUpstreamConfig> {
    Vec::new()
}

fn default_appbase_app_api_upstream() -> String {
    env::var("CRAW_CHAT_APPBASE_APP_API_UPSTREAM")
        .or_else(|_| {
            env::var("SDKWORK_APPBASE_APP_API_BIND_ADDR")
                .map(|bind_addr| format!("http://{}", bind_addr.trim()))
        })
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_APPBASE_APP_API_UPSTREAM.to_owned())
}

pub fn service_upstream(service_id: &str, base_url: &str) -> ServiceUpstreamConfig {
    ServiceUpstreamConfig {
        service_id: service_id.to_owned(),
        base_url: base_url.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::{GatewayRuntimeMode, WebGatewayConfig};

    fn gateway_config_env_guard() -> std::sync::MutexGuard<'static, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    struct ScopedEnvVar {
        name: &'static str,
        previous: Option<String>,
    }

    impl ScopedEnvVar {
        fn set(name: &'static str, value: &str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::set_var(name, value);
            }
            Self { name, previous }
        }

        fn remove(name: &'static str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::remove_var(name);
            }
            Self { name, previous }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                unsafe {
                    std::env::set_var(self.name, previous);
                }
                return;
            }

            unsafe {
                std::env::remove_var(self.name);
            }
        }
    }

    fn unique_temp_root(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("craw_chat_gateway_config_{prefix}_{unique}"))
    }

    #[test]
    fn test_web_gateway_config_loads_bind_addr_from_server_yaml() {
        let temp_root = unique_temp_root("server_yaml");
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let server_yaml_path = temp_root.join("server.yaml");
        fs::write(
            &server_yaml_path,
            r#"instance:
  name: "default"

network:
  bindAddress: "127.0.0.1:28080"
"#,
        )
        .expect("server yaml should be written");

        let config = WebGatewayConfig::from_server_config_file(&server_yaml_path)
            .expect("server yaml should produce a gateway config");
        assert_eq!(config.bind_addr, "127.0.0.1:28080");

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn test_web_gateway_config_requires_server_yaml_when_loading_file_mode() {
        let temp_root = unique_temp_root("missing_server_yaml");
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let missing_path = temp_root.join("server.yaml");

        let error = WebGatewayConfig::from_server_config_file(&missing_path)
            .expect_err("missing config file should return an error");
        assert!(
            error.contains("server config file"),
            "missing config error should mention the server config file: {error}"
        );

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn test_web_gateway_config_loads_bind_addr_from_chat_toml() {
        let temp_root = unique_temp_root("chat_toml");
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let chat_toml_path = temp_root.join("chat.toml");
        fs::write(
            &chat_toml_path,
            r#"[runtime]
deployment_mode = "server"
app_code = "chat"

[server]
bind_address = "127.0.0.1:38080"
"#,
        )
        .expect("chat toml should be written");

        let config = WebGatewayConfig::from_server_config_file(&chat_toml_path)
            .expect("chat toml should produce a gateway config");
        assert_eq!(config.bind_addr, "127.0.0.1:38080");

        let _ = fs::remove_dir_all(temp_root);
    }

    #[test]
    fn test_standard_sdkwork_chat_server_bind_env_takes_precedence() {
        let _guard = gateway_config_env_guard();
        let _standard_bind = ScopedEnvVar::set("SDKWORK_CHAT_SERVER_BIND", "127.0.0.1:39080");
        let _legacy_bind = ScopedEnvVar::set("CRAW_CHAT_WEB_GATEWAY_BIND", "127.0.0.1:18079");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.bind_addr, "127.0.0.1:39080");
    }

    #[test]
    fn test_web_gateway_config_defaults_to_split_upstreams() {
        let _guard = gateway_config_env_guard();
        let _runtime_mode = ScopedEnvVar::remove("CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE");
        let _appbase_upstream = ScopedEnvVar::remove("CRAW_CHAT_APPBASE_APP_API_UPSTREAM");
        let _appbase_bind_addr = ScopedEnvVar::remove("SDKWORK_APPBASE_APP_API_BIND_ADDR");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Split);
        assert_eq!(
            config.upstream_base_url("sdkwork-appbase-app-api"),
            Some("http://127.0.0.1:18090")
        );
        assert_eq!(
            config.upstream_base_url("session-gateway"),
            Some("http://127.0.0.1:18080")
        );
        assert_eq!(
            config.upstream_base_url("ops-service"),
            Some("http://127.0.0.1:18091")
        );
    }

    #[test]
    fn test_web_gateway_embedded_mode_uses_no_external_upstreams() {
        let _guard = gateway_config_env_guard();
        let _runtime_mode = ScopedEnvVar::set("CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE", "embedded");
        let _appbase_upstream = ScopedEnvVar::set(
            "CRAW_CHAT_APPBASE_APP_API_UPSTREAM",
            "http://127.0.0.1:19090/",
        );
        let _appbase_bind_addr = ScopedEnvVar::remove("SDKWORK_APPBASE_APP_API_BIND_ADDR");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Embedded);
        assert!(config.upstreams.is_empty());
        assert_eq!(config.upstream_base_url("sdkwork-appbase-app-api"), None);
        assert_eq!(config.upstream_base_url("session-gateway"), None);
        assert_eq!(config.upstream_base_url("conversation-runtime"), None);
        assert_eq!(config.upstream_base_url("ops-service"), None);
    }

    #[test]
    fn test_web_gateway_embedded_mode_ignores_appbase_bind_addr_env() {
        let _guard = gateway_config_env_guard();
        let _runtime_mode = ScopedEnvVar::set("CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE", "local");
        let _appbase_upstream = ScopedEnvVar::remove("CRAW_CHAT_APPBASE_APP_API_UPSTREAM");
        let _appbase_bind_addr =
            ScopedEnvVar::set("SDKWORK_APPBASE_APP_API_BIND_ADDR", "127.0.0.1:28090");

        let config = WebGatewayConfig::from_env();

        assert_eq!(config.runtime_mode, GatewayRuntimeMode::Embedded);
        assert!(config.upstreams.is_empty());
        assert_eq!(config.upstream_base_url("sdkwork-appbase-app-api"), None);
    }
}
