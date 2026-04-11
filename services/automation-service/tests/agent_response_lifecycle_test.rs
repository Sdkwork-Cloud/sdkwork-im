use axum::response::IntoResponse;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use craw_chat_contract_agent::AgentSubject;
use http_body_util::BodyExt;
use im_auth_context::AuthContext;
use im_domain_events::CommitEnvelope;
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
fn test_agent_response_stream_and_tool_call_lifecycle_are_verifiable() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = automation_service::AutomationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permissions: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_agent".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"hello"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    let started = runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_agent".into(),
                stream_id: "st_agent_demo".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "ag_demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect("agent response stream should start");
    assert_eq!(started.stream_id, "st_agent_demo");
    assert_eq!(started.state.as_wire_value(), "opened");

    let delta = runtime
        .append_agent_response_delta(
            &auth,
            "st_agent_demo",
            automation_service::AppendAgentResponseDeltaRequest {
                frame_seq: 1,
                frame_type: "delta.text".into(),
                schema_ref: Some("schema://agent/response.delta#chunk".into()),
                encoding: "json".into(),
                payload: r#"{"delta":"hello"}"#.into(),
                attributes: BTreeMap::from([("chunk".into(), "1".into())]),
            },
        )
        .expect("agent response delta should append");
    assert_eq!(delta.sender.id, "ag_demo");
    assert_eq!(delta.sender.kind, "agent");
    assert_eq!(delta.sender.member_id.as_deref(), Some("cm_agent"));
    assert_eq!(delta.sender.session_id.as_deref(), Some("s_agent"));

    let tool_requested = runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_agent".into(),
                tool_call_id: "tc_lookup".into(),
                tool_name: "knowledge.search".into(),
                arguments_payload: r#"{"query":"hello"}"#.into(),
            },
        )
        .expect("tool call request should succeed");
    assert_eq!(tool_requested.state.as_str(), "requested");

    let tool_completed = runtime
        .complete_agent_tool_call(
            &auth,
            "ae_agent",
            "tc_lookup",
            automation_service::CompleteAgentToolCallRequest {
                result_payload: r#"{"hits":[{"id":"doc_1"}]}"#.into(),
            },
        )
        .expect("tool call completion should succeed");
    assert_eq!(tool_completed.state.as_str(), "completed");

    let completed = runtime
        .complete_agent_response(
            &auth,
            "st_agent_demo",
            automation_service::CompleteAgentResponseRequest {
                frame_seq: 1,
                result_message_id: Some("m_agent_final".into()),
            },
        )
        .expect("agent response stream should complete");
    assert_eq!(completed.state.as_wire_value(), "completed");
    assert_eq!(
        completed.result_message_id.as_deref(),
        Some("m_agent_final")
    );

    let events = journal.recorded();
    let event_types = events
        .iter()
        .map(|event| event.event_type.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        event_types,
        vec![
            "automation.execution_requested",
            "automation.execution_completed",
            "automation.agent_response_started",
            "automation.agent_response_delta",
            "automation.agent_tool_call_requested",
            "automation.agent_tool_call_completed",
            "automation.agent_response_completed",
        ]
    );

    let delta_payload: serde_json::Value =
        serde_json::from_str(&events[3].payload).expect("delta payload should be valid json");
    assert_eq!(delta_payload["sender"]["kind"], "agent");
    assert_eq!(delta_payload["sender"]["id"], "ag_demo");
    assert_eq!(delta_payload["frameType"], "delta.text");

    let tool_payload: serde_json::Value =
        serde_json::from_str(&events[5].payload).expect("tool payload should be valid json");
    assert_eq!(tool_payload["toolCallId"], "tc_lookup");
    assert_eq!(tool_payload["state"], "completed");
}

