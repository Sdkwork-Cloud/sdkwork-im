use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::*;
use im_domain_core::realtime::RealtimeSubscription;
use im_domain_core::session::{DevicePresenceStatus, DevicePresenceView};
use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, CommitEnvelope, CommitJournal,
    ContractError, NotificationTaskRecord, NotificationTaskStore, PresenceStateRecord,
    PresenceStateStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore, RealtimeSubscriptionRecord,
    RealtimeSubscriptionStore, RtcStateRecord, RtcStateStore, StreamStateRecord, StreamStateStore,
};

fn unique_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_disconnect_fence_store_{unique}.json"))
}

fn unique_checkpoint_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_realtime_checkpoint_store_{unique}.json"))
}

fn unique_subscription_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "craw_chat_realtime_subscription_store_{unique}.json"
    ))
}

fn unique_commit_journal_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_commit_journal_{unique}.json"))
}

fn unique_stream_state_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_stream_state_store_{unique}.json"))
}

fn unique_rtc_state_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_rtc_state_store_{unique}.json"))
}

fn unique_notification_task_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_notification_task_store_{unique}.json"))
}

fn unique_automation_execution_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "craw_chat_automation_execution_store_{unique}.json"
    ))
}

fn unique_presence_state_store_file() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_presence_state_store_{unique}.json"))
}

