use std::collections::BTreeSet;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::header::AUTHORIZATION;
use axum::http::{HeaderMap, HeaderValue};
use craw_chat_ccp_core::{CcpActor, CcpAuthority, CcpSender};
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;

pub const PUBLIC_BEARER_HS256_SECRET_ENV: &str = "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET";
pub const PUBLIC_BEARER_REQUIRE_EXP_ENV: &str = "CRAW_CHAT_PUBLIC_BEARER_REQUIRE_EXP";
pub const PUBLIC_BEARER_MAX_TTL_SECONDS_ENV: &str = "CRAW_CHAT_PUBLIC_BEARER_MAX_TTL_SECONDS";
pub const PUBLIC_BEARER_REQUIRED_ISS_ENV: &str = "CRAW_CHAT_PUBLIC_BEARER_REQUIRED_ISS";
pub const PUBLIC_BEARER_REQUIRED_AUD_ENV: &str = "CRAW_CHAT_PUBLIC_BEARER_REQUIRED_AUD";
const TEMPORAL_CLAIM_CLOCK_SKEW_SECONDS: u64 = 60;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthContext {
    pub tenant_id: String,
    pub actor_id: String,
    pub actor_kind: String,
    pub session_id: Option<String>,
    pub device_id: Option<String>,
    pub permissions: BTreeSet<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthContextError {
    code: &'static str,
    message: String,
}

impl AuthContextError {
    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    fn missing(message: impl Into<String>) -> Self {
        Self {
            code: "auth_context_missing",
            message: message.into(),
        }
    }

    fn invalid(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AuthContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AuthContextError {}

impl AuthContext {
    pub fn has_permission(&self, permission: &str) -> bool {
        if permission.trim().is_empty() {
            return false;
        }

        if self.permissions.contains("*")
            || self.permissions.contains("tenant.admin")
            || self.permissions.contains(permission)
        {
            return true;
        }

        let segments: Vec<&str> = permission.split('.').collect();
        for index in 1..segments.len() {
            let wildcard = format!("{}.*", segments[..index].join("."));
            if self.permissions.contains(wildcard.as_str()) {
                return true;
            }
        }

        false
    }

    pub fn ccp_authority(&self) -> CcpAuthority {
        CcpAuthority::new(
            self.tenant_id.clone(),
            CcpSender::new(
                self.actor_id.clone(),
                self.device_id.clone(),
                self.session_id.clone(),
            ),
            CcpActor::new(self.actor_id.clone(), self.actor_kind.clone()),
        )
    }
}

pub fn resolve_auth_context(headers: &HeaderMap) -> Result<AuthContext, AuthContextError> {
    if let Some(value) = headers.get(AUTHORIZATION) {
        return resolve_bearer_header(value);
    }

    resolve_trusted_headers(headers)
}

pub fn resolve_bearer_auth_context(headers: &HeaderMap) -> Result<AuthContext, AuthContextError> {
    let value = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| AuthContextError::missing("authorization bearer token is required"))?;

    resolve_bearer_header(value)
}

pub fn resolve_public_bearer_auth_context(
    headers: &HeaderMap,
) -> Result<AuthContext, AuthContextError> {
    let value = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| AuthContextError::missing("authorization bearer token is required"))?;
    let secret = std::env::var(PUBLIC_BEARER_HS256_SECRET_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AuthContextError::invalid(
                "public_bearer_secret_missing",
                format!(
                    "public bearer verifier secret is missing: {}",
                    PUBLIC_BEARER_HS256_SECRET_ENV
                ),
            )
        })?;

    resolve_signed_bearer_header(value, secret.as_bytes())
}

pub fn encode_hs256_bearer_token(claims: &Value, secret: &str) -> Result<String, AuthContextError> {
    let header = serde_json::json!({
        "alg": "HS256",
        "typ": "JWT"
    });
    let header_segment = encode_base64url(
        serde_json::to_vec(&header)
            .map_err(|_| {
                AuthContextError::invalid("jwt_header_invalid", "jwt header is not valid json")
            })?
            .as_slice(),
    );
    let payload_segment = encode_base64url(
        serde_json::to_vec(claims)
            .map_err(|_| {
                AuthContextError::invalid(
                    "jwt_claims_invalid",
                    "jwt claims payload is not valid json",
                )
            })?
            .as_slice(),
    );
    let signing_input = format!("{header_segment}.{payload_segment}");
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).map_err(|_| {
        AuthContextError::invalid(
            "public_bearer_secret_invalid",
            "public bearer verifier secret is invalid",
        )
    })?;
    mac.update(signing_input.as_bytes());
    let signature = mac.finalize().into_bytes();
    let signature_segment = encode_base64url(signature.as_slice());

    Ok(format!(
        "{header_segment}.{payload_segment}.{signature_segment}"
    ))
}

