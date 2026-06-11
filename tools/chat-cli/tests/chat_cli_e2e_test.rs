use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

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

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_cli_real_auth_runtime_{unique}"))
}

fn resolve_usable_bash() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    #[cfg(windows)]
    {
        candidates.push(PathBuf::from(r"C:\Program Files\Git\usr\bin\bash.exe"));
        candidates.push(PathBuf::from(r"C:\Program Files\Git\bin\bash.exe"));
        if let Ok(output) = Command::new("where").arg("bash").output()
            && output.status.success()
        {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed
                    .to_ascii_lowercase()
                    .contains(r"\windows\system32\bash.exe")
                {
                    continue;
                }
                candidates.push(PathBuf::from(trimmed));
            }
        }
    }
    #[cfg(not(windows))]
    candidates.push(PathBuf::from("bash"));

    const BASH_PROBE_SENTINEL: &str = "craw_chat_cli_bash_probe_ok";
    const BASH_PROBE_SCRIPT: &str = "command -v grep >/dev/null 2>&1 && command -v sed >/dev/null 2>&1 && command -v mktemp >/dev/null 2>&1 && printf craw_chat_cli_bash_probe_ok";

    candidates.into_iter().find(|candidate| {
        let version_ok = Command::new(candidate)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false);
        if !version_ok {
            return false;
        }

        Command::new(candidate)
            .arg("-lc")
            .arg(BASH_PROBE_SCRIPT)
            .output()
            .map(|output| {
                output.status.success() && output.stdout.starts_with(BASH_PROBE_SENTINEL.as_bytes())
            })
            .unwrap_or(false)
    })
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

async fn run_real_login_watch_validation_flow(
    base_url: &str,
    conversation_id: &str,
    owner_session_id: &str,
    owner_device_id: &str,
    guest_session_id: &str,
    guest_device_id: &str,
    validation_message: &str,
) {
    let owner_user_id = "u_owner".to_owned();
    let guest_user_id = "u_guest".to_owned();

    let create_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url,
            "--tenant-id",
            "t_demo",
            "--user-id",
            owner_user_id.as_str(),
            "--session-id",
            owner_session_id,
            "--device-id",
            owner_device_id,
            "create-conversation",
            "--conversation-id",
            conversation_id,
            "--conversation-type",
            "group",
        ])
        .expect("create conversation args should parse"),
    )
    .await
    .expect("conversation creation should succeed");
    let create_json = command_output_json(create_output);
    assert_eq!(create_json["conversationId"], conversation_id);

    let add_member_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url,
            "--tenant-id",
            "t_demo",
            "--user-id",
            owner_user_id.as_str(),
            "--session-id",
            owner_session_id,
            "--device-id",
            owner_device_id,
            "add-member",
            "--conversation-id",
            conversation_id,
            "--principal-id",
            guest_user_id.as_str(),
            "--principal-kind",
            "user",
            "--role",
            "member",
        ])
        .expect("add member args should parse"),
    )
    .await
    .expect("member add should succeed");
    let add_member_json = command_output_json(add_member_output);
    assert_eq!(add_member_json["principalId"], guest_user_id);

    let watch_base_url = base_url.to_owned();
    let watch_conversation_id = conversation_id.to_owned();
    let watch_guest_user_id = guest_user_id.clone();
    let watch_guest_session_id = guest_session_id.to_owned();
    let watch_guest_device_id = guest_device_id.to_owned();
    let watch_task = tokio::spawn(async move {
        execute_command(
            parse_cli_args([
                "craw-chat-cli",
                "--base-url",
                watch_base_url.as_str(),
                "--tenant-id",
                "t_demo",
                "--user-id",
                watch_guest_user_id.as_str(),
                "--session-id",
                watch_guest_session_id.as_str(),
                "--device-id",
                watch_guest_device_id.as_str(),
                "watch",
                "--conversation-id",
                watch_conversation_id.as_str(),
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
            base_url,
            "--tenant-id",
            "t_demo",
            "--user-id",
            owner_user_id.as_str(),
            "--session-id",
            owner_session_id,
            "--device-id",
            owner_device_id,
            "send-message",
            "--conversation-id",
            conversation_id,
            "--summary",
            validation_message,
            "--text",
            validation_message,
            "--client-msg-id",
            "cli_real_auth_watch_msg_1",
        ])
        .expect("send args should parse"),
    )
    .await
    .expect("message send should succeed");
    let send_json = command_output_json(send_output);
    assert_eq!(send_json["summary"], validation_message);

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
        .expect("watch should observe push event window after real login");
    let payload: Value = serde_json::from_str(
        pushed_window["window"]["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["summary"], validation_message);

    let timeline_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url,
            "--tenant-id",
            "t_demo",
            "--user-id",
            guest_user_id.as_str(),
            "--session-id",
            guest_session_id,
            "--device-id",
            guest_device_id,
            "timeline",
            "--conversation-id",
            conversation_id,
        ])
        .expect("timeline args should parse"),
    )
    .await
    .expect("timeline should succeed");
    let timeline_json = command_output_json(timeline_output);
    assert!(
        timeline_json["items"]
            .as_array()
            .expect("timeline items should be an array")
            .iter()
            .any(|item| item["summary"] == validation_message),
        "timeline should contain validation message after real-login flow"
    );
}

