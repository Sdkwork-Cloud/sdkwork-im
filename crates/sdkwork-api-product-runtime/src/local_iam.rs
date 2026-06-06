use crate::{json_error_response, RuntimeProxyState, JSON_CONTENT_TYPE};
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::Response,
    routing::{get, post},
    Router,
};
use rand::random;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

const LOCAL_APP_ID: &str = "sdkwork-chat-pc";
const LOCAL_TENANT_ID: &str = "sdkwork-local-tenant";
const LOCAL_ORGANIZATION_ID: &str = "sdkwork-local-org";
const LOCAL_DEFAULT_USER_ID: &str = "U1000000000";
const LOCAL_PUBLIC_USER_ID_MIN: u64 = 1_000_000_000;
const LOCAL_PUBLIC_USER_ID_RANGE: u64 = 9_000_000_000;
const LOCAL_DEFAULT_USERNAME: &str = "local-default@sdkwork-iam.local";
const LOCAL_DEFAULT_PHONE: &str = "13800000000";
const LOCAL_DEFAULT_DISPLAY_NAME: &str = "Local Default User";
const LOCAL_VERIFY_CODE: &str = "123456";
const LOCAL_EXPIRES_AT: &str = "2099-01-01T00:00:00Z";
const LOCAL_QR_CREATED_AT: &str = "2026-06-03T13:23:32Z";
const LOCAL_QR_UPDATED_AT: &str = "2026-06-03T13:23:32Z";
const LOCAL_QR_SCANNED_AT: &str = "2026-06-03T13:24:32Z";
const LOCAL_QR_COMPLETED_AT: &str = "2026-06-03T13:25:32Z";

#[derive(Clone, Default)]
pub(crate) struct LocalIamState {
    inner: Arc<Mutex<LocalIamInner>>,
}

#[derive(Default)]
struct LocalIamInner {
    qr_sessions: HashMap<String, LocalQrSession>,
    sessions_by_id: HashMap<String, Value>,
    token_index: HashMap<String, String>,
}

#[derive(Clone)]
struct LocalQrSession {
    session_key: String,
    purpose: String,
    status: String,
    fallback_url: String,
    expires_at: String,
    created_at: String,
    updated_at: String,
    scanned_at: Option<String>,
    completed_at: Option<String>,
    session: Option<Value>,
}

impl LocalIamState {
    fn lock(&self) -> std::sync::MutexGuard<'_, LocalIamInner> {
        self.inner.lock().unwrap_or_else(|error| error.into_inner())
    }

    fn create_qr_session(&self, input: &Value) -> Value {
        let session_key = format!("{:032x}", random::<u128>());
        let purpose = string_field(input, &["purpose", "scene"]).unwrap_or_else(|| "login".into());
        let fallback_url = local_qr_fallback_url(session_key.as_str(), purpose.as_str());
        let qr_session = LocalQrSession {
            session_key: session_key.clone(),
            purpose,
            status: "pending".into(),
            fallback_url,
            expires_at: LOCAL_EXPIRES_AT.into(),
            created_at: LOCAL_QR_CREATED_AT.into(),
            updated_at: LOCAL_QR_UPDATED_AT.into(),
            scanned_at: None,
            completed_at: None,
            session: None,
        };
        let response = qr_session.to_json();
        self.lock().qr_sessions.insert(session_key, qr_session);
        response
    }

    fn get_qr_session(&self, session_key: &str) -> Option<Value> {
        self.lock()
            .qr_sessions
            .get(session_key)
            .map(LocalQrSession::to_json)
    }

    fn mark_qr_scanned(&self, session_key: &str) -> Option<Value> {
        let mut inner = self.lock();
        let qr_session = inner.qr_sessions.get_mut(session_key)?;
        qr_session.status = "scanned".into();
        qr_session.scanned_at = Some(LOCAL_QR_SCANNED_AT.into());
        qr_session.updated_at = LOCAL_QR_SCANNED_AT.into();
        Some(qr_session.to_json())
    }

    fn confirm_qr_with_password(&self, session_key: &str, input: &Value) -> Option<Value> {
        let auth_session = build_auth_session(input);
        let mut inner = self.lock();
        let response = {
            let qr_session = inner.qr_sessions.get_mut(session_key)?;
            qr_session.status = "completed".into();
            qr_session.completed_at = Some(LOCAL_QR_COMPLETED_AT.into());
            qr_session.updated_at = LOCAL_QR_COMPLETED_AT.into();
            qr_session.session = Some(auth_session.clone());
            qr_session.to_json()
        };
        inner.store_auth_session(auth_session);
        Some(response)
    }

    fn issue_password_session(&self, input: &Value) -> Value {
        let auth_session = build_auth_session(input);
        self.lock().store_auth_session(auth_session.clone());
        auth_session
    }

    fn session_for_token(&self, token: &str) -> Option<Value> {
        self.lock().session_for_token(token)
    }

    fn session_for_id(&self, session_id: &str) -> Option<Value> {
        self.lock().sessions_by_id.get(session_id).cloned()
    }

    fn delete_session_for_token(&self, token: &str) {
        let mut inner = self.lock();
        let Some(session_id) = inner.token_index.get(token).cloned() else {
            return;
        };
        inner.sessions_by_id.remove(session_id.as_str());
        inner.token_index.retain(|_, value| value != &session_id);
    }
}

