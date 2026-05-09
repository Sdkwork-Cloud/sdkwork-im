use super::*;

pub(super) async fn request_notification(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestNotification>,
) -> Result<Json<NotificationRequestResponse>, axum::response::Response> {
    let is_bearer_request = headers.contains_key(axum::http::header::AUTHORIZATION);
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let result = state
        .notification_runtime
        .request_notification_from_public_api(&auth, request, is_bearer_request)
        .map_err(IntoResponse::into_response)?;
    let is_new = result.is_new;
    let task = result.task.clone();

    if is_new {
        let _ = state.audit_runtime.record_anchor(
            &auth,
            RecordAuditAnchor {
                record_id: stable_local_audit_record_id("audit_", task.notification_id.as_str()),
                aggregate_type: "notification".into(),
                aggregate_id: stable_local_audit_aggregate_id(
                    "notification",
                    task.notification_id.as_str(),
                ),
                action: "notification.requested".into(),
                payload: Some(
                    serde_json::json!({
                        "notificationId": task.notification_id,
                        "sourceEventType": task.source_event_type,
                        "recipientId": task.recipient_id,
                    })
                    .to_string(),
                ),
            },
        );
    }

    Ok(Json(result.into()))
}

pub(super) async fn list_notifications(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let items = state
        .notification_runtime
        .list_notifications(&auth)
        .map_err(IntoResponse::into_response)?;
    Ok(Json(serde_json::json!({
        "items": items
    })))
}

pub(super) async fn get_notification(
    Path(notification_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<NotificationTask>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let task = state
        .notification_runtime
        .get_notification(&auth, notification_id.as_str())
        .map_err(IntoResponse::into_response)?;
    Ok(Json(task))
}

pub(super) async fn request_automation_execution(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestAutomationExecution>,
) -> Result<Json<automation_service::AutomationExecutionRequestResponse>, axum::response::Response>
{
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let result = state
        .automation_runtime
        .request_execution_with_outcome(&auth, request)
        .map_err(IntoResponse::into_response)?;
    let is_new = result.is_new;
    let execution = result.execution.clone();

    if is_new {
        let _ = state.audit_runtime.record_anchor(
            &auth,
            RecordAuditAnchor {
                record_id: automation_audit_record_id(
                    auth.actor_kind.as_str(),
                    "automation.execution_requested",
                    execution.execution_id.as_str(),
                ),
                aggregate_type: "automation_execution".into(),
                aggregate_id: execution.execution_id.clone(),
                action: "automation.execution_requested".into(),
                payload: execution.input_payload.clone(),
            },
        );

        let _ = state
            .notification_runtime
            .request_automation_result_notification(
                &auth,
                notification_service::RequestAutomationResultNotification {
                    execution_id: execution.execution_id.clone(),
                    target_ref: execution.target_ref.clone(),
                    output_payload: execution.output_payload.clone(),
                },
            );
    }

    Ok(Json(result.into()))
}

pub(super) async fn get_automation_execution(
    Path(execution_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AutomationExecution>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let execution = state
        .automation_runtime
        .get_execution(&auth, execution_id.as_str())
        .map_err(IntoResponse::into_response)?;
    Ok(Json(execution))
}

pub(super) async fn get_automation_governance(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<automation_service::AutomationGovernanceSnapshot>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let snapshot = state
        .automation_runtime
        .governance_snapshot(&auth)
        .map_err(IntoResponse::into_response)?;
    Ok(Json(snapshot))
}

pub(super) async fn start_agent_response(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<automation_service::StartAgentResponseRequest>,
) -> Result<Json<im_domain_core::stream::StreamSession>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let session = state
        .automation_runtime
        .start_agent_response(&auth, request)
        .map_err(IntoResponse::into_response)?;
    record_automation_audit_anchor(
        &state,
        &auth,
        session.stream_id.as_str(),
        "automation.agent_response_started",
        &session,
    );
    Ok(Json(session))
}

pub(super) async fn append_agent_response_delta(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<automation_service::AppendAgentResponseDeltaRequest>,
) -> Result<Json<im_domain_core::stream::StreamFrame>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let frame = state
        .automation_runtime
        .append_agent_response_delta(&auth, stream_id.as_str(), request)
        .map_err(IntoResponse::into_response)?;
    record_automation_audit_anchor(
        &state,
        &auth,
        frame.stream_id.as_str(),
        "automation.agent_response_delta",
        &frame,
    );
    Ok(Json(frame))
}

