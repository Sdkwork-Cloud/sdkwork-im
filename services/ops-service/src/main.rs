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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18091")
        .await
        .map_err(|error| format!("ops-service failed to bind local listener: {error}"))?;

    axum::serve(listener, ops_service::build_public_app())
        .await
        .map_err(|error| format!("ops-service server should run: {error}"))?;
    Ok(())
}
