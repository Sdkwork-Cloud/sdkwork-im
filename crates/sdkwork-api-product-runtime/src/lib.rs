use anyhow::{Context, Result};
use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, HeaderValue, Method, StatusCode, Uri},
    response::{Redirect, Response},
    routing::{any, get},
    Router,
};
use bytes::Bytes;
use rand::random;
use reqwest::Client;
use sdkwork_api_config::{StandaloneConfig, StandaloneConfigLoader};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Component, Path as StdPath, PathBuf},
};
use tokio::{fs, net::TcpListener, sync::oneshot};
use url::{Host, Url};

mod admin_sandbox;

use admin_sandbox::{handle_admin_sandbox_request, SharedAdminSandboxState};

const JSON_CONTENT_TYPE: &str = "application/json; charset=utf-8";
const ADMIN_BACKEND_NOT_CONFIGURED_MESSAGE: &str = "Admin backend proxy target is not configured. Set SDKWORK_ADMIN_PROXY_TARGET to a compatible /api/admin backend.";
const CACHE_CONTROL_HEADER: &str = "cache-control";
const CONTENT_SECURITY_POLICY_HEADER: &str = "content-security-policy";
const CROSS_ORIGIN_RESOURCE_POLICY_HEADER: &str = "cross-origin-resource-policy";
const PERMISSIONS_POLICY_HEADER: &str = "permissions-policy";
const REFERRER_POLICY_HEADER: &str = "referrer-policy";
const X_CONTENT_TYPE_OPTIONS_HEADER: &str = "x-content-type-options";
const X_FRAME_OPTIONS_HEADER: &str = "x-frame-options";
const DEFAULT_PERMISSIONS_POLICY: &str =
    "accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()";

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
    portal_api_base_url: String,
    site_dirs: ProductSiteDirs,
}

enum ResolvedSiteAsset {
    StaticFile(PathBuf),
    SpaShell(PathBuf),
}

impl RouterProductRuntime {
    pub async fn start(
        _loader: StandaloneConfigLoader,
        config: StandaloneConfig,
        options: RouterProductRuntimeOptions,
    ) -> Result<Self> {
        validate_product_site_dirs(options.site_dirs.clone()).await?;
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
            let state = SharedAdminSandboxState::seeded();
            let credentials = state.login_credentials();
            eprintln!(
                "warning: SDKWORK_ADMIN_SANDBOX is enabled. Admin sandbox login: {} / {} ({}). Override with SDKWORK_ADMIN_SANDBOX_EMAIL and SDKWORK_ADMIN_SANDBOX_PASSWORD.",
                credentials.email, credentials.password, credentials.source
            );
            Some(state)
        } else {
            None
        };
        let portal_api_base_url = config.portal_api_base_url;
        let site_dirs = options.site_dirs;

        let app = Router::new()
            .route("/api/admin", any(proxy_admin_request))
            .route("/api/admin/{*path}", any(proxy_admin_request))
            .route("/api", any(api_not_found))
            .route("/api/{*path}", any(api_not_found))
            .route("/admin", get(redirect_admin_root))
            .route("/admin/", get(serve_admin_site))
            .route("/admin/{*path}", get(serve_admin_site))
            .route("/", get(serve_portal_site))
            .route("/{*path}", get(serve_portal_site))
            .with_state(RuntimeProxyState {
                client: Client::new(),
                admin_proxy_target,
                admin_sandbox,
                portal_api_base_url,
                site_dirs: site_dirs.clone(),
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
            _site_dirs: site_dirs,
        })
    }

    pub fn public_base_url(&self) -> Option<&str> {
        Some(self.base_url.as_str())
    }
}

async fn validate_product_site_dirs(site_dirs: ProductSiteDirs) -> Result<()> {
    validate_site_dir(site_dirs.admin_site_dir.as_path(), "admin").await?;
    validate_site_dir(site_dirs.portal_site_dir.as_path(), "portal").await?;
    Ok(())
}

