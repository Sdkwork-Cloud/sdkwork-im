//! White-box unit tests for session-gateway presence runtime.
//!
//! Extracted from the implementation file so it stays focused on logic
//! while the tests (which exercise private items) live beside it. Mounted
//! back via `#[cfg(test)] #[path = "presence_tests.rs"] mod tests;` so
//! `use super::{...}` still resolves to the parent module unchanged.

use super::*;
use im_domain_core::presence::{PresenceClientView, PresenceStatus};

fn presence_record(device_id: &str, last_seen_at: &str) -> PresenceStateRecord {
    PresenceStateRecord {
        tenant_id: "t_demo".into(),
        principal_kind: "user".into(),
        principal_id: "u_demo".into(),
        device_id: device_id.into(),
        presence: PresenceClientView {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: device_id.into(),
            platform: None,
            session_id: Some(format!("s_{device_id}")),
            status: PresenceStatus::Online,
            last_sync_seq: 0,
            last_resume_at: Some(last_seen_at.into()),
            last_seen_at: Some(last_seen_at.into()),
        },
        resume_required: false,
        updated_at: last_seen_at.into(),
    }
}

#[test]
fn test_presence_state_store_load_recovers_from_poisoned_lock() {
    let store = RuntimeMemoryPresenceStateStore::default();
    let _ = std::panic::catch_unwind({
        let state = store.state.clone();
        move || {
            let _guard = state.lock().expect("presence state store should lock");
            panic!("poison presence state store lock");
        }
    });

    let restored = store
        .load_state("t_demo", "user", "u_demo", "d_poison")
        .expect("poisoned lock should be recovered");
    assert!(restored.is_none());
}

#[test]
fn test_presence_state_store_seen_at_cutoff_compares_rfc3339_by_instant() {
    let store = RuntimeMemoryPresenceStateStore::default();
    store
        .save_state(presence_record(
            "d_later_fraction",
            "2026-05-06T00:00:00.100Z",
        ))
        .expect("later presence save should succeed");
    store
        .save_state(presence_record("d_whole_second", "2026-05-06T00:00:00Z"))
        .expect("whole-second presence save should succeed");

    let stale = store
        .list_online_states_seen_at_or_before("2026-05-06T00:00:00Z", 10)
        .expect("stale online list should succeed");

    assert_eq!(
        stale
            .iter()
            .map(|record| record.device_id.as_str())
            .collect::<Vec<_>>(),
        vec!["d_whole_second"]
    );
}
