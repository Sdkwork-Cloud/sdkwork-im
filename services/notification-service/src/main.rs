#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18087")
        .await
        .expect("notification-service should bind local listener");

    axum::serve(listener, notification_service::build_public_app())
        .await
        .expect("notification-service server should run");
}
