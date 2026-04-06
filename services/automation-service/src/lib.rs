use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
pub use im_domain_core::automation::{AutomationExecution, AutomationExecutionState};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, CommitJournal, CommitPosition,
    ContractError,
};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    runtime: Arc<AutomationRuntime>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestAutomationExecution {
    pub execution_id: String,
    pub trigger_type: String,
    pub target_kind: String,
    pub target_ref: String,
    pub input_payload: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AutomationExecutionRequestResult {
    pub execution: AutomationExecution,
    pub is_new: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

pub struct AutomationRuntime {
    executions: Mutex<HashMap<String, AutomationExecution>>,
    journal: Arc<dyn CommitJournal + Send + Sync>,
    execution_store: Arc<dyn AutomationExecutionStore>,
}

#[derive(Default)]
struct NoopJournal;

impl CommitJournal for NoopJournal {
    fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        Ok(CommitPosition::new("noop", 0))
    }
}

impl Default for AutomationRuntime {
    fn default() -> Self {
        Self::with_journal(Arc::new(NoopJournal))
    }
}

#[derive(Debug)]
pub struct AutomationError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl AutomationError {
    fn not_found(execution_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "automation_execution_not_found",
            message: format!("automation execution not found: {execution_id}"),
        }
    }

    fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
        }
    }

    fn conflict(execution_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "automation_execution_conflict",
            message: format!("automation execution conflict: {execution_id}"),
        }
    }

    fn automation_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "automation_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "automation_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "automation_store_unsupported",
                message,
            },
        }
    }
}

impl From<AuthContextError> for AutomationError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<ContractError> for AutomationError {
    fn from(_value: ContractError) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "journal_unavailable",
            message: "commit journal unavailable".into(),
        }
    }
}

impl axum::response::IntoResponse for AutomationError {
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

impl AutomationRuntime {
    pub fn with_journal<J>(journal: Arc<J>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self::with_journal_and_store(
            journal,
            Arc::new(RuntimeMemoryAutomationExecutionStore::default()),
        )
    }