async fn validate_site_dir(site_dir: &StdPath, site_name: &str) -> Result<()> {
    let metadata = fs::metadata(site_dir).await.with_context(|| {
        format!(
            "desktop runtime {site_name} site directory is missing: {}",
            site_dir.display()
        )
    })?;
    if !metadata.is_dir() {
        anyhow::bail!(
            "desktop runtime {site_name} site directory is not a directory: {}",
            site_dir.display()
        );
    }

    let index_path = site_dir.join("index.html");
    let index_metadata = fs::metadata(index_path.as_path()).await.with_context(|| {
        format!(
            "desktop runtime {site_name} site is missing index.html: {}",
            index_path.display()
        )
    })?;
    if !index_metadata.is_file() {
        anyhow::bail!(
            "desktop runtime {site_name} site index.html is not a file: {}",
            index_path.display()
        );
    }

    Ok(())
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

async fn api_not_found() -> Response {
    json_error_response(StatusCode::NOT_FOUND, "Runtime route not found.")
}

async fn redirect_admin_root() -> Redirect {
    Redirect::permanent("/admin/")
}

async fn serve_admin_site(State(state): State<RuntimeProxyState>, uri: Uri) -> Response {
    let request_path = uri.path().strip_prefix("/admin").unwrap_or("/");
    serve_site_request(state.site_dirs.admin_site_dir.as_path(), request_path).await
}

async fn serve_portal_site(State(state): State<RuntimeProxyState>, uri: Uri) -> Response {
    match resolve_site_request_asset(state.site_dirs.portal_site_dir.as_path(), uri.path()).await {
        Ok(ResolvedSiteAsset::StaticFile(path)) => serve_site_file(path.as_path()).await,
        Ok(ResolvedSiteAsset::SpaShell(path)) => {
            serve_portal_shell(path.as_path(), state.portal_api_base_url.as_str()).await
        }
        Err(response) => response,
    }
}

async fn serve_site_request(site_dir: &StdPath, request_path: &str) -> Response {
    match resolve_site_request_asset(site_dir, request_path).await {
        Ok(ResolvedSiteAsset::StaticFile(path) | ResolvedSiteAsset::SpaShell(path)) => {
            serve_site_file(path.as_path()).await
        }
        Err(response) => response,
    }
}

async fn resolve_site_request_asset(
    site_dir: &StdPath,
    request_path: &str,
) -> Result<ResolvedSiteAsset, Response> {
    let Some(relative_path) = sanitize_site_relative_path(request_path) else {
        return Err(text_response(StatusCode::NOT_FOUND, "Not Found"));
    };

    if relative_path.as_os_str().is_empty() {
        return Ok(ResolvedSiteAsset::SpaShell(site_dir.join("index.html")));
    }

    let candidate = site_dir.join(&relative_path);
    let top_level_index = relative_path == PathBuf::from("index.html");
    match fs::metadata(&candidate).await {
        Ok(metadata) if metadata.is_file() => {
            return Ok(if top_level_index {
                ResolvedSiteAsset::SpaShell(candidate)
            } else {
                ResolvedSiteAsset::StaticFile(candidate)
            });
        }
        Ok(metadata) if metadata.is_dir() => {
            let nested_index = candidate.join("index.html");
            return Ok(ResolvedSiteAsset::StaticFile(nested_index));
        }
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(text_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to inspect runtime site asset: {error}"),
            ));
        }
    }

    if request_looks_like_static_asset(relative_path.as_path()) {
        return Err(text_response(StatusCode::NOT_FOUND, "Not Found"));
    }

    Ok(ResolvedSiteAsset::SpaShell(site_dir.join("index.html")))
}

fn sanitize_site_relative_path(request_path: &str) -> Option<PathBuf> {
    let trimmed = request_path.trim_start_matches('/');
    let mut normalized = PathBuf::new();

    for component in StdPath::new(trimmed).components() {
        match component {
            Component::Normal(part) => normalized.push(part),
            Component::CurDir => {}
            Component::RootDir => {}
            Component::ParentDir | Component::Prefix(_) => return None,
        }
    }

    Some(normalized)
}

