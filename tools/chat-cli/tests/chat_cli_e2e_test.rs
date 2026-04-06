use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;

use axum::Router;
use craw_chat_cli::{
    CommandOutput, execute_command, execute_interactive_command_with_io, parse_cli_args,
};
use serde_json::Value;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time::{sleep, timeout};

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
    (format!("http://127.0.0.1:{}", address.port()), handle)
}

fn command_output_json(output: CommandOutput) -> Value {
    match output {
        CommandOutput::Json(value) => value,
        other => panic!("expected json output, got {other:?}"),
    }
}

fn command_output_frames(output: CommandOutput) -> Vec<Value> {
    match output {
        CommandOutput::Frames(values) => values,
        other => panic!("expected frame output, got {other:?}"),
    }
}

#[tokio::test]
async fn test_chat_cli_can_drive_two_party_http_and_websocket_validation_flow() {
    unsafe {
        std::env::set_var(
            im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV,
            "local-chat-cli-secret",
        );
    }

    let app = local_minimal_node::build_public_app();
    let (base_url, handle) = spawn_server(app).await;

    let create_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "create-conversation",
            "--conversation-id",
            "c_cli_demo",
            "--conversation-type",
            "group",
        ])
        .expect("create args should parse"),
    )
    .await
    .expect("create conversation should succeed");
    let create_json = command_output_json(create_output);
    assert_eq!(create_json["conversationId"], "c_cli_demo");

    let add_member_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "add-member",
            "--conversation-id",
            "c_cli_demo",
            "--principal-id",
            "u_guest",
            "--principal-kind",
            "user",
            "--role",
            "member",
        ])
        .expect("add-member args should parse"),
    )
    .await
    .expect("add member should succeed");
    let add_member_json = command_output_json(add_member_output);
    assert_eq!(add_member_json["principalId"], "u_guest");

    let watch_base_url = base_url.clone();
    let watch_task = tokio::spawn(async move {
        execute_command(
            parse_cli_args([
                "craw-chat-cli",
                "--base-url",
                watch_base_url.as_str(),
                "--tenant-id",
                "t_demo",
                "--user-id",
                "u_guest",
                "--session-id",
                "s_guest",
                "--device-id",
                "d_guest",
                "--public-bearer-secret",
                "local-chat-cli-secret",
                "watch",
                "--conversation-id",
                "c_cli_demo",
                "--event-type",
                "message.posted",
                "--exit-after-events",
                "1",
                "--idle-timeout-seconds",
                "5",
            ])
            .expect("watch args should parse"),
        )
        .await
    });

    sleep(Duration::from_millis(250)).await;

    let send_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "send-message",
            "--conversation-id",
            "c_cli_demo",
            "--summary",
            "hello from cli",
            "--text",
            "hello from cli",
            "--client-msg-id",
            "cli_msg_1",
        ])
        .expect("send args should parse"),
    )
    .await
    .expect("send message should succeed");
    let send_json = command_output_json(send_output);
    assert_eq!(send_json["summary"], "hello from cli");

    let watch_output = timeout(Duration::from_secs(10), watch_task)
        .await
        .expect("watch task should complete before timeout")
        .expect("watch task should join")
        .expect("watch command should succeed");
    let watch_frames = command_output_frames(watch_output);
    assert!(
        watch_frames
            .iter()
            .any(|frame| frame["type"] == "realtime.connected")
    );
    assert!(
        watch_frames
            .iter()
            .any(|frame| frame["type"] == "subscriptions.synced")
    );
    let pushed_window = watch_frames
        .iter()
        .find(|frame| frame["type"] == "event.window" && frame["reason"] == "push")
        .expect("watch should observe push event window");
    let payload: Value = serde_json::from_str(
        pushed_window["window"]["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["summary"], "hello from cli");

    let timeline_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_guest",
            "--session-id",
            "s_guest",
            "--device-id",
            "d_guest",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "timeline",
            "--conversation-id",
            "c_cli_demo",
        ])
        .expect("timeline args should parse"),
    )
    .await
    .expect("timeline should succeed");
    let timeline_json = command_output_json(timeline_output);
    assert_eq!(timeline_json["items"][0]["summary"], "hello from cli");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_chat_cli_chat_session_can_receive_and_send_messages_before_quit() {
    unsafe {
        std::env::set_var(
            im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV,
            "local-chat-cli-secret",
        );
    }

    let app = local_minimal_node::build_public_app();
    let (base_url, handle) = spawn_server(app).await;

    execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "create-conversation",
            "--conversation-id",
            "c_cli_chat_session_demo",
            "--conversation-type",
            "group",
        ])
        .expect("create args should parse"),
    )
    .await
    .expect("create conversation should succeed");

    execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "add-member",
            "--conversation-id",
            "c_cli_chat_session_demo",
            "--principal-id",
            "u_guest",
            "--principal-kind",
            "user",
            "--role",
            "member",
        ])
        .expect("add-member args should parse"),
    )
    .await
    .expect("add member should succeed");

    let chat_session_command = parse_cli_args([
        "craw-chat-cli",
        "--base-url",
        base_url.as_str(),
        "--tenant-id",
        "t_demo",
        "--user-id",
        "u_guest",
        "--session-id",
        "s_guest",
        "--device-id",
        "d_guest",
        "--public-bearer-secret",
        "local-chat-cli-secret",
        "chat-session",
        "--conversation-id",
        "c_cli_chat_session_demo",
        "--label",
        "guest",
    ])
    .expect("chat-session args should parse");

    let (mut chat_input_writer, chat_input_reader) = tokio::io::duplex(4096);
    let (mut chat_output_reader, chat_output_writer) = tokio::io::duplex(4096);
    let chat_session_task = tokio::spawn(async move {
        execute_interactive_command_with_io(
            chat_session_command,
            chat_input_reader,
            chat_output_writer,
        )
        .await
    });

    sleep(Duration::from_millis(750)).await;

    execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "send-message",
            "--conversation-id",
            "c_cli_chat_session_demo",
            "--summary",
            "hello from owner",
            "--text",
            "hello from owner",
            "--client-msg-id",
            "cli_owner_msg_chat_session_1",
        ])
        .expect("send args should parse"),
    )
    .await
    .expect("owner send should succeed");

    sleep(Duration::from_millis(250)).await;

    chat_input_writer
        .write_all(b"hello from guest\n/quit\n")
        .await
        .expect("chat-session input should write");
    chat_input_writer
        .shutdown()
        .await
        .expect("chat-session input should shutdown");

    timeout(Duration::from_secs(10), chat_session_task)
        .await
        .expect("chat-session task should complete before timeout")
        .expect("chat-session task should join")
        .expect("chat-session should succeed");

    let mut stdout = String::new();
    chat_output_reader
        .read_to_string(&mut stdout)
        .await
        .expect("chat-session output should be readable");
    assert!(
        stdout.contains("hello from owner"),
        "chat-session should render inbound owner message\nstdout:\n{stdout}"
    );
    assert!(
        stdout.contains("[guest] hello from guest"),
        "chat-session should render local guest send line\nstdout:\n{stdout}"
    );

    let timeline_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "timeline",
            "--conversation-id",
            "c_cli_chat_session_demo",
        ])
        .expect("timeline args should parse"),
    )
    .await
    .expect("timeline should succeed");
    let timeline_json = command_output_json(timeline_output);
    let summaries = timeline_json["items"]
        .as_array()
        .expect("timeline items should be array")
        .iter()
        .map(|item| item["summary"].as_str().unwrap_or_default().to_owned())
        .collect::<Vec<_>>();
    assert!(
        summaries
            .iter()
            .any(|summary| summary == "hello from guest"),
        "timeline should include guest message: {summaries:?}"
    );

    handle.abort();
    let _ = handle.await;
}