pub(super) async fn complete_agent_response(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<automation_service::CompleteAgentResponseRequest>,
) -> Result<Json<im_domain_core::stream::StreamSession>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let session = state
        .automation_runtime
        .complete_agent_response(&auth, stream_id.as_str(), request)
        .map_err(IntoResponse::into_response)?;
    record_automation_audit_anchor(
        &state,
        &auth,
        session.stream_id.as_str(),
        "automation.agent_response_completed",
        &session,
    );
    Ok(Json(session))
}

pub(super) async fn request_agent_tool_call(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<automation_service::RequestAgentToolCallRequest>,
) -> Result<Json<automation_service::AgentToolCall>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let tool_call_requires_override =
        automation_service::automation_tool_requires_operator_override(request.tool_name.as_str());
    let operator_override_active =
        auth.has_permission(automation_service::automation_operator_override_permission());
    let tool_call = match state
        .automation_runtime
        .request_agent_tool_call(&auth, request.clone())
    {
        Ok(tool_call) => tool_call,
        Err(error) => {
            if error.code() == "automation_guardrail_denied" {
                record_automation_audit_anchor(
                    &state,
                    &auth,
                    request.execution_id.as_str(),
                    "automation.guardrail_denied",
                    &serde_json::json!({
                        "executionId": request.execution_id,
                        "toolCallId": request.tool_call_id,
                        "toolName": request.tool_name,
                        "operatorOverridePermission": automation_service::automation_operator_override_permission(),
                    }),
                );
            }
            return Err(error.into_response());
        }
    };
    if tool_call_requires_override && operator_override_active {
        record_automation_audit_anchor(
            &state,
            &auth,
            tool_call.execution_id.as_str(),
            "automation.operator_override_applied",
            &serde_json::json!({
                "executionId": tool_call.execution_id,
                "toolCallId": tool_call.tool_call_id,
                "toolName": tool_call.tool_name,
                "operatorOverridePermission": automation_service::automation_operator_override_permission(),
                "operatorOverrideActive": true,
            }),
        );
    }
    record_automation_audit_anchor(
        &state,
        &auth,
        tool_call.tool_call_id.as_str(),
        "automation.agent_tool_call_requested",
        &tool_call,
    );
    Ok(Json(tool_call))
}

pub(super) async fn complete_agent_tool_call(
    Path((execution_id, tool_call_id)): Path<(String, String)>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<automation_service::CompleteAgentToolCallRequest>,
) -> Result<Json<automation_service::AgentToolCall>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    let tool_call = state
        .automation_runtime
        .complete_agent_tool_call(&auth, execution_id.as_str(), tool_call_id.as_str(), request)
        .map_err(IntoResponse::into_response)?;
    record_automation_audit_anchor(
        &state,
        &auth,
        tool_call.tool_call_id.as_str(),
        "automation.agent_tool_call_completed",
        &tool_call,
    );
    Ok(Json(tool_call))
}

pub(super) async fn record_audit_anchor(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RecordAuditAnchor>,
) -> Result<Json<AuditRecordMutationResponse>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_audit_write_access(&auth).map_err(IntoResponse::into_response)?;
    audit_service::validate_record_audit_anchor_request(&request)
        .map_err(IntoResponse::into_response)?;
    let request_key = audit_record_request_key(&auth, request.record_id.as_str());
    let outcome = state
        .audit_runtime
        .record_anchor_with_outcome(&auth, request)
        .map_err(IntoResponse::into_response)?;
    Ok(Json(AuditRecordMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

fn record_automation_audit_anchor<T: serde::Serialize>(
    state: &AppState,
    auth: &im_auth_context::AuthContext,
    aggregate_id: &str,
    action: &str,
    payload: &T,
) {
    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: automation_audit_record_id(auth.actor_kind.as_str(), action, aggregate_id),
            aggregate_type: "automation_execution".into(),
            aggregate_id: automation_audit_aggregate_id(action, aggregate_id),
            action: action.into(),
            payload: serde_json::to_string(payload).ok(),
        },
    );
}

fn automation_audit_aggregate_id(action: &str, aggregate_id: &str) -> String {
    let namespace = match action {
        "automation.agent_response_started"
        | "automation.agent_response_delta"
        | "automation.agent_response_completed" => "automation-stream",
        "automation.agent_tool_call_requested" | "automation.agent_tool_call_completed" => {
            "automation-tool-call"
        }
        _ => "automation-execution",
    };
    stable_local_audit_aggregate_id(namespace, aggregate_id)
}

fn automation_audit_record_id(actor_kind: &str, action: &str, aggregate_id: &str) -> String {
    stable_local_audit_record_id(
        format!("audit_{action}_{actor_kind}_").as_str(),
        aggregate_id,
    )
}

