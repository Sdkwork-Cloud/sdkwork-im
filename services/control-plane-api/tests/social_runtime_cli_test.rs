use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use im_adapters_local_disk::FileCommitJournal;
use im_domain_core::social::direct_chat_pair_hash;
use im_domain_events::social::{DirectChatBoundPayload, SocialEventType, social_commit_envelope};
use im_domain_events::{AggregateType, EventActor};
use im_platform_contracts::CommitJournal;

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "craw_chat_control_plane_social_cli_runtime_{unique}_{sequence}"
    ))
}

fn state_file(runtime_dir: &Path, file_name: &str) -> PathBuf {
    runtime_dir.join("state").join(file_name)
}

fn social_tx_marker_file(runtime_dir: &Path) -> PathBuf {
    state_file(runtime_dir, "social-transaction-marker.json")
}

fn write_social_direct_chat_commit(runtime_dir: &Path) {
    let journal_path = state_file(runtime_dir, "social-commit-journal.json");
    let journal = FileCommitJournal::new("social", journal_path);
    let payload = DirectChatBoundPayload {
        direct_chat_id: "dc_social_cli_001".into(),
        conversation_id: "c_social_cli_001".into(),
        left_actor_id: "actor_alice".into(),
        right_actor_id: "actor_bob".into(),
        pair_hash: direct_chat_pair_hash("actor_alice", "actor_bob")
            .expect("direct chat pair hash should normalize"),
        bound_at: "2026-04-11T02:00:00Z".into(),
    };
    let payload_json =
        serde_json::to_string(&payload).expect("social direct chat payload should serialize");
    journal
        .append(social_commit_envelope(
            "evt_social_cli_001",
            "t_demo",
            AggregateType::DirectChat,
            payload.direct_chat_id.as_str(),
            SocialEventType::DirectChatBound,
            1,
            EventActor {
                actor_id: "operator_cli".into(),
                actor_kind: "operator".into(),
                actor_session_id: None,
            },
            payload.bound_at.as_str(),
            payload.bound_at.as_str(),
            payload_json.as_str(),
        ))
        .expect("social direct chat commit should append to journal");
}

fn run_control_plane_cli(args: &[&str]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_control-plane-api"))
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("control-plane-api binary should spawn");

    for _ in 0..20 {
        if child
            .try_wait()
            .expect("control-plane-api process status should be readable")
            .is_some()
        {
            return child
                .wait_with_output()
                .expect("control-plane-api output should collect");
        }
        thread::sleep(Duration::from_millis(100));
    }

    let _ = child.kill();
    let _ = child.wait();
    panic!("control-plane-api cli did not exit within timeout");
}

#[test]
fn test_control_plane_repair_social_runtime_dir_cli_replays_journal_into_snapshot() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(state_file(runtime_dir.as_path(), "").as_path())
        .expect("runtime state dir should be created");
    write_social_direct_chat_commit(runtime_dir.as_path());

    let output = run_control_plane_cli(&[
        "repair-social-runtime-dir",
        "--runtime-dir",
        runtime_dir
            .to_str()
            .expect("runtime dir should be valid utf-8"),
        "--json",
    ]);

    assert!(
        output.status.success(),
        "repair-social-runtime-dir should exit successfully. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("cli json output should be valid");
    assert_eq!(report["status"], "repaired");
    assert_eq!(report["journalAuthority"], true);
    assert_eq!(report["snapshotUpdated"], true);
    assert_eq!(report["transactionMarkerCleared"], false);
    assert_eq!(report["aggregateCounts"]["directChats"], 1);

    let snapshot_body = fs::read_to_string(state_file(runtime_dir.as_path(), "social-state.json"))
        .expect("social state snapshot should be materialized by cli repair");
    let snapshot_json: serde_json::Value =
        serde_json::from_str(&snapshot_body).expect("social state snapshot should be valid json");
    assert_eq!(
        snapshot_json["direct_chats"]["dc_social_cli_001"]["direct_chat"]["conversationId"],
        "c_social_cli_001"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_control_plane_repair_social_runtime_dir_cli_reports_transaction_marker_clearance_after_snapshot_failure()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(state_file(runtime_dir.as_path(), "").as_path())
        .expect("runtime state dir should be created");
    write_social_direct_chat_commit(runtime_dir.as_path());
    fs::write(
        social_tx_marker_file(runtime_dir.as_path()),
        r#"{
  "status":"pending_snapshot_repair",
  "eventId":"evt_social_cli_001"
}"#,
    )
    .expect("pending social tx marker should be written");

    let output = run_control_plane_cli(&[
        "repair-social-runtime-dir",
        "--runtime-dir",
        runtime_dir
            .to_str()
            .expect("runtime dir should be valid utf-8"),
        "--json",
    ]);

    assert!(
        output.status.success(),
        "repair-social-runtime-dir should exit successfully. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("cli json output should be valid");
    assert_eq!(report["status"], "repaired");
    assert_eq!(report["journalAuthority"], true);
    assert_eq!(report["snapshotUpdated"], true);
    assert_eq!(report["transactionMarkerCleared"], true);
    assert_eq!(report["aggregateCounts"]["directChats"], 1);
    assert!(
        !social_tx_marker_file(runtime_dir.as_path()).exists(),
        "pending social tx marker should be cleared by cli repair"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[test]
fn test_control_plane_repair_social_runtime_dir_cli_rejects_missing_runtime_dir_value() {
    let output = run_control_plane_cli(&["repair-social-runtime-dir", "--runtime-dir"]);

    assert!(
        !output.status.success(),
        "repair-social-runtime-dir must fail when --runtime-dir has no value. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--runtime-dir requires a value"),
        "cli stderr should explain missing --runtime-dir value. stderr: {stderr}"
    );
    assert!(
        !stderr.to_lowercase().contains("panicked"),
        "cli must return a controlled error instead of panic. stderr: {stderr}"
    );
}
