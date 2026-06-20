mod config;

use std::net::{IpAddr, SocketAddr};
use std::path::Path;

use axum::routing::get;
use axum::Router;
use config::{load_gateway_config, resolve_config_path, resolve_gateway_config, ResolvedGatewayConfig};
use sdkwork_api_config::StandaloneConfigLoader;
use sdkwork_api_product_runtime::{
    ProductSiteDirs, RouterProductRuntimeOptions, build_product_runtime_router,
};
use sdkwork_im_gateway_config::WebGatewayConfig;
use sdkwork_im_gateway_observability::{
    build_startup_summary_with_registry, format_startup_summary,
};
use tower_http::cors::{Any, CorsLayer};
use web_gateway::{build_app_with_registry_and_product_runtime, build_gateway_registry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let args: Vec<String> = std::env::args().collect();
    let config_path = resolve_config_path(&args)?;
    let file_config = load_gateway_config(Path::new(&config_path))?;
    let gateway_config = resolve_gateway_config(file_config)?;

    let bind_addr: SocketAddr = gateway_config
        .bind
        .parse()
        .map_err(|error| format!("invalid bind address `{}`: {error}", gateway_config.bind))?;
    let base_url = format!("http://{}", display_listener_addr(bind_addr));
    apply_collapsed_standalone_urls(&base_url, &bind_addr);

    let _im_db = sdkwork_im_database_pool::bootstrap_im_database_from_env()
        .await
        .map_err(|error| format!("failed to bootstrap IM database lifecycle: {error}"))?;

    let iam_router = sdkwork_router_iam_app_api::build_sdkwork_appbase_app_api_router()
        .await
        .map_err(|error| format!("failed to build embedded IAM router: {error}"))?;

    let web_config = WebGatewayConfig::from_env();
    let registry = build_gateway_registry()?;
    let product_runtime_router = build_gateway_product_runtime_router(base_url.as_str()).await?;
    println!(
        "{}",
        format_startup_summary(&build_startup_summary_with_registry(
            &web_config,
            &registry,
            base_url.clone(),
        ))
    );

    let im_router = build_app_with_registry_and_product_runtime(
        web_config,
        registry,
        Some(product_runtime_router),
    );

    let app = Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .merge(iam_router)
        .merge(im_router)
        .layer(build_cors_layer(&gateway_config));

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    tracing::info!(
        target: "sdkwork.im",
        event = "im.standalone_gateway.listen",
        service = %gateway_config.service_name,
        environment = %gateway_config.environment,
        bind = %bind_addr,
        "listening"
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

fn apply_collapsed_standalone_urls(base_url: &str, bind_addr: &SocketAddr) {
    let bind = format!("{}:{}", bind_addr.ip(), bind_addr.port());
    let websocket_url = format!("ws://{}", display_listener_addr(*bind_addr));
    for (key, value) in [
        ("SDKWORK_IM_APPLICATION_PUBLIC_INGRESS_BIND", bind.as_str()),
        ("SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL", base_url),
        ("SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL", websocket_url.as_str()),
        ("SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL", base_url),
        ("SDKWORK_API_GATEWAY_BASE_URL", base_url),
        ("SDKWORK_API_GATEWAY_BIND", bind.as_str()),
    ] {
        unsafe {
            std::env::set_var(key, value);
        }
    }
}

async fn build_gateway_product_runtime_router(base_url: &str) -> Result<Router, String> {
    let (_loader, mut standalone_config) =
        StandaloneConfigLoader::from_env().map_err(|error| error.to_string())?;
    standalone_config.portal_api_base_url = base_url.trim_end_matches('/').to_owned();

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");
    let site_dirs = ProductSiteDirs::new(
        repo_root.join("apps").join("sdkwork-im-admin").join("dist"),
        repo_root.join("apps").join("sdkwork-im-portal").join("dist"),
    );
    build_product_runtime_router(
        standalone_config,
        RouterProductRuntimeOptions::desktop(site_dirs),
    )
    .await
    .map_err(|error| error.to_string())
}

fn build_cors_layer(config: &ResolvedGatewayConfig) -> CorsLayer {
    if config.allow_any_origin {
        return CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);
    }

    let mut layer = CorsLayer::new().allow_methods(Any).allow_headers(Any);
    for origin in &config.allowed_origins {
        if let Ok(parsed) = origin.parse::<axum::http::HeaderValue>() {
            layer = layer.allow_origin(parsed);
        }
    }
    layer
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

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}
