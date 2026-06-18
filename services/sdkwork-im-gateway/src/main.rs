use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use sdkwork_api_config::StandaloneConfigLoader;
use sdkwork_api_product_runtime::{
    ProductSiteDirs, RouterProductRuntimeOptions, build_product_runtime_router,
};
use sdkwork_im_gateway_config::WebGatewayConfig;
use sdkwork_im_gateway_observability::{
    build_startup_summary_with_registry, format_startup_summary,
};

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            tracing::error!("{error}");
            ExitCode::FAILURE
        }
    }
}

enum StartupMode {
    Run(WebGatewayConfig),
    ExitSuccess,
}

async fn run() -> Result<(), String> {
    let StartupMode::Run(config) = resolve_startup_mode()? else {
        return Ok(());
    };
    let listener = tokio::net::TcpListener::bind(config.bind_addr.as_str())
        .await
        .map_err(|error| format!("sdkwork-im-gateway failed to bind listener: {error}"))?;
    let local_addr = listener
        .local_addr()
        .map_err(|error| format!("sdkwork-im-gateway failed to resolve listener addr: {error}"))?;
    let base_url = format!("http://{}", display_listener_addr(local_addr));
    let registry = web_gateway::build_gateway_registry()?;
    let product_runtime_router = build_gateway_product_runtime_router(base_url.as_str()).await?;
    println!(
        "{}",
        format_startup_summary(&build_startup_summary_with_registry(
            &config,
            &registry,
            base_url.clone(),
        ))
    );

    axum::serve(
        listener,
        web_gateway::build_app_with_registry_and_product_runtime(
            config,
            registry,
            Some(product_runtime_router),
        ),
    )
    .with_graceful_shutdown(async {
        tokio::signal::ctrl_c().await.ok();
    })
    .await
    .map_err(|error| format!("sdkwork-im-gateway server should run: {error}"))?;
    Ok(())
}

async fn build_gateway_product_runtime_router(base_url: &str) -> Result<axum::Router, String> {
    let (_loader, mut standalone_config) =
        StandaloneConfigLoader::from_env().map_err(|error| error.to_string())?;

    if !has_explicit_portal_api_base_url() {
        standalone_config.portal_api_base_url = base_url.trim_end_matches('/').to_owned();
    }

    let site_dirs = resolve_product_site_dirs();
    build_product_runtime_router(
        standalone_config,
        RouterProductRuntimeOptions::desktop(site_dirs),
    )
    .await
    .map_err(|error| error.to_string())
}

fn resolve_product_site_dirs() -> ProductSiteDirs {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .to_path_buf();

    let admin_site_dir =
        resolve_site_dir_from_env(&["SDKWORK_IM_ADMIN_SITE_DIR", "SDKWORK_IM_ADMIN_SITE_DIR"])
            .unwrap_or_else(|| repo_root.join("apps").join("sdkwork-im-admin").join("dist"));
    let portal_site_dir =
        resolve_site_dir_from_env(&["SDKWORK_IM_PORTAL_SITE_DIR", "SDKWORK_IM_PORTAL_SITE_DIR"])
            .unwrap_or_else(|| {
                repo_root
                    .join("apps")
                    .join("sdkwork-im-portal")
                    .join("dist")
            });

    ProductSiteDirs::new(admin_site_dir, portal_site_dir)
}

fn resolve_site_dir_from_env(env_names: &[&str]) -> Option<PathBuf> {
    env_names.iter().find_map(|env_name| {
        std::env::var(env_name)
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    })
}

fn has_explicit_portal_api_base_url() -> bool {
    [
        "SDKWORK_IM_PORTAL_API_BASE_URL",
        "SDKWORK_PORTAL_API_BASE_URL",
        "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
        "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
        "SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND",
    ]
    .iter()
    .any(|env_name| {
        std::env::var(env_name)
            .ok()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
    })
}

fn display_listener_addr(addr: SocketAddr) -> String {
    let host = match addr.ip() {
        IpAddr::V4(ip) if ip.is_unspecified() => "127.0.0.1".to_owned(),
        IpAddr::V4(ip) => ip.to_string(),
        IpAddr::V6(ip) if ip.is_unspecified() => "[::1]".to_owned(),
        IpAddr::V6(ip) => format!("[{ip}]"),
    };

    format!("{host}:{}", addr.port())
}

fn resolve_startup_mode() -> Result<StartupMode, String> {
    let mut args = std::env::args().skip(1);
    let mut config_path: Option<PathBuf> = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--config" => {
                let Some(path) = args.next() else {
                    return Err("missing value for --config".to_owned());
                };
                config_path = Some(PathBuf::from(path));
            }
            "-h" | "--help" => {
                println!("Usage: sdkwork-im-server [--config <chat.toml>]");
                println!(
                    "Start the Sdkwork IM unified gateway using env defaults or a chat.toml file."
                );
                return Ok(StartupMode::ExitSuccess);
            }
            unknown => {
                return Err(format!("unsupported argument: {unknown}"));
            }
        }
    }

    let config = match config_path {
        Some(path) => WebGatewayConfig::from_server_config_file(path)?,
        None => WebGatewayConfig::from_env(),
    };
    Ok(StartupMode::Run(config))
}

#[cfg(test)]
mod tests {
    use super::has_explicit_portal_api_base_url;
    use std::sync::{Mutex, OnceLock};

    fn env_guard() -> std::sync::MutexGuard<'static, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("web gateway main env guard should not be poisoned")
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
            match &self.previous {
                Some(value) => unsafe {
                    std::env::set_var(self.name, value);
                },
                None => unsafe {
                    std::env::remove_var(self.name);
                },
            }
        }
    }

    #[test]
    fn application_public_http_url_env_counts_as_explicit_portal_binding() {
        let _guard = env_guard();
        let _portal = ScopedEnvVar::remove("SDKWORK_IM_PORTAL_API_BASE_URL");
        let _sdkwork_portal = ScopedEnvVar::remove("SDKWORK_PORTAL_API_BASE_URL");
        let _application_public_http = ScopedEnvVar::set(
            "SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL",
            "https://im.sdkwork.com",
        );
        let _bind = ScopedEnvVar::remove("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND");

        assert!(has_explicit_portal_api_base_url());
    }
}