async fn login_seeded_im_user(
    base_url: &str,
    user_id: &str,
    _password: &str,
    session_id: &str,
    device_id: &str,
) -> (String, String) {
    let token_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url,
            "--tenant-id",
            "t_demo",
            "--user-id",
            user_id,
            "--session-id",
            session_id,
            "--device-id",
            device_id,
            "token",
            "--token-only",
        ])
        .expect("seeded token args should parse"),
    )
    .await
    .expect("seeded token should resolve");
    let token_json = command_output_json(token_output);
    let access_token = token_json["token"]
        .as_str()
        .expect("seeded token command should return access token")
        .to_owned();
    (user_id.to_owned(), access_token)
}

async fn prepare_real_login_conversation(base_url: &str, conversation_id: &str) {
    let (owner_user_id, owner_bearer) = login_seeded_im_user(
        base_url,
        "u_owner",
        "Owner#2026",
        "s_owner_setup",
        "d_owner_setup",
    )
    .await;

    let create_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url,
            "--tenant-id",
            "t_demo",
            "--user-id",
            owner_user_id.as_str(),
            "--session-id",
            "s_owner_setup",
            "--device-id",
            "d_owner_setup",
            "create-conversation",
            "--conversation-id",
            conversation_id,
            "--conversation-type",
            "group",
        ])
        .expect("real-login create args should parse"),
    )
    .await
    .expect("real-login conversation create should succeed");
    let create_json = command_output_json(create_output);
    assert_eq!(create_json["conversationId"], conversation_id);

    let add_member_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url,
            "--tenant-id",
            "t_demo",
            "--user-id",
            owner_user_id.as_str(),
            "--session-id",
            "s_owner_setup",
            "--device-id",
            "d_owner_setup",
            "add-member",
            "--conversation-id",
            conversation_id,
            "--principal-id",
            "u_guest",
            "--principal-kind",
            "user",
            "--role",
            "member",
        ])
        .expect("real-login add-member args should parse"),
    )
    .await
    .expect("real-login add member should succeed");
    let add_member_json = command_output_json(add_member_output);
    assert_eq!(add_member_json["principalId"], "u_guest");
}

