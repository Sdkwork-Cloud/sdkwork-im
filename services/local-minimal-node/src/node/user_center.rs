use std::sync::OnceLock;

use axum::body::Body;
use axum::http::HeaderMap;
use axum::http::HeaderValue;
use axum::http::header::AUTHORIZATION;
use axum::http::header::CONTENT_TYPE;
use axum::response::Response;
use serde::Serialize;
use serde_json::json;

use super::*;

const USER_CENTER_MODE_ENV: &str = "SDKWORK_USER_CENTER_MODE";
const USER_CENTER_MODE_ALIAS_ENV: &str = "CRAW_CHAT_USER_CENTER_MODE";
const USER_CENTER_LOCAL_API_BASE_PATH_ENV: &str = "SDKWORK_USER_CENTER_LOCAL_API_BASE_PATH";
const USER_CENTER_LOCAL_API_BASE_PATH_ALIAS_ENV: &str = "CRAW_CHAT_USER_CENTER_LOCAL_API_BASE_PATH";
const USER_CENTER_PROVIDER_KEY_ENV: &str = "SDKWORK_USER_CENTER_PROVIDER_KEY";
const USER_CENTER_PROVIDER_KEY_ALIAS_ENV: &str = "CRAW_CHAT_USER_CENTER_PROVIDER_KEY";
const USER_CENTER_AUTHORIZATION_HEADER_NAME_ENV: &str =
    "SDKWORK_USER_CENTER_AUTHORIZATION_HEADER_NAME";
const USER_CENTER_AUTHORIZATION_HEADER_NAME_ALIAS_ENV: &str =
    "CRAW_CHAT_USER_CENTER_AUTHORIZATION_HEADER_NAME";
const USER_CENTER_ACCESS_TOKEN_HEADER_NAME_ENV: &str =
    "SDKWORK_USER_CENTER_ACCESS_TOKEN_HEADER_NAME";
const USER_CENTER_ACCESS_TOKEN_HEADER_NAME_ALIAS_ENV: &str =
    "CRAW_CHAT_USER_CENTER_ACCESS_TOKEN_HEADER_NAME";
const USER_CENTER_REFRESH_TOKEN_HEADER_NAME_ENV: &str =
    "SDKWORK_USER_CENTER_REFRESH_TOKEN_HEADER_NAME";
const USER_CENTER_REFRESH_TOKEN_HEADER_NAME_ALIAS_ENV: &str =
    "CRAW_CHAT_USER_CENTER_REFRESH_TOKEN_HEADER_NAME";
const USER_CENTER_SESSION_HEADER_NAME_ENV: &str = "SDKWORK_USER_CENTER_SESSION_HEADER_NAME";
const USER_CENTER_SESSION_HEADER_NAME_ALIAS_ENV: &str = "CRAW_CHAT_USER_CENTER_SESSION_HEADER_NAME";
const USER_CENTER_AUTHORIZATION_SCHEME_ENV: &str = "SDKWORK_USER_CENTER_AUTHORIZATION_SCHEME";
const USER_CENTER_AUTHORIZATION_SCHEME_ALIAS_ENV: &str =
    "CRAW_CHAT_USER_CENTER_AUTHORIZATION_SCHEME";
const USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_ENV: &str =
    "SDKWORK_USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN";
const USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_ALIAS_ENV: &str =
    "CRAW_CHAT_USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN";
const USER_CENTER_APP_API_BASE_URL_ENV: &str = "SDKWORK_USER_CENTER_APP_API_BASE_URL";
const USER_CENTER_APP_API_BASE_URL_ALIAS_ENV: &str = "CRAW_CHAT_USER_CENTER_APP_API_BASE_URL";
const USER_CENTER_EXTERNAL_BASE_URL_ENV: &str = "SDKWORK_USER_CENTER_EXTERNAL_BASE_URL";
const USER_CENTER_EXTERNAL_BASE_URL_ALIAS_ENV: &str = "CRAW_CHAT_USER_CENTER_EXTERNAL_BASE_URL";
const USER_CENTER_APP_ID_ENV: &str = "SDKWORK_USER_CENTER_APP_ID";
const USER_CENTER_APP_ID_ALIAS_ENV: &str = "CRAW_CHAT_USER_CENTER_APP_ID";
const USER_CENTER_SECRET_ID_ENV: &str = "SDKWORK_USER_CENTER_SECRET_ID";
const USER_CENTER_SECRET_ID_ALIAS_ENV: &str = "CRAW_CHAT_USER_CENTER_SECRET_ID";
const USER_CENTER_SHARED_SECRET_ENV: &str = "SDKWORK_USER_CENTER_SHARED_SECRET";
const USER_CENTER_SHARED_SECRET_ALIAS_ENV: &str = "CRAW_CHAT_USER_CENTER_SHARED_SECRET";
const USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS_ENV: &
    str = "SDKWORK_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS";
const USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS_ALIAS_ENV: &
    str = "CRAW_CHAT_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS";

const USER_CENTER_DEFAULT_LOCAL_API_BASE_PATH: &str = "/api/app/v1/user-center";
const USER_CENTER_DEFAULT_PROVIDER_KEY: &str = "craw-chat-local";
const USER_CENTER_DEFAULT_AUTHORIZATION_HEADER_NAME: &str = "Authorization";
const USER_CENTER_DEFAULT_ACCESS_TOKEN_HEADER_NAME: &str = "Access-Token";
const USER_CENTER_DEFAULT_REFRESH_TOKEN_HEADER_NAME: &str = "Refresh-Token";
const USER_CENTER_DEFAULT_SESSION_HEADER_NAME: &str = "x-sdkwork-user-center-session-id";
const USER_CENTER_DEFAULT_AUTHORIZATION_SCHEME: &str = "Bearer";
const USER_CENTER_DEFAULT_APP_ID: &str = "craw-chat";
const USER_CENTER_DEFAULT_HANDSHAKE_FRESHNESS_WINDOW_MS: u64 = 30_000;

const USER_CENTER_STANDARD_APP_ID_HEADER_NAME: &str = "x-sdkwork-app-id";
const USER_CENTER_STANDARD_PROVIDER_KEY_HEADER_NAME: &str =
    "x-sdkwork-user-center-provider-key";
const USER_CENTER_STANDARD_HANDSHAKE_MODE_HEADER_NAME: &str =
    "x-sdkwork-user-center-handshake-mode";