impl LocalIamInner {
    fn store_auth_session(&mut self, auth_session: Value) {
        let Some(session_id) = auth_session["sessionId"].as_str().map(str::to_owned) else {
            return;
        };

        for token_field in ["accessToken", "authToken", "refreshToken"] {
            if let Some(token) = auth_session[token_field].as_str() {
                self.token_index
                    .insert(token.to_owned(), session_id.clone());
            }
        }
        self.sessions_by_id.insert(session_id, auth_session);
    }

    fn session_for_token(&self, token: &str) -> Option<Value> {
        let session_id = self.token_index.get(token)?;
        self.sessions_by_id.get(session_id).cloned()
    }
}

impl LocalQrSession {
    fn to_json(&self) -> Value {
        let mut value = json!({
            "id": format!("qr_auth_session_{}", self.session_key),
            "sessionKey": self.session_key,
            "purpose": self.purpose,
            "defaultAccountId": null,
            "defaultEntryId": null,
            "defaultProvider": null,
            "defaultAccountType": null,
            "status": self.status,
            "qrContent": {
                "content": self.fallback_url,
                "mode": "fallback_url",
            },
            "fallbackUrl": self.fallback_url,
            "scannedAt": self.scanned_at,
            "completedAt": self.completed_at,
            "expiresAt": self.expires_at,
            "createdAt": self.created_at,
            "updatedAt": self.updated_at,
        });
        if let Some(session) = &self.session {
            value["session"] = session.clone();
        }
        value
    }
}

pub(crate) fn local_iam_router() -> Router<RuntimeProxyState> {
    Router::new()
        .route("/app/v3/api/auth/sessions", post(create_auth_session))
        .route(
            "/app/v3/api/auth/sessions/current",
            get(get_current_session)
                .patch(update_current_session)
                .delete(delete_current_session),
        )
        .route(
            "/app/v3/api/auth/sessions/refresh",
            post(refresh_auth_session),
        )
        .route("/app/v3/api/auth/registrations", post(create_registration))
        .route(
            "/app/v3/api/auth/verification_codes",
            post(create_verification_code),
        )
        .route(
            "/app/v3/api/auth/verification_codes/verify",
            post(verify_verification_code),
        )
        .route(
            "/app/v3/api/auth/password_reset_requests",
            post(create_password_reset_request),
        )
        .route(
            "/app/v3/api/auth/password_resets",
            post(create_password_reset),
        )
        .route(
            "/app/v3/api/auth/oauth_authorization_urls",
            get(get_oauth_authorization_url),
        )
        .route(
            "/app/v3/api/auth/oauth_sessions",
            post(create_oauth_session),
        )
        .route("/app/v3/api/iam/users/current", get(get_current_user))
        .route("/app/v3/api/iam/organizations", get(list_organizations))
        .route(
            "/app/v3/api/iam/organizations/tree",
            get(get_organization_tree),
        )
        .route(
            "/app/v3/api/iam/organization_memberships",
            get(list_organization_memberships),
        )
        .route("/app/v3/api/iam/departments", get(list_departments))
        .route(
            "/app/v3/api/iam/departments/tree",
            get(get_department_tree),
        )
        .route(
            "/app/v3/api/iam/department_assignments",
            get(list_department_assignments),
        )
        .route("/app/v3/api/iam/positions", get(list_positions))
        .route(
            "/app/v3/api/iam/position_assignments",
            get(list_position_assignments),
        )
        .route("/app/v3/api/iam/role_bindings", get(list_role_bindings))
        .route("/app/v3/api/system/iam/runtime", get(get_iam_runtime))
        .route(
            "/app/v3/api/system/iam/verification_policy",
            get(get_iam_verification_policy),
        )
        .route(
            "/app/v3/api/open_platform/qr_auth/sessions",
            post(create_qr_auth_session),
        )
        .route(
            "/app/v3/api/open_platform/qr_auth/sessions/{session_key}",
            get(get_qr_auth_session),
        )
        .route(
            "/app/v3/api/open_platform/qr_auth/sessions/{session_key}/scans",
            post(create_qr_auth_scan),
        )
        .route(
            "/app/v3/api/open_platform/qr_auth/sessions/{session_key}/passwords",
            post(create_qr_auth_password),
        )
}

