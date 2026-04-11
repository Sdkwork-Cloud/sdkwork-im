use std::collections::BTreeSet;
use std::thread::sleep;
use std::time::Duration;

use im_auth_context::AuthContext;

#[test]
fn test_record_anchor_and_export_bundle() {
    let runtime = audit_service::AuditRuntime::default();
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    let record = runtime.record_anchor(
        &auth,
        audit_service::RecordAuditAnchor {
            record_id: "audit_demo".into(),
            aggregate_type: "automation_execution".into(),
            aggregate_id: "ae_demo".into(),
            action: "automation.execution_requested".into(),
            payload: Some(r#"{"targetRef":"wf_demo"}"#.into()),
        },
    );

    assert_eq!(record.record_id, "audit_demo");
    assert_eq!(record.actor_id, "u_demo");
    assert_eq!(record.actor_session_id.as_deref(), Some("s_demo"));

    let export = runtime.export_bundle(&auth);
    assert_eq!(export.total, 1);
    assert_eq!(export.items[0].aggregate_id, "ae_demo");
    assert_eq!(export.items[0].action, "automation.execution_requested");
}

#[test]
fn test_recorded_at_advances_between_distinct_records() {
    let runtime = audit_service::AuditRuntime::default();
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    let first = runtime.record_anchor(
        &auth,
        audit_service::RecordAuditAnchor {
            record_id: "audit_time_first".into(),
            aggregate_type: "notification".into(),
            aggregate_id: "ntf_time_first".into(),
            action: "notification.requested".into(),
            payload: None,
        },
    );

    sleep(Duration::from_millis(5));

    let second = runtime.record_anchor(
        &auth,
        audit_service::RecordAuditAnchor {
            record_id: "audit_time_second".into(),
            aggregate_type: "notification".into(),
            aggregate_id: "ntf_time_second".into(),
            action: "notification.dispatched".into(),
            payload: None,
        },
    );

    assert_ne!(
        first.recorded_at, second.recorded_at,
        "distinct audit records must not reuse a fixed recorded_at timestamp"
    );
}

#[test]
fn test_export_bundle_includes_verifiable_chain_and_detects_tampering() {
    let runtime = audit_service::AuditRuntime::default();
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::new(),
    };

    runtime.record_anchor(
        &auth,
        audit_service::RecordAuditAnchor {
            record_id: "audit_chain_first".into(),
            aggregate_type: "notification".into(),
            aggregate_id: "ntf_chain_first".into(),
            action: "notification.requested".into(),
            payload: Some(r#"{"step":"first"}"#.into()),
        },
    );
    runtime.record_anchor(
        &auth,
        audit_service::RecordAuditAnchor {
            record_id: "audit_chain_second".into(),
            aggregate_type: "notification".into(),
            aggregate_id: "ntf_chain_second".into(),
            action: "notification.dispatched".into(),
            payload: Some(r#"{"step":"second"}"#.into()),
        },
    );

    let export = runtime.export_bundle(&auth);
    assert_eq!(export.total, 2);
    assert!(
        export.chain_valid,
        "freshly exported bundle should report chain_valid=true"
    );
    assert!(
        export.chain_head_hash.is_some(),
        "freshly exported bundle should expose non-empty chain head hash"
    );
    assert!(
        audit_service::verify_audit_export_bundle_integrity(&export),
        "freshly exported bundle should pass integrity verification"
    );

    let mut tampered = export.clone();
    tampered.items[1].action = "notification.tampered".into();
    tampered.chain_valid = true;
    assert!(
        !audit_service::verify_audit_export_bundle_integrity(&tampered),
        "tampered bundle should fail integrity verification"
    );
}
