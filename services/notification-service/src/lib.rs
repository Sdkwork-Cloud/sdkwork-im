use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::{Arc, Mutex, MutexGuard};

use axum::extract::{DefaultBodyLimit, Extension, Path, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use craw_chat_api_registry::HttpMethod;
use craw_chat_contract_core::ContractError;
use craw_chat_contract_message::{CommitJournal, CommitPosition};
use craw_chat_contract_notification::{NotificationTaskRecord, NotificationTaskStore};
use craw_chat_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use im_app_context::{
    AppContext, AppContextError, resolve_app_context, resolve_app_context_for_request,
};
pub use im_domain_core::notification::{NotificationStatus, NotificationTask};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_time::utc_now_rfc3339_millis;
use projection_service::{ProjectionAccessError, TimelineProjectionService};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;

const NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_ENV: &str =
    "CRAW_CHAT_NOTIFICATION_MAX_IN_FLIGHT_REQUESTS";
const NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const NOTIFICATION_MAX_REQUEST_BODY_BYTES_ENV: &str =
    "CRAW_CHAT_NOTIFICATION_MAX_REQUEST_BODY_BYTES";
const NOTIFICATION_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const NOTIFICATION_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const NOTIFICATION_REQUIRE_DUAL_TOKEN_HEADERS_ENV: &str =
    "CRAW_CHAT_NOTIFICATION_REQUIRE_DUAL_TOKEN_HEADERS";

#[derive(Clone)]
struct AppState {
    runtime: Arc<NotificationRuntime>,
}

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
    require_dual_token_headers: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestNotification {
    pub notification_id: String,
    pub source_event_id: String,
    pub source_event_type: String,
    pub category: String,
    pub channel: String,
    pub recipient_id: String,
    pub recipient_kind: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationRecipient {
    pub recipient_id: String,
    pub recipient_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestNotificationFanout {
    pub notification_id_seed: String,
    pub source_event_id: String,
    pub source_event_type: String,
    pub category: String,
    pub channel: String,
    pub recipients: BTreeSet<NotificationRecipient>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestAutomationResultNotification {
    pub execution_id: String,
    pub target_ref: String,
    pub output_payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestMessagePostedNotifications {
    pub source_event_id: String,
    pub conversation_id: String,
    pub message_id: String,
    pub message_seq: u64,
    pub message_type: String,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRequestResult {
    pub task: NotificationTask,
    pub is_new: bool,
    pub request_key: String,
    pub delivery_status: NotificationRequestDeliveryStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationRequestDeliveryStatus {
    Accepted,
    Applied,
    Replayed,
    Failed,
}

impl NotificationRequestDeliveryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Applied => "applied",
            Self::Replayed => "replayed",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRequestResponse {
    #[serde(flatten)]
    pub task: NotificationTask,
    pub request_key: String,
    pub delivery_status: NotificationRequestDeliveryStatus,
    pub proof_version: String,
}

impl From<NotificationRequestResult> for NotificationRequestResponse {
    fn from(value: NotificationRequestResult) -> Self {
        Self {
            task: value.task,
            request_key: value.request_key,
            delivery_status: value.delivery_status,
            proof_version: NOTIFICATION_REQUEST_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct NotificationListResponse {
    items: Vec<NotificationTask>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

pub struct NotificationRuntime {
    tasks: Mutex<NotificationRuntimeTaskState>,
    restored_recipients: Mutex<HashSet<String>>,
    journal: Arc<dyn CommitJournal + Send + Sync>,
    task_store: Arc<dyn NotificationTaskStore>,
    projection_service: Arc<TimelineProjectionService>,
}

#[derive(Default)]
struct NotificationRuntimeTaskState {
    tasks: HashMap<String, NotificationTask>,
    tasks_by_recipient: HashMap<String, BTreeSet<String>>,
}

const NOTIFICATION_REQUEST_DELIVERY_PROOF_VERSION: &str = "notification.request.delivery-proof.v1";

#[derive(Default)]
struct NoopJournal;

impl CommitJournal for NoopJournal {
    fn append(&self, _envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        Ok(CommitPosition::new("noop", 0))
    }
}

impl Default for NotificationRuntime {
    fn default() -> Self {
        Self::with_journal(Arc::new(NoopJournal))
    }
}

#[derive(Debug)]
pub struct NotificationError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl NotificationError {
    pub fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    fn not_found(notification_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "notification_not_found",
            message: format!("notification not found: {notification_id}"),
        }
    }

    fn forbidden(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code,
            message: message.into(),
        }
    }

    fn conflict(notification_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "notification_conflict",
            message: format!(
                "notification request conflicts with existing notification idempotency key: {notification_id}"
            ),
        }
    }

    fn payload_too_large(field: &'static str, max_bytes: usize, actual_bytes: usize) -> Self {
        Self {
            status: axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            code: "payload_too_large",
            message: format!(
                "payload too large for {field}: max={max_bytes} bytes, actual={actual_bytes} bytes"
            ),
        }
    }

    fn notification_store(value: ContractError) -> Self {
        match value {
            ContractError::Unavailable(message) => Self {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "notification_store_unavailable",
                message,
            },
            ContractError::Conflict(message) => Self {
                status: axum::http::StatusCode::CONFLICT,
                code: "notification_store_conflict",
                message,
            },
            ContractError::UnsupportedCapability(message) => Self {
                status: axum::http::StatusCode::NOT_IMPLEMENTED,
                code: "notification_store_unsupported",
                message,
            },
        }
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl From<AppContextError> for NotificationError {
    fn from(value: AppContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl From<ContractError> for NotificationError {
    fn from(_value: ContractError) -> Self {
        Self {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "journal_unavailable",
            message: "commit journal unavailable".into(),
        }
    }
}

impl From<ProjectionAccessError> for NotificationError {
    fn from(value: ProjectionAccessError) -> Self {
        Self {
            status: value.status(),
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl axum::response::IntoResponse for NotificationError {
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

impl NotificationRuntime {
    pub fn with_journal<J>(journal: Arc<J>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self::with_journal_and_store_and_projection(
            journal,
            Arc::new(RuntimeMemoryNotificationTaskStore::default()),
            Arc::new(TimelineProjectionService::default()),
        )
    }

    pub fn with_journal_and_store<J, S>(journal: Arc<J>, task_store: Arc<S>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        S: NotificationTaskStore + 'static,
    {
        Self::with_journal_and_store_and_projection(
            journal,
            task_store,
            Arc::new(TimelineProjectionService::default()),
        )
    }

    pub fn with_journal_and_projection<J>(
        journal: Arc<J>,
        projection_service: Arc<TimelineProjectionService>,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
    {
        Self::with_journal_and_store_and_projection(
            journal,
            Arc::new(RuntimeMemoryNotificationTaskStore::default()),
            projection_service,
        )
    }

    pub fn with_journal_and_store_and_projection<J, S>(
        journal: Arc<J>,
        task_store: Arc<S>,
        projection_service: Arc<TimelineProjectionService>,
    ) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        S: NotificationTaskStore + 'static,
    {
        Self {
            tasks: Mutex::new(NotificationRuntimeTaskState::default()),
            restored_recipients: Mutex::new(HashSet::new()),
            journal,
            task_store,
            projection_service,
        }
    }

    fn ensure_notification_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<(), NotificationError> {
        let scope_key = notification_scope_key(tenant_id, notification_id);
        if self
            .tasks
            .lock_notification()
            .tasks
            .contains_key(scope_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .task_store
            .load_task(tenant_id, notification_id)
            .map_err(NotificationError::notification_store)?;
        if let Some(record) = restored {
            let mut state = self.tasks.lock_notification();
            insert_runtime_notification_task(&mut state, scope_key, record.task);
        }

        Ok(())
    }

    fn ensure_recipient_tasks(
        &self,
        tenant_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
    ) -> Result<(), NotificationError> {
        let recipient_key =
            notification_recipient_scope_key(tenant_id, recipient_kind, recipient_id);
        if self
            .restored_recipients
            .lock_notification()
            .contains(recipient_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .task_store
            .list_tasks_for_recipient(tenant_id, recipient_kind, recipient_id)
            .map_err(NotificationError::notification_store)?;
        let mut state = self.tasks.lock_notification();
        for record in restored {
            insert_runtime_notification_task(
                &mut state,
                notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str()),
                record.task,
            );
        }
        drop(state);
        self.restored_recipients
            .lock_notification()
            .insert(recipient_key);

        Ok(())
    }

    pub fn request_notification(
        &self,
        auth: &AppContext,
        request: RequestNotification,
    ) -> Result<NotificationTask, NotificationError> {
        Ok(self.request_notification_with_outcome(auth, request)?.task)
    }

    pub fn request_notification_with_outcome(
        &self,
        auth: &AppContext,
        request: RequestNotification,
    ) -> Result<NotificationRequestResult, NotificationError> {
        validate_notification_request_payload_size(&request)?;
        self.ensure_notification_task(auth.tenant_id.as_str(), request.notification_id.as_str())?;
        let request_key =
            notification_request_key(auth.tenant_id.as_str(), request.notification_id.as_str());
        let notification_key =
            notification_scope_key(auth.tenant_id.as_str(), request.notification_id.as_str());
        let mut state = self.tasks.lock_notification();

        if let Some(existing) = state.tasks.get(notification_key.as_str()).cloned() {
            if notification_matches_request(&existing, &request) {
                let delivery_status = delivery_status_from_notification_status(&existing.status);
                return Ok(NotificationRequestResult {
                    task: existing,
                    is_new: false,
                    request_key,
                    delivery_status,
                });
            }

            return Err(NotificationError::conflict(
                request.notification_id.as_str(),
            ));
        }

        let requested_at = utc_now_rfc3339_millis();
        let dispatched_at = utc_now_rfc3339_millis();

        let requested = NotificationTask {
            tenant_id: auth.tenant_id.clone(),
            notification_id: request.notification_id.clone(),
            source_event_id: request.source_event_id.clone(),
            source_event_type: request.source_event_type.clone(),
            category: request.category.clone(),
            channel: request.channel.clone(),
            recipient_id: request.recipient_id.clone(),
            recipient_kind: request.recipient_kind.clone(),
            status: NotificationStatus::Requested,
            title: request.title.clone(),
            body: request.body.clone(),
            payload: request.payload.clone(),
            requested_at: requested_at.clone(),
            dispatched_at: None,
            failure_reason: None,
        };
        self.append_event(auth, &requested, "notification.requested", 1)?;

        let dispatched = NotificationTask {
            status: NotificationStatus::Dispatched,
            dispatched_at: Some(dispatched_at),
            ..requested
        };
        self.append_event(auth, &dispatched, "notification.dispatched", 2)?;

        insert_runtime_notification_task(&mut state, notification_key.clone(), dispatched.clone());
        if let Err(error) = self.task_store.save_task(self.task_record(&dispatched)) {
            remove_runtime_notification_task(&mut state, notification_key.as_str());
            return Err(NotificationError::notification_store(error));
        }

        Ok(NotificationRequestResult {
            task: dispatched,
            is_new: true,
            request_key,
            delivery_status: NotificationRequestDeliveryStatus::Applied,
        })
    }

    pub fn request_notification_from_app_context(
        &self,
        auth: &AppContext,
        request: RequestNotification,
    ) -> Result<NotificationRequestResult, NotificationError> {
        ensure_notification_request_access(
            auth,
            request.recipient_id.as_str(),
            request.recipient_kind.as_str(),
        )?;
        self.request_notification_with_outcome(auth, request)
    }

    pub fn request_notification_fanout(
        &self,
        auth: &AppContext,
        request: RequestNotificationFanout,
    ) -> Result<Vec<NotificationTask>, NotificationError> {
        let mut tasks = Vec::new();

        for recipient in request.recipients.into_iter().filter(|recipient| {
            recipient.recipient_id != auth.actor_id || recipient.recipient_kind != auth.actor_kind
        }) {
            tasks.push(self.request_notification(
                auth,
                RequestNotification {
                    notification_id: fanout_notification_id(
                        request.notification_id_seed.as_str(),
                        &recipient,
                    ),
                    source_event_id: request.source_event_id.clone(),
                    source_event_type: request.source_event_type.clone(),
                    category: request.category.clone(),
                    channel: request.channel.clone(),
                    recipient_id: recipient.recipient_id,
                    recipient_kind: recipient.recipient_kind,
                    title: request.title.clone(),
                    body: request.body.clone(),
                    payload: request.payload.clone(),
                },
            )?);
        }

        Ok(tasks)
    }

    pub fn request_message_posted_notifications(
        &self,
        auth: &AppContext,
        request: RequestMessagePostedNotifications,
    ) -> Result<Vec<NotificationTask>, NotificationError> {
        let RequestMessagePostedNotifications {
            source_event_id,
            conversation_id,
            message_id,
            message_seq,
            message_type,
            summary,
        } = request;
        let category = if message_type == "signal" {
            "rtc.event"
        } else {
            "message.new"
        };
        let recipients = self
            .projection_service
            .message_posted_notification_recipients_from_auth_context(
                auth,
                conversation_id.as_str(),
            )?
            .into_iter()
            .map(|recipient| NotificationRecipient {
                recipient_id: recipient.principal_id,
                recipient_kind: recipient.principal_kind,
            })
            .collect::<BTreeSet<_>>();
        let notification_id_seed = message_id.clone();
        let payload = serde_json::json!({
            "conversationId": conversation_id,
            "messageId": message_id,
            "messageSeq": message_seq,
            "messageType": message_type,
        })
        .to_string();

        self.request_notification_fanout(
            auth,
            RequestNotificationFanout {
                notification_id_seed,
                source_event_id,
                source_event_type: "message.posted".into(),
                category: category.into(),
                channel: "inapp".into(),
                recipients,
                title: summary.clone(),
                body: summary,
                payload: Some(payload),
            },
        )
    }

    pub fn request_automation_result_notification(
        &self,
        auth: &AppContext,
        request: RequestAutomationResultNotification,
    ) -> Result<NotificationTask, NotificationError> {
        self.request_notification(
            auth,
            RequestNotification {
                notification_id: automation_notification_id(
                    auth.actor_kind.as_str(),
                    request.execution_id.as_str(),
                ),
                source_event_id: automation_notification_source_event_id(
                    auth.actor_kind.as_str(),
                    request.execution_id.as_str(),
                ),
                source_event_type: "automation.execution_completed".into(),
                category: "automation.result".into(),
                channel: "inapp".into(),
                recipient_id: auth.actor_id.clone(),
                recipient_kind: auth.actor_kind.clone(),
                title: Some("Automation completed".into()),
                body: Some(request.target_ref),
                payload: request.output_payload,
            },
        )
    }

    pub fn list_notifications(
        &self,
        auth: &AppContext,
    ) -> Result<Vec<NotificationTask>, NotificationError> {
        self.ensure_recipient_tasks(
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
        )?;
        let state = self.tasks.lock_notification();
        let mut items: Vec<_> = notification_keys_for_recipient(
            &state,
            auth.tenant_id.as_str(),
            auth.actor_kind.as_str(),
            auth.actor_id.as_str(),
        )
        .into_iter()
        .filter_map(|task_key| state.tasks.get(task_key.as_str()).cloned())
        .collect();
        items.sort_by(|left, right| {
            notification_sort_key(right)
                .cmp(&notification_sort_key(left))
                .then_with(|| right.notification_id.cmp(&left.notification_id))
        });
        Ok(items)
    }

    pub fn get_notification(
        &self,
        auth: &AppContext,
        notification_id: &str,
    ) -> Result<NotificationTask, NotificationError> {
        self.ensure_notification_task(auth.tenant_id.as_str(), notification_id)?;
        self.tasks
            .lock_notification()
            .tasks
            .get(notification_scope_key(auth.tenant_id.as_str(), notification_id).as_str())
            .filter(|task| notification_visible_to_actor(task, auth))
            .cloned()
            .ok_or_else(|| NotificationError::not_found(notification_id))
    }

    fn task_record(&self, task: &NotificationTask) -> NotificationTaskRecord {
        NotificationTaskRecord {
            tenant_id: task.tenant_id.clone(),
            notification_id: task.notification_id.clone(),
            task: task.clone(),
            updated_at: utc_now_rfc3339_millis(),
        }
    }

    fn append_event(
        &self,
        auth: &AppContext,
        task: &NotificationTask,
        event_type: &str,
        ordering_seq: u64,
    ) -> Result<(), NotificationError> {
        let committed_at = task
            .dispatched_at
            .clone()
            .unwrap_or_else(|| task.requested_at.clone());
        let envelope = CommitEnvelope {
            event_id: format!(
                "evt_{}_{}",
                task.notification_id,
                event_type.replace('.', "_")
            ),
            tenant_id: auth.tenant_id.clone(),
            event_type: event_type.into(),
            event_version: 1,
            aggregate_type: AggregateType::Notification,
            aggregate_id: task.notification_id.clone(),
            scope_type: "notification".into(),
            scope_id: task.notification_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                auth.tenant_id.as_str(),
                task.notification_id.as_str(),
            ),
            ordering_seq,
            causation_id: Some(task.source_event_id.clone()),
            correlation_id: Some(task.source_event_id.clone()),
            idempotency_key: Some(format!(
                "{}:{}:{}",
                task.notification_id, event_type, ordering_seq
            )),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: task.requested_at.clone(),
            committed_at,
            payload_schema: Some("notification.task.v1".into()),
            payload: serde_json::to_string(task).map_err(|error| NotificationError {
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "notification_payload_invalid",
                message: format!(
                    "failed to serialize notification task into commit envelope: {error}"
                ),
            })?,
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        self.journal.append(envelope)?;
        Ok(())
    }
}

#[derive(Clone, Default)]
struct RuntimeMemoryNotificationTaskStore {
    state: Arc<Mutex<RuntimeMemoryNotificationTaskState>>,
}

#[derive(Default)]
struct RuntimeMemoryNotificationTaskState {
    tasks: HashMap<String, NotificationTaskRecord>,
    tasks_by_recipient: HashMap<String, BTreeSet<String>>,
}

impl NotificationTaskStore for RuntimeMemoryNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        Ok(self
            .state
            .lock_notification()
            .tasks
            .get(notification_scope_key(tenant_id, notification_id).as_str())
            .cloned())
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        let notification_key =
            notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str());
        let mut state = self.state.lock_notification();
        if let Some(previous) = state.tasks.get(notification_key.as_str()).cloned() {
            remove_notification_recipient_index(
                &mut state.tasks_by_recipient,
                notification_key.as_str(),
                &previous,
            );
            let merged = previous.merge_monotonic(record);
            insert_notification_recipient_index(
                &mut state.tasks_by_recipient,
                notification_key.as_str(),
                &merged,
            );
            state.tasks.insert(notification_key, merged);
            return Ok(());
        }
        insert_notification_recipient_index(
            &mut state.tasks_by_recipient,
            notification_key.as_str(),
            &record,
        );
        state.tasks.insert(notification_key, record);
        Ok(())
    }

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        let state = self.state.lock_notification();
        let task_keys = state
            .tasks_by_recipient
            .get(notification_recipient_scope_key(tenant_id, recipient_kind, recipient_id).as_str())
            .cloned()
            .unwrap_or_default();
        Ok(task_keys
            .into_iter()
            .filter_map(|task_key| state.tasks.get(task_key.as_str()).cloned())
            .collect())
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(NotificationRuntime::default()))
}

pub fn build_public_app() -> Router {
    let guardrails = PublicAppGuardrails {
        request_gate: Arc::new(Semaphore::new(resolve_max_in_flight_requests())),
        require_dual_token_headers: resolve_require_dual_token_headers(),
    };
    build_default_app()
        .layer(DefaultBodyLimit::max(resolve_max_http_request_body_bytes()))
        .layer(middleware::from_fn_with_state(
            guardrails,
            require_app_context,
        ))
}

pub fn build_app(runtime: Arc<NotificationRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .route(
            "/app/v3/api/notifications/requests",
            post(request_notification),
        )
        .route("/app/v3/api/notifications", get(list_notifications))
        .route(
            "/app/v3/api/notifications/{notification_id}",
            get(get_notification),
        )
        .with_state(AppState { runtime })
}

async fn require_app_context(
    State(guardrails): State<PublicAppGuardrails>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" | "/openapi.json" | "/docs" => next.run(request).await,
        _ => {
            let permit = match guardrails.request_gate.clone().try_acquire_owned() {
                Ok(permit) => permit,
                Err(_) => {
                    return NotificationError {
                        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        code: "http_overloaded",
                        message:
                            "server is at maximum in-flight request capacity, please retry later"
                                .to_owned(),
                    }
                    .into_response();
                }
            };
            if guardrails.require_dual_token_headers
                && let Err(error) = require_dual_token_headers(request.headers())
            {
                return error.into_response();
            }
            let resolved = match resolve_app_context_for_request(
                request.headers(),
                request.uri().path(),
                request.method().as_str(),
            ) {
                Ok(resolved) => resolved,
                Err(error) => return NotificationError::from(error).into_response(),
            };
            request
                .extensions_mut()
                .insert(resolved.app_request_context);
            request.extensions_mut().insert(resolved.app_context);
            let response = next.run(request).await;
            drop(permit);
            response
        }
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "notification-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "notification-service",
    })
}

async fn openapi_json() -> Result<Json<serde_json::Value>, NotificationError> {
    Ok(Json(
        build_notification_service_openapi_document()
            .map_err(|message| NotificationError::internal("openapi_export_failed", message))?,
    ))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&notification_service_openapi_spec()))
}

fn build_notification_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_app",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &notification_service_openapi_spec(),
        &routes,
        notification_service_tag,
        notification_service_requires_app_context,
        notification_service_summary,
    ))
}

fn notification_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Craw Chat Notification Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the notification-service router for notification request mutation and notification query flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn notification_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        _ => "notifications".to_owned(),
    }
}

fn notification_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn notification_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check notification service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check notification service readiness".to_owned(),
        _ => format!(
            "{} {}",
            notification_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn notification_service_method_display(method: HttpMethod) -> &'static str {
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

async fn request_notification(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestNotification>,
) -> Result<Json<NotificationRequestResponse>, NotificationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(
        state
            .runtime
            .request_notification_from_app_context(&auth, request)?
            .into(),
    ))
}

async fn list_notifications(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<NotificationListResponse>, NotificationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(NotificationListResponse {
        items: state.runtime.list_notifications(&auth)?,
    }))
}

async fn get_notification(
    Path(notification_id): Path<String>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<NotificationTask>, NotificationError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    Ok(Json(
        state
            .runtime
            .get_notification(&auth, notification_id.as_str())?,
    ))
}

fn notification_scope_key(tenant_id: &str, notification_id: &str) -> String {
    scope_key_parts(&[tenant_id, notification_id])
}

fn notification_recipient_scope_key(
    tenant_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
) -> String {
    scope_key_parts(&[tenant_id, recipient_kind, recipient_id])
}

fn scope_key_parts(parts: &[&str]) -> String {
    parts
        .iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

fn record_notification_recipient_scope_key(record: &NotificationTaskRecord) -> String {
    notification_recipient_scope_key(
        record.tenant_id.as_str(),
        record.task.recipient_kind.as_str(),
        record.task.recipient_id.as_str(),
    )
}

fn insert_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    index
        .entry(record_notification_recipient_scope_key(record))
        .or_default()
        .insert(notification_key.to_owned());
}

fn remove_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    record: &NotificationTaskRecord,
) {
    let recipient_key = record_notification_recipient_scope_key(record);
    if let Some(task_keys) = index.get_mut(recipient_key.as_str()) {
        task_keys.remove(notification_key);
        if task_keys.is_empty() {
            index.remove(recipient_key.as_str());
        }
    }
}

fn runtime_notification_recipient_scope_key(task: &NotificationTask) -> String {
    notification_recipient_scope_key(
        task.tenant_id.as_str(),
        task.recipient_kind.as_str(),
        task.recipient_id.as_str(),
    )
}

fn insert_runtime_notification_task(
    state: &mut NotificationRuntimeTaskState,
    notification_key: String,
    task: NotificationTask,
) {
    if let Some(previous) = state.tasks.get(notification_key.as_str()).cloned() {
        remove_runtime_notification_recipient_index(
            &mut state.tasks_by_recipient,
            notification_key.as_str(),
            &previous,
        );
    }
    insert_runtime_notification_recipient_index(
        &mut state.tasks_by_recipient,
        notification_key.as_str(),
        &task,
    );
    state.tasks.insert(notification_key, task);
}

fn remove_runtime_notification_task(
    state: &mut NotificationRuntimeTaskState,
    notification_key: &str,
) -> Option<NotificationTask> {
    let removed = state.tasks.remove(notification_key)?;
    remove_runtime_notification_recipient_index(
        &mut state.tasks_by_recipient,
        notification_key,
        &removed,
    );
    Some(removed)
}

fn insert_runtime_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    task: &NotificationTask,
) {
    index
        .entry(runtime_notification_recipient_scope_key(task))
        .or_default()
        .insert(notification_key.to_owned());
}

fn remove_runtime_notification_recipient_index(
    index: &mut HashMap<String, BTreeSet<String>>,
    notification_key: &str,
    task: &NotificationTask,
) {
    let recipient_key = runtime_notification_recipient_scope_key(task);
    if let Some(task_keys) = index.get_mut(recipient_key.as_str()) {
        task_keys.remove(notification_key);
        if task_keys.is_empty() {
            index.remove(recipient_key.as_str());
        }
    }
}

fn notification_keys_for_recipient(
    state: &NotificationRuntimeTaskState,
    tenant_id: &str,
    recipient_kind: &str,
    recipient_id: &str,
) -> BTreeSet<String> {
    state
        .tasks_by_recipient
        .get(notification_recipient_scope_key(tenant_id, recipient_kind, recipient_id).as_str())
        .cloned()
        .unwrap_or_default()
}

fn notification_request_key(tenant_id: &str, notification_id: &str) -> String {
    notification_scope_key(tenant_id, notification_id)
}

fn notification_sort_key(task: &NotificationTask) -> (&str, &str) {
    (
        task.dispatched_at
            .as_deref()
            .unwrap_or(task.requested_at.as_str()),
        task.requested_at.as_str(),
    )
}

fn delivery_status_from_notification_status(
    status: &NotificationStatus,
) -> NotificationRequestDeliveryStatus {
    match status {
        NotificationStatus::Requested => NotificationRequestDeliveryStatus::Accepted,
        NotificationStatus::Dispatched => NotificationRequestDeliveryStatus::Replayed,
        NotificationStatus::Failed => NotificationRequestDeliveryStatus::Failed,
    }
}

fn notification_matches_request(task: &NotificationTask, request: &RequestNotification) -> bool {
    task.notification_id == request.notification_id.as_str()
        && task.source_event_id == request.source_event_id.as_str()
        && task.source_event_type == request.source_event_type.as_str()
        && task.category == request.category.as_str()
        && task.channel == request.channel.as_str()
        && task.recipient_id == request.recipient_id.as_str()
        && task.recipient_kind == request.recipient_kind
        && task.title.as_ref() == request.title.as_ref()
        && task.body.as_ref() == request.body.as_ref()
        && task.payload.as_ref() == request.payload.as_ref()
}

fn ensure_notification_request_access(
    auth: &AppContext,
    recipient_id: &str,
    recipient_kind: &str,
) -> Result<(), NotificationError> {
    if (recipient_id == auth.actor_id && recipient_kind == auth.actor_kind.as_str())
        || auth.has_permission("notification.write")
    {
        return Ok(());
    }

    Err(NotificationError::forbidden(
        "permission_denied",
        "missing required permission to request notifications for other recipients: notification.write",
    ))
}

fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, NotificationError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(NotificationError::from),
    }
}

