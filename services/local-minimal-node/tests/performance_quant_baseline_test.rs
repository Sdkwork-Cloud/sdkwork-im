use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use futures_util::StreamExt;
use futures_util::future::join_all;
use http_body_util::BodyExt;
use serde::Deserialize;
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tokio::time::{Duration, timeout};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tower::ServiceExt;

trait AppContextRequestBuilderExt {
    fn demo_app_context(self) -> Self;
}

impl AppContextRequestBuilderExt for axum::http::request::Builder {
    fn demo_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_demo")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "s_demo")
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Step11LocalQuantBaseline {
    profile: String,
    tier: String,
    connection: ConnectionBaseline,
    message: MessageBaseline,
    stream: StreamBaseline,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConnectionBaseline {
    connection_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MessageBaseline {
    message_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StreamBaseline {
    frame_count: usize,
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn local_baseline_path() -> PathBuf {
    workspace_root()
        .join("tools")
        .join("perf")
        .join("step-11-cp11-2-local-baseline.json")
}

fn load_local_baseline() -> Step11LocalQuantBaseline {
    let path = local_baseline_path();
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing Step 11 local baseline config: {}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|_| panic!("invalid Step 11 local baseline config: {}", path.display()))
}

fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

fn p95_ms(samples: &[f64]) -> f64 {
    let mut ordered = samples.to_vec();
    ordered.sort_by(|left, right| {
        left.partial_cmp(right)
            .expect("values should be comparable")
    });
    let index = ((ordered.len() as f64) * 0.95).ceil() as usize;
    ordered[index.saturating_sub(1)]
}

fn print_metric(metric: Value) {
    println!("STEP11_METRIC {}", metric);
}

async fn spawn_server(app: Router) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should expose local address");
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("server should run");
    });
    (format!("127.0.0.1:{}", address.port()), handle)
}

async fn next_text_json(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Value {
    let message = timeout(Duration::from_secs(5), socket.next())
        .await
        .expect("websocket frame should arrive before timeout")
        .expect("websocket should stay open")
        .expect("websocket frame should decode");
    match message {
        Message::Text(text) => serde_json::from_str(text.as_str())
            .expect("websocket text frame should contain valid json"),
        other => panic!("expected text frame, got {other:?}"),
    }
}

#[test]
fn test_step11_local_quant_baseline_config_and_operator_doc_are_frozen() {
    let baseline = load_local_baseline();
    assert_eq!(baseline.profile, "local-minimal");
    assert_eq!(baseline.tier, "CI Smoke Tier");
    assert!(
        baseline.connection.connection_count > 0,
        "connectionCount must be greater than zero"
    );
    assert!(
        baseline.message.message_count > 0,
        "messageCount must be greater than zero"
    );
    assert!(
        baseline.stream.frame_count > 0,
        "frameCount must be greater than zero"
    );

    let doc_path = workspace_root()
        .join("docs")
        .join("部署")
        .join("性能与灾备演练场景.md");
    let doc = fs::read_to_string(&doc_path)
        .unwrap_or_else(|_| panic!("missing Step 11 operator doc: {}", doc_path.display()));
    assert!(doc.contains("tools/perf/step-11-cp11-2-local-baseline.json"));
    assert!(doc.contains("services/local-minimal-node/tests/performance_quant_baseline_test.rs"));
}

#[tokio::test]
async fn test_step11_local_connection_quant_baseline_emits_metrics() {
    let baseline = load_local_baseline();
    let app = local_minimal_node::build_default_app();
    let (address, handle) = spawn_server(app).await;

    let total_start = Instant::now();
    let latencies_ms = join_all((0..baseline.connection.connection_count).map(|index| {
        let address = address.clone();
        async move {
            let mut request = format!("ws://{address}/im/v3/api/realtime/ws")
                .into_client_request()
                .expect("websocket request should build");
            request.headers_mut().insert(
                "x-sdkwork-tenant-id",
                "t_demo".parse().expect("tenant header should parse"),
            );
            request.headers_mut().insert(
                "x-sdkwork-user-id",
                format!("u_step11_conn_{index}")
                    .parse()
                    .expect("user header should parse"),
            );
            request.headers_mut().insert(
                "x-sdkwork-actor-kind",
                "user".parse().expect("actor kind header should parse"),
            );
            request.headers_mut().insert(
                "x-sdkwork-session-id",
                format!("s_step11_conn_{index}")
                    .parse()
                    .expect("session header should parse"),
            );
            request.headers_mut().insert(
                "x-sdkwork-device-id",
                format!("d_step11_conn_{index}")
                    .parse()
                    .expect("device header should parse"),
            );

            let started = Instant::now();
            let (mut socket, _) = connect_async(request)
                .await
                .expect("websocket connection should succeed");
            let connected = next_text_json(&mut socket).await;
            assert_eq!(connected["type"], "realtime.connected");
            socket.close(None).await.expect("websocket should close");
            started.elapsed().as_secs_f64() * 1000.0
        }
    }))
    .await;
    let total_duration_ms = total_start.elapsed().as_secs_f64() * 1000.0;

    handle.abort();
    let _ = handle.await;

    let total_seconds = (total_duration_ms / 1000.0).max(f64::EPSILON);
    let connections_per_second = baseline.connection.connection_count as f64 / total_seconds;
    assert_eq!(latencies_ms.len(), baseline.connection.connection_count);
    assert!(connections_per_second > 0.0);

    print_metric(json!({
        "scenario": "connection",
        "profile": baseline.profile,
        "tier": baseline.tier,
        "connectionCount": baseline.connection.connection_count,
        "successCount": latencies_ms.len(),
        "totalDurationMs": round3(total_duration_ms),
        "connectP95Ms": round3(p95_ms(&latencies_ms)),
        "connectionsPerSecond": round3(connections_per_second)
    }));
}

#[tokio::test]
async fn test_step11_local_message_quant_baseline_emits_metrics() {
    let baseline = load_local_baseline();
    let app = local_minimal_node::build_default_app();
    let conversation_id = "c_step11_message_baseline";

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"conversationId\":\"{conversation_id}\",\"conversationType\":\"group\"}}"
                )))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let mut latencies_ms = Vec::with_capacity(baseline.message.message_count);
    let total_start = Instant::now();
    for index in 0..baseline.message.message_count {
        let started = Instant::now();
        let post_message = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/im/v3/api/chat/conversations/{conversation_id}/messages"))
                    .demo_app_context()
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        "{{\"clientMsgId\":\"step11_msg_{index}\",\"summary\":\"step11 message {index}\",\"text\":\"step11 message {index}\"}}"
                    )))
                    .unwrap(),
            )
            .await
            .expect("post message should succeed");
        assert_eq!(post_message.status(), StatusCode::OK);
        latencies_ms.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    let total_duration_ms = total_start.elapsed().as_secs_f64() * 1000.0;

    let summary = app
        .oneshot(
            Request::builder()
                .uri(format!("/im/v3/api/chat/conversations/{conversation_id}"))
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary should succeed");
    assert_eq!(summary.status(), StatusCode::OK);
    let summary_body = summary
        .into_body()
        .collect()
        .await
        .expect("summary body should collect")
        .to_bytes();
    let summary_json: Value =
        serde_json::from_slice(&summary_body).expect("summary should be valid json");
    let persisted_message_count = summary_json["messageCount"]
        .as_u64()
        .expect("messageCount should be numeric");

    let total_seconds = (total_duration_ms / 1000.0).max(f64::EPSILON);
    let message_tps = baseline.message.message_count as f64 / total_seconds;
    assert_eq!(
        persisted_message_count,
        baseline.message.message_count as u64
    );
    assert!(message_tps > 0.0);

    print_metric(json!({
        "scenario": "message",
        "profile": baseline.profile,
        "tier": baseline.tier,
        "messageCount": baseline.message.message_count,
        "persistedMessageCount": persisted_message_count,
        "totalDurationMs": round3(total_duration_ms),
        "postP95Ms": round3(p95_ms(&latencies_ms)),
        "messageTps": round3(message_tps)
    }));
}