async fn create_auth_session(State(state): State<RuntimeProxyState>, body: Bytes) -> Response {
    let input = match parse_json_body(body) {
        Ok(input) => input,
        Err(response) => return response,
    };
    if let Err(response) = validate_password_input(&input) {
        return response;
    }

    json_response(state.local_iam.issue_password_session(&input))
}

async fn create_registration(State(state): State<RuntimeProxyState>, body: Bytes) -> Response {
    let input = match parse_json_body(body) {
        Ok(input) => input,
        Err(response) => return response,
    };
    if let Err(response) = validate_password_input(&input) {
        return response;
    }

    json_response(state.local_iam.issue_password_session(&input))
}

async fn create_oauth_session(State(state): State<RuntimeProxyState>, body: Bytes) -> Response {
    let input = parse_json_body(body).unwrap_or_else(|_| json!({}));
    json_response(state.local_iam.issue_password_session(&input))
}

async fn get_current_session(
    State(state): State<RuntimeProxyState>,
    headers: HeaderMap,
) -> Response {
    let Some(session) = resolve_session_from_headers(&state, &headers) else {
        return json_error_response(StatusCode::UNAUTHORIZED, "IAM session token is required.");
    };

    json_response(session)
}

async fn update_current_session(
    State(state): State<RuntimeProxyState>,
    headers: HeaderMap,
    _body: Bytes,
) -> Response {
    let Some(session) = resolve_session_from_headers(&state, &headers) else {
        return json_error_response(StatusCode::UNAUTHORIZED, "IAM session token is required.");
    };

    json_response(session)
}

async fn delete_current_session(
    State(state): State<RuntimeProxyState>,
    headers: HeaderMap,
) -> Response {
    if let Some(token) = session_token_from_headers(&headers) {
        state.local_iam.delete_session_for_token(token.as_str());
    }

    json_response(json!({ "success": true }))
}

async fn refresh_auth_session(State(state): State<RuntimeProxyState>, body: Bytes) -> Response {
    let input = match parse_json_body(body) {
        Ok(input) => input,
        Err(response) => return response,
    };
    let Some(refresh_token) = string_field(&input, &["refreshToken"]) else {
        return json_error_response(StatusCode::BAD_REQUEST, "refreshToken is required.");
    };

    let Some(session) = state.local_iam.session_for_token(refresh_token.as_str()) else {
        return json_error_response(StatusCode::UNAUTHORIZED, "refreshToken is not recognized.");
    };

    json_response(session)
}

async fn get_current_user(State(state): State<RuntimeProxyState>, headers: HeaderMap) -> Response {
    let Some(session) = resolve_session_from_headers(&state, &headers) else {
        return json_error_response(StatusCode::UNAUTHORIZED, "IAM session token is required.");
    };

    json_response(session["user"].clone())
}

async fn list_organizations(Query(query): Query<HashMap<String, String>>) -> Response {
    json_response(json!({
        "items": filter_items(local_organizations(), &query, &[
            ("tenantId", "tenantId"),
            ("parentOrganizationId", "parentOrganizationId"),
        ]),
    }))
}

async fn get_organization_tree() -> Response {
    json_response(json!({
        "items": local_organization_tree(),
    }))
}