#[test]
fn test_file_commit_journal_persists_across_reopen() {
    let file_path = unique_commit_journal_file();
    let journal = FileCommitJournal::new("local-minimal", &file_path);
    journal
        .append(CommitEnvelope::minimal(
            "evt_demo_1",
            "t_demo",
            "conversation.created",
            "conversation",
            "c_demo",
            0,
        ))
        .expect("append should succeed");
    journal
        .append(CommitEnvelope::minimal(
            "evt_demo_2",
            "t_demo",
            "message.posted",
            "conversation",
            "c_demo",
            1,
        ))
        .expect("append should succeed");

    let reopened = FileCommitJournal::new("local-minimal", &file_path);
    let recorded = reopened.recorded().expect("recorded should succeed");
    assert_eq!(recorded.len(), 2);
    assert_eq!(recorded[0].event_id, "evt_demo_1");
    assert_eq!(recorded[1].event_id, "evt_demo_2");

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_read_commit_journal_file_restores_minimal_events() {
    let file_path = unique_commit_journal_file();
    fs::write(
        &file_path,
        serde_json::to_vec_pretty(&vec![CommitEnvelope::minimal(
            "evt_demo_1",
            "t_demo",
            "conversation.created",
            "conversation",
            "c_demo",
            0,
        )])
        .expect("journal payload should serialize"),
    )
    .expect("journal file should be written");

    let restored = read_commit_journal_file(&file_path).expect("journal should parse");
    assert_eq!(restored.len(), 1);
    assert_eq!(restored[0].event_id, "evt_demo_1");

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_validate_checkpoint_store_file_rejects_array_shape() {
    let file_path = unique_checkpoint_store_file();
    fs::write(&file_path, b"[]").expect("checkpoint file should be written");

    let error = validate_realtime_checkpoint_store_file(&file_path)
        .expect_err("array-shaped checkpoint store should be rejected");
    assert!(matches!(error, ContractError::Unavailable(_)));
    let message = match error {
        ContractError::Unavailable(message) => message,
        other => panic!("unexpected error variant: {other:?}"),
    };
    assert!(message.contains("failed to parse realtime checkpoint store"));

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_checkpoint_store_persists_across_reopen() {
    let file_path = unique_checkpoint_store_file();
    let store = FileRealtimeCheckpointStore::new(&file_path);
    store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 7,
            acked_through_seq: 5,
            trimmed_through_seq: 5,
            updated_at: "2026-04-06T00:00:00.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileRealtimeCheckpointStore::new(&file_path);
    let restored = reopened
        .load_checkpoint("t_demo", "u_demo", "d_pad")
        .expect("load should succeed")
        .expect("checkpoint should exist");
    assert_eq!(restored.latest_realtime_seq, 7);
    assert_eq!(restored.acked_through_seq, 5);
    assert_eq!(restored.trimmed_through_seq, 5);

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_disconnect_fence_store_persists_and_clears_across_reopen() {
    let file_path = unique_store_file();
    let store = FileRealtimeDisconnectFenceStore::new(&file_path);
    store
        .save_fence(RealtimeDisconnectFenceRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            session_id: Some("s_old".into()),
            owner_node_id: "node_a".into(),
            disconnected_at: "2026-04-06T00:00:00.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileRealtimeDisconnectFenceStore::new(&file_path);
    let restored = reopened
        .load_fence("t_demo", "u_demo", "d_pad")
        .expect("load should succeed")
        .expect("fence should exist");
    assert_eq!(restored.session_id.as_deref(), Some("s_old"));
    assert_eq!(restored.owner_node_id, "node_a");

    assert!(
        reopened
            .clear_fence("t_demo", "u_demo", "d_pad")
            .expect("clear should succeed")
    );

    let reopened_after_clear = FileRealtimeDisconnectFenceStore::new(&file_path);
    assert!(
        reopened_after_clear
            .load_fence("t_demo", "u_demo", "d_pad")
            .expect("load after clear should succeed")
            .is_none()
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_subscription_store_persists_across_reopen() {
    let file_path = unique_subscription_store_file();
    let store = FileRealtimeSubscriptionStore::new(&file_path);
    store
        .save_subscriptions(RealtimeSubscriptionRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            items: vec![RealtimeSubscription {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
                subscribed_at: "2026-04-06T00:00:00.000Z".into(),
            }],
            synced_at: "2026-04-06T00:00:00.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileRealtimeSubscriptionStore::new(&file_path);
    let restored = reopened
        .load_subscriptions("t_demo", "u_demo", "d_pad")
        .expect("load should succeed")
        .expect("subscriptions should exist");
    assert_eq!(restored.items.len(), 1);
    assert_eq!(restored.items[0].scope_id, "c_demo");
    assert_eq!(restored.items[0].event_types, vec!["message.posted"]);
    assert_eq!(restored.synced_at, "2026-04-06T00:00:00.000Z");

    assert!(
        reopened
            .clear_subscriptions("t_demo", "u_demo", "d_pad")
            .expect("clear should succeed")
    );

    let reopened_after_clear = FileRealtimeSubscriptionStore::new(&file_path);
    assert!(
        reopened_after_clear
            .load_subscriptions("t_demo", "u_demo", "d_pad")
            .expect("load after clear should succeed")
            .is_none()
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_stream_state_store_persists_across_reopen() {
    let file_path = unique_stream_state_store_file();
    let store = FileStreamStateStore::new(&file_path);
    store
        .save_state(StreamStateRecord {
            tenant_id: "t_demo".into(),
            stream_id: "st_demo".into(),
            session: im_domain_core::stream::StreamSession {
                tenant_id: "t_demo".into(),
                stream_id: "st_demo".into(),
                stream_type: "custom.delta.text".into(),
                scope_kind: "request".into(),
                scope_id: "req_demo".into(),
                durability_class: im_domain_core::stream::StreamDurabilityClass::DurableSession,
                ordering_scope: "stream".into(),
                schema_ref: Some("custom.delta.text.v1".into()),
                state: im_domain_core::stream::StreamSessionState::Active,
                last_frame_seq: 1,
                last_checkpoint_seq: Some(1),
                result_message_id: None,
                opened_at: "2026-04-06T00:00:00.000Z".into(),
                closed_at: None,
                expires_at: None,
            },
            frames: vec![im_domain_core::stream::StreamFrame {
                tenant_id: "t_demo".into(),
                stream_id: "st_demo".into(),
                stream_type: "custom.delta.text".into(),
                scope_kind: "request".into(),
                scope_id: "req_demo".into(),
                frame_seq: 1,
                frame_type: "delta".into(),
                schema_ref: Some("custom.delta.text.v1".into()),
                encoding: "json".into(),
                payload: "{\"delta\":\"hello\"}".into(),
                sender: im_domain_core::message::Sender {
                    id: "u_demo".into(),
                    kind: "user".into(),
                    member_id: None,
                    device_id: Some("d_demo".into()),
                    session_id: Some("s_demo".into()),
                    metadata: BTreeMap::new(),
                },
                attributes: BTreeMap::new(),
                occurred_at: "2026-04-06T00:00:00.000Z".into(),
            }],
            updated_at: "2026-04-06T00:00:00.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileStreamStateStore::new(&file_path);
    let restored = reopened
        .load_state("t_demo", "st_demo")
        .expect("load should succeed")
        .expect("stream state should exist");
    assert_eq!(restored.session.last_frame_seq, 1);
    assert_eq!(restored.frames.len(), 1);
    assert_eq!(restored.frames[0].frame_seq, 1);

    assert!(
        reopened
            .clear_state("t_demo", "st_demo")
            .expect("clear should succeed")
    );

    let reopened_after_clear = FileStreamStateStore::new(&file_path);
    assert!(
        reopened_after_clear
            .load_state("t_demo", "st_demo")
            .expect("load after clear should succeed")
            .is_none()
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_rtc_state_store_persists_across_reopen() {
    let file_path = unique_rtc_state_store_file();
    let store = FileRtcStateStore::new(&file_path);
    store
        .save_state(RtcStateRecord {
            tenant_id: "t_demo".into(),
            rtc_session_id: "rtc_demo".into(),
            session: im_domain_core::rtc::RtcSession {
                tenant_id: "t_demo".into(),
                rtc_session_id: "rtc_demo".into(),
                conversation_id: Some("c_demo".into()),
                rtc_mode: "voice".into(),
                initiator_id: "u_demo".into(),
                provider_plugin_id: Some("webrtc".into()),
                provider_session_id: Some("ps_demo".into()),
                access_endpoint: Some("wss://rtc.example.test/session/ps_demo".into()),
                provider_region: Some("cn-shanghai".into()),
                state: im_domain_core::rtc::RtcSessionState::Accepted,
                signaling_stream_id: Some("st_demo".into()),
                artifact_message_id: Some("msg_accept".into()),
                started_at: "2026-04-06T00:00:00.000Z".into(),
                ended_at: None,
            },
            signals: vec![im_domain_core::rtc::RtcSignalEvent {
                tenant_id: "t_demo".into(),
                rtc_session_id: "rtc_demo".into(),
                conversation_id: Some("c_demo".into()),
                rtc_mode: "voice".into(),
                signal_type: "rtc.offer".into(),
                schema_ref: Some("webrtc.offer.v1".into()),
                payload: "{\"sdp\":\"offer\"}".into(),
                sender: im_domain_core::message::Sender {
                    id: "u_demo".into(),
                    kind: "user".into(),
                    member_id: None,
                    device_id: Some("d_demo".into()),
                    session_id: Some("s_demo".into()),
                    metadata: BTreeMap::new(),
                },
                signaling_stream_id: Some("st_demo".into()),
                occurred_at: "2026-04-06T00:00:01.000Z".into(),
            }],
            updated_at: "2026-04-06T00:00:02.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileRtcStateStore::new(&file_path);
    let restored = reopened
        .load_state("t_demo", "rtc_demo")
        .expect("load should succeed")
        .expect("rtc state should exist");
    assert_eq!(
        restored.session.state,
        im_domain_core::rtc::RtcSessionState::Accepted
    );
    assert_eq!(
        restored.session.signaling_stream_id.as_deref(),
        Some("st_demo")
    );
    assert_eq!(restored.session.provider_plugin_id.as_deref(), Some("webrtc"));
    assert_eq!(
        restored.session.access_endpoint.as_deref(),
        Some("wss://rtc.example.test/session/ps_demo")
    );
    assert_eq!(restored.signals.len(), 1);
    assert_eq!(restored.signals[0].signal_type, "rtc.offer");

    assert!(
        reopened
            .clear_state("t_demo", "rtc_demo")
            .expect("clear should succeed")
    );

    let reopened_after_clear = FileRtcStateStore::new(&file_path);
    assert!(
        reopened_after_clear
            .load_state("t_demo", "rtc_demo")
            .expect("load after clear should succeed")
            .is_none()
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_notification_task_store_persists_across_reopen() {
    let file_path = unique_notification_task_store_file();
    let store = FileNotificationTaskStore::new(&file_path);
    store
        .save_task(NotificationTaskRecord {
            tenant_id: "t_demo".into(),
            notification_id: "ntf_demo".into(),
            task: im_domain_core::notification::NotificationTask {
                tenant_id: "t_demo".into(),
                notification_id: "ntf_demo".into(),
                source_event_id: "evt_demo".into(),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: "u_demo".into(),
                status: im_domain_core::notification::NotificationStatus::Dispatched,
                title: Some("hello".into()),
                body: Some("world".into()),
                payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                requested_at: "2026-04-06T00:00:00.000Z".into(),
                dispatched_at: Some("2026-04-06T00:00:01.000Z".into()),
                failure_reason: None,
            },
            updated_at: "2026-04-06T00:00:01.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileNotificationTaskStore::new(&file_path);
    let restored = reopened
        .load_task("t_demo", "ntf_demo")
        .expect("load should succeed")
        .expect("notification task should exist");
    assert_eq!(restored.task.notification_id, "ntf_demo");
    assert_eq!(restored.task.recipient_id, "u_demo");

    let listed = reopened
        .list_tasks_for_recipient("t_demo", "u_demo")
        .expect("list should succeed");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].notification_id, "ntf_demo");

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_automation_execution_store_persists_across_reopen() {
    let file_path = unique_automation_execution_store_file();
    let store = FileAutomationExecutionStore::new(&file_path);
    store
        .save_execution(AutomationExecutionRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            execution_id: "ae_demo".into(),
            execution: im_domain_core::automation::AutomationExecution {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                principal_kind: "user".into(),
                execution_id: "ae_demo".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                output_payload: Some("{\"accepted\":true}".into()),
                state: im_domain_core::automation::AutomationExecutionState::Succeeded,
                retry_count: 0,
                requested_at: "2026-04-06T00:00:00.000Z".into(),
                completed_at: Some("2026-04-06T00:00:01.000Z".into()),
                failure_reason: None,
            },
            updated_at: "2026-04-06T00:00:01.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FileAutomationExecutionStore::new(&file_path);
    let restored = reopened
        .load_execution("t_demo", "u_demo", "ae_demo")
        .expect("load should succeed")
        .expect("automation execution should exist");
    assert_eq!(restored.execution.execution_id, "ae_demo");
    assert_eq!(restored.execution.principal_id, "u_demo");
    assert_eq!(
        restored.execution.state,
        im_domain_core::automation::AutomationExecutionState::Succeeded
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_presence_state_store_persists_across_reopen() {
    let file_path = unique_presence_state_store_file();
    let store = FilePresenceStateStore::new(&file_path);
    store
        .save_state(PresenceStateRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            presence: DevicePresenceView {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                device_id: "d_pad".into(),
                platform: None,
                session_id: None,
                status: DevicePresenceStatus::Offline,
                last_sync_seq: 7,
                last_resume_at: Some("2026-04-06T00:00:00.000Z".into()),
                last_seen_at: Some("2026-04-06T00:00:01.000Z".into()),
            },
            resume_required: true,
            updated_at: "2026-04-06T00:00:01.000Z".into(),
        })
        .expect("save should succeed");
    store
        .save_state(PresenceStateRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_phone".into(),
            presence: DevicePresenceView {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                device_id: "d_phone".into(),
                platform: None,
                session_id: None,
                status: DevicePresenceStatus::Offline,
                last_sync_seq: 0,
                last_resume_at: None,
                last_seen_at: None,
            },
            resume_required: false,
            updated_at: "2026-04-06T00:00:02.000Z".into(),
        })
        .expect("save should succeed");

    let reopened = FilePresenceStateStore::new(&file_path);
    let restored = reopened
        .load_state("t_demo", "u_demo", "d_pad")
        .expect("load should succeed")
        .expect("presence state should exist");
    assert_eq!(restored.device_id, "d_pad");
    assert!(restored.resume_required);
    assert_eq!(restored.presence.last_sync_seq, 7);

    let listed = reopened
        .list_states_for_principal("t_demo", "u_demo")
        .expect("list should succeed");
    assert_eq!(listed.len(), 2);
    assert!(listed.iter().any(|record| record.device_id == "d_pad"));
    assert!(listed.iter().any(|record| record.device_id == "d_phone"));

    let _ = fs::remove_file(file_path);
}
