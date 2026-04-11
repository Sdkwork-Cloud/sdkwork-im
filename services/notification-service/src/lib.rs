use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::{Arc, Mutex, MutexGuard};

use axum::extract::{Path, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use craw_chat_contract_core::ContractError;
use craw_chat_contract_message::{CommitJournal, CommitPosition};
use craw_chat_contract_notification::{NotificationTaskRecord, NotificationTaskStore};
use im_auth_context::{
    AuthContext, AuthContextError, resolve_auth_context, resolve_public_bearer_auth_context,
};
pub use im_domain_core::notification::{NotificationStatus, NotificationTask};
use im_domain_events::{AggregateType, CommitEnvelope, EventActor};
use im_time::utc_now_rfc3339_millis;
use projection_service::{ProjectionAccessError, TimelineProjectionService};
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestNotificationFanout {
    pub notification_id_seed: String,
    pub source_event_id: String,
    pub source_event_type: String,
    pub category: String,
    pub channel: String,
    pub recipient_ids: BTreeSet<String>,
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
    projection_service: Arc<TimelineProjectionService>,
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
            tasks: Mutex::new(HashMap::new()),
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
                .lock_notification()
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
            .lock_notification()
            .contains(recipient_key.as_str())
        {
            return Ok(());
        }

        let restored = self
            .task_store
            .list_tasks_for_recipient(tenant_id, recipient_id)
            .map_err(NotificationError::notification_store)?;
        let mut tasks = self.tasks.lock_notification();
        for record in restored {
            tasks.insert(
                notification_scope_key(record.tenant_id.as_str(), record.notification_id.as_str()),
                record.task,
            );
        }
        drop(tasks);
        self.restored_recipients
            .lock_notification()
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
        let mut tasks = self.tasks.lock_notification();

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

    pub fn request_notification_from_public_api(
        &self,
        auth: &AuthContext,
        request: RequestNotification,
        is_bearer_request: bool,
    ) -> Result<NotificationRequestResult, NotificationError> {
        ensure_notification_request_access(auth, request.recipient_id.as_str(), is_bearer_request)?;
        self.request_notification_with_outcome(auth, request)
    }

    pub fn request_notification_fanout(
        &self,
        auth: &AuthContext,
        request: RequestNotificationFanout,
    ) -> Result<Vec<NotificationTask>, NotificationError> {
        let mut tasks = Vec::new();

        for recipient_id in request
            .recipient_ids
            .into_iter()
            .filter(|recipient_id| recipient_id.as_str() != auth.actor_id.as_str())
        {
            tasks.push(self.request_notification(
                auth,
                RequestNotification {
                    notification_id: format!(
                        "ntf_{}_{}",
                        request.notification_id_seed, recipient_id
                    ),
                    source_event_id: request.source_event_id.clone(),
                    source_event_type: request.source_event_type.clone(),
                    category: request.category.clone(),
                    channel: request.channel.clone(),
                    recipient_id,
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
        auth: &AuthContext,
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
        let recipient_ids = self
            .projection_service
            .message_posted_notification_principal_ids_from_auth_context(
                auth,
                conversation_id.as_str(),
            )?
            .into_iter()
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
                recipient_ids,
                title: summary.clone(),
                body: summary,
                payload: Some(payload),
            },
        )
    }

    pub fn request_automation_result_notification(
        &self,
        auth: &AuthContext,
        request: RequestAutomationResultNotification,
    ) -> Result<NotificationTask, NotificationError> {
        self.request_notification(
            auth,
            RequestNotification {
                notification_id: format!("ntf_automation_{}", request.execution_id),
                source_event_id: format!(
                    "evt_{}_automation_execution_completed",
                    request.execution_id
                ),
                source_event_type: "automation.execution_completed".into(),
                category: "automation.result".into(),
                channel: "inapp".into(),
                recipient_id: auth.actor_id.clone(),
                title: Some("Automation completed".into()),
                body: Some(request.target_ref),
                payload: request.output_payload,
            },
        )
    }

    pub fn list_notifications(
        &self,
        auth: &AuthContext,
    ) -> Result<Vec<NotificationTask>, NotificationError> {
        self.ensure_recipient_tasks(auth.tenant_id.as_str(), auth.actor_id.as_str())?;
        let prefix = format!("{}:", auth.tenant_id);
        let mut items: Vec<_> = self
            .tasks
            .lock_notification()
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
            .lock_notification()
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
            .lock_notification()
            .get(notification_scope_key(tenant_id, notification_id).as_str())
            .cloned())
    }

    fn save_task(&self, record: NotificationTaskRecord) -> Result<(), ContractError> {
        self.tasks.lock_notification().insert(
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
            .lock_notification()
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
    Ok(Json(
        state
            .runtime
            .request_notification_from_public_api(&auth, request, is_bearer_request)?
            .task,
    ))
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

trait NotificationMutexExt<T> {
    fn lock_notification(&self) -> MutexGuard<'_, T>;
}

impl<T> NotificationMutexExt<T> for Mutex<T> {
    fn lock_notification(&self) -> MutexGuard<'_, T> {
        match self.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("warning: recovering poisoned mutex in notification-service");
                poisoned.into_inner()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{self, AssertUnwindSafe};

    fn demo_auth_context() -> AuthContext {
        AuthContext {
            tenant_id: "t_demo".into(),
            actor_id: "u_demo".into(),
            actor_kind: "user".into(),
            session_id: Some("s_demo".into()),
            device_id: Some("d_demo".into()),
            permissions: Default::default(),
        }
    }

    fn poison_mutex<T>(mutex: &Mutex<T>) {
        let _ = panic::catch_unwind(AssertUnwindSafe(|| {
            let _guard = mutex.lock().expect("test poison lock should succeed");
            panic!("intentional poison for regression coverage");
        }));
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
        poison_mutex(&store.tasks);

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
}
