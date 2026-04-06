#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18081")
        .await
        .expect("control-plane-api should bind local listener");

    axum::serve(listener, control_plane_api::build_public_app())
        .await
        .expect("control-plane-api server should run");
}