pub(super) async fn list_audit_records(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_audit_read_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(serde_json::json!({
        "items": state.audit_runtime.list_records(&auth)
    })))
}

pub(super) async fn export_audit_bundle(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AuditExportBundle>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_audit_read_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(state.audit_runtime.export_bundle(&auth)))
}

pub(super) async fn get_ops_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<OpsHealthResponse>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.health_view()))
}

pub(super) async fn get_ops_cluster(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ClusterView>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.cluster_view()))
}

pub(super) async fn get_ops_lag(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<LagView>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.lag_view()))
}

pub(super) async fn get_ops_replay_status(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ops_service::ProjectionReplayStatusView>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.replay_status_view()))
}

pub(super) async fn get_ops_runtime_dir(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RuntimeDirInspectionView>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.runtime_dir_view()))
}

pub(super) async fn get_ops_provider_bindings(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingsView>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.provider_bindings_view()))
}

pub(super) async fn get_ops_provider_binding_drift(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingDriftView>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.provider_binding_drift_view()))
}

pub(super) async fn get_ops_diagnostics(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<DiagnosticBundle>, axum::response::Response> {
    let auth = resolve_auth_context(&headers)
        .map_err(ApiError::from)
        .map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.diagnostic_bundle()))
}

pub(super) fn refresh_node_operational_view(state: &AppState) {
    let lifecycle = state
        .realtime_cluster
        .node_lifecycle(state.node_id.as_str())
        .unwrap_or_else(|| session_gateway::RealtimeNodeLifecycleView {
            node_id: state.node_id.clone(),
            drain_status: "active".into(),
            rebalance_state: "stable".into(),
            owned_route_count: 0,
        });
    state.ops_runtime.set_node_lifecycle(
        lifecycle.drain_status.as_str(),
        lifecycle.rebalance_state.as_str(),
    );
    let routes = state
        .realtime_cluster
        .routes_for_node(state.node_id.as_str())
        .into_iter()
        .map(|route| RouteOwnershipView {
            tenant_id: route.tenant_id,
            principal_id: route.principal_id,
            device_id: route.device_id,
            owner_node_id: route.owner_node_id,
            connection_kind: route.connection_kind,
            bound_at: route.bound_at,
        })
        .collect::<Vec<_>>();
    state.ops_runtime.update_route_ownership(routes);
    let inspection = match state.runtime_dir.as_deref() {
        Some(runtime_dir) => inspect_runtime_dir(runtime_dir),
        None => RuntimeDirInspectionView::unmanaged(),
    };
    state.ops_runtime.update_runtime_dir_inspection(inspection);
    state
        .ops_runtime
        .replace_provider_binding_snapshots(state.provider_binding_snapshots());
    state.ops_runtime.update_projection_live_lag(
        state
            .projection_service
            .projection_live_lag_items()
            .into_iter()
            .map(|item| ops_service::LagItem {
                component: item.component,
                scope_id: item.scope_id,
                current_offset: item.current_offset,
                committed_offset: item.committed_offset,
                lag: item.lag,
            })
            .collect(),
    );
    state
        .ops_runtime
        .update_projection_plane(map_projection_plane_observability(
            state.projection_service.projection_plane_observability(),
        ));
    state
        .ops_runtime
        .update_side_effect_outboxes(message_side_effect_outbox_diagnostics_for_ops(state));
    state
        .ops_runtime
        .update_realtime_inbox(realtime_inbox_diagnostics_for_ops(state));
}

fn message_side_effect_outbox_diagnostics_for_ops(
    state: &AppState,
) -> Vec<ops_service::SideEffectOutboxDiagnosticsView> {
    match side_effect_outbox::message_side_effect_outbox_diagnostics(state) {
        Ok(view) => view,
        Err(_error) => vec![ops_service::SideEffectOutboxDiagnosticsView {
            name: "message_realtime_delivery".into(),
            status: "unavailable".into(),
            pending_count: 0,
            delivered_count: 0,
            failed_attempt_count: 0,
            oldest_pending_created_at: None,
        }],
    }
}

