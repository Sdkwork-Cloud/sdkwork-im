use std::path::PathBuf;
use std::process::ExitCode;

use craw_chat_gateway_config::WebGatewayConfig;
use craw_chat_gateway_observability::{
    build_startup_summary_with_registry, format_startup_summary,
};

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
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
    let base_url = format!("http://{local_addr}");
    let registry = web_gateway::build_gateway_registry()?;
    println!(
        "{}",
        format_startup_summary(&build_startup_summary_with_registry(
            &config, &registry, base_url,
        ))
    );

    axum::serve(
        listener,
        web_gateway::build_app_with_registry(config, registry),
    )
    .await
    .map_err(|error| format!("web-gateway server should run: {error}"))?;
    Ok(())
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
