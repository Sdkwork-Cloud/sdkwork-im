use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use im_auth_context::encode_hs256_bearer_token;
use serde_json::json;

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);
const TEST_PUBLIC_BEARER_SECRET: &str = "blackbox-public-bearer-secret";

struct ManagedServerProcess {
    child: Child,
    base_url: String,
}

impl Drop for ManagedServerProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

struct HttpResponse {
    status_code: u16,
    body: Vec<u8>,
}

fn unique_runtime_dir(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}_{sequence}"))
}

fn reserve_local_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("ephemeral port should bind");
    let port = listener
        .local_addr()
        .expect("ephemeral listener should expose local addr")
        .port();
    drop(listener);
    port
}

fn spawn_local_minimal_server(runtime_dir: &Path, port: u16) -> ManagedServerProcess {
    spawn_local_minimal_server_with_env(runtime_dir, port, &[])
}

fn spawn_local_minimal_server_with_env(
    runtime_dir: &Path,
    port: u16,
    extra_env: &[(&str, &str)],
) -> ManagedServerProcess {
    let bind_addr = format!("127.0.0.1:{port}");
    let mut command = Command::new(env!("CARGO_BIN_EXE_local-minimal-node"));
    command
        .env("CRAW_CHAT_RUNTIME_DIR", runtime_dir)
        .env("CRAW_CHAT_BIND_ADDR", bind_addr.as_str())
        .env(
            "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET",
            TEST_PUBLIC_BEARER_SECRET,
        )
        .env(
            "CRAW_CHAT_FRIEND_REQUEST_CURSOR_HS256_SECRET",
            "blackbox-test-secret",
        )
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    for (name, value) in extra_env {
        command.env(name, value);
    }
    let child = command
        .spawn()
        .expect("local-minimal-node process should spawn");
    let server = ManagedServerProcess {
        child,
        base_url: format!("http://{bind_addr}"),
    };
    wait_for_healthz(server.base_url.as_str(), Duration::from_secs(10));
    server
}

fn wait_for_healthz(base_url: &str, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if let Ok(response) = send_http_request(base_url, "GET", "/healthz", &[], None)
            && response.status_code == 200
        {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }
    panic!(
        "server {base_url} did not become healthy within {:?}",
        timeout
    );
}

fn send_http_request(
    base_url: &str,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: Option<&str>,
) -> Result<HttpResponse, String> {
    let authority = base_url
        .strip_prefix("http://")
        .ok_or_else(|| format!("unsupported base url: {base_url}"))?;
    let mut stream = TcpStream::connect(authority)
        .map_err(|error| format!("failed to connect to {authority}: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|error| format!("failed to set read timeout: {error}"))?;
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .map_err(|error| format!("failed to set write timeout: {error}"))?;

    let body = body.unwrap_or("");
    let mut request = format!(
        "{method} {path} HTTP/1.1\r\nHost: {authority}\r\nConnection: close\r\nContent-Length: {}\r\n",
        body.len()
    );
    for (name, value) in headers {
        request.push_str(name);
        request.push_str(": ");
        request.push_str(value);
        request.push_str("\r\n");
    }
    request.push_str("\r\n");
    request.push_str(body);

    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("failed to write request: {error}"))?;
    stream
        .flush()
        .map_err(|error| format!("failed to flush request: {error}"))?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|error| format!("failed to read response: {error}"))?;

    let header_end = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| "http response missing header terminator".to_owned())?;
    let headers_raw = &response[..header_end];
    let body_start = header_end + 4;
    let status_line_end = headers_raw
        .windows(2)
        .position(|window| window == b"\r\n")
        .unwrap_or(headers_raw.len());
    let status_line = std::str::from_utf8(&headers_raw[..status_line_end])
        .map_err(|error| format!("response status line should be utf8: {error}"))?;
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| format!("response status line missing code: {status_line}"))?
        .parse::<u16>()
        .map_err(|error| format!("response status code should parse: {error}"))?;

    Ok(HttpResponse {
        status_code,
        body: response[body_start..].to_vec(),
    })
}