    pub fn with_journal_and_store<J, S>(journal: Arc<J>, execution_store: Arc<S>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        S: AutomationExecutionStore + 'static,
    {
        Self {
            executions: Mutex::new(HashMap::new()),
            journal,
            execution_store,
        }
    }

    fn ensure_execution_state(
        &self,
        tenant_id: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<(), AutomationError> {
        let scope_key = execution_scope_key(tenant_id, principal_id, execution_id);
        if self
            .executions
            .lock()
            .expect("automation runtime should lock")
            .contains_key(scope_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .execution_store
            .load_execution(tenant_id, principal_id, execution_id)
            .map_err(AutomationError::automation_store)?;
        if let Some(record) = restored {
            self.executions
                .lock()
                .expect("automation runtime should lock")
                .insert(scope_key, record.execution);
        }

        Ok(())
    }

    pub fn request_execution(
        &self,
        auth: &AuthContext,
        request: RequestAutomationExecution,
    ) -> Result<AutomationExecution, AutomationError> {
        Ok(self
            .request_execution_with_outcome(auth, request)?
            .execution)
    }

    pub fn request_execution_with_outcome(
        &self,
        auth: &AuthContext,
        request: RequestAutomationExecution,
    ) -> Result<AutomationExecutionRequestResult, AutomationError> {
        ensure_automation_execute_access(auth)?;
        self.ensure_execution_state(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        )?;
        let execution_key = execution_scope_key(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            request.execution_id.as_str(),
        );
        let mut executions = self
            .executions
            .lock()
            .expect("automation runtime should lock");

        if let Some(existing) = executions.get(execution_key.as_str()).cloned() {
            if execution_matches_request(&existing, &request) {
                return Ok(AutomationExecutionRequestResult {
                    execution: existing,
                    is_new: false,
                });
            }

            return Err(AutomationError::conflict(request.execution_id.as_str()));
        }

        let requested_at = utc_now_rfc3339_millis();
        let completed_at = utc_now_rfc3339_millis();
        let output_payload = Some(
            serde_json::json!({
                "accepted": true,
                "targetRef": request.target_ref,
            })
            .to_string(),
        );

        let requested = AutomationExecution {
            tenant_id: auth.tenant_id.clone(),
            principal_id: auth.actor_id.clone(),
            principal_kind: auth.actor_kind.clone(),
            execution_id: request.execution_id.clone(),
            trigger_type: request.trigger_type.clone(),
            target_kind: request.target_kind.clone(),
            target_ref: request.target_ref.clone(),
            input_payload: request.input_payload.clone(),
            output_payload: None,
            state: AutomationExecutionState::Requested,
            retry_count: 0,
            requested_at: requested_at.clone(),
            completed_at: None,
            failure_reason: None,
        };
        self.append_event(auth, &requested, "automation.execution_requested", 1)?;

        let completed = AutomationExecution {
            output_payload,
            state: AutomationExecutionState::Succeeded,
            completed_at: Some(completed_at),
            ..requested
        };
        self.append_event(auth, &completed, "automation.execution_completed", 2)?;

        executions.insert(execution_key.clone(), completed.clone());
        if let Err(error) = self
            .execution_store
            .save_execution(self.execution_record(&completed))
        {
            executions.remove(execution_key.as_str());
            return Err(AutomationError::automation_store(error));
        }

        Ok(AutomationExecutionRequestResult {
            execution: completed,
            is_new: true,
        })
    }

    pub fn get_execution(
        &self,
        auth: &AuthContext,
        execution_id: &str,
    ) -> Result<AutomationExecution, AutomationError> {
        ensure_automation_read_access(auth)?;
        self.ensure_execution_state(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            execution_id,
        )?;
        self.executions
            .lock()
            .expect("automation runtime should lock")
            .get(
                execution_scope_key(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    execution_id,
                )
                .as_str(),
            )
            .cloned()
            .ok_or_else(|| AutomationError::not_found(execution_id))
    }

    fn execution_record(&self, execution: &AutomationExecution) -> AutomationExecutionRecord {
        AutomationExecutionRecord {
            tenant_id: execution.tenant_id.clone(),
            principal_id: execution.principal_id.clone(),
            execution_id: execution.execution_id.clone(),
            execution: execution.clone(),
            updated_at: utc_now_rfc3339_millis(),
        }
    }

    fn append_event(
        &self,
        auth: &AuthContext,
        execution: &AutomationExecution,
        event_type: &str,
        ordering_seq: u64,
    ) -> Result<(), AutomationError> {
        let committed_at = execution
            .completed_at
            .clone()
            .unwrap_or_else(|| execution.requested_at.clone());
        let envelope = CommitEnvelope {
            event_id: format!(
                "evt_{}_{}",
                execution.execution_id,
                event_type.replace('.', "_")
            ),
            tenant_id: auth.tenant_id.clone(),
            event_type: event_type.into(),
            event_version: 1,
            aggregate_type: AggregateType::AutomationExecution,
            aggregate_id: execution.execution_id.clone(),
            scope_type: "automation_execution".into(),
            scope_id: execution.execution_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                auth.tenant_id.as_str(),
                execution.execution_id.as_str(),
            ),
            ordering_seq,
            causation_id: None,
            correlation_id: Some(execution.execution_id.clone()),
            idempotency_key: Some(execution.execution_id.clone()),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: execution.requested_at.clone(),
            committed_at,
            payload_schema: Some("automation.execution.v1".into()),
            payload: serde_json::to_string(execution)
                .expect("automation execution should serialize into commit envelope"),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        self.journal.append(envelope)?;
        Ok(())
    }
}

#[derive(Clone, Default)]
struct RuntimeMemoryAutomationExecutionStore {
    executions: Arc<Mutex<HashMap<String, AutomationExecutionRecord>>>,
}

impl AutomationExecutionStore for RuntimeMemoryAutomationExecutionStore {
    fn load_execution(
        &self,
        tenant_id: &str,
        principal_id: &str,
        execution_id: &str,
    ) -> Result<Option<AutomationExecutionRecord>, ContractError> {
        Ok(self
            .executions
            .lock()
            .expect("automation execution store should lock")
            .get(execution_scope_key(tenant_id, principal_id, execution_id).as_str())
            .cloned())
    }

    fn save_execution(&self, record: AutomationExecutionRecord) -> Result<(), ContractError> {
        self.executions
            .lock()
            .expect("automation execution store should lock")
            .insert(
                execution_scope_key(
                    record.tenant_id.as_str(),
                    record.principal_id.as_str(),
                    record.execution_id.as_str(),
                ),
                record,
            );
        Ok(())
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(AutomationRuntime::default()))
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(runtime: Arc<AutomationRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/automation/executions", post(request_execution))
        .route(
            "/api/v1/automation/executions/{execution_id}",
            get(get_execution),
        )
        .with_state(AppState { runtime })
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => AutomationError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "automation-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "automation-service",
    })
}

async fn request_execution(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestAutomationExecution>,
) -> Result<Json<AutomationExecution>, AutomationError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(state.runtime.request_execution(&auth, request)?))
}

async fn get_execution(
    Path(execution_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AutomationExecution>, AutomationError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state.runtime.get_execution(&auth, execution_id.as_str())?,
    ))
}

fn ensure_automation_execute_access(auth: &AuthContext) -> Result<(), AutomationError> {
    if auth.has_permission("automation.execute") {
        return Ok(());
    }

    Err(AutomationError::forbidden("automation.execute"))
}

fn ensure_automation_read_access(auth: &AuthContext) -> Result<(), AutomationError> {
    if auth.has_permission("automation.read") {
        return Ok(());
    }

    Err(AutomationError::forbidden("automation.read"))
}

fn execution_scope_key(tenant_id: &str, principal_id: &str, execution_id: &str) -> String {
    format!("{tenant_id}:{principal_id}:{execution_id}")
}

fn execution_matches_request(
    existing: &AutomationExecution,
    request: &RequestAutomationExecution,
) -> bool {
    existing.trigger_type == request.trigger_type
        && existing.target_kind == request.target_kind
        && existing.target_ref == request.target_ref
        && existing.input_payload == request.input_payload
}
