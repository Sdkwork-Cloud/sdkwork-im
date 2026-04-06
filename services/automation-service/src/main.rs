#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18088")
        .await
        .expect("automation-service should bind local listener");

    axum::serve(listener, automation_service::build_public_app())
        .await
        .expect("automation-service server should run");
}
