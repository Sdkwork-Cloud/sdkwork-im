use super::*;

pub(super) async fn get_automation_governance(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<automation_service::AutomationGovernanceSnapshot>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    let snapshot = state
        .automation_runtime
        .governance_snapshot(&auth)
        .map_err(IntoResponse::into_response)?;
    Ok(Json(snapshot))
}

pub(super) async fn record_audit_anchor(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<RecordAuditAnchor>,
) -> Result<Json<AuditRecordMutationResponse>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
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

pub(super) async fn list_audit_records(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_audit_read_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(serde_json::json!({
        "items": state.audit_runtime.list_records(&auth)
    })))
}

pub(super) async fn export_audit_bundle(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<AuditExportBundle>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_audit_read_access(&auth).map_err(IntoResponse::into_response)?;
    Ok(Json(state.audit_runtime.export_bundle(&auth)))
}

pub(super) async fn get_ops_health(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<OpsHealthResponse>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.health_view()))
}

pub(super) async fn get_ops_cluster(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ClusterView>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.cluster_view()))
}

pub(super) async fn get_ops_lag(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<LagView>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.lag_view()))
}

pub(super) async fn get_ops_replay_status(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ops_service::ProjectionReplayStatusView>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.replay_status_view()))
}

pub(super) async fn get_ops_commercial_readiness(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
) -> Result<Json<CommercialReadinessReport>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    evaluate_commercial_readiness_from_env(resolve_commercial_evidence_root())
        .map(Json)
        .map_err(|error| {
            ApiError::unavailable(
                "commercial_readiness_evidence_unavailable",
                format!("commercial readiness evidence unavailable: {error}"),
            )
            .into_response()
        })
}

pub(super) async fn get_ops_runtime_dir(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<RuntimeDirInspectionView>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.runtime_dir_view()))
}

pub(super) async fn get_ops_provider_bindings(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingsView>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.provider_bindings_view()))
}

pub(super) async fn get_ops_provider_binding_drift(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingDriftView>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
    access::ensure_ops_read_access(&auth).map_err(IntoResponse::into_response)?;
    refresh_node_operational_view(&state);
    Ok(Json(state.ops_runtime.provider_binding_drift_view()))
}

pub(super) async fn get_ops_diagnostics(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<DiagnosticBundle>, axum::response::Response> {
    let auth = resolve_request_app_context(auth, &headers).map_err(IntoResponse::into_response)?;
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
            client_route_window_count: snapshot.client_route_window_count,
            pending_event_count: snapshot.pending_event_count,
            max_client_route_window_event_count: snapshot.max_client_route_window_event_count,
            client_route_window_capacity: snapshot.client_route_window_capacity,
            max_client_route_window_usage_permille: snapshot.max_client_route_window_usage_permille,
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
            client_route_window_count: 0,
            pending_event_count: 0,
            max_client_route_window_event_count: 0,
            client_route_window_capacity: 0,
            max_client_route_window_usage_permille: 0,
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
            client_route_sync_snapshot_persist: map_projection_metric_counter(
                view.metrics.client_route_sync_snapshot_persist,
            ),
            client_route_sync_snapshot_restore: map_projection_metric_counter(
                view.metrics.client_route_sync_snapshot_restore,
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