async fn list_organization_memberships(Query(query): Query<HashMap<String, String>>) -> Response {
    json_response(json!({
        "items": filter_items(local_organization_memberships(), &query, &[
            ("tenantId", "tenantId"),
            ("organizationId", "organizationId"),
            ("userId", "userId"),
        ]),
    }))
}

async fn list_departments(Query(query): Query<HashMap<String, String>>) -> Response {
    json_response(json!({
        "items": filter_items(local_departments(), &query, &[
            ("tenantId", "tenantId"),
            ("organizationId", "organizationId"),
            ("parentDepartmentId", "parentDepartmentId"),
        ]),
    }))
}

async fn get_department_tree(Query(query): Query<HashMap<String, String>>) -> Response {
    let organization_id = query.get("organizationId").map(String::as_str);
    json_response(json!({
        "items": local_department_tree(organization_id),
    }))
}

async fn list_department_assignments(Query(query): Query<HashMap<String, String>>) -> Response {
    json_response(json!({
        "items": filter_items(local_department_assignments(), &query, &[
            ("tenantId", "tenantId"),
            ("organizationId", "organizationId"),
            ("departmentId", "departmentId"),
            ("userId", "userId"),
        ]),
    }))
}

async fn list_positions(Query(query): Query<HashMap<String, String>>) -> Response {
    json_response(json!({
        "items": filter_items(local_positions(), &query, &[
            ("tenantId", "tenantId"),
            ("organizationId", "organizationId"),
            ("departmentId", "departmentId"),
        ]),
    }))
}

async fn list_position_assignments(Query(query): Query<HashMap<String, String>>) -> Response {
    json_response(json!({
        "items": filter_items(local_position_assignments(), &query, &[
            ("tenantId", "tenantId"),
            ("departmentAssignmentId", "departmentAssignmentId"),
            ("userId", "userId"),
        ]),
    }))
}

async fn list_role_bindings(Query(query): Query<HashMap<String, String>>) -> Response {
    json_response(json!({
        "items": filter_items(local_role_bindings(), &query, &[
            ("tenantId", "tenantId"),
            ("scopeKind", "scopeKind"),
            ("scopeId", "scopeId"),
            ("principalId", "principalId"),
        ]),
    }))
}

async fn get_iam_runtime() -> Response {
    json_response(json!({
        "appId": LOCAL_APP_ID,
        "environment": "dev",
        "deploymentMode": "local",
        "oauthProviders": [],
        "registrationEnabled": true,
        "passwordResetEnabled": true,
        "qrAuthEnabled": true,
    }))
}

fn local_organizations() -> Vec<Value> {
    vec![
        json!({
            "organizationId": "sdkwork-local-group",
            "tenantId": LOCAL_TENANT_ID,
            "name": "SDKWork Local Group",
            "code": "SDKWORK-GROUP",
            "parentOrganizationId": null,
            "organizationKind": "group",
            "tenantBoundaryKind": "root_tenant",
            "dataBoundaryKind": "tenant_shared",
            "appBoundaryEnabled": true,
            "verificationStatus": "verified",
            "status": "active",
            "order": 0,
            "path": "/sdkwork-local-group",
        }),
        json!({
            "organizationId": LOCAL_ORGANIZATION_ID,
            "tenantId": LOCAL_TENANT_ID,
            "name": "Craw Chat Local Company",
            "code": "CRAW-CHAT-LOCAL",
            "parentOrganizationId": "sdkwork-local-group",
            "organizationKind": "company",
            "tenantBoundaryKind": "sub_tenant",
            "dataBoundaryKind": "organization_isolated",
            "appBoundaryEnabled": true,
            "verificationStatus": "verified",
            "status": "active",
            "order": 10,
            "path": "/sdkwork-local-group/sdkwork-local-org",
        }),
    ]
}