const USER_CENTER_STANDARD_SECRET_ID_HEADER_NAME: &str = "x-sdkwork-user-center-secret-id";
const USER_CENTER_STANDARD_SIGNATURE_HEADER_NAME: &str = "x-sdkwork-user-center-signature";
const USER_CENTER_STANDARD_SIGNED_AT_HEADER_NAME: &str = "x-sdkwork-user-center-signed-at";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserCenterRuntimeMode {
    BuiltinLocal,
    SdkworkCloudAppApi,
    ExternalUserCenter,
}
impl UserCenterRuntimeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BuiltinLocal => "builtin-local",
            Self::SdkworkCloudAppApi => "sdkwork-cloud-app-api",
            Self::ExternalUserCenter => "external-user-center",
        }
    }

    pub fn is_remote(&self) -> bool {
        !matches!(self, Self::BuiltinLocal)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserCenterProviderKind {
    Local,
    SdkworkCloudAppApi,
    ExternalUserCenter,
}

impl UserCenterProviderKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::SdkworkCloudAppApi => "sdkwork-cloud-app-api",
            Self::ExternalUserCenter => "external-user-center",
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserCenterRuntimeConfig {
    pub access_token_header_name: String,
    pub allow_authorization_fallback_to_access_token: bool,
    pub app_api_base_url: Option<String>,
    pub app_id: String,
    pub authorization_header_name: String,
    pub authorization_scheme: String,
    pub external_base_url: Option<String>,
    pub handshake_freshness_window_ms: u64,
    pub local_api_base_path: String,
    pub mode: UserCenterRuntimeMode,
    pub provider_key: String,
    pub provider_kind: UserCenterProviderKind,
    pub refresh_token_header_name: String,
    pub secret_id: Option<String>,
    pub session_header_name: String,
    pub shared_secret: Option<String>,
}

pub fn resolve_user_center_runtime_config() -> Result<UserCenterRuntimeConfig, String> {
    let mode = resolve_user_center_mode()?;
    let app_api_base_url = read_runtime_text(&[
        USER_CENTER_APP_API_BASE_URL_ENV,
        USER_CENTER_APP_API_BASE_URL_ALIAS_ENV,
    ]);
    let external_base_url = read_runtime_text(&[
        USER_CENTER_EXTERNAL_BASE_URL_ENV,
        USER_CENTER_EXTERNAL_BASE_URL_ALIAS_ENV,
    ]);
    let provider_kind = match mode {
        UserCenterRuntimeMode::BuiltinLocal => UserCenterProviderKind::Local,
        UserCenterRuntimeMode::SdkworkCloudAppApi => UserCenterProviderKind::SdkworkCloudAppApi,
        UserCenterRuntimeMode::ExternalUserCenter => UserCenterProviderKind::ExternalUserCenter,
    };
    let provider_key = read_runtime_text(&[
        USER_CENTER_PROVIDER_KEY_ENV,
        USER_CENTER_PROVIDER_KEY_ALIAS_ENV,
    ])
    .unwrap_or_else(|| match provider_kind {
        UserCenterProviderKind::Local => USER_CENTER_DEFAULT_PROVIDER_KEY.into(),
        UserCenterProviderKind::SdkworkCloudAppApi => "craw-chat-app-api".into(),
        UserCenterProviderKind::ExternalUserCenter => "craw-chat-external".into(),
    });
    let local_api_base_path = normalize_api_base_path(
        read_runtime_text(&[
            USER_CENTER_LOCAL_API_BASE_PATH_ENV,
            USER_CENTER_LOCAL_API_BASE_PATH_ALIAS_ENV,
        ])
        .as_deref()
        .unwrap_or(USER_CENTER_DEFAULT_LOCAL_API_BASE_PATH),
    );
    let config = UserCenterRuntimeConfig {
        access_token_header_name: read_runtime_text(&[
            USER_CENTER_ACCESS_TOKEN_HEADER_NAME_ENV,
            USER_CENTER_ACCESS_TOKEN_HEADER_NAME_ALIAS_ENV,
        ])
        .unwrap_or_else(|| USER_CENTER_DEFAULT_ACCESS_TOKEN_HEADER_NAME.into()),
        allow_authorization_fallback_to_access_token: read_runtime_bool(&[
            USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_ENV,
            USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_ALIAS_ENV,
        ])
        .unwrap_or(true),
        app_api_base_url,
        app_id: read_runtime_text(&[USER_CENTER_APP_ID_ENV, USER_CENTER_APP_ID_ALIAS_ENV])
            .unwrap_or_else(|| USER_CENTER_DEFAULT_APP_ID.into()),
        authorization_header_name: read_runtime_text(&[
            USER_CENTER_AUTHORIZATION_HEADER_NAME_ENV,
            USER_CENTER_AUTHORIZATION_HEADER_NAME_ALIAS_ENV,
        ])
        .unwrap_or_else(|| USER_CENTER_DEFAULT_AUTHORIZATION_HEADER_NAME.into()),
        authorization_scheme: read_runtime_text(&[
            USER_CENTER_AUTHORIZATION_SCHEME_ENV,
            USER_CENTER_AUTHORIZATION_SCHEME_ALIAS_ENV,
        ])
        .unwrap_or_else(|| USER_CENTER_DEFAULT_AUTHORIZATION_SCHEME.into()),
        external_base_url,
        handshake_freshness_window_ms: read_runtime_u64(&[
            USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS_ENV,
            USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS_ALIAS_ENV,
        ])
        .unwrap_or(USER_CENTER_DEFAULT_HANDSHAKE_FRESHNESS_WINDOW_MS),
        local_api_base_path,
        mode,
        provider_key,
        provider_kind,
        refresh_token_header_name: read_runtime_text(&[
            USER_CENTER_REFRESH_TOKEN_HEADER_NAME_ENV,
            USER_CENTER_REFRESH_TOKEN_HEADER_NAME_ALIAS_ENV,
        ])
        .unwrap_or_else(|| USER_CENTER_DEFAULT_REFRESH_TOKEN_HEADER_NAME.into()),
        secret_id: read_runtime_text(&[USER_CENTER_SECRET_ID_ENV, USER_CENTER_SECRET_ID_ALIAS_ENV]),
        session_header_name: read_runtime_text(&[
            USER_CENTER_SESSION_HEADER_NAME_ENV,
            USER_CENTER_SESSION_HEADER_NAME_ALIAS_ENV,
        ])
        .unwrap_or_else(|| USER_CENTER_DEFAULT_SESSION_HEADER_NAME.into()),
        shared_secret: read_runtime_text(&[
            USER_CENTER_SHARED_SECRET_ENV,
            USER_CENTER_SHARED_SECRET_ALIAS_ENV,
        ]),
    };
    validate_runtime_config(&config)?;
    Ok(config)
}

pub(super) fn resolve_effective_user_center_runtime_config() -> UserCenterRuntimeConfig {
    resolve_user_center_runtime_config().unwrap_or_else(|_| UserCenterRuntimeConfig {
        access_token_header_name: USER_CENTER_DEFAULT_ACCESS_TOKEN_HEADER_NAME.into(),
        allow_authorization_fallback_to_access_token: true,
        app_api_base_url: None,
        app_id: USER_CENTER_DEFAULT_APP_ID.into(),
        authorization_header_name: USER_CENTER_DEFAULT_AUTHORIZATION_HEADER_NAME.into(),
        authorization_scheme: USER_CENTER_DEFAULT_AUTHORIZATION_SCHEME.into(),
        external_base_url: None,
        handshake_freshness_window_ms: USER_CENTER_DEFAULT_HANDSHAKE_FRESHNESS_WINDOW_MS,
        local_api_base_path: USER_CENTER_DEFAULT_LOCAL_API_BASE_PATH.into(),
        mode: UserCenterRuntimeMode::BuiltinLocal,
        provider_key: USER_CENTER_DEFAULT_PROVIDER_KEY.into(),
        provider_kind: UserCenterProviderKind::Local,
        refresh_token_header_name: USER_CENTER_DEFAULT_REFRESH_TOKEN_HEADER_NAME.into(),
        secret_id: None,
        session_header_name: USER_CENTER_DEFAULT_SESSION_HEADER_NAME.into(),
        shared_secret: None,
    })
}

pub(super) fn resolve_auth_context(headers: &HeaderMap) -> Result<AuthContext, ApiError> {
    let config = resolve_user_center_runtime_config()
        .map_err(|error| ApiError::service_unavailable("auth_authority_unavailable", error))?;
    let mut normalized_headers = headers.clone();

    if normalized_headers.get(AUTHORIZATION).is_none() {
        if let Some(value) = read_header_case_insensitive(
            headers,
            config.authorization_header_name.as_str(),
        ) {
            normalized_headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(value.as_str()).map_err(|_| {
                    ApiError::unauthorized(
                        "authorization_invalid",
                        "configured authorization header is not a valid header value",
                    )
                })?,
            );
        } else if config.allow_authorization_fallback_to_access_token {
            if let Some(access_token) = read_header_case_insensitive(
                headers,
                config.access_token_header_name.as_str(),
            ) {
                let authorization = format!(
                    "{} {}",
                    config.authorization_scheme.as_str(),
                    access_token.as_str()
                );
                normalized_headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(authorization.as_str()).map_err(|_| {
                        ApiError::unauthorized(
                            "authorization_invalid",
                            "configured access-token header is not a valid bearer token value",
                        )
                    })?,
                );
            }
        }
    }

    im_auth_context::resolve_auth_context(&normalized_headers).map_err(ApiError::from)
}

