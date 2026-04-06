#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18085")
        .await
        .expect("rtc-signaling-service should bind local listener");

    axum::serve(listener, rtc_signaling_service::build_public_app())
        .await
        .expect("rtc-signaling-service server should run");
}