fn local_organization_tree() -> Vec<Value> {
    vec![json!({
        "organizationId": "sdkwork-local-group",
        "tenantId": LOCAL_TENANT_ID,
        "name": "SDKWork Local Group",
        "code": "SDKWORK-GROUP",
        "parentOrganizationId": null,
        "organizationKind": "group",
        "tenantBoundaryKind": "root_tenant",
        "dataBoundaryKind": "tenant_shared",
        "appBoundaryEnabled": true,
        "verificationStatus": "verified",
        "status": "active",
        "order": 0,
        "path": "/sdkwork-local-group",
        "children": [
            {
                "organizationId": LOCAL_ORGANIZATION_ID,
                "tenantId": LOCAL_TENANT_ID,
                "name": "Craw Chat Local Company",
                "code": "CRAW-CHAT-LOCAL",
                "parentOrganizationId": "sdkwork-local-group",
                "organizationKind": "company",
                "tenantBoundaryKind": "sub_tenant",
                "dataBoundaryKind": "organization_isolated",
                "appBoundaryEnabled": true,
                "verificationStatus": "verified",
                "status": "active",
                "order": 10,
                "path": "/sdkwork-local-group/sdkwork-local-org",
                "children": [],
            }
        ],
    })]
}

fn local_departments() -> Vec<Value> {
    vec![
        json!({
            "departmentId": "dept-root",
            "tenantId": LOCAL_TENANT_ID,
            "organizationId": LOCAL_ORGANIZATION_ID,
            "name": "Company Headquarters",
            "code": "HQ",
            "parentDepartmentId": null,
            "departmentKind": "headquarters",
            "status": "active",
            "order": 0,
            "path": "/dept-root",
            "costCenterCode": "HQ-000",
            "managerMembershipId": "membership-local-default",
        }),
        json!({
            "departmentId": "dept-product",
            "tenantId": LOCAL_TENANT_ID,
            "organizationId": LOCAL_ORGANIZATION_ID,
            "name": "Product",
            "code": "PRODUCT",
            "parentDepartmentId": "dept-root",
            "departmentKind": "department",
            "status": "active",
            "order": 10,
            "path": "/dept-root/dept-product",
            "costCenterCode": "PD-010",
        }),
        json!({
            "departmentId": "dept-support",
            "tenantId": LOCAL_TENANT_ID,
            "organizationId": LOCAL_ORGANIZATION_ID,
            "name": "Support",
            "code": "SUPPORT",
            "parentDepartmentId": "dept-root",
            "departmentKind": "department",
            "status": "active",
            "order": 20,
            "path": "/dept-root/dept-support",
            "costCenterCode": "SP-020",
        }),
    ]
}

fn local_department_tree(organization_id: Option<&str>) -> Vec<Value> {
    if let Some(organization_id) = organization_id {
        if organization_id != LOCAL_ORGANIZATION_ID {
            return Vec::new();
        }
    }

    vec![json!({
        "departmentId": "dept-root",
        "tenantId": LOCAL_TENANT_ID,
        "organizationId": LOCAL_ORGANIZATION_ID,
        "name": "Company Headquarters",
        "code": "HQ",
        "parentDepartmentId": null,
        "departmentKind": "headquarters",
        "status": "active",
        "order": 0,
        "path": "/dept-root",
        "children": [
            {
                "departmentId": "dept-product",
                "tenantId": LOCAL_TENANT_ID,
                "organizationId": LOCAL_ORGANIZATION_ID,
                "name": "Product",
                "code": "PRODUCT",
                "parentDepartmentId": "dept-root",
                "departmentKind": "department",
                "status": "active",
                "order": 10,
                "path": "/dept-root/dept-product",
                "children": [],
            },
            {
                "departmentId": "dept-support",
                "tenantId": LOCAL_TENANT_ID,
                "organizationId": LOCAL_ORGANIZATION_ID,
                "name": "Support",
                "code": "SUPPORT",
                "parentDepartmentId": "dept-root",
                "departmentKind": "department",
                "status": "active",
                "order": 20,
                "path": "/dept-root/dept-support",
                "children": [],
            }
        ],
    })]
}

fn local_organization_memberships() -> Vec<Value> {
    vec![json!({
        "membershipId": "membership-local-default",
        "tenantId": LOCAL_TENANT_ID,
        "organizationId": LOCAL_ORGANIZATION_ID,
        "userId": LOCAL_DEFAULT_USER_ID,
        "displayName": LOCAL_DEFAULT_DISPLAY_NAME,
        "username": LOCAL_DEFAULT_USERNAME,
        "email": LOCAL_DEFAULT_USERNAME,
        "phone": LOCAL_DEFAULT_PHONE,
        "avatarUrl": null,
        "membershipType": "employee",
        "status": "active",
        "primary": true,
        "joinedAt": "2026-06-03T00:00:00Z",
    })]
}