pub(super) async fn get_user_center_health() -> Json<Value> {
    match resolve_user_center_runtime_config() {
        Ok(config) => Json(json!({
            "status": "healthy",
            "mode": config.mode.as_str(),
            "providerKind": config.provider_kind.as_str(),
            "providerKey": config.provider_key,
            "localApiBasePath": config.local_api_base_path,
            "upstreamEnabled": config.mode.is_remote(),
        })),
        Err(error) => Json(json!({
            "status": "unavailable",
            "code": "auth_authority_unavailable",
            "message": error,
        })),
    }
}

pub(super) async fn proxy_json_request<T>(
    config: &UserCenterRuntimeConfig,
    method: &str,
    path: &str,
    headers: &HeaderMap,
    body: Option<&T>,
) -> Result<Response, ApiError>
where
    T: Serialize,
{
    let url = build_upstream_request_url(config, path)?;
    let reqwest_method = reqwest::Method::from_bytes(method.as_bytes()).map_err(|error| {
        ApiError::service_unavailable(
            "auth_authority_unavailable",
            format!("invalid upstream request method {method}: {error}"),
        )
    })?;
    let mut request = user_center_http_client().request(reqwest_method, url.as_str());
    for (name, value) in build_upstream_request_headers(config, method, path, headers) {
        request = request.header(name.as_str(), value.as_str());
    }
    if let Some(body) = body {
        request = request.json(body);
    }

    let response = request.send().await.map_err(|error| {
        ApiError::service_unavailable(
            "auth_authority_unavailable",
            format!("failed to reach upstream user-center authority {url}: {error}"),
        )
    })?;
    build_proxy_response(config, response).await
}

