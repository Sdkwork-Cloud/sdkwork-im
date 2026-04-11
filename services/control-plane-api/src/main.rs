use std::process::ExitCode;

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

async fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    if let Some(command) = args.next() {
        if command == "repair-social-runtime-dir" {
            let mut runtime_dir = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        let value = next_option_value(&mut args, "--runtime-dir")?;
                        runtime_dir = Some(std::path::PathBuf::from(value));
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: control-plane-api repair-social-runtime-dir [--runtime-dir <path>] [--json]"
                        );
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "Unknown argument for repair-social-runtime-dir: {argument}"
                        ));
                    }
                }
            }

            let Some(runtime_dir) = runtime_dir.or_else(control_plane_api::configured_runtime_dir)
            else {
                return Err(
                    "--runtime-dir is required for repair-social-runtime-dir when CRAW_CHAT_RUNTIME_DIR is unset"
                        .to_owned(),
                );
            };

            let report = control_plane_api::repair_social_runtime_dir(runtime_dir)
                .map_err(|error| format!("failed to repair social runtime-dir: {error}"))?;
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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:18081")
        .await
        .map_err(|error| format!("control-plane-api failed to bind local listener: {error}"))?;

    let app = match control_plane_api::configured_public_shared_channel_sync_trigger().map_err(
        |error| format!("standalone shared-channel sync trigger config should be valid: {error}"),
    )? {
        Some(trigger) => {
            control_plane_api::build_public_app_with_shared_channel_sync_trigger(trigger)
        }
        None => control_plane_api::build_public_app(),
    };

    axum::serve(listener, app)
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