#[tokio::test]
async fn test_restricted_tool_call_requires_operator_override_and_is_auditable() {
    let journal = Arc::new(RecordingJournal::default());
    let runtime = automation_service::AutomationRuntime::with_journal(journal.clone());
    let auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_demo".into(),
        actor_kind: "user".into(),
        session_id: Some("s_user".into()),
        device_id: None,
        permissions: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
        ]),
    };

    runtime
        .request_execution(
            &auth,
            automation_service::RequestAutomationExecution {
                execution_id: "ae_guardrail".into(),
                trigger_type: "agent.manual".into(),
                target_kind: "conversation".into(),
                target_ref: "c_demo".into(),
                input_payload: Some(r#"{"prompt":"shutdown"}"#.into()),
            },
        )
        .expect("execution request should succeed");

    runtime
        .start_agent_response(
            &auth,
            automation_service::StartAgentResponseRequest {
                execution_id: "ae_guardrail".into(),
                stream_id: "st_guardrail".into(),
                stream_type: "agent.response.delta".into(),
                conversation_id: "c_demo".into(),
                schema_ref: Some("schema://agent/response.delta".into()),
                member_id: Some("cm_agent".into()),
                agent: AgentSubject {
                    agent_id: "ag_demo".into(),
                    session_id: Some("s_agent".into()),
                    metadata: BTreeMap::from([
                        ("agentMode".into(), "assistant".into()),
                        ("capabilityProfileId".into(), "stable-agent".into()),
                    ]),
                },
            },
        )
        .expect("agent response stream should start");

    let denied = runtime
        .request_agent_tool_call(
            &auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_guardrail".into(),
                tool_call_id: "tc_guardrail_denied".into(),
                tool_name: "ops.shutdown".into(),
                arguments_payload: r#"{"scope":"tenant"}"#.into(),
            },
        )
        .expect_err("restricted tool call should be rejected before operator override");
    let denied_response = denied.into_response();
    assert_eq!(denied_response.status(), axum::http::StatusCode::FORBIDDEN);
    let denied_body = denied_response
        .into_body()
        .collect()
        .await
        .expect("guardrail denied body should collect")
        .to_bytes();
    let denied_json: serde_json::Value =
        serde_json::from_slice(&denied_body).expect("guardrail denied body should be valid json");
    assert_eq!(denied_json["code"], "automation_guardrail_denied");

    let override_auth = AuthContext {
        permissions: BTreeSet::from([
            "automation.execute".to_string(),
            "automation.read".to_string(),
            "automation.operator_override".to_string(),
        ]),
        ..auth.clone()
    };

    let tool_requested = runtime
        .request_agent_tool_call(
            &override_auth,
            automation_service::RequestAgentToolCallRequest {
                execution_id: "ae_guardrail".into(),
                tool_call_id: "tc_guardrail_allowed".into(),
                tool_name: "ops.shutdown".into(),
                arguments_payload: r#"{"scope":"tenant"}"#.into(),
            },
        )
        .expect("operator override should allow restricted tool call");
    assert_eq!(tool_requested.state.as_str(), "requested");

    let events = journal.recorded();
    let event_types = events
        .iter()
        .map(|event| event.event_type.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        event_types,
        vec![
            "automation.execution_requested",
            "automation.execution_completed",
            "automation.agent_response_started",
            "automation.guardrail_denied",
            "automation.operator_override_applied",
            "automation.agent_tool_call_requested",
        ]
    );

    let denied_payload: serde_json::Value = serde_json::from_str(&events[3].payload)
        .expect("guardrail denied payload should be valid json");
    assert_eq!(denied_payload["toolName"], "ops.shutdown");
    assert_eq!(
        denied_payload["operatorOverridePermission"],
        "automation.operator_override"
    );

    let override_payload: serde_json::Value = serde_json::from_str(&events[4].payload)
        .expect("override payload should be valid json");
    assert_eq!(override_payload["toolName"], "ops.shutdown");
    assert_eq!(override_payload["operatorOverrideActive"], true);
}