pub(super) fn login_path(config: &UserCenterRuntimeConfig) -> String {
    format!("{}/session/login", config.local_api_base_path)
}

pub(super) fn refresh_path(config: &UserCenterRuntimeConfig) -> String {
    format!("{}/session/refresh", config.local_api_base_path)
}

pub(super) fn profile_path(config: &UserCenterRuntimeConfig) -> String {
    format!("{}/profile", config.local_api_base_path)
}

pub(super) fn health_path(config: &UserCenterRuntimeConfig) -> String {
    format!("{}/health", config.local_api_base_path)
}

pub(super) fn mode_requires_remote_authority(config: &UserCenterRuntimeConfig) -> bool {
    config.mode.is_remote()
}

pub(super) fn signed_handshake_headers(
    config: &UserCenterRuntimeConfig,
    method: &str,
    path: &str,
) -> Option<Vec<(String, String)>> {
    let secret_id = config.secret_id.as_ref()?;
    let shared_secret = config.shared_secret.as_ref()?;
    let signed_at = im_time::utc_now_rfc3339_millis();
    let signing_message = [
        config.app_id.as_str(),
        config.provider_key.as_str(),
        "provider-shared-secret",
        method,
        path,
        signed_at.as_str(),
    ]
    .join("\n");
    let signature = hmac_sha256_hex(shared_secret.as_str(), signing_message.as_str());

    Some(vec![
        (USER_CENTER_STANDARD_APP_ID_HEADER_NAME.into(), config.app_id.clone()),
        (
            USER_CENTER_STANDARD_PROVIDER_KEY_HEADER_NAME.into(),
            config.provider_key.clone(),
        ),
        (
            USER_CENTER_STANDARD_HANDSHAKE_MODE_HEADER_NAME.into(),
            "provider-shared-secret".into(),
        ),
        (
            USER_CENTER_STANDARD_SECRET_ID_HEADER_NAME.into(),
            secret_id.clone(),
        ),
        (USER_CENTER_STANDARD_SIGNATURE_HEADER_NAME.into(), signature),
        (USER_CENTER_STANDARD_SIGNED_AT_HEADER_NAME.into(), signed_at),
    ])
}

fn hmac_sha256_hex(secret: &str, message: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("shared secret should be usable as hmac key");
    mac.update(message.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

fn user_center_http_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("user-center upstream reqwest client should build")
    })
}

fn build_upstream_request_url(
    config: &UserCenterRuntimeConfig,
    path: &str,
) -> Result<String, ApiError> {
    let base_url = match config.mode {
        UserCenterRuntimeMode::BuiltinLocal => {
            return Err(ApiError::service_unavailable(
                "auth_authority_unavailable",
                "upstream user-center authority is unavailable in builtin-local mode",
            ));
        }
        UserCenterRuntimeMode::SdkworkCloudAppApi => config.app_api_base_url.as_deref(),
        UserCenterRuntimeMode::ExternalUserCenter => config.external_base_url.as_deref(),
    }
    .map(str::trim)
    .filter(|value| !value.is_empty())
    .ok_or_else(|| {
        ApiError::service_unavailable(
            "auth_authority_unavailable",
            "remote user-center base URL is missing",
        )
    })?;

    let normalized_path = if path.starts_with('/') {
        path.to_owned()
    } else {
        format!("/{path}")
    };
    Ok(format!(
        "{}{}",
        base_url.trim_end_matches('/'),
        normalized_path
    ))
}

