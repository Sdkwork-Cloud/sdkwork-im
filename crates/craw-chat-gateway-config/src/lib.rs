use std::env;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

const DEFAULT_GATEWAY_BIND_ADDR: &str = "127.0.0.1:18079";

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
        Self::with_bind_addr(
            env::var("CRAW_CHAT_WEB_GATEWAY_BIND")
                .ok()
                .map(|value| value.trim().to_owned())
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| DEFAULT_GATEWAY_BIND_ADDR.to_owned()),
        )
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
        Ok(Self::with_bind_addr(bind_addr))
    }

    pub fn upstream_base_url(&self, service_id: &str) -> Option<&str> {
        self.upstreams
            .iter()
            .find(|upstream| upstream.service_id == service_id)
            .map(|upstream| upstream.base_url.as_str())
    }

    fn with_bind_addr(bind_addr: String) -> Self {
        Self {
            bind_addr,
            runtime_mode: GatewayRuntimeMode::Split,
            strict_startup: false,
            upstreams: default_split_upstreams(),
        }
    }
}

fn parse_server_config_bind_addr(content: &str) -> Option<String> {
    let mut in_network_block = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let is_top_level = !line.starts_with(' ') && !line.starts_with('\t');
        if is_top_level {
            in_network_block = trimmed == "network:";
            continue;
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
    }

    None
}

pub fn default_split_upstreams() -> Vec<ServiceUpstreamConfig> {
    vec![
        service_upstream("session-gateway", "http://127.0.0.1:18080"),
        service_upstream("control-plane-api", "http://127.0.0.1:18081"),
        service_upstream("conversation-runtime", "http://127.0.0.1:18082"),
        service_upstream("projection-service", "http://127.0.0.1:18083"),
        service_upstream("streaming-service", "http://127.0.0.1:18084"),
        service_upstream("rtc-signaling-service", "http://127.0.0.1:18085"),
        service_upstream("media-service", "http://127.0.0.1:18086"),
        service_upstream("notification-service", "http://127.0.0.1:18087"),
        service_upstream("automation-service", "http://127.0.0.1:18088"),
        service_upstream("audit-service", "http://127.0.0.1:18089"),
        service_upstream("ops-service", "http://127.0.0.1:18091"),
    ]
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
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::WebGatewayConfig;

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
}