pub fn resolve_trusted_headers(headers: &HeaderMap) -> Result<AuthContext, AuthContextError> {
    let tenant_id = resolve_header(headers, &["x-tenant-id"])?;
    let actor_id = resolve_header(headers, &["x-actor-id", "x-user-id"])?;
    let actor_kind =
        resolve_optional_header(headers, &["x-actor-kind"]).unwrap_or_else(|| "user".into());
    let session_id = resolve_optional_header(headers, &["x-session-id"]);
    let device_id = resolve_optional_header(headers, &["x-device-id"]);
    let permissions = resolve_permissions_from_headers(headers);

    Ok(AuthContext {
        tenant_id,
        actor_id,
        actor_kind,
        session_id,
        device_id,
        permissions,
    })
}

fn resolve_bearer_header(value: &HeaderValue) -> Result<AuthContext, AuthContextError> {
    let raw = value.to_str().map_err(|_| {
        AuthContextError::invalid(
            "authorization_invalid",
            "authorization header is not valid utf-8",
        )
    })?;
    let token = raw
        .strip_prefix("Bearer ")
        .or_else(|| raw.strip_prefix("bearer "))
        .ok_or_else(|| {
            AuthContextError::invalid(
                "authorization_scheme_invalid",
                "authorization scheme must be Bearer",
            )
        })?;

    resolve_bearer_token(token)
}

fn resolve_signed_bearer_header(
    value: &HeaderValue,
    secret: &[u8],
) -> Result<AuthContext, AuthContextError> {
    let raw = value.to_str().map_err(|_| {
        AuthContextError::invalid(
            "authorization_invalid",
            "authorization header is not valid utf-8",
        )
    })?;
    let token = raw
        .strip_prefix("Bearer ")
        .or_else(|| raw.strip_prefix("bearer "))
        .ok_or_else(|| {
            AuthContextError::invalid(
                "authorization_scheme_invalid",
                "authorization scheme must be Bearer",
            )
        })?;

    resolve_hs256_bearer_token(token, secret)
}

fn resolve_bearer_token(token: &str) -> Result<AuthContext, AuthContextError> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return Err(AuthContextError::invalid(
            "jwt_shape_invalid",
            "bearer token must be a jwt-like token",
        ));
    }

    let payload = decode_base64url(parts[1])?;
    let claims: Value = serde_json::from_slice(&payload).map_err(|_| {
        AuthContextError::invalid("jwt_claims_invalid", "jwt claims payload is not valid json")
    })?;

    let tenant_id = resolve_claim(
        &claims,
        &["tenant_id", "tenantId"],
        "jwt tenant claim is missing",
    )?;
    let actor_id = resolve_claim(
        &claims,
        &["sub", "actor_id", "actorId", "user_id", "userId"],
        "jwt actor claim is missing",
    )?;
    let actor_kind = find_claim(
        &claims,
        &["actor_kind", "actorKind", "principal_type", "principalType"],
    )
    .unwrap_or_else(|| "user".into());
    let session_id = find_claim(&claims, &["sid", "session_id", "sessionId"]);
    let device_id = find_claim(&claims, &["did", "device_id", "deviceId"]);
    let permissions = resolve_permissions_from_claims(&claims);

    Ok(AuthContext {
        tenant_id,
        actor_id,
        actor_kind,
        session_id,
        device_id,
        permissions,
    })
}

fn resolve_hs256_bearer_token(token: &str, secret: &[u8]) -> Result<AuthContext, AuthContextError> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(AuthContextError::invalid(
            "jwt_shape_invalid",
            "public bearer token must contain header, payload, and signature",
        ));
    }

    let header_bytes = decode_base64url(parts[0])?;
    let header: Value = serde_json::from_slice(&header_bytes).map_err(|_| {
        AuthContextError::invalid("jwt_header_invalid", "jwt header is not valid json")
    })?;
    let algorithm = header
        .get("alg")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            AuthContextError::invalid(
                "jwt_algorithm_invalid",
                "public bearer token algorithm must be HS256",
            )
        })?;
    if algorithm != "HS256" {
        return Err(AuthContextError::invalid(
            "jwt_algorithm_invalid",
            "public bearer token algorithm must be HS256",
        ));
    }

    let signature = decode_base64url(parts[2])?;
    let mut mac = Hmac::<Sha256>::new_from_slice(secret).map_err(|_| {
        AuthContextError::invalid(
            "public_bearer_secret_invalid",
            "public bearer verifier secret is invalid",
        )
    })?;
    mac.update(format!("{}.{}", parts[0], parts[1]).as_bytes());
    mac.verify_slice(signature.as_slice()).map_err(|_| {
        AuthContextError::invalid(
            "authorization_signature_invalid",
            "public bearer token signature is invalid",
        )
    })?;

    let payload = decode_base64url(parts[1])?;
    let claims: Value = serde_json::from_slice(&payload).map_err(|_| {
        AuthContextError::invalid("jwt_claims_invalid", "jwt claims payload is not valid json")
    })?;
    validate_temporal_claims(&claims)?;
    validate_public_contract_claims(&claims)?;

    resolve_bearer_token(token)
}

