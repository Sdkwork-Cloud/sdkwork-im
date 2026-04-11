use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::State;
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
use im_time::utc_now_rfc3339_millis;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone)]
struct AppState {
    runtime: Arc<AuditRuntime>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRecord {
    pub tenant_id: String,
    pub record_id: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub action: String,
    pub actor_id: String,
    pub actor_kind: String,
    pub actor_session_id: Option<String>,
    pub payload: Option<String>,
    pub recorded_at: String,
    pub chain_prev_hash: Option<String>,
    pub chain_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditExportBundle {
    pub tenant_id: String,
    pub exported_at: String,
    pub total: usize,
    pub items: Vec<AuditRecord>,
    pub chain_head_hash: Option<String>,
    pub chain_valid: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditChainVerification {
    pub tenant_id: String,
    pub verified_at: String,
    pub total: usize,
    pub chain_head_hash: Option<String>,
    pub chain_valid: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordAuditAnchor {
    pub record_id: String,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub action: String,
    pub payload: Option<String>,
}

#[derive(Default)]
pub struct AuditRuntime {
    records: Mutex<HashMap<String, Vec<AuditRecord>>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AuditRecordListResponse {
    items: Vec<AuditRecord>,
}

#[derive(Debug)]
pub struct AuditError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl From<AuthContextError> for AuditError {
    fn from(value: AuthContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl AuditError {
    fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
        }
    }
}

impl axum::response::IntoResponse for AuditError {
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

impl AuditRuntime {
    pub fn record_anchor(&self, auth: &AuthContext, request: RecordAuditAnchor) -> AuditRecord {
        let recorded_at = utc_now_rfc3339_millis();
        let mut records = self.records.lock().expect("audit runtime should lock");
        let tenant_records = records.entry(auth.tenant_id.clone()).or_default();
        let chain_prev_hash = tenant_records
            .last()
            .map(|record| record.chain_hash.clone());
        let chain_hash = compute_audit_record_chain_hash(
            auth.tenant_id.as_str(),
            request.record_id.as_str(),
            request.aggregate_type.as_str(),
            request.aggregate_id.as_str(),
            request.action.as_str(),
            auth.actor_id.as_str(),
            auth.actor_kind.as_str(),
            auth.session_id.as_deref(),
            request.payload.as_deref(),
            recorded_at.as_str(),
            chain_prev_hash.as_deref(),
        );
        let record = AuditRecord {
            tenant_id: auth.tenant_id.clone(),
            record_id: request.record_id,
            aggregate_type: request.aggregate_type,
            aggregate_id: request.aggregate_id,
            action: request.action,
            actor_id: auth.actor_id.clone(),
            actor_kind: auth.actor_kind.clone(),
            actor_session_id: auth.session_id.clone(),
            payload: request.payload,
            recorded_at,
            chain_prev_hash,
            chain_hash,
        };
        tenant_records.push(record.clone());
        record
    }

    pub fn list_records(&self, auth: &AuthContext) -> Vec<AuditRecord> {
        self.records
            .lock()
            .expect("audit runtime should lock")
            .get(auth.tenant_id.as_str())
            .cloned()
            .unwrap_or_default()
    }

    pub fn export_bundle(&self, auth: &AuthContext) -> AuditExportBundle {
        let items = self.list_records(auth);
        let chain_head_hash = items.last().map(|record| record.chain_hash.clone());
        let chain_valid = verify_audit_records_chain(auth.tenant_id.as_str(), items.as_slice());
        AuditExportBundle {
            tenant_id: auth.tenant_id.clone(),
            exported_at: utc_now_rfc3339_millis(),
            total: items.len(),
            items,
            chain_head_hash,
            chain_valid,
        }
    }

    pub fn verify_chain(&self, auth: &AuthContext) -> AuditChainVerification {
        let items = self.list_records(auth);
        let chain_head_hash = items.last().map(|record| record.chain_hash.clone());
        let chain_valid = verify_audit_records_chain(auth.tenant_id.as_str(), items.as_slice());
        AuditChainVerification {
            tenant_id: auth.tenant_id.clone(),
            verified_at: utc_now_rfc3339_millis(),
            total: items.len(),
            chain_head_hash,
            chain_valid,
        }
    }
}

pub fn verify_audit_export_bundle_integrity(bundle: &AuditExportBundle) -> bool {
    if bundle.total != bundle.items.len() {
        return false;
    }

    let actual_chain_valid = verify_audit_records_chain(bundle.tenant_id.as_str(), &bundle.items);
    if bundle.chain_valid != actual_chain_valid {
        return false;
    }

    let actual_chain_head_hash = bundle.items.last().map(|record| record.chain_hash.clone());
    if bundle.chain_head_hash != actual_chain_head_hash {
        return false;
    }

    actual_chain_valid
}

fn verify_audit_records_chain(tenant_id: &str, items: &[AuditRecord]) -> bool {
    let mut previous_hash: Option<&str> = None;

    for item in items {
        if item.tenant_id != tenant_id {
            return false;
        }
        if item.chain_prev_hash.as_deref() != previous_hash {
            return false;
        }

        let expected_hash = compute_audit_record_chain_hash(
            item.tenant_id.as_str(),
            item.record_id.as_str(),
            item.aggregate_type.as_str(),
            item.aggregate_id.as_str(),
            item.action.as_str(),
            item.actor_id.as_str(),
            item.actor_kind.as_str(),
            item.actor_session_id.as_deref(),
            item.payload.as_deref(),
            item.recorded_at.as_str(),
            previous_hash,
        );
        if item.chain_hash != expected_hash {
            return false;
        }

        previous_hash = Some(item.chain_hash.as_str());
    }

    true
}

fn compute_audit_record_chain_hash(
    tenant_id: &str,
    record_id: &str,
    aggregate_type: &str,
    aggregate_id: &str,
    action: &str,
    actor_id: &str,
    actor_kind: &str,
    actor_session_id: Option<&str>,
    payload: Option<&str>,
    recorded_at: &str,
    chain_prev_hash: Option<&str>,
) -> String {
    let canonical = serde_json::json!([
        tenant_id,
        record_id,
        aggregate_type,
        aggregate_id,
        action,
        actor_id,
        actor_kind,
        actor_session_id.unwrap_or(""),
        payload.unwrap_or(""),
        recorded_at,
        chain_prev_hash.unwrap_or(""),
    ]);
    let canonical_bytes = serde_json::to_vec(&canonical).expect("canonical audit hash payload");
    let digest = Sha256::digest(canonical_bytes.as_slice());
    digest_to_hex(digest.as_slice())
}

fn digest_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write;
        let _ = write!(&mut output, "{byte:02x}");
    }
    output
}

pub fn build_default_app() -> Router {
    build_app(Arc::new(AuditRuntime::default()))
}

pub fn build_public_app() -> Router {
    build_default_app().layer(middleware::from_fn(require_public_bearer_auth))
}

pub fn build_app(runtime: Arc<AuditRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/api/v1/audit/records", post(record_anchor))
        .route("/api/v1/audit/records", get(list_records))
        .route("/api/v1/audit/export", get(export_bundle))
        .route("/api/v1/audit/verify", get(verify_chain))
        .with_state(AppState { runtime })
}

async fn require_public_bearer_auth(request: Request<axum::body::Body>, next: Next) -> Response {
    match request.uri().path() {
        "/healthz" | "/readyz" => next.run(request).await,
        _ => match resolve_public_bearer_auth_context(request.headers()) {
            Ok(_) => next.run(request).await,
            Err(error) => AuditError::from(error).into_response(),
        },
    }
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "audit-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "audit-service",
    })
}

async fn record_anchor(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RecordAuditAnchor>,
) -> Result<Json<AuditRecord>, AuditError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_audit_write_access(&auth)?;
    Ok(Json(state.runtime.record_anchor(&auth, request)))
}

async fn list_records(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AuditRecordListResponse>, AuditError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_audit_read_access(&auth)?;
    Ok(Json(AuditRecordListResponse {
        items: state.runtime.list_records(&auth),
    }))
}

async fn export_bundle(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AuditExportBundle>, AuditError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_audit_read_access(&auth)?;
    Ok(Json(state.runtime.export_bundle(&auth)))
}

async fn verify_chain(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AuditChainVerification>, AuditError> {
    let auth = resolve_auth_context(&headers)?;
    ensure_audit_read_access(&auth)?;
    Ok(Json(state.runtime.verify_chain(&auth)))
}

fn ensure_audit_read_access(auth: &AuthContext) -> Result<(), AuditError> {
    if auth.has_permission("audit.read") {
        return Ok(());
    }

    Err(AuditError::forbidden("audit.read"))
}

fn ensure_audit_write_access(auth: &AuthContext) -> Result<(), AuditError> {
    if auth.has_permission("audit.write") {
        return Ok(());
    }

    Err(AuditError::forbidden("audit.write"))
}