fn send_json_request(
    base_url: &str,
    method: &str,
    path: &str,
    user_id: &str,
    body: Option<&str>,
) -> Result<(u16, serde_json::Value), String> {
    let authorization = signed_bearer_for_user(user_id, &[]);
    let mut headers = vec![
        ("authorization", authorization.as_str()),
        ("x-tenant-id", "t_demo"),
        ("x-user-id", user_id),
    ];
    if body.is_some() {
        headers.push(("content-type", "application/json"));
    }
    let response = send_http_request(base_url, method, path, &headers, body)?;
    let json = serde_json::from_slice::<serde_json::Value>(&response.body)
        .map_err(|error| format!("response body should be valid json: {error}"))?;
    Ok((response.status_code, json))
}

fn signed_bearer_for_user(user_id: &str, permissions: &[&str]) -> String {
    let token = encode_hs256_bearer_token(
        &json!({
            "tenant_id": "t_demo",
            "sub": user_id,
            "actor_kind": "user",
            "sid": format!("s_{user_id}"),
            "did": format!("d_{user_id}"),
            "client_kind": "im_user",
            "permissions": permissions,
            "iss": "craw-chat",
            "aud": "craw-chat-public",
        }),
        TEST_PUBLIC_BEARER_SECRET,
    )
    .expect("signed bearer token should encode");
    format!("Bearer {token}")
}

