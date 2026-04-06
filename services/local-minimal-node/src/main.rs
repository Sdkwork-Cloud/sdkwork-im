#[tokio::main]
async fn main() {
    let mut args = std::env::args().skip(1);
    if let Some(command) = args.next() {
        if command == "inspect-runtime-dir" {
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
                            "Usage: local-minimal-node inspect-runtime-dir [--runtime-dir <path>] [--json]"
                        );
                        return;
                    }
                    _ => {
                        eprintln!("Unknown argument for inspect-runtime-dir: {argument}");
                        std::process::exit(1);
                    }
                }
            }

            let inspection = local_minimal_node::inspect_runtime_dir(
                runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir),
            );

            if json_output {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&inspection)
                        .expect("runtime-dir inspection should serialize")
                );
            } else {
                println!(
                    "{}",
                    local_minimal_node::format_runtime_dir_inspection(&inspection)
                );
            }
            return;
        }

        if command == "repair-runtime-dir" {
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
                            "Usage: local-minimal-node repair-runtime-dir [--runtime-dir <path>] [--json]"
                        );
                        return;
                    }
                    _ => {
                        eprintln!("Unknown argument for repair-runtime-dir: {argument}");
                        std::process::exit(1);
                    }
                }
            }

            let repair = local_minimal_node::repair_runtime_dir(
                runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir),
            );

            if json_output {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&repair)
                        .expect("runtime-dir repair report should serialize")
                );
            } else {
                println!("{}", local_minimal_node::format_runtime_dir_repair(&repair));
            }
            return;
        }

        if command == "restore-runtime-dir" {
            let mut runtime_dir = None;
            let mut backup_dir = None;
            let mut expected_preview_fingerprint = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        let value = args.next().expect("--runtime-dir requires a value");
                        runtime_dir = Some(std::path::PathBuf::from(value));
                    }
                    "--backup-dir" => {
                        let value = args.next().expect("--backup-dir requires a value");
                        backup_dir = Some(std::path::PathBuf::from(value));
                    }
                    "--expected-preview-fingerprint" => {
                        let value = args
                            .next()
                            .expect("--expected-preview-fingerprint requires a value");
                        expected_preview_fingerprint = Some(value);
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: local-minimal-node restore-runtime-dir --backup-dir <path> [--runtime-dir <path>] [--expected-preview-fingerprint <value>] [--json]"
                        );
                        return;
                    }
                    _ => {
                        eprintln!("Unknown argument for restore-runtime-dir: {argument}");
                        std::process::exit(1);
                    }
                }
            }

            let runtime_dir = runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir);
            let Some(backup_dir) = backup_dir else {
                eprintln!("--backup-dir is required for restore-runtime-dir");
                std::process::exit(1);
            };

            match local_minimal_node::restore_runtime_dir_with_expected_preview_fingerprint(
                runtime_dir,
                backup_dir,
                expected_preview_fingerprint.as_deref(),
            ) {
                Ok(report) => {
                    if json_output {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&report)
                                .expect("runtime-dir restore report should serialize")
                        );
                    } else {
                        println!(
                            "{}",
                            local_minimal_node::format_runtime_dir_restore(&report)
                        );
                    }
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
            return;
        }

        if command == "preview-runtime-restore" {
            let mut runtime_dir = None;
            let mut backup_dir = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        let value = args.next().expect("--runtime-dir requires a value");
                        runtime_dir = Some(std::path::PathBuf::from(value));
                    }
                    "--backup-dir" => {
                        let value = args.next().expect("--backup-dir requires a value");
                        backup_dir = Some(std::path::PathBuf::from(value));
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: local-minimal-node preview-runtime-restore --backup-dir <path> [--runtime-dir <path>] [--json]"
                        );
                        return;
                    }
                    _ => {
                        eprintln!("Unknown argument for preview-runtime-restore: {argument}");
                        std::process::exit(1);
                    }
                }
            }

            let runtime_dir = runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir);
            let Some(backup_dir) = backup_dir else {
                eprintln!("--backup-dir is required for preview-runtime-restore");
                std::process::exit(1);
            };

            match local_minimal_node::preview_restore_runtime_dir(runtime_dir, backup_dir) {
                Ok(report) => {
                    if json_output {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&report)
                                .expect("runtime-dir restore preview should serialize")
                        );
                    } else {
                        println!(
                            "{}",
                            local_minimal_node::format_runtime_dir_restore_preview(&report)
                        );
                    }
                }
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
            return;
        }

        if command == "list-runtime-backups" {
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
                            "Usage: local-minimal-node list-runtime-backups [--runtime-dir <path>] [--json]"
                        );
                        return;
                    }
                    _ => {
                        eprintln!("Unknown argument for list-runtime-backups: {argument}");
                        std::process::exit(1);
                    }
                }
            }

            let catalog = local_minimal_node::list_runtime_backups(
                runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir),
            );

            if json_output {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&catalog)
                        .expect("runtime-dir backup catalog should serialize")
                );
            } else {
                println!(
                    "{}",
                    local_minimal_node::format_runtime_backup_catalog(&catalog)
                );
            }
            return;
        }

        eprintln!("Unknown command: {command}");
        std::process::exit(1);
    }

    let bind_addr = local_minimal_node::resolve_bind_addr();
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .expect("local-minimal-node should bind local listener");

    axum::serve(listener, local_minimal_node::build_public_app())
        .await
        .expect("local-minimal-node server should run");
}
