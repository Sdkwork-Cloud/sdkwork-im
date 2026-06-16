use std::collections::{BTreeMap, HashMap};
use std::ops::Bound::{Excluded, Unbounded};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

use axum::extract::{DefaultBodyLimit, Extension, Query, State};
use axum::http::{HeaderMap, Request};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::{
    Json, Router,
    routing::{get, post},
};
use im_app_context::{
    AppContext, AppContextError, resolve_app_context, resolve_app_context_for_request,
};
use im_time::utc_now_rfc3339_millis;
use sdkwork_im_api_registry::HttpMethod;
use sdkwork_im_openapi::{
    OpenApiServiceSpec, build_openapi_document, extract_routes_from_function, render_docs_html,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::sync::Semaphore;

const AUDIT_RECORD_ID_MAX_BYTES: usize = 256;
const AUDIT_AGGREGATE_TYPE_MAX_BYTES: usize = 128;
const AUDIT_AGGREGATE_ID_MAX_BYTES: usize = 256;
const AUDIT_ACTION_MAX_BYTES: usize = 128;
const AUDIT_PAYLOAD_MAX_BYTES: usize = 128 * 1024;
const AUDIT_RECORD_LIST_MAX_LIMIT: usize = 1000;
const AUDIT_RECORD_DELIVERY_PROOF_VERSION: &str = "audit.record.delivery-proof.v1";
const AUDIT_MAX_IN_FLIGHT_REQUESTS_ENV: &str = "SDKWORK_IM_AUDIT_MAX_IN_FLIGHT_REQUESTS";
const AUDIT_MAX_IN_FLIGHT_REQUESTS_DEFAULT: usize = 1_000;
const AUDIT_MAX_IN_FLIGHT_REQUESTS_MAX: usize = 20_000;
const AUDIT_MAX_REQUEST_BODY_BYTES_ENV: &str = "SDKWORK_IM_AUDIT_MAX_REQUEST_BODY_BYTES";
const AUDIT_MAX_REQUEST_BODY_BYTES_DEFAULT: usize = 5 * 1024 * 1024;
const AUDIT_MAX_REQUEST_BODY_BYTES_MAX: usize = 20 * 1024 * 1024;
const AUDIT_REQUIRE_DUAL_TOKEN_HEADERS_ENV: &str = "SDKWORK_IM_AUDIT_REQUIRE_DUAL_TOKEN_HEADERS";

#[derive(Clone)]
struct AppState {
    runtime: Arc<AuditRuntime>,
}

#[derive(Clone)]
struct PublicAppGuardrails {
    request_gate: Arc<Semaphore>,
    require_dual_token_headers: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRecord {
    pub tenant_id: String,
    pub record_id: String,
    pub audit_seq: u64,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAuditRecordsQuery {
    pub after_audit_seq: Option<u64>,
    pub limit: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuditRecordMutationOutcome {
    pub record: AuditRecord,
    pub applied: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditRecordDeliveryStatus {
    Applied,
    Replayed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRecordMutationResponse {
    #[serde(flatten)]
    pub record: AuditRecord,
    pub request_key: String,
    pub delivery_status: AuditRecordDeliveryStatus,
    pub proof_version: String,
}

impl AuditRecordMutationResponse {
    pub fn from_outcome(outcome: AuditRecordMutationOutcome, request_key: String) -> Self {
        Self {
            record: outcome.record,
            request_key,
            delivery_status: if outcome.applied {
                AuditRecordDeliveryStatus::Applied
            } else {
                AuditRecordDeliveryStatus::Replayed
            },
            proof_version: AUDIT_RECORD_DELIVERY_PROOF_VERSION.into(),
        }
    }
}

#[derive(Default)]
pub struct AuditRuntime {
    records: RwLock<HashMap<String, TenantAuditRecords>>,
}

#[derive(Default)]
struct TenantAuditRecords {
    by_record_id: HashMap<String, AuditRecord>,
    by_audit_seq: BTreeMap<u64, String>,
    record_order: Vec<String>,
}

impl TenantAuditRecords {
    fn get(&self, record_id: &str) -> Option<&AuditRecord> {
        self.by_record_id.get(record_id)
    }

    fn last(&self) -> Option<&AuditRecord> {
        self.record_order
            .last()
            .and_then(|record_id| self.by_record_id.get(record_id.as_str()))
    }

    fn push(&mut self, record: AuditRecord) {
        self.record_order.push(record.record_id.clone());
        self.by_audit_seq
            .insert(record.audit_seq, record.record_id.clone());
        self.by_record_id.insert(record.record_id.clone(), record);
    }

    fn ordered_items(&self) -> Vec<AuditRecord> {
        self.record_order
            .iter()
            .filter_map(|record_id| self.by_record_id.get(record_id.as_str()).cloned())
            .collect()
    }

    fn next_audit_seq(&self) -> u64 {
        self.by_audit_seq
            .last_key_value()
            .map_or(1, |(seq, _)| seq + 1)
    }

    fn window(&self, after_audit_seq: u64, limit: usize) -> AuditRecordListResponse {
        let mut items = Vec::new();
        let mut has_more = false;
        for (_, record_id) in self
            .by_audit_seq
            .range((Excluded(after_audit_seq), Unbounded))
        {
            if items.len() == limit {
                has_more = true;
                break;
            }
            if let Some(record) = self.by_record_id.get(record_id.as_str()).cloned() {
                items.push(record);
            }
        }
        let next_after_audit_seq = items.last().map(|record| record.audit_seq);

        AuditRecordListResponse {
            items,
            next_after_audit_seq,
            has_more,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditRecordListResponse {
    items: Vec<AuditRecord>,
    next_after_audit_seq: Option<u64>,
    has_more: bool,
}

#[derive(Debug)]
pub struct AuditError {
    status: axum::http::StatusCode,
    code: &'static str,
    message: String,
}

impl From<AppContextError> for AuditError {
    fn from(value: AppContextError) -> Self {
        Self {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}

impl AuditError {
    fn internal(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code,
            message: message.into(),
        }
    }

    fn conflict(record_id: &str) -> Self {
        Self {
            status: axum::http::StatusCode::CONFLICT,
            code: "audit_record_conflict",
            message: format!(
                "audit record request conflicts with existing idempotency key: {record_id}"
            ),
        }
    }

    fn forbidden(required_permission: &'static str) -> Self {
        Self {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "permission_denied",
            message: format!("missing required permission: {required_permission}"),
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
    pub fn record_anchor(
        &self,
        auth: &AppContext,
        request: RecordAuditAnchor,
    ) -> Result<AuditRecord, AuditError> {
        Ok(self.record_anchor_with_outcome(auth, request)?.record)
    }

    pub fn record_anchor_with_outcome(
        &self,
        auth: &AppContext,
        request: RecordAuditAnchor,
    ) -> Result<AuditRecordMutationOutcome, AuditError> {
        validate_record_audit_anchor_request(&request)?;
        let recorded_at = utc_now_rfc3339_millis();
        let mut records = self.write_records("record_anchor");
        let tenant_records = records.entry(auth.tenant_id.clone()).or_default();
        if let Some(existing) = tenant_records.get(request.record_id.as_str()).cloned() {
            if audit_record_matches_request(&existing, auth, &request) {
                return Ok(AuditRecordMutationOutcome {
                    record: existing,
                    applied: false,
                });
            }
            return Err(AuditError::conflict(request.record_id.as_str()));
        }
        let chain_prev_hash = tenant_records
            .last()
            .map(|record| record.chain_hash.clone());
        let next_audit_seq = tenant_records.next_audit_seq();
        let chain_hash = compute_audit_record_chain_hash(AuditRecordHashInput {
            tenant_id: auth.tenant_id.as_str(),
            record_id: request.record_id.as_str(),
            audit_seq: next_audit_seq,
            aggregate_type: request.aggregate_type.as_str(),
            aggregate_id: request.aggregate_id.as_str(),
            action: request.action.as_str(),
            actor_id: auth.actor_id.as_str(),
            actor_kind: auth.actor_kind.as_str(),
            actor_session_id: auth.session_id.as_deref(),
            payload: request.payload.as_deref(),
            recorded_at: recorded_at.as_str(),
            chain_prev_hash: chain_prev_hash.as_deref(),
        });
        let record = AuditRecord {
            tenant_id: auth.tenant_id.clone(),
            record_id: request.record_id,
            audit_seq: next_audit_seq,
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
        Ok(AuditRecordMutationOutcome {
            record,
            applied: true,
        })
    }

    pub fn list_records(&self, auth: &AppContext) -> Vec<AuditRecord> {
        self.read_records("list_records")
            .get(auth.tenant_id.as_str())
            .map(TenantAuditRecords::ordered_items)
            .unwrap_or_default()
    }

    pub fn list_records_window(
        &self,
        auth: &AppContext,
        query: ListAuditRecordsQuery,
    ) -> Result<AuditRecordListResponse, AuditError> {
        let after_audit_seq = query.after_audit_seq.unwrap_or(0);
        let limit = query.limit.unwrap_or(100);
        if limit == 0 {
            return Err(AuditError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: "limit must be greater than 0".into(),
            });
        }
        if limit > AUDIT_RECORD_LIST_MAX_LIMIT {
            return Err(AuditError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: "invalid_limit",
                message: format!(
                    "limit must be less than or equal to {AUDIT_RECORD_LIST_MAX_LIMIT}"
                ),
            });
        }

        Ok(self
            .read_records("list_records_window")
            .get(auth.tenant_id.as_str())
            .map(|tenant_records| tenant_records.window(after_audit_seq, limit))
            .unwrap_or_else(|| AuditRecordListResponse {
                items: Vec::new(),
                next_after_audit_seq: None,
                has_more: false,
            }))
    }

    pub fn export_bundle(&self, auth: &AppContext) -> AuditExportBundle {
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

    pub fn verify_chain(&self, auth: &AppContext) -> AuditChainVerification {
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

    fn read_records(
        &self,
        operation: &'static str,
    ) -> RwLockReadGuard<'_, HashMap<String, TenantAuditRecords>> {
        match self.records.read() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!(
                    "recovering poisoned audit-service records read lock during {operation}"
                );
                poisoned.into_inner()
            }
        }
    }

    fn write_records(
        &self,
        operation: &'static str,
    ) -> RwLockWriteGuard<'_, HashMap<String, TenantAuditRecords>> {
        match self.records.write() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::warn!(
                    "recovering poisoned audit-service records write lock during {operation}"
                );
                poisoned.into_inner()
            }
        }
    }
}

fn validate_payload_size(
    field: &'static str,
    value: &str,
    max_bytes: usize,
) -> Result<(), AuditError> {
    let actual_bytes = value.len();
    if actual_bytes > max_bytes {
        return Err(AuditError::payload_too_large(
            field,
            max_bytes,
            actual_bytes,
        ));
    }
    Ok(())
}

pub fn validate_record_audit_anchor_request(request: &RecordAuditAnchor) -> Result<(), AuditError> {
    validate_payload_size(
        "recordId",
        request.record_id.as_str(),
        AUDIT_RECORD_ID_MAX_BYTES,
    )?;
    validate_payload_size(
        "aggregateType",
        request.aggregate_type.as_str(),
        AUDIT_AGGREGATE_TYPE_MAX_BYTES,
    )?;
    validate_payload_size(
        "aggregateId",
        request.aggregate_id.as_str(),
        AUDIT_AGGREGATE_ID_MAX_BYTES,
    )?;
    validate_payload_size("action", request.action.as_str(), AUDIT_ACTION_MAX_BYTES)?;
    if let Some(payload) = request.payload.as_deref() {
        validate_payload_size("payload", payload, AUDIT_PAYLOAD_MAX_BYTES)?;
    }
    Ok(())
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

        let expected_hash = compute_audit_record_chain_hash(AuditRecordHashInput {
            tenant_id: item.tenant_id.as_str(),
            record_id: item.record_id.as_str(),
            audit_seq: item.audit_seq,
            aggregate_type: item.aggregate_type.as_str(),
            aggregate_id: item.aggregate_id.as_str(),
            action: item.action.as_str(),
            actor_id: item.actor_id.as_str(),
            actor_kind: item.actor_kind.as_str(),
            actor_session_id: item.actor_session_id.as_deref(),
            payload: item.payload.as_deref(),
            recorded_at: item.recorded_at.as_str(),
            chain_prev_hash: previous_hash,
        });
        if item.chain_hash != expected_hash {
            return false;
        }

        previous_hash = Some(item.chain_hash.as_str());
    }

    true
}

struct AuditRecordHashInput<'a> {
    tenant_id: &'a str,
    record_id: &'a str,
    audit_seq: u64,
    aggregate_type: &'a str,
    aggregate_id: &'a str,
    action: &'a str,
    actor_id: &'a str,
    actor_kind: &'a str,
    actor_session_id: Option<&'a str>,
    payload: Option<&'a str>,
    recorded_at: &'a str,
    chain_prev_hash: Option<&'a str>,
}

fn compute_audit_record_chain_hash(input: AuditRecordHashInput<'_>) -> String {
    let canonical = serde_json::json!([
        input.tenant_id,
        input.record_id,
        input.audit_seq,
        input.aggregate_type,
        input.aggregate_id,
        input.action,
        input.actor_id,
        input.actor_kind,
        input.actor_session_id.unwrap_or(""),
        input.payload.unwrap_or(""),
        input.recorded_at,
        input.chain_prev_hash.unwrap_or(""),
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

pub fn build_app(runtime: Arc<AuditRuntime>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/openapi.json", get(openapi_json))
        .route("/docs", get(docs))
        .route("/backend/v3/api/audit/records", post(record_anchor))
        .route("/backend/v3/api/audit/records", get(list_records))
        .route("/backend/v3/api/audit/export", get(export_bundle))
        .route("/backend/v3/api/audit/verify", get(verify_chain))
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
                    return AuditError {
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
                Err(error) => return AuditError::from(error).into_response(),
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
        service: "audit-service",
    })
}

async fn readyz() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "audit-service",
    })
}

async fn openapi_json() -> Result<Json<serde_json::Value>, AuditError> {
    Ok(Json(build_audit_service_openapi_document().map_err(
        |message| AuditError::internal("openapi_export_failed", message),
    )?))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&audit_service_openapi_spec()))
}

fn build_audit_service_openapi_document() -> Result<serde_json::Value, String> {
    let routes = extract_routes_from_function(
        include_str!("lib.rs"),
        "build_app",
        &[],
        &["/openapi.json", "/docs"],
    )?;

    Ok(build_openapi_document(
        &audit_service_openapi_spec(),
        &routes,
        audit_service_tag,
        audit_service_requires_app_context,
        audit_service_summary,
    ))
}

fn audit_service_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Sdkwork IM Audit Service API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Live OpenAPI contract generated from the audit-service router for audit record mutation, export, verification, and record listing flows.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn audit_service_tag(path: &str, _method: HttpMethod) -> String {
    match path {
        "/healthz" | "/readyz" => "system".to_owned(),
        _ => "audit".to_owned(),
    }
}

fn audit_service_requires_app_context(path: &str, _method: HttpMethod) -> bool {
    !matches!(path, "/healthz" | "/readyz")
}

fn audit_service_summary(path: &str, method: HttpMethod) -> String {
    match (path, method) {
        ("/healthz", HttpMethod::Get) => "Check audit service health".to_owned(),
        ("/readyz", HttpMethod::Get) => "Check audit service readiness".to_owned(),
        _ => format!(
            "{} {}",
            audit_service_method_display(method),
            path.trim_matches('/').replace('/', " ")
        ),
    }
}

fn audit_service_method_display(method: HttpMethod) -> &'static str {
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

async fn record_anchor(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<RecordAuditAnchor>,
) -> Result<Json<AuditRecordMutationResponse>, AuditError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_audit_write_access(&auth)?;
    validate_record_audit_anchor_request(&request)?;
    let request_key = audit_record_request_key(&auth, request.record_id.as_str());
    Ok(Json(AuditRecordMutationResponse::from_outcome(
        state.runtime.record_anchor_with_outcome(&auth, request)?,
        request_key,
    )))
}

async fn list_records(
    Query(query): Query<ListAuditRecordsQuery>,
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AuditRecordListResponse>, AuditError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_audit_read_access(&auth)?;
    Ok(Json(state.runtime.list_records_window(&auth, query)?))
}

async fn export_bundle(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AuditExportBundle>, AuditError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_audit_read_access(&auth)?;
    Ok(Json(state.runtime.export_bundle(&auth)))
}

async fn verify_chain(
    auth: Option<Extension<AppContext>>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AuditChainVerification>, AuditError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_audit_read_access(&auth)?;
    Ok(Json(state.runtime.verify_chain(&auth)))
}

fn ensure_audit_read_access(auth: &AppContext) -> Result<(), AuditError> {
    if auth.has_permission("audit.read") {
        return Ok(());
    }

    Err(AuditError::forbidden("audit.read"))
}

fn ensure_audit_write_access(auth: &AppContext) -> Result<(), AuditError> {
    if auth.has_permission("audit.write") {
        return Ok(());
    }

    Err(AuditError::forbidden("audit.write"))
}

fn resolve_request_app_context(
    auth: Option<Extension<AppContext>>,
    headers: &HeaderMap,
) -> Result<AppContext, AuditError> {
    match auth {
        Some(Extension(auth)) => Ok(auth),
        None => resolve_app_context(headers).map_err(AuditError::from),
    }
}

fn require_dual_token_headers(headers: &HeaderMap) -> Result<(), AuditError> {
    if !has_bearer_auth_token(headers) {
        return Err(AuditError {
            status: axum::http::StatusCode::UNAUTHORIZED,
            code: "auth_token_missing",
            message: "authorization header must provide a bearer token".to_owned(),
        });
    }
    if !has_access_token_header(headers) {
        return Err(AuditError {
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
    std::env::var(AUDIT_MAX_IN_FLIGHT_REQUESTS_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(AUDIT_MAX_IN_FLIGHT_REQUESTS_DEFAULT)
        .min(AUDIT_MAX_IN_FLIGHT_REQUESTS_MAX)
}

fn resolve_max_http_request_body_bytes() -> usize {
    std::env::var(AUDIT_MAX_REQUEST_BODY_BYTES_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&parsed| parsed > 0)
        .unwrap_or(AUDIT_MAX_REQUEST_BODY_BYTES_DEFAULT)
        .min(AUDIT_MAX_REQUEST_BODY_BYTES_MAX)
}

fn resolve_require_dual_token_headers() -> bool {
    std::env::var(AUDIT_REQUIRE_DUAL_TOKEN_HEADERS_ENV)
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

pub fn audit_record_request_key(auth: &AppContext, record_id: &str) -> String {
    format!("{}:audit-record:{}", auth.tenant_id, record_id)
}

fn audit_record_matches_request(
    existing: &AuditRecord,
    auth: &AppContext,
    request: &RecordAuditAnchor,
) -> bool {
    existing.tenant_id == auth.tenant_id
        && existing.record_id == request.record_id
        && existing.aggregate_type == request.aggregate_type
        && existing.aggregate_id == request.aggregate_id
        && existing.action == request.action
        && existing.actor_id == auth.actor_id
        && existing.actor_kind == auth.actor_kind
        && existing.payload == request.payload
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, HeaderValue};

    use super::*;

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
