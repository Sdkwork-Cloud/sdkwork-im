use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_request_and_get_execution_over_http() {
    let app = automation_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_demo",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("request execution should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create body should be valid json");
    assert_eq!(create_json["executionId"], "ae_http_demo");
    assert_eq!(create_json["state"], "succeeded");

    let get_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/automation/executions/ae_http_demo")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get execution should succeed");
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body = get_response
        .into_body()
        .collect()
        .await
        .expect("get body should collect")
        .to_bytes();
    let get_json: serde_json::Value =
        serde_json::from_slice(&get_body).expect("get body should be valid json");
    assert_eq!(get_json["targetRef"], "wf_http_demo");
    assert_eq!(get_json["triggerType"], "webhook.manual");
}

#[tokio::test]
async fn test_duplicate_execution_id_is_idempotent_and_conflicting_retry_is_rejected_over_http() {
    let app = automation_service::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first execution request should succeed");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first body should be valid json");

    let idempotent_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent retry should return response");
    assert_eq!(idempotent_response.status(), StatusCode::OK);
    let idempotent_body = idempotent_response
        .into_body()
        .collect()
        .await
        .expect("idempotent body should collect")
        .to_bytes();
    let idempotent_json: serde_json::Value =
        serde_json::from_slice(&idempotent_body).expect("idempotent body should be valid json");
    assert_eq!(idempotent_json, first_json);

    let conflicting_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_other",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting retry should return response");
    assert_eq!(conflicting_response.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_response
        .into_body()
        .collect()
        .await
        .expect("conflicting body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value =
        serde_json::from_slice(&conflicting_body).expect("conflicting body should be valid json");
    assert_eq!(conflicting_json["code"], "automation_execution_conflict");
}

#[tokio::test]
async fn test_agent_response_and_tool_call_lifecycle_over_http() {
    let app = automation_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_agent",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_agent",
                        "streamId":"st_http_agent",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);
    let start_body = start_response
        .into_body()
        .collect()
        .await
        .expect("start body should collect")
        .to_bytes();
    let start_json: serde_json::Value =
        serde_json::from_slice(&start_body).expect("start body should be valid json");
    assert_eq!(start_json["streamId"], "st_http_agent");
    assert_eq!(start_json["state"], "opened");

    let delta_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses/st_http_agent/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "frameType":"delta.text",
                        "schemaRef":"schema://agent/response.delta#chunk",
                        "encoding":"json",
                        "payload":"{\"delta\":\"hello\"}",
                        "attributes":{"chunk":"1"}
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response delta should return response");
    assert_eq!(delta_response.status(), StatusCode::OK);
    let delta_body = delta_response
        .into_body()
        .collect()
        .await
        .expect("delta body should collect")
        .to_bytes();
    let delta_json: serde_json::Value =
        serde_json::from_slice(&delta_body).expect("delta body should be valid json");
    assert_eq!(delta_json["sender"]["kind"], "agent");
    assert_eq!(delta_json["sender"]["id"], "ag_demo");

    let tool_request_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-tool-calls")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_agent",
                        "toolCallId":"tc_http_lookup",
                        "toolName":"knowledge.search",
                        "argumentsPayload":"{\"query\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tool request should return response");
    assert_eq!(tool_request_response.status(), StatusCode::OK);
    let tool_request_body = tool_request_response
        .into_body()
        .collect()
        .await
        .expect("tool request body should collect")
        .to_bytes();
    let tool_request_json: serde_json::Value = serde_json::from_slice(&tool_request_body)
        .expect("tool request body should be valid json");
    assert_eq!(tool_request_json["state"], "requested");

    let tool_complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions/ae_http_agent/agent-tool-calls/tc_http_lookup/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "resultPayload":"{\"hits\":[{\"id\":\"doc_1\"}]}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tool completion should return response");
    assert_eq!(tool_complete_response.status(), StatusCode::OK);
    let tool_complete_body = tool_complete_response
        .into_body()
        .collect()
        .await
        .expect("tool complete body should collect")
        .to_bytes();
    let tool_complete_json: serde_json::Value = serde_json::from_slice(&tool_complete_body)
        .expect("tool complete body should be valid json");
    assert_eq!(tool_complete_json["state"], "completed");

    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses/st_http_agent/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "resultMessageId":"m_http_agent"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response complete should return response");
    assert_eq!(complete_response.status(), StatusCode::OK);
    let complete_body = complete_response
        .into_body()
        .collect()
        .await
        .expect("complete body should collect")
        .to_bytes();
    let complete_json: serde_json::Value =
        serde_json::from_slice(&complete_body).expect("complete body should be valid json");
    assert_eq!(complete_json["state"], "completed");
    assert_eq!(complete_json["resultMessageId"], "m_http_agent");
}

#[tokio::test]
async fn test_automation_governance_surface_and_operator_override_over_http() {
    let app = automation_service::build_default_app();

    let governance_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/automation/governance")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("governance request should return response");
    assert_eq!(governance_response.status(), StatusCode::OK);
    let governance_body = governance_response
        .into_body()
        .collect()
        .await
        .expect("governance body should collect")
        .to_bytes();
    let governance_json: serde_json::Value =
        serde_json::from_slice(&governance_body).expect("governance body should be valid json");
    assert_eq!(governance_json["capabilityProfileId"], "stable-agent");
    assert_eq!(
        governance_json["operatorOverridePermission"],
        "automation.operator_override"
    );
    assert_eq!(governance_json["operatorOverrideActive"], false);
    assert_eq!(governance_json["restrictedToolPrefixes"][0], "ops.");

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_guardrail",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_demo",
                        "inputPayload":"{\"prompt\":\"shutdown\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("execution request should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_guardrail",
                        "streamId":"st_http_guardrail",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let denied_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-tool-calls")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_guardrail",
                        "toolCallId":"tc_http_guardrail_denied",
                        "toolName":"ops.shutdown",
                        "argumentsPayload":"{\"scope\":\"tenant\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("restricted tool request should return response");
    assert_eq!(denied_response.status(), StatusCode::FORBIDDEN);
    let denied_body = denied_response
        .into_body()
        .collect()
        .await
        .expect("denied body should collect")
        .to_bytes();
    let denied_json: serde_json::Value =
        serde_json::from_slice(&denied_body).expect("denied body should be valid json");
    assert_eq!(denied_json["code"], "automation_guardrail_denied");

    let override_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-tool-calls")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header(
                    "x-permissions",
                    "automation.execute automation.read automation.operator_override",
                )
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_guardrail",
                        "toolCallId":"tc_http_guardrail_allowed",
                        "toolName":"ops.shutdown",
                        "argumentsPayload":"{\"scope\":\"tenant\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("override tool request should return response");
    assert_eq!(override_response.status(), StatusCode::OK);
    let override_body = override_response
        .into_body()
        .collect()
        .await
        .expect("override body should collect")
        .to_bytes();
    let override_json: serde_json::Value =
        serde_json::from_slice(&override_body).expect("override body should be valid json");
    assert_eq!(override_json["state"], "requested");

    let override_governance_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/automation/governance")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header(
                    "x-permissions",
                    "automation.read automation.operator_override",
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("override governance request should return response");
    assert_eq!(override_governance_response.status(), StatusCode::OK);
    let override_governance_body = override_governance_response
        .into_body()
        .collect()
        .await
        .expect("override governance body should collect")
        .to_bytes();
    let override_governance_json: serde_json::Value = serde_json::from_slice(
        &override_governance_body,
    )
    .expect("override governance body should be valid json");
    assert_eq!(override_governance_json["operatorOverrideActive"], true);
}
