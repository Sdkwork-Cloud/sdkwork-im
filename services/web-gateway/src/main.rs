use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use craw_chat_gateway_config::WebGatewayConfig;
use craw_chat_gateway_observability::{
    build_startup_summary_with_registry, format_startup_summary,
};
use sdkwork_api_config::StandaloneConfigLoader;
use sdkwork_api_product_runtime::{
    ProductSiteDirs, RouterProductRuntimeOptions, build_product_runtime_router,
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
        .map_err(|error| format!("web-gateway failed to bind listener: {error}"))?;
    let local_addr = listener
        .local_addr()
        .map_err(|error| format!("web-gateway failed to resolve listener addr: {error}"))?;
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
    .map_err(|error| format!("web-gateway server should run: {error}"))?;
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

    let admin_site_dir = resolve_site_dir_from_env("CRAW_CHAT_ADMIN_SITE_DIR")
        .unwrap_or_else(|| repo_root.join("apps").join("craw-chat-admin").join("dist"));
    let portal_site_dir = resolve_site_dir_from_env("CRAW_CHAT_PORTAL_SITE_DIR")
        .unwrap_or_else(|| repo_root.join("apps").join("craw-chat-portal").join("dist"));

    ProductSiteDirs::new(admin_site_dir, portal_site_dir)
}

fn resolve_site_dir_from_env(env_name: &str) -> Option<PathBuf> {
    std::env::var(env_name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn has_explicit_portal_api_base_url() -> bool {
    [
        "CRAW_CHAT_PORTAL_API_BASE_URL",
        "SDKWORK_PORTAL_API_BASE_URL",
        "CRAW_CHAT_BIND_ADDR",
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
                println!("Usage: craw-chat-server [--config <server.yaml>]");
                println!(
                    "Start the Craw Chat unified gateway using env defaults or a server.yaml file."
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
