use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use im_adapters_local_memory::{MemoryRealtimeCheckpointStore, MemoryRealtimeSubscriptionStore};
use im_domain_core::realtime::RealtimeEvent;
use im_platform_contracts::{RealtimeCheckpointRecord, RealtimeCheckpointStore};
use session_gateway::{
    RealtimeDeliveryRuntime, RealtimeRuntimeError, RealtimeSubscriptionItemInput,
};

fn expect_ok<T>(result: Result<T, RealtimeRuntimeError>) -> T {
    result.expect("realtime runtime operation should succeed")
}

#[test]
fn test_ack_events_trims_window_and_tracks_checkpoint() {
    let runtime = RealtimeDeliveryRuntime::default();
    expect_ok(runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let delivered = expect_ok(runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(delivered, 1);

    let ack = expect_ok(runtime.ack_events("t_demo", "u_demo", "d_pad", 1));
    assert_eq!(ack.acked_through_seq, 1);
    assert_eq!(ack.trimmed_through_seq, 1);
    assert_eq!(ack.retained_event_count, 0);

    let window = expect_ok(runtime.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(window.items.len(), 0);
    assert_eq!(window.acked_through_seq, 1);
    assert_eq!(window.trimmed_through_seq, 1);
    assert!(!window.has_more);
}

#[test]
fn test_ack_events_is_monotonic_and_clamped_to_latest_sequence() {
    let runtime = RealtimeDeliveryRuntime::default();
    expect_ok(runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    expect_ok(runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));

    let first_ack = expect_ok(runtime.ack_events("t_demo", "u_demo", "d_pad", 99));
    assert_eq!(first_ack.acked_through_seq, 1);
    assert_eq!(first_ack.trimmed_through_seq, 1);

    let second_ack = expect_ok(runtime.ack_events("t_demo", "u_demo", "d_pad", 0));
    assert_eq!(second_ack.acked_through_seq, 1);
    assert_eq!(second_ack.trimmed_through_seq, 1);
}

#[test]
fn test_runtime_persists_checkpoint_and_recovers_on_rebuild() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store(checkpoint_store.clone());
    expect_ok(runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    expect_ok(runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    expect_ok(runtime.ack_events("t_demo", "u_demo", "d_pad", 1));

    let persisted = checkpoint_store
        .checkpoint("t_demo", "u_demo", "d_pad")
        .expect("checkpoint should be persisted");
    assert_eq!(persisted.latest_realtime_seq, 1);
    assert_eq!(persisted.acked_through_seq, 1);
    assert_eq!(persisted.trimmed_through_seq, 1);

    let rebuilt_runtime = RealtimeDeliveryRuntime::with_checkpoint_store(checkpoint_store);
    let restored = expect_ok(rebuilt_runtime.window_checkpoint("t_demo", "u_demo", "d_pad"));
    assert_eq!(restored.latest_realtime_seq, 1);
    assert_eq!(restored.acked_through_seq, 1);
    assert_eq!(restored.trimmed_through_seq, 1);

    expect_ok(rebuilt_runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));
    let delivered = expect_ok(rebuilt_runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_2"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(delivered, 1);

    let after_rebuild = expect_ok(rebuilt_runtime.window_checkpoint("t_demo", "u_demo", "d_pad"));
    assert_eq!(after_rebuild.latest_realtime_seq, 2);
}

#[test]
fn test_runtime_restores_persisted_subscriptions_on_rebuild_without_resync() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let subscription_store = Arc::new(MemoryRealtimeSubscriptionStore::default());
    let runtime =
        RealtimeDeliveryRuntime::with_stores(checkpoint_store.clone(), subscription_store.clone());
    expect_ok(runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let rebuilt_runtime =
        RealtimeDeliveryRuntime::with_stores(checkpoint_store, subscription_store);
    expect_ok(rebuilt_runtime.ensure_device_state("t_demo", "u_demo", "d_pad"));

    let delivered = expect_ok(rebuilt_runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(delivered, 1);
}

#[test]
fn test_sync_subscriptions_rejects_oversized_event_types_payload() {
    let runtime = RealtimeDeliveryRuntime::default();
    let oversized_event_types = (0..300)
        .map(|index| format!("evt_{index:03}_{}", "x".repeat(64)))
        .collect::<Vec<_>>();

    let error = runtime
        .sync_subscriptions(
            "t_demo",
            "u_demo",
            "d_pad",
            vec![RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: oversized_event_types,
            }],
        )
        .expect_err("oversized eventTypes payload should be rejected");

    assert_eq!(error.code, "payload_too_large");
    assert!(
        error.message.contains("eventTypes"),
        "error should point to eventTypes payload guard, got: {}",
        error.message
    );
}

#[test]
fn test_sync_subscriptions_rejects_too_many_subscription_items() {
    let runtime = RealtimeDeliveryRuntime::default();
    let oversized_items = (0..300)
        .map(|index| RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: format!("c_{index:03}"),
            event_types: Vec::new(),
        })
        .collect::<Vec<_>>();

    let error = runtime
        .sync_subscriptions("t_demo", "u_demo", "d_pad", oversized_items)
        .expect_err("too many subscription items should be rejected");

    assert_eq!(error.code, "payload_too_large");
    assert!(
        error.message.contains("items"),
        "error should point to items payload guard, got: {}",
        error.message
    );
}

#[test]
fn test_sync_subscriptions_rejects_oversized_total_subscription_payload() {
    let runtime = RealtimeDeliveryRuntime::default();
    let oversized_items = (0..40)
        .map(|index| RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: format!("c_{index:03}_{}", "x".repeat(480)),
            event_types: (0..120)
                .map(|event_index| format!("evt_{event_index:02}_{}", "y".repeat(120)))
                .collect(),
        })
        .collect::<Vec<_>>();

    let error = runtime
        .sync_subscriptions("t_demo", "u_demo", "d_pad", oversized_items)
        .expect_err("oversized total subscription payload should be rejected");

    assert_eq!(error.code, "payload_too_large");
    assert!(
        error.message.contains("items"),
        "error should point to total items payload guard, got: {}",
        error.message
    );
}

#[test]
fn test_runtime_isolates_same_actor_id_across_principal_kinds() {
    let runtime = RealtimeDeliveryRuntime::default();
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "t_demo",
        "u_demo",
        "user",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_user".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));
    expect_ok(runtime.sync_subscriptions_for_principal_kind(
        "t_demo",
        "u_demo",
        "agent",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_agent".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let user_delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "t_demo",
        "u_demo",
        "user",
        "conversation",
        "c_user",
        "message.posted",
        r#"{"messageId":"msg_user"}"#.into(),
        vec!["d_pad".into()],
    ));
    let agent_delivered = expect_ok(runtime.publish_scope_event_for_principal_kind(
        "t_demo",
        "u_demo",
        "agent",
        "conversation",
        "c_agent",
        "message.posted",
        r#"{"messageId":"msg_agent"}"#.into(),
        vec!["d_pad".into()],
    ));

    assert_eq!(user_delivered, 1);
    assert_eq!(agent_delivered, 1);

    let user_window = expect_ok(
        runtime.list_events_for_principal_kind("t_demo", "u_demo", "user", "d_pad", 0, 10),
    );
    let agent_window = expect_ok(
        runtime.list_events_for_principal_kind("t_demo", "u_demo", "agent", "d_pad", 0, 10),
    );

    assert_eq!(user_window.items.len(), 1);
    assert_eq!(user_window.items[0].scope_id, "c_user");
    assert_eq!(user_window.items[0].payload, r#"{"messageId":"msg_user"}"#);
    assert_eq!(agent_window.items.len(), 1);
    assert_eq!(agent_window.items[0].scope_id, "c_agent");
    assert_eq!(
        agent_window.items[0].payload,
        r#"{"messageId":"msg_agent"}"#
    );
}

#[test]
fn test_runtime_clamps_invalid_checkpoint_invariants_on_restore() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    checkpoint_store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 3,
            acked_through_seq: 9,
            trimmed_through_seq: 11,
            updated_at: "2026-04-05T12:30:00Z".into(),
        })
        .expect("invalid checkpoint fixture should save");

    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store(checkpoint_store.clone());
    let restored = expect_ok(runtime.window_checkpoint("t_demo", "u_demo", "d_pad"));
    assert_eq!(restored.latest_realtime_seq, 3);
    assert_eq!(restored.acked_through_seq, 3);
    assert_eq!(restored.trimmed_through_seq, 3);

    let persisted = checkpoint_store
        .checkpoint("t_demo", "u_demo", "d_pad")
        .expect("checkpoint should exist after normalization");
    assert_eq!(persisted.latest_realtime_seq, 3);
    assert_eq!(persisted.acked_through_seq, 3);
    assert_eq!(persisted.trimmed_through_seq, 3);
}

