#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18089")
        .await
        .expect("audit-service should bind local listener");

    axum::serve(listener, audit_service::build_public_app())
        .await
        .expect("audit-service server should run");
}
