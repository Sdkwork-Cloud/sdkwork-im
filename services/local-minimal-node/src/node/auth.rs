use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use hex::{decode as hex_decode, encode as hex_encode};
use im_auth_context::{AuthContext, PUBLIC_BEARER_HS256_SECRET_ENV, encode_hs256_bearer_token};
use pbkdf2::pbkdf2_hmac_array;
use rand::{RngCore, rngs::OsRng};
use serde::de::DeserializeOwned;
use serde_json::json;
use subtle::ConstantTimeEq;

use super::*;

const ACCESS_TOKEN_TTL_SECONDS: u64 = 60 * 60;
const REFRESH_TOKEN_TTL_SECONDS: u64 = 60 * 60 * 24 * 30;
const PASSWORD_ITERATIONS: u32 = 120_000;
const AUTH_MAX_BINDING_ID_BYTES: usize = 256;
const CLIENT_KIND_IM_USER: &str = "im_user";
const CLIENT_KIND_PORTAL_OPERATOR: &str = "portal_operator";
const TOKEN_ISSUER: &str = "craw-chat";
const TOKEN_AUDIENCE: &str = "craw-chat-public";

#[derive(Clone)]
pub(super) struct AuthRuntime {
    store: Arc<Mutex<AuthStore>>,
}

struct AuthStore {
    accounts: Vec<AuthAccountRecord>,
    refresh_sessions: Vec<AuthRefreshSessionRecord>,
    accounts_path: Option<PathBuf>,
    refresh_sessions_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthAccountRecord {
    tenant_id: String,
    account_id: String,
    login: String,
    client_kind: String,
    actor_id: String,
    actor_kind: String,
    name: String,
    role: String,
    email: String,
    password_hash: String,
    password_salt: String,
    password_iterations: u32,
    permissions: Vec<String>,
    disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthRefreshSessionRecord {
    refresh_token: String,
    tenant_id: String,
    account_id: String,
    actor_id: String,
    client_kind: String,
    session_id: String,
    device_id: String,
    expires_at: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LoginRequest {
    tenant_id: String,
    login: String,
    password: String,
    device_id: Option<String>,
    session_id: Option<String>,
    client_kind: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RefreshRequest {
    refresh_token: String,
    device_id: Option<String>,
    session_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LoginResponse {
    access_token: String,
    refresh_token: String,
    expires_at: u64,
    user: AuthUserView,
    workspace: Option<Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct MeResponse {
    tenant_id: String,
    user: AuthUserView,
    workspace: Option<Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AuthUserView {
    id: String,
    login: String,
    name: String,
    role: String,
    email: String,
    actor_kind: String,
    client_kind: String,
    permissions: Vec<String>,
}

struct SeedAccountDefinition {
    tenant_id: &'static str,
    account_id: &'static str,
    login: &'static str,
    password: &'static str,
    client_kind: &'static str,
    actor_id: &'static str,
    actor_kind: &'static str,
    name: &'static str,
    role: &'static str,
    email: &'static str,
    permissions: &'static [&'static str],
}

const SEEDED_ACCOUNTS: &[SeedAccountDefinition] = &[
    SeedAccountDefinition {
        tenant_id: "t_demo",
        account_id: "acct_demo_owner",
        login: "u_owner",
        password: "Owner#2026",
        client_kind: CLIENT_KIND_IM_USER,
        actor_id: "u_owner",
        actor_kind: "user",
        name: "Owner Demo",
        role: "Conversation Owner",
        email: "owner@nebula-commerce.example",
        permissions: &[
            "conversation.*",
            "realtime.*",
            "rtc.*",
            "media.*",
            "stream.*",
        ],
    },
    SeedAccountDefinition {
        tenant_id: "t_demo",
        account_id: "acct_demo_guest",
        login: "u_guest",
        password: "Guest#2026",
        client_kind: CLIENT_KIND_IM_USER,
        actor_id: "u_guest",
        actor_kind: "user",
        name: "Guest Demo",
        role: "Conversation Guest",
        email: "guest@nebula-commerce.example",
        permissions: &[
            "conversation.read",
            "realtime.read",
            "rtc.read",
            "media.read",
        ],
    },
    SeedAccountDefinition {
        tenant_id: "t_demo",
        account_id: "acct_demo_user",
        login: "u_demo",
        password: "Demo#2026",
        client_kind: CLIENT_KIND_IM_USER,
        actor_id: "u_demo",
        actor_kind: "user",
        name: "Demo User",
        role: "Demo Operator",
        email: "demo@nebula-commerce.example",
        permissions: &[
            "conversation.*",
            "realtime.*",
            "rtc.*",
            "media.*",
            "stream.*",
        ],
    },
    SeedAccountDefinition {
        tenant_id: "t_demo",
        account_id: "acct_portal_demo",
        login: "ops_demo",
        password: "Portal#2026",
        client_kind: CLIENT_KIND_PORTAL_OPERATOR,
        actor_id: "ops_demo",
        actor_kind: "user",
        name: "Lin Tao",
        role: "Tenant Operations Lead",
        email: "lin.tao@nebula-commerce.example",
        permissions: &["portal.access", "portal.read", "ops.read", "audit.read"],
    },
];

impl AuthRuntime {
    pub(super) fn new(runtime_dir: Option<PathBuf>) -> Self {
        let (accounts_path, refresh_sessions_path) = auth_store_paths(runtime_dir.as_deref());
        let (accounts_path, mut accounts) =
            load_managed_auth_store::<AuthAccountRecord>(accounts_path, "auth accounts");
        let (refresh_sessions_path, mut refresh_sessions) =
            load_managed_auth_store::<AuthRefreshSessionRecord>(
                refresh_sessions_path,
                "auth refresh sessions",
            );
        prune_expired_refresh_sessions(&mut refresh_sessions, current_unix_epoch_seconds());

        seed_accounts(&mut accounts);
        let store = AuthStore {
            accounts,
            refresh_sessions,
            accounts_path,
            refresh_sessions_path,
        };
        if let Err(error) = persist_store(&store) {
            eprintln!("failed to persist seeded auth store during bootstrap: {error}");
        }

        Self {
            store: Arc::new(Mutex::new(store)),
        }
    }

    pub(super) fn login(&self, request: LoginRequest) -> Result<LoginResponse, ApiError> {
        let tenant_id = required_trimmed(
            request.tenant_id.as_str(),
            "auth_login_invalid",
            "tenantId is required",
        )?
        .to_owned();
        let login = required_trimmed(
            request.login.as_str(),
            "auth_login_invalid",
            "login is required",
        )?
        .to_owned();
        if request.password.trim().is_empty() {
            return Err(ApiError::bad_request(
                "auth_login_invalid",
                "password is required",
            ));
        }
        let password = request.password.as_str();
        let client_kind = resolve_client_kind(request.client_kind.as_deref())?;
        let device_id =
            validate_optional_auth_binding_id("deviceId", request.device_id.as_deref())?;
        let session_id =
            validate_optional_auth_binding_id("sessionId", request.session_id.as_deref())?;
        let mut store = self.lock_store()?;

        let account = store
            .accounts
            .iter()
            .find(|candidate| {
                candidate.tenant_id == tenant_id
                    && candidate.client_kind == client_kind
                    && candidate.login.eq_ignore_ascii_case(login.as_str())
            })
            .cloned()
            .ok_or_else(|| {
                ApiError::unauthorized("auth_login_invalid", "account login or password is invalid")
            })?;

        if account.disabled {
            return Err(ApiError::forbidden(
                "auth_account_disabled",
                "account is disabled",
            ));
        }

        if !verify_password(&account, password)? {
            return Err(ApiError::unauthorized(
                "auth_login_invalid",
                "account login or password is invalid",
            ));
        }

        self.issue_session(&mut store, &account, device_id, session_id)
    }

    pub(super) fn refresh(&self, request: RefreshRequest) -> Result<LoginResponse, ApiError> {
        let refresh_token = required_trimmed(
            request.refresh_token.as_str(),
            "auth_refresh_invalid",
            "refreshToken is required",
        )?
        .to_owned();
        let device_id =
            validate_optional_auth_binding_id("deviceId", request.device_id.as_deref())?;
        let session_id =
            validate_optional_auth_binding_id("sessionId", request.session_id.as_deref())?;
        let mut store = self.lock_store()?;
        let now = current_unix_epoch_seconds();
        let pruned_expired_sessions =
            prune_expired_refresh_sessions(&mut store.refresh_sessions, now);
        let session_index = match store
            .refresh_sessions
            .iter()
            .position(|candidate| candidate.refresh_token == refresh_token)
        {
            Some(index) => index,
            None => {
                if pruned_expired_sessions {
                    persist_store(&store).map_err(|error| {
                        ApiError::service_unavailable(
                            "auth_store_unavailable",
                            format!("failed to persist expired refresh token eviction: {error}"),
                        )
                    })?;
                }
                return Err(ApiError::unauthorized(
                    "auth_refresh_invalid",
                    "refresh token is invalid",
                ));
            }
        };
        let session = store.refresh_sessions.remove(session_index);
        if session.expires_at <= now {
            persist_store(&store).map_err(|error| {
                ApiError::service_unavailable(
                    "auth_store_unavailable",
                    format!("failed to persist expired refresh token eviction: {error}"),
                )
            })?;
            return Err(ApiError::unauthorized(
                "auth_refresh_expired",
                "refresh token is expired",
            ));
        }

        if let Some(device_id) = device_id
            && !device_id.is_empty()
            && device_id != session.device_id
        {
            persist_store(&store).map_err(|error| {
                ApiError::service_unavailable(
                    "auth_store_unavailable",
                    format!("failed to persist revoked refresh token state: {error}"),
                )
            })?;
            return Err(ApiError::unauthorized(
                "auth_session_revoked",
                "refresh token device binding is invalid",
            ));
        }

        if let Some(session_id) = session_id
            && !session_id.is_empty()
            && session_id != session.session_id
        {
            persist_store(&store).map_err(|error| {
                ApiError::service_unavailable(
                    "auth_store_unavailable",
                    format!("failed to persist revoked refresh token state: {error}"),
                )
            })?;
            return Err(ApiError::unauthorized(
                "auth_session_revoked",
                "refresh token session binding is invalid",
            ));
        }

        let account = store
            .accounts
            .iter()
            .find(|candidate| {
                candidate.tenant_id == session.tenant_id
                    && candidate.account_id == session.account_id
            })
            .cloned()
            .ok_or_else(|| {
                ApiError::unauthorized(
                    "auth_refresh_invalid",
                    "refresh token account could not be resolved",
                )
            })?;

        if account.disabled {
            persist_store(&store).map_err(|error| {
                ApiError::service_unavailable(
                    "auth_store_unavailable",
                    format!("failed to persist revoked refresh token state: {error}"),
                )
            })?;
            return Err(ApiError::forbidden(
                "auth_account_disabled",
                "account is disabled",
            ));
        }

        self.issue_session(
            &mut store,
            &account,
            Some(session.device_id.as_str()),
            Some(session.session_id.as_str()),
        )
    }

    pub(super) fn me(
        &self,
        auth: &AuthContext,
        client_kind_hint: Option<&str>,
    ) -> Result<MeResponse, ApiError> {
        let store = self.lock_store()?;
        let account = store
            .accounts
            .iter()
            .find(|candidate| {
                candidate.tenant_id == auth.tenant_id
                    && candidate.actor_id == auth.actor_id
                    && account_matches_auth_context(candidate, auth, client_kind_hint)
            })
            .or_else(|| {
                store.accounts.iter().find(|candidate| {
                    candidate.tenant_id == auth.tenant_id && candidate.actor_id == auth.actor_id
                })
            })
            .cloned()
            .ok_or_else(|| {
                ApiError::unauthorized("auth_context_invalid", "auth subject could not be resolved")
            })?;

        Ok(MeResponse {
            tenant_id: account.tenant_id.clone(),
            user: account_user_view(&account),
            workspace: workspace_for_account(&account),
        })
    }

    fn issue_session(
        &self,
        store: &mut MutexGuard<'_, AuthStore>,
        account: &AuthAccountRecord,
        requested_device_id: Option<&str>,
        requested_session_id: Option<&str>,
    ) -> Result<LoginResponse, ApiError> {
        let device_id = optional_trimmed(requested_device_id)
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| default_device_id(account.actor_id.as_str()));
        let session_id = optional_trimmed(requested_session_id)
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| default_session_id(account.actor_id.as_str()));
        let now = current_unix_epoch_seconds();
        let expires_at = now + ACCESS_TOKEN_TTL_SECONDS;
        let access_token =
            issue_access_token(account, device_id.as_str(), session_id.as_str(), now)?;
        let refresh_token = generate_secret_token(32);

        prune_expired_refresh_sessions(&mut store.refresh_sessions, now);
        store.refresh_sessions.retain(|session| {
            !(session.tenant_id == account.tenant_id
                && session.account_id == account.account_id
                && session.device_id == device_id
                && session.session_id == session_id)
        });
        store.refresh_sessions.push(AuthRefreshSessionRecord {
            refresh_token: refresh_token.clone(),
            tenant_id: account.tenant_id.clone(),
            account_id: account.account_id.clone(),
            actor_id: account.actor_id.clone(),
            client_kind: account.client_kind.clone(),
            session_id,
            device_id,
            expires_at: now + REFRESH_TOKEN_TTL_SECONDS,
        });
        persist_store(store).map_err(|error| {
            ApiError::service_unavailable(
                "auth_store_unavailable",
                format!("failed to persist auth session state: {error}"),
            )
        })?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            expires_at,
            user: account_user_view(account),
            workspace: workspace_for_account(account),
        })
    }

    fn lock_store(&self) -> Result<MutexGuard<'_, AuthStore>, ApiError> {
        self.store.lock().map_err(|_| {
            ApiError::service_unavailable("auth_store_unavailable", "auth store lock is poisoned")
        })
    }
}

pub(super) async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    Ok(Json(state.auth_runtime.login(request)?))
}

