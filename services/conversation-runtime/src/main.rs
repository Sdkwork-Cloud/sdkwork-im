use std::process::ExitCode;
use std::sync::Arc;

const PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV: &str = "CRAW_CHAT_PRINCIPAL_DIRECTORY_CATALOG_PATH";

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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18082")
        .await
        .map_err(|error| format!("conversation-runtime failed to bind local listener: {error}"))?;

    let app = match std::env::var(PRINCIPAL_DIRECTORY_CATALOG_PATH_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    {
        Some(catalog_path) => {
            let directory = conversation_runtime::StaticPrincipalDirectory::from_json_file(
                std::path::Path::new(catalog_path.as_str()),
            )?;
            conversation_runtime::build_public_app_with_principal_directory(Arc::new(directory))
        }
        None => conversation_runtime::build_public_app(),
    };

    axum::serve(listener, app)
        .await
        .map_err(|error| format!("conversation-runtime server should run: {error}"))?;
    Ok(())
}