fn build_upstream_request_headers(
    config: &UserCenterRuntimeConfig,
    method: &str,
    path: &str,
    headers: &HeaderMap,
) -> Vec<(String, String)> {
    let mut outbound = Vec::new();
    copy_governed_header(
        &mut outbound,
        headers,
        config.authorization_header_name.as_str(),
        &[USER_CENTER_DEFAULT_AUTHORIZATION_HEADER_NAME],
    );
    copy_governed_header(
        &mut outbound,
        headers,
        config.access_token_header_name.as_str(),
        &[USER_CENTER_DEFAULT_ACCESS_TOKEN_HEADER_NAME],
    );
    copy_governed_header(
        &mut outbound,
        headers,
        config.refresh_token_header_name.as_str(),
        &[USER_CENTER_DEFAULT_REFRESH_TOKEN_HEADER_NAME],
    );
    copy_governed_header(
        &mut outbound,
        headers,
        config.session_header_name.as_str(),
        &[USER_CENTER_DEFAULT_SESSION_HEADER_NAME],
    );
    if let Some(handshake_headers) = signed_handshake_headers(config, method, path) {
        outbound.extend(handshake_headers);
    }
    outbound
}

fn copy_governed_header(
    outbound: &mut Vec<(String, String)>,
    headers: &HeaderMap,
    outbound_name: &str,
    fallback_names: &[&str],
) {
    let mut aliases = Vec::with_capacity(fallback_names.len() + 1);
    aliases.push(outbound_name);
    aliases.extend(fallback_names.iter().copied());
    if let Some(value) = read_first_header_case_insensitive(headers, aliases.as_slice()) {
        if !outbound
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(outbound_name))
        {
            outbound.push((outbound_name.to_owned(), value));
        }
    }
}

async fn build_proxy_response(
    config: &UserCenterRuntimeConfig,
    response: reqwest::Response,
) -> Result<Response, ApiError> {
    let status = axum::http::StatusCode::from_u16(response.status().as_u16()).map_err(|error| {
        ApiError::service_unavailable(
            "auth_authority_unavailable",
            format!("upstream user-center returned invalid status code: {error}"),
        )
    })?;
    let response_headers = response.headers().clone();
    let body_bytes = response.bytes().await.map_err(|error| {
        ApiError::service_unavailable(
            "auth_authority_unavailable",
            format!("failed to read upstream user-center response body: {error}"),
        )
    })?;
    let mut builder = axum::http::Response::builder().status(status);
    for header_name in [
        CONTENT_TYPE.as_str(),
        config.authorization_header_name.as_str(),
        config.access_token_header_name.as_str(),
        config.refresh_token_header_name.as_str(),
        config.session_header_name.as_str(),
        USER_CENTER_DEFAULT_AUTHORIZATION_HEADER_NAME,
        USER_CENTER_DEFAULT_ACCESS_TOKEN_HEADER_NAME,
        USER_CENTER_DEFAULT_REFRESH_TOKEN_HEADER_NAME,
        USER_CENTER_DEFAULT_SESSION_HEADER_NAME,
    ] {
        if let Some(value) = find_reqwest_header_case_insensitive(&response_headers, header_name) {
            builder = builder.header(header_name, value);
        }
    }

    Ok(builder
        .body(Body::from(body_bytes))
        .expect("proxied user-center response should build"))
}