fn require_dual_token_headers(headers: &HeaderMap) -> Result<(), NotificationError> {
    if !has_bearer_auth_token(headers) {
        return Err(NotificationError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "auth_token_missing",
            message: "authorization header must provide a bearer token".to_owned(),
        });
    }
    if !has_access_token_header(headers) {
        return Err(NotificationError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "access_token_missing",
            message: "access-token header is required".to_owned(),
        });
    }
    Ok(())
}

fn has_bearer_auth_token(headers: &HeaderMap) -> bool {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .and_then(|value| {
            let (scheme, token) = value.split_once(' ')?;
            if scheme.eq_ignore_ascii_case("bearer") && !token.trim().is_empty() {
                return Some(());
            }
            None
        })
        .is_some()
}

fn has_access_token_header(headers: &HeaderMap) -> bool {
    headers
        .get("access-token")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .is_some_and(|value| !value.is_empty())
}

fn resolve_max_in_flight_requests() -> usize {
    std::env::var(NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(NOTIFICATION_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(NOTIFICATION_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(NOTIFICATION_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(NOTIFICATION_MAX_REQUEST_BODY_BYTES_MAX)
}

fn resolve_require_dual_token_headers() -> bool {
    std::env::var(NOTIFICATION_REQUIRE_DUAL_TOKEN_HEADERS_ENV)
        .ok()
        .map(|value| parse_truthy_env_flag(Some(value)))
        .unwrap_or(true)
}

fn parse_truthy_env_flag(raw: Option<String>) -> bool {
    raw.as_deref().map(str::trim).is_some_and(|value| {
        matches!(
            value.to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    })
}

const NOTIFICATION_MAX_TITLE_BYTES: usize = 8 * 1024;
const NOTIFICATION_MAX_BODY_BYTES: usize = 64 * 1024;
const NOTIFICATION_MAX_PAYLOAD_BYTES: usize = 256 * 1024;
const NOTIFICATION_MAX_NOTIFICATION_ID_BYTES: usize = 512;
const NOTIFICATION_MAX_SOURCE_EVENT_ID_BYTES: usize = 512;
const NOTIFICATION_MAX_SOURCE_EVENT_TYPE_BYTES: usize = 128;
const NOTIFICATION_MAX_CATEGORY_BYTES: usize = 128;
const NOTIFICATION_MAX_CHANNEL_BYTES: usize = 64;
const NOTIFICATION_MAX_RECIPIENT_ID_BYTES: usize = 256;
const NOTIFICATION_MAX_RECIPIENT_KIND_BYTES: usize = 64;

fn validate_payload_size(
    field: &'static str,
    payload: &str,
    max_bytes: usize,
) -> Result<(), NotificationError> {
    let payload_len = payload.len();
    if payload_len > max_bytes {
        return Err(NotificationError::payload_too_large(
            field,
            max_bytes,
            payload_len,
        ));
    }
    Ok(())
}

fn validate_notification_request_payload_size(
    request: &RequestNotification,
) -> Result<(), NotificationError> {
    validate_payload_size(
        "notificationId",
        request.notification_id.as_str(),
        NOTIFICATION_MAX_NOTIFICATION_ID_BYTES,
    )?;
    validate_payload_size(
        "sourceEventId",
        request.source_event_id.as_str(),
        NOTIFICATION_MAX_SOURCE_EVENT_ID_BYTES,
    )?;
    validate_payload_size(
        "sourceEventType",
        request.source_event_type.as_str(),
        NOTIFICATION_MAX_SOURCE_EVENT_TYPE_BYTES,
    )?;
    validate_payload_size(
        "category",
        request.category.as_str(),
        NOTIFICATION_MAX_CATEGORY_BYTES,
    )?;
    validate_payload_size(
        "channel",
        request.channel.as_str(),
        NOTIFICATION_MAX_CHANNEL_BYTES,
    )?;
    validate_payload_size(
        "recipientId",
        request.recipient_id.as_str(),
        NOTIFICATION_MAX_RECIPIENT_ID_BYTES,
    )?;
    validate_payload_size(
        "recipientKind",
        request.recipient_kind.as_str(),
        NOTIFICATION_MAX_RECIPIENT_KIND_BYTES,
    )?;
    if let Some(title) = request.title.as_deref() {
        validate_payload_size("title", title, NOTIFICATION_MAX_TITLE_BYTES)?;
    }
    if let Some(body) = request.body.as_deref() {
        validate_payload_size("body", body, NOTIFICATION_MAX_BODY_BYTES)?;
    }
    if let Some(payload) = request.payload.as_deref() {
        validate_payload_size("payload", payload, NOTIFICATION_MAX_PAYLOAD_BYTES)?;
    }
    Ok(())
}

fn notification_visible_to_actor(task: &NotificationTask, auth: &AppContext) -> bool {
    task.recipient_id == auth.actor_id && task.recipient_kind == auth.actor_kind
}

fn fanout_notification_id(notification_id_seed: &str, recipient: &NotificationRecipient) -> String {
    format!(
        "ntf_{}_{}_{}",
        notification_id_seed, recipient.recipient_kind, recipient.recipient_id
    )
}

fn automation_notification_id(actor_kind: &str, execution_id: &str) -> String {
    format!("ntf_automation_{actor_kind}_{execution_id}")
}

fn automation_notification_source_event_id(actor_kind: &str, execution_id: &str) -> String {
    format!("evt_{actor_kind}_{execution_id}_automation_execution_completed")
}

trait NotificationMutexExt<T> {
    fn lock_notification(&self) -> MutexGuard<'_, T>;
}

impl<T> NotificationMutexExt<T> for Mutex<T> {
    fn lock_notification(&self) -> MutexGuard<'_, T> {
        match self.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!("recovering poisoned mutex in notification-service");
                poisoned.into_inner()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue};

    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    fn notification_task_record(
        notification_id: &str,
        recipient_kind: &str,
        recipient_id: &str,
        status: NotificationStatus,
        dispatched_at: Option<&str>,
        failure_reason: Option<&str>,
        updated_at: &str,
    ) -> NotificationTaskRecord {
        NotificationTaskRecord {
            tenant_id: "t_demo".into(),
            notification_id: notification_id.into(),
            task: NotificationTask {
                tenant_id: "t_demo".into(),
                notification_id: notification_id.into(),
                source_event_id: format!("evt_{notification_id}"),
                source_event_type: "message.posted".into(),
                category: "message.new".into(),
                channel: "inapp".into(),
                recipient_id: recipient_id.into(),
                recipient_kind: recipient_kind.to_owned(),
                status,
                title: Some("hello".into()),
                body: Some("world".into()),
                payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                requested_at: "2026-05-06T00:00:00.000Z".into(),
                dispatched_at: dispatched_at.map(str::to_owned),
                failure_reason: failure_reason.map(str::to_owned),
            },
            updated_at: updated_at.into(),
        }
    }

    fn demo_auth_context() -> AppContext {
        AppContext {
            tenant_id: "t_demo".into(),
            organization_id: None,
            user_id: "u_demo".into(),
            actor_id: "u_demo".into(),
            actor_kind: "user".into(),
            session_id: Some("s_demo".into()),
            app_id: None,
            environment: None,
            deployment_mode: None,
            auth_level: None,
            data_scope: Default::default(),
            permission_scope: Default::default(),
            device_id: Some("d_demo".into()),
        }
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
    }

    #[test]
    fn test_notification_runtime_uses_recipient_index_for_listing() {
        let source = include_str!("lib.rs").replace("\r\n", "\n");

        assert!(
            source.contains("tasks_by_recipient: HashMap<String, BTreeSet<String>>"),
            "notification runtime should maintain a tenant/recipient-kind/recipient-id task index"
        );
        assert!(
            source.contains("notification_keys_for_recipient("),
            "list_notifications should read notification keys from the runtime recipient index"
        );
        assert!(
            !source.contains(".iter()\n            .filter(|(key, task)| {\n                key.starts_with(prefix.as_str()) && notification_visible_to_actor(task, auth)\n            })"),
            "list_notifications must not full-scan the runtime task cache"
        );
    }

    #[test]
    fn test_list_notifications_recovers_from_poisoned_tasks_lock() {
        let runtime = NotificationRuntime::default();
        let auth = demo_auth_context();
        poison_mutex(&runtime.tasks);

        let result = panic::catch_unwind(AssertUnwindSafe(|| runtime.list_notifications(&auth)));
        assert!(
            result.is_ok(),
            "list_notifications should not panic when tasks lock is poisoned"
        );
        let list_result = result.expect("panic status should be captured");
        assert!(
            list_result.is_ok(),
            "list_notifications should recover from poisoned tasks lock"
        );
    }

    #[test]
    fn test_get_notification_recovers_from_poisoned_tasks_lock() {
        let runtime = NotificationRuntime::default();
        let auth = demo_auth_context();
        poison_mutex(&runtime.tasks);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            runtime.get_notification(&auth, "ntf_missing")
        }));
        assert!(
            result.is_ok(),
            "get_notification should not panic when tasks lock is poisoned"
        );
        let get_result = result.expect("panic status should be captured");
        assert!(
            get_result.is_err(),
            "get_notification should return controlled error after lock recovery"
        );
    }

    #[test]
    fn test_list_notifications_recovers_from_poisoned_restored_recipients_lock() {
        let runtime = NotificationRuntime::default();
        let auth = demo_auth_context();
        poison_mutex(&runtime.restored_recipients);

        let result = panic::catch_unwind(AssertUnwindSafe(|| runtime.list_notifications(&auth)));
        assert!(
            result.is_ok(),
            "list_notifications should not panic when restored-recipient lock is poisoned"
        );
        let list_result = result.expect("panic status should be captured");
        assert!(
            list_result.is_ok(),
            "list_notifications should recover from poisoned restored-recipient lock"
        );
    }

    #[test]
    fn test_runtime_memory_task_store_load_recovers_from_poisoned_lock() {
        let store = RuntimeMemoryNotificationTaskStore::default();
        poison_mutex(&store.state);

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            store.load_task("t_demo", "ntf_poison_store")
        }));
        assert!(
            result.is_ok(),
            "notification task store load should not panic when lock is poisoned"
        );
        let load_result = result.expect("panic status should be captured");
        assert!(
            load_result.is_ok(),
            "notification task store load should recover from poisoned lock"
        );
    }

    #[test]
    fn test_runtime_memory_task_store_uses_recipient_kind_index_for_listing() {
        let source = include_str!("lib.rs").replace("\r\n", "\n");

        assert!(
            source.contains("tasks_by_recipient: HashMap<String, BTreeSet<String>>"),
            "runtime notification task store should maintain a tenant/recipient-kind/recipient-id index"
        );
        assert!(
            source.contains("notification_recipient_scope_key("),
            "runtime notification task store should include recipient_kind in its index key"
        );
    }

    #[test]
    fn test_runtime_memory_task_store_lists_only_matching_recipient_kind() {
        let store = RuntimeMemoryNotificationTaskStore::default();
        store
            .save_task(notification_task_record(
                "ntf_user",
                "user",
                "shared_id",
                NotificationStatus::Dispatched,
                Some("2026-05-06T00:00:02.000Z"),
                None,
                "2026-05-06T00:00:02.000Z",
            ))
            .expect("user notification save should succeed");
        store
            .save_task(notification_task_record(
                "ntf_system",
                "system",
                "shared_id",
                NotificationStatus::Dispatched,
                Some("2026-05-06T00:00:03.000Z"),
                None,
                "2026-05-06T00:00:03.000Z",
            ))
            .expect("system notification save should succeed");

        let listed = store
            .list_tasks_for_recipient("t_demo", "user", "shared_id")
            .expect("recipient listing should succeed");

        assert_eq!(
            listed
                .iter()
                .map(|record| record.notification_id.as_str())
                .collect::<Vec<_>>(),
            vec!["ntf_user"]
        );
    }

    #[test]
    fn test_runtime_memory_task_store_rejects_stale_status_regression_writes() {
        let store = RuntimeMemoryNotificationTaskStore::default();
        store
            .save_task(notification_task_record(
                "ntf_demo",
                "user",
                "u_demo",
                NotificationStatus::Dispatched,
                Some("2026-05-06T00:00:02.000Z"),
                None,
                "2026-05-06T00:00:02.000Z",
            ))
            .expect("current notification save should succeed");
        store
            .save_task(notification_task_record(
                "ntf_demo",
                "user",
                "u_demo",
                NotificationStatus::Requested,
                None,
                None,
                "2026-05-06T00:00:01.000Z",
            ))
            .expect("stale notification save should not fail the caller");

        let restored = store
            .load_task("t_demo", "ntf_demo")
            .expect("notification load should succeed")
            .expect("notification should be present");
        assert_eq!(restored.task.status, NotificationStatus::Dispatched);
        assert_eq!(
            restored.task.dispatched_at.as_deref(),
            Some("2026-05-06T00:00:02.000Z")
        );
        assert_eq!(restored.updated_at, "2026-05-06T00:00:02.000Z");
    }

    #[test]
    fn parse_truthy_env_flag_accepts_common_truthy_values() {
        for value in ["1", "true", "TRUE", " yes ", "On"] {
            assert!(parse_truthy_env_flag(Some(value.to_owned())));
        }
        for value in ["0", "false", "off", "no", "", "  "] {
            assert!(!parse_truthy_env_flag(Some(value.to_owned())));
        }
        assert!(!parse_truthy_env_flag(None));
    }

    #[test]
    fn dual_token_header_helpers_validate_auth_and_access_headers() {
        let mut headers = HeaderMap::new();
        assert!(!has_bearer_auth_token(&headers));
        assert!(!has_access_token_header(&headers));

        headers.insert(
            axum::http::header::AUTHORIZATION,
            HeaderValue::from_static("Bearer auth_token"),
        );
        headers.insert("access-token", HeaderValue::from_static("access_token"));
        assert!(has_bearer_auth_token(&headers));
        assert!(has_access_token_header(&headers));
    }
}
