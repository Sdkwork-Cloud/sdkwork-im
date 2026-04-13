use anyhow::{Context, Result};
use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, Method, StatusCode, Uri},
    response::Response,
    routing::any,
    Router,
};
use bytes::Bytes;
use reqwest::Client;
use sdkwork_api_config::{StandaloneConfig, StandaloneConfigLoader};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};
use tokio::{net::TcpListener, sync::oneshot};

mod admin_sandbox;

use admin_sandbox::{handle_admin_sandbox_request, SharedAdminSandboxState};

const JSON_CONTENT_TYPE: &str = "application/json; charset=utf-8";
const ADMIN_BACKEND_NOT_CONFIGURED_MESSAGE: &str = "Admin backend proxy target is not configured. Set SDKWORK_ADMIN_PROXY_TARGET to a compatible /api/admin backend.";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductSiteDirs {
    pub admin_site_dir: PathBuf,
    pub portal_site_dir: PathBuf,
}

impl ProductSiteDirs {
    pub fn new(admin_site_dir: impl Into<PathBuf>, portal_site_dir: impl Into<PathBuf>) -> Self {
        Self {
            admin_site_dir: admin_site_dir.into(),
            portal_site_dir: portal_site_dir.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouterProductRuntimeOptions {
    pub site_dirs: ProductSiteDirs,
}

impl RouterProductRuntimeOptions {
    pub fn desktop(site_dirs: ProductSiteDirs) -> Self {
        Self { site_dirs }
    }
}

#[derive(Debug)]
pub struct RouterProductRuntime {
    base_url: String,
    shutdown_tx: Option<oneshot::Sender<()>>,
    _site_dirs: ProductSiteDirs,
}

#[derive(Clone)]
struct RuntimeProxyState {
    client: Client,
    admin_proxy_target: String,
    admin_sandbox: Option<SharedAdminSandboxState>,
}

impl RouterProductRuntime {
    pub async fn start(
        _loader: StandaloneConfigLoader,
        config: StandaloneConfig,
        options: RouterProductRuntimeOptions,
    ) -> Result<Self> {
        let listener = TcpListener::bind(resolve_runtime_bind_addr(
            config.runtime_bind_addr.as_str(),
        )?)
        .await
        .context("failed to bind local desktop runtime listener")?;
        let local_addr = listener
            .local_addr()
            .context("failed to resolve local desktop runtime listener address")?;
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let admin_proxy_target = trim_trailing_slash(config.admin_proxy_target);
        let admin_sandbox = if admin_proxy_target.trim().is_empty() && config.admin_sandbox_enabled
        {
            Some(SharedAdminSandboxState::seeded())
        } else {
            None
        };

        let app = Router::new()
            .route("/api/admin", any(proxy_admin_request))
            .route("/api/admin/*path", any(proxy_admin_request))
            .with_state(RuntimeProxyState {
                client: Client::new(),
                admin_proxy_target,
                admin_sandbox,
            });

        tokio::spawn(async move {
            let _ = axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = shutdown_rx.await;
                })
                .await;
        });

        Ok(Self {
            base_url: format!("http://{local_addr}"),
            shutdown_tx: Some(shutdown_tx),
            _site_dirs: options.site_dirs,
        })
    }

    pub fn public_base_url(&self) -> Option<&str> {
        Some(self.base_url.as_str())
    }
}

impl Drop for RouterProductRuntime {
    fn drop(&mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }
    }
}

fn resolve_runtime_bind_addr(value: &str) -> Result<SocketAddr> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Ok(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0));
    }

    trimmed
        .parse()
        .with_context(|| format!("invalid desktop runtime bind address: {trimmed}"))
}

fn trim_trailing_slash(value: String) -> String {
    value.trim().trim_end_matches('/').to_owned()
}

fn rewrite_admin_proxy_path(uri: &Uri) -> String {
    let path_and_query = uri
        .path_and_query()
        .map(|value| value.as_str())
        .unwrap_or("/api/admin");
    let suffix = path_and_query
        .strip_prefix("/api/admin")
        .unwrap_or_default();

    if suffix.is_empty() {
        return "/admin".to_owned();
    }

    if suffix.starts_with('/') || suffix.starts_with('?') {
        return format!("/admin{suffix}");
    }

    format!("/admin/{suffix}")
}

async fn proxy_admin_request(
    State(state): State<RuntimeProxyState>,
    method: Method,
    headers: HeaderMap,
    uri: Uri,
    body: Bytes,
) -> Response {
    if let Some(admin_sandbox) = &state.admin_sandbox {
        return handle_admin_sandbox_request(admin_sandbox, method, headers, uri, body).await;
    }

    if state.admin_proxy_target.trim().is_empty() {
        return json_error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            ADMIN_BACKEND_NOT_CONFIGURED_MESSAGE,
        );
    }

    let upstream_url = format!(
        "{}{}",
        state.admin_proxy_target,
        rewrite_admin_proxy_path(&uri),
    );
    let mut request_builder = state.client.request(method, upstream_url);

    for (name, value) in headers.iter() {
        if *name == header::HOST || *name == header::CONTENT_LENGTH || *name == header::CONNECTION {
            continue;
        }
        request_builder = request_builder.header(name, value);
    }

    match request_builder.body(body).send().await {
        Ok(upstream_response) => build_proxy_response(upstream_response).await,
        Err(error) => json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("desktop admin proxy request failed: {error}").as_str(),
        ),
    }
}

fn json_error_response(status: StatusCode, message: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(Body::from(format!(
            "{{\"error\":{{\"message\":\"{}\"}},\"status\":{}}}",
            escape_json_string(message),
            status.as_u16()
        )))
        .expect("json proxy error response should build")
}

fn escape_json_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

async fn build_proxy_response(upstream_response: reqwest::Response) -> Response {
    let status = upstream_response.status();
    let headers = upstream_response.headers().clone();
    let body = upstream_response.bytes().await.unwrap_or_default();
    let mut response_builder = Response::builder().status(status);

    for (name, value) in headers.iter() {
        if *name == header::TRANSFER_ENCODING || *name == header::CONNECTION {
            continue;
        }
        response_builder = response_builder.header(name, value);
    }

    response_builder
        .body(Body::from(body))
        .expect("proxied admin response should build")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn proxy_admin_request_returns_structured_503_when_backend_target_is_missing() {
        let response = proxy_admin_request(
            State(RuntimeProxyState {
                client: Client::new(),
                admin_proxy_target: String::new(),
                admin_sandbox: None,
            }),
            Method::GET,
            HeaderMap::new(),
            Uri::from_static("/api/admin/auth/login"),
            Bytes::new(),
        )
        .await;

        let status = response.status();
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");
        let body_text = String::from_utf8(body.to_vec()).expect("response body should be utf8");

        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            content_type.as_deref(),
            Some("application/json; charset=utf-8")
        );
        assert!(body_text.contains("SDKWORK_ADMIN_PROXY_TARGET"));
        assert!(body_text.contains("/api/admin"));
    }

    #[test]
    fn standalone_config_tracks_admin_sandbox_mode() {
        let config_source = include_str!("../../sdkwork-api-config/src/lib.rs");

        assert!(config_source.contains("SDKWORK_ADMIN_SANDBOX"));
    }
}