#[tokio::test]
async fn test_chat_cli_can_drive_two_party_http_and_websocket_validation_flow() {
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
async fn test_chat_cli_keeps_real_login_watch_flow_healthy_after_runtime_dir_restart() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let (base_url_before, handle_before) = spawn_server(app_before).await;
    run_real_login_watch_validation_flow(
        base_url_before.as_str(),
        "c_cli_real_auth_restart_before",
        "s_owner_live",
        "d_owner_live",
        "s_guest_live",
        "d_guest_live",
        "before restart real login watch",
    )
    .await;
    handle_before.abort();
    let _ = handle_before.await;

    let app_after = local_minimal_node::build_public_app_with_runtime_dir(runtime_dir.as_path());
    let (base_url_after, handle_after) = spawn_server(app_after).await;
    run_real_login_watch_validation_flow(
        base_url_after.as_str(),
        "c_cli_real_auth_restart_after",
        "s_owner_live",
        "d_owner_live",
        "s_guest_live",
        "d_guest_live",
        "after restart real login watch",
    )
    .await;
    handle_after.abort();
    let _ = handle_after.await;

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_chat_cli_chat_session_can_receive_and_send_messages_before_quit() {
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
#[cfg(windows)]
#[cfg(windows)]
#[tokio::test]
async fn test_chat_window_cmd_wrapper_accepts_gnu_style_named_flags_for_interactive_session() {
    let app = local_minimal_node::build_public_app();
    let (base_url, handle) = spawn_server(app).await;

    prepare_real_login_conversation(base_url.as_str(), "c_cli_chat_window_cmd_gnu_demo").await;

    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let wrapper_path = repo_root.join("bin").join("chat-window.cmd");

    let mut child = Command::new("cmd.exe")
        .arg("/c")
        .arg(wrapper_path)
        .arg("--base-url")
        .arg(base_url.as_str())
        .arg("--tenant-id")
        .arg("t_demo")
        .arg("--conversation-id")
        .arg("c_cli_chat_window_cmd_gnu_demo")
        .arg("--user-id")
        .arg("u_guest")
        .arg("--session-id")
        .arg("s_guest")
        .arg("--device-id")
        .arg("d_guest")
        .arg("--label")
        .arg("guest-gnu")
        .arg("--message-prefix")
        .arg("[gnu] ")
        .current_dir(&repo_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("chat-window.cmd wrapper should spawn");

    let mut stdin = child.stdin.take().expect("wrapper stdin should exist");
    stdin
        .write_all(b"hello from chat-window cmd gnu\n/quit\n")
        .expect("chat-window.cmd stdin should accept chat input");
    drop(stdin);

    let output = timeout(
        Duration::from_secs(20),
        tokio::task::spawn_blocking(move || child.wait_with_output()),
    )
    .await
    .expect("chat-window.cmd wrapper should complete before timeout")
    .expect("chat-window.cmd wait task should join")
    .expect("chat-window.cmd should complete");

    assert!(
        output.status.success(),
        "chat-window.cmd interactive session should exit successfully when called with gnu-style named flags\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout_text = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout_text.contains(
            format!(
                "Opening chat session: conversation=c_cli_chat_window_cmd_gnu_demo user=u_guest label=guest-gnu baseUrl={base_url}"
            )
            .as_str()
        ),
        "chat-window.cmd must preserve the GNU-style launch contract for conversation, user, label, and base-url\nstdout:\n{}\nstderr:\n{}",
        stdout_text,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !stdout_text.contains("authorization_signature_invalid")
            && !String::from_utf8_lossy(&output.stderr).contains("authorization_signature_invalid"),
        "chat-window.cmd should not depend on a poisoned inherited public bearer secret when seeded real login is available\nstdout:\n{}\nstderr:\n{}",
        stdout_text,
        String::from_utf8_lossy(&output.stderr)
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
            "timeline",
            "--conversation-id",
            "c_cli_chat_window_cmd_gnu_demo",
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
            .any(|summary| summary == "[gnu] hello from chat-window cmd gnu"),
        "chat-window.cmd must preserve --message-prefix when called with gnu-style named flags: {summaries:?}"
    );

    handle.abort();
    let _ = handle.await;
}

#[cfg(windows)]
#[tokio::test]
async fn test_chat_window_cmd_wrapper_preserves_exclamation_mark_in_message_prefix() {
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
            "create-conversation",
            "--conversation-id",
            "c_cli_chat_window_cmd_bang_demo",
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
            "add-member",
            "--conversation-id",
            "c_cli_chat_window_cmd_bang_demo",
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
    let wrapper_path = repo_root.join("bin").join("chat-window.cmd");

    let mut child = Command::new("cmd.exe")
        .arg("/c")
        .arg(wrapper_path)
        .arg("--base-url")
        .arg(base_url.as_str())
        .arg("--tenant-id")
        .arg("t_demo")
        .arg("--conversation-id")
        .arg("c_cli_chat_window_cmd_bang_demo")
        .arg("--user-id")
        .arg("u_guest")
        .arg("--session-id")
        .arg("s_guest")
        .arg("--device-id")
        .arg("d_guest")
        .arg("--label")
        .arg("guest-bang")
        .arg("--message-prefix")
        .arg("[bang!] ")
        .current_dir(&repo_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("chat-window.cmd bang wrapper should spawn");

    let mut stdin = child.stdin.take().expect("wrapper stdin should exist");
    stdin
        .write_all(b"hello from chat-window cmd bang\n/quit\n")
        .expect("chat-window.cmd bang stdin should accept chat input");
    drop(stdin);

    let output = timeout(
        Duration::from_secs(20),
        tokio::task::spawn_blocking(move || child.wait_with_output()),
    )
    .await
    .expect("chat-window.cmd bang wrapper should complete before timeout")
    .expect("chat-window.cmd bang wait task should join")
    .expect("chat-window.cmd bang run should complete");

    assert!(
        output.status.success(),
        "chat-window.cmd interactive session with exclamation prefix should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
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
            "timeline",
            "--conversation-id",
            "c_cli_chat_window_cmd_bang_demo",
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
            .any(|summary| summary == "[bang!] hello from chat-window cmd bang"),
        "chat-window.cmd must preserve ! in --message-prefix: {summaries:?}"
    );

    handle.abort();
    let _ = handle.await;
}

#[cfg(windows)]
#[tokio::test]
async fn test_chat_cli_cmd_wrapper_preserves_help_contract() {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let wrapper_path = repo_root.join("bin").join("chat-cli.cmd");

    let output = tokio::task::spawn_blocking(move || {
        Command::new("cmd.exe")
            .arg("/c")
            .arg(wrapper_path)
            .arg("--help")
            .current_dir(repo_root)
            .output()
    })
    .await
    .expect("cmd wrapper wait task should join")
    .expect("cmd wrapper process should complete");

    assert!(
        output.status.success(),
        "cmd wrapper help should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout_text = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout_text.contains("Usage: craw-chat-cli"),
        "cmd wrapper help must preserve the CLI usage surface\nstdout:\n{}\nstderr:\n{}",
        stdout_text,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(windows)]
#[tokio::test]
async fn test_start_local_cmd_help_surfaces_gnu_style_named_flags() {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let wrapper_path = repo_root.join("bin").join("start-local.cmd");

    let output = tokio::task::spawn_blocking(move || {
        Command::new("cmd.exe")
            .arg("/c")
            .arg(wrapper_path)
            .arg("--help")
            .current_dir(repo_root)
            .output()
    })
    .await
    .expect("start-local.cmd help wait task should join")
    .expect("start-local.cmd help process should complete");

    assert!(
        output.status.success(),
        "start-local.cmd help should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout_text = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout_text.contains("--release")
            && stdout_text.contains("--foreground")
            && stdout_text.contains("--bind-addr"),
        "start-local.cmd help must surface the Windows GNU-style operator contract\nstdout:\n{}\nstderr:\n{}",
        stdout_text,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(windows)]
#[tokio::test]
async fn test_status_local_cmd_help_surfaces_gnu_style_named_flags() {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let wrapper_path = repo_root.join("bin").join("status-local.cmd");

    let output = tokio::task::spawn_blocking(move || {
        Command::new("cmd.exe")
            .arg("/c")
            .arg(wrapper_path)
            .arg("--help")
            .current_dir(repo_root)
            .output()
    })
    .await
    .expect("status-local.cmd help wait task should join")
    .expect("status-local.cmd help process should complete");

    assert!(
        output.status.success(),
        "status-local.cmd help should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout_text = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout_text.contains("--profile") && stdout_text.contains("--runtime_dir"),
        "status-local.cmd help must surface the Windows GNU-style operator contract\nstdout:\n{}\nstderr:\n{}",
        stdout_text,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(windows)]
#[tokio::test]
async fn test_chat_window_cmd_help_surfaces_gnu_style_named_flags() {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root should exist")
        .to_path_buf();
    let wrapper_path = repo_root.join("bin").join("chat-window.cmd");

    let output = tokio::task::spawn_blocking(move || {
        Command::new("cmd.exe")
            .arg("/c")
            .arg(wrapper_path)
            .arg("--help")
            .current_dir(repo_root)
            .output()
    })
    .await
    .expect("chat-window.cmd help wait task should join")
    .expect("chat-window.cmd help process should complete");

    assert!(
        output.status.success(),
        "chat-window.cmd help should exit successfully\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout_text = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout_text.contains("--conversation-id")
            && stdout_text.contains("--user-id")
            && stdout_text.contains("--message-prefix")
            && stdout_text.contains("--login")
            && stdout_text.contains("--password"),
        "chat-window.cmd help must surface the Windows GNU-style launch contract\nstdout:\n{}\nstderr:\n{}",
        stdout_text,
        String::from_utf8_lossy(&output.stderr)
    );
}