pub(super) async fn refresh(
    State(state): State<AppState>,
    Json(request): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    Ok(Json(state.auth_runtime.refresh(request)?))
}

pub(super) async fn me(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MeResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let client_kind_hint = client_kind_from_headers(&headers);
    Ok(Json(
        state.auth_runtime.me(&auth, client_kind_hint.as_deref())?,
    ))
}

fn account_user_view(account: &AuthAccountRecord) -> AuthUserView {
    AuthUserView {
        id: account.actor_id.clone(),
        login: account.login.clone(),
        name: account.name.clone(),
        role: account.role.clone(),
        email: account.email.clone(),
        actor_kind: account.actor_kind.clone(),
        client_kind: account.client_kind.clone(),
        permissions: account.permissions.clone(),
    }
}

fn workspace_for_account(account: &AuthAccountRecord) -> Option<Value> {
    if account.client_kind == CLIENT_KIND_PORTAL_OPERATOR {
        Some(super::portal::workspace_snapshot())
    } else {
        None
    }
}

fn resolve_client_kind(client_kind: Option<&str>) -> Result<String, ApiError> {
    let value = optional_trimmed(client_kind).unwrap_or(CLIENT_KIND_IM_USER);
    match value {
        CLIENT_KIND_IM_USER | CLIENT_KIND_PORTAL_OPERATOR => Ok(value.to_owned()),
        _ => Err(ApiError::bad_request(
            "auth_client_kind_invalid",
            format!("unsupported clientKind: {value}"),
        )),
    }
}