fn local_department_assignments() -> Vec<Value> {
    vec![json!({
        "assignmentId": "assignment-local-default-product",
        "membershipId": "membership-local-default",
        "tenantId": LOCAL_TENANT_ID,
        "organizationId": LOCAL_ORGANIZATION_ID,
        "departmentId": "dept-product",
        "userId": LOCAL_DEFAULT_USER_ID,
        "displayName": LOCAL_DEFAULT_DISPLAY_NAME,
        "username": LOCAL_DEFAULT_USERNAME,
        "email": LOCAL_DEFAULT_USERNAME,
        "phone": LOCAL_DEFAULT_PHONE,
        "avatarUrl": null,
        "assignmentType": "primary",
        "status": "active",
        "positionId": "position-product-owner",
        "positionName": "Product Owner",
        "roleCodes": ["org.member", "department.product_owner"],
        "startedAt": "2026-06-03T00:00:00Z",
    })]
}

fn local_positions() -> Vec<Value> {
    vec![json!({
        "positionId": "position-product-owner",
        "tenantId": LOCAL_TENANT_ID,
        "organizationId": LOCAL_ORGANIZATION_ID,
        "departmentId": "dept-product",
        "name": "Product Owner",
        "code": "PRODUCT_OWNER",
        "positionKind": "job",
        "status": "active",
        "level": "P5",
    })]
}

fn local_position_assignments() -> Vec<Value> {
    vec![json!({
        "positionAssignmentId": "position-assignment-local-default-product-owner",
        "departmentAssignmentId": "assignment-local-default-product",
        "positionId": "position-product-owner",
        "tenantId": LOCAL_TENANT_ID,
        "organizationId": LOCAL_ORGANIZATION_ID,
        "departmentId": "dept-product",
        "userId": LOCAL_DEFAULT_USER_ID,
        "positionName": "Product Owner",
        "status": "active",
    })]
}

fn local_role_bindings() -> Vec<Value> {
    vec![
        json!({
            "roleBindingId": "role-binding-local-org-admin",
            "tenantId": LOCAL_TENANT_ID,
            "roleCode": "organization.admin",
            "principalKind": "organization_membership",
            "principalId": "membership-local-default",
            "scopeKind": "organization",
            "scopeId": LOCAL_ORGANIZATION_ID,
            "status": "active",
            "expiresAt": null,
        }),
        json!({
            "roleBindingId": "role-binding-local-product-owner",
            "tenantId": LOCAL_TENANT_ID,
            "roleCode": "department.product_owner",
            "principalKind": "department_assignment",
            "principalId": "assignment-local-default-product",
            "scopeKind": "department",
            "scopeId": "dept-product",
            "status": "active",
            "expiresAt": null,
        }),
    ]
}

fn filter_items(items: Vec<Value>, query: &HashMap<String, String>, keys: &[(&str, &str)]) -> Vec<Value> {
    items
        .into_iter()
        .filter(|item| {
            keys.iter().all(|(query_key, field_key)| {
                let Some(expected) = query.get(*query_key).map(String::as_str).map(str::trim).filter(|value| !value.is_empty()) else {
                    return true;
                };
                item.get(*field_key)
                    .and_then(Value::as_str)
                    .map(|actual| actual == expected)
                    .unwrap_or(false)
            })
        })
        .collect()
}

async fn get_iam_verification_policy() -> Response {
    json_response(json!({
        "password": true,
        "email": false,
        "phone": false,
        "captcha": false,
        "mfa": false,
    }))
}

async fn create_verification_code(body: Bytes) -> Response {
    let input = match parse_json_body(body) {
        Ok(input) => input,
        Err(response) => return response,
    };
    let target = string_field(&input, &["target"]).unwrap_or_else(|| LOCAL_DEFAULT_USERNAME.into());

    json_response(json!({
        "codeId": format!("local-code-{:032x}", random::<u128>()),
        "expiresInSeconds": 300,
        "maskedTarget": mask_target(target.as_str()),
    }))
}

