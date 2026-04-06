#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:18082")
        .await
        .expect("conversation-runtime should bind local listener");

    axum::serve(listener, conversation_runtime::build_public_app())
        .await
        .expect("conversation-runtime server should run");
}