fn issue_access_token(
    account: &AuthAccountRecord,
    device_id: &str,
    session_id: &str,
    now: u64,
) -> Result<String, ApiError> {
    let secret = std::env::var(PUBLIC_BEARER_HS256_SECRET_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ApiError::service_unavailable(
                "auth_signing_secret_missing",
                format!(
                    "public bearer signing secret is missing: {}",
                    PUBLIC_BEARER_HS256_SECRET_ENV
                ),
            )
        })?;
    let claims = json!({
        "tenant_id": account.tenant_id,
        "sub": account.actor_id,
        "actor_kind": account.actor_kind,
        "sid": session_id,
        "did": device_id,
        "client_kind": account.client_kind,
        "permissions": account.permissions,
        "iss": TOKEN_ISSUER,
        "aud": TOKEN_AUDIENCE,
        "iat": now,
        "exp": now + ACCESS_TOKEN_TTL_SECONDS
    });
    encode_hs256_bearer_token(&claims, secret.as_str()).map_err(|error| {
        ApiError::service_unavailable(
            "auth_token_issue_failed",
            format!("failed to issue access token: {error}"),
        )
    })
}

fn seed_accounts(accounts: &mut Vec<AuthAccountRecord>) {
    for definition in SEEDED_ACCOUNTS {
        if accounts
            .iter()
            .any(|candidate| candidate.account_id == definition.account_id)
        {
            continue;
        }

        let salt = random_bytes::<16>();
        let password_hash =
            derive_password_hash(definition.password, salt.as_slice(), PASSWORD_ITERATIONS);
        accounts.push(AuthAccountRecord {
            tenant_id: definition.tenant_id.into(),
            account_id: definition.account_id.into(),
            login: definition.login.into(),
            client_kind: definition.client_kind.into(),
            actor_id: definition.actor_id.into(),
            actor_kind: definition.actor_kind.into(),
            name: definition.name.into(),
            role: definition.role.into(),
            email: definition.email.into(),
            password_hash,
            password_salt: hex_encode(salt),
            password_iterations: PASSWORD_ITERATIONS,
            permissions: definition
                .permissions
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
            disabled: false,
        });
    }
}

