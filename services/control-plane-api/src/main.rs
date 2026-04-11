#[tokio::main]
async fn main() {
    let mut args = std::env::args().skip(1);
    if let Some(command) = args.next() {
        if command == "repair-social-runtime-dir" {
            let mut runtime_dir = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        let value = args.next().expect("--runtime-dir requires a value");
                        runtime_dir = Some(std::path::PathBuf::from(value));
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: control-plane-api repair-social-runtime-dir [--runtime-dir <path>] [--json]"
                        );
                        return;
                    }
                    _ => {
                        eprintln!("Unknown argument for repair-social-runtime-dir: {argument}");
                        std::process::exit(1);
                    }
                }
            }

            let Some(runtime_dir) = runtime_dir.or_else(control_plane_api::configured_runtime_dir)
            else {
                eprintln!(
                    "--runtime-dir is required for repair-social-runtime-dir when CRAW_CHAT_RUNTIME_DIR is unset"
                );
                std::process::exit(1);
            };

            match control_plane_api::repair_social_runtime_dir(runtime_dir) {
                Ok(report) => {
                    if json_output {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&report)
                                .expect("social runtime repair report should serialize")
                        );
                    } else {
                        println!(
                            "{}",
                            control_plane_api::format_social_runtime_dir_repair(&report)
                        );
                    }
                }
                Err(error) => {
                    eprintln!("failed to repair social runtime-dir: {error}");
                    std::process::exit(1);
                }
            }
            return;
        }

        eprintln!("Unknown command for control-plane-api: {command}");
        std::process::exit(1);
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:18081")
        .await
        .expect("control-plane-api should bind local listener");

    let app = match control_plane_api::configured_public_shared_channel_sync_trigger()
        .expect("standalone shared-channel sync trigger config should be valid")
    {
        Some(trigger) => {
            control_plane_api::build_public_app_with_shared_channel_sync_trigger(trigger)
        }
        None => control_plane_api::build_public_app(),
    };

    axum::serve(listener, app)
        .await
        .expect("control-plane-api server should run");
}
