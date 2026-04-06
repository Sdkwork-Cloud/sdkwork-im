#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18086")
        .await
        .expect("media-service should bind local listener");

    axum::serve(listener, media_service::build_public_app())
        .await
        .expect("media-service server should run");
}