fn verify_password(account: &AuthAccountRecord, password: &str) -> Result<bool, ApiError> {
    let salt = hex_decode(account.password_salt.as_str()).map_err(|error| {
        ApiError::service_unavailable(
            "auth_store_invalid",
            format!("stored password salt is invalid: {error}"),
        )
    })?;
    let expected_hash = hex_decode(account.password_hash.as_str()).map_err(|error| {
        ApiError::service_unavailable(
            "auth_store_invalid",
            format!("stored password hash is invalid: {error}"),
        )
    })?;
    let actual_hash = pbkdf2_hmac_array::<sha2::Sha256, 32>(
        password.as_bytes(),
        salt.as_slice(),
        account.password_iterations,
    );
    Ok(actual_hash
        .as_slice()
        .ct_eq(expected_hash.as_slice())
        .into())
}

fn derive_password_hash(password: &str, salt: &[u8], iterations: u32) -> String {
    let derived = pbkdf2_hmac_array::<sha2::Sha256, 32>(password.as_bytes(), salt, iterations);
    hex_encode(derived)
}

fn prune_expired_refresh_sessions(
    refresh_sessions: &mut Vec<AuthRefreshSessionRecord>,
    now: u64,
) -> bool {
    let original_len = refresh_sessions.len();
    refresh_sessions.retain(|session| session.expires_at > now);
    refresh_sessions.len() != original_len
}