#[tokio::test]
async fn test_step11_local_stream_quant_baseline_emits_metrics() {
    let baseline = load_local_baseline();
    let app = local_minimal_node::build_default_app();
    let conversation_id = "c_step11_stream_baseline";
    let stream_id = "st_step11_stream_baseline";

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"conversationId\":\"{conversation_id}\",\"conversationType\":\"group\"}}"
                )))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let open_stream = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    "{{\"streamId\":\"{stream_id}\",\"streamType\":\"custom.delta.text\",\"scopeKind\":\"conversation\",\"scopeId\":\"{conversation_id}\",\"durabilityClass\":\"durableSession\",\"schemaRef\":\"custom.delta.text.v1\"}}"
                )))
                .unwrap(),
        )
        .await
        .expect("open stream should succeed");
    assert_eq!(open_stream.status(), StatusCode::OK);

    let mut latencies_ms = Vec::with_capacity(baseline.stream.frame_count);
    let total_start = Instant::now();
    for frame_seq in 1..=baseline.stream.frame_count {
        let started = Instant::now();
        let append_frame = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/im/v3/api/streams/{stream_id}/frames"))
                    .demo_app_context()
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        "{{\"frameSeq\":{frame_seq},\"frameType\":\"delta\",\"schemaRef\":\"custom.delta.text.v1\",\"encoding\":\"json\",\"payload\":\"{{\\\"delta\\\":\\\"frame-{frame_seq}\\\"}}\"}}"
                    )))
                    .unwrap(),
            )
            .await
            .expect("append frame should succeed");
        assert_eq!(append_frame.status(), StatusCode::OK);
        latencies_ms.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    let total_duration_ms = total_start.elapsed().as_secs_f64() * 1000.0;

    let list_frames = app
        .oneshot(
            Request::builder()
                .uri(format!(
                    "/im/v3/api/streams/{stream_id}/frames?afterFrameSeq=0&limit={}",
                    baseline.stream.frame_count
                ))
                .demo_app_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames should succeed");
    assert_eq!(list_frames.status(), StatusCode::OK);
    let list_frames_body = list_frames
        .into_body()
        .collect()
        .await
        .expect("list frames body should collect")
        .to_bytes();
    let list_frames_json: Value =
        serde_json::from_slice(&list_frames_body).expect("frame list should be valid json");
    let persisted_frame_count = list_frames_json["items"]
        .as_array()
        .expect("items should be an array")
        .len();

    let total_seconds = (total_duration_ms / 1000.0).max(f64::EPSILON);
    let frames_per_second = baseline.stream.frame_count as f64 / total_seconds;
    assert_eq!(persisted_frame_count, baseline.stream.frame_count);
    assert!(frames_per_second > 0.0);

    print_metric(json!({
        "scenario": "stream",
        "profile": baseline.profile,
        "tier": baseline.tier,
        "frameCount": baseline.stream.frame_count,
        "persistedFrameCount": persisted_frame_count,
        "totalDurationMs": round3(total_duration_ms),
        "appendP95Ms": round3(p95_ms(&latencies_ms)),
        "framesPerSecond": round3(frames_per_second)
    }));
}