async fn verify_verification_code(body: Bytes) -> Response {
    let input = match parse_json_body(body) {
        Ok(input) => input,
        Err(response) => return response,
    };
    let verified = string_field(&input, &["code"])
        .map(|code| code == LOCAL_VERIFY_CODE)
        .unwrap_or(false);

    json_response(json!({
        "verified": verified,
        "verificationToken": if verified {
            format!("local-verification-{:032x}", random::<u128>())
        } else {
            String::new()
        },
    }))
}

async fn create_password_reset_request(body: Bytes) -> Response {
    let input = parse_json_body(body).unwrap_or_else(|_| json!({}));
    let target = string_field(&input, &["target", "username", "email", "phone"])
        .unwrap_or_else(|| LOCAL_DEFAULT_USERNAME.into());

    json_response(json!({
        "requestId": format!("local-reset-{:032x}", random::<u128>()),
        "expiresInSeconds": 300,
        "maskedTarget": mask_target(target.as_str()),
    }))
}

async fn create_password_reset(body: Bytes) -> Response {
    let input = match parse_json_body(body) {
        Ok(input) => input,
        Err(response) => return response,
    };
    if let Err(response) = validate_password_input(&input) {
        return response;
    }

    json_response(json!({ "success": true }))
}

async fn get_oauth_authorization_url() -> Response {
    json_response(json!({
        "authorizationUrl": "sdkwork-local-iam://oauth/authorization",
        "provider": "local",
        "state": format!("local-oauth-{:032x}", random::<u128>()),
    }))
}

async fn create_qr_auth_session(State(state): State<RuntimeProxyState>, body: Bytes) -> Response {
    let input = parse_json_body(body).unwrap_or_else(|_| json!({}));
    json_response(state.local_iam.create_qr_session(&input))
}

async fn get_qr_auth_session(
    State(state): State<RuntimeProxyState>,
    Path(session_key): Path<String>,
) -> Response {
    match state.local_iam.get_qr_session(session_key.as_str()) {
        Some(session) => json_response(session),
        None => json_error_response(StatusCode::NOT_FOUND, "QR auth session was not found."),
    }
}

async fn create_qr_auth_scan(
    State(state): State<RuntimeProxyState>,
    Path(session_key): Path<String>,
    _body: Bytes,
) -> Response {
    match state.local_iam.mark_qr_scanned(session_key.as_str()) {
        Some(session) => json_response(session),
        None => json_error_response(StatusCode::NOT_FOUND, "QR auth session was not found."),
    }
}

async fn create_qr_auth_password(
    State(state): State<RuntimeProxyState>,
    Path(session_key): Path<String>,
    body: Bytes,
) -> Response {
    let input = match parse_json_body(body) {
        Ok(input) => input,
        Err(response) => return response,
    };
    if let Err(response) = validate_password_input(&input) {
        return response;
    }

    match state
        .local_iam
        .confirm_qr_with_password(session_key.as_str(), &input)
    {
        Some(session) => json_response(session),
        None => json_error_response(StatusCode::NOT_FOUND, "QR auth session was not found."),
    }
}

fn parse_json_body(body: Bytes) -> Result<Value, Response> {
    if body.is_empty() {
        return Ok(json!({}));
    }

    serde_json::from_slice::<Value>(&body).map_err(|error| {
        json_error_response(
            StatusCode::BAD_REQUEST,
            format!("request body must be valid JSON: {error}").as_str(),
        )
    })
}

fn validate_password_input(input: &Value) -> Result<(), Response> {
    let Some(password) = string_field(input, &["password"]) else {
        return Err(json_error_response(
            StatusCode::BAD_REQUEST,
            "password is required.",
        ));
    };
    if password.is_empty() {
        return Err(json_error_response(
            StatusCode::BAD_REQUEST,
            "password is required.",
        ));
    }
    if let Some(confirm_password) = string_field(input, &["confirmPassword"]) {
        if confirm_password != password {
            return Err(json_error_response(
                StatusCode::BAD_REQUEST,
                "confirmPassword must match password.",
            ));
        }
    }

    Ok(())
}

fn resolve_session_from_headers(state: &RuntimeProxyState, headers: &HeaderMap) -> Option<Value> {
    if let Some(token) = session_token_from_headers(headers) {
        if let Some(session) = state.local_iam.session_for_token(token.as_str()) {
            return Some(session);
        }
    }

    headers
        .get("x-sdkwork-session-id")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| state.local_iam.session_for_id(value.trim()))
}

