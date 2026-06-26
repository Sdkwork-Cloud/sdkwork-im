use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard};

use axum::extract::{DefaultBodyLimit, Extension, State};
use axum::http::{HeaderMap, Request, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{Json, Router, routing::{get, post}};
use im_app_context::{AppContext, AppContextError, resolve_app_context};
use im_adapters_postgres_journal::{PostgresJournalConfig, RetentionCleanupReport, purge_expired_retention_batch, retention_purge_metrics};
use sdkwork_im_web_bootstrap::{im_service_router_config, mount_im_infra_routes};
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

const OPS_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_OPS_MAX_IN_FLIGHT_REQUESTS";
const OPS_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const OPS_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const OPS_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_OPS_MAX_REQUEST_BODY_BYTES";
const OPS_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const OPS_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const IM_DATABASE_URL_ENV: &str = "SDKWORK_IM_DATABASE_URL";
const RETENTION_PURGE_DEFAULT_BATCH_SIZE: i64 = 500;
const RETENTION_PURGE_MAX_BATCH_SIZE: i64 = 5_000;

#[derive(Clone)]
pub struct AppState {
    runtime: Arc<OpsRuntime>,
}

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceHealthView {
    pub service: String,
    pub status: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpsHealthResponse {
    pub status: String,
    pub items: Vec<ServiceHealthView>,
    pub projection_plane: ProjectionPlaneHealthView,
    pub realtime_inbox: RealtimeInboxDiagnosticsView,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterNodeView {
    pub node_id: String,
    pub profile: String,
    pub bind_addr: String,
    pub drain_status: String,
    pub rebalance_state: String,
    pub client_route_count: usize,
    pub owned_scopes: Vec<String>,
    pub services: Vec<ServiceHealthView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterView {
    pub nodes: Vec<ClusterNodeView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LagItem {
    pub component: String,
    pub scope_id: String,
    pub current_offset: u64,
    pub committed_offset: u64,
    pub lag: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LagView {
    pub items: Vec<LagItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionReplayStatusView {
    pub generated_at: String,
    pub status: String,
    pub replay: ProjectionReplayMetricsView,
    pub replay_throughput_per_second: u64,
    pub lag: Vec<LagItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirInspectionItem {
    pub file_name: String,
    pub path: String,
    pub required: bool,
    pub exists: bool,
    pub parseable: bool,
    pub status: String,
    pub size_bytes: Option<u64>,
    pub parse_error: Option<String>,
    pub recommended_action: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeDirInspectionView {
    pub status: String,
    pub runtime_dir: Option<String>,
    pub state_dir: Option<String>,
    pub healthy_file_count: usize,
    pub missing_file_count: usize,
    pub corrupt_file_count: usize,
    pub files: Vec<RuntimeDirInspectionItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetentionPurgeResponse {
    pub generated_at: String,
    pub batch_size: i64,
    pub commit_journal_deleted: u64,
    pub conversation_messages_deleted: u64,
    pub message_media_refs_deleted: u64,
    pub outbox_events_deleted: u64,
    pub inbox_events_deleted: u64,
    pub projection_timeline_entries_deleted: u64,
    pub realtime_device_events_deleted: u64,
}

impl RuntimeDirInspectionView {
    pub fn unmanaged() -> Self {
        Self {
            status: "unmanaged".into(),
            runtime_dir: None,
            state_dir: None,
            healthy_file_count: 0,
            missing_file_count: 0,
            corrupt_file_count: 0,
            files: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionPlaneMetricCounterView {
    pub attempt_count: u64,
    pub success_count: u64,
    pub failure_count: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionPlaneMetricsView {
    pub conversation_snapshot_persist: ProjectionPlaneMetricCounterView,
    pub conversation_snapshot_restore: ProjectionPlaneMetricCounterView,
    pub client_route_sync_snapshot_persist: ProjectionPlaneMetricCounterView,
    pub client_route_sync_snapshot_restore: ProjectionPlaneMetricCounterView,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionReplayMetricsView {
    pub backlog_size: u64,
    pub replayed_event_count: u64,
    pub duration_ms: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionUpdateDelayView {
    pub timeline_ms: u64,
    pub inbox_ms: u64,
    pub source_event_type: Option<String>,
    pub scope_id: Option<String>,
    pub recorded_at: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionPlaneTraceView {
    pub trace_id: String,
    pub operation: String,
    pub scope_type: String,
    pub scope_id: String,
    pub outcome: String,
    pub recorded_at: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionPlaneLogView {
    pub level: String,
    pub code: String,
    pub operation: String,
    pub scope_type: String,
    pub scope_id: String,
    pub message: String,
    pub recorded_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionPlaneHealthView {
    pub status: String,
    pub metrics: ProjectionPlaneMetricsView,
    pub replay: ProjectionReplayMetricsView,
    pub rebuild_duration_ms: u64,
    pub update_delay: ProjectionUpdateDelayView,
    pub last_failure_code: Option<String>,
    pub last_failure_message: Option<String>,
}

impl Default for ProjectionPlaneHealthView {
    fn default() -> Self {
        Self {
            status: "idle".into(),
            metrics: ProjectionPlaneMetricsView::default(),
            replay: ProjectionReplayMetricsView::default(),
            rebuild_duration_ms: 0,
            update_delay: ProjectionUpdateDelayView::default(),
            last_failure_code: None,
            last_failure_message: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionPlaneDiagnosticsView {
    pub status: String,
    pub metrics: ProjectionPlaneMetricsView,
    pub replay: ProjectionReplayMetricsView,
    pub rebuild_duration_ms: u64,
    pub update_delay: ProjectionUpdateDelayView,
    pub last_failure_code: Option<String>,
    pub last_failure_message: Option<String>,
    pub traces: Vec<ProjectionPlaneTraceView>,
    pub logs: Vec<ProjectionPlaneLogView>,
}

impl Default for ProjectionPlaneDiagnosticsView {
    fn default() -> Self {
        Self {
            status: "idle".into(),
            metrics: ProjectionPlaneMetricsView::default(),
            replay: ProjectionReplayMetricsView::default(),
            rebuild_duration_ms: 0,
            update_delay: ProjectionUpdateDelayView::default(),
            last_failure_code: None,
            last_failure_message: None,
            traces: Vec::new(),
            logs: Vec::new(),
        }
    }
}

impl From<ProjectionPlaneDiagnosticsView> for ProjectionPlaneHealthView {
    fn from(value: ProjectionPlaneDiagnosticsView) -> Self {
        Self {
            status: value.status,
            metrics: value.metrics,
            replay: value.replay,
            rebuild_duration_ms: value.rebuild_duration_ms,
            update_delay: value.update_delay,
            last_failure_code: value.last_failure_code,
            last_failure_message: value.last_failure_message,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SideEffectOutboxDiagnosticsView {
    pub name: String,
    pub status: String,
    pub pending_count: u64,
    pub delivered_count: u64,
    pub failed_attempt_count: u64,
    pub oldest_pending_created_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeInboxDiagnosticsView {
    pub status: String,
    pub client_route_window_count: u64,
    pub pending_event_count: u64,
    pub max_client_route_window_event_count: u64,
    pub client_route_window_capacity: u64,
    pub max_client_route_window_usage_permille: u64,
    pub max_trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub max_capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
    pub oldest_pending_occurred_at: Option<String>,
    pub high_risk_windows: Vec<RealtimeInboxHighRiskWindowView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeInboxHighRiskWindowView {
    pub tenant_id: String,
    pub principal_kind: String,
    pub principal_id: String,
    pub device_id: String,
    pub pending_event_count: u64,
    pub trimmed_through_seq: u64,
    pub capacity_trimmed_event_count: u64,
    pub capacity_trimmed_through_seq: u64,
    pub last_capacity_trimmed_at: Option<String>,
    pub usage_permille: u64,
    pub oldest_pending_occurred_at: Option<String>,
}

impl Default for RealtimeInboxDiagnosticsView {
    fn default() -> Self {
        Self {
            status: "ok".into(),
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
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticBundle {
    pub generated_at: String,
    pub profile: String,
    pub node_id: String,
    pub bind_addr: String,
    pub drain_status: String,
    pub rebalance_state: String,
    pub owned_scopes: Vec<String>,
    pub services: Vec<ServiceHealthView>,
    pub lag: Vec<LagItem>,
    pub client_routes: Vec<RouteOwnershipView>,
    pub provider_bindings: Vec<ProviderBindingSnapshotView>,
    pub provider_binding_drift: ProviderBindingDriftView,
    pub projection_plane: ProjectionPlaneDiagnosticsView,
    pub side_effect_outboxes: Vec<SideEffectOutboxDiagnosticsView>,
    pub realtime_inbox: RealtimeInboxDiagnosticsView,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteOwnershipView {
    pub tenant_id: String,
    pub principal_id: String,
    pub device_id: String,
    pub owner_node_id: String,
    pub connection_kind: String,
    pub bound_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingItemView {
    pub domain: String,
    pub default_plugin_id: Option<String>,
    pub selected_plugin_id: Option<String>,
    pub selection_source: String,
    pub tenant_override_allowed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingSnapshotView {
    pub interface_version: String,
    pub tenant_id: Option<String>,
    pub effective_bindings: Vec<ProviderBindingItemView>,
    pub precedence: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingsView {
    pub items: Vec<ProviderBindingSnapshotView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingDriftItemView {
    pub tenant_id: String,
    pub domain: String,
    pub baseline_selected_plugin_id: Option<String>,
    pub selected_plugin_id: Option<String>,
    pub baseline_selection_source: String,
    pub selection_source: String,
    pub drift_kind: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderBindingDriftView {
    pub baseline_tenant_id: Option<String>,
    pub items: Vec<ProviderBindingDriftItemView>,
}

pub struct OpsRuntime {
    node_id: String,
    profile: String,
    bind_addr: String,
    services: Vec<ServiceHealthView>,
    owned_scopes: Vec<String>,
    lag_items: Mutex<Vec<LagItem>>,
    drain_status: Mutex<String>,
    rebalance_state: Mutex<String>,
    client_routes: Mutex<Vec<RouteOwnershipView>>,
    provider_bindings: Mutex<BTreeMap<String, ProviderBindingSnapshotView>>,
    runtime_dir_inspection: Mutex<RuntimeDirInspectionView>,
    projection_plane: Mutex<ProjectionPlaneDiagnosticsView>,
    side_effect_outboxes: Mutex<Vec<SideEffectOutboxDiagnosticsView>>,
    realtime_inbox: Mutex<RealtimeInboxDiagnosticsView>,
}

impl Default for OpsRuntime {
    fn default() -> Self {
        Self::new(
            "ops_node_1",
            "standalone",
            "127.0.0.1:18091",
            vec!["ops-service".into()],
            vec!["node:ops_node_1".into()],
        )
    }
}

impl OpsRuntime {
    pub fn new(
        node_id: impl Into<String>,
        profile: impl Into<String>,
        bind_addr: impl Into<String>,
        service_names: Vec<String>,
        owned_scopes: Vec<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            profile: profile.into(),
            bind_addr: bind_addr.into(),
            services: service_names
                .into_iter()
                .map(|service| ServiceHealthView {
                    service,
                    status: "ok".into(),
                })
                .collect(),
            owned_scopes,
            lag_items: Mutex::new(default_lag_items()),
            drain_status: Mutex::new("active".into()),
            rebalance_state: Mutex::new("stable".into()),
            client_routes: Mutex::new(Vec::new()),
            provider_bindings: Mutex::new(BTreeMap::new()),
            runtime_dir_inspection: Mutex::new(RuntimeDirInspectionView::unmanaged()),
            projection_plane: Mutex::new(ProjectionPlaneDiagnosticsView::default()),
            side_effect_outboxes: Mutex::new(Vec::new()),
            realtime_inbox: Mutex::new(RealtimeInboxDiagnosticsView::default()),
        }
    }

    pub fn set_node_lifecycle(&self, drain_status: &str, rebalance_state: &str) {
        *lock_ops_mutex(&self.drain_status, "ops drain status") = drain_status.into();
        *lock_ops_mutex(&self.rebalance_state, "ops rebalance state") = rebalance_state.into();
    }

    pub fn update_route_ownership(&self, mut client_routes: Vec<RouteOwnershipView>) {
        client_routes.sort_by(|left, right| {
            left.tenant_id
                .cmp(&right.tenant_id)
                .then_with(|| left.principal_id.cmp(&right.principal_id))
                .then_with(|| left.device_id.cmp(&right.device_id))
        });
        *lock_ops_mutex(&self.client_routes, "ops client routes") = client_routes;
    }

    pub fn update_runtime_dir_inspection(&self, inspection: RuntimeDirInspectionView) {
        *lock_ops_mutex(&self.runtime_dir_inspection, "ops runtime_dir inspection") = inspection;
    }

    pub fn update_provider_binding_snapshot(&self, snapshot: ProviderBindingSnapshotView) {
        let key = snapshot.tenant_id.clone().unwrap_or_default();
        lock_ops_mutex(&self.provider_bindings, "ops provider bindings").insert(key, snapshot);
    }

    pub fn replace_provider_binding_snapshots(&self, snapshots: Vec<ProviderBindingSnapshotView>) {
        let mut provider_bindings =
            lock_ops_mutex(&self.provider_bindings, "ops provider bindings");
        provider_bindings.clear();
        for snapshot in snapshots {
            let key = snapshot.tenant_id.clone().unwrap_or_default();
            provider_bindings.insert(key, snapshot);
        }
    }

    pub fn update_projection_plane(&self, projection_plane: ProjectionPlaneDiagnosticsView) {
        *lock_ops_mutex(&self.projection_plane, "ops projection-plane") = projection_plane;
    }

    pub fn update_side_effect_outboxes(
        &self,
        mut side_effect_outboxes: Vec<SideEffectOutboxDiagnosticsView>,
    ) {
        side_effect_outboxes.sort_by(|left, right| left.name.cmp(&right.name));
        *lock_ops_mutex(&self.side_effect_outboxes, "ops side-effect outboxes") =
            side_effect_outboxes;
    }

    pub fn update_realtime_inbox(&self, realtime_inbox: RealtimeInboxDiagnosticsView) {
        *lock_ops_mutex(&self.realtime_inbox, "ops realtime inbox") = realtime_inbox;
    }

    pub fn update_projection_replay_lag(&self, mut projection_lag_items: Vec<LagItem>) {
        projection_lag_items.retain(|item| item.component == "projection_replay");
        if projection_lag_items.is_empty() {
            projection_lag_items.push(default_projection_replay_lag_item());
        }
        projection_lag_items.sort_by(|left, right| left.scope_id.cmp(&right.scope_id));

        let mut lag_items = lock_ops_mutex(&self.lag_items, "ops lag items");
        let mut merged = lag_items
            .iter()
            .filter(|item| item.component != "projection_replay")
            .cloned()
            .collect::<Vec<_>>();
        merged.extend(projection_lag_items);
        *lag_items = merged;
    }

    pub fn update_projection_live_lag(&self, mut projection_lag_items: Vec<LagItem>) {
        projection_lag_items.retain(|item| item.component == "projection_live");
        projection_lag_items.sort_by(|left, right| left.scope_id.cmp(&right.scope_id));

        let mut lag_items = lock_ops_mutex(&self.lag_items, "ops lag items");
        let mut merged = lag_items
            .iter()
            .filter(|item| item.component != "projection_live")
            .cloned()
            .collect::<Vec<_>>();
        merged.extend(projection_lag_items);
        *lag_items = merged;
    }

    pub fn node_id(&self) -> &str {
        self.node_id.as_str()
    }

    pub fn health_view(&self) -> OpsHealthResponse {
        let projection_plane =
            lock_ops_mutex(&self.projection_plane, "ops projection-plane").clone();
        let realtime_inbox = lock_ops_mutex(&self.realtime_inbox, "ops realtime inbox").clone();
        let status = rollup_health_status(
            self.services
                .iter()
                .map(|service| service.status.as_str())
                .chain([
                    projection_plane.status.as_str(),
                    realtime_inbox.status.as_str(),
                ]),
        )
        .into();
        OpsHealthResponse {
            status,
            items: self.services.clone(),
            projection_plane: projection_plane.into(),
            realtime_inbox,
        }
    }

    pub fn cluster_view(&self) -> ClusterView {
        let drain_status = lock_ops_mutex(&self.drain_status, "ops drain status").clone();
        let rebalance_state = lock_ops_mutex(&self.rebalance_state, "ops rebalance state").clone();
        let client_route_count = lock_ops_mutex(&self.client_routes, "ops client routes").len();
        ClusterView {
            nodes: vec![ClusterNodeView {
                node_id: self.node_id.clone(),
                profile: self.profile.clone(),
                bind_addr: self.bind_addr.clone(),
                drain_status,
                rebalance_state,
                client_route_count,
                owned_scopes: self.owned_scopes.clone(),
                services: self.services.clone(),
            }],
        }
    }

    pub fn lag_view(&self) -> LagView {
        LagView {
            items: lock_ops_mutex(&self.lag_items, "ops lag items").clone(),
        }
    }

    pub fn runtime_dir_view(&self) -> RuntimeDirInspectionView {
        lock_ops_mutex(&self.runtime_dir_inspection, "ops runtime_dir inspection").clone()
    }

    pub fn provider_bindings_view(&self) -> ProviderBindingsView {
        ProviderBindingsView {
            items: lock_ops_mutex(&self.provider_bindings, "ops provider bindings")
                .values()
                .cloned()
                .collect(),
        }
    }

    pub fn provider_binding_drift_view(&self) -> ProviderBindingDriftView {
        let provider_bindings = lock_ops_mutex(&self.provider_bindings, "ops provider bindings");
        let Some(global_snapshot) = provider_bindings.get("") else {
            return ProviderBindingDriftView::default();
        };

        let baseline_bindings = global_snapshot
            .effective_bindings
            .iter()
            .map(|binding| (binding.domain.clone(), binding))
            .collect::<BTreeMap<_, _>>();

        let items = provider_bindings
            .iter()
            .filter_map(|(tenant_key, snapshot)| {
                if tenant_key.is_empty() {
                    return None;
                }

                Some(
                    snapshot
                        .effective_bindings
                        .iter()
                        .filter_map(|binding| {
                            let baseline = baseline_bindings.get(binding.domain.as_str())?;
                            provider_binding_drift_item(tenant_key.as_str(), baseline, binding)
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .collect();

        ProviderBindingDriftView {
            baseline_tenant_id: None,
            items,
        }
    }

    pub fn replay_status_view(&self) -> ProjectionReplayStatusView {
        let projection_plane =
            lock_ops_mutex(&self.projection_plane, "ops projection-plane").clone();
        let lag = lock_ops_mutex(&self.lag_items, "ops lag items")
            .iter()
            .filter(|item| item.component == "projection_replay")
            .cloned()
            .collect::<Vec<_>>();
        ProjectionReplayStatusView {
            generated_at: utc_now_rfc3339_millis(),
            status: projection_replay_status(&projection_plane.replay).into(),
            replay_throughput_per_second: projection_replay_throughput_per_second(
                &projection_plane.replay,
            ),
            replay: projection_plane.replay,
            lag,
        }
    }

    pub fn diagnostic_bundle(&self) -> DiagnosticBundle {
        let drain_status = lock_ops_mutex(&self.drain_status, "ops drain status").clone();
        let rebalance_state = lock_ops_mutex(&self.rebalance_state, "ops rebalance state").clone();
        let client_routes = lock_ops_mutex(&self.client_routes, "ops client routes").clone();
        let provider_bindings = lock_ops_mutex(&self.provider_bindings, "ops provider bindings")
            .values()
            .cloned()
            .collect();
        let provider_binding_drift = self.provider_binding_drift_view();
        let projection_plane =
            lock_ops_mutex(&self.projection_plane, "ops projection-plane").clone();
        let side_effect_outboxes =
            lock_ops_mutex(&self.side_effect_outboxes, "ops side-effect outboxes").clone();
        let realtime_inbox = lock_ops_mutex(&self.realtime_inbox, "ops realtime inbox").clone();
        let lag = lock_ops_mutex(&self.lag_items, "ops lag items").clone();
        DiagnosticBundle {
            generated_at: utc_now_rfc3339_millis(),
            profile: self.profile.clone(),
            node_id: self.node_id.clone(),
            bind_addr: self.bind_addr.clone(),
            drain_status,
            rebalance_state,
            owned_scopes: self.owned_scopes.clone(),
            services: self.services.clone(),
            lag,
            client_routes,
            provider_bindings,
            provider_binding_drift,
            projection_plane,
            side_effect_outboxes,
            realtime_inbox,
        }
    }
}

fn lock_ops_mutex<'a, T>(mutex: &'a Mutex<T>, lock_name: &'static str) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            tracing::warn!("recovered poisoned ops mutex lock={lock_name}");
            poisoned.into_inner()
        }
    }
}

fn default_lag_items() -> Vec<LagItem> {
    vec![
        LagItem {
            component: "commit_journal".into(),
            scope_id: "self-hosted.split-services.development".into(),
            current_offset: 0,
            committed_offset: 0,
            lag: 0,
        },
        default_projection_replay_lag_item(),
    ]
}

fn rollup_health_status<'a>(statuses: impl IntoIterator<Item = &'a str>) -> &'static str {
    let mut severity = 0_u8;
    for status in statuses {
        severity = severity.max(health_status_severity(status));
    }
    match severity {
        4 => "critical",
        3 => "unavailable",
        2 => "degraded",
        _ => "ok",
    }
}

fn health_status_severity(status: &str) -> u8 {
    match status {
        "critical" => 4,
        "unavailable" => 3,
        "degraded" => 2,
        "ok" | "idle" => 0,
        _ => 2,
    }
}

fn default_projection_replay_lag_item() -> LagItem {
    LagItem {
        component: "projection_replay".into(),
        scope_id: "projection:*".into(),
        current_offset: 0,
        committed_offset: 0,
        lag: 0,
    }
}

fn projection_replay_status(replay: &ProjectionReplayMetricsView) -> &'static str {
    if replay.backlog_size == 0 && replay.replayed_event_count == 0 {
        "idle"
    } else {
        "replayed"
    }
}

fn projection_replay_throughput_per_second(replay: &ProjectionReplayMetricsView) -> u64 {
    if replay.duration_ms == 0 {
        0
    } else {
        replay.replayed_event_count.saturating_mul(1000) / replay.duration_ms
    }
}

#[derive(Debug)]
pub struct OpsError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl From<AppContextError> for OpsError {
    fn from(value: AppContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl OpsError {
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
        }
    }

    fn service_unavailable(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code,
            message: message.into(),
        }
    }
}

impl axum::response::IntoResponse for OpsError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(serde_json::json!({
                "code": self.code,
                "message": self.message
            })),
        )
            .into_response()
    }
}

pub fn default_app_state() -> AppState {
    AppState {
        runtime: Arc::new(OpsRuntime::default()),
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(OpsRuntime::default()))
}

pub fn build_domain_api_router(state: AppState) -> Router {
    Router::new()
        .route("/backend/v3/api/ops/health", get(get_ops_health))
        .route("/backend/v3/api/ops/cluster", get(get_cluster))
        .route("/backend/v3/api/ops/lag", get(get_lag))
        .route("/backend/v3/api/ops/replay_status", get(get_replay_status))
        .route("/backend/v3/api/ops/runtime_dir", get(get_runtime_dir))
        .route(
            "/backend/v3/api/ops/provider_bindings",
            get(get_provider_bindings),
        )
        .route(
            "/backend/v3/api/ops/provider_bindings/drift",
            get(get_provider_binding_drift),
        )
        .route("/backend/v3/api/ops/diagnostics", get(get_diagnostics))
        .route(
            "/backend/v3/api/ops/retention/purge",
            post(post_retention_purge),
        )
        .with_state(state)
}

pub fn apply_public_http_guardrails(router: Router) -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
    };
    router
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            enforce_in_flight_gate,
        ))
}

pub fn build_public_app() -> Router {
    mount_im_infra_routes(
        apply_public_http_guardrails(build_business_router(Arc::new(OpsRuntime::default()))),
        im_service_router_config(),
    )
}

pub fn build_app(runtime: Arc<OpsRuntime>) -> Router {
    mount_im_infra_routes(
        build_business_router(runtime),
        im_service_router_config(),
    )
}

fn build_business_router(runtime: Arc<OpsRuntime>) -> Router {
    let state = AppState { runtime };
    Router::new()
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .merge(build_domain_api_router(state))
}

async fn enforce_in_flight_gate(
    State(guardrails): State<PublicAppGuardrails>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    if matches!(
        request.uri().path(),
        "/healthz" | "/readyz" | "/livez" | "/metrics" | "/openapi.json" | "/docs"
    ) {
        return next.run(request).await;
    }
    let permit = match guardrails.request_gate.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            return OpsError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "http_overloaded",
                message:
                    "server is at maximum in-flight request capacity, please retry later".to_owned(),
            }
            .into_response();
        }
    };
    let response = next.run(request).await;
    drop(permit);
    response
}

async fn openapi_json() -> Result<Json<serde_json::Value>, OpsError> {
    Ok(Json(build_ops_service_openapi_document().map_err(
        |message| OpsError::internal("openapi_export_failed", message),
    )?))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&ops_service_openapi_spec()))
}

fn build_ops_service_openapi_document() -> Result<serde_json::Value, String> {
    let source = include_str!("lib.rs");
    let mut routes = extract_routes_from_function(
        source,
        "build_business_router",
        &[],
        &["/openapi.json", "/docs"],
    )?;
    routes.extend(extract_routes_from_function(
        source,
        "build_domain_api_router",
        &[],
        &[],
    )?);

    Ok(build_openapi_document(
        &ops_service_openapi_spec(),
        &routes,
        ops_service_tag,
        ops_service_requires_app_context,
        ops_service_summary,
    ))
}

fn ops_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Ops Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the ops-service router for cluster, lag, diagnostics, runtime_dir, replay status, and provider binding inspections.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn ops_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" | "/metrics" => "system".to_owned(),
        path if path.contains("provider_bindings") => "provider_bindings".to_owned(),
        path if path.contains("diagnostics") => "diagnostics".to_owned(),
        path if path.contains("retention") => "retention".to_owned(),
        _ => "ops".to_owned(),
    }
}

fn ops_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz" | "/metrics")
}

fn ops_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check ops service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check ops service readiness".to_owned(),
        _ => format!(
            "{} {}",
            ops_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn ops_service_method_display(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Delete => "Delete",
        HttpMethod::Get => "Get",
        HttpMethod::Head => "Head",
        HttpMethod::Options => "Options",
        HttpMethod::Patch => "Patch",
        HttpMethod::Post => "Post",
        HttpMethod::Put => "Put",
    }
}

async fn get_ops_health(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<OpsHealthResponse>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.health_view()))
}

async fn get_cluster(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ClusterView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.cluster_view()))
}

async fn get_lag(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<LagView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.lag_view()))
}

async fn get_runtime_dir(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<RuntimeDirInspectionView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.runtime_dir_view()))
}

async fn get_provider_bindings(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingsView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.provider_bindings_view()))
}

async fn get_provider_binding_drift(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProviderBindingDriftView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.provider_binding_drift_view()))
}

async fn get_replay_status(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ProjectionReplayStatusView>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.replay_status_view()))
}

async fn get_diagnostics(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<DiagnosticBundle>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_read_access(&auth)?;
    Ok(Json(state.runtime.diagnostic_bundle()))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RetentionPurgeQuery {
    batch_size: Option<i64>,
}

async fn post_retention_purge(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    axum::extract::Query(query): axum::extract::Query<RetentionPurgeQuery>,
) -> Result<Json<RetentionPurgeResponse>, OpsError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_ops_write_access(&auth)?;

    let database_url = std::env::var(IM_DATABASE_URL_ENV).map_err(|_| {
        OpsError::service_unavailable(
            "database_unconfigured",
            format!("{IM_DATABASE_URL_ENV} is required for retention purge"),
        )
    })?;
    let batch_size = query
        .batch_size
        .unwrap_or(RETENTION_PURGE_DEFAULT_BATCH_SIZE)
        .clamp(1, RETENTION_PURGE_MAX_BATCH_SIZE);
    let config = PostgresJournalConfig::new(database_url);
    let pool = config.connect_pool().map_err(|error| {
        OpsError::service_unavailable("database_unavailable", format!("{error:?}"))
    })?;

    let report = tokio::task::spawn_blocking(move || purge_expired_retention_batch(&pool, Some(batch_size)))
        .await
        .map_err(|_| OpsError::internal("retention_purge_failed", "retention purge worker panicked"))?
        .map_err(|error| OpsError::internal("retention_purge_failed", format!("{error:?}")))?;

    Ok(Json(retention_purge_response(batch_size, report)))
}

fn retention_purge_response(batch_size: i64, report: RetentionCleanupReport) -> RetentionPurgeResponse {
    RetentionPurgeResponse {
        generated_at: utc_now_rfc3339_millis(),
        batch_size,
        commit_journal_deleted: report.commit_journal_deleted,
        conversation_messages_deleted: report.conversation_messages_deleted,
        message_media_refs_deleted: report.message_media_refs_deleted,
        outbox_events_deleted: report.outbox_events_deleted,
        inbox_events_deleted: report.inbox_events_deleted,
        projection_timeline_entries_deleted: report.projection_timeline_entries_deleted,
        realtime_device_events_deleted: report.realtime_device_events_deleted,
    }
}

fn ensure_ops_read_access(auth: &AppContext) -> Result<(), OpsError> {
    if auth.has_permission("ops.read") {
        return Ok(());
    }

    Err(OpsError::forbidden("ops.read"))
}

fn ensure_ops_write_access(auth: &AppContext) -> Result<(), OpsError> {
    if auth.has_permission("ops.write") {
        return Ok(());
    }

    Err(OpsError::forbidden("ops.write"))
}

fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, OpsError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(OpsError::from),
    }
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(OPS_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(OPS_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(OPS_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(OPS_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(OPS_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(OPS_MAX_REQUEST_BODY_BYTES_MAX)
}

fn provider_binding_drift_item(
    tenant_id: &str,
    baseline: &ProviderBindingItemView,
    binding: &ProviderBindingItemView,
) -> Option<ProviderBindingDriftItemView> {
    let plugin_changed = baseline.selected_plugin_id != binding.selected_plugin_id;
    let source_changed = baseline.selection_source != binding.selection_source;
    if !plugin_changed && !source_changed {
        return None;
    }

    let drift_kind = match (plugin_changed, source_changed) {
        (true, true) => "plugin_and_selection_source_changed",
        (true, false) => "plugin_changed",
        (false, true) => "selection_source_changed",
        (false, false) => unreachable!("drift item should only be built when drift exists"),
    };

    Some(ProviderBindingDriftItemView {
        tenant_id: tenant_id.into(),
        domain: binding.domain.clone(),
        baseline_selected_plugin_id: baseline.selected_plugin_id.clone(),
        selected_plugin_id: binding.selected_plugin_id.clone(),
        baseline_selection_source: baseline.selection_source.clone(),
        selection_source: binding.selection_source.clone(),
        drift_kind: drift_kind.into(),
    })
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue};

    use super::*;

    #[test]
    fn test_health_view_recovers_from_poisoned_projection_plane_lock() {
        let runtime = OpsRuntime::default();
        let _ = std::panic::catch_unwind(|| {
            let _guard = runtime
                .projection_plane
                .lock()
                .expect("ops projection-plane should lock");
            panic!("poison ops projection-plane lock");
        });

        let health = runtime.health_view();
        assert_eq!(health.projection_plane.status, "idle");
    }
}