fn validate_public_contract_claims(claims: &Value) -> Result<(), AuthContextError> {
    if let Some(required_issuer) = resolve_public_bearer_required_issuer() {
        let issuer = claims
            .get("iss")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                AuthContextError::invalid(
                    "jwt_issuer_invalid",
                    format!(
                        "public bearer token must include iss claim matching {}",
                        PUBLIC_BEARER_REQUIRED_ISS_ENV
                    ),
                )
            })?;
        if issuer != required_issuer {
            return Err(AuthContextError::invalid(
                "jwt_issuer_invalid",
                format!(
                    "public bearer token issuer must match {}",
                    PUBLIC_BEARER_REQUIRED_ISS_ENV
                ),
            ));
        }
    }

    if let Some(required_audience) = resolve_public_bearer_required_audience() {
        let matches_required_audience = match claims.get("aud") {
            Some(Value::String(value)) => value.trim() == required_audience,
            Some(Value::Array(values)) => values
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .any(|value| value == required_audience),
            _ => false,
        };
        if !matches_required_audience {
            return Err(AuthContextError::invalid(
                "jwt_audience_invalid",
                format!(
                    "public bearer token audience must match {}",
                    PUBLIC_BEARER_REQUIRED_AUD_ENV
                ),
            ));
        }
    }

    Ok(())
}

fn validate_temporal_claims(claims: &Value) -> Result<(), AuthContextError> {
    let now = current_unix_epoch_seconds();
    let not_before = resolve_optional_numeric_claim(claims, "nbf")?;
    if let Some(not_before) = not_before
        && now.saturating_add(TEMPORAL_CLAIM_CLOCK_SKEW_SECONDS) < not_before
    {
        return Err(AuthContextError::invalid(
            "jwt_not_yet_valid",
            "public bearer token is not valid yet",
        ));
    }

    let expires_at = resolve_optional_numeric_claim(claims, "exp")?;
    if require_public_bearer_exp_claim() && expires_at.is_none() {
        return Err(AuthContextError::invalid(
            "jwt_exp_required",
            format!(
                "public bearer token must include exp claim when {} is enabled",
                PUBLIC_BEARER_REQUIRE_EXP_ENV
            ),
        ));
    }
    if let Some(expires_at) = expires_at
        && now.saturating_sub(TEMPORAL_CLAIM_CLOCK_SKEW_SECONDS) >= expires_at
    {
        return Err(AuthContextError::invalid(
            "jwt_expired",
            "public bearer token is expired",
        ));
    }

    let issued_at = resolve_optional_numeric_claim(claims, "iat")?;
    if let Some(issued_at) = issued_at
        && issued_at > now.saturating_add(TEMPORAL_CLAIM_CLOCK_SKEW_SECONDS)
    {
        return Err(AuthContextError::invalid(
            "jwt_issued_at_invalid",
            "public bearer token has invalid issued-at claim",
        ));
    }

    if let Some(max_ttl_seconds) = resolve_public_bearer_max_ttl_seconds() {
        let expires_at = expires_at.ok_or_else(|| {
            AuthContextError::invalid(
                "jwt_exp_required",
                format!(
                    "public bearer token must include exp claim when {} is enabled",
                    PUBLIC_BEARER_MAX_TTL_SECONDS_ENV
                ),
            )
        })?;
        let issued_reference = issued_at.unwrap_or(now);
        if expires_at.saturating_sub(issued_reference)
            > max_ttl_seconds.saturating_add(TEMPORAL_CLAIM_CLOCK_SKEW_SECONDS)
        {
            return Err(AuthContextError::invalid(
                "jwt_ttl_exceeded",
                format!(
                    "public bearer token ttl exceeds maximum allowed by {}",
                    PUBLIC_BEARER_MAX_TTL_SECONDS_ENV
                ),
            ));
        }
    }

    Ok(())
}