fn session_token_from_headers(headers: &HeaderMap) -> Option<String> {
    for name in ["access-token", "auth-token"] {
        if let Some(token) = headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return Some(token.to_owned());
        }
    }

    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())?
        .trim();
    if authorization.len() >= 7 && authorization[..7].eq_ignore_ascii_case("bearer ") {
        return Some(authorization[7..].trim().to_owned());
    }

    None
}

fn build_auth_session(input: &Value) -> Value {
    let session_id = format!("local-session-{:032x}", random::<u128>());
    let user_id = user_id_for_input(input);
    let user = build_user(input, user_id.as_str());
    let device_id = string_field(input, &["deviceId"]);

    json!({
        "accessToken": format!("local-access-{session_id}"),
        "authToken": format!("local-auth-{session_id}"),
        "refreshToken": format!("local-refresh-{session_id}"),
        "expiresAt": LOCAL_EXPIRES_AT,
        "sessionId": session_id,
        "context": {
            "appId": LOCAL_APP_ID,
            "tenantId": LOCAL_TENANT_ID,
            "organizationId": LOCAL_ORGANIZATION_ID,
            "userId": user_id,
            "sessionId": session_id,
            "environment": "dev",
            "deploymentMode": "local",
            "authLevel": "password",
            "dataScope": ["local"],
            "permissionScope": ["*"],
            "actorId": user_id,
            "actorKind": "user",
            "deviceId": device_id,
        },
        "user": user,
        "userInfo": user,
    })
}

fn build_user(input: &Value, user_id: &str) -> Value {
    let username = string_field(input, &["username", "email", "phone"])
        .unwrap_or_else(|| LOCAL_DEFAULT_USERNAME.into());
    let display_name = string_field(input, &["name", "displayName", "nickname"])
        .unwrap_or_else(|| LOCAL_DEFAULT_DISPLAY_NAME.into());
    let email = string_field(input, &["email"]).or_else(|| {
        if username.contains('@') {
            Some(username.clone())
        } else {
            None
        }
    });
    let phone = string_field(input, &["phone"]).or_else(|| {
        if username.chars().all(|ch| ch.is_ascii_digit()) {
            Some(username.clone())
        } else {
            Some(LOCAL_DEFAULT_PHONE.into())
        }
    });

    json!({
        "id": user_id,
        "userId": user_id,
        "username": username,
        "displayName": display_name,
        "name": display_name,
        "nickname": display_name,
        "email": email,
        "phone": phone,
        "avatar": {},
    })
}

fn user_id_for_input(input: &Value) -> String {
    let username = string_field(input, &["username", "email", "phone"])
        .unwrap_or_else(|| LOCAL_DEFAULT_USERNAME.into());
    if username == LOCAL_DEFAULT_USERNAME || username == LOCAL_DEFAULT_PHONE {
        return LOCAL_DEFAULT_USER_ID.into();
    }

    let numeric_id =
        LOCAL_PUBLIC_USER_ID_MIN + stable_hash(username.as_bytes()) % LOCAL_PUBLIC_USER_ID_RANGE;
    format!("U{numeric_id:010}")
}

fn stable_hash(value: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in value {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn string_field(input: &Value, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        input
            .get(*name)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
    })
}

fn mask_target(target: &str) -> String {
    if let Some((name, domain)) = target.split_once('@') {
        let first = name.chars().next().unwrap_or('*');
        return format!("{first}***@{domain}");
    }
    if target.len() > 4 {
        return format!("{}****{}", &target[..3], &target[target.len() - 2..]);
    }
    "****".into()
}

fn local_qr_fallback_url(session_key: &str, purpose: &str) -> String {
    format!(
        "https://127.0.0.1:3900/auth/qr/{session_key}?session_key={session_key}&purpose={purpose}&scan_source=browser"
    )
}

fn ok_envelope(data: Value) -> Value {
    json!({
        "code": "2000",
        "msg": "SUCCESS",
        "data": data,
    })
}

fn json_response(value: Value) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(axum::body::Body::from(ok_envelope(value).to_string()))
        .expect("local IAM JSON response should build")
}
