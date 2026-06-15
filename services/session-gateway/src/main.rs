use std::{net::SocketAddr, process::ExitCode};

const DEFAULT_SESSION_GATEWAY_BIND_ADDR: &str = "127.0.0.1:18080";

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
    let bind_addr = resolve_bind_addr()?;
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .map_err(|error| {
            format!("session-gateway failed to bind listener at {bind_addr}: {error}")
        })?;

    axum::serve(listener, session_gateway::build_public_app())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await
        .map_err(|error| format!("session-gateway server should run: {error}"))?;
    Ok(())
}

fn resolve_bind_addr() -> Result<SocketAddr, String> {
    let session_gateway_bind_addr = std::env::var("SESSION_GATEWAY_BIND_ADDR").ok();
    let workspace_bind_addr = std::env::var("SDKWORK_IM_BIND_ADDR").ok();

    resolve_bind_addr_from_env(
        session_gateway_bind_addr.as_deref(),
        workspace_bind_addr.as_deref(),
    )
}

fn resolve_bind_addr_from_env(
    session_gateway_bind_addr: Option<&str>,
    workspace_bind_addr: Option<&str>,
) -> Result<SocketAddr, String> {
    let bind_addr = [session_gateway_bind_addr, workspace_bind_addr]
        .into_iter()
        .flatten()
        .map(str::trim)
        .find(|value| !value.is_empty())
        .unwrap_or(DEFAULT_SESSION_GATEWAY_BIND_ADDR);

    bind_addr
        .parse()
        .map_err(|error| format!("invalid session-gateway bind address `{bind_addr}`: {error}"))
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use super::resolve_bind_addr_from_env;

    #[test]
    fn resolve_bind_addr_prefers_service_specific_env_value() {
        let resolved = resolve_bind_addr_from_env(Some("0.0.0.0:28080"), Some("127.0.0.1:18090"))
            .expect("service-specific bind addr should parse");

        assert_eq!(
            resolved,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 28080)
        );
    }

    #[test]
    fn resolve_bind_addr_falls_back_to_workspace_bind_addr() {
        let resolved = resolve_bind_addr_from_env(None, Some("127.0.0.1:18090"))
            .expect("workspace bind addr should parse");

        assert_eq!(
            resolved,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18090)
        );
    }

    #[test]
    fn resolve_bind_addr_uses_default_when_no_env_values_are_present() {
        let resolved =
            resolve_bind_addr_from_env(None, None).expect("default bind addr should parse");

        assert_eq!(
            resolved,
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18080)
        );
    }

    #[test]
    fn resolve_bind_addr_rejects_invalid_values() {
        let error = resolve_bind_addr_from_env(Some("not-a-socket-addr"), None)
            .expect_err("invalid bind addr should fail");

        assert!(
            error.contains("invalid session-gateway bind address"),
            "unexpected error: {error}"
        );
    }
}