#[test]
fn test_local_minimal_process_blackbox_concurrent_accept_and_cancel_across_instances_converge() {
    let runtime_dir = unique_runtime_dir("process_social_accept_cancel");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let port_a = reserve_local_port();
    let port_b = reserve_local_port();
    let server_a = spawn_local_minimal_server(runtime_dir.as_path(), port_a);
    let server_b = spawn_local_minimal_server(runtime_dir.as_path(), port_b);

    let (submit_status, submit_json) = send_json_request(
        server_a.base_url.as_str(),
        "POST",
        "/api/v1/social/friend-requests",
        "u_alice",
        Some(
            r#"{
                "targetUserId":"u_bob",
                "requestMessage":"process blackbox accept cancel race"
            }"#,
        ),
    )
    .expect("submit request should succeed");
    assert_eq!(submit_status, 200);
    let request_id = submit_json["friendRequest"]["requestId"]
        .as_str()
        .expect("submitted request should expose request id")
        .to_owned();

    let accept_base_url = server_a.base_url.clone();
    let accept_path = format!("/api/v1/social/friend-requests/{request_id}/accept");
    let accept_handle = thread::spawn(move || {
        send_json_request(
            accept_base_url.as_str(),
            "POST",
            accept_path.as_str(),
            "u_bob",
            None,
        )
    });

    let cancel_base_url = server_b.base_url.clone();
    let cancel_path = format!("/api/v1/social/friend-requests/{request_id}/cancel");
    let cancel_handle = thread::spawn(move || {
        send_json_request(
            cancel_base_url.as_str(),
            "POST",
            cancel_path.as_str(),
            "u_alice",
            None,
        )
    });

    let (accept_status, accept_json) = accept_handle
        .join()
        .expect("accept thread should join")
        .expect("accept request should return http response");
    let (cancel_status, cancel_json) = cancel_handle
        .join()
        .expect("cancel thread should join")
        .expect("cancel request should return http response");

    let success_count = [accept_status, cancel_status]
        .into_iter()
        .filter(|status| *status == 200)
        .count();
    let conflict_count = [accept_status, cancel_status]
        .into_iter()
        .filter(|status| *status == 409)
        .count();
    assert_eq!(
        success_count, 1,
        "exactly one terminal operation should win the cross-process accept/cancel race; accept_status={accept_status}, cancel_status={cancel_status}, accept_json={accept_json}, cancel_json={cancel_json}"
    );
    assert_eq!(
        conflict_count, 1,
        "the losing terminal operation should be rejected with conflict; accept_status={accept_status}, cancel_status={cancel_status}, accept_json={accept_json}, cancel_json={cancel_json}"
    );

    let expected_final_status = if accept_status == 200 {
        assert_eq!(accept_json["friendRequest"]["status"], "accepted");
        assert_eq!(cancel_json["code"], "friend_request_not_pending");
        "accepted"
    } else {
        assert_eq!(cancel_status, 200);
        assert_eq!(cancel_json["friendRequest"]["status"], "canceled");
        assert_eq!(accept_json["code"], "friend_request_not_pending");
        "canceled"
    };

    let snapshot_path = format!("/api/v1/control/social/friend-requests/{request_id}");
    let admin_bearer = signed_bearer_for_user("u_admin", &["control.read"]);
    let snapshot_response = send_http_request(
        server_a.base_url.as_str(),
        "GET",
        snapshot_path.as_str(),
        &[
            ("authorization", admin_bearer.as_str()),
            ("x-tenant-id", "t_demo"),
            ("x-user-id", "u_admin"),
        ],
        None,
    )
    .expect("snapshot request should return response");
    assert_eq!(snapshot_response.status_code, 200);
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_response.body)
        .expect("snapshot response should be valid json");
    assert_eq!(
        snapshot_json["friendRequest"]["status"],
        expected_final_status
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_local_minimal_process_blackbox_concurrent_accepts_converge_idempotently_across_instances() {
    let runtime_dir = unique_runtime_dir("process_social_accept_accept");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let port_a = reserve_local_port();
    let port_b = reserve_local_port();
    let server_a = spawn_local_minimal_server_with_env(
        runtime_dir.as_path(),
        port_a,
        &[(
            "CRAW_CHAT_TEST_SOCIAL_ACCEPT_REPAIR_STORE_IO_DELAY_MS",
            "250",
        )],
    );
    let server_b = spawn_local_minimal_server_with_env(
        runtime_dir.as_path(),
        port_b,
        &[(
            "CRAW_CHAT_TEST_SOCIAL_ACCEPT_REPAIR_STORE_IO_DELAY_MS",
            "250",
        )],
    );

    let (submit_status, submit_json) = send_json_request(
        server_a.base_url.as_str(),
        "POST",
        "/api/v1/social/friend-requests",
        "u_alice",
        Some(
            r#"{
                "targetUserId":"u_bob",
                "requestMessage":"process blackbox accept accept race"
            }"#,
        ),
    )
    .expect("submit request before concurrent accepts should succeed");
    assert_eq!(submit_status, 200);
    let request_id = submit_json["friendRequest"]["requestId"]
        .as_str()
        .expect("submitted request should expose request id")
        .to_owned();

    let accept_path = format!("/api/v1/social/friend-requests/{request_id}/accept");
    let first_accept_base_url = server_a.base_url.clone();
    let first_accept_path = accept_path.clone();
    let first_accept_handle = thread::spawn(move || {
        send_json_request(
            first_accept_base_url.as_str(),
            "POST",
            first_accept_path.as_str(),
            "u_bob",
            None,
        )
    });

    let second_accept_base_url = server_b.base_url.clone();
    let second_accept_handle = thread::spawn(move || {
        send_json_request(
            second_accept_base_url.as_str(),
            "POST",
            accept_path.as_str(),
            "u_bob",
            None,
        )
    });

    let (first_accept_status, first_accept_json) = first_accept_handle
        .join()
        .expect("first accept thread should join")
        .expect("first accept request should return http response");
    let (second_accept_status, second_accept_json) = second_accept_handle
        .join()
        .expect("second accept thread should join")
        .expect("second accept request should return http response");

    assert_eq!(
        first_accept_status, 200,
        "first cross-process accept should converge successfully: {first_accept_json}"
    );
    assert_eq!(
        second_accept_status, 200,
        "second cross-process accept should converge successfully: {second_accept_json}"
    );
    assert_eq!(first_accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(second_accept_json["friendRequest"]["status"], "accepted");
    assert_eq!(
        second_accept_json["friendship"]["friendshipId"],
        first_accept_json["friendship"]["friendshipId"]
    );
    assert_eq!(
        second_accept_json["directChat"]["directChatId"],
        first_accept_json["directChat"]["directChatId"]
    );
    assert_eq!(
        second_accept_json["conversation"]["conversationId"],
        first_accept_json["conversation"]["conversationId"]
    );

    let snapshot_path = format!("/api/v1/control/social/friend-requests/{request_id}");
    let admin_bearer = signed_bearer_for_user("u_admin", &["control.read"]);
    let snapshot_response = send_http_request(
        server_a.base_url.as_str(),
        "GET",
        snapshot_path.as_str(),
        &[
            ("authorization", admin_bearer.as_str()),
            ("x-tenant-id", "t_demo"),
            ("x-user-id", "u_admin"),
        ],
        None,
    )
    .expect("request snapshot after concurrent accepts should return response");
    assert_eq!(snapshot_response.status_code, 200);
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_response.body)
        .expect("request snapshot after concurrent accepts should be valid json");
    assert_eq!(snapshot_json["friendRequest"]["status"], "accepted");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_local_minimal_process_blackbox_cross_instance_accept_and_submit_same_pair_converge() {
    let runtime_dir = unique_runtime_dir("process_social_accept_submit");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let port_a = reserve_local_port();
    let port_b = reserve_local_port();
    let server_a = spawn_local_minimal_server_with_env(
        runtime_dir.as_path(),
        port_a,
        &[("CRAW_CHAT_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS", "400")],
    );
    let server_b = spawn_local_minimal_server(runtime_dir.as_path(), port_b);

    let (submit_status, submit_json) = send_json_request(
        server_a.base_url.as_str(),
        "POST",
        "/api/v1/social/friend-requests",
        "u_alice",
        Some(
            r#"{
                "targetUserId":"u_bob",
                "requestMessage":"process blackbox accept submit race"
            }"#,
        ),
    )
    .expect("submit request before accept/submit race should succeed");
    assert_eq!(submit_status, 200);
    let request_id = submit_json["friendRequest"]["requestId"]
        .as_str()
        .expect("submitted request should expose request id")
        .to_owned();

    let accept_base_url = server_a.base_url.clone();
    let accept_path = format!("/api/v1/social/friend-requests/{request_id}/accept");
    let accept_handle = thread::spawn(move || {
        send_json_request(
            accept_base_url.as_str(),
            "POST",
            accept_path.as_str(),
            "u_bob",
            None,
        )
    });

    thread::sleep(Duration::from_millis(150));

    let (submit_again_status, submit_again_json) = send_json_request(
        server_b.base_url.as_str(),
        "POST",
        "/api/v1/social/friend-requests",
        "u_alice",
        Some(
            r#"{
                "targetUserId":"u_bob",
                "requestMessage":"process blackbox accept submit retry"
            }"#,
        ),
    )
    .expect("submit request during cross-process accept should return response");
    assert_eq!(
        submit_again_status, 409,
        "submit during accepted-but-not-yet-materialized window should synchronously repair the pending acceptance and then reject the duplicate pair as already active: {submit_again_json}"
    );
    assert_eq!(submit_again_json["code"], "friendship_already_active");

    let (accept_status, accept_json) = accept_handle
        .join()
        .expect("accept thread should join")
        .expect("accept request should return http response");
    assert_eq!(accept_status, 200);
    assert_eq!(accept_json["friendRequest"]["status"], "accepted");

    let snapshot_path = format!("/api/v1/control/social/friend-requests/{request_id}");
    let admin_bearer = signed_bearer_for_user("u_admin", &["control.read"]);
    let snapshot_response = send_http_request(
        server_b.base_url.as_str(),
        "GET",
        snapshot_path.as_str(),
        &[
            ("authorization", admin_bearer.as_str()),
            ("x-tenant-id", "t_demo"),
            ("x-user-id", "u_admin"),
        ],
        None,
    )
    .expect("request snapshot after accept/submit race should return response");
    assert_eq!(snapshot_response.status_code, 200);
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_response.body)
        .expect("request snapshot after accept/submit race should be valid json");
    assert_eq!(snapshot_json["friendRequest"]["status"], "accepted");

    let contacts_response = send_http_request(
        server_b.base_url.as_str(),
        "GET",
        "/api/v1/contacts",
        &[
            (
                "authorization",
                signed_bearer_for_user("u_alice", &[]).as_str(),
            ),
            ("x-tenant-id", "t_demo"),
            ("x-user-id", "u_alice"),
        ],
        None,
    )
    .expect("contacts after accept/submit race should return response");
    assert_eq!(contacts_response.status_code, 200);
    let contacts_json: serde_json::Value = serde_json::from_slice(&contacts_response.body)
        .expect("contacts after accept/submit race should be valid json");
    let items = contacts_json["items"]
        .as_array()
        .expect("contacts after accept/submit race should include items");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["targetUserId"], "u_bob");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_local_minimal_process_blackbox_cross_instance_submit_same_pair_converges_to_single_pending_request()
 {
    let runtime_dir = unique_runtime_dir("process_social_cross_submit");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let port_a = reserve_local_port();
    let port_b = reserve_local_port();
    let server_a = spawn_local_minimal_server(runtime_dir.as_path(), port_a);
    let server_b = spawn_local_minimal_server(runtime_dir.as_path(), port_b);

    let submit_alice_base_url = server_a.base_url.clone();
    let submit_alice = thread::spawn(move || {
        send_json_request(
            submit_alice_base_url.as_str(),
            "POST",
            "/api/v1/social/friend-requests",
            "u_alice",
            Some(
                r#"{
                    "targetUserId":"u_bob",
                    "requestMessage":"cross instance alice to bob"
                }"#,
            ),
        )
    });

    let submit_bob_base_url = server_b.base_url.clone();
    let submit_bob = thread::spawn(move || {
        send_json_request(
            submit_bob_base_url.as_str(),
            "POST",
            "/api/v1/social/friend-requests",
            "u_bob",
            Some(
                r#"{
                    "targetUserId":"u_alice",
                    "requestMessage":"cross instance bob to alice"
                }"#,
            ),
        )
    });

    let (alice_status, alice_json) = submit_alice
        .join()
        .expect("alice submit thread should join")
        .expect("alice submit should return http response");
    let (bob_status, bob_json) = submit_bob
        .join()
        .expect("bob submit thread should join")
        .expect("bob submit should return http response");

    assert_eq!(alice_status, 200);
    assert_eq!(bob_status, 200);
    assert_eq!(alice_json["friendRequest"]["status"], "pending");
    assert_eq!(bob_json["friendRequest"]["status"], "pending");

    let alice_request_id = alice_json["friendRequest"]["requestId"]
        .as_str()
        .expect("alice submit response should expose request id");
    let bob_request_id = bob_json["friendRequest"]["requestId"]
        .as_str()
        .expect("bob submit response should expose request id");
    assert_eq!(
        alice_request_id, bob_request_id,
        "cross-instance same-pair submit must converge to a single pending request id"
    );

    let snapshot_path = format!("/api/v1/control/social/friend-requests/{alice_request_id}");
    let admin_bearer = signed_bearer_for_user("u_admin", &["control.read"]);
    let snapshot_response = send_http_request(
        server_a.base_url.as_str(),
        "GET",
        snapshot_path.as_str(),
        &[
            ("authorization", admin_bearer.as_str()),
            ("x-tenant-id", "t_demo"),
            ("x-user-id", "u_admin"),
        ],
        None,
    )
    .expect("snapshot request should return response");
    assert_eq!(snapshot_response.status_code, 200);

    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_response.body)
        .expect("snapshot response should be valid json");
    assert_eq!(snapshot_json["friendRequest"]["status"], "pending");

    let requester = snapshot_json["friendRequest"]["requesterUserId"]
        .as_str()
        .expect("snapshot should expose requester user id");
    let target = snapshot_json["friendRequest"]["targetUserId"]
        .as_str()
        .expect("snapshot should expose target user id");
    assert!(
        (requester == "u_alice" && target == "u_bob")
            || (requester == "u_bob" && target == "u_alice"),
        "snapshot pair should remain within the competing user pair"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_local_minimal_process_blackbox_cross_instance_remove_and_submit_converge() {
    let runtime_dir = unique_runtime_dir("process_social_remove_submit");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let port_a = reserve_local_port();
    let port_b = reserve_local_port();
    let server_a = spawn_local_minimal_server(runtime_dir.as_path(), port_a);
    let server_b = spawn_local_minimal_server(runtime_dir.as_path(), port_b);

    let (submit_status, submit_json) = send_json_request(
        server_a.base_url.as_str(),
        "POST",
        "/api/v1/social/friend-requests",
        "u_alice",
        Some(
            r#"{
                "targetUserId":"u_bob",
                "requestMessage":"establish friendship before remove submit race"
            }"#,
        ),
    )
    .expect("submit request before remove/submit race should succeed");
    assert_eq!(submit_status, 200);
    let request_id = submit_json["friendRequest"]["requestId"]
        .as_str()
        .expect("submitted request should expose request id")
        .to_owned();

    let accept_path = format!("/api/v1/social/friend-requests/{request_id}/accept");
    let (accept_status, accept_json) = send_json_request(
        server_b.base_url.as_str(),
        "POST",
        accept_path.as_str(),
        "u_bob",
        None,
    )
    .expect("accept request before remove/submit race should succeed");
    assert_eq!(accept_status, 200);
    let friendship_id = accept_json["friendship"]["friendshipId"]
        .as_str()
        .expect("accept response should expose friendship id")
        .to_owned();

    let remove_base_url = server_a.base_url.clone();
    let remove_path = format!("/api/v1/social/friendships/{friendship_id}/remove");
    let remove_friendship = thread::spawn(move || {
        send_json_request(
            remove_base_url.as_str(),
            "POST",
            remove_path.as_str(),
            "u_alice",
            None,
        )
    });

    let submit_again_base_url = server_b.base_url.clone();
    let submit_again = thread::spawn(move || {
        send_json_request(
            submit_again_base_url.as_str(),
            "POST",
            "/api/v1/social/friend-requests",
            "u_alice",
            Some(
                r#"{
                    "targetUserId":"u_bob",
                    "requestMessage":"cross instance remove submit race"
                }"#,
            ),
        )
    });

    let (remove_status, remove_json) = remove_friendship
        .join()
        .expect("remove thread should join")
        .expect("remove request should return http response");
    let (submit_again_status, submit_again_json) = submit_again
        .join()
        .expect("submit-again thread should join")
        .expect("submit-again request should return http response");

    assert_eq!(remove_status, 200);
    assert_eq!(remove_json["friendship"]["status"], "removed");
    match submit_again_status {
        200 => assert_eq!(submit_again_json["friendRequest"]["status"], "pending"),
        409 => assert_eq!(submit_again_json["code"], "friendship_already_active"),
        other => panic!("unexpected cross-instance remove/submit race submit status: {other}"),
    }

    let admin_bearer = signed_bearer_for_user("u_admin", &["control.read"]);
    let friendship_snapshot_path = format!("/api/v1/control/social/friendships/{friendship_id}");
    let friendship_snapshot = send_http_request(
        server_a.base_url.as_str(),
        "GET",
        friendship_snapshot_path.as_str(),
        &[
            ("authorization", admin_bearer.as_str()),
            ("x-tenant-id", "t_demo"),
            ("x-user-id", "u_admin"),
        ],
        None,
    )
    .expect("friendship snapshot request should return response");
    assert_eq!(friendship_snapshot.status_code, 200);
    let friendship_snapshot_json: serde_json::Value =
        serde_json::from_slice(&friendship_snapshot.body)
            .expect("friendship snapshot should be valid json");
    assert_eq!(friendship_snapshot_json["friendship"]["status"], "removed");

    if submit_again_status == 200 {
        let new_request_id = submit_again_json["friendRequest"]["requestId"]
            .as_str()
            .expect("successful resubmit should expose request id");
        let request_snapshot_path =
            format!("/api/v1/control/social/friend-requests/{new_request_id}");
        let request_snapshot = send_http_request(
            server_b.base_url.as_str(),
            "GET",
            request_snapshot_path.as_str(),
            &[
                ("authorization", admin_bearer.as_str()),
                ("x-tenant-id", "t_demo"),
                ("x-user-id", "u_admin"),
            ],
            None,
        )
        .expect("resubmitted request snapshot should return response");
        assert_eq!(request_snapshot.status_code, 200);
        let request_snapshot_json: serde_json::Value =
            serde_json::from_slice(&request_snapshot.body)
                .expect("resubmitted request snapshot should be valid json");
        assert_eq!(request_snapshot_json["friendRequest"]["status"], "pending");
    }

    let (alice_contacts_status, alice_contacts_json) = send_json_request(
        server_a.base_url.as_str(),
        "GET",
        "/api/v1/contacts",
        "u_alice",
        None,
    )
    .expect("alice contacts after remove/submit race should return response");
    assert_eq!(alice_contacts_status, 200);
    assert!(
        alice_contacts_json["items"]
            .as_array()
            .expect("alice contacts response should expose items array")
            .is_empty(),
        "removed friendship must not remain in alice contacts after remove/submit race: runtime_dir={}, remove_status={}, submit_status={}, contacts={}",
        runtime_dir.display(),
        remove_status,
        submit_again_status,
        alice_contacts_json
    );

    let (bob_contacts_status, bob_contacts_json) = send_json_request(
        server_b.base_url.as_str(),
        "GET",
        "/api/v1/contacts",
        "u_bob",
        None,
    )
    .expect("bob contacts after remove/submit race should return response");
    assert_eq!(bob_contacts_status, 200);
    assert!(
        bob_contacts_json["items"]
            .as_array()
            .expect("bob contacts response should expose items array")
            .is_empty(),
        "removed friendship must not remain in bob contacts after remove/submit race: runtime_dir={}, remove_status={}, submit_status={}, contacts={}",
        runtime_dir.display(),
        remove_status,
        submit_again_status,
        bob_contacts_json
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
