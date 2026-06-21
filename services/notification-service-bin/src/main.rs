use std::process::ExitCode;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_NOTIFICATION_SERVICE_BIND_ADDR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18087";

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
    let bind_addr = std::env::var(BIND_ADDR_ENV).unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("notification-service failed to bind local listener: {error}"))?;

    axum::serve(listener, sdkwork_router_im_notification_app_api::build_public_app())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await
        .map_err(|error| format!("notification-service server should run: {error}"))?;
    Ok(())
}
