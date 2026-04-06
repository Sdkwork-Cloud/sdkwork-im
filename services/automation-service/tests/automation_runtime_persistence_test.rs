use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use im_adapters_local_memory::MemoryAutomationExecutionStore;
use im_auth_context::AuthContext;
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};

#[derive(Clone, Default)]
struct RecordingJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl CommitJournal for RecordingJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[test]
fn test_runtime_restores_automation_projection_on_rebuild_with_shared_store() {
    let journal = Arc::new(RecordingJournal::default());
    let execution_store = Arc::new(MemoryAutomationExecutionStore::default());
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

    let runtime_before = automation_service::AutomationRuntime::with_journal_and_store(
        journal.clone(),
        execution_store.clone(),
    );

    runtime_before
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_rebuild".into(),
                trigger_type: "webhook.manual".into(),
                target_kind: "workflow".into(),
                target_ref: "wf_rebuild".into(),
                input_payload: Some(r#"{"conversationId":"c_demo"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    let runtime_after =
        automation_service::AutomationRuntime::with_journal_and_store(journal, execution_store);

    let execution = runtime_after
        .get_execution(&auth, "ae_rebuild")
        .expect("execution should restore after rebuild");
    assert_eq!(execution.execution_id, "ae_rebuild");
    assert_eq!(execution.state.as_str(), "succeeded");
}
