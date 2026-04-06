#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18083")
        .await
        .expect("projection-service should bind local listener");

    axum::serve(listener, projection_service::build_public_app())
        .await
        .expect("projection-service server should run");
}