fn validate_optional_auth_binding_id<'a>(
    field: &'static str,
    value: Option<&'a str>,
) -> Result<Option<&'a str>, ApiError> {
    let value = optional_trimmed(value);
    if let Some(value) = value {
        let actual_bytes = value.len();
        if actual_bytes > AUTH_MAX_BINDING_ID_BYTES {
            return Err(ApiError::payload_too_large(
                field,
                AUTH_MAX_BINDING_ID_BYTES,
                actual_bytes,
            ));
        }
    }

    Ok(value)
}

fn auth_store_paths(runtime_dir: Option<&Path>) -> (Option<PathBuf>, Option<PathBuf>) {
    let Some(runtime_dir) = runtime_dir else {
        return (None, None);
    };
    let state_dir = runtime_dir.join("state");
    if let Err(error) = fs::create_dir_all(&state_dir) {
        eprintln!(
            "failed to create auth state dir {}: {error}",
            state_dir.display()
        );
        return (None, None);
    }
    (
        Some(state_dir.join("auth-accounts.json")),
        Some(state_dir.join("auth-refresh-sessions.json")),
    )
}

fn load_managed_auth_store<T>(
    path: Option<PathBuf>,
    label: &'static str,
) -> (Option<PathBuf>, Vec<T>)
where
    T: DeserializeOwned,
{
    let Some(path) = path else {
        return (None, Vec::new());
    };

    if let Err(error) = recover_pending_json_temp_file(path.as_path()) {
        eprintln!(
            "failed to recover pending {label} temp file {}: {error}",
            path.display()
        );
        return (None, Vec::new());
    }

    match load_json_file::<Vec<T>>(Some(path.as_path())) {
        Ok(Some(value)) => (Some(path), value),
        Ok(None) => (Some(path), Vec::new()),
        Err(error) => {
            eprintln!("failed to load {label} {}: {error}", path.display());
            match quarantine_invalid_json_file(path.as_path()) {
                Ok(quarantine_path) => {
                    eprintln!(
                        "quarantined invalid {label} {} at {}",
                        path.display(),
                        quarantine_path.display()
                    );
                    (Some(path), Vec::new())
                }
                Err(quarantine_error) => {
                    eprintln!(
                        "failed to quarantine invalid {label} {}: {quarantine_error}",
                        path.display()
                    );
                    (None, Vec::new())
                }
            }
        }
    }
}