fn realtime_inbox_diagnostics_for_ops(
    state: &AppState,
) -> ops_service::RealtimeInboxDiagnosticsView {
    match state.realtime_runtime.realtime_inbox_diagnostics() {
        Ok(snapshot) => ops_service::RealtimeInboxDiagnosticsView {
            status: snapshot.status,
            device_window_count: snapshot.device_window_count,
            pending_event_count: snapshot.pending_event_count,
            max_device_window_event_count: snapshot.max_device_window_event_count,
            device_window_capacity: snapshot.device_window_capacity,
            max_device_window_usage_permille: snapshot.max_device_window_usage_permille,
            max_trimmed_through_seq: snapshot.max_trimmed_through_seq,
            capacity_trimmed_event_count: snapshot.capacity_trimmed_event_count,
            max_capacity_trimmed_through_seq: snapshot.max_capacity_trimmed_through_seq,
            last_capacity_trimmed_at: snapshot.last_capacity_trimmed_at,
            oldest_pending_occurred_at: snapshot.oldest_pending_occurred_at,
            high_risk_windows: snapshot
                .high_risk_windows
                .into_iter()
                .map(|window| ops_service::RealtimeInboxHighRiskWindowView {
                    tenant_id: window.tenant_id,
                    principal_kind: window.principal_kind,
                    principal_id: window.principal_id,
                    device_id: window.device_id,
                    pending_event_count: window.pending_event_count,
                    trimmed_through_seq: window.trimmed_through_seq,
                    capacity_trimmed_event_count: window.capacity_trimmed_event_count,
                    capacity_trimmed_through_seq: window.capacity_trimmed_through_seq,
                    last_capacity_trimmed_at: window.last_capacity_trimmed_at,
                    usage_permille: window.usage_permille,
                    oldest_pending_occurred_at: window.oldest_pending_occurred_at,
                })
                .collect(),
        },
        Err(_error) => ops_service::RealtimeInboxDiagnosticsView {
            status: "unavailable".into(),
            device_window_count: 0,
            pending_event_count: 0,
            max_device_window_event_count: 0,
            device_window_capacity: 0,
            max_device_window_usage_permille: 0,
            max_trimmed_through_seq: 0,
            capacity_trimmed_event_count: 0,
            max_capacity_trimmed_through_seq: 0,
            last_capacity_trimmed_at: None,
            oldest_pending_occurred_at: None,
            high_risk_windows: Vec::new(),
        },
    }
}

fn map_projection_plane_observability(
    view: projection_service::ProjectionPlaneObservabilityView,
) -> ops_service::ProjectionPlaneDiagnosticsView {
    ops_service::ProjectionPlaneDiagnosticsView {
        status: view.status,
        metrics: ops_service::ProjectionPlaneMetricsView {
            conversation_snapshot_persist: map_projection_metric_counter(
                view.metrics.conversation_snapshot_persist,
            ),
            conversation_snapshot_restore: map_projection_metric_counter(
                view.metrics.conversation_snapshot_restore,
            ),
            device_sync_snapshot_persist: map_projection_metric_counter(
                view.metrics.device_sync_snapshot_persist,
            ),
            device_sync_snapshot_restore: map_projection_metric_counter(
                view.metrics.device_sync_snapshot_restore,
            ),
        },
        replay: ops_service::ProjectionReplayMetricsView {
            backlog_size: view.replay.backlog_size,
            replayed_event_count: view.replay.replayed_event_count,
            duration_ms: view.replay.duration_ms,
        },
        rebuild_duration_ms: view.rebuild_duration_ms,
        update_delay: ops_service::ProjectionUpdateDelayView {
            timeline_ms: view.update_delay.timeline_ms,
            inbox_ms: view.update_delay.inbox_ms,
            source_event_type: view.update_delay.source_event_type,
            scope_id: view.update_delay.scope_id,
            recorded_at: view.update_delay.recorded_at,
        },
        last_failure_code: view.last_failure_code,
        last_failure_message: view.last_failure_message,
        traces: view
            .traces
            .into_iter()
            .map(|trace| ops_service::ProjectionPlaneTraceView {
                trace_id: trace.trace_id,
                operation: trace.operation,
                scope_type: trace.scope_type,
                scope_id: trace.scope_id,
                outcome: trace.outcome,
                recorded_at: trace.recorded_at,
            })
            .collect(),
        logs: view
            .logs
            .into_iter()
            .map(|log| ops_service::ProjectionPlaneLogView {
                level: log.level,
                code: log.code,
                operation: log.operation,
                scope_type: log.scope_type,
                scope_id: log.scope_id,
                message: log.message,
                recorded_at: log.recorded_at,
            })
            .collect(),
    }
}

fn map_projection_metric_counter(
    counter: projection_service::ProjectionOperationMetricView,
) -> ops_service::ProjectionPlaneMetricCounterView {
    ops_service::ProjectionPlaneMetricCounterView {
        attempt_count: counter.attempt_count,
        success_count: counter.success_count,
        failure_count: counter.failure_count,
    }
}
