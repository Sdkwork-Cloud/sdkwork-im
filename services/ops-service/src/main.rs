#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18091")
        .await
        .expect("ops-service should bind local listener");

    axum::serve(listener, ops_service::build_public_app())
        .await
        .expect("ops-service server should run");
}
