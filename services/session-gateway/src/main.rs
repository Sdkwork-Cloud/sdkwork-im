#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18080")
        .await
        .expect("session-gateway should bind local listener");

    axum::serve(listener, session_gateway::build_public_app())
        .await
        .expect("session-gateway server should run");
}