fn resolve_optional_numeric_claim(
    claims: &Value,
    claim_name: &'static str,
) -> Result<Option<u64>, AuthContextError> {
    let Some(value) = claims.get(claim_name) else {
        return Ok(None);
    };

    let parsed = match value {
        Value::Number(number) => number.as_u64(),
        Value::String(text) => {
            let text = text.trim();
            if text.is_empty() {
                None
            } else {
                text.parse::<u64>().ok()
            }
        }
        _ => None,
    };

    parsed
        .ok_or_else(|| {
            AuthContextError::invalid(
                "jwt_temporal_claim_invalid",
                format!("jwt temporal claim {claim_name} must be an integer"),
            )
        })
        .map(Some)
}

fn current_unix_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_secs()
}

fn require_public_bearer_exp_claim() -> bool {
    std::env::var(PUBLIC_BEARER_REQUIRE_EXP_ENV)
        .ok()
        .is_some_and(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
}

fn resolve_public_bearer_max_ttl_seconds() -> Option<u64> {
    std::env::var(PUBLIC_BEARER_MAX_TTL_SECONDS_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
}

pub fn resolve_public_bearer_required_issuer() -> Option<String> {
    std::env::var(PUBLIC_BEARER_REQUIRED_ISS_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

pub fn resolve_public_bearer_required_audience() -> Option<String> {
    std::env::var(PUBLIC_BEARER_REQUIRED_AUD_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn resolve_header(headers: &HeaderMap, names: &[&str]) -> Result<String, AuthContextError> {
    resolve_optional_header(headers, names).ok_or_else(|| {
        AuthContextError::missing(format!("missing trusted auth header: {}", names[0]))
    })
}

fn resolve_optional_header(headers: &HeaderMap, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        headers
            .get(*name)
            .and_then(|value| value.to_str().ok())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn resolve_claim(
    claims: &Value,
    names: &[&str],
    missing_message: &'static str,
) -> Result<String, AuthContextError> {
    find_claim(claims, names).ok_or_else(|| AuthContextError::missing(missing_message))
}

fn find_claim(claims: &Value, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        claims
            .get(*name)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    })
}

fn resolve_permissions_from_headers(headers: &HeaderMap) -> BTreeSet<String> {
    let mut permissions = BTreeSet::new();

    for name in ["x-permissions", "x-scope", "x-scopes"] {
        if let Some(value) = headers.get(name).and_then(|value| value.to_str().ok()) {
            append_permission_str(&mut permissions, value);
        }
    }

    permissions
}

fn resolve_permissions_from_claims(claims: &Value) -> BTreeSet<String> {
    let mut permissions = BTreeSet::new();

    for name in ["permissions", "perms", "scope", "scp"] {
        if let Some(value) = claims.get(name) {
            append_permission_value(&mut permissions, value);
        }
    }

    permissions
}

fn append_permission_value(permissions: &mut BTreeSet<String>, value: &Value) {
    match value {
        Value::String(text) => append_permission_str(permissions, text),
        Value::Array(items) => {
            for item in items {
                if let Some(text) = item.as_str() {
                    append_permission_str(permissions, text);
                }
            }
        }
        _ => {}
    }
}

fn append_permission_str(permissions: &mut BTreeSet<String>, raw: &str) {
    for token in raw.split(|ch: char| ch.is_whitespace() || ch == ',') {
        let permission = token.trim();
        if !permission.is_empty() {
            permissions.insert(permission.to_owned());
        }
    }
}

fn decode_base64url(input: &str) -> Result<Vec<u8>, AuthContextError> {
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
                return Err(AuthContextError::invalid(
                    "jwt_encoding_invalid",
                    "jwt payload segment is not valid base64url",
                ));
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
        return Err(AuthContextError::invalid(
            "jwt_encoding_invalid",
            "jwt payload segment has invalid trailing bits",
        ));
    }

    Ok(output)
}

fn encode_base64url(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    let mut output = String::with_capacity((input.len() * 4).div_ceil(3));
    let mut chunks = input.chunks_exact(3);
    for chunk in &mut chunks {
        let value = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | chunk[2] as u32;
        output.push(ALPHABET[((value >> 18) & 0x3f) as usize] as char);
        output.push(ALPHABET[((value >> 12) & 0x3f) as usize] as char);
        output.push(ALPHABET[((value >> 6) & 0x3f) as usize] as char);
        output.push(ALPHABET[(value & 0x3f) as usize] as char);
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        let first = remainder[0] as u32;
        let second = remainder.get(1).copied().unwrap_or_default() as u32;
        let value = (first << 16) | (second << 8);
        output.push(ALPHABET[((value >> 18) & 0x3f) as usize] as char);
        output.push(ALPHABET[((value >> 12) & 0x3f) as usize] as char);
        if remainder.len() == 2 {
            output.push(ALPHABET[((value >> 6) & 0x3f) as usize] as char);
        }
    }

    output
}
