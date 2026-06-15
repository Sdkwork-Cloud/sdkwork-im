use std::process::ExitCode;

const BIND_ADDR_ENV: &str = "SDKWORK_IM_CONTROL_PLANE_API_BIND_ADDR";
const DEFAULT_BIND_ADDR: &str = "127.0.0.1:18081";

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
            eprintln!("{error}");
            tracing::error!("{error}");
            ExitCode::FAILURE
        }
    }
}

async fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    if let Some(command) = args.next() {
        if command == "print-openapi" {
            let body = serde_json::to_string_pretty(&control_plane_api::render_openapi_document())
                .map_err(|error| format!("failed to serialize control-plane OpenAPI: {error}"))?;
            println!("{body}");
            return Ok(());
        }

        if command == "repair-social-runtime_dir" {
            let mut runtime_dir = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime_dir" => {
                        let value = next_option_value(&mut args, "--runtime_dir")?;
                        runtime_dir = Some(std::path::PathBuf::from(value));
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: control-plane-api repair-social-runtime_dir [--runtime_dir <path>] [--json]"
                        );
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "Unknown argument for repair-social-runtime_dir: {argument}"
                        ));
                    }
                }
            }

            let Some(runtime_dir) = runtime_dir.or_else(control_plane_api::configured_runtime_dir)
            else {
                return Err(
                    "--runtime_dir is required for repair-social-runtime_dir when SDKWORK_IM_RUNTIME_DIR is unset"
                        .to_owned(),
                );
            };

            let report = control_plane_api::repair_social_runtime_dir(runtime_dir)
                .map_err(|error| format!("failed to repair social runtime_dir: {error}"))?;
            if json_output {
                let body = serde_json::to_string_pretty(&report).map_err(|error| {
                    format!("social runtime repair report should serialize: {error}")
                })?;
                println!("{body}");
            } else {
                println!(
                    "{}",
                    control_plane_api::format_social_runtime_dir_repair(&report)
                );
            }
            return Ok(());
        }

        return Err(format!("Unknown command for control-plane-api: {command}"));
    }

    let bind_addr = std::env::var(BIND_ADDR_ENV).unwrap_or_else(|_| DEFAULT_BIND_ADDR.to_owned());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("control-plane-api failed to bind local listener: {error}"))?;

    let app = match control_plane_api::configured_dual_token_shared_channel_sync_trigger().map_err(
        |error| format!("standalone shared-channel sync trigger config should be valid: {error}"),
    )? {
        Some(trigger) => {
            control_plane_api::build_public_app_with_shared_channel_sync_trigger(trigger)
        }
        None => control_plane_api::build_public_app(),
    };

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
        })
        .await
        .map_err(|error| format!("control-plane-api server should run: {error}"))?;
    Ok(())
}

fn next_option_value(
    args: &mut impl Iterator<Item = String>,
    option: &str,
) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("{option} requires a value"))
}