fn load_json_file<T>(path: Option<&Path>) -> Result<Option<T>, String>
where
    T: DeserializeOwned,
{
    let Some(path) = path else {
        return Ok(None);
    };
    if !path.is_file() {
        return Ok(None);
    }

    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    serde_json::from_str(&content)
        .map(Some)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn persist_store(store: &AuthStore) -> Result<(), String> {
    if let Some(path) = store.accounts_path.as_deref() {
        persist_json_file(path, &store.accounts)?;
    }
    if let Some(path) = store.refresh_sessions_path.as_deref() {
        persist_json_file(path, &store.refresh_sessions)?;
    }
    Ok(())
}

fn persist_json_file<T>(path: &Path, value: &T) -> Result<(), String>
where
    T: Serialize,
{
    let content = serde_json::to_string_pretty(value)
        .map_err(|error| format!("failed to serialize {}: {error}", path.display()))?;
    let temp_path = json_temp_path(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to prepare {}: {error}", parent.display()))?;
    }
    if temp_path.exists() {
        fs::remove_file(&temp_path).map_err(|error| {
            format!(
                "failed to clear stale temp file {}: {error}",
                temp_path.display()
            )
        })?;
    }

    let mut temp_file = File::create(&temp_path).map_err(|error| {
        format!(
            "failed to create temp file {}: {error}",
            temp_path.display()
        )
    })?;
    temp_file
        .write_all(content.as_bytes())
        .map_err(|error| format!("failed to write temp file {}: {error}", temp_path.display()))?;
    temp_file
        .sync_all()
        .map_err(|error| format!("failed to sync temp file {}: {error}", temp_path.display()))?;
    drop(temp_file);

    fs::rename(&temp_path, path).map_err(|error| {
        format!(
            "failed to replace {} with {}: {error}",
            path.display(),
            temp_path.display()
        )
    })
}

fn recover_pending_json_temp_file(path: &Path) -> Result<(), String> {
    let temp_path = json_temp_path(path);
    if !temp_path.exists() {
        return Ok(());
    }

    if path.exists() {
        return fs::remove_file(&temp_path).map_err(|error| {
            format!(
                "failed to remove stale temp file {}: {error}",
                temp_path.display()
            )
        });
    }

    fs::rename(&temp_path, path).map_err(|error| {
        format!(
            "failed to promote pending temp file {} to {}: {error}",
            temp_path.display(),
            path.display()
        )
    })
}

fn quarantine_invalid_json_file(path: &Path) -> Result<PathBuf, String> {
    let quarantine_path = unique_json_sidecar_path(path, "invalid");
    fs::rename(path, &quarantine_path).map_err(|error| {
        format!(
            "failed to quarantine invalid file {} to {}: {error}",
            path.display(),
            quarantine_path.display()
        )
    })?;
    Ok(quarantine_path)
}

fn json_temp_path(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "auth-store.json".into());
    path.with_file_name(format!("{file_name}.tmp"))
}