fn request_looks_like_static_asset(relative_path: &StdPath) -> bool {
    relative_path.extension().is_some()
}

async fn serve_site_file(path: &StdPath) -> Response {
    match fs::read(path).await {
        Ok(body) => {
            let content_type = mime_guess::from_path(path)
                .first_or_octet_stream()
                .essence_str()
                .to_owned();
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .body(Body::from(body))
                .expect("runtime site file response should build");
            let is_html = response
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .map(|value| value.starts_with("text/html"))
                .unwrap_or(false);
            if is_html {
                apply_html_shell_headers(response.headers_mut(), None);
            } else {
                apply_site_security_headers(response.headers_mut());
            }
            response
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            text_response(StatusCode::NOT_FOUND, "Not Found")
        }
        Err(error) => text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read runtime site asset: {error}"),
        ),
    }
}

async fn serve_portal_shell(path: &StdPath, portal_api_base_url: &str) -> Response {
    match fs::read_to_string(path).await {
        Ok(html) => {
            let script_nonce = create_script_nonce();
            let injected = inject_portal_api_base_url(
                html.as_str(),
                portal_api_base_url,
                script_nonce.as_str(),
            );
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                .body(Body::from(injected))
                .expect("portal shell response should build");
            apply_html_shell_headers(
                response.headers_mut(),
                Some(HtmlShellSecurityPolicy::for_portal_shell(
                    portal_api_base_url,
                    script_nonce.as_str(),
                )),
            );
            response
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            text_response(StatusCode::NOT_FOUND, "Not Found")
        }
        Err(error) => text_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read runtime portal shell: {error}"),
        ),
    }
}

fn inject_portal_api_base_url(html: &str, portal_api_base_url: &str, script_nonce: &str) -> String {
    let serialized_url = serde_json::to_string(portal_api_base_url)
        .expect("portal api base url should serialize into javascript");
    let script = format!(
        "<script nonce=\"{script_nonce}\">window.__CRAW_CHAT_PORTAL_API_BASE_URL__ = {serialized_url};</script>"
    );

    if let Some(head_close_index) = html.find("</head>") {
        let mut injected = String::with_capacity(html.len() + script.len());
        injected.push_str(&html[..head_close_index]);
        injected.push_str(script.as_str());
        injected.push_str(&html[head_close_index..]);
        return injected;
    }

    format!("{script}{html}")
}

struct HtmlShellSecurityPolicy {
    connect_src: String,
    script_nonce: Option<String>,
}

impl HtmlShellSecurityPolicy {
    fn default_shell() -> Self {
        Self {
            connect_src: "'self'".to_owned(),
            script_nonce: None,
        }
    }

    fn for_portal_shell(portal_api_base_url: &str, script_nonce: &str) -> Self {
        Self {
            connect_src: resolve_connect_src(portal_api_base_url),
            script_nonce: Some(script_nonce.to_owned()),
        }
    }
}

fn create_script_nonce() -> String {
    format!("{:032x}", random::<u128>())
}