#[cfg(windows)]
#[tokio::test]
async fn test_chat_cli_powershell_entry_wrapper_can_send_interactive_messages() {
    unsafe {
        std::env::set_var(
            im_auth_context::PUBLIC_BEARER_HS256_SECRET_ENV,
            "local-chat-cli-secret",
        );
    }

    let app = local_minimal_node::build_public_app();
    let (base_url, handle) = spawn_server(app).await;

    execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "create-conversation",
            "--conversation-id",
            "c_cli_wrapper_chat_session_demo",
            "--conversation-type",
            "group",
        ])
        .expect("create args should parse"),
    )
    .await
    .expect("create conversation should succeed");

    execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "add-member",
            "--conversation-id",
            "c_cli_wrapper_chat_session_demo",
            "--principal-id",
            "u_guest",
            "--principal-kind",
            "user",
            "--role",
            "member",
        ])
        .expect("add-member args should parse"),
    )
    .await
    .expect("add member should succeed");

    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let wrapper_path = repo_root.join("bin").join("chat-cli.ps1");

    let mut child = Command::new("powershell.exe")
        .arg("-NoProfile")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(wrapper_path)
        .arg("--")
        .arg("--base-url")
        .arg(base_url.as_str())
        .arg("--tenant-id")
        .arg("t_demo")
        .arg("--user-id")
        .arg("u_guest")
        .arg("--session-id")
        .arg("s_guest")
        .arg("--device-id")
        .arg("d_guest")
        .arg("--public-bearer-secret")
        .arg("local-chat-cli-secret")
        .arg("chat-session")
        .arg("--conversation-id")
        .arg("c_cli_wrapper_chat_session_demo")
        .arg("--label")
        .arg("guest")
        .current_dir(&repo_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("powershell wrapper should spawn");

    let mut stdin = child.stdin.take().expect("wrapper stdin should exist");
    stdin
        .write_all(b"hello from wrapper test\n/quit\n")
        .expect("wrapper stdin should accept chat input");
    drop(stdin);

    let output = tokio::task::spawn_blocking(move || child.wait_with_output())
        .await
        .expect("wrapper wait task should join")
        .expect("wrapper process should complete");
    assert!(
        output.status.success(),
        "wrapper should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout_text = String::from_utf8_lossy(&output.stdout);
    let stderr_text = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stdout_text.contains("Finished `dev` profile")
            && !stdout_text.contains("Running `target\\debug\\craw-chat-cli.exe")
            && !stderr_text.contains("Finished `dev` profile")
            && !stderr_text.contains("Running `target\\debug\\craw-chat-cli.exe"),
        "wrapper must not leak cargo launcher output into interactive session\nstdout:\n{stdout_text}\nstderr:\n{stderr_text}"
    );

    let timeline_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "timeline",
            "--conversation-id",
            "c_cli_wrapper_chat_session_demo",
        ])
        .expect("timeline args should parse"),
    )
    .await
    .expect("timeline should succeed");
    let timeline_json = command_output_json(timeline_output);
    let summaries = timeline_json["items"]
        .as_array()
        .expect("timeline items should be array")
        .iter()
        .map(|item| item["summary"].as_str().unwrap_or_default().to_owned())
        .collect::<Vec<_>>();
    assert!(
        summaries
            .iter()
            .any(|summary| summary == "hello from wrapper test"),
        "timeline should include wrapper-sent message\nstdout:\n{}\nstderr:\n{}\nsummaries:\n{summaries:?}",
        stdout_text,
        stderr_text
    );

    handle.abort();
    let _ = handle.await;
}