fn unique_json_sidecar_path(path: &Path, tag: &str) -> PathBuf {
    let file_name = path
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "auth-store.json".into());
    let mut attempt = current_unix_epoch_millis();
    loop {
        let candidate = path.with_file_name(format!("{file_name}.{tag}-{attempt}"));
        if !candidate.exists() {
            return candidate;
        }
        attempt += 1;
    }
}

fn default_device_id(actor_id: &str) -> String {
    format!("d_{}", sanitize_identifier(actor_id))
}

fn default_session_id(actor_id: &str) -> String {
    format!("s_{}", sanitize_identifier(actor_id))
}

fn sanitize_identifier(raw: &str) -> String {
    let mut sanitized = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
    }
    if sanitized.is_empty() {
        "user".into()
    } else {
        sanitized
    }
}

fn required_trimmed<'a>(
    value: &'a str,
    code: &'static str,
    message: &'static str,
) -> Result<&'a str, ApiError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ApiError::bad_request(code, message));
    }
    Ok(trimmed)
}

fn optional_trimmed(value: Option<&str>) -> Option<&str> {
    value
        .map(str::trim)
        .filter(|candidate| !candidate.is_empty())
}

fn account_matches_auth_context(
    account: &AuthAccountRecord,
    auth: &AuthContext,
    client_kind_hint: Option<&str>,
) -> bool {
    if account.actor_kind != auth.actor_kind {
        return false;
    }

    if let Some(client_kind_hint) = client_kind_hint
        && account.client_kind != client_kind_hint
    {
        return false;
    }

    if auth.permissions.is_empty() {
        return true;
    }

    account.permissions.len() == auth.permissions.len()
        && account
            .permissions
            .iter()
            .all(|permission| auth.permissions.contains(permission.as_str()))
}

fn client_kind_from_headers(headers: &HeaderMap) -> Option<String> {
    let authorization = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    let token = authorization
        .strip_prefix("Bearer ")
        .or_else(|| authorization.strip_prefix("bearer "))?;
    let payload_segment = token.split('.').nth(1)?;
    let payload = decode_base64url_segment(payload_segment).ok()?;
    let claims: Value = serde_json::from_slice(payload.as_slice()).ok()?;
    claims
        .get("client_kind")
        .or_else(|| claims.get("clientKind"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn decode_base64url_segment(input: &str) -> Result<Vec<u8>, String> {
    let mut output = Vec::with_capacity((input.len() * 3) / 4 + 3);
    let mut buffer = 0u32;
    let mut bits = 0u8;

    for byte in input.bytes() {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            b'=' => continue,
            _ => {
                return Err("jwt payload segment is not valid base64url".into());
            }
        } as u32;

        buffer = (buffer << 6) | value;
        bits += 6;

        while bits >= 8 {
            bits -= 8;
            output.push(((buffer >> bits) & 0xff) as u8);
        }
    }

    if bits > 0 && (buffer & ((1 << bits) - 1)) != 0 {
        return Err("jwt payload segment has invalid trailing bits".into());
    }

    Ok(output)
}

fn current_unix_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn current_unix_epoch_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn generate_secret_token(bytes: usize) -> String {
    let mut buffer = vec![0u8; bytes];
    OsRng.fill_bytes(buffer.as_mut_slice());
    hex_encode(buffer)
}

fn random_bytes<const N: usize>() -> [u8; N] {
    let mut buffer = [0u8; N];
    OsRng.fill_bytes(&mut buffer);
    buffer
}
