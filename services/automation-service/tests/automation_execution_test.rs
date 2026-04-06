use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use im_auth_context::AuthContext;
use im_domain_events::{AggregateType, CommitEnvelope};
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};

#[derive(Clone, Default)]
struct RecordingJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl RecordingJournal {
    fn recorded(&self) -> Vec<CommitEnvelope> {
        self.events.lock().expect("journal should lock").clone()
    }
}

impl CommitJournal for RecordingJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[test]
fn test_request_execution_appends_requested_and_completed_events() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = automation_service::AutomationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::from(["automation.execute".to_string()]),
    };

    let execution = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_demo".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    assert_eq!(execution.execution_id, "ae_demo");
    assert_eq!(execution.state.as_str(), "succeeded");
    assert_eq!(execution.retry_count, 0);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].event_type, "automation.execution_requested");
    assert_eq!(events[1].event_type, "automation.execution_completed");
    assert_eq!(events[0].aggregate_type, AggregateType::AutomationExecution);
    assert_eq!(events[0].actor.actor_id, "u_demo");

    let payload: serde_json::Value =
        serde_json::from_str(&events[1].payload).expect("payload should be valid json");
    assert_eq!(payload["executionId"], "ae_demo");
    assert_eq!(payload["targetKind"], "workflow");
    assert_eq!(payload["state"], "succeeded");
}

#[test]
fn test_get_execution_is_scoped_to_requesting_principal() {
    let runtime = automation_service::AutomationRuntime::default();
    let owner_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_owner".into(),
        actor_kind: "user".into(),
        session_id: Some("s_owner".into()),
        device_id: None,
        permissions: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };
    let other_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_other".into(),
        actor_kind: "user".into(),
        session_id: Some("s_other".into()),
        device_id: None,
        permissions: BTreeSet::from(["automation.read".to_string()]),
    };

    runtime
        .request_execution(
            &owner_auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_principal_scoped".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    let error = runtime
        .get_execution(&other_auth, "ae_principal_scoped")
        .expect_err("foreign principal should not read execution");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
}

#[test]
fn test_duplicate_request_execution_is_idempotent_when_payload_matches() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = automation_service::AutomationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    let first = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_idempotent".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first execution request should succeed");
    let second = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_idempotent".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("duplicate execution request should be idempotent");

    assert_eq!(second, first);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
}

#[test]
fn test_duplicate_request_execution_rejects_conflicting_payload() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = automation_service::AutomationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::from(["automation.execute".to_string()]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_conflict".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_demo".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("first execution request should succeed");

    let error = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_conflict".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_other".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect_err("conflicting duplicate should be rejected");
    let response = axum::response::IntoResponse::into_response(error);
    assert_eq!(response.status(), axum::http::StatusCode::CONFLICT);

    let events = journal.recorded();
    assert_eq!(events.len(), 2);
}

#[test]
fn test_execution_timestamps_advance_between_distinct_requests() {
    let runtime = automation_service::AutomationRuntime::default();
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_demo".into()),
        device_id: None,
        permissions: BTreeSet::from(["automation.execute".to_string()]),
    };

    let first = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_time_first".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_first".into(),
                input_payload: Some(r#"{"step":"first"}"#.into()),
            },
        )
        .expect("first execution should succeed");

    sleep(Duration::from_millis(5));

    let second = runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_time_second".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_second".into(),
                input_payload: Some(r#"{"step":"second"}"#.into()),
            },
        )
        .expect("second execution should succeed");

    assert_ne!(
        first.requested_at, second.requested_at,
        "distinct execution requests must not reuse a fixed requested_at timestamp"
    );
    assert_ne!(
        first.completed_at, second.completed_at,
        "distinct execution requests must not reuse a fixed completed_at timestamp"
    );
}
