use std::collections::{HashMap, HashSet};
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
pub use im_domain_core::notification::{NotificationStatus, NotificationTask};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_platform_contracts::{
    CommitJournal, CommitPosition, ContractError, NotificationTaskRecord, NotificationTaskStore,
};
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
struct AppState {
    runtime: Arc<NotificationRuntime>,
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
    pub title: Option<String>,
    pub body: Option<String>,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRequestResult {
    pub task: NotificationTask,
    pub is_new: bool,
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
    tasks: Mutex<HashMap<String, NotificationTask>>,
    restored_recipients: Mutex<HashSet<String>>,
    journal: Arc<dyn CommitJournal + Send + Sync>,
    task_store: Arc<dyn NotificationTaskStore>,
}

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
}

impl From<AuthContextError> for NotificationError {
    fn from(value: AuthContextError) -> Self {
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
        Self::with_journal_and_store(
            journal,
            Arc::new(RuntimeMemoryNotificationTaskStore::default()),
        )
    }

    pub fn with_journal_and_store<J, S>(journal: Arc<J>, task_store: Arc<S>) -> Self
    where
        J: CommitJournal + Send + Sync + 'static,
        S: NotificationTaskStore + 'static,
    {
        Self {
            tasks: Mutex::new(HashMap::new()),
            restored_recipients: Mutex::new(HashSet::new()),
            journal,
            task_store,
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
            .lock()
            .expect("notification runtime should lock")
            .contains_key(scope_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .task_store
            .load_task(tenant_id, notification_id)
            .map_err(NotificationError::notification_store)?;
        if let Some(record) = restored {
            self.tasks
                .lock()
                .expect("notification runtime should lock")
                .insert(scope_key, record.task);
        }

        Ok(())
    }

    fn ensure_recipient_tasks(
        &self,
        tenant_id: &str,
        recipient_id: &str,
    ) -> Result<(), NotificationError> {
        let recipient_key = recipient_scope_key(tenant_id, recipient_id);
        if self
            .restored_recipients
            .lock()
            .expect("notification runtime should lock")
            .contains(recipient_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .task_store
            .list_tasks_for_recipient(tenant_id, recipient_id)
            .map_err(NotificationError::notification_store)?;
        let mut tasks = self.tasks.lock().expect("notification runtime should lock");
        for record in restored {
            tasks.insert(
                notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str()),
                record.task,
            );
        }
        drop(tasks);
        self.restored_recipients
            .lock()
            .expect("notification runtime should lock")
            .insert(recipient_key);

        Ok(())
    }

    pub fn request_notification(
        &self,
        auth: &AuthContext,
        request: RequestNotification,
    ) -> Result<NotificationTask, NotificationError> {
        Ok(self.request_notification_with_outcome(auth, request)?.task)
    }

    pub fn request_notification_with_outcome(
        &self,
        auth: &AuthContext,
        request: RequestNotification,
    ) -> Result<NotificationRequestResult, NotificationError> {
        self.ensure_notification_task(auth.tenant_id.as_str(), request.notification_id.as_str())?;
        let notification_key =
            notification_scope_key(auth.tenant_id.as_str(), request.notification_id.as_str());
        let mut tasks = self.tasks.lock().expect("notification runtime should lock");

        if let Some(existing) = tasks.get(notification_key.as_str()).cloned() {
            if notification_matches_request(&existing, &request) {
                return Ok(NotificationRequestResult {
                    task: existing,
                    is_new: false,
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

        tasks.insert(notification_key.clone(), dispatched.clone());
        if let Err(error) = self.task_store.save_task(self.task_record(&dispatched)) {
            tasks.remove(notification_key.as_str());
            return Err(NotificationError::notification_store(error));
        }

        Ok(NotificationRequestResult {
            task: dispatched,
            is_new: true,
        })
    }

    pub fn list_notifications(
        &self,
        auth: &AuthContext,
    ) -> Result<Vec<NotificationTask>, NotificationError> {
        self.ensure_recipient_tasks(auth.tenant_id.as_str(), auth.actor_id.as_str())?;
        let prefix = format!("{}:", auth.tenant_id);
        let mut items: Vec<_> = self
            .tasks
            .lock()
            .expect("notification runtime should lock")
            .iter()
            .filter(|(key, task)| {
                key.starts_with(prefix.as_str()) && task.recipient_id == auth.actor_id
            })
            .map(|(_, task)| task.clone())
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
        auth: &AuthContext,
        notification_id: &str,
    ) -> Result<NotificationTask, NotificationError> {
        self.ensure_notification_task(auth.tenant_id.as_str(), notification_id)?;
        self.tasks
            .lock()
            .expect("notification runtime should lock")
            .get(notification_scope_key(auth.tenant_id.as_str(), notification_id).as_str())
            .filter(|task| task.recipient_id == auth.actor_id)
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
        auth: &AuthContext,
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
            idempotency_key: Some(task.notification_id.clone()),
            actor: EventActor {
                actor_id: auth.actor_id.clone(),
                actor_kind: auth.actor_kind.clone(),
                actor_session_id: auth.session_id.clone(),
            },
            occurred_at: task.requested_at.clone(),
            committed_at,
            payload_schema: Some("notification.task.v1".into()),
            payload: serde_json::to_string(task)
                .expect("notification task should serialize into commit envelope"),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };
        self.journal.append(envelope)?;
        Ok(())
    }
}

#[derive(Clone, Default)]
struct RuntimeMemoryNotificationTaskStore {
    tasks: Arc<Mutex<HashMap<String, NotificationTaskRecord>>>,
}

impl NotificationTaskStore for RuntimeMemoryNotificationTaskStore {
    fn load_task(
        &self,
        tenant_id: &str,
        notification_id: &str,
    ) -> Result<Option<NotificationTaskRecord>, ContractError> {
        Ok(self
            .tasks
            .lock()
            .expect("notification task store should lock")
            .get(notification_scope_key(tenant_id, notification_id).as_str())
            .cloned())
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        self.tasks
            .lock()
            .expect("notification task store should lock")
            .insert(
                notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str()),
                record,
            );
        Ok(())
    }

    fn list_tasks_for_recipient(
        &self,
        tenant_id: &str,
        recipient_id: &str,
    ) -> Result<Vec<NotificationTaskRecord>, ContractError> {
        Ok(self
            .tasks
            .lock()
            .expect("notification task store should lock")
            .values()
            .filter(|record| {
                record.tenant_id == tenant_id && record.task.recipient_id == recipient_id
            })
            .cloned()
            .collect())
    }
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(NotificationRuntime::default()))
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(runtime: Arc<NotificationRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/notifications/requests", post(request_notification))
        .route("/api/v1/notifications", get(list_notifications))
        .route(
            "/api/v1/notifications/{notification_id}",
            get(get_notification),
        )
        .with_state(AppState { runtime })
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => NotificationError::from(error).into_response(),
        },
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

async fn request_notification(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RequestNotification>,
) -> Result<Json<NotificationTask>, NotificationError> {
    let is_bearer_request = headers.contains_key(axum::http::header::AUTHORIZATION);
    let auth = resolve_auth_context(&headers)?;
    ensure_notification_request_access(&auth, request.recipient_id.as_str(), is_bearer_request)?;
    Ok(Json(state.runtime.request_notification(&auth, request)?))
}

async fn list_notifications(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<NotificationListResponse>, NotificationError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(NotificationListResponse {
        items: state.runtime.list_notifications(&auth)?,
    }))
}

async fn get_notification(
    Path(notification_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<NotificationTask>, NotificationError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .runtime
            .get_notification(&auth, notification_id.as_str())?,
    ))
}

fn notification_scope_key(tenant_id: &str, notification_id: &str) -> String {
    format!("{tenant_id}:{notification_id}")
}

fn recipient_scope_key(tenant_id: &str, recipient_id: &str) -> String {
    format!("{tenant_id}:{recipient_id}")
}

fn notification_sort_key(task: &NotificationTask) -> (&str, &str) {
    (
        task.dispatched_at
            .as_deref()
            .unwrap_or(task.requested_at.as_str()),
        task.requested_at.as_str(),
    )
}

fn notification_matches_request(task: &NotificationTask, request: &RequestNotification) -> bool {
    task.notification_id == request.notification_id.as_str()
        && task.source_event_id == request.source_event_id.as_str()
        && task.source_event_type == request.source_event_type.as_str()
        && task.category == request.category.as_str()
        && task.channel == request.channel.as_str()
        && task.recipient_id == request.recipient_id.as_str()
        && task.title.as_ref() == request.title.as_ref()
        && task.body.as_ref() == request.body.as_ref()
        && task.payload.as_ref() == request.payload.as_ref()
}

fn ensure_notification_request_access(
    auth: &AuthContext,
    recipient_id: &str,
    is_bearer_request: bool,
) -> Result<(), NotificationError> {
    if !is_bearer_request
        || recipient_id == auth.actor_id
        || auth.has_permission("notification.write")
    {
        return Ok(());
    }

    Err(NotificationError::forbidden(
        "permission_denied",
        "missing required permission to request notifications for other recipients: notification.write",
    ))
}
