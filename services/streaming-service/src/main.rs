#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18084")
        .await
        .expect("streaming-service should bind local listener");

    axum::serve(listener, streaming_service::build_public_app())
        .await
        .expect("streaming-service server should run");
}