#[test]
fn test_restore_device_state_clamps_invalid_checkpoint_fields_before_persist() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store(checkpoint_store.clone());

    expect_ok(
        runtime.restore_device_state(session_gateway::RealtimeDeviceStateSnapshot {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![],
            events: vec![],
            latest_realtime_seq: 4,
            acked_through_seq: 8,
            trimmed_through_seq: 12,
        }),
    );

    let restored = expect_ok(runtime.window_checkpoint("t_demo", "u_demo", "d_pad"));
    assert_eq!(restored.latest_realtime_seq, 4);
    assert_eq!(restored.acked_through_seq, 4);
    assert_eq!(restored.trimmed_through_seq, 4);

    let persisted = checkpoint_store
        .checkpoint("t_demo", "u_demo", "d_pad")
        .expect("checkpoint should exist after restore");
    assert_eq!(persisted.latest_realtime_seq, 4);
    assert_eq!(persisted.acked_through_seq, 4);
    assert_eq!(persisted.trimmed_through_seq, 4);
}

#[test]
fn test_restore_device_state_normalizes_event_order_for_monotonic_pagination() {
    let runtime = RealtimeDeliveryRuntime::default();

    expect_ok(
        runtime.restore_device_state(session_gateway::RealtimeDeviceStateSnapshot {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![],
            events: vec![
                realtime_event("t_demo", "u_demo", "d_pad", 3, "msg_3"),
                realtime_event("t_demo", "u_demo", "d_pad", 1, "msg_1"),
                realtime_event("t_demo", "u_demo", "d_pad", 2, "msg_2"),
            ],
            latest_realtime_seq: 3,
            acked_through_seq: 0,
            trimmed_through_seq: 0,
        }),
    );

    let first_page = expect_ok(runtime.list_events("t_demo", "u_demo", "d_pad", 0, 2));
    assert_eq!(
        first_page
            .items
            .iter()
            .map(|item| item.realtime_seq)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(first_page.next_after_seq, Some(2));
    assert!(first_page.has_more);

    let second_page = expect_ok(runtime.list_events("t_demo", "u_demo", "d_pad", 2, 2));
    assert_eq!(
        second_page
            .items
            .iter()
            .map(|item| item.realtime_seq)
            .collect::<Vec<_>>(),
        vec![3]
    );
    assert_eq!(second_page.next_after_seq, Some(3));
    assert!(!second_page.has_more);
}

#[test]
fn test_restore_device_state_deduplicates_realtime_sequences() {
    let runtime = RealtimeDeliveryRuntime::default();

    expect_ok(
        runtime.restore_device_state(session_gateway::RealtimeDeviceStateSnapshot {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![],
            events: vec![
                realtime_event("t_demo", "u_demo", "d_pad", 1, "msg_1_first"),
                realtime_event("t_demo", "u_demo", "d_pad", 1, "msg_1_duplicate"),
                realtime_event("t_demo", "u_demo", "d_pad", 2, "msg_2"),
            ],
            latest_realtime_seq: 2,
            acked_through_seq: 0,
            trimmed_through_seq: 0,
        }),
    );

    let window = expect_ok(runtime.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(
        window
            .items
            .iter()
            .map(|item| item.realtime_seq)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(window.items.len(), 2);
}

#[test]
fn test_restore_device_state_discards_events_at_or_below_trimmed_boundary() {
    let runtime = RealtimeDeliveryRuntime::default();

    expect_ok(
        runtime.restore_device_state(session_gateway::RealtimeDeviceStateSnapshot {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            subscriptions: vec![],
            events: vec![
                realtime_event("t_demo", "u_demo", "d_pad", 1, "msg_1"),
                realtime_event("t_demo", "u_demo", "d_pad", 2, "msg_2"),
                realtime_event("t_demo", "u_demo", "d_pad", 3, "msg_3"),
            ],
            latest_realtime_seq: 3,
            acked_through_seq: 2,
            trimmed_through_seq: 2,
        }),
    );

    let window = expect_ok(runtime.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(
        window
            .items
            .iter()
            .map(|item| item.realtime_seq)
            .collect::<Vec<_>>(),
        vec![3]
    );
    assert_eq!(window.acked_through_seq, 2);
    assert_eq!(window.trimmed_through_seq, 2);
}

#[test]
fn test_sync_subscriptions_advances_sync_timestamps_between_calls() {
    let runtime = RealtimeDeliveryRuntime::default();

    let first = expect_ok(runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_first".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    sleep(Duration::from_millis(5));

    let second = expect_ok(runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_second".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    assert_ne!(
        first.synced_at, second.synced_at,
        "separate subscription sync calls must not reuse a fixed synced_at timestamp"
    );
    assert_ne!(
        first.items[0].subscribed_at, second.items[0].subscribed_at,
        "separate subscription sync calls must not reuse a fixed subscribed_at timestamp"
    );
}

#[test]
fn test_clearing_device_subscriptions_stops_future_realtime_delivery() {
    let runtime = RealtimeDeliveryRuntime::default();
    expect_ok(runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    let first_delivery = expect_ok(runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(first_delivery, 1);

    expect_ok(runtime.clear_device_subscriptions("t_demo", "u_demo", "d_pad"));

    let second_delivery = expect_ok(runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_2"}"#.into(),
        vec!["d_pad".into()],
    ));
    assert_eq!(second_delivery, 0);

    let window = expect_ok(runtime.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(window.items.len(), 1);
    assert_eq!(window.items[0].realtime_seq, 1);
    assert_eq!(window.items[0].payload, r#"{"messageId":"msg_demo_1"}"#);
}

#[test]
fn test_publish_scope_event_advances_occurred_at_between_events() {
    let runtime = RealtimeDeliveryRuntime::default();
    expect_ok(runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    expect_ok(runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));

    sleep(Duration::from_millis(5));

    expect_ok(runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_2"}"#.into(),
        vec!["d_pad".into()],
    ));

    let window = expect_ok(runtime.list_events("t_demo", "u_demo", "d_pad", 0, 10));
    assert_eq!(window.items.len(), 2);
    assert_ne!(
        window.items[0].occurred_at, window.items[1].occurred_at,
        "separate realtime events must not reuse a fixed occurred_at timestamp"
    );
}

#[test]
fn test_checkpoint_updated_at_advances_after_new_persisted_mutation() {
    let checkpoint_store = Arc::new(MemoryRealtimeCheckpointStore::default());
    let runtime = RealtimeDeliveryRuntime::with_checkpoint_store(checkpoint_store.clone());
    expect_ok(runtime.sync_subscriptions(
        "t_demo",
        "u_demo",
        "d_pad",
        vec![RealtimeSubscriptionItemInput {
            scope_type: "conversation".into(),
            scope_id: "c_demo".into(),
            event_types: vec!["message.posted".into()],
        }],
    ));

    expect_ok(runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_1"}"#.into(),
        vec!["d_pad".into()],
    ));
    let first = checkpoint_store
        .checkpoint("t_demo", "u_demo", "d_pad")
        .expect("first checkpoint should be persisted");

    sleep(Duration::from_millis(5));

    expect_ok(runtime.publish_scope_event(
        "t_demo",
        "u_demo",
        "conversation",
        "c_demo",
        "message.posted",
        r#"{"messageId":"msg_demo_2"}"#.into(),
        vec!["d_pad".into()],
    ));
    let second = checkpoint_store
        .checkpoint("t_demo", "u_demo", "d_pad")
        .expect("second checkpoint should be persisted");

    assert_ne!(
        first.updated_at, second.updated_at,
        "separate persisted checkpoint writes must not reuse a fixed updated_at timestamp"
    );
}

fn realtime_event(
    tenant_id: &str,
    principal_id: &str,
    device_id: &str,
    realtime_seq: u64,
    message_id: &str,
) -> RealtimeEvent {
    RealtimeEvent {
        tenant_id: tenant_id.into(),
        principal_id: principal_id.into(),
        device_id: device_id.into(),
        realtime_seq,
        scope_type: "conversation".into(),
        scope_id: "c_demo".into(),
        event_type: "message.posted".into(),
        delivery_class: "ephemeral".into(),
        payload: format!(r#"{{"messageId":"{message_id}"}}"#),
        occurred_at: "2026-04-05T12:30:00Z".into(),
    }
}