fn apply_site_security_headers(headers: &mut HeaderMap) {
    headers.insert(
        X_CONTENT_TYPE_OPTIONS_HEADER,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        REFERRER_POLICY_HEADER,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(X_FRAME_OPTIONS_HEADER, HeaderValue::from_static("DENY"));
    headers.insert(
        PERMISSIONS_POLICY_HEADER,
        HeaderValue::from_static(DEFAULT_PERMISSIONS_POLICY),
    );
    headers.insert(
        CROSS_ORIGIN_RESOURCE_POLICY_HEADER,
        HeaderValue::from_static("same-origin"),
    );
}

fn apply_html_shell_headers(headers: &mut HeaderMap, policy: Option<HtmlShellSecurityPolicy>) {
    let policy = policy.unwrap_or_else(HtmlShellSecurityPolicy::default_shell);
    apply_site_security_headers(headers);
    headers.insert(CACHE_CONTROL_HEADER, HeaderValue::from_static("no-store"));
    headers.insert(
        CONTENT_SECURITY_POLICY_HEADER,
        HeaderValue::from_str(create_html_content_security_policy(policy).as_str())
            .expect("html shell content security policy should be valid"),
    );
}

fn create_html_content_security_policy(policy: HtmlShellSecurityPolicy) -> String {
    let script_src = match policy.script_nonce.as_deref() {
        Some(script_nonce) => format!("'self' 'nonce-{script_nonce}'"),
        None => "'self'".to_owned(),
    };

    format!(
        "default-src 'self'; base-uri 'self'; connect-src {}; font-src 'self' data:; form-action 'self'; frame-ancestors 'none'; img-src 'self' data: blob:; object-src 'none'; script-src {}; style-src 'self' 'unsafe-inline'",
        policy.connect_src, script_src
    )
}

fn resolve_connect_src(portal_api_base_url: &str) -> String {
    let mut sources = vec!["'self'".to_owned()];

    if let Ok(url) = Url::parse(portal_api_base_url) {
        if matches!(url.scheme(), "http" | "https") {
            let origin = url.origin().ascii_serialization();
            push_unique_source(&mut sources, origin);
            if let Some(websocket_origin) = websocket_origin_for_url(&url) {
                push_unique_source(&mut sources, websocket_origin);
            }
        }
    }

    sources.join(" ")
}

fn push_unique_source(sources: &mut Vec<String>, value: String) {
    if !sources.iter().any(|existing| existing == &value) {
        sources.push(value);
    }
}

fn websocket_origin_for_url(url: &Url) -> Option<String> {
    let websocket_scheme = match url.scheme() {
        "http" => "ws",
        "https" => "wss",
        _ => return None,
    };
    let host = match url.host()? {
        Host::Domain(value) => value.to_owned(),
        Host::Ipv4(value) => value.to_string(),
        Host::Ipv6(value) => format!("[{value}]"),
    };

    match url.port() {
        Some(port) => Some(format!("{websocket_scheme}://{host}:{port}")),
        None => Some(format!("{websocket_scheme}://{host}")),
    }
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

fn text_response(status: StatusCode, message: impl Into<String>) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(Body::from(message.into()))
        .expect("text runtime response should build")
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
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    struct TestSiteDir {
        path: PathBuf,
    }

    impl TestSiteDir {
        fn new(label: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos();
            let path =
                std::env::temp_dir().join(format!("sdkwork-api-product-runtime-{label}-{unique}"));
            fs::create_dir_all(&path).expect("test site dir should be creatable");
            Self { path }
        }

        fn path(&self) -> &Path {
            self.path.as_path()
        }

        fn write(&self, relative_path: &str, body: &str) {
            let file_path = self.path.join(relative_path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).expect("test file parent dir should be creatable");
            }
            fs::write(file_path, body).expect("test site file should be writable");
        }
    }

    impl Drop for TestSiteDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    async fn start_runtime(site_dirs: ProductSiteDirs) -> RouterProductRuntime {
        RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "http://127.0.0.1:18090".into(),
                admin_sandbox_enabled: false,
            },
            RouterProductRuntimeOptions::desktop(site_dirs),
        )
        .await
        .expect("desktop product runtime should start")
    }

    async fn fetch_response(base_url: &str, path: &str) -> reqwest::Response {
        reqwest::Client::new()
            .get(format!("{base_url}{path}"))
            .send()
            .await
            .expect("runtime request should succeed")
    }

    fn response_header(response: &reqwest::Response, name: &str) -> Option<String> {
        response
            .headers()
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned)
    }

    #[tokio::test]
    async fn proxy_admin_request_returns_structured_503_when_backend_target_is_missing() {
        let response = proxy_admin_request(
            State(RuntimeProxyState {
                client: Client::new(),
                admin_proxy_target: String::new(),
                admin_sandbox: None,
                portal_api_base_url: "http://127.0.0.1:18090".into(),
                site_dirs: ProductSiteDirs::new(".", "."),
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

    #[tokio::test]
    async fn router_runtime_serves_portal_root_and_admin_shell_with_spa_fallbacks() {
        let admin_site_dir = TestSiteDir::new("admin-site");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");
        admin_site_dir.write("assets/admin.js", "console.log('admin-asset');");

        let portal_site_dir = TestSiteDir::new("portal-site");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");
        portal_site_dir.write("assets/portal.js", "console.log('portal-asset');");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let admin_index = fetch_response(base_url.as_str(), "/admin/").await;
        assert_eq!(admin_index.status(), StatusCode::OK);
        assert!(admin_index
            .text()
            .await
            .expect("admin index body should be readable")
            .contains("admin-shell"));

        let admin_route = fetch_response(base_url.as_str(), "/admin/operators/shift").await;
        assert_eq!(admin_route.status(), StatusCode::OK);
        assert!(admin_route
            .text()
            .await
            .expect("admin route body should be readable")
            .contains("admin-shell"));

        let admin_asset = fetch_response(base_url.as_str(), "/admin/assets/admin.js").await;
        assert_eq!(admin_asset.status(), StatusCode::OK);
        assert_eq!(
            admin_asset
                .text()
                .await
                .expect("admin asset body should be readable"),
            "console.log('admin-asset');"
        );

        let portal_index = fetch_response(base_url.as_str(), "/").await;
        assert_eq!(portal_index.status(), StatusCode::OK);
        assert!(portal_index
            .text()
            .await
            .expect("portal index body should be readable")
            .contains("portal-shell"));

        let portal_route = fetch_response(base_url.as_str(), "/workspace/inbox").await;
        assert_eq!(portal_route.status(), StatusCode::OK);
        assert!(portal_route
            .text()
            .await
            .expect("portal route body should be readable")
            .contains("portal-shell"));

        let portal_asset = fetch_response(base_url.as_str(), "/assets/portal.js").await;
        assert_eq!(portal_asset.status(), StatusCode::OK);
        assert_eq!(
            portal_asset
                .text()
                .await
                .expect("portal asset body should be readable"),
            "console.log('portal-asset');"
        );
    }

    #[tokio::test]
    async fn router_runtime_injects_portal_api_base_url_into_portal_shell() {
        let admin_site_dir = TestSiteDir::new("admin-injection");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");

        let portal_site_dir = TestSiteDir::new("portal-injection");
        portal_site_dir.write(
            "index.html",
            "<!doctype html><html><head><title>portal-shell</title></head><body>portal</body></html>",
        );

        let runtime = RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "https://portal-api.example.com/runtime-edge".into(),
                admin_sandbox_enabled: false,
            },
            RouterProductRuntimeOptions::desktop(ProductSiteDirs::new(
                admin_site_dir.path().to_path_buf(),
                portal_site_dir.path().to_path_buf(),
            )),
        )
        .await
        .expect("desktop product runtime should start");
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let portal_index = fetch_response(base_url.as_str(), "/").await;
        assert_eq!(portal_index.status(), StatusCode::OK);
        let content_security_policy =
            response_header(&portal_index, CONTENT_SECURITY_POLICY_HEADER)
                .expect("portal shell should include a content security policy");
        assert!(content_security_policy.contains("https://portal-api.example.com"));
        assert!(content_security_policy.contains("wss://portal-api.example.com"));
        assert_eq!(
            response_header(&portal_index, CACHE_CONTROL_HEADER).as_deref(),
            Some("no-store")
        );
        assert_eq!(
            response_header(&portal_index, X_CONTENT_TYPE_OPTIONS_HEADER).as_deref(),
            Some("nosniff")
        );
        assert_eq!(
            response_header(&portal_index, REFERRER_POLICY_HEADER).as_deref(),
            Some("strict-origin-when-cross-origin")
        );
        assert_eq!(
            response_header(&portal_index, X_FRAME_OPTIONS_HEADER).as_deref(),
            Some("DENY")
        );
        assert_eq!(
            response_header(&portal_index, PERMISSIONS_POLICY_HEADER).as_deref(),
            Some(DEFAULT_PERMISSIONS_POLICY)
        );
        assert_eq!(
            response_header(&portal_index, CROSS_ORIGIN_RESOURCE_POLICY_HEADER).as_deref(),
            Some("same-origin")
        );
        let body = portal_index
            .text()
            .await
            .expect("portal index body should be readable");
        assert!(body.contains("__CRAW_CHAT_PORTAL_API_BASE_URL__"));
        assert!(body.contains("https://portal-api.example.com/runtime-edge"));
        let nonce_start = body
            .find("script nonce=\"")
            .expect("portal shell should inject a nonce-backed script")
            + "script nonce=\"".len();
        let nonce_end = body[nonce_start..]
            .find('"')
            .map(|offset| nonce_start + offset)
            .expect("portal shell nonce should terminate");
        let nonce = &body[nonce_start..nonce_end];
        assert!(content_security_policy.contains(format!("'nonce-{nonce}'").as_str()));
        assert!(content_security_policy.contains("script-src 'self' 'nonce-"));
        assert!(!content_security_policy.contains("script-src 'self' 'unsafe-inline'"));
    }

    #[tokio::test]
    async fn router_runtime_applies_security_headers_to_admin_shells_and_static_assets() {
        let admin_site_dir = TestSiteDir::new("admin-security");
        admin_site_dir.write(
            "index.html",
            "<!doctype html><html><head><title>admin-shell</title></head><body>admin</body></html>",
        );

        let portal_site_dir = TestSiteDir::new("portal-security");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");
        portal_site_dir.write("assets/portal.js", "console.log('portal-asset');");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let admin_shell = fetch_response(base_url.as_str(), "/admin/").await;
        assert_eq!(admin_shell.status(), StatusCode::OK);
        let admin_csp = response_header(&admin_shell, CONTENT_SECURITY_POLICY_HEADER)
            .expect("admin shell should emit a content security policy");
        assert!(admin_csp.contains("connect-src 'self'"));
        assert!(!admin_csp.contains("nonce-"));
        assert_eq!(
            response_header(&admin_shell, CACHE_CONTROL_HEADER).as_deref(),
            Some("no-store")
        );

        let portal_asset = fetch_response(base_url.as_str(), "/assets/portal.js").await;
        assert_eq!(portal_asset.status(), StatusCode::OK);
        assert_eq!(
            response_header(&portal_asset, X_CONTENT_TYPE_OPTIONS_HEADER).as_deref(),
            Some("nosniff")
        );
        assert_eq!(
            response_header(&portal_asset, REFERRER_POLICY_HEADER).as_deref(),
            Some("strict-origin-when-cross-origin")
        );
        assert_eq!(
            response_header(&portal_asset, X_FRAME_OPTIONS_HEADER).as_deref(),
            Some("DENY")
        );
        assert_eq!(
            response_header(&portal_asset, PERMISSIONS_POLICY_HEADER).as_deref(),
            Some(DEFAULT_PERMISSIONS_POLICY)
        );
        assert_eq!(
            response_header(&portal_asset, CROSS_ORIGIN_RESOURCE_POLICY_HEADER).as_deref(),
            Some("same-origin")
        );
        assert_eq!(
            response_header(&portal_asset, CONTENT_SECURITY_POLICY_HEADER),
            None
        );
        assert_eq!(response_header(&portal_asset, CACHE_CONTROL_HEADER), None);
    }

    #[tokio::test]
    async fn router_runtime_refuses_to_start_without_admin_index_html() {
        let admin_site_dir = TestSiteDir::new("admin-missing-index");
        let portal_site_dir = TestSiteDir::new("portal-valid-index");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");

        let error = RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "http://127.0.0.1:18090".into(),
                admin_sandbox_enabled: false,
            },
            RouterProductRuntimeOptions::desktop(ProductSiteDirs::new(
                admin_site_dir.path().to_path_buf(),
                portal_site_dir.path().to_path_buf(),
            )),
        )
        .await
        .expect_err("runtime should fail fast when admin index is missing");

        assert!(error.to_string().contains("admin"));
        assert!(error.to_string().contains("index.html"));
    }

    #[tokio::test]
    async fn router_runtime_refuses_to_start_without_portal_index_html() {
        let admin_site_dir = TestSiteDir::new("admin-valid-index");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");
        let portal_site_dir = TestSiteDir::new("portal-missing-index");

        let error = RouterProductRuntime::start(
            StandaloneConfigLoader,
            StandaloneConfig {
                runtime_bind_addr: "127.0.0.1:0".into(),
                admin_proxy_target: String::new(),
                portal_api_base_url: "http://127.0.0.1:18090".into(),
                admin_sandbox_enabled: false,
            },
            RouterProductRuntimeOptions::desktop(ProductSiteDirs::new(
                admin_site_dir.path().to_path_buf(),
                portal_site_dir.path().to_path_buf(),
            )),
        )
        .await
        .expect_err("runtime should fail fast when portal index is missing");

        assert!(error.to_string().contains("portal"));
        assert!(error.to_string().contains("index.html"));
    }

    #[tokio::test]
    async fn router_runtime_keeps_api_and_missing_assets_outside_spa_fallback() {
        let admin_site_dir = TestSiteDir::new("admin-api-guard");
        admin_site_dir.write("index.html", "<!doctype html><title>admin-shell</title>");

        let portal_site_dir = TestSiteDir::new("portal-api-guard");
        portal_site_dir.write("index.html", "<!doctype html><title>portal-shell</title>");

        let runtime = start_runtime(ProductSiteDirs::new(
            admin_site_dir.path().to_path_buf(),
            portal_site_dir.path().to_path_buf(),
        ))
        .await;
        tokio::task::yield_now().await;

        let base_url = runtime
            .public_base_url()
            .expect("runtime should expose a public base url")
            .to_owned();

        let missing_admin_asset =
            fetch_response(base_url.as_str(), "/admin/assets/missing.js").await;
        assert_eq!(missing_admin_asset.status(), StatusCode::NOT_FOUND);
        assert!(!missing_admin_asset
            .text()
            .await
            .expect("missing admin asset body should be readable")
            .contains("admin-shell"));

        let missing_portal_asset = fetch_response(base_url.as_str(), "/assets/missing.js").await;
        assert_eq!(missing_portal_asset.status(), StatusCode::NOT_FOUND);
        assert!(!missing_portal_asset
            .text()
            .await
            .expect("missing portal asset body should be readable")
            .contains("portal-shell"));

        let unknown_api = fetch_response(base_url.as_str(), "/api/runtime-health").await;
        assert_eq!(unknown_api.status(), StatusCode::NOT_FOUND);
        assert!(!unknown_api
            .text()
            .await
            .expect("unknown api body should be readable")
            .contains("portal-shell"));

        let admin_api = fetch_response(base_url.as_str(), "/api/admin/auth/login").await;
        assert_eq!(admin_api.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(admin_api
            .text()
            .await
            .expect("admin api body should be readable")
            .contains("SDKWORK_ADMIN_PROXY_TARGET"));
    }

    #[test]
    fn standalone_config_tracks_admin_sandbox_mode() {
        let config_source = include_str!("../../sdkwork-api-config/src/lib.rs");

        assert!(config_source.contains("SDKWORK_ADMIN_SANDBOX"));
    }
}