fn validate_runtime_config(config: &UserCenterRuntimeConfig) -> Result<(), String> {
    if !config.mode.is_remote() {
        return Ok(());
    }

    let (base_url, base_url_env) = match config.mode {
        UserCenterRuntimeMode::SdkworkCloudAppApi => (
            config.app_api_base_url.as_ref(),
            USER_CENTER_APP_API_BASE_URL_ENV,
        ),
        UserCenterRuntimeMode::ExternalUserCenter => (
            config.external_base_url.as_ref(),
            USER_CENTER_EXTERNAL_BASE_URL_ENV,
        ),
        UserCenterRuntimeMode::BuiltinLocal => unreachable!("local mode is handled above"),
    };

    if base_url.is_none() {
        return Err(format!(
            "{base_url_env} is required when {USER_CENTER_MODE_ENV}={}",
            config.mode.as_str()
        ));
    }
    if config.provider_key.trim().is_empty() {
        return Err(format!(
            "{USER_CENTER_PROVIDER_KEY_ENV} is required when {USER_CENTER_MODE_ENV}={}",
            config.mode.as_str()
        ));
    }
    if config.secret_id.is_none() {
        return Err(format!(
            "{USER_CENTER_SECRET_ID_ENV} is required when {USER_CENTER_MODE_ENV}={}",
            config.mode.as_str()
        ));
    }
    if config.shared_secret.is_none() {
        return Err(format!(
            "{USER_CENTER_SHARED_SECRET_ENV} is required when {USER_CENTER_MODE_ENV}={}",
            config.mode.as_str()
        ));
    }

    Ok(())
}

fn resolve_user_center_mode() -> Result<UserCenterRuntimeMode, String> {
    let configured = read_runtime_text(&[USER_CENTER_MODE_ENV, USER_CENTER_MODE_ALIAS_ENV])
        .unwrap_or_else(|| "builtin-local".into());
    match configured.trim().to_ascii_lowercase().as_str() {
        "" | "builtin-local" => Ok(UserCenterRuntimeMode::BuiltinLocal),
        "sdkwork-cloud-app-api" => {
            Ok(UserCenterRuntimeMode::SdkworkCloudAppApi)
        }
        "external-user-center" => Ok(UserCenterRuntimeMode::ExternalUserCenter),
        other => Err(format!(
            "{USER_CENTER_MODE_ENV} must be one of: builtin-local, sdkwork-cloud-app-api, external-user-center; received {other}"
        )),
    }
}

fn normalize_api_base_path(value: &str) -> String {
    let normalized = value.trim();
    if normalized.is_empty() || normalized == "/" {
        return USER_CENTER_DEFAULT_LOCAL_API_BASE_PATH.into();
    }

    let prefixed = if normalized.starts_with('/') {
        normalized.to_owned()
    } else {
        format!("/{normalized}")
    };
    prefixed.trim_end_matches('/').to_owned()
}

fn read_runtime_text(keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| std::env::var(key).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn read_runtime_bool(keys: &[&str]) -> Option<bool> {
    read_runtime_text(keys).and_then(|value| match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    })
}

fn read_runtime_u64(keys: &[&str]) -> Option<u64> {
    read_runtime_text(keys).and_then(|value| value.parse::<u64>().ok())
}

fn read_header_case_insensitive(headers: &HeaderMap, expected_name: &str) -> Option<String> {
    let expected_name = expected_name.trim().to_ascii_lowercase();
    headers.iter().find_map(|(name, value)| {
        if name.as_str().to_ascii_lowercase() != expected_name {
            return None;
        }

        value
            .to_str()
            .ok()
            .map(str::trim)
            .filter(|candidate| !candidate.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn read_first_header_case_insensitive(headers: &HeaderMap, expected_names: &[&str]) -> Option<String> {
    expected_names
        .iter()
        .find_map(|name| read_header_case_insensitive(headers, name))
}

fn find_reqwest_header_case_insensitive(
    headers: &reqwest::header::HeaderMap,
    expected_name: &str,
) -> Option<String> {
    let expected_name = expected_name.trim().to_ascii_lowercase();
    headers.iter().find_map(|(name, value)| {
        if name.as_str().to_ascii_lowercase() != expected_name {
            return None;
        }

        value
            .to_str()
            .ok()
            .map(str::trim)
            .filter(|candidate| !candidate.is_empty())
            .map(ToOwned::to_owned)
    })
}
