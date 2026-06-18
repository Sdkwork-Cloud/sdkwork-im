use std::process::ExitCode;
use std::sync::Arc;

use social_service::SocialRuntime;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_COMMS_SOCIAL_SERVICE_BIND_ADDR";
const LEGACY_BIND_ADDR_ENV: &str = "SDKWORK_IM_SOCIAL_SERVICE_BIND_ADDR";
const RUNTIME_DIR_ENV: &str = "SDKWORK_IM_RUNTIME_DIR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18092";

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

async fn run() -> Result<(), String> {
    let bind_addr = std::env::var(BIND_ADDR_ENV)
        .or_else(|_| std::env::var(LEGACY_BIND_ADDR_ENV))
        .unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("comms-social-service failed to bind {bind_addr}: {error}"))?;

    let social_runtime = build_social_runtime()?;
    let postgres_state = social_service::try_postgres_app_state_from_database_url_env();
    let app =
        social_service::build_public_app_with_postgres_extension(social_runtime, postgres_state);

    tracing::info!(
        "comms-social-service listening on {}",
        listener.local_addr().map_err(|error| error.to_string())?
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await
        .map_err(|error| format!("comms-social-service server should run: {error}"))?;
    Ok(())
}

fn build_social_runtime() -> Result<Arc<SocialRuntime>, String> {
    match std::env::var(RUNTIME_DIR_ENV) {
        Ok(runtime_dir) if !runtime_dir.trim().is_empty() => Ok(Arc::new(
            SocialRuntime::from_runtime_dir(runtime_dir.as_str()),
        )),
        _ => Ok(Arc::new(SocialRuntime::default())),
    }
}
