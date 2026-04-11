use std::path::PathBuf;
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
        if command == "inspect-runtime-dir" {
            let mut runtime_dir = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        runtime_dir = Some(PathBuf::from(next_option_value(
                            &mut args,
                            "--runtime-dir",
                        )?));
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: local-minimal-node inspect-runtime-dir [--runtime-dir <path>] [--json]"
                        );
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "Unknown argument for inspect-runtime-dir: {argument}"
                        ));
                    }
                }
            }

            let inspection = local_minimal_node::inspect_runtime_dir(
                runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir),
            );

            if json_output {
                print_json_pretty(&inspection, "runtime-dir inspection")?;
            } else {
                println!(
                    "{}",
                    local_minimal_node::format_runtime_dir_inspection(&inspection)
                );
            }
            return Ok(());
        }

        if command == "repair-runtime-dir" {
            let mut runtime_dir = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        runtime_dir = Some(PathBuf::from(next_option_value(
                            &mut args,
                            "--runtime-dir",
                        )?));
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: local-minimal-node repair-runtime-dir [--runtime-dir <path>] [--json]"
                        );
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "Unknown argument for repair-runtime-dir: {argument}"
                        ));
                    }
                }
            }

            let repair = local_minimal_node::repair_runtime_dir(
                runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir),
            );

            if json_output {
                print_json_pretty(&repair, "runtime-dir repair report")?;
            } else {
                println!("{}", local_minimal_node::format_runtime_dir_repair(&repair));
            }
            return Ok(());
        }

        if command == "restore-runtime-dir" {
            let mut runtime_dir = None;
            let mut backup_dir = None;
            let mut expected_preview_fingerprint = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        runtime_dir = Some(PathBuf::from(next_option_value(
                            &mut args,
                            "--runtime-dir",
                        )?));
                    }
                    "--backup-dir" => {
                        backup_dir =
                            Some(PathBuf::from(next_option_value(&mut args, "--backup-dir")?));
                    }
                    "--expected-preview-fingerprint" => {
                        expected_preview_fingerprint = Some(next_option_value(
                            &mut args,
                            "--expected-preview-fingerprint",
                        )?);
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: local-minimal-node restore-runtime-dir --backup-dir <path> [--runtime-dir <path>] [--expected-preview-fingerprint <value>] [--json]"
                        );
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "Unknown argument for restore-runtime-dir: {argument}"
                        ));
                    }
                }
            }

            let runtime_dir = runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir);
            let Some(backup_dir) = backup_dir else {
                return Err("--backup-dir is required for restore-runtime-dir".to_owned());
            };

            let report = local_minimal_node::restore_runtime_dir_with_expected_preview_fingerprint(
                runtime_dir,
                backup_dir,
                expected_preview_fingerprint.as_deref(),
            )?;
            if json_output {
                print_json_pretty(&report, "runtime-dir restore report")?;
            } else {
                println!(
                    "{}",
                    local_minimal_node::format_runtime_dir_restore(&report)
                );
            }
            return Ok(());
        }

        if command == "preview-runtime-restore" {
            let mut runtime_dir = None;
            let mut backup_dir = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        runtime_dir = Some(PathBuf::from(next_option_value(
                            &mut args,
                            "--runtime-dir",
                        )?));
                    }
                    "--backup-dir" => {
                        backup_dir =
                            Some(PathBuf::from(next_option_value(&mut args, "--backup-dir")?));
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: local-minimal-node preview-runtime-restore --backup-dir <path> [--runtime-dir <path>] [--json]"
                        );
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "Unknown argument for preview-runtime-restore: {argument}"
                        ));
                    }
                }
            }

            let runtime_dir = runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir);
            let Some(backup_dir) = backup_dir else {
                return Err("--backup-dir is required for preview-runtime-restore".to_owned());
            };

            let report = local_minimal_node::preview_restore_runtime_dir(runtime_dir, backup_dir)?;
            if json_output {
                print_json_pretty(&report, "runtime-dir restore preview")?;
            } else {
                println!(
                    "{}",
                    local_minimal_node::format_runtime_dir_restore_preview(&report)
                );
            }
            return Ok(());
        }

        if command == "list-runtime-backups" {
            let mut runtime_dir = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        runtime_dir = Some(PathBuf::from(next_option_value(
                            &mut args,
                            "--runtime-dir",
                        )?));
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: local-minimal-node list-runtime-backups [--runtime-dir <path>] [--json]"
                        );
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "Unknown argument for list-runtime-backups: {argument}"
                        ));
                    }
                }
            }

            let catalog = local_minimal_node::list_runtime_backups(
                runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir),
            );

            if json_output {
                print_json_pretty(&catalog, "runtime-dir backup catalog")?;
            } else {
                println!(
                    "{}",
                    local_minimal_node::format_runtime_backup_catalog(&catalog)
                );
            }
            return Ok(());
        }

        if command == "archive-runtime-backup" {
            let mut runtime_dir = None;
            let mut backup_dir = None;
            let mut retention_days = None;
            let mut legal_hold = false;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        runtime_dir = Some(PathBuf::from(next_option_value(
                            &mut args,
                            "--runtime-dir",
                        )?));
                    }
                    "--backup-dir" => {
                        backup_dir =
                            Some(PathBuf::from(next_option_value(&mut args, "--backup-dir")?));
                    }
                    "--retention-days" => {
                        let value = next_option_value(&mut args, "--retention-days")?;
                        retention_days = Some(value.parse::<u64>().map_err(|error| {
                            format!("--retention-days expects an integer number of days: {error}")
                        })?);
                    }
                    "--legal-hold" => {
                        legal_hold = true;
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: local-minimal-node archive-runtime-backup --backup-dir <path> [--runtime-dir <path>] [--retention-days <days>] [--legal-hold] [--json]"
                        );
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "Unknown argument for archive-runtime-backup: {argument}"
                        ));
                    }
                }
            }

            let runtime_dir = runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir);
            let Some(backup_dir) = backup_dir else {
                return Err("--backup-dir is required for archive-runtime-backup".to_owned());
            };

            let report = if retention_days.is_none() && !legal_hold {
                local_minimal_node::archive_runtime_backup(runtime_dir, backup_dir)
            } else {
                local_minimal_node::archive_runtime_backup_with_policy(
                    runtime_dir,
                    backup_dir,
                    retention_days.unwrap_or(30),
                    legal_hold,
                )
            }?;

            if json_output {
                print_json_pretty(&report, "runtime-dir archive report")?;
            } else {
                println!(
                    "{}",
                    local_minimal_node::format_runtime_dir_archive(&report)
                );
            }
            return Ok(());
        }

        if command == "prune-archived-runtime-backups" {
            let mut runtime_dir = None;
            let mut json_output = false;

            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--runtime-dir" => {
                        runtime_dir = Some(PathBuf::from(next_option_value(
                            &mut args,
                            "--runtime-dir",
                        )?));
                    }
                    "--json" => {
                        json_output = true;
                    }
                    "-h" | "--help" => {
                        eprintln!(
                            "Usage: local-minimal-node prune-archived-runtime-backups [--runtime-dir <path>] [--json]"
                        );
                        return Ok(());
                    }
                    _ => {
                        return Err(format!(
                            "Unknown argument for prune-archived-runtime-backups: {argument}"
                        ));
                    }
                }
            }

            let report = local_minimal_node::prune_archived_runtime_backups(
                runtime_dir.unwrap_or_else(local_minimal_node::resolve_runtime_dir),
            )?;
            if json_output {
                print_json_pretty(&report, "runtime-dir archive prune report")?;
            } else {
                println!(
                    "{}",
                    local_minimal_node::format_runtime_dir_archive_prune(&report)
                );
            }
            return Ok(());
        }

        return Err(format!("Unknown command: {command}"));
    }

    let bind_addr = local_minimal_node::resolve_bind_addr();
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str())
        .await
        .map_err(|error| format!("local-minimal-node failed to bind local listener: {error}"))?;

    axum::serve(listener, local_minimal_node::build_public_app())
        .await
        .map_err(|error| format!("local-minimal-node server should run: {error}"))?;
    Ok(())
}

fn next_option_value(
    args: &mut impl Iterator<Item = String>,
    option: &str,
) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("{option} requires a value"))
}

fn print_json_pretty(value: &impl serde::Serialize, description: &str) -> Result<(), String> {
    let body = serde_json::to_string_pretty(value)
        .map_err(|error| format!("{description} should serialize: {error}"))?;
    println!("{body}");
    Ok(())
}
